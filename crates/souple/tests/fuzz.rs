//! A deterministic randomized test of what the engine promises, through the public API only.
//!
//! Each case is a seed. It makes a random forest (depth, fan-out, measured leaves that hug or not,
//! hosted roots, scope hosts, variables defined on nodes and globally) and a random sequence of
//! edits, with a pass between batches: style changes over every `Style` field, insert / move /
//! detach / destroy, content changes with `mark_dirty`, variables set and unset, viewports given,
//! changed and taken, scale changes. Any number may be hostile: NaN, ±inf, ±1e30, -0, subnormal,
//! negative sizes, gaps and flex factors, formulas that divide by zero or overflow. After every
//! pass:
//!
//! - (a) nothing panicked;
//! - (b) every published number is finite;
//! - (c) INCREMENTAL == FROM SCRATCH: a fresh `Tree` built to the same state, laid out once, gives
//!   the same published layout for every node of a root that is laid out — the property that finds
//!   cache bugs. (A root no pass lays out keeps its old boxes by design, and so does a root hosted
//!   in such a tree: those are not compared.);
//! - (d) the changed set is exact: a node is reported iff its published layout differs from the
//!   last pass's, or it is published for the first time; the flags say what differs;
//! - (e) a second pass with no edits reports nothing and calls the measurer zero times;
//! - and no node under a laid-out root is left dirty.
//!
//! The measurer is a fake that obeys the `Measurer` contract (`measure.rs`): text of equal-width
//! words broken greedily, and images. A second test uses a measurer that answers hostile numbers
//! (and breaks the contract), and checks (a), (b), (d) and (e) only.
//!
//! ```text
//! cargo test -p souple --test fuzz                                        the default count
//! SOUPLE_FUZZ_CASES=20000 cargo test -p souple --test fuzz --release      a long run
//! SOUPLE_FUZZ_SEED=1234 SOUPLE_FUZZ_CASES=1 cargo test -p souple --test fuzz   one case again
//! ```
//!
//! A failure prints the case's seed and the shortest sequence of edits the shrinker found that
//! still fails the same way.
//!
//! One known difference is left out of (c) (the README, "Pixel snapping"): an edge within float
//! error of a tie — an absolute position is a sum of offsets in f32 — may land on the other pixel
//! after a whole-pixel move. Tolerated (a whole device px or two, nothing else) and counted;
//! `SOUPLE_FUZZ_EXACT=1` makes it a failure.

use std::cell::Cell;
use std::collections::{HashMap, HashSet};
use std::fmt::Write as _;
use std::panic::{self, AssertUnwindSafe};

use souple::{
    AlignContent, AlignItems, Available, BoxSizing, Change, Display, Edges, FlexDirection, FlexWrap, Formula,
    JustifyContent, Length, MeasureInput, MeasureOutput, Measurer, NodeId, NodeLayout, Op as F, Overflow, Position,
    Size, Style, Tree, VarId, Viewport,
};

// ── the random source ─────────────────────────────────────────────────────────────────────────

/// splitmix64: small, fast, and the same sequence on every platform.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    fn below(&mut self, n: usize) -> usize {
        (self.next() % n.max(1) as u64) as usize
    }

    fn chance(&mut self, percent: usize) -> bool {
        self.below(100) < percent
    }

    fn pick<T: Copy>(&mut self, items: &[T]) -> T {
        items[self.below(items.len())]
    }
}

const HOSTILE: [f32; 15] = [
    f32::NAN,
    f32::INFINITY,
    f32::NEG_INFINITY,
    1e30,
    -1e30,
    -0.0,
    1e-30,
    -1e-30,
    f32::MAX,
    f32::MIN,
    f32::MIN_POSITIVE,
    1e-45,
    3e38,
    -7.0,
    -1e9,
];

const VARS: [VarId; 5] = [VarId::new(0), VarId::new(1), VarId::new(2), VarId::new(3), VarId::new(63)];

/// The generator: a random source and how hostile its numbers are (percent of numbers drawn from
/// [`HOSTILE`]).
struct Gen {
    rng: Rng,
    hostile: usize,
}

impl Gen {
    fn number(&mut self, ordinary: impl FnOnce(&mut Rng) -> f32) -> f32 {
        if self.hostile > 0 && self.rng.chance(self.hostile) {
            self.rng.pick(&HOSTILE)
        } else {
            ordinary(&mut self.rng)
        }
    }

    fn px(&mut self) -> f32 {
        self.number(|r| match r.below(7) {
            0 => 0.0,
            1 => r.below(40) as f32,
            2 => r.below(400) as f32,
            3 => r.below(4000) as f32 / 10.0,
            4 => r.below(300) as f32 + 0.5,
            5 => r.below(100) as f32 / 3.0,
            _ => -(r.below(50) as f32),
        })
    }

    fn percent(&mut self) -> f32 {
        self.number(|r| r.pick(&[0.0, 10.0, 25.0, 33.333, 50.0, 66.6, 100.0, 150.0, -10.0, 12.5]))
    }

    fn factor(&mut self) -> f32 {
        self.number(|r| r.pick(&[0.0, 0.5, 1.0, 1.0, 2.0, 3.0, 0.25, 1.5]))
    }

    fn var(&mut self) -> VarId {
        self.rng.pick(&VARS)
    }

    fn length(&mut self) -> Length {
        match self.rng.below(14) {
            0..=3 => Length::Auto,
            4..=7 => Length::Px(self.px()),
            8..=9 => Length::Percent(self.percent()),
            10..=11 => Length::Var(self.var(), self.factor()),
            _ => self.calc(),
        }
    }

    /// A formula: usually a well-typed random expression, sometimes a random op list (most of
    /// which `Formula::build` refuses — that it refuses without a panic is part of the test).
    fn calc(&mut self) -> Length {
        let mut ops = Vec::new();
        match self.rng.below(10) {
            0 => {
                for _ in 0..self.rng.below(8) {
                    ops.push(self.any_op());
                }
            }
            1 => {
                // Deeper than the evaluator's stack: refused as TooDeep.
                let n = 17 + self.rng.below(4);
                for _ in 0..n {
                    ops.push(F::Percent(self.percent()));
                }
                for _ in 1..n {
                    ops.push(F::Add);
                }
            }
            _ => self.expr(&mut ops, 3),
        }
        match Formula::build(&ops) {
            Ok(length) => length,
            Err(_) => Length::Px(self.px()),
        }
    }

    fn any_op(&mut self) -> F {
        match self.rng.below(11) {
            0 => F::Px(self.px()),
            1 => F::Percent(self.percent()),
            2 => F::Var(self.var(), self.factor()),
            3 => F::Num(self.factor()),
            4 => F::Add,
            5 => F::Sub,
            6 => F::Mul,
            7 => F::Div,
            8 => F::Min(self.rng.below(4) as u8),
            9 => F::Max(self.rng.below(4) as u8),
            _ => F::Clamp,
        }
    }

