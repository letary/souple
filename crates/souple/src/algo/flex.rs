//! The flex container: css-flexbox-1 §9, one function per step, in the spec's order and under its
//! names, so a step can be read against the spec side by side.
//!
//! The rules this file follows:
//! - the target is what CSS (Chrome) does, not Yoga: an item's automatic minimum is its content
//!   (§4.5), the space distributions fall back as css-align-3 says (`distribute`), a percent of an
//!   indefinite size is `auto`;
//! - the container's WIDTH is decided before anything is laid out in it — from its items'
//!   min-/max-content contributions when nothing fixes it — the way CSS sizes the inline axis
//!   first; the height then follows from the layout at that width (a column wrapping into several
//!   columns is the exception: its width is the sum of the columns, known only from the lines);
//! - positions are worked out logically (from main-start and cross-start) and flipped to physical
//!   once, in `place_items`, so a reversed axis has no branch of its own anywhere else;
//! - a sizing question (`Mode::Size`) runs every step but writes nothing; only `Mode::Layout`
//!   lays the items out at their used sizes and writes the boxes.

use super::absolute::{self, Frame};
use super::axis::{Axes, Axis};
use super::model::{relative_offset, BoxModel};
use super::{ask, compute, finish, hide, Ask, Input, Mode, Output, Reads};
use crate::{
    AlignContent, AlignItems, Available, Display, Edges, FlexWrap, JustifyContent, Tree, Measurer, NodeId,
    Overflow, Position, Size, Style,
};

/// How far a line may overflow before an item breaks onto the next one: float sums of percents
/// (three items of 33.333%) land a hair over the container and must still share a line.
const FIT_EPSILON: f32 = 1e-3;

/// A container's working memory. `Tree` keeps one per nesting level in use and hands them out
/// again, so a pass in the steady state allocates nothing.
#[derive(Default)]
pub(crate) struct Scratch {
    items: Vec<Item>,
    lines: Vec<Line>,
}

/// An in-flow child and what the steps learn about it. Sizes are border boxes.
struct Item {
    node: NodeId,
    model: BoxModel,
    align: AlignItems,
    grow: f32,
    shrink: f32,
    /// A definite `flex-basis`; `None` = `auto`: the main size property, else the content.
    basis: Option<f32>,
    /// The minimum main size: the `min` property's, or the automatic minimum once resolved.
    min_main: f32,
    /// The automatic minimum (§4.5) is still to be found. It costs a min-content query, and only a
    /// line with negative free space can need it — see `determine_flex_base_sizes`.
    min_pending: bool,
    /// A cross size known before layout: the definite one, or the stretch of a single-line
    /// container with a definite cross size (§9.8.1).
    known_cross: Option<f32>,
    base: f32,
    hypothetical: f32,
    /// The main size while flexing, then the used one.
    target: f32,
    frozen: bool,
    violation: f32,
    hypothetical_cross: f32,
    cross: f32,
    /// The first baseline from the border box's top; asked only of baseline-aligned items.
    baseline: Option<f32>,
    /// From the content box's main-start to the border box: logical.
    main_pos: f32,
    /// From the line's cross-start to the border box: logical.
    cross_pos: f32,
    /// The border box's origin in the container's border box: physical.
    position: (f32, f32),
}

struct Line {
    /// The line's items: `items[start..end]`.
    start: usize,
    end: usize,
    cross: f32,
    /// The deepest baseline of the line's baseline-aligned items, margin included: where their
    /// baselines line up.
    above: f32,
    /// From the content box's cross-start: logical.
    pos: f32,
}

/// The container's own constants, and the sizes the steps decide on the way.
struct Container {
    node: NodeId,
    axes: Axes,
    wrap: bool,
    /// `overflow: scroll`: the content may run past the box along its main axis.
    scrolls: bool,
    justify: JustifyContent,
    align_items: AlignItems,
    align_content: AlignContent,
    model: BoxModel,
    /// The padding plus the border, per edge.
    pb: Edges<f32>,
    /// The content box: known from the start, or fixed by a step (`fix_inner`).
    inner: Size<Option<f32>>,
    /// The content box's height is definite (`Input::height_definite`): the items' percent
    /// heights resolve against it. A height fixed from the content by a step is not.
    height_definite: bool,
    /// The space the items are offered (§9.2.2): the content box where known, else the
    /// container's own space less its margin, border and padding.
    avail: Size<Available>,
    gap_main: f32,
    gap_cross: f32,
}

/// The container's box under `input` (`compute` has missed its cache).
pub(super) fn compute_container(tree: &mut Tree, node: NodeId, input: &Input, measurer: &mut dyn Measurer) -> Output {
    let mut scratch = tree.scratch.pop().unwrap_or_default();
    let output = run(tree, node, input, &mut scratch, measurer);
    tree.scratch.push(scratch);
    output
}

