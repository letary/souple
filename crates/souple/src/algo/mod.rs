//! The layout algorithm: CSS flexbox (css-flexbox-1 §9), in logical px, unrounded.
//!
//! The contract with the rest of the crate (`tree.rs`):
//!
//! - read `tree.styles[slot].computed()` (never `authored`), `tree.links`, `tree.meta[slot].measured`;
//! - write `tree.raw[slot]`: the node's box relative to its PARENT's border box, its resolved
//!   padding and border, its content size — in logical px, UNROUNDED (the publish walk snaps);
//! - set `tree.meta[slot].laid = true` on every node whose own final layout ran (its children
//!   were placed again) — the publish walk descends only there;
//! - keep what is reusable between passes in `tree.cache[slot]`; `Cache::clear` is called when the
//!   node or anything under it changed.
//!
//! One recursive question drives everything: [`compute`] — "what is this node's box under these
//! constraints?" ([`Input`]) — answered as a border-box size and a baseline ([`Output`]). A parent
//! asks it of a child as often as the flex steps need (a base size, a min-content size, a cross
//! size), in [`Mode::Size`], which writes nothing; then once in [`Mode::Layout`], which fixes the
//! child's box and lays out its subtree. Every answer is cached on the node, so a subtree nothing
//! changed in answers from its cache without recursing, and a pass after one small change redoes
//! only the path from that change to its root. A measured leaf also keeps what its Measurer said
//! ([`measure`]): one answer serves every later question it implies, so the embedder is asked
//! as rarely as the content allows.
//!
//! The files: `flex.rs` the container algorithm, `leaf.rs` measured and childless boxes,
//! `absolute.rs` out-of-flow children, `model.rs` the resolved box model, `axis.rs` main / cross.

mod absolute;
mod axis;
mod flex;
mod leaf;
mod model;

use crate::tree::{finite_edges, BoxEdges, NodeLayout};
use crate::{Available, Display, Length, MeasureInput, MeasureOutput, Measurer, NodeId, Size, Style, Tree};

use axis::Axis;
pub(crate) use flex::Scratch;
use model::{relative_offset, BoxModel};

/// What a parent asks of a child. Every size is a BORDER box.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Input {
    /// Per axis, the size the parent fixed or the space it offers — one or the other: on a known
    /// axis the space offered plays no part, so a question does not carry it (and two questions
    /// are not told apart by it). Read through `known()` and `available()`.
    pub ask: Size<Ask>,
    /// The containing block — the parent's content box: what the child's percents are of.
    pub parent: Size<Option<f32>>,
    pub mode: Mode,
    /// Ignore the child's own size, min and max properties on this axis: what its content alone
    /// asks for there (a flex base size, the content size suggestion of an automatic minimum, an
    /// intrinsic contribution). The other axis keeps them: a column item's `max-width` still wraps
    /// its text while its height is asked for.
    pub content: Option<Axis>,
    /// What the parent reads of the answer: the less, the less there is to work out.
    pub reads: Reads,
    /// A known height is DEFINITE: the child's children may take a percent of it. A width once
    /// known always is (CSS resolves a percent width against the final width of even a
    /// shrink-to-fit box), but a height that came from content is not: a percent of it is `auto`
    /// (CSS 2 §10.5, css-flexbox-1 §9.8).
    pub height_definite: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Mode {
    /// A sizing question: the answer only, nothing written.
    Size,
    /// The final layout: the node's own box is written to `raw` and its children are placed.
    Layout,
}

/// What a parent reads of an answer, from the least to the most: an answer holding more serves a
/// question reading less.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Reads {
    /// The width alone (an intrinsic contribution, a row's base size, a column's cross size). A
    /// container whose width is known, or decided before its layout (CSS sizes the inline axis
    /// first), answers it without laying out its children — which would ask them about heights
    /// at a width that is not the final one.
    Width,
    Size,
    /// The size and the first baseline (baseline alignment); without it, a container spares the
    /// work of finding its baseline.
    Baseline,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct Output {
    /// The border box (its height unset in an answer that `Reads::Width`).
    pub size: Size<f32>,
    /// The first baseline from the top of the border box; `None` = none (the parent synthesizes
    /// one from the bottom edge). Only searched for when `Reads::Baseline` asked.
    pub baseline: Option<f32>,
}

/// One axis of a question.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Ask {
    /// Fixed by the parent (a definite size, a flexed main size, a stretch): the child takes it as
    /// it is, without clamping it again.
    Known(f32),
    /// Open: the space offered to the child's MARGIN box.
    Offer(Available),
}