    /// A length-valued expression in postfix.
    fn expr(&mut self, ops: &mut Vec<F>, depth: usize) {
        if depth == 0 || self.rng.chance(35) {
            ops.push(match self.rng.below(3) {
                0 => F::Px(self.px()),
                1 => F::Percent(self.percent()),
                _ => F::Var(self.var(), self.factor()),
            });
            return;
        }
        match self.rng.below(5) {
            0 => {
                self.expr(ops, depth - 1);
                self.expr(ops, depth - 1);
                ops.push(if self.rng.chance(50) { F::Add } else { F::Sub });
            }
            1 => {
                if self.rng.chance(50) {
                    self.expr(ops, depth - 1);
                    self.number_expr(ops);
                } else {
                    self.number_expr(ops);
                    self.expr(ops, depth - 1);
                }
                ops.push(F::Mul);
            }
            2 => {
                self.expr(ops, depth - 1);
                self.number_expr(ops);
                ops.push(F::Div);
            }
            3 => {
                let n = 1 + self.rng.below(4);
                for _ in 0..n {
                    self.expr(ops, depth - 1);
                }
                ops.push(if self.rng.chance(50) { F::Min(n as u8) } else { F::Max(n as u8) });
            }
            _ => {
                for _ in 0..3 {
                    self.expr(ops, depth - 1);
                }
                ops.push(F::Clamp);
            }
        }
    }

    /// A number-valued expression: a literal, or two that fold (to 0 sometimes: a division by it
    /// is refused).
    fn number_expr(&mut self, ops: &mut Vec<F>) {
        if self.rng.chance(15) {
            ops.push(F::Num(self.rng.pick(&[0.0, -0.0])));
        } else if self.rng.chance(20) {
            let a = self.factor();
            ops.push(F::Num(a));
            ops.push(F::Num(if self.rng.chance(50) { a } else { self.factor() }));
            ops.push(self.rng.pick(&[F::Sub, F::Add, F::Mul]));
        } else {
            ops.push(F::Num(self.factor()));
        }
    }

    fn edit(&mut self) -> Edit {
        match self.rng.below(16) {
            0 => Edit::Display(if self.rng.chance(85) { Display::Flex } else { Display::None }),
            1 => Edit::Position(if self.rng.chance(75) { Position::Relative } else { Position::Absolute }),
            2 => Edit::BoxSizing(self.rng.pick(&[BoxSizing::ContentBox, BoxSizing::BorderBox])),
            3 => Edit::Overflow(self.rng.pick(&[Overflow::Visible, Overflow::Hidden, Overflow::Scroll])),
            4 => Edit::Direction(self.rng.pick(&[
                FlexDirection::Row,
                FlexDirection::Column,
                FlexDirection::RowReverse,
                FlexDirection::ColumnReverse,
            ])),
            5 => Edit::Wrap(self.rng.pick(&[FlexWrap::NoWrap, FlexWrap::Wrap, FlexWrap::WrapReverse])),
            6 => Edit::Justify(self.rng.pick(&[
                JustifyContent::FlexStart,
                JustifyContent::FlexEnd,
                JustifyContent::Center,
                JustifyContent::SpaceBetween,
                JustifyContent::SpaceAround,
                JustifyContent::SpaceEvenly,
            ])),
            7 => Edit::Items(self.align()),
            8 => Edit::SelfAlign(if self.rng.chance(40) { None } else { Some(self.align()) }),
            9 => Edit::Content(self.rng.pick(&[
                AlignContent::Stretch,
                AlignContent::FlexStart,
                AlignContent::FlexEnd,
                AlignContent::Center,
                AlignContent::SpaceBetween,
                AlignContent::SpaceAround,
                AlignContent::SpaceEvenly,
            ])),
            10 => Edit::Grow(self.number(|r| r.pick(&[0.0, 1.0, 1.0, 2.0, 0.5, 0.25, 3.0]))),
            11 => Edit::Shrink(self.number(|r| r.pick(&[0.0, 1.0, 1.0, 2.0, 0.5, 4.0]))),
            12 => Edit::Aspect(if self.rng.chance(50) {
                None
            } else {
                Some(self.number(|r| r.pick(&[1.0, 0.5, 2.0, 1.5, 16.0 / 9.0, 0.0, -1.0])))
            }),
            _ => Edit::Length(self.rng.below(LENGTHS), self.length()),
        }
    }

    fn align(&mut self) -> AlignItems {
        self.rng.pick(&[AlignItems::Stretch, AlignItems::FlexStart, AlignItems::FlexEnd, AlignItems::Center, AlignItems::Baseline])
    }

    fn style(&mut self) -> Style {
        let mut style = Style::default();
        for _ in 0..self.rng.below(9) {
            let edit = self.edit();
            edit.apply(&mut style);
        }
        style
    }

    fn content(&mut self) -> Content {
        if self.rng.chance(75) {
            Content::Text {
                words: self.rng.below(13) as u16,
                word: self.rng.pick(&[5.0, 10.0, 12.0, 7.3, 33.0, 60.0, 81.5]),
                space: self.rng.pick(&[0.0, 4.0, 5.5, 10.0]),
                line: self.rng.pick(&[10.0, 13.7, 20.0]),
            }
        } else {
            Content::Image {
                width: self.rng.pick(&[0.0, 16.0, 48.5, 120.0, 300.0]),
                height: self.rng.pick(&[0.0, 16.0, 30.25, 90.0]),
                fit: self.rng.chance(50),
            }
        }
    }

    fn viewport_size(&mut self) -> Size<Option<f32>> {
        let axis = |g: &mut Self| {
            if g.rng.chance(20) {
                None
            } else {
                Some(g.number(|r| r.pick(&[320.0, 390.0, 400.0, 768.0, 1000.0, 99.5, 0.0, 50.0, 1280.25])))
            }
        };
        Size::new(axis(self), axis(self))
    }

    fn scale(&mut self) -> f32 {
        self.number(|r| r.pick(&[1.0, 2.0, 3.0, 1.5, 2.625, 0.75, 1.25]))
    }
}

// ── style edits ───────────────────────────────────────────────────────────────────────────────

/// The length fields of `Style`, addressed by number.
const LENGTHS: usize = 25;

fn length_mut(style: &mut Style, index: usize) -> &mut Length {
    match index {
        0 => &mut style.flex_basis,
        1 => &mut style.row_gap,
        2 => &mut style.column_gap,
        3 => &mut style.size.width,
        4 => &mut style.size.height,
        5 => &mut style.min_size.width,
        6 => &mut style.min_size.height,
        7 => &mut style.max_size.width,
        8 => &mut style.max_size.height,
        9 => &mut style.margin.left,
        10 => &mut style.margin.top,
        11 => &mut style.margin.right,
        12 => &mut style.margin.bottom,
        13 => &mut style.padding.left,
        14 => &mut style.padding.top,
        15 => &mut style.padding.right,
        16 => &mut style.padding.bottom,
        17 => &mut style.border.left,
        18 => &mut style.border.top,
        19 => &mut style.border.right,
        20 => &mut style.border.bottom,
        21 => &mut style.inset.left,
        22 => &mut style.inset.top,
        23 => &mut style.inset.right,
        _ => &mut style.inset.bottom,
    }
}