fn run(tree: &mut Tree, node: NodeId, input: &Input, scratch: &mut Scratch, measurer: &mut dyn Measurer) -> Output {
    let Scratch { items, lines } = scratch;
    let mut c = Container::new(tree, node, input);
    // A question about the width alone, once the width is settled, is answered: nothing below
    // changes it. (The height in such an answer is not read, nor cached for a question that would.)
    if input.reads == Reads::Width && c.inner.width.is_some() {
        return Output { size: c.border_box(), baseline: None };
    }
    collect_items(tree, &c, items, input.mode);

    if c.inner.width.is_none() && (c.axes.row || !c.wrap) {
        let width = intrinsic_inline_size(tree, &c, items, measurer);
        c.fix_inner(true, width);
        if input.reads == Reads::Width {
            return Output { size: c.border_box(), baseline: None };
        }
        c.resolve_gaps(tree.styles[node.slot()].computed());
        // Collected again: the items' percents of the width resolve now.
        collect_items(tree, &c, items, input.mode);
    }

    determine_flex_base_sizes(tree, &c, items, measurer);
    collect_lines(&c, items, lines);
    if c.inner.main(c.axes.row).is_none() {
        determine_main_size(&mut c, items, lines);
    }
    for line in lines.iter() {
        resolve_flexible_lengths(tree, &c, &mut items[line.start..line.end], measurer);
    }
    determine_hypothetical_cross_sizes(tree, &c, items, measurer);
    determine_line_cross_sizes(&c, items, lines);
    determine_container_cross_size(&mut c, lines);
    stretch_lines(&c, lines);
    determine_used_cross_sizes(&c, items, lines);
    for line in lines.iter() {
        align_main(&c, &mut items[line.start..line.end]);
        align_cross(&c, &mut items[line.start..line.end], line);
    }
    align_lines(&c, lines);
    place_items(&c, items, lines);

    let size = c.border_box();
    let baseline = if input.reads == Reads::Baseline { first_baseline(tree, &c, items, lines, measurer) } else { None };
    if input.mode == Mode::Layout {
        layout_items(tree, &c, items, measurer);
    }
    Output { size, baseline }
}

impl Container {
    fn new(tree: &Tree, node: NodeId, input: &Input) -> Self {
        let style = &tree.styles[node.slot()].computed();
        let model = BoxModel::resolve(style, input.parent, input.content);
        let known = model.with_aspect(Size::new(
            input.known().width.or(model.size.width),
            input.known().height.or(model.size.height),
        ));
        let pb = model.pb();
        let inner = Size::new(
            known.width.map(|width| (width - pb.width).max(0.0)),
            known.height.map(|height| (height - pb.height).max(0.0)),
        );
        let offered = |horizontal: bool| match inner.get(horizontal) {
            Some(px) => Available::Definite(px),
            None => input.available().get(horizontal).shrink(model.margin.sum(horizontal) + pb.get(horizontal)),
        };
        let mut container = Self {
            node,
            axes: Axes::new(style.flex_direction, style.flex_wrap),
            wrap: style.flex_wrap != FlexWrap::NoWrap,
            scrolls: style.overflow == Overflow::Scroll,
            justify: style.justify_content,
            align_items: style.align_items,
            align_content: style.align_content,
            model,
            pb: model.padding_border(),
            inner,
            height_definite: model.size.height.is_some() || (input.known().height.is_some() && input.height_definite),
            avail: Size::new(offered(true), offered(false)),
            gap_main: 0.0,
            gap_cross: 0.0,
        };
        container.resolve_gaps(style);
        container
    }

    /// A percent gap is of the content box on its own axis; of an indefinite one it is 0 — and
    /// stays 0 once a height has come from the content (Chrome does not resolve it again).
    fn resolve_gaps(&mut self, style: &Style) {
        let base = self.percent_base();
        let between_columns = style.column_gap.px_or_zero(base.width).max(0.0);
        let between_rows = style.row_gap.px_or_zero(base.height).max(0.0);
        (self.gap_main, self.gap_cross) =
            if self.axes.row { (between_columns, between_rows) } else { (between_rows, between_columns) };
    }

    /// A content-box size a step decided: from now on the layout works in it, and the items are
    /// offered it. (A width so fixed is definite for percents too; a height is not.)
    fn fix_inner(&mut self, horizontal: bool, px: f32) {
        if horizontal {
            self.inner.width = Some(px);
            self.avail.width = Available::Definite(px);
        } else {
            self.inner.height = Some(px);
            self.avail.height = Available::Definite(px);
        }
    }

    fn inner_main(&self) -> f32 {
        self.inner.main(self.axes.row).unwrap_or(0.0)
    }

    fn inner_cross(&self) -> f32 {
        self.inner.cross(self.axes.row).unwrap_or(0.0)
    }

    fn border_box(&self) -> Size<f32> {
        let pb = self.model.pb();
        Size::new(self.inner.width.unwrap_or(0.0) + pb.width, self.inner.height.unwrap_or(0.0) + pb.height)
    }

    /// The content box as the items' percent base: its width once known, its height when definite.
    fn percent_base(&self) -> Size<Option<f32>> {
        Size::new(self.inner.width, self.inner.height.filter(|_| self.height_definite))
    }

    /// The height the container fixes for an item is definite (§9.8): a column's flexed main size
    /// when the column's own height is; a row's stretched cross size always (9.8.3); a height the
    /// item's content gave it never.
    fn item_height_definite(&self, item: &Item) -> bool {
        if self.axes.row { stretched(self, item) } else { self.height_definite }
    }
}