/// The axes of a question from what is known and what is offered (on a known axis, the offer is
/// dropped).
pub(crate) fn ask(known: Size<Option<f32>>, available: Size<Available>) -> Size<Ask> {
    let axis = |known: Option<f32>, offer: Available| known.map_or(Ask::Offer(offer), Ask::Known);
    Size::new(axis(known.width, available.width), axis(known.height, available.height))
}

impl Input {
    /// The final layout of a child whose box the parent has fixed.
    pub fn layout(size: Size<f32>, parent: Size<Option<f32>>, height_definite: bool) -> Self {
        Self {
            ask: Size::new(Ask::Known(size.width), Ask::Known(size.height)),
            parent,
            mode: Mode::Layout,
            content: None,
            reads: Reads::Size,
            height_definite,
        }
    }

    /// The sizes the parent fixed.
    pub fn known(&self) -> Size<Option<f32>> {
        let known = |ask: Ask| match ask {
            Ask::Known(size) => Some(size),
            Ask::Offer(_) => None,
        };
        Size::new(known(self.ask.width), known(self.ask.height))
    }

    /// The space offered on each axis (on a known axis, its size).
    pub fn available(&self) -> Size<Available> {
        let offer = |ask: Ask| match ask {
            Ask::Known(size) => Available::Definite(size),
            Ask::Offer(offer) => offer,
        };
        Size::new(offer(self.ask.width), offer(self.ask.height))
    }
}

// ── the cache ────────────────────────────────────────────────────────────────────────────────

/// How many answers a node keeps. A pass asks a node a handful of distinct questions (a base
/// size, a min-content size, a cross size, a contribution); kept across passes, they make a clean
/// subtree free the next time its parent is laid out.
const ENTRIES: usize = 6;

/// One remembered answer: to a sizing question (any node), or — a measured leaf's — the
/// Measurer's own answer to what it was asked. The second is the costly part of a leaf, and the
/// one whose reuse rules are about content rather than margins and clamps (`implies`).
#[derive(Clone, Copy)]
enum Answer {
    Size(Input, Output),
    Measure(MeasureInput, MeasureOutput),
}

/// The node's remembered answers. `Tree::mark_dirty` clears it on a changed node and every
/// ancestor; a node with nothing changed under it keeps answering from here.
#[derive(Default)]
pub(crate) struct Cache {
    answers: [Option<Answer>; ENTRIES],
    /// Where the next answer goes: the oldest slot.
    next: u8,
    /// The input of the last final layout. Its results are in `raw` — the node's and its whole
    /// subtree's — so the same input again has nothing to do.
    layout: Option<(Input, Output)>,
}

impl Cache {
    pub fn clear(&mut self) {
        *self = Self::default();
    }

    /// The answer to the sizing question `input`: only the very same question answers it (on a
    /// known axis the space offered plays no part). A container's answer may hang on anything the
    /// question holds — a percent child resolves against a known width and not against an open
    /// one; a leaf's implications live a level down, in its Measurer's answers.
    fn size(&self, input: &Input) -> Option<Output> {
        self.answers.iter().flatten().find_map(|answer| match answer {
            Answer::Size(cached, output) if same_question(cached, input) => Some(*output),
            _ => None,
        })
    }

    /// The Measurer's answer to `question`, if one it gave already implies it.
    fn measured(&self, question: &MeasureInput) -> Option<MeasureOutput> {
        self.answers.iter().flatten().find_map(|answer| match answer {
            Answer::Measure(asked, output) if implies(asked, output, question) => Some(*output),
            _ => None,
        })
    }

    /// An answer the Measurer gave at this content width: asked at it as a known width, or answering
    /// it to an offer that held it (`held`). Its baseline is the content's at that width whatever
    /// the box's height (content sits at the top of its box: the Measurer contract), so a leaf whose
    /// two sizes are known reads its baseline here (`leaf::measured`).
    fn answer_at(&self, width: f32) -> Option<MeasureOutput> {
        self.answers.iter().flatten().find_map(|answer| match answer {
            Answer::Measure(asked, output) => {
                let at = match asked.known.width {
                    Some(known) => known == width,
                    None => output.size.width == width && held(asked.available.width, width),
                };
                at.then_some(*output)
            }
            Answer::Size(..) => None,
        })
    }