const LENGTH_NAMES: [&str; LENGTHS] = [
    "flex_basis", "row_gap", "column_gap", "size.width", "size.height", "min_size.width", "min_size.height",
    "max_size.width", "max_size.height", "margin.left", "margin.top", "margin.right", "margin.bottom",
    "padding.left", "padding.top", "padding.right", "padding.bottom", "border.left", "border.top",
    "border.right", "border.bottom", "inset.left", "inset.top", "inset.right", "inset.bottom",
];

#[derive(Clone, Debug)]
enum Edit {
    Display(Display),
    Position(Position),
    BoxSizing(BoxSizing),
    Overflow(Overflow),
    Direction(FlexDirection),
    Wrap(FlexWrap),
    Justify(JustifyContent),
    Items(AlignItems),
    SelfAlign(Option<AlignItems>),
    Content(AlignContent),
    Grow(f32),
    Shrink(f32),
    Aspect(Option<f32>),
    Length(usize, Length),
}

impl Edit {
    fn apply(&self, style: &mut Style) {
        match self {
            Edit::Display(value) => style.display = *value,
            Edit::Position(value) => style.position = *value,
            Edit::BoxSizing(value) => style.box_sizing = *value,
            Edit::Overflow(value) => style.overflow = *value,
            Edit::Direction(value) => style.flex_direction = *value,
            Edit::Wrap(value) => style.flex_wrap = *value,
            Edit::Justify(value) => style.justify_content = *value,
            Edit::Items(value) => style.align_items = *value,
            Edit::SelfAlign(value) => style.align_self = *value,
            Edit::Content(value) => style.align_content = *value,
            Edit::Grow(value) => style.flex_grow = *value,
            Edit::Shrink(value) => style.flex_shrink = *value,
            Edit::Aspect(value) => style.aspect_ratio = *value,
            Edit::Length(index, length) => *length_mut(style, *index) = length.clone(),
        }
    }
}

/// The fields of `style` that differ from `Style::default()`, for a readable reproduction.
fn style_diff(style: &Style) -> String {
    let default = Style::default();
    let mut out = Vec::new();
    macro_rules! field {
        ($name:ident) => {
            if style.$name != default.$name {
                out.push(format!("{}: {:?}", stringify!($name), style.$name));
            }
        };
    }
    field!(display);
    field!(position);
    field!(box_sizing);
    field!(overflow);
    field!(flex_direction);
    field!(flex_wrap);
    field!(justify_content);
    field!(align_items);
    field!(align_self);
    field!(align_content);
    field!(flex_grow);
    field!(flex_shrink);
    field!(aspect_ratio);
    let mut copy = style.clone();
    let mut base = default.clone();
    for (index, name) in LENGTH_NAMES.iter().enumerate() {
        let length = length_mut(&mut copy, index).clone();
        if length != *length_mut(&mut base, index) {
            out.push(format!("{name}: {length:?}"));
        }
    }
    format!("{{ {} }}", out.join(", "))
}

// ── the measurer ──────────────────────────────────────────────────────────────────────────────

/// What a measured leaf holds, as its measurer sees it.
#[derive(Clone, Debug, PartialEq)]
enum Content {
    /// `words` words of one width, broken greedily at spaces; a baseline in the first line. (Greedy
    /// breaking keeps the `Measurer` contract at any widths; the trees where unequal ones tell a
    /// squeezed answer from the text at its width are in `regressions.rs`.)
    Text { words: u16, word: f32, space: f32, line: f32 },
    /// An intrinsic size; `fit`: it shrinks to a smaller offer and keeps its ratio.
    Image { width: f32, height: f32, fit: bool },
}

impl Content {
    fn measure(&self, input: MeasureInput) -> MeasureOutput {
        match *self {
            Content::Text { words, word, space, line } => {
                let limit = match (input.known.width, input.available.width) {
                    (Some(width), _) => Some(width),
                    (None, Available::Definite(width)) => Some(width),
                    (None, Available::MaxContent) => None,
                    (None, Available::MinContent) => Some(0.0),
                };
                let (mut lines, mut current, mut longest) = (0u32, 0.0f32, 0.0f32);
                for _ in 0..words {
                    if lines == 0 {
                        lines = 1;
                        current = word;
                    } else if limit.is_some_and(|limit| current + space + word > limit) {
                        lines += 1;
                        current = word;
                    } else {
                        current = current + space + word;
                    }
                    longest = longest.max(current);
                }
                MeasureOutput { size: Size::new(longest, lines as f32 * line), baseline: (lines > 0).then_some(line * 0.8) }
            }
            Content::Image { width, height, fit } => {
                let ratio = if width > 0.0 { height / width } else { 0.0 };
                let w = match (input.known.width, input.available.width) {
                    (Some(known), _) if fit => known,
                    (None, Available::Definite(offer)) if fit => width.min(offer.max(0.0)),
                    _ => width,
                };
                let h = if fit { w * ratio } else { height };
                MeasureOutput { size: Size::new(w, h), baseline: None }
            }
        }
    }
}

/// The measurer of one tree: the content of each measured node, by its id in that tree.
struct Fake<'a> {
    contents: &'a HashMap<NodeId, Content>,
    calls: usize,
    /// `Some`: answer hostile numbers instead (the second test), drawn from this.
    hostile: Option<Rng>,
}

impl Measurer for Fake<'_> {
    fn measure(&mut self, node: NodeId, input: MeasureInput) -> MeasureOutput {
        self.calls += 1;
        if let Some(rng) = &mut self.hostile {
            let number = |rng: &mut Rng| if rng.chance(50) { rng.pick(&HOSTILE) } else { rng.below(500) as f32 / 3.0 };
            let size = Size::new(number(rng), number(rng));
            let baseline = if rng.chance(50) { Some(number(rng)) } else { None };
            return MeasureOutput { size, baseline };
        }
        self.contents.get(&node).map_or(MeasureOutput::default(), |content| content.measure(input))
    }
}

// ── the model: the state an embedder knows, from which a fresh tree is built ──────────────────

#[derive(Clone, Copy, Debug, PartialEq)]
enum Port {
    Size(Size<Option<f32>>),
    Node(u32),
}

#[derive(Clone)]
struct ModelNode {
    alive: bool,
    parent: Option<u32>,
    children: Vec<u32>,
    style: Style,
    measured: bool,
    hug: bool,
    content: Content,
    defs: Vec<(VarId, f32)>,
    viewport: Option<Port>,
    scope_host: Option<u32>,
}