/// §9.1: the in-flow children are the items. A `display: none` child takes no part (and, in the
/// final layout, is hidden); an absolute one is placed after the flex layout (`absolute.rs`).
fn collect_items(tree: &mut Tree, c: &Container, items: &mut Vec<Item>, mode: Mode) {
    items.clear();
    let row = c.axes.row;
    let (cross_start, cross_end) = (c.axes.cross_start(), c.axes.cross_end());
    let mut child = tree.links[c.node.slot()].first_child;
    while let Some(id) = child {
        child = tree.links[id.slot()].next;
        let style = &tree.styles[id.slot()].computed();
        if style.display == Display::None {
            if mode == Mode::Layout {
                hide(tree, id);
            }
            continue;
        }
        if style.position == Position::Absolute {
            continue;
        }

        let model = BoxModel::resolve(style, c.percent_base(), None);
        let pb = model.pb();
        // A percent basis of an indefinite main size resolves to nothing and falls back to the
        // main size property, as `auto` does. A formula may compute below zero
        // (`calc(10px - 50%)`): CSS clamps a basis at 0, as it does every size.
        let basis = style
            .flex_basis
            .px(c.percent_base().main(row))
            .map(|px| px.max(0.0))
            .map(|px| if model.border_box { px } else { px + pb.main(row) });
        let min_length = if row { &style.min_size.width } else { &style.min_size.height };
        let min_pending = min_length.is_auto() && style.overflow == Overflow::Visible;
        let align = style.align_self.unwrap_or(c.align_items);
        let auto_cross = model.margin_auto.get(cross_start) || model.margin_auto.get(cross_end);
        // §9.8.1 asks for a DEFINITE cross size: a row's height that came from its content is
        // known in the final layout, but the items must be sized as they were when it was found.
        let stretched_known = align == AlignItems::Stretch && !c.wrap && !auto_cross;
        let known_cross = model.size.cross(row).or_else(|| {
            let line = c.percent_base().cross(row).filter(|_| stretched_known)?;
            Some(model.clamp(!row, line - model.margin.sum(!row)))
        });

        items.push(Item {
            node: id,
            model,
            align,
            grow: style.flex_grow.max(0.0),
            shrink: style.flex_shrink.max(0.0),
            basis,
            min_main: model.min.main(row).unwrap_or(0.0).max(pb.main(row)),
            min_pending,
            known_cross,
            base: 0.0,
            hypothetical: 0.0,
            target: 0.0,
            frozen: false,
            violation: 0.0,
            hypothetical_cross: 0.0,
            cross: 0.0,
            baseline: None,
            main_pos: 0.0,
            cross_pos: 0.0,
            position: (0.0, 0.0),
        });
    }
}

/// The container's inner width when nothing fixes it (css-sizing-3 §5): in a min- or max-content
/// space, its min- or max-content width; in a definite space, fit-content —
/// `min(max-content, max(min-content, space))` — within its min and max.
fn intrinsic_inline_size(tree: &mut Tree, c: &Container, items: &[Item], measurer: &mut dyn Measurer) -> f32 {
    let fit = match c.avail.width {
        Available::MaxContent => contributions(tree, c, items, Available::MaxContent, measurer),
        Available::MinContent => contributions(tree, c, items, Available::MinContent, measurer),
        Available::Definite(space) => {
            // The min-content width is needed only when the content does not fit.
            let max_content = contributions(tree, c, items, Available::MaxContent, measurer);
            if max_content <= space {
                max_content
            } else {
                contributions(tree, c, items, Available::MinContent, measurer).max(space)
            }
        }
    };
    let pb = c.model.pb().width;
    (c.model.clamp(true, fit + pb) - pb).max(0.0)
}

/// The items' min- or max-content width contributions (their size under that constraint, within
/// their min and max, plus margins), put together: side by side in a row — except a wrapping row's
/// min-content, which is its widest item — and the widest of them in a column. An item's
/// `flex-basis` plays no part, as in Chrome.
fn contributions(tree: &mut Tree, c: &Container, items: &[Item], sizing: Available, measurer: &mut dyn Measurer) -> f32 {
    let row = c.axes.row;
    let side_by_side = row && !(c.wrap && sizing == Available::MinContent);
    let mut total = 0.0f32;
    for item in items {
        let model = &item.model;
        let width = match model.size.width {
            Some(width) => width,
            None => {
                // The same question as the item's base size (`determine_flex_base_sizes`), so a
                // row asks its items once for both.
                let known_height = if row { item.known_cross } else { model.size.height };
                let offer = if sizing == Available::MaxContent { content_width(tree, c, item) } else { sizing };
                let input = Input {
                    ask: ask(Size::new(None, known_height), width_question(offer)),
                    parent: c.percent_base(),
                    mode: Mode::Size,
                    content: Some(Axis::Horizontal),
                    reads: Reads::Width,
                    height_definite: true,
                };
                model.clamp(true, compute(tree, item.node, input, measurer).size.width)
            }
        };
        let outer = width + model.margin.horizontal();
        total = if side_by_side { total + outer } else { total.max(outer) };
    }
    if side_by_side && items.len() > 1 {
        total += c.gap_main * (items.len() - 1) as f32;
    }
    total
}