    /// Remembers `answer` in place of the oldest answer it may replace. A Measurer's answer
    /// outranks a sizing answer of the same leaf — the one is a call to the embedder, the other a
    /// few additions over it — so a sizing answer never pushes a Measurer's out (it is not kept
    /// when the Measurer's fill every slot), and a Measurer's pushes a sizing answer out first.
    /// (A container holds sizing answers only: plain rotation.)
    fn put(&mut self, answer: Answer) {
        let next = self.next as usize;
        let measured = |slot: usize| matches!(self.answers[slot], Some(Answer::Measure(..)));
        let slot = match (0..ENTRIES).map(|k| (next + k) % ENTRIES).find(|&slot| !measured(slot)) {
            Some(slot) => slot,
            None if matches!(answer, Answer::Measure(..)) => next,
            None => return,
        };
        self.answers[slot] = Some(answer);
        self.next = ((slot + 1) % ENTRIES) as u8;
    }
}

fn same_question(cached: &Input, input: &Input) -> bool {
    // Axis by axis: the same known size, or the same offer on an open axis (`Ask`).
    cached.ask == input.ask
        && cached.content == input.content
        && cached.parent == input.parent
        && cached.height_definite == input.height_definite
        && cached.reads >= input.reads
}

/// The Measurer answered `answer` to `asked`; does that answer `question` too? Axis by axis, the
/// rules Yoga's measure cache relies on — they hold for any Measurer that sizes its content the
/// way CSS sizes content (offered more room than the content takes, or less room that the content
/// still fits in, the content is laid out the same):
///
/// - a known size: the same size, or the size the content answered when the axis was open and the
///   room offered there held it (text asked again at its own natural width breaks the same way);
/// - an open axis: the same offer; or an unbounded (max-content) answer that fits within the new
///   offer; or a tighter offer than before that the answer still fits within.
///
/// An answer the room did not hold is the content squeezed, not the content at that size: a
/// min-content answer (every soft wrap taken), or one wider than a definite offer (its longest
/// word overflowed). "a bb cccccc" at min-content is three lines, 60 wide; laid out at a known 60
/// it is two. Such an answer serves no question at a known size — the Measurer is asked there.
///
/// A height offered with a question about the height (a row item's cross size, in a row of known
/// height) thus mints no new question as long as the content fits in it. Where it does not fit (a
/// Measurer that caps its height at the offer, as Yoga's text measure does), the question is asked
/// again: the answer can differ. (A question about a width offers no height at all:
/// `flex::width_question`.) A min-content question is matched only by itself.
fn implies(asked: &MeasureInput, answer: &MeasureOutput, question: &MeasureInput) -> bool {
    let axis = |horizontal: bool| {
        let answered = answer.size.get(horizontal);
        match (question.known.get(horizontal), asked.known.get(horizontal)) {
            (Some(size), Some(was)) => size == was,
            // On an axis it was told, the Measurer's answer is not its own: only an open axis
            // says what the content takes — and only where its offer held the answer.
            (Some(size), None) => size == answered && held(asked.available.get(horizontal), answered),
            (None, Some(_)) => false,
            (None, None) => match (asked.available.get(horizontal), question.available.get(horizontal)) {
                (was, now) if was == now => true,
                (Available::MaxContent, Available::Definite(now)) => answered <= now,
                (Available::Definite(was), Available::Definite(now)) => now < was && answered <= now,
                _ => false,
            },
        }
    };
    axis(true) && axis(false)
}

/// The room `offer` held the content's `answered` size on that axis: the answer is what the
/// content takes at that size, not the content squeezed (`implies`).
fn held(offer: Available, answered: f32) -> bool {
    match offer {
        Available::MaxContent => true,
        Available::Definite(room) => answered <= room,
        Available::MinContent => false,
    }
}

// ── the dispatch ─────────────────────────────────────────────────────────────────────────────

/// The node's box under `input`: from the cache, or from the algorithm of its kind.
pub(crate) fn compute(tree: &mut Tree, node: NodeId, input: Input, measurer: &mut dyn Measurer) -> Output {
    let slot = node.slot();
    let measured = tree.meta[slot].measured;
    let leaf = measured || tree.links[slot].first_child.is_none();
    let input = normalize(tree.styles[slot].computed(), input, leaf);
    // A final layout always runs on a fixed box: an axis the caller left open (a content-sized
    // root, an absolute child) is sized first. That is the CSS order — the size, then the layout
    // inside it — and it makes a child's percent of this box's width resolve against its final
    // width. The height found that way came from the content: not definite.
    if input.mode == Mode::Layout && (input.known().width.is_none() || input.known().height.is_none()) {
        let size = compute(tree, node, Input { mode: Mode::Size, reads: Reads::Size, ..input }, measurer).size;
        let height_definite = input.height_definite && input.known().height.is_some();
        let ask = Size::new(Ask::Known(size.width), Ask::Known(size.height));
        return compute(tree, node, Input { ask, height_definite, ..input }, measurer);
    }

    let cache = &tree.cache[slot];
    let cached = match input.mode {
        Mode::Layout => cache.layout.filter(|(cached, _)| *cached == input).map(|(_, output)| output),
        Mode::Size => cache.size(&input),
    };
    if let Some(output) = cached {
        return output;
    }
    let output = if measured {
        leaf::measured(tree, node, &input, measurer)
    } else if leaf {
        leaf::empty(tree, node, &input)
    } else {
        flex::compute_container(tree, node, &input, measurer)
    };
    let cache = &mut tree.cache[slot];
    match input.mode {
        Mode::Layout => cache.layout = Some((input, output)),
        Mode::Size => cache.put(Answer::Size(input, output)),
    }
    output
}