#[derive(Clone, Debug)]
enum Op {
    Create { id: u32, style: Option<Box<Style>> },
    Destroy(u32),
    Insert { parent: u32, child: u32, before: Option<u32> },
    Detach(u32),
    SetStyle(u32, Box<Style>),
    Edit(u32, Edit),
    Measured(u32, bool),
    Hug(u32, bool),
    /// New content, and the `mark_dirty` the contract asks for.
    Content(u32, Content),
    MarkDirty(u32),
    Var(u32, VarId, Option<f32>),
    Global(VarId, f32),
    Viewport(u32, Port),
    ClearViewport(u32),
    ScopeHost(u32, Option<u32>),
    Scale(f32),
    Pass,
}

impl Op {
    fn describe(&self) -> String {
        match self {
            Op::Create { id, style: None } => format!("n{id} = create_node()"),
            Op::Create { id, style: Some(style) } => format!("n{id} = create_node_with_style({})", style_diff(style)),
            Op::Destroy(id) => format!("destroy(n{id})"),
            Op::Insert { parent, child, before } => {
                format!("insert(n{parent}, n{child}, {})", before.map_or("None".to_string(), |b| format!("Some(n{b})")))
            }
            Op::Detach(id) => format!("detach(n{id})"),
            Op::SetStyle(id, style) => format!("set_style(n{id}, {})", style_diff(style)),
            Op::Edit(id, edit) => format!("update_style(n{id}, {})", describe_edit(edit)),
            Op::Measured(id, on) => format!("set_measured(n{id}, {on})"),
            Op::Hug(id, on) => format!("set_hug(n{id}, {on})"),
            Op::Content(id, content) => format!("content(n{id}) = {content:?}; mark_dirty(n{id})"),
            Op::MarkDirty(id) => format!("mark_dirty(n{id})"),
            Op::Var(id, var, value) => format!("set_var(n{id}, var {}, {value:?})", var.index()),
            Op::Global(var, value) => format!("set_global(var {}, {value:?})", var.index()),
            Op::Viewport(id, Port::Size(size)) => format!("set_viewport(n{id}, Size({:?}, {:?}))", size.width, size.height),
            Op::Viewport(id, Port::Node(host)) => format!("set_viewport(n{id}, Viewport::node(n{host}))"),
            Op::ClearViewport(id) => format!("clear_viewport(n{id})"),
            Op::ScopeHost(id, host) => {
                format!("set_scope_host(n{id}, {})", host.map_or("None".to_string(), |h| format!("Some(n{h})")))
            }
            Op::Scale(scale) => format!("set_scale({scale:?})"),
            Op::Pass => "compute_layout()".to_string(),
        }
    }
}

fn describe_edit(edit: &Edit) -> String {
    match edit {
        Edit::Length(index, length) => format!("{} = {length:?}", LENGTH_NAMES[*index]),
        other => format!("{other:?}"),
    }
}

#[derive(Default)]
struct Model {
    nodes: Vec<Option<ModelNode>>,
    globals: Vec<(VarId, f32)>,
    scale: f32,
}

impl Model {
    fn new() -> Self {
        Self { nodes: Vec::new(), globals: Vec::new(), scale: 1.0 }
    }

    fn get(&self, id: u32) -> Option<&ModelNode> {
        self.nodes.get(id as usize)?.as_ref().filter(|node| node.alive)
    }

    fn node(&mut self, id: u32) -> &mut ModelNode {
        self.nodes[id as usize].as_mut().expect("a live model node")
    }

    fn alive(&self, id: u32) -> bool {
        self.get(id).is_some()
    }

    fn live_ids(&self) -> Vec<u32> {
        (0..self.nodes.len() as u32).filter(|&id| self.alive(id)).collect()
    }

    fn root_of(&self, id: u32) -> u32 {
        let mut at = id;
        while let Some(parent) = self.get(at).and_then(|node| node.parent) {
            at = parent;
        }
        at
    }

    /// `from` reaches `target` going up any link: a parent, else a root's viewport host AND its
    /// scope host (both, whichever is active: the library refuses a cycle through either).
    fn reaches(&self, from: u32, target: u32) -> bool {
        let mut stack = vec![from];
        let mut seen = HashSet::new();
        while let Some(at) = stack.pop() {
            if at == target {
                return true;
            }
            if !seen.insert(at) {
                continue;
            }
            let Some(node) = self.get(at) else { continue };
            match node.parent {
                Some(parent) => stack.push(parent),
                None => {
                    if let Some(Port::Node(host)) = node.viewport {
                        stack.push(host);
                    }
                    if let Some(host) = node.scope_host {
                        stack.push(host);
                    }
                }
            }
        }
        false
    }

    /// The root is laid out by every pass from a state of its own: a size viewport, or the box of a
    /// host in a tree that is itself laid out so.
    fn fully_live(&self, root: u32) -> bool {
        match self.get(root).and_then(|node| node.viewport) {
            Some(Port::Size(_)) => true,
            Some(Port::Node(host)) => self.fully_live(self.root_of(host)),
            None => false,
        }
    }

    fn subtree(&self, id: u32) -> Vec<u32> {
        let mut out = vec![id];
        let mut next = 0;
        while next < out.len() {
            out.extend(self.get(out[next]).map(|node| node.children.clone()).unwrap_or_default());
            next += 1;
        }
        out
    }

    fn unlink(&mut self, child: u32) {
        if let Some(parent) = self.node(child).parent.take() {
            self.node(parent).children.retain(|&c| c != child);
        }
    }