/// §9.2.3: the flex base size — a definite `flex-basis`, else the main size property, else the
/// content's max-content size (its min and max ignored; a hugging leaf's: what it takes within the
/// row, `content_width`) — and the hypothetical main size: the base within min and max.
fn determine_flex_base_sizes(tree: &mut Tree, c: &Container, items: &mut [Item], measurer: &mut dyn Measurer) {
    let row = c.axes.row;
    for item in items.iter_mut() {
        let base = match item.basis.or(item.model.size.main(row)) {
            Some(px) => px,
            None => {
                let sizing = if row { content_width(tree, c, item) } else { Available::MaxContent };
                let input = Input {
                    ask: ask(Size::from_axes(row, None, item.known_cross), main_question(c, sizing)),
                    parent: c.percent_base(),
                    mode: Mode::Size,
                    content: Some(Axis::of(row)),
                    reads: if row { Reads::Width } else { Reads::Size },
                    height_definite: true,
                };
                compute(tree, item.node, input, measurer).size.main(row)
            }
        };
        // A border box holds its padding and border, whatever the basis says.
        item.base = base.max(item.model.pb().main(row));
        // The automatic minimum is never above the main size property nor above the max-content
        // size, so it can clamp nothing here unless an explicit flex-basis set the base.
        if item.min_pending && item.basis.is_some() {
            resolve_auto_min(tree, c, item, measurer);
        }
        item.hypothetical = clamp_main(c, item, item.base);
    }
}

/// §4.5, the automatic minimum size of an item whose overflow is visible: the content size
/// suggestion — its min-content main size, within its max (and, with an aspect ratio, within its
/// min and max cross sizes carried over) — capped by the specified size suggestion, a definite
/// main size.
fn resolve_auto_min(tree: &mut Tree, c: &Container, item: &mut Item, measurer: &mut dyn Measurer) {
    item.min_pending = false;
    let row = c.axes.row;
    // In the block axis the min- and max-content sizes are one — the height of the content at its
    // width — so a column asks the base size's question again, and the cache answers it.
    let sizing = if row { Available::MinContent } else { Available::MaxContent };
    let input = Input {
        ask: ask(Size::from_axes(row, None, item.known_cross), main_question(c, sizing)),
        parent: c.percent_base(),
        mode: Mode::Size,
        content: Some(Axis::of(row)),
        reads: if row { Reads::Width } else { Reads::Size },
        height_definite: true,
    };
    let mut min = compute(tree, item.node, input, measurer).size.main(row);
    let model = &item.model;
    if model.aspect.is_some() {
        if let Some(min_cross) = model.min.cross(row) {
            min = min.max(model.transfer(!row, min_cross));
        }
        if let Some(max_cross) = model.max.cross(row) {
            min = min.min(model.transfer(!row, max_cross));
        }
    }
    if let Some(size) = model.size.main(row) {
        min = min.min(size);
    }
    if let Some(max) = model.max.main(row) {
        min = min.min(max);
    }
    item.min_main = min.max(model.pb().main(row));
}

/// The item's main size within its min and max (the minimum wins).
fn clamp_main(c: &Container, item: &Item, size: f32) -> f32 {
    let size = match item.model.max.main(c.axes.row) {
        Some(max) => size.min(max),
        None => size,
    };
    size.max(item.min_main)
}

/// The cross space the container offers an item: its content box's, for the item's MARGIN box
/// (`Input::available`) — the item takes its own margins off it.
fn cross_space(c: &Container) -> Available {
    c.avail.cross(c.axes.row)
}

/// The space offered in a question about a width alone (`Reads::Width`): the width as given, and
/// NO height. Text is measured by the width it is offered; the height plays no part in a width
/// (CSS sizes the inline axis without it). So a Measurer never sees an offered height in a width
/// question — a height limit (`max-height`, a line clamp) is the embedder's own to apply — and a
/// question about a width is one question under every ancestor's height, not one per ancestor.
fn width_question(width: Available) -> Size<Available> {
    Size::new(width, Available::MaxContent)
}

/// What a container offers where CSS asks an item's max-content WIDTH — a row's flex base size, a
/// max-content contribution: `MaxContent`, except to a leaf that HUGS (`Tree::set_hug`). That one
/// is offered the container's available width — all of it, not what its siblings leave — and the
/// width its Measurer answers is its base, as Yoga measures a flex basis "at most" the container
/// (`computeFlexBasisForChild`). So a text that wraps inside 362 px and reports a longest line of
/// 306 is 306 wide, where CSS makes it 362 (its max-content shrunk to the offer); an ellipsized
/// line is as wide as what it kept. This is a deliberate departure from css-flexbox (§9.2.3 E
/// sizes a base as max-content): it is how app frameworks size text. Nothing else changes — a
/// definite width, a stretch, a grow or shrink share and a min all fix the width as they do for
/// any item, and the leaf is then measured at that width. As in Yoga, a row that scrolls offers
/// no bound along its main axis.
///
/// The one property that acts otherwise is `max-width`: for a hugging leaf it is a limit on the
/// OFFER, not a width to fill. Wherever its width is open — this question, a column's cross size,
/// an absolute box — the leaf is offered at most its max (`leaf::hug_limit`; Yoga's
/// `constrainMaxSizeForMode`) and keeps what the Measurer answers: a text cut by a max of 175 that
/// wraps to a longest line of 168 is 168 wide, where a leaf that does not hug fills the 175.
fn content_width(tree: &Tree, c: &Container, item: &Item) -> Available {
    // Read here, next to the question to the child (whose flags `compute` reads anyway), not for
    // every item collected: most never get this question.
    let meta = &tree.meta[item.node.slot()];
    if meta.measured && meta.hug && !(c.axes.row && c.scrolls) { c.avail.width } else { Available::MaxContent }
}