/// The Measurer's answer to `question` about the measured leaf `node`: from the leaf's cache
/// when an answer it gave implies it (`implies`), else asked — and remembered.
pub(crate) fn measure(tree: &mut Tree, node: NodeId, question: MeasureInput, measurer: &mut dyn Measurer) -> MeasureOutput {
    let cache = &mut tree.cache[node.slot()];
    if let Some(answer) = cache.measured(&question) {
        return answer;
    }
    let answer = measurer.measure(node, question);
    cache.put(Answer::Measure(question, answer));
    answer
}

/// One question asked two ways is one cache entry. A node without size, min or max properties on
/// an axis answers a content question about it as it answers the ordinary one; a node whose own
/// lengths hold no percent cannot depend on its containing block (its children's percents are of
/// ITS box, which the rest of the input fixes); whether a height is definite matters only to the
/// children of a node that lays some out — so those parts of the input are dropped before the
/// lookup.
fn normalize(style: &Style, input: Input, leaf: bool) -> Input {
    let sizes = [
        &style.size.width,
        &style.size.height,
        &style.min_size.width,
        &style.min_size.height,
        &style.max_size.width,
        &style.max_size.height,
    ];
    let edges = [&style.margin, &style.padding, &style.border];
    let percent = |length: &Length| match length {
        Length::Percent(_) => true,
        Length::Calc(formula) => formula.has_percent(),
        _ => false,
    };
    let reads_parent = sizes.iter().any(|length| percent(length))
        || edges.iter().any(|edges| [&edges.left, &edges.top, &edges.right, &edges.bottom].into_iter().any(percent));
    // sizes[0], [2], [4] are the horizontal ones.
    let content = input.content.filter(|axis| {
        let first = if axis.is_horizontal() { 0 } else { 1 };
        [sizes[first], sizes[first + 2], sizes[first + 4]].iter().any(|length| !length.is_auto())
    });
    Input {
        content,
        parent: if reads_parent { input.parent } else { Size::new(None, None) },
        height_definite: input.height_definite || leaf || input.known().height.is_none(),
        ..input
    }
}

/// Lays `root` out in `viewport` (`None` on an axis = unbounded). On an axis the viewport bounds
/// and the root's style leaves auto, the root fills the viewport less its margins (its min and max
/// still apply); on an unbounded one it is sized by its content. The viewport is also the percent
/// base of the root's own lengths.
pub(crate) fn layout_root(tree: &mut Tree, root: NodeId, viewport: Size<Option<f32>>, measurer: &mut dyn Measurer) {
    let slot = root.slot();
    let style = &tree.styles[slot].computed();
    if style.display == Display::None {
        hide(tree, root);
        return;
    }
    let model = BoxModel::resolve(style, viewport, None);
    let shift = relative_offset(style, viewport);
    let fill = |horizontal: bool| {
        model.size.get(horizontal).or_else(|| {
            let space = viewport.get(horizontal)? - model.margin.sum(horizontal);
            Some(model.clamp(horizontal, space))
        })
    };
    let available = |space: Option<f32>| space.map_or(Available::MaxContent, Available::Definite);
    let input = Input {
        ask: ask(
            model.with_aspect(Size::new(fill(true), fill(false))),
            Size::new(available(viewport.width), available(viewport.height)),
        ),
        parent: viewport,
        mode: Mode::Layout,
        content: None,
        reads: Reads::Size,
        height_definite: true,
    };
    compute(tree, root, input, measurer);
    let rect = &mut tree.raw[slot].rect;
    rect.x = model.margin.left + shift.0;
    rect.y = model.margin.top + shift.1;
}

// ── shared by the node kinds ─────────────────────────────────────────────────────────────────