    /// Applies `op` if it is valid in this state (the API would accept it); `false`: skipped.
    fn apply(&mut self, op: &Op) -> bool {
        match op {
            Op::Create { id, style } => {
                let id = *id as usize;
                if self.nodes.len() <= id {
                    self.nodes.resize(id + 1, None);
                }
                if self.nodes[id].is_some() {
                    return false;
                }
                self.nodes[id] = Some(ModelNode {
                    alive: true,
                    parent: None,
                    children: Vec::new(),
                    style: style.as_deref().cloned().unwrap_or_default(),
                    measured: false,
                    hug: false,
                    content: Content::Text { words: 0, word: 10.0, space: 0.0, line: 10.0 },
                    defs: Vec::new(),
                    viewport: None,
                    scope_host: None,
                });
            }
            Op::Destroy(id) => {
                if !self.alive(*id) {
                    return false;
                }
                let gone: HashSet<u32> = self.subtree(*id).into_iter().collect();
                self.unlink(*id);
                for live in self.live_ids() {
                    let node = self.node(live);
                    if matches!(node.viewport, Some(Port::Node(host)) if gone.contains(&host)) {
                        node.viewport = None;
                    }
                    if node.scope_host.is_some_and(|host| gone.contains(&host)) {
                        node.scope_host = None;
                    }
                }
                for id in gone {
                    self.node(id).alive = false;
                }
            }
            Op::Insert { parent, child, before } => {
                let (parent, child) = (*parent, *child);
                if !self.alive(parent) || !self.alive(child) || self.get(child).unwrap().viewport.is_some() {
                    return false;
                }
                if *before == Some(child) || self.reaches(parent, child) {
                    return false;
                }
                if before.is_some_and(|before| self.get(before).is_none_or(|node| node.parent != Some(parent))) {
                    return false;
                }
                self.unlink(child);
                let node = self.node(child);
                node.scope_host = None;
                node.parent = Some(parent);
                let children = &mut self.node(parent).children;
                let at = before.and_then(|b| children.iter().position(|&c| c == b)).unwrap_or(children.len());
                children.insert(at, child);
            }
            Op::Detach(id) => {
                if !self.alive(*id) {
                    return false;
                }
                self.unlink(*id);
            }
            Op::SetStyle(id, style) => {
                if !self.alive(*id) {
                    return false;
                }
                self.node(*id).style = (**style).clone();
            }
            Op::Edit(id, edit) => {
                if !self.alive(*id) {
                    return false;
                }
                edit.apply(&mut self.node(*id).style);
            }
            Op::Measured(id, on) => {
                if !self.alive(*id) {
                    return false;
                }
                self.node(*id).measured = *on;
            }
            Op::Hug(id, on) => {
                if !self.alive(*id) {
                    return false;
                }
                self.node(*id).hug = *on;
            }
            Op::Content(id, content) => {
                if !self.alive(*id) {
                    return false;
                }
                self.node(*id).content = content.clone();
            }
            Op::MarkDirty(id) => {
                if !self.alive(*id) {
                    return false;
                }
            }
            Op::Var(id, var, value) => {
                if !self.alive(*id) {
                    return false;
                }
                let defs = &mut self.node(*id).defs;
                defs.retain(|(defined, _)| defined != var);
                if let Some(value) = value {
                    defs.push((*var, *value));
                }
            }
            Op::Global(var, value) => {
                self.globals.retain(|(defined, _)| defined != var);
                self.globals.push((*var, *value));
            }
            Op::Viewport(id, port) => {
                if !self.alive(*id) || self.get(*id).unwrap().parent.is_some() {
                    return false;
                }
                if matches!(*port, Port::Node(host) if !self.alive(host) || self.reaches(host, *id)) {
                    return false;
                }
                self.node(*id).viewport = Some(*port);
            }
            Op::ClearViewport(id) => {
                if !self.alive(*id) {
                    return false;
                }
                self.node(*id).viewport = None;
            }
            Op::ScopeHost(id, host) => {
                if !self.alive(*id) || self.get(*id).unwrap().parent.is_some() {
                    return false;
                }
                if host.is_some_and(|host| !self.alive(host) || self.reaches(host, *id)) {
                    return false;
                }
                self.node(*id).scope_host = *host;
            }
            Op::Scale(scale) => {
                // The library ignores a scale that is not a positive finite number.
                if scale.is_finite() && *scale > 0.0 {
                    self.scale = *scale;
                }
            }
            Op::Pass => {}
        }
        true
    }
}

// ── the generator of a case ───────────────────────────────────────────────────────────────────

fn generate(seed: u64) -> Vec<Op> {
    let mut rng = Rng(seed);
    let hostile = match rng.below(10) {
        0..=3 => 0,
        4..=6 => 3,
        7..=8 => 10,
        _ => 30,
    };
    let mut g = Gen { rng, hostile };
    let mut model = Model::new();
    let mut ops = Vec::new();
    let mut next_id = 0u32;
    let push = |model: &mut Model, ops: &mut Vec<Op>, op: Op| {
        if model.apply(&op) {
            ops.push(op);
            true
        } else {
            false
        }
    };

    // The forest: a few trees, built top-down or children-first.
    let trees = 1 + g.rng.below(3);
    let budget = 4 + g.rng.below(40);
    let mut roots = Vec::new();
    for _ in 0..trees {
        let mut frontier = Vec::new();
        let root = next_id;
        next_id += 1;
        let style = if g.rng.chance(50) { Some(Box::new(g.style())) } else { None };
        push(&mut model, &mut ops, Op::Create { id: root, style });
        roots.push(root);
        frontier.push((root, 0usize));
        while let Some((parent, depth)) = frontier.pop() {
            if depth >= 5 || model.live_ids().len() >= budget {
                continue;
            }
            for _ in 0..g.rng.below(5) {
                let child = next_id;
                next_id += 1;
                let style = Some(Box::new(g.style()));
                push(&mut model, &mut ops, Op::Create { id: child, style });
                push(&mut model, &mut ops, Op::Insert { parent, child, before: None });
                frontier.push((child, depth + 1));
            }
        }
    }
    for id in model.live_ids() {
        if model.get(id).unwrap().children.is_empty() && g.rng.chance(45) {
            push(&mut model, &mut ops, Op::Measured(id, true));
            let content = g.content();
            push(&mut model, &mut ops, Op::Content(id, content));
            if g.rng.chance(50) {
                push(&mut model, &mut ops, Op::Hug(id, true));
            }
        }
        if g.rng.chance(15) {
            let (var, value) = (g.var(), g.px());
            push(&mut model, &mut ops, Op::Var(id, var, Some(value)));
        }
    }
    for _ in 0..g.rng.below(3) {
        let (var, value) = (g.var(), g.px());
        push(&mut model, &mut ops, Op::Global(var, value));
    }
    if g.rng.chance(40) {
        let scale = g.scale();
        push(&mut model, &mut ops, Op::Scale(scale));
    }
    for &root in &roots {
        if g.rng.chance(80) {
            let size = g.viewport_size();
            push(&mut model, &mut ops, Op::Viewport(root, Port::Size(size)));
        } else if g.rng.chance(50) {
            let live = model.live_ids();
            let host = g.rng.pick(&live);
            push(&mut model, &mut ops, Op::Viewport(root, Port::Node(host)));
        }
    }
    ops.push(Op::Pass);

    // The edits, a pass after each batch.
    for _ in 0..(5 + g.rng.below(30)) {
        for _ in 0..(1 + g.rng.below(4)) {
            let live = model.live_ids();
            if live.is_empty() {
                let id = next_id;
                next_id += 1;
                push(&mut model, &mut ops, Op::Create { id, style: None });
                continue;
            }
            let node = g.rng.pick(&live);
            let other = g.rng.pick(&live);
            let op = match g.rng.below(100) {
                0..=21 => Op::Edit(node, g.edit()),
                22..=26 => Op::SetStyle(node, Box::new(g.style())),
                27..=36 => {
                    let siblings = model.get(other).unwrap().children.clone();
                    let before = if siblings.is_empty() || g.rng.chance(40) { None } else { Some(g.rng.pick(&siblings)) };
                    Op::Insert { parent: other, child: node, before }
                }
                37..=40 => Op::Detach(node),
                41..=43 => Op::Destroy(node),
                44..=49 => {
                    let id = next_id;
                    next_id += 1;
                    let style = if g.rng.chance(50) { Some(Box::new(g.style())) } else { None };
                    push(&mut model, &mut ops, Op::Create { id, style });
                    if g.rng.chance(50) {
                        push(&mut model, &mut ops, Op::Measured(id, true));
                        let content = g.content();
                        push(&mut model, &mut ops, Op::Content(id, content));
                    }
                    Op::Insert { parent: other, child: id, before: None }
                }
                50..=53 => Op::Measured(node, g.rng.chance(60)),
                54..=56 => Op::Hug(node, g.rng.chance(50)),
                57..=64 => Op::Content(node, g.content()),
                65..=68 => Op::MarkDirty(node),
                69..=78 => {
                    let var = g.var();
                    Op::Var(node, var, if g.rng.chance(75) { Some(g.px()) } else { None })
                }
                79..=82 => Op::Global(g.var(), g.px()),
                83..=88 => Op::Viewport(model.root_of(node), Port::Size(g.viewport_size())),
                89..=90 => Op::Viewport(model.root_of(node), Port::Node(other)),
                91..=92 => Op::ClearViewport(model.root_of(node)),
                93..=94 => Op::ScopeHost(model.root_of(node), if g.rng.chance(70) { Some(other) } else { None }),
                _ => Op::Scale(g.scale()),
            };
            push(&mut model, &mut ops, op);
        }
        ops.push(Op::Pass);
    }
    ops
}