/// The space a row offers an item when it asks for the item's main size (`sizing` along the main
/// axis): a row asks for a width (`width_question`); a column asks for a height, at the width
/// it offers.
fn main_question(c: &Container, sizing: Available) -> Size<Available> {
    if c.axes.row { width_question(sizing) } else { Size::new(cross_space(c), sizing) }
}

/// The outer (margin-box) main sizes of a line's items, `size` of each, plus the gaps between them.
fn outer_main_sum(c: &Container, items: &[Item], size: impl Fn(&Item) -> f32) -> f32 {
    let sizes: f32 = items.iter().map(|item| size(item) + item.model.margin.sum(c.axes.row)).sum();
    sizes + c.gap_main * items.len().saturating_sub(1) as f32
}

/// §9.3.5: a single-line container puts every item on one line; a multi-line one starts a new
/// line before the item that would overflow its inner main size. A column whose height is still
/// open breaks at its max-height, or never.
fn collect_lines(c: &Container, items: &[Item], lines: &mut Vec<Line>) {
    lines.clear();
    let row = c.axes.row;
    let limit = if c.wrap {
        c.inner.main(row).or_else(|| c.model.max.main(row).map(|max| (max - c.model.pb().main(row)).max(0.0)))
    } else {
        None
    };
    let mut start = 0;
    let mut used = 0.0;
    for (index, item) in items.iter().enumerate() {
        let outer = item.hypothetical + item.model.margin.sum(row);
        if index > start && limit.is_some_and(|limit| used + c.gap_main + outer > limit + FIT_EPSILON) {
            lines.push(Line::new(start, index));
            start = index;
            used = outer;
        } else {
            used += if index > start { c.gap_main + outer } else { outer };
        }
    }
    lines.push(Line::new(start, items.len()));
}

impl Line {
    fn new(start: usize, end: usize) -> Self {
        Self { start, end, cross: 0.0, above: 0.0, pos: 0.0 }
    }
}

/// §9.3.4 for a main size still open — a column of auto height: its longest line, within its min
/// and max.
fn determine_main_size(c: &mut Container, items: &[Item], lines: &[Line]) {
    let row = c.axes.row;
    let longest = lines
        .iter()
        .map(|line| outer_main_sum(c, &items[line.start..line.end], |item| item.hypothetical))
        .fold(0.0, f32::max);
    let pb = c.model.pb().main(row);
    c.fix_inner(row, (c.model.clamp(row, longest + pb) - pb).max(0.0));
}

/// §9.7: the line's items grow (or shrink) from their base sizes to fill its inner main size, in
/// proportion to their flex factors; an item that would cross its min or max is frozen there and
/// the rest flex again.
fn resolve_flexible_lengths(tree: &mut Tree, c: &Container, items: &mut [Item], measurer: &mut dyn Measurer) {
    if items.is_empty() {
        return;
    }
    let row = c.axes.row;
    let inner = c.inner_main();

    // 1. The used flex factor: grow when the hypothetical sizes leave space, else shrink.
    let growing = outer_main_sum(c, items, |item| item.hypothetical) < inner;
    let factor = |item: &Item| if growing { item.grow } else { item.shrink };

    // 2. Size inflexible items: no factor, or a base already past the hypothetical size.
    for item in items.iter_mut() {
        item.target = item.hypothetical;
        item.frozen = factor(item) == 0.0
            || (growing && item.base > item.hypothetical)
            || (!growing && item.base < item.hypothetical);
    }

    // 3. The initial free space: frozen items at their target, the others at their base.
    let free_space = |items: &[Item]| {
        inner - outer_main_sum(c, items, |item| if item.frozen { item.target } else { item.base })
    };
    let initial = free_space(items);

    // Only negative free space takes an item under its base size — and so near a deferred
    // automatic minimum (a line that exactly fits, as every content-sized one does, needs none).
    if initial < 0.0 {
        for item in items.iter_mut().filter(|item| item.min_pending && !item.frozen) {
            resolve_auto_min(tree, c, item, measurer);
        }
    }

    // 4. Loop until every item is frozen.
    while items.iter().any(|item| !item.frozen) {
        // b. The remaining free space; flex factors summing under 1 take only that share of it.
        let factors: f32 = items.iter().filter(|item| !item.frozen).map(factor).sum();
        let mut free = free_space(items);
        if factors < 1.0 && (initial * factors).abs() < free.abs() {
            free = initial * factors;
        }

        // c. Distribute it: growing by the flex-grow factor, shrinking by the flex-shrink factor
        //    scaled by the inner (content-box) base size, so a large item gives up more.
        let scaled = |item: &Item| item.shrink * (item.base - item.model.pb().main(row)).max(0.0);
        let scaled_sum: f32 = items.iter().filter(|item| !item.frozen).map(scaled).sum();
        for item in items.iter_mut().filter(|item| !item.frozen) {
            let share = if growing {
                item.grow / factors
            } else if scaled_sum > 0.0 {
                scaled(item) / scaled_sum
            } else {
                0.0
            };
            item.target = item.base + free * share;
        }

        // d. Fix min/max violations.
        let mut total = 0.0;
        for item in items.iter_mut().filter(|item| !item.frozen) {
            let clamped = clamp_main(c, item, item.target);
            item.violation = clamped - item.target;
            item.target = clamped;
            total += item.violation;
        }

        // e. Freeze over-flexed items: all of them when nothing was violated, else the side of
        //    the larger violation. A NaN total (an infinite flex factor: its share is inf / inf)
        //    is no violation either way: everything freezes, or no iteration would ever freeze
        //    anything and the loop would not end.
        // (Not `total != 0.0`: that is true for a NaN.)
        let either_way = total.abs() > 0.0;
        for item in items.iter_mut().filter(|item| !item.frozen) {
            item.frozen = !either_way || (total > 0.0 && item.violation > 0.0) || (total < 0.0 && item.violation < 0.0);
        }
    }
}