/// Writes the node's own final box — everything but its origin, which the parent writes — and
/// marks it laid. `extent` is the far edge of the children's margin boxes from the padding box's
/// origin (0 × 0 without children).
fn finish(tree: &mut Tree, node: NodeId, model: &BoxModel, size: Size<f32>, extent: Size<f32>) {
    let slot = node.slot();
    let raw = &mut tree.raw[slot];
    raw.rect.width = size.width;
    raw.rect.height = size.height;
    // Published as they are: finite, whatever a hostile style made of them.
    raw.unsnapped = BoxEdges {
        margin: finite_edges(model.margin),
        padding: finite_edges(model.padding),
        border: finite_edges(model.border),
    };
    let padding_box = Size::new(
        (size.width - model.border.horizontal()).max(0.0),
        (size.height - model.border.vertical()).max(0.0),
    );
    raw.content_size = Size::new(
        padding_box.width.max(extent.width + model.padding.right),
        padding_box.height.max(extent.height + model.padding.bottom),
    );
    tree.meta[slot].laid = true;
}

/// A node that takes no part in the layout (`display: none`, or a child of a measured leaf): it
/// and its subtree are 0 × 0 at the origin. Their caches are dropped with their boxes — a cached
/// final layout would otherwise claim boxes that are no longer in `raw` when the node shows again.
fn hide(tree: &mut Tree, node: NodeId) {
    let slot = node.slot();
    tree.raw[slot] = NodeLayout::default();
    tree.meta[slot].laid = true;
    tree.cache[slot].clear();
    let mut child = tree.links[slot].first_child;
    while let Some(id) = child {
        hide(tree, id);
        child = tree.links[id.slot()].next;
    }
}

#[cfg(test)]
mod tests {
    use super::implies;
    use crate::{Available, MeasureInput, MeasureOutput, Size};
    use Available::{Definite, MaxContent, MinContent};

    fn question(width: (Option<f32>, Available), height: (Option<f32>, Available)) -> MeasureInput {
        MeasureInput { known: Size::new(width.0, height.0), available: Size::new(width.1, height.1) }
    }

    fn open(offer: Available) -> (Option<f32>, Available) {
        (None, offer)
    }

    fn at(size: f32) -> (Option<f32>, Available) {
        (Some(size), Definite(size))
    }

    #[test]
    fn a_measurer_answer_implies_the_offers_its_content_fits() {
        let unbounded = question(open(MaxContent), open(MaxContent));
        let answer = MeasureOutput { size: Size::new(100.0, 40.0), baseline: None };
        let implied = |asked: &MeasureInput, new: MeasureInput| implies(asked, &answer, &new);

        // An unbounded answer serves any offer it fits in — on either axis — and no tighter one:
        // a Measurer may wrap to the offer, or cap its height at it.
        assert!(implied(&unbounded, question(open(Definite(100.0)), open(Definite(40.0)))));
        assert!(!implied(&unbounded, question(open(Definite(99.0)), open(MaxContent))));
        assert!(!implied(&unbounded, question(open(MaxContent), open(Definite(39.0)))));

        // Asked again at the size it answered: the same layout. Not at another size, though larger:
        // a box wider than its text may still lay its content out differently (an image scales).
        assert!(implied(&unbounded, question(at(100.0), open(MaxContent))));
        assert!(!implied(&unbounded, question(at(120.0), open(MaxContent))));

        // A tighter offer the answer still fits; never a looser one (text wrapped at 200 may unwrap
        // at 300).
        let bounded = question(open(Definite(200.0)), open(MaxContent));
        assert!(implied(&bounded, question(open(Definite(150.0)), open(MaxContent))));
        assert!(!implied(&bounded, question(open(Definite(300.0)), open(MaxContent))));

        // A min-content answer serves only its own question — not even the known size it answered:
        // the content squeezed is not the content at that size.
        let min = question(open(MinContent), open(MaxContent));
        assert!(implied(&min, min));
        assert!(!implied(&min, question(open(Definite(100.0)), open(MaxContent))));
        assert!(!implied(&min, question(at(100.0), open(MaxContent))));

        // A known size equal to the answer, where the offer held it; not where the content
        // overflowed the offer (a word longer than the line).
        assert!(implied(&bounded, question(at(100.0), open(MaxContent))));
        let overflowed = question(open(Definite(80.0)), open(MaxContent));
        assert!(!implied(&overflowed, question(at(100.0), open(MaxContent))));

        // On an axis it was told, the Measurer's answer is not its own: only the same size serves.
        let told = question(at(80.0), open(MaxContent));
        assert!(implied(&told, question(at(80.0), open(MaxContent))));
        assert!(!implied(&told, question(at(100.0), open(MaxContent))));
        assert!(!implied(&told, question(open(MaxContent), open(MaxContent))));
    }
}