// ── the run of a case ─────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    Panic,
    NonFinite,
    Scratch,
    ChangeSet,
    SecondPass,
    Dirty,
}

#[derive(Debug)]
struct Failure {
    kind: Kind,
    /// The index of the op after which it failed.
    at: usize,
    message: String,
}

fn finite(layout: &NodeLayout) -> bool {
    let NodeLayout { rect, margin, padding, border, content_size } = layout;
    let edges = |e: &Edges<f32>| [e.left, e.top, e.right, e.bottom];
    [rect.x, rect.y, rect.width, rect.height, content_size.width, content_size.height]
        .into_iter()
        .chain(edges(margin))
        .chain(edges(padding))
        .chain(edges(border))
        .all(f32::is_finite)
}

/// The incremental tree and what the checks remember between passes.
struct Run {
    model: Model,
    tree: Tree,
    ids: HashMap<u32, NodeId>,
    back: HashMap<NodeId, u32>,
    contents: HashMap<NodeId, Content>,
    /// The nodes some pass has published.
    published: HashSet<u32>,
    /// `Some`: the hostile measurer's source — the contract is off, (c) is not checked.
    hostile: Option<u64>,
    passes: usize,
    /// The boxes that differed from scratch by the known snapping difference only.
    snapped: Cell<usize>,
}

impl Run {
    fn new(hostile: Option<u64>) -> Self {
        Self {
            model: Model::new(),
            tree: Tree::new(),
            ids: HashMap::new(),
            back: HashMap::new(),
            contents: HashMap::new(),
            published: HashSet::new(),
            hostile,
            passes: 0,
            snapped: Cell::new(0),
        }
    }

    /// The hostile measurer's source for this pass: the same in every replay of the case.
    fn hostile_rng(&self) -> Option<Rng> {
        self.hostile.map(|seed| Rng(seed ^ (self.passes as u64).wrapping_mul(0x2545_F491_4F6C_DD1D)))
    }

    fn apply(&mut self, op: &Op, at: usize) -> Result<(), Failure> {
        if !self.model.apply(op) {
            return Ok(());
        }
        let id = |run: &Self, model_id: u32| run.ids[&model_id];
        match op {
            Op::Create { id: model_id, style } => {
                let node = match style {
                    Some(style) => self.tree.create_node_with_style((**style).clone()),
                    None => self.tree.create_node(),
                };
                self.ids.insert(*model_id, node);
                self.back.insert(node, *model_id);
                self.contents.insert(node, self.model.get(*model_id).unwrap().content.clone());
            }
            Op::Destroy(model_id) => {
                let gone = self.model.nodes.iter().enumerate().filter_map(|(i, n)| {
                    n.as_ref().filter(|n| !n.alive).map(|_| i as u32)
                });
                let gone: Vec<u32> = gone.filter(|i| self.ids.contains_key(i)).collect();
                self.tree.destroy(id(self, *model_id));
                for model_id in gone {
                    if let Some(node) = self.ids.remove(&model_id) {
                        self.back.remove(&node);
                        self.contents.remove(&node);
                    }
                }
            }
            Op::Insert { parent, child, before } => {
                let before = before.map(|b| id(self, b));
                self.tree.insert(id(self, *parent), id(self, *child), before);
            }
            Op::Detach(model_id) => self.tree.detach(id(self, *model_id)),
            Op::SetStyle(model_id, style) => self.tree.set_style(id(self, *model_id), (**style).clone()),
            Op::Edit(model_id, edit) => self.tree.update_style(id(self, *model_id), |style| edit.apply(style)),
            Op::Measured(model_id, on) => self.tree.set_measured(id(self, *model_id), *on),
            Op::Hug(model_id, on) => self.tree.set_hug(id(self, *model_id), *on),
            Op::Content(model_id, content) => {
                let node = id(self, *model_id);
                self.contents.insert(node, content.clone());
                self.tree.mark_dirty(node);
            }
            Op::MarkDirty(model_id) => self.tree.mark_dirty(id(self, *model_id)),
            Op::Var(model_id, var, value) => self.tree.set_var(id(self, *model_id), *var, *value),
            Op::Global(var, value) => self.tree.set_global(*var, *value),
            Op::Viewport(model_id, port) => {
                let viewport = match *port {
                    Port::Size(size) => Viewport::Size(size),
                    Port::Node(host) => Viewport::node(id(self, host)),
                };
                self.tree.set_viewport(id(self, *model_id), viewport);
            }
            Op::ClearViewport(model_id) => self.tree.clear_viewport(id(self, *model_id)),
            Op::ScopeHost(model_id, host) => {
                let host = host.map(|h| id(self, h));
                self.tree.set_scope_host(id(self, *model_id), host);
            }
            Op::Scale(scale) => self.tree.set_scale(*scale),
            Op::Pass => return self.pass(at),
        }
        Ok(())
    }