/// §9.4.7: each item's cross size at its used main size — a known one as it is, else the
/// content's, fit into the space offered (stretching comes later, in step 11).
fn determine_hypothetical_cross_sizes(tree: &mut Tree, c: &Container, items: &mut [Item], measurer: &mut dyn Measurer) {
    let row = c.axes.row;
    for item in items.iter_mut() {
        let baseline = baseline_aligned(c, item);
        let cross = item.known_cross.map_or(Ask::Offer(cross_space(c)), Ask::Known);
        let input = Input {
            ask: Size::from_axes(row, Ask::Known(item.target), cross),
            parent: c.percent_base(),
            mode: Mode::Size,
            content: None,
            // A column item's cross size is its width; a row item's is its height (and with it,
            // maybe, its baseline).
            reads: match (row, baseline) {
                (false, _) => Reads::Width,
                (true, false) => Reads::Size,
                (true, true) => Reads::Baseline,
            },
            // A row item's known height is a stretch or its own; a column item's is the flexed one.
            height_definite: row || c.height_definite,
        };
        match item.known_cross {
            Some(cross) => {
                item.hypothetical_cross = cross;
                if baseline {
                    item.baseline = compute(tree, item.node, input, measurer).baseline;
                }
            }
            None => {
                let out = compute(tree, item.node, input, measurer);
                item.hypothetical_cross = out.size.cross(row);
                item.baseline = out.baseline;
            }
        }
    }
}

/// The item takes part in baseline alignment. Only a row's items do — a column's baselines run
/// across its cross axis — and not in `wrap-reverse` (where the line's cross-start is its bottom;
/// such an item is placed as `flex-start`), nor with an `auto` cross margin, which wins.
fn baseline_aligned(c: &Container, item: &Item) -> bool {
    c.axes.row
        && !c.axes.cross_reverse
        && item.align == AlignItems::Baseline
        && !item.model.margin_auto.top
        && !item.model.margin_auto.bottom
}

/// An item that fills its line's cross size: `align-self: stretch`, an auto cross size, no
/// `auto` cross margin.
fn stretched(c: &Container, item: &Item) -> bool {
    item.align == AlignItems::Stretch
        && item.model.size.cross(c.axes.row).is_none()
        && !item.model.margin_auto.get(c.axes.cross_start())
        && !item.model.margin_auto.get(c.axes.cross_end())
}

/// §9.4.8: a single-line container with a definite cross size has one line of that size. Other
/// lines are as large as their largest item, the baseline-aligned ones measured as the group they
/// form (the deepest baseline above, the deepest descent below); a single line then stays within
/// the container's min and max.
fn determine_line_cross_sizes(c: &Container, items: &[Item], lines: &mut [Line]) {
    let row = c.axes.row;
    for line in lines.iter_mut() {
        let (mut largest, mut above, mut below) = (0.0f32, 0.0f32, 0.0f32);
        for item in &items[line.start..line.end] {
            let outer = item.hypothetical_cross + item.model.margin.sum(!row);
            if baseline_aligned(c, item) {
                let baseline = item.model.margin.top + item.baseline.unwrap_or(item.hypothetical_cross);
                above = above.max(baseline);
                below = below.max(outer - baseline);
            } else {
                largest = largest.max(outer);
            }
        }
        line.above = above;
        line.cross = largest.max(above + below);
    }
    if !c.wrap {
        let pb = c.model.pb().cross(row);
        lines[0].cross = match c.inner.cross(row) {
            Some(cross) => cross,
            None => (c.model.clamp(!row, lines[0].cross + pb) - pb).max(0.0),
        };
    }
}