    fn pass(&mut self, at: usize) -> Result<(), Failure> {
        let fail = |kind: Kind, message: String| Err(Failure { kind, at, message });
        self.passes += 1;
        let live = self.model.live_ids();
        let before: HashMap<u32, NodeLayout> = live.iter().map(|&m| (m, *self.tree.layout(self.ids[&m]))).collect();
        let mut measurer = Fake { contents: &self.contents, calls: 0, hostile: self.hostile_rng() };
        self.tree.compute_layout(&mut measurer);
        let changes: Vec<Change> = self.tree.changes().to_vec();

        // (b) every published number is finite.
        for &m in &live {
            let layout = self.tree.layout(self.ids[&m]);
            if !finite(layout) {
                return fail(Kind::NonFinite, format!("n{m} published {layout:?}"));
            }
        }

        // (d) the changed set is exact.
        let mut reported: HashMap<u32, Change> = HashMap::new();
        for change in &changes {
            let Some(&m) = self.back.get(&change.node) else {
                return fail(Kind::ChangeSet, format!("a change for a node that is not alive: {change:?}"));
            };
            if reported.insert(m, *change).is_some() {
                return fail(Kind::ChangeSet, format!("n{m} reported twice"));
            }
        }
        for &m in &live {
            let node = self.ids[&m];
            let (old, new) = (before[&m], *self.tree.layout(node));
            let under_viewport = self.model.get(self.model.root_of(m)).unwrap().viewport.is_some();
            let first = !self.published.contains(&m) && under_viewport;
            let expected = new != old || first;
            match (reported.get(&m), expected) {
                (Some(_), false) => return fail(Kind::ChangeSet, format!("n{m} reported, but its layout is as it was: {new:?}")),
                (None, true) if first => return fail(Kind::ChangeSet, format!("n{m} is under a laid-out root and was never published")),
                (None, true) => return fail(Kind::ChangeSet, format!("n{m} changed and was not reported: {old:?} -> {new:?}")),
                (Some(change), true) => {
                    let flags = (
                        (new.rect.x, new.rect.y) != (old.rect.x, old.rect.y),
                        (new.rect.width, new.rect.height) != (old.rect.width, old.rect.height),
                        (new.margin, new.padding, new.border, new.content_size)
                            != (old.margin, old.padding, old.border, old.content_size),
                    );
                    if (change.moved, change.resized, change.inner) != flags {
                        return fail(Kind::ChangeSet, format!("n{m}: flags {change:?}, expected (moved, resized, inner) = {flags:?}"));
                    }
                    self.published.insert(m);
                }
                (None, false) => {}
            }
        }

        // No node under a laid-out root is left dirty.
        for &m in &live {
            if self.model.get(self.model.root_of(m)).unwrap().viewport.is_some() && self.tree.needs_layout_at(self.ids[&m]) {
                return fail(Kind::Dirty, format!("n{m} is dirty after the pass"));
            }
        }

        // (e) a second pass has nothing to do.
        if self.tree.needs_layout() {
            return fail(Kind::SecondPass, "needs_layout() right after a pass".to_string());
        }
        let mut measurer = Fake { contents: &self.contents, calls: 0, hostile: self.hostile_rng() };
        self.tree.compute_layout(&mut measurer);
        let again = self.tree.changes().len();
        if again != 0 || measurer.calls != 0 {
            return fail(Kind::SecondPass, format!("a pass with no edits reported {again} changes and measured {} times", measurer.calls));
        }

        // (c) incremental == from scratch.
        if self.hostile.is_none() {
            self.compare_with_scratch(at)?;
        }
        Ok(())
    }

    fn compare_with_scratch(&self, at: usize) -> Result<(), Failure> {
        let model = &self.model;
        let mut fresh = Tree::new();
        let mut ids = HashMap::new();
        let mut contents = HashMap::new();
        let live = model.live_ids();
        for &m in &live {
            let node = model.get(m).unwrap();
            // Both ways in, alternating: a fresh tree must not depend on which.
            let id = if m % 2 == 0 {
                fresh.create_node_with_style(node.style.clone())
            } else {
                let id = fresh.create_node();
                fresh.set_style(id, node.style.clone());
                id
            };
            fresh.set_measured(id, node.measured);
            fresh.set_hug(id, node.hug);
            contents.insert(id, node.content.clone());
            ids.insert(m, id);
        }
        for &m in &live {
            for &child in &model.get(m).unwrap().children {
                fresh.append(ids[&m], ids[&child]);
            }
            for &(var, value) in &model.get(m).unwrap().defs {
                fresh.set_var(ids[&m], var, Some(value));
            }
        }
        for &(var, value) in &model.globals {
            fresh.set_global(var, value);
        }
        fresh.set_scale(model.scale);
        for &m in &live {
            let node = model.get(m).unwrap();
            if let Some(host) = node.scope_host {
                fresh.set_scope_host(ids[&m], Some(ids[&host]));
            }
        }
        for &m in &live {
            match model.get(m).unwrap().viewport {
                Some(Port::Size(size)) => fresh.set_viewport(ids[&m], Viewport::Size(size)),
                Some(Port::Node(host)) => fresh.set_viewport(ids[&m], Viewport::node(ids[&host])),
                None => {}
            }
        }
        let mut measurer = Fake { contents: &contents, calls: 0, hostile: None };
        fresh.compute_layout(&mut measurer);

        // Past 2^24 device px an f32 no longer holds every whole pixel: there an edge is only as
        // exact as f32 allows, and the two may differ by that much (the README's promise). A node
        // whose own edges, or an ancestor's, reach so far is not compared.
        let mut far_incremental = HashMap::new();
        let mut far_fresh = HashMap::new();
        let limit = EXACT / model.scale;
        let mut huge = |m: u32| {
            absolute(model, &self.tree, &self.ids, m, &mut far_incremental).2 >= limit
                || absolute(model, &fresh, &ids, m, &mut far_fresh).2 >= limit
        };
        let mut skip = HashSet::new();
        for &m in &live {
            let mut at = Some(m);
            while let Some(id) = at {
                if huge(id) {
                    skip.insert(m);
                    break;
                }
                let node = model.get(id).unwrap();
                at = node.parent.or(match node.viewport {
                    Some(Port::Node(host)) => Some(host),
                    _ => None,
                });
            }
        }

        for &m in &live {
            if !model.fully_live(model.root_of(m)) || skip.contains(&m) {
                continue;
            }
            let (incremental, scratch) = (self.tree.layout(self.ids[&m]), fresh.layout(ids[&m]));
            if incremental == scratch {
                continue;
            }
            if !exact() && snapping_difference(incremental, scratch, model.scale) {
                self.snapped.set(self.snapped.get() + 1);
                continue;
            }
            return Err(Failure {
                kind: Kind::Scratch,
                at,
                message: format!("n{m}: incremental {incremental:?}\n          from scratch {scratch:?}"),
            });
        }
        Ok(())
    }
}

/// `SOUPLE_FUZZ_EXACT=1`: no tolerance for the known snapping difference (`snapping_difference`).
fn exact() -> bool {
    flag("SOUPLE_FUZZ_EXACT")
}

fn flag(name: &str) -> bool {
    std::env::var_os(name).is_some_and(|value| value != "0")
}

/// The known difference (README, "Pixel snapping"): after a subtree moved by whole device pixels the
/// publish walk does not snap it again. A tie rounds toward +∞, the same pixel wherever the subtree
/// sits; but an edge within float error of a tie — its absolute position a sum of f32 offsets, the
/// sum taken anew at the new place — may land on the other side of it, one device px from where a
/// layout from scratch puts it. Everything but the rect is the same; each of the rect's numbers is
/// a difference of two snapped edges (the box's own two, or its near edge and its parent's), so it
/// is off by a whole number of device px, one or two — never by a fraction of one.
fn snapping_difference(a: &NodeLayout, b: &NodeLayout, scale: f32) -> bool {
    let pixels = |x: f32, y: f32| {
        let off = (x - y) * scale;
        let whole = off.round();
        whole.abs() <= 2.0 && (off - whole).abs() <= 0.01
    };
    (a.margin, a.padding, a.border, a.content_size) == (b.margin, b.padding, b.border, b.content_size)
        && pixels(a.rect.x, b.rect.x)
        && pixels(a.rect.y, b.rect.y)
        && pixels(a.rect.width, b.rect.width)
        && pixels(a.rect.height, b.rect.height)
}

/// 2^24: the largest range in which an f32 holds every whole number.
const EXACT: f32 = 16_777_216.0;

/// The node's published origin in its outermost root's space (hosts followed), and the largest
/// magnitude of its four edges there. Memoized per tree.
fn absolute(
    model: &Model,
    tree: &Tree,
    ids: &HashMap<u32, NodeId>,
    m: u32,
    memo: &mut HashMap<u32, (f32, f32, f32)>,
) -> (f32, f32, f32) {
    if let Some(&known) = memo.get(&m) {
        return known;
    }
    let node = model.get(m).unwrap();
    let layout = tree.layout(ids[&m]);
    let (px, py) = match (node.parent, node.viewport) {
        (Some(parent), _) => {
            let (x, y, _) = absolute(model, tree, ids, parent, memo);
            (x, y)
        }
        (None, Some(Port::Node(host))) => {
            let (x, y, _) = absolute(model, tree, ids, host, memo);
            let host = tree.layout(ids[&host]);
            (x + host.border.left + host.padding.left, y + host.border.top + host.padding.top)
        }
        _ => (0.0, 0.0),
    };
    let (x, y) = (px + layout.rect.x, py + layout.rect.y);
    let far = [x, y, x + layout.rect.width, y + layout.rect.height].into_iter().fold(0.0f32, |far, v| far.max(v.abs()));
    // A NaN or an overflow is as far as it gets.
    let far = if far.is_finite() { far } else { f32::INFINITY };
    memo.insert(m, (x, y, far));
    (x, y, far)
}

/// Replays `ops` on a new run: an op the state no longer allows is skipped. `Ok`: how many boxes
/// differed from scratch by the known snapping difference only.
fn replay(ops: &[Op], hostile: Option<u64>) -> Result<usize, Failure> {
    let outcome = panic::catch_unwind(AssertUnwindSafe(|| {
        let mut run = Run::new(hostile);
        for (at, op) in ops.iter().enumerate() {
            run.apply(op, at)?;
        }
        run.apply(&Op::Pass, ops.len())?;
        Ok(run.snapped.get())
    }));
    match outcome {
        Ok(result) => result,
        Err(payload) => {
            let message = payload
                .downcast_ref::<&str>()
                .map(|s| s.to_string())
                .or_else(|| payload.downcast_ref::<String>().cloned())
                .unwrap_or_else(|| "a panic".to_string());
            Err(Failure { kind: Kind::Panic, at: usize::MAX, message })
        }
    }
}

/// The shortest op list found (by removing chunks, then single ops) that still fails as `kind`.
fn shrink(ops: Vec<Op>, kind: Kind, hostile: Option<u64>) -> Vec<Op> {
    let mut ops = ops;
    let mut budget = 3000usize;
    let mut chunk = ops.len().div_ceil(2).max(1);
    loop {
        let mut start = 0;
        let mut removed = false;
        while start < ops.len() && budget > 0 {
            budget -= 1;
            let end = (start + chunk).min(ops.len());
            let candidate: Vec<Op> = ops[..start].iter().chain(&ops[end..]).cloned().collect();
            if replay(&candidate, hostile).is_err_and(|failure| failure.kind == kind) {
                ops = candidate;
                removed = true;
            } else {
                start = end;
            }
        }
        if budget == 0 || (chunk == 1 && !removed) {
            return ops;
        }
        if !removed {
            chunk = chunk.div_ceil(2);
        }
    }
}

fn env_number(name: &str) -> Option<u64> {
    let text = std::env::var(name).ok()?;
    let text = text.trim();
    match text.strip_prefix("0x") {
        Some(hex) => u64::from_str_radix(hex, 16).ok(),
        None => text.parse().ok(),
    }
}

fn fuzz(name: &str, default_cases: u64, hostile_measurer: bool) {
    let cases = env_number("SOUPLE_FUZZ_CASES").unwrap_or(default_cases);
    let base = env_number("SOUPLE_FUZZ_SEED").unwrap_or(0x5009_1E00);
    let (mut ops_total, mut passes_total, mut snapped) = (0usize, 0usize, 0usize);
    let started = std::time::Instant::now();
    for case in 0..cases {
        let seed = base.wrapping_add(case);
        let ops = generate(seed);
        ops_total += ops.len();
        passes_total += ops.iter().filter(|op| matches!(op, Op::Pass)).count();
        let hostile = hostile_measurer.then_some(seed);
        let failure = match replay(&ops, hostile) {
            Ok(count) => {
                snapped += count;
                continue;
            }
            Err(failure) => failure,
        };

        // Shrink quietly: every replay that panics would print its message.
        let hook = panic::take_hook();
        panic::set_hook(Box::new(|_| {}));
        let small = shrink(ops, failure.kind, hostile);
        panic::set_hook(hook);
        let again = replay(&small, hostile).err();
        let mut report = String::new();
        writeln!(report, "{name}: case seed {seed} failed ({:?}): {}", failure.kind, failure.message).unwrap();
        writeln!(report, "replay: SOUPLE_FUZZ_SEED={seed} SOUPLE_FUZZ_CASES=1 cargo test -p souple --test fuzz {name}").unwrap();
        writeln!(report, "shortest failing sequence ({} ops):", small.len()).unwrap();
        for op in &small {
            writeln!(report, "    {}", op.describe()).unwrap();
        }
        writeln!(report, "    compute_layout()   <- the final pass").unwrap();
        if let Some(again) = again {
            writeln!(report, "which fails ({:?}) after op {}: {}", again.kind, again.at, again.message).unwrap();
        }
        panic!("{report}");
    }
    eprintln!(
        "{name}: {cases} cases, {ops_total} ops, {passes_total} passes in {:.1} s; {snapped} boxes a device px or two \
         off after a whole-pixel move (an edge within float error of a tie: the known snapping difference)",
        started.elapsed().as_secs_f64()
    );
}

#[test]
fn incremental_layout_matches_a_tree_built_from_scratch() {
    fuzz("incremental_layout_matches_a_tree_built_from_scratch", 300, false);
}

#[test]
fn a_hostile_measurer_neither_panics_nor_publishes_a_non_finite_box() {
    fuzz("a_hostile_measurer_neither_panics_nor_publishes_a_non_finite_box", 150, true);
}