/// The lines' cross sizes plus the gaps between them.
fn lines_cross_sum(c: &Container, lines: &[Line]) -> f32 {
    lines.iter().map(|line| line.cross).sum::<f32>() + c.gap_cross * lines.len().saturating_sub(1) as f32
}

/// §9.4.15: a cross size still open is the lines', within the container's min and max. (Decided
/// here, before the lines are stretched, so that `align-content` works in the final size — a
/// min-height included.)
fn determine_container_cross_size(c: &mut Container, lines: &[Line]) {
    let row = c.axes.row;
    if c.inner.cross(row).is_none() {
        let pb = c.model.pb().cross(row);
        let cross = (c.model.clamp(!row, lines_cross_sum(c, lines) + pb) - pb).max(0.0);
        c.fix_inner(!row, cross);
    }
}

/// §9.4.9 `align-content: stretch`: a multi-line container's lines share its free cross space.
fn stretch_lines(c: &Container, lines: &mut [Line]) {
    if !c.wrap || c.align_content != AlignContent::Stretch {
        return;
    }
    let free = c.inner_cross() - lines_cross_sum(c, lines);
    if free > 0.0 {
        let share = free / lines.len() as f32;
        for line in lines.iter_mut() {
            line.cross += share;
        }
    }
}

/// §9.4.11: a stretched item fills its line (within its min and max); any other keeps its
/// hypothetical cross size.
fn determine_used_cross_sizes(c: &Container, items: &mut [Item], lines: &[Line]) {
    let row = c.axes.row;
    for line in lines {
        for item in &mut items[line.start..line.end] {
            item.cross = if stretched(c, item) {
                item.model.clamp(!row, line.cross - item.model.margin.sum(!row))
            } else {
                item.hypothetical_cross
            };
        }
    }
}

/// §9.5.12–13: the `auto` main margins of a line take its positive free space; what they leave
/// (or a negative free space) `justify-content` distributes.
fn align_main(c: &Container, items: &mut [Item]) {
    if items.is_empty() {
        return;
    }
    let (start, end) = (c.axes.main_start(), c.axes.main_end());
    let mut free = c.inner_main() - outer_main_sum(c, items, |item| item.target);
    let autos: usize = items
        .iter()
        .map(|item| item.model.margin_auto.get(start) as usize + item.model.margin_auto.get(end) as usize)
        .sum();
    if autos > 0 {
        let share = free.max(0.0) / autos as f32;
        for item in items.iter_mut() {
            for side in [start, end] {
                if item.model.margin_auto.get(side) {
                    item.model.margin.set(side, share);
                }
            }
        }
        free = free.min(0.0);
    }
    let (mut pos, between) = distribute(justify(c.justify), free, items.len(), c.axes.main_reverse);
    for item in items.iter_mut() {
        pos += item.model.margin.get(start);
        item.main_pos = pos;
        pos += item.target + item.model.margin.get(end) + c.gap_main + between;
    }
}

/// §9.6.13–14: in its line, an item with an `auto` cross margin is centred or pushed by it
/// (overflowing, it keeps to the cross-start); a baseline-aligned item lines its baseline up with
/// the group's; any other is placed by `align-self`.
fn align_cross(c: &Container, items: &mut [Item], line: &Line) {
    let (start, end) = (c.axes.cross_start(), c.axes.cross_end());
    for item in items.iter_mut() {
        let baseline = baseline_aligned(c, item);
        let auto = (item.model.margin_auto.get(start), item.model.margin_auto.get(end));
        let margin = &mut item.model.margin;
        let free = line.cross - item.cross - margin.get(start) - margin.get(end);
        item.cross_pos = if auto.0 || auto.1 {
            if free > 0.0 {
                match auto {
                    (true, true) => {
                        margin.set(start, free / 2.0);
                        margin.set(end, free / 2.0);
                    }
                    (true, false) => margin.set(start, free),
                    _ => margin.set(end, free),
                }
            }
            margin.get(start)
        } else if baseline {
            line.above - item.baseline.unwrap_or(item.cross)
        } else {
            let share = match item.align {
                AlignItems::FlexEnd => free,
                AlignItems::Center => free / 2.0,
                _ => 0.0,
            };
            margin.get(start) + share
        };
    }
}

/// §9.6.16: `align-content` places a multi-line container's lines in its cross space (a single
/// line already fills it).
fn align_lines(c: &Container, lines: &mut [Line]) {
    let free = c.inner_cross() - lines_cross_sum(c, lines);
    let how = if !c.wrap {
        Distribute::Start
    } else {
        match c.align_content {
            AlignContent::FlexStart | AlignContent::Stretch => Distribute::Start,
            AlignContent::FlexEnd => Distribute::End,
            AlignContent::Center => Distribute::Center,
            AlignContent::SpaceBetween => Distribute::Between,
            AlignContent::SpaceAround => Distribute::Around,
            AlignContent::SpaceEvenly => Distribute::Evenly,
        }
    };
    let (mut pos, between) = distribute(how, free, lines.len(), c.axes.cross_reverse);
    for line in lines.iter_mut() {
        line.pos = pos;
        pos += line.cross + c.gap_cross + between;
    }
}

/// The logical positions turned physical: each border box's origin in the container's border box.
/// A reversed axis is a flip of the content box, here and nowhere else.
fn place_items(c: &Container, items: &mut [Item], lines: &[Line]) {
    let (inner_main, inner_cross) = (c.inner_main(), c.inner_cross());
    for line in lines {
        for item in &mut items[line.start..line.end] {
            let main = if c.axes.main_reverse { inner_main - item.main_pos - item.target } else { item.main_pos };
            let cross = line.pos + item.cross_pos;
            let cross = if c.axes.cross_reverse { inner_cross - cross - item.cross } else { cross };
            let (x, y) = if c.axes.row { (main, cross) } else { (cross, main) };
            item.position = (c.pb.left + x, c.pb.top + y);
        }
    }
}

/// §8.5: the container's first baseline is that of its first line's baseline-aligned items, else
/// that of its first item (synthesized from the item's bottom edge when it has none); without
/// items there is none.
fn first_baseline(
    tree: &mut Tree,
    c: &Container,
    items: &[Item],
    lines: &[Line],
    measurer: &mut dyn Measurer,
) -> Option<f32> {
    let line = lines.first().filter(|line| line.end > line.start)?;
    let aligned = (line.start..line.end).find(|&index| baseline_aligned(c, &items[index]));
    let item = &items[aligned.unwrap_or(line.start)];
    let size = Size::from_axes(c.axes.row, item.target, item.cross);
    let baseline = match aligned {
        Some(_) => item.baseline,
        None => {
            let input = Input {
                mode: Mode::Size,
                reads: Reads::Baseline,
                ..Input::layout(size, c.percent_base(), c.item_height_definite(item))
            };
            compute(tree, item.node, input, measurer).baseline
        }
    };
    Some(item.position.1 + baseline.unwrap_or(size.height))
}

/// The final layout: each item laid out at its used size and placed (plus its relative shift),
/// then the absolute children, then the container's own box.
fn layout_items(tree: &mut Tree, c: &Container, items: &[Item], measurer: &mut dyn Measurer) {
    let border = c.model.border;
    let mut extent = Size::new(0.0f32, 0.0f32);
    for item in items {
        let size = Size::from_axes(c.axes.row, item.target, item.cross);
        compute(tree, item.node, Input::layout(size, c.percent_base(), c.item_height_definite(item)), measurer);
        let slot = item.node.slot();
        let shift = relative_offset(tree.styles[slot].computed(), c.percent_base());
        let (x, y) = (item.position.0 + shift.0, item.position.1 + shift.1);
        let rect = &mut tree.raw[slot].rect;
        rect.x = x;
        rect.y = y;
        extent.width = extent.width.max(x - border.left + size.width + item.model.margin.right);
        extent.height = extent.height.max(y - border.top + size.height + item.model.margin.bottom);
    }
    let size = c.border_box();
    let frame = Frame {
        size,
        border,
        padding: c.model.padding,
        axes: c.axes,
        justify: c.justify,
        align_items: c.align_items,
    };
    absolute::layout_children(tree, c.node, &frame, &mut extent, measurer);
    finish(tree, c.node, &c.model, size, extent);
}

/// How free space is shared among `n` boxes (the items of a line, the lines of a container).
#[derive(Clone, Copy)]
enum Distribute {
    Start,
    End,
    Center,
    Between,
    Around,
    Evenly,
}

fn justify(justify: JustifyContent) -> Distribute {
    match justify {
        JustifyContent::FlexStart => Distribute::Start,
        JustifyContent::FlexEnd => Distribute::End,
        JustifyContent::Center => Distribute::Center,
        JustifyContent::SpaceBetween => Distribute::Between,
        JustifyContent::SpaceAround => Distribute::Around,
        JustifyContent::SpaceEvenly => Distribute::Evenly,
    }
}

/// Where packing starts (from the logical start), and the extra space between neighbours. The
/// distributions fall back as css-align-3 §4.3 says, and Chrome does: `space-between` to
/// `flex-start`; `space-around` and `space-evenly` to SAFE center — centred while there is room;
/// overflowing, at the writing mode's start (the left / top edge), which on a `reversed` axis is
/// the logical end. (css-flexbox-1's older text has them overflow both ways, as `center` does.)
fn distribute(how: Distribute, free: f32, n: usize, reversed: bool) -> (f32, f32) {
    let n = n.max(1) as f32;
    match how {
        Distribute::Start => (0.0, 0.0),
        Distribute::End => (free, 0.0),
        Distribute::Center => (free / 2.0, 0.0),
        Distribute::Between if free > 0.0 && n > 1.0 => (0.0, free / (n - 1.0)),
        Distribute::Between => (0.0, 0.0),
        Distribute::Around if free > 0.0 => (free / n / 2.0, free / n),
        Distribute::Evenly if free > 0.0 => (free / (n + 1.0), free / (n + 1.0)),
        Distribute::Around | Distribute::Evenly => (if reversed { free } else { 0.0 }, 0.0),
    }
}
