//! The flex algorithm, one feature at a time. Every expected box is worked out by hand from
//! css-flexbox-1 (the arithmetic is in the comments), at scale 1 with whole px, so snapping plays
//! no part.

use std::collections::HashMap;

use souple::{
    AlignContent, AlignItems, Available, BoxSizing, Display, Edges, FlexDirection, FlexWrap, JustifyContent, Tree,
    Length, MeasureInput, MeasureOutput, Measurer, NoMeasure, NodeId, Overflow, Position, Size, Style, Viewport,
};

fn px(value: f32) -> Length {
    Length::Px(value)
}

fn pct(value: f32) -> Length {
    Length::Percent(value)
}

fn sized(width: f32, height: f32) -> Style {
    Style { size: Size::new(px(width), px(height)), ..Style::default() }
}

fn wide(width: f32) -> Style {
    Style { size: Size::new(px(width), Length::Auto), ..Style::default() }
}

fn tall(height: f32) -> Style {
    Style { size: Size::new(Length::Auto, px(height)), ..Style::default() }
}

fn column() -> Style {
    Style { flex_direction: FlexDirection::Column, ..Style::default() }
}

/// A root in a viewport of `width` × `height` (`None` = unbounded).
fn root(tree: &mut Tree, width: Option<f32>, height: Option<f32>, style: Style) -> NodeId {
    let root = tree.create_node();
    tree.set_style(root, style);
    tree.set_viewport(root, Viewport::Size(Size::new(width, height)));
    root
}

fn screen(tree: &mut Tree, width: f32, height: f32, style: Style) -> NodeId {
    root(tree, Some(width), Some(height), style)
}

fn child(tree: &mut Tree, parent: NodeId, style: Style) -> NodeId {
    let node = tree.create_node();
    tree.set_style(node, style);
    tree.append(parent, node);
    node
}

/// x, y, width, height.
fn rect(tree: &Tree, node: NodeId) -> [f32; 4] {
    let rect = tree.layout(node).rect;
    [rect.x, rect.y, rect.width, rect.height]
}

/// Text made of words: `total` px on one line, its longest word `word` px. Laid out at a width it
/// takes `ceil(total / width)` lines of 20 px, baseline 16 px into each line. Every call is logged.
#[derive(Default)]
struct Text {
    texts: HashMap<NodeId, (f32, f32)>,
    calls: Vec<NodeId>,
}

impl Text {
    fn add(&mut self, tree: &mut Tree, parent: NodeId, style: Style, total: f32, word: f32) -> NodeId {
        let node = child(tree, parent, style);
        tree.set_measured(node, true);
        self.texts.insert(node, (total, word));
        node
    }
}

impl Measurer for Text {
    fn measure(&mut self, node: NodeId, input: MeasureInput) -> MeasureOutput {
        self.calls.push(node);
        let (total, word) = self.texts[&node];
        let width = input.known.width.unwrap_or(match input.available.width {
            Available::MinContent => word,
            Available::MaxContent => total,
            Available::Definite(space) => total.min(space).max(word),
        });
        let lines = (total / width.max(word) - 1e-4).ceil().max(1.0);
        MeasureOutput { size: Size::new(width, lines * 20.0), baseline: Some(16.0) }
    }
}

// ── direction ───────────────────────────────────────────────────────────────────────────────

#[test]
fn a_row_places_items_side_by_side_and_stretches_them() {
    let mut tree = Tree::new();
    let root = screen(&mut tree, 300.0, 100.0, Style::default());
    let a = child(&mut tree, root, wide(50.0));
    let b = child(&mut tree, root, wide(100.0));
    tree.compute_layout(&mut NoMeasure);
    assert_eq!(rect(&tree, root), [0.0, 0.0, 300.0, 100.0]);
    assert_eq!(rect(&tree, a), [0.0, 0.0, 50.0, 100.0]);
    assert_eq!(rect(&tree, b), [50.0, 0.0, 100.0, 100.0]);
}

#[test]
fn a_column_stacks_items() {
    let mut tree = Tree::new();
    let root = screen(&mut tree, 100.0, 300.0, column());
    let a = child(&mut tree, root, tall(50.0));
    let b = child(&mut tree, root, tall(100.0));
    tree.compute_layout(&mut NoMeasure);
    assert_eq!(rect(&tree, a), [0.0, 0.0, 100.0, 50.0]);
    assert_eq!(rect(&tree, b), [0.0, 50.0, 100.0, 100.0]);
}

#[test]
fn reversed_directions_start_at_the_far_edge() {
    let mut tree = Tree::new();
    let row = screen(&mut tree, 300.0, 100.0, Style { flex_direction: FlexDirection::RowReverse, ..Style::default() });
    let a = child(&mut tree, row, wide(50.0));
    let b = child(&mut tree, row, wide(100.0));
    let col = screen(&mut tree, 100.0, 300.0, Style { flex_direction: FlexDirection::ColumnReverse, ..Style::default() });
    let c = child(&mut tree, col, tall(50.0));
    let d = child(&mut tree, col, tall(100.0));
    tree.compute_layout(&mut NoMeasure);
    // Main-start is the right (bottom) edge: 300 - 50 = 250, then 250 - 100 = 150.
    assert_eq!(rect(&tree, a), [250.0, 0.0, 50.0, 100.0]);
    assert_eq!(rect(&tree, b), [150.0, 0.0, 100.0, 100.0]);
    assert_eq!(rect(&tree, c), [0.0, 250.0, 100.0, 50.0]);
    assert_eq!(rect(&tree, d), [0.0, 150.0, 100.0, 100.0]);
}

// ── flexible lengths ────────────────────────────────────────────────────────────────────────

#[test]
fn grow_shares_the_free_space_by_factor() {
    let mut tree = Tree::new();
    let root = screen(&mut tree, 300.0, 100.0, Style::default());
    let flex = |grow: f32| Style { flex_grow: grow, flex_basis: px(0.0), ..Style::default() };
    let a = child(&mut tree, root, flex(1.0));
    let b = child(&mut tree, root, flex(2.0));
    // Another row: bases 50 and 100, free 150, an equal share each.
    let other = screen(&mut tree, 300.0, 100.0, Style::default());
    let c = child(&mut tree, other, Style { flex_grow: 1.0, ..wide(50.0) });
    let d = child(&mut tree, other, Style { flex_grow: 1.0, ..wide(100.0) });
    tree.compute_layout(&mut NoMeasure);
    assert_eq!(rect(&tree, a), [0.0, 0.0, 100.0, 100.0]);
    assert_eq!(rect(&tree, b), [100.0, 0.0, 200.0, 100.0]);
    assert_eq!(rect(&tree, c)[2], 125.0);
    assert_eq!(rect(&tree, d), [125.0, 0.0, 175.0, 100.0]);
}

#[test]
fn shrink_is_weighted_by_the_base_size() {
    let mut tree = Tree::new();
    let root = screen(&mut tree, 300.0, 100.0, Style::default());
    // 200 + 400 overflow 300 by 300: shrink factors 1 × 200 and 1 × 400 take 100 and 200.
    let a = child(&mut tree, root, wide(200.0));
    let b = child(&mut tree, root, wide(400.0));
    tree.compute_layout(&mut NoMeasure);
    assert_eq!(rect(&tree, a), [0.0, 0.0, 100.0, 100.0]);
    assert_eq!(rect(&tree, b), [100.0, 0.0, 200.0, 100.0]);
}

#[test]
fn a_max_violation_freezes_the_item_and_the_rest_flex_again() {
    let mut tree = Tree::new();
    let root = screen(&mut tree, 300.0, 100.0, Style::default());
    let flex = Style { flex_grow: 1.0, flex_basis: px(0.0), ..Style::default() };
    // 150 each; a is clamped to 50 (a negative violation) and frozen; b takes 300 - 50.
    let a = child(&mut tree, root, Style { max_size: Size::new(px(50.0), Length::Auto), ..flex.clone() });
    let b = child(&mut tree, root, flex);
    tree.compute_layout(&mut NoMeasure);
    assert_eq!(rect(&tree, a)[2], 50.0);
    assert_eq!(rect(&tree, b), [50.0, 0.0, 250.0, 100.0]);
}

#[test]
fn a_min_violation_freezes_the_item_and_the_rest_shrink_again() {
    let mut tree = Tree::new();
    let root = screen(&mut tree, 100.0, 100.0, Style::default());
    // 100 + 100 in 100: -50 each; a is clamped up to 80 and frozen; b takes 100 - 80 = 20.
    let a = child(&mut tree, root, Style { min_size: Size::new(px(80.0), Length::Auto), ..wide(100.0) });
    let b = child(&mut tree, root, wide(100.0));
    tree.compute_layout(&mut NoMeasure);
    assert_eq!(rect(&tree, a)[2], 80.0);
    assert_eq!(rect(&tree, b), [80.0, 0.0, 20.0, 100.0]);
}

#[test]
fn the_automatic_minimum_is_the_content_unless_overflow_clips() {
    let mut tree = Tree::new();
    let mut text = Text::default();
    let root = screen(&mut tree, 100.0, 100.0, Style::default());
    // a: max-content 100, min-content 80. 100 + 100 in 100: -50 each, a stops at 80, b gets 20.
    let a = text.add(&mut tree, root, Style::default(), 100.0, 80.0);
    let b = child(&mut tree, root, wide(100.0));
    tree.compute_layout(&mut text);
    assert_eq!(rect(&tree, a)[2], 80.0);
    assert_eq!(rect(&tree, b), [80.0, 0.0, 20.0, 100.0]);

    // A scroll container's automatic minimum is 0: both shrink to 50.
    tree.update_style(a, |style| style.overflow = Overflow::Scroll);
    tree.compute_layout(&mut text);
    assert_eq!(rect(&tree, a)[2], 50.0);
    assert_eq!(rect(&tree, b), [50.0, 0.0, 50.0, 100.0]);
}

#[test]
fn a_column_item_does_not_shrink_under_its_content_height() {
    let mut tree = Tree::new();
    let mut text = Text::default();
    let root = screen(&mut tree, 100.0, 100.0, column());
    // a: 300 px of text at width 100 = 3 lines = 60. 60 + 100 in 100 shrinks a under 60 → frozen at
    // 60; b takes 100 - 60 = 40.
    let a = text.add(&mut tree, root, Style::default(), 300.0, 50.0);
    let b = child(&mut tree, root, tall(100.0));
    tree.compute_layout(&mut text);
    assert_eq!(rect(&tree, a), [0.0, 0.0, 100.0, 60.0]);
    assert_eq!(rect(&tree, b), [0.0, 60.0, 100.0, 40.0]);
}

// ── wrapping ────────────────────────────────────────────────────────────────────────────────

#[test]
fn wrap_breaks_lines_and_align_content_places_them() {
    let mut tree = Tree::new();
    let wrap = |align_content| Style { flex_wrap: FlexWrap::Wrap, align_content, ..Style::default() };
    // 80 + 80 fit in 200, a third does not: lines [a b] [c], each 50 tall.
    let start = screen(&mut tree, 200.0, 200.0, wrap(AlignContent::FlexStart));
    let s: Vec<NodeId> = (0..3).map(|_| child(&mut tree, start, sized(80.0, 50.0))).collect();
    // Stretch: the free 100 is shared, each line 100 tall, the second at 100.
    let stretch = screen(&mut tree, 200.0, 200.0, wrap(AlignContent::Stretch));
    let t: Vec<NodeId> = (0..3).map(|_| child(&mut tree, stretch, sized(80.0, 50.0))).collect();
    // Center: the two lines (100) centred in 200.
    let center = screen(&mut tree, 200.0, 200.0, wrap(AlignContent::Center));
    let u: Vec<NodeId> = (0..3).map(|_| child(&mut tree, center, sized(80.0, 50.0))).collect();
    // Space-between: first line at 0, the last at 200 - 50.
    let between = screen(&mut tree, 200.0, 200.0, wrap(AlignContent::SpaceBetween));
    let v: Vec<NodeId> = (0..3).map(|_| child(&mut tree, between, sized(80.0, 50.0))).collect();
    tree.compute_layout(&mut NoMeasure);
    assert_eq!(rect(&tree, s[0]), [0.0, 0.0, 80.0, 50.0]);
    assert_eq!(rect(&tree, s[1]), [80.0, 0.0, 80.0, 50.0]);
    assert_eq!(rect(&tree, s[2]), [0.0, 50.0, 80.0, 50.0]);
    assert_eq!(rect(&tree, t[2])[1], 100.0);
    assert_eq!(rect(&tree, u[0])[1], 50.0);
    assert_eq!(rect(&tree, u[2])[1], 100.0);
    assert_eq!(rect(&tree, v[2])[1], 150.0);
}

#[test]
fn wrap_reverse_stacks_lines_from_the_bottom() {
    let mut tree = Tree::new();
    let style = Style { flex_wrap: FlexWrap::WrapReverse, align_content: AlignContent::FlexStart, ..Style::default() };
    let root = screen(&mut tree, 200.0, 200.0, style);
    let items: Vec<NodeId> = (0..3).map(|_| child(&mut tree, root, sized(80.0, 50.0))).collect();
    tree.compute_layout(&mut NoMeasure);
    // Cross-start is the bottom: the first line at 200 - 50, the second above it.
    assert_eq!(rect(&tree, items[0]), [0.0, 150.0, 80.0, 50.0]);
    assert_eq!(rect(&tree, items[1]), [80.0, 150.0, 80.0, 50.0]);
    assert_eq!(rect(&tree, items[2]), [0.0, 100.0, 80.0, 50.0]);
}

#[test]
fn gaps_separate_items_and_lines() {
    let mut tree = Tree::new();
    let row = screen(&mut tree, 300.0, 100.0, Style { column_gap: px(10.0), ..Style::default() });
    let a: Vec<NodeId> = (0..3).map(|_| child(&mut tree, row, wide(50.0))).collect();
    // 100 + 10 + 100 > 200: one item per line, the lines 20 apart.
    let style = Style {
        flex_wrap: FlexWrap::Wrap,
        align_content: AlignContent::FlexStart,
        column_gap: px(10.0),
        row_gap: px(20.0),
        ..Style::default()
    };
    let wrap = screen(&mut tree, 200.0, 300.0, style);
    let b: Vec<NodeId> = (0..3).map(|_| child(&mut tree, wrap, sized(100.0, 50.0))).collect();
    tree.compute_layout(&mut NoMeasure);
    assert_eq!([rect(&tree, a[0])[0], rect(&tree, a[1])[0], rect(&tree, a[2])[0]], [0.0, 60.0, 120.0]);
    assert_eq!([rect(&tree, b[0])[1], rect(&tree, b[1])[1], rect(&tree, b[2])[1]], [0.0, 70.0, 140.0]);
}

// ── alignment ───────────────────────────────────────────────────────────────────────────────

fn justified(justify: JustifyContent, width: f32, items: &[f32]) -> Vec<f32> {
    let mut tree = Tree::new();
    let root = screen(&mut tree, width, 100.0, Style { justify_content: justify, ..Style::default() });
    let nodes: Vec<NodeId> =
        items.iter().map(|w| child(&mut tree, root, Style { flex_shrink: 0.0, ..wide(*w) })).collect();
    tree.compute_layout(&mut NoMeasure);
    nodes.iter().map(|node| rect(&tree, *node)[0]).collect()
}

#[test]
fn justify_content_distributes_the_free_space() {
    use JustifyContent::*;
    // 3 × 50 in 300: free 150.
    assert_eq!(justified(FlexEnd, 300.0, &[50.0; 3]), [150.0, 200.0, 250.0]);
    assert_eq!(justified(Center, 300.0, &[50.0; 3]), [75.0, 125.0, 175.0]);
    assert_eq!(justified(SpaceBetween, 300.0, &[50.0; 3]), [0.0, 125.0, 250.0]);
    // 50 around each item: 25 at the ends.
    assert_eq!(justified(SpaceAround, 300.0, &[50.0; 3]), [25.0, 125.0, 225.0]);
    // 3 × 60 in 300: free 120, 30 in each of the 4 gaps.
    assert_eq!(justified(SpaceEvenly, 300.0, &[60.0; 3]), [30.0, 120.0, 210.0]);
    // Negative free space (-60): space-between falls back to flex-start, space-around and
    // space-evenly to SAFE center (css-align-3) — the start, once overflowing. Center itself is
    // unsafe: it overflows both ways.
    assert_eq!(justified(SpaceBetween, 100.0, &[80.0; 2]), [0.0, 80.0]);
    assert_eq!(justified(SpaceAround, 100.0, &[80.0; 2]), [0.0, 80.0]);
    assert_eq!(justified(SpaceEvenly, 100.0, &[80.0; 2]), [0.0, 80.0]);
    assert_eq!(justified(Center, 100.0, &[80.0; 2]), [-30.0, 50.0]);
    // A single item: space-evenly is centred.
    assert_eq!(justified(SpaceEvenly, 300.0, &[100.0]), [100.0]);
}

#[test]
fn a_safe_fallback_overflows_past_the_writing_modes_end_even_in_reverse() {
    let mut tree = Tree::new();
    let style = Style {
        flex_direction: FlexDirection::RowReverse,
        justify_content: JustifyContent::SpaceAround,
        ..Style::default()
    };
    let root = screen(&mut tree, 100.0, 100.0, style);
    let a = child(&mut tree, root, Style { flex_shrink: 0.0, ..wide(80.0) });
    let b = child(&mut tree, root, Style { flex_shrink: 0.0, ..wide(80.0) });
    tree.compute_layout(&mut NoMeasure);
    // Safe center overflowing aligns as `start` — the left edge, not row-reverse's main-start:
    // b (last, leftmost) at 0, a after it, overflowing on the right.
    assert_eq!(rect(&tree, b)[0], 0.0);
    assert_eq!(rect(&tree, a)[0], 80.0);
}

#[test]
fn align_items_and_align_self_place_items_in_the_line() {
    let mut tree = Tree::new();
    let root = screen(&mut tree, 300.0, 100.0, Style { align_items: AlignItems::Center, ..Style::default() });
    let center = child(&mut tree, root, sized(50.0, 20.0));
    let end = child(&mut tree, root, Style { align_self: Some(AlignItems::FlexEnd), ..sized(50.0, 20.0) });
    let start = child(&mut tree, root, Style { align_self: Some(AlignItems::FlexStart), ..sized(50.0, 20.0) });
    let stretch = child(&mut tree, root, Style { align_self: Some(AlignItems::Stretch), ..wide(50.0) });
    // A definite cross size is not stretched: flex-start.
    let fixed = child(&mut tree, root, Style { align_self: Some(AlignItems::Stretch), ..sized(50.0, 20.0) });
    tree.compute_layout(&mut NoMeasure);
    assert_eq!(rect(&tree, center), [0.0, 40.0, 50.0, 20.0]);
    assert_eq!(rect(&tree, end), [50.0, 80.0, 50.0, 20.0]);
    assert_eq!(rect(&tree, start), [100.0, 0.0, 50.0, 20.0]);
    assert_eq!(rect(&tree, stretch), [150.0, 0.0, 50.0, 100.0]);
    assert_eq!(rect(&tree, fixed), [200.0, 0.0, 50.0, 20.0]);
}

#[test]
fn auto_margins_take_the_free_space_on_both_axes() {
    let mut tree = Tree::new();
    let root = screen(&mut tree, 300.0, 100.0, Style::default());
    let auto = Length::Auto;
    let pushed = child(&mut tree, root, Style { margin: Edges { left: auto.clone(), ..Edges::all(px(0.0)) }, ..sized(50.0, 50.0) });
    let other = screen(&mut tree, 300.0, 100.0, Style::default());
    // All four auto: centred in the line (main free 250, cross free 50); justify-content has nothing left.
    let centred = child(&mut tree, other, Style { margin: Edges::all(auto), ..sized(50.0, 50.0) });
    tree.compute_layout(&mut NoMeasure);
    assert_eq!(rect(&tree, pushed), [250.0, 0.0, 50.0, 50.0]);
    assert_eq!(rect(&tree, centred), [125.0, 25.0, 50.0, 50.0]);
}

#[test]
fn baseline_alignment_lines_up_first_baselines() {
    let mut tree = Tree::new();
    let mut text = Text::default();
    let root = screen(&mut tree, 300.0, 100.0, Style { align_items: AlignItems::Baseline, ..Style::default() });
    // a's baseline: padding 10 + 16 = 26; b's: 16. Both line up at 26: b moves down 10.
    let padded = Style { padding: Edges { top: px(10.0), ..Edges::all(px(0.0)) }, ..Style::default() };
    let a = text.add(&mut tree, root, padded.clone(), 50.0, 50.0);
    let b = text.add(&mut tree, root, Style::default(), 50.0, 50.0);
    // A container's baseline is its first item's: padding 20 + 16 = 36 — the deepest, so a and b
    // move down to it.
    let nested = child(&mut tree, root, Style { padding: Edges { top: px(20.0), ..Edges::all(px(0.0)) }, ..Style::default() });
    let inner = text.add(&mut tree, nested, Style::default(), 50.0, 50.0);
    tree.compute_layout(&mut text);
    assert_eq!(rect(&tree, a), [0.0, 10.0, 50.0, 30.0]);
    assert_eq!(rect(&tree, b), [50.0, 20.0, 50.0, 20.0]);
    assert_eq!(rect(&tree, nested), [100.0, 0.0, 50.0, 40.0]);
    assert_eq!(rect(&tree, inner), [0.0, 20.0, 50.0, 20.0]);
}

// ── the box model ───────────────────────────────────────────────────────────────────────────

fn start_row() -> Style {
    Style { align_items: AlignItems::FlexStart, ..Style::default() }
}

fn edges(left: f32, top: f32, right: f32, bottom: f32) -> Edges<Length> {
    Edges { left: px(left), top: px(top), right: px(right), bottom: px(bottom) }
}

#[test]
fn padding_and_margin_percents_are_of_the_width_on_every_edge() {
    let mut tree = Tree::new();
    let root = screen(&mut tree, 400.0, 200.0, start_row());
    // padding-top 10% of 400 (not of 200) = 40; margin-left 5% of 400 = 20.
    let style = Style {
        padding: Edges { top: pct(10.0), ..edges(0.0, 0.0, 0.0, 0.0) },
        margin: Edges { left: pct(5.0), ..edges(0.0, 0.0, 0.0, 0.0) },
        ..sized(100.0, 50.0)
    };
    let a = child(&mut tree, root, style);
    tree.compute_layout(&mut NoMeasure);
    assert_eq!(rect(&tree, a), [20.0, 0.0, 100.0, 90.0]);
    assert_eq!(tree.layout(a).padding.top, 40.0);
}

#[test]
fn box_sizing_picks_the_box_the_size_measures() {
    let mut tree = Tree::new();
    let root = screen(&mut tree, 400.0, 200.0, start_row());
    let padded = Style { padding: edges(10.0, 10.0, 10.0, 10.0), ..sized(100.0, 50.0) };
    let content = child(&mut tree, root, padded.clone());
    let border = child(&mut tree, root, Style { box_sizing: BoxSizing::BorderBox, ..padded });
    tree.compute_layout(&mut NoMeasure);
    assert_eq!(rect(&tree, content), [0.0, 0.0, 120.0, 70.0]);
    assert_eq!(rect(&tree, border), [120.0, 0.0, 100.0, 50.0]);
}

#[test]
fn a_percent_size_is_of_the_parent_and_auto_against_an_indefinite_one() {
    let mut tree = Tree::new();
    let root = screen(&mut tree, 400.0, 200.0, start_row());
    let a = child(&mut tree, root, Style { size: Size::new(pct(50.0), pct(25.0)), ..Style::default() });
    // The open root's height is indefinite: 50% of it is auto (0 here), the root is 30 tall.
    let open = root_of_width(&mut tree, 400.0, column());
    let b = child(&mut tree, open, Style { size: Size::new(Length::Auto, pct(50.0)), ..Style::default() });
    let c = child(&mut tree, open, tall(30.0));
    tree.compute_layout(&mut NoMeasure);
    assert_eq!(rect(&tree, a), [0.0, 0.0, 200.0, 50.0]);
    assert_eq!(rect(&tree, open), [0.0, 0.0, 400.0, 30.0]);
    assert_eq!(rect(&tree, b), [0.0, 0.0, 400.0, 0.0]);
    assert_eq!(rect(&tree, c), [0.0, 0.0, 400.0, 30.0]);
}

fn root_of_width(tree: &mut Tree, width: f32, style: Style) -> NodeId {
    root(tree, Some(width), None, style)
}

#[test]
fn a_percent_height_resolves_only_against_a_definite_height() {
    let mut tree = Tree::new();
    let half_tall = Style { size: Size::new(px(10.0), pct(50.0)), ..Style::default() };
    // §9.8.3: a stretched item's height is definite — 50% of its 100.
    let row = screen(&mut tree, 300.0, 100.0, Style::default());
    let stretched = child(&mut tree, row, wide(50.0));
    let half = child(&mut tree, stretched, half_tall.clone());
    // §9.8.2: the flexed height of an item of a definite column is definite — 25% of its 400.
    let col = screen(&mut tree, 100.0, 400.0, column());
    let flexed = child(&mut tree, col, Style { flex_grow: 1.0, ..column() });
    let quarter = child(&mut tree, flexed, Style { size: Size::new(Length::Auto, pct(25.0)), ..Style::default() });
    // A height that came from the content is not: 50% of it is auto (0 here), even in the final
    // layout, where the height is known.
    let open = root_of_width(&mut tree, 100.0, column());
    let item = child(&mut tree, open, column());
    let lost = child(&mut tree, item, half_tall);
    child(&mut tree, item, tall(20.0));
    tree.compute_layout(&mut NoMeasure);
    assert_eq!(rect(&tree, half), [0.0, 0.0, 10.0, 50.0]);
    assert_eq!(rect(&tree, flexed), [0.0, 0.0, 100.0, 400.0]);
    assert_eq!(rect(&tree, quarter), [0.0, 0.0, 100.0, 100.0]);
    assert_eq!(rect(&tree, item), [0.0, 0.0, 100.0, 20.0]);
    assert_eq!(rect(&tree, lost), [0.0, 0.0, 10.0, 0.0]);
}

#[test]
fn a_wrapping_column_of_open_height_breaks_at_its_max_height() {
    let mut tree = Tree::new();
    let style = Style {
        flex_wrap: FlexWrap::Wrap,
        align_content: AlignContent::FlexStart,
        max_size: Size::new(Length::Auto, px(100.0)),
        ..column()
    };
    let root = root_of_width(&mut tree, 300.0, style);
    let items: Vec<NodeId> = (0..3).map(|_| child(&mut tree, root, sized(50.0, 40.0))).collect();
    tree.compute_layout(&mut NoMeasure);
    // 40 + 40 fit under 100, a third does not: lines [a b] [c]; the height is the longest line.
    assert_eq!(rect(&tree, root), [0.0, 0.0, 300.0, 80.0]);
    assert_eq!(rect(&tree, items[1]), [0.0, 40.0, 50.0, 40.0]);
    assert_eq!(rect(&tree, items[2]), [50.0, 0.0, 50.0, 40.0]);
}

#[test]
fn min_and_max_clamp_a_definite_size() {
    let mut tree = Tree::new();
    let a_root = screen(&mut tree, 400.0, 200.0, start_row());
    let a = child(&mut tree, a_root, Style { max_size: Size::new(px(300.0), Length::Auto), ..sized(500.0, 10.0) });
    let b_root = screen(&mut tree, 400.0, 200.0, start_row());
    let b = child(&mut tree, b_root, Style { min_size: Size::new(Length::Auto, px(40.0)), ..sized(50.0, 10.0) });
    tree.compute_layout(&mut NoMeasure);
    assert_eq!(rect(&tree, a), [0.0, 0.0, 300.0, 10.0]);
    assert_eq!(rect(&tree, b), [0.0, 0.0, 50.0, 40.0]);
}

#[test]
fn aspect_ratio_fills_the_axis_left_auto() {
    let mut tree = Tree::new();
    let row = screen(&mut tree, 400.0, 300.0, start_row());
    let a = child(&mut tree, row, Style { aspect_ratio: Some(2.0), ..wide(100.0) });
    // Content-box sizing: the ratio holds for the content box, 100 × 50, inside 10 of padding.
    let b = child(&mut tree, row, Style { aspect_ratio: Some(2.0), padding: edges(10.0, 10.0, 10.0, 10.0), ..wide(100.0) });
    // Stretched to the column's 300: 300 / 3 = 100 tall.
    let col = screen(&mut tree, 300.0, 300.0, column());
    let c = child(&mut tree, col, Style { aspect_ratio: Some(3.0), ..Style::default() });
    tree.compute_layout(&mut NoMeasure);
    assert_eq!(rect(&tree, a), [0.0, 0.0, 100.0, 50.0]);
    assert_eq!(rect(&tree, b), [100.0, 0.0, 120.0, 70.0]);
    assert_eq!(rect(&tree, c), [0.0, 0.0, 300.0, 100.0]);
}

// ── out of flow ─────────────────────────────────────────────────────────────────────────────

fn absolute(inset: Edges<Length>, style: Style) -> Style {
    Style { position: Position::Absolute, inset, ..style }
}

fn insets(left: Option<f32>, top: Option<f32>, right: Option<f32>, bottom: Option<f32>) -> Edges<Length> {
    let length = |value: Option<f32>| value.map_or(Length::Auto, px);
    Edges { left: length(left), top: length(top), right: length(right), bottom: length(bottom) }
}

#[test]
fn absolute_children_are_placed_by_insets_against_the_padding_box() {
    let mut tree = Tree::new();
    // Border 5, padding 10: the padding box is 290 × 190 at (5, 5).
    let style = Style { border: edges(5.0, 5.0, 5.0, 5.0), padding: edges(10.0, 10.0, 10.0, 10.0), ..Style::default() };
    let root = screen(&mut tree, 300.0, 200.0, style);
    let a = child(&mut tree, root, absolute(insets(Some(20.0), Some(30.0), None, None), sized(50.0, 40.0)));
    let b = child(&mut tree, root, absolute(insets(None, None, Some(20.0), Some(30.0)), sized(50.0, 40.0)));
    let c = child(&mut tree, root, absolute(insets(Some(10.0), Some(0.0), Some(10.0), None), tall(10.0)));
    let d = child(&mut tree, root, absolute(insets(Some(0.0), Some(0.0), None, None), Style { size: Size::new(pct(50.0), px(10.0)), ..Style::default() }));
    let flow = child(&mut tree, root, wide(50.0));
    tree.compute_layout(&mut NoMeasure);
    assert_eq!(rect(&tree, a), [25.0, 35.0, 50.0, 40.0]);
    // 5 + 290 - 20 - 50 = 225; 5 + 190 - 30 - 40 = 125.
    assert_eq!(rect(&tree, b), [225.0, 125.0, 50.0, 40.0]);
    // Both horizontal insets: 290 - 10 - 10 = 270 wide.
    assert_eq!(rect(&tree, c), [15.0, 5.0, 270.0, 10.0]);
    // 50% of the padding box's 290.
    assert_eq!(rect(&tree, d), [5.0, 5.0, 145.0, 10.0]);
    // The in-flow item ignores them all: at the content box, stretched to its 170.
    assert_eq!(rect(&tree, flow), [15.0, 15.0, 50.0, 170.0]);
}

#[test]
fn an_absolute_child_without_insets_takes_its_static_position() {
    let mut tree = Tree::new();
    let centred = Style { justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..Style::default() };
    let root = screen(&mut tree, 300.0, 200.0, centred);
    let a = child(&mut tree, root, absolute(insets(None, None, None, None), sized(50.0, 40.0)));
    // In a row-reverse, flex-start is the right edge.
    let reverse = screen(&mut tree, 300.0, 200.0, Style { flex_direction: FlexDirection::RowReverse, ..Style::default() });
    let b = child(&mut tree, reverse, absolute(insets(None, None, None, None), sized(50.0, 40.0)));
    tree.compute_layout(&mut NoMeasure);
    assert_eq!(rect(&tree, a), [125.0, 80.0, 50.0, 40.0]);
    assert_eq!(rect(&tree, b), [250.0, 0.0, 50.0, 40.0]);
}

#[test]
fn relative_insets_shift_the_box_and_nothing_else() {
    let mut tree = Tree::new();
    let root = screen(&mut tree, 300.0, 100.0, Style::default());
    let a = child(&mut tree, root, Style { inset: insets(Some(10.0), Some(5.0), None, None), ..sized(50.0, 50.0) });
    let b = child(&mut tree, root, sized(50.0, 50.0));
    let c = child(&mut tree, root, Style { inset: insets(None, None, Some(10.0), None), ..sized(50.0, 50.0) });
    tree.compute_layout(&mut NoMeasure);
    assert_eq!(rect(&tree, a), [10.0, 5.0, 50.0, 50.0]);
    assert_eq!(rect(&tree, b), [50.0, 0.0, 50.0, 50.0]);
    assert_eq!(rect(&tree, c), [90.0, 0.0, 50.0, 50.0]);
}

#[test]
fn display_none_takes_no_space_and_zeroes_its_subtree() {
    let mut tree = Tree::new();
    let root = screen(&mut tree, 300.0, 100.0, Style::default());
    let a = child(&mut tree, root, wide(50.0));
    let hidden = child(&mut tree, root, wide(100.0));
    let inside = child(&mut tree, hidden, sized(10.0, 10.0));
    let c = child(&mut tree, root, wide(50.0));
    tree.compute_layout(&mut NoMeasure);
    assert_eq!(rect(&tree, hidden), [50.0, 0.0, 100.0, 100.0]);
    assert_eq!(rect(&tree, inside), [0.0, 0.0, 10.0, 10.0]);

    tree.update_style(hidden, |style| style.display = Display::None);
    tree.compute_layout(&mut NoMeasure);
    assert_eq!(rect(&tree, a), [0.0, 0.0, 50.0, 100.0]);
    assert_eq!(rect(&tree, hidden), [0.0; 4]);
    assert_eq!(rect(&tree, inside), [0.0; 4]);
    assert_eq!(rect(&tree, c), [50.0, 0.0, 50.0, 100.0]);

    // Shown again, the subtree is laid out again — not answered from a cache of boxes long gone.
    tree.update_style(hidden, |style| style.display = Display::Flex);
    tree.compute_layout(&mut NoMeasure);
    assert_eq!(rect(&tree, hidden), [50.0, 0.0, 100.0, 100.0]);
    assert_eq!(rect(&tree, inside), [0.0, 0.0, 10.0, 10.0]);
    assert_eq!(rect(&tree, c), [150.0, 0.0, 50.0, 100.0]);
}

// ── measured leaves ─────────────────────────────────────────────────────────────────────────

#[test]
fn a_measured_leaf_is_asked_in_content_box_px() {
    let mut tree = Tree::new();
    let root = screen(&mut tree, 100.0, 300.0, column());
    let leaf = child(&mut tree, root, Style { padding: edges(10.0, 10.0, 10.0, 10.0), ..Style::default() });
    tree.set_measured(leaf, true);
    let mut asked = Vec::new();
    let mut measurer = |_: NodeId, input: MeasureInput| {
        asked.push(input);
        // 120 px of text: two lines at the 80 px it gets.
        let width = input.known.width.unwrap();
        MeasureOutput { size: Size::new(width, (120.0 / width).ceil() * 20.0), baseline: None }
    };
    tree.compute_layout(&mut measurer);
    assert_eq!(rect(&tree, leaf), [0.0, 0.0, 100.0, 60.0]);
    assert!(!asked.is_empty());
    assert!(asked.iter().all(|input| input.known.width == Some(80.0) && input.known.height.is_none()));
}

#[test]
fn a_measured_leaf_keeps_its_size_off_the_grid_and_takes_no_more() {
    // The one test here off whole px: a measured leaf's far edge goes out as far as its size takes
    // from the SNAPPED near edge, no further. A 40 px text at y 0.3: 0..40 (taking the far edge up
    // to the grid gave 0..41); a 20.2 px one at y 0.6, its near edge snapped down to 1: 1..22, 21 px
    // (the grid's far edge, 21, would have left it 20, under what it measured).
    let mut tree = Tree::new();
    let first = screen(&mut tree, 100.0, 300.0, column());
    child(&mut tree, first, tall(0.3));
    let fixed = child(&mut tree, first, tall(40.0));
    tree.set_measured(fixed, true);
    let second = screen(&mut tree, 100.0, 300.0, column());
    child(&mut tree, second, tall(0.6));
    let measured = child(&mut tree, second, Style::default());
    tree.set_measured(measured, true);
    let mut measurer = |_: NodeId, input: MeasureInput| MeasureOutput {
        size: Size::new(input.known.width.unwrap_or(100.0), input.known.height.unwrap_or(20.2)),
        baseline: None,
    };
    tree.compute_layout(&mut measurer);
    assert_eq!(rect(&tree, fixed), [0.0, 0.0, 100.0, 40.0]);
    assert_eq!(rect(&tree, measured), [0.0, 1.0, 100.0, 21.0]);
}

#[test]
fn a_leaf_with_both_axes_known_is_never_measured() {
    let mut tree = Tree::new();
    let mut text = Text::default();
    let root = screen(&mut tree, 300.0, 100.0, start_row());
    let leaf = text.add(&mut tree, root, sized(50.0, 20.0), 500.0, 50.0);
    tree.compute_layout(&mut text);
    assert_eq!(rect(&tree, leaf), [0.0, 0.0, 50.0, 20.0]);
    assert!(text.calls.is_empty());
}

#[test]
fn a_max_width_wraps_the_text_at_the_max() {
    let mut tree = Tree::new();
    let mut text = Text::default();
    let max = Style { max_size: Size::new(px(100.0), Length::Auto), ..Style::default() };
    // 300 px of text held to 100: three lines, 60 tall — in a row (the max is on the main axis)
    // and in a column (on the cross axis).
    let row = screen(&mut tree, 400.0, 300.0, start_row());
    let a = text.add(&mut tree, row, max.clone(), 300.0, 50.0);
    let col = screen(&mut tree, 400.0, 300.0, Style { align_items: AlignItems::FlexStart, ..column() });
    let b = text.add(&mut tree, col, max, 300.0, 50.0);
    tree.compute_layout(&mut text);
    assert_eq!(rect(&tree, a), [0.0, 0.0, 100.0, 60.0]);
    assert_eq!(rect(&tree, b), [0.0, 0.0, 100.0, 60.0]);
}

#[test]
fn a_cross_margin_is_taken_off_the_space_offered_once() {
    let mut tree = Tree::new();
    let mut text = Text::default();
    // 300 px of text in a 200 px column, not stretched, 20 px of margin on its left: its margin box
    // has 200, its border box 180 — fit-content: 180, two lines.
    let col = screen(&mut tree, 200.0, 300.0, Style { align_items: AlignItems::FlexStart, ..column() });
    let margin = Edges { left: px(20.0), ..edges(0.0, 0.0, 0.0, 0.0) };
    let a = text.add(&mut tree, col, Style { margin, ..Style::default() }, 300.0, 50.0);
    tree.compute_layout(&mut text);
    assert_eq!(rect(&tree, a), [20.0, 0.0, 180.0, 40.0]);
}

#[test]
fn an_auto_width_box_is_sized_before_it_is_laid_out() {
    let mut tree = Tree::new();
    let mut text = Text::default();
    // A column of auto width, placed at the start: as wide as its widest item (100); the narrower
    // item stretches to it.
    let root = screen(&mut tree, 400.0, 400.0, Style { align_items: AlignItems::FlexStart, ..column() });
    let list = child(&mut tree, root, column());
    let long = text.add(&mut tree, list, Style::default(), 100.0, 50.0);
    let short = text.add(&mut tree, list, Style::default(), 60.0, 60.0);
    // A row of auto width that does not fit its 200: fit-content = max(min-content 100, 200) = 200;
    // its two 150 px texts shrink to 100 each and wrap onto two lines.
    let narrow = screen(&mut tree, 200.0, 400.0, Style { align_items: AlignItems::FlexStart, ..column() });
    let line = child(&mut tree, narrow, Style::default());
    let first = text.add(&mut tree, line, Style::default(), 150.0, 50.0);
    let second = text.add(&mut tree, line, Style::default(), 150.0, 50.0);
    tree.compute_layout(&mut text);
    assert_eq!(rect(&tree, list), [0.0, 0.0, 100.0, 40.0]);
    assert_eq!(rect(&tree, long), [0.0, 0.0, 100.0, 20.0]);
    assert_eq!(rect(&tree, short), [0.0, 20.0, 100.0, 20.0]);
    assert_eq!(rect(&tree, line), [0.0, 0.0, 200.0, 40.0]);
    assert_eq!(rect(&tree, first), [0.0, 0.0, 100.0, 40.0]);
    assert_eq!(rect(&tree, second), [100.0, 0.0, 100.0, 40.0]);
}

// ── the root and the content size ───────────────────────────────────────────────────────────

#[test]
fn an_unbounded_root_is_sized_by_its_content() {
    let mut tree = Tree::new();
    let open = root(&mut tree, None, None, Style::default());
    child(&mut tree, open, sized(50.0, 30.0));
    child(&mut tree, open, sized(20.0, 10.0));
    // The viewport's width with a max: 300, not 400; margins come off the viewport.
    let capped = screen(&mut tree, 400.0, 200.0, Style { max_size: Size::new(px(300.0), Length::Auto), ..Style::default() });
    let margined = screen(&mut tree, 400.0, 200.0, Style { margin: edges(10.0, 20.0, 10.0, 20.0), ..Style::default() });
    // A definite style size wins over the viewport, and a percent is of the viewport.
    let fixed = screen(&mut tree, 400.0, 200.0, Style { size: Size::new(pct(50.0), px(80.0)), ..Style::default() });
    tree.compute_layout(&mut NoMeasure);
    assert_eq!(rect(&tree, open), [0.0, 0.0, 70.0, 30.0]);
    assert_eq!(rect(&tree, capped), [0.0, 0.0, 300.0, 200.0]);
    assert_eq!(rect(&tree, margined), [10.0, 20.0, 380.0, 160.0]);
    assert_eq!(rect(&tree, fixed), [0.0, 0.0, 200.0, 80.0]);
}

#[test]
fn the_content_size_is_what_overflows_the_padding_box() {
    let mut tree = Tree::new();
    let style = Style {
        flex_direction: FlexDirection::Column,
        overflow: Overflow::Scroll,
        padding: edges(0.0, 0.0, 0.0, 10.0),
        ..Style::default()
    };
    let root = screen(&mut tree, 100.0, 100.0, style);
    let tall_item = Style { flex_shrink: 0.0, margin: edges(0.0, 0.0, 0.0, 5.0), ..sized(50.0, 80.0) };
    child(&mut tree, root, tall_item.clone());
    child(&mut tree, root, tall_item);
    let far = child(&mut tree, root, absolute(insets(Some(120.0), Some(0.0), None, None), sized(30.0, 10.0)));
    tree.compute_layout(&mut NoMeasure);
    assert_eq!(rect(&tree, far), [120.0, 0.0, 30.0, 10.0]);
    // Height: 80 + 5 + 80 + 5 = 170, plus the end padding 10. Width: the absolute child's 150.
    assert_eq!(tree.layout(root).content_size, Size::new(150.0, 180.0));
}

// ── caching and the changed set ─────────────────────────────────────────────────────────────

#[test]
fn a_new_scale_snaps_every_box_again() {
    let mut tree = Tree::new();
    let root = screen(&mut tree, 100.0, 100.0, Style::default());
    // A level down: the wrapper is answered from its cache when the scale changes.
    let wrapper = child(&mut tree, root, Style::default());
    child(&mut tree, wrapper, wide(10.5));
    let b = child(&mut tree, wrapper, wide(10.0));
    tree.compute_layout(&mut NoMeasure);
    // b's left edge is at 10.5: rounded to 11 on a 1 px grid, kept on a 0.5 px one.
    assert_eq!(rect(&tree, b)[0], 11.0);
    tree.set_scale(2.0);
    tree.compute_layout(&mut NoMeasure);
    let changes = tree.changes().to_vec();
    assert_eq!(rect(&tree, b)[0], 10.5);
    assert!(changes.iter().any(|change| change.node == b && change.moved));
}

#[test]
fn a_pass_without_changes_reports_nothing_and_measures_nothing() {
    let mut tree = Tree::new();
    let mut text = Text::default();
    let root = screen(&mut tree, 200.0, 400.0, column());
    let leaves: Vec<NodeId> = (0..3).map(|_| text.add(&mut tree, root, Style::default(), 100.0, 50.0)).collect();
    tree.compute_layout(&mut text);
    let first = tree.changes().len();
    assert_eq!(first, 4, "every box is new");
    assert_eq!(text.calls.len(), 3, "a stretched text in a column is measured once");

    text.calls.clear();
    tree.mark_dirty(root);
    tree.compute_layout(&mut text);
    assert!(tree.changes().is_empty());
    assert!(text.calls.is_empty(), "the leaves answered from their caches");
    assert_eq!(rect(&tree, leaves[2]), [0.0, 40.0, 200.0, 20.0]);
}

#[test]
fn a_changed_leaf_is_measured_again_alone_and_the_changes_name_what_moved() {
    let mut tree = Tree::new();
    let mut text = Text::default();
    let root = screen(&mut tree, 200.0, 400.0, column());
    let wrapper = child(&mut tree, root, column());
    let leaves: Vec<NodeId> = (0..3).map(|_| text.add(&mut tree, wrapper, Style::default(), 100.0, 50.0)).collect();
    let after = text.add(&mut tree, root, Style::default(), 100.0, 50.0);
    tree.compute_layout(&mut text);
    text.calls.clear();

    // The middle text grows to three lines at 200: 60 tall.
    text.texts.insert(leaves[1], (500.0, 50.0));
    tree.mark_dirty(leaves[1]);
    tree.compute_layout(&mut text);
    let changes = tree.changes().to_vec();
    assert_eq!(text.calls, [leaves[1]]);

    let mut changed: Vec<NodeId> = changes.iter().map(|change| change.node).collect();
    changed.sort_by_key(|node| node.to_bits());
    let mut expected = vec![wrapper, leaves[1], leaves[2], after];
    expected.sort_by_key(|node| node.to_bits());
    assert_eq!(changed, expected, "the root keeps its box; the first leaf neither moved nor resized");
    let find = |node: NodeId| *changes.iter().find(|change| change.node == node).unwrap();
    assert!(find(wrapper).resized && !find(wrapper).moved);
    assert!(find(leaves[1]).resized && !find(leaves[1]).moved);
    assert!(find(leaves[2]).moved && !find(leaves[2]).resized);
    assert!(find(after).moved && !find(after).resized);
    assert_eq!(rect(&tree, leaves[2]), [0.0, 80.0, 200.0, 20.0]);
    assert_eq!(rect(&tree, after), [0.0, 100.0, 200.0, 20.0]);
}

#[test]
fn a_row_of_text_is_measured_once_per_item() {
    let mut tree = Tree::new();
    let mut text = Text::default();
    let root = screen(&mut tree, 400.0, 100.0, start_row());
    let a = text.add(&mut tree, root, Style::default(), 100.0, 50.0);
    let b = text.add(&mut tree, root, Style::default(), 100.0, 50.0);
    tree.compute_layout(&mut text);
    // Its max-content width answers the cross-size question too: asked again at its own width.
    assert_eq!(text.calls.len(), 2);
    assert_eq!(rect(&tree, a), [0.0, 0.0, 100.0, 20.0]);
    assert_eq!(rect(&tree, b), [100.0, 0.0, 100.0, 20.0]);
}

#[test]
fn a_text_is_not_asked_again_under_an_offered_height_it_fits_in() {
    let mut tree = Tree::new();
    let mut text = Text::default();
    // The row's height is open while the column sizes it (the text is offered no height), then
    // known in the row's final layout (the text is offered 20): its answer, 20 tall, fits — so the
    // text is asked once, its max-content width answering the cross size at that width too.
    let root = screen(&mut tree, 300.0, 400.0, column());
    let row = child(&mut tree, root, start_row());
    let a = text.add(&mut tree, row, Style::default(), 100.0, 50.0);
    tree.compute_layout(&mut text);
    assert_eq!(rect(&tree, row), [0.0, 0.0, 300.0, 20.0]);
    assert_eq!(rect(&tree, a), [0.0, 0.0, 100.0, 20.0]);
    assert_eq!(text.calls, [a]);
}

#[test]
fn a_measurer_that_caps_its_height_is_asked_again_when_its_answer_does_not_fit() {
    let mut tree = Tree::new();
    let root = screen(&mut tree, 300.0, 400.0, column());
    // The row is held to 30: its text, 40 tall where nothing bounds it, is offered 30 when the
    // row's final layout asks for its height (its cross size, at its width). A Measurer that caps a
    // height at the offer answers 30 there — which no cached answer implies, so it is asked. (The
    // width questions offer no height at all.)
    let row = child(&mut tree, root, Style { max_size: Size::new(Length::Auto, px(30.0)), ..start_row() });
    let leaf = child(&mut tree, row, Style::default());
    tree.set_measured(leaf, true);
    let mut asked = Vec::new();
    let mut capping = |_: NodeId, input: MeasureInput| {
        asked.push(input.available.height);
        let height = match (input.known.height, input.available.height) {
            (Some(height), _) => height,
            (None, Available::Definite(offer)) => offer.min(40.0),
            (None, _) => 40.0,
        };
        MeasureOutput { size: Size::new(input.known.width.unwrap_or(100.0), height), baseline: None }
    };
    tree.compute_layout(&mut capping);
    assert_eq!(asked, [Available::MaxContent, Available::Definite(30.0)]);
    assert_eq!(rect(&tree, row), [0.0, 0.0, 300.0, 30.0]);
    assert_eq!(rect(&tree, leaf), [0.0, 0.0, 100.0, 30.0]);
}

#[test]
fn a_width_is_asked_without_a_height() {
    let mut tree = Tree::new();
    let root = screen(&mut tree, 300.0, 400.0, column());
    // A row 20 tall asks its text for its width (its base size): no height is offered, though
    // the row has one. Its answer, 20 tall, then serves the height question at that width too.
    let row = child(&mut tree, root, Style { size: Size::new(Length::Auto, px(20.0)), ..start_row() });
    let leaf = child(&mut tree, row, Style::default());
    tree.set_measured(leaf, true);
    let mut asked = Vec::new();
    let mut measurer = |_: NodeId, input: MeasureInput| {
        asked.push(input);
        MeasureOutput { size: Size::new(input.known.width.unwrap_or(100.0), 20.0), baseline: None }
    };
    tree.compute_layout(&mut measurer);
    assert_eq!(asked.len(), 1);
    assert_eq!(asked[0].available, Size::new(Available::MaxContent, Available::MaxContent));
    assert_eq!(rect(&tree, leaf), [0.0, 0.0, 100.0, 20.0]);
}

#[test]
fn a_content_sized_column_asks_its_text_only_at_its_final_width() {
    let mut tree = Tree::new();
    let mut text = Text::default();
    // The row asks the column for its base size: a width alone — 200, its widest text — found
    // from the texts' max-content widths, without laying them out at 200. The column then grows
    // to 400, and its texts are asked their height there. Each text: two questions.
    let root = screen(&mut tree, 400.0, 100.0, start_row());
    let col = child(&mut tree, root, Style { flex_grow: 1.0, ..column() });
    let a = text.add(&mut tree, col, Style::default(), 100.0, 50.0);
    let b = text.add(&mut tree, col, Style::default(), 200.0, 50.0);
    tree.compute_layout(&mut text);
    assert_eq!(rect(&tree, col), [0.0, 0.0, 400.0, 40.0]);
    assert_eq!(rect(&tree, a), [0.0, 0.0, 400.0, 20.0]);
    assert_eq!(rect(&tree, b), [0.0, 20.0, 400.0, 20.0]);
    assert_eq!(text.calls.iter().filter(|node| **node == a).count(), 2);
    assert_eq!(text.calls.iter().filter(|node| **node == b).count(), 2);
}

#[test]
fn a_style_change_deep_in_one_branch_leaves_the_other_branches_alone() {
    let mut tree = Tree::new();
    let mut text = Text::default();
    let root = screen(&mut tree, 300.0, 300.0, Style::default());
    let left = child(&mut tree, root, column());
    let right = child(&mut tree, root, column());
    let left_text = text.add(&mut tree, left, Style::default(), 100.0, 50.0);
    let right_box = child(&mut tree, right, sized(20.0, 20.0));
    let right_text = text.add(&mut tree, right, Style::default(), 100.0, 50.0);
    tree.compute_layout(&mut text);
    text.calls.clear();

    // The right box widens: the right column's width (its max-content) grows, the left column
    // keeps its box. Only the right text, stretched to the new width, is measured again.
    tree.set_style(right_box, sized(150.0, 20.0));
    tree.compute_layout(&mut text);
    let changes = tree.changes().to_vec();
    assert!(!text.calls.is_empty() && text.calls.iter().all(|node| *node == right_text));
    assert!(changes.iter().all(|change| change.node != left && change.node != left_text));
    assert!(changes.iter().any(|change| change.node == right && change.resized));
    assert_eq!(rect(&tree, right), [100.0, 0.0, 150.0, 300.0]);
}

#[test]
fn a_length_that_computes_below_zero_is_zero() {
    use souple::{Formula, Op};
    // calc(10px - 50%) of 100 is -40: CSS clamps sizes, a basis, padding and gaps at 0.
    let negative = Formula::build(&[Op::Px(10.0), Op::Percent(50.0), Op::Sub]).unwrap();
    let mut tree = Tree::new();
    let root = tree.create_node();
    let [first, second] = [tree.create_node(), tree.create_node()];
    tree.append(root, first);
    tree.append(root, second);
    tree.update_style(root, |style| {
        style.size = Size::new(Length::Px(100.0), Length::Px(100.0));
        style.column_gap = negative.clone();
    });
    tree.update_style(first, |style| {
        style.size.width = negative.clone();
        style.padding.left = negative.clone();
    });
    tree.update_style(second, |style| style.flex_basis = negative.clone());
    tree.set_viewport(root, Viewport::unbounded());
    tree.compute_layout(&mut NoMeasure);
    assert_eq!(tree.layout(first).rect.width, 0.0);
    assert_eq!(tree.layout(second).rect.x, 0.0, "no negative gap pulls the second box back");
    assert_eq!(tree.layout(second).rect.width, 0.0);
}

// ── hugging leaves ──────────────────────────────────────────────────────────────────────────

const SPACE: f32 = 10.0;
const ELLIPSIS: f32 = 12.0;
const LINE: f32 = 20.0;

/// Text as a text engine lays it out: words of the given widths, a 10 px space between two on a
/// line, broken between words to fit the width it is given, reporting its LONGEST LINE (not the
/// width offered) and 20 px a line. A `single` text is one ellipsized line instead: the words that
/// fit and a 12 px ellipsis. Every call is logged.
#[derive(Default)]
struct Words {
    texts: HashMap<NodeId, (Vec<f32>, bool)>,
    calls: Vec<NodeId>,
}

impl Words {
    fn add(&mut self, tree: &mut Tree, parent: NodeId, style: Style, words: &[f32], single: bool, hug: bool) -> NodeId {
        let node = child(tree, parent, style);
        tree.set_measured(node, true);
        tree.set_hug(node, hug);
        self.texts.insert(node, (words.to_vec(), single));
        node
    }
}

/// The widths of the lines `words` break into within `limit`.
fn lines(words: &[f32], limit: f32) -> Vec<f32> {
    let mut lines = vec![words[0]];
    for &word in &words[1..] {
        let last = lines.last_mut().unwrap();
        if *last + SPACE + word <= limit {
            *last += SPACE + word;
        } else {
            lines.push(word);
        }
    }
    lines
}

impl Measurer for Words {
    fn measure(&mut self, node: NodeId, input: MeasureInput) -> MeasureOutput {
        self.calls.push(node);
        let (words, single) = &self.texts[&node];
        let natural = lines(words, f32::INFINITY)[0];
        let limit = match (input.known.width, input.available.width) {
            (Some(width), _) | (None, Available::Definite(width)) => width,
            (None, Available::MaxContent) => f32::INFINITY,
            (None, Available::MinContent) => 0.0,
        };
        let (width, height) = if *single {
            // The longest prefix of words that fits with its ellipsis (the ellipsis alone if none).
            let mut kept = 0.0;
            let mut prefix = 0.0;
            for (index, word) in words.iter().enumerate() {
                prefix += if index == 0 { *word } else { SPACE + word };
                if prefix + ELLIPSIS <= limit {
                    kept = prefix;
                }
            }
            (if natural <= limit { natural } else { kept + ELLIPSIS }, LINE)
        } else {
            let lines = lines(words, limit);
            (lines.iter().copied().fold(0.0, f32::max), lines.len() as f32 * LINE)
        };
        MeasureOutput { size: Size::new(input.known.width.unwrap_or(width), height), baseline: None }
    }
}

/// Four words, 380 px on one line: within 362 they break as 100 + 100 + 100 (320) and 50.
const PARAGRAPH: [f32; 4] = [100.0, 100.0, 100.0, 50.0];

#[test]
fn a_hugging_text_is_as_wide_as_its_longest_line() {
    let mut tree = Tree::new();
    let mut words = Words::default();
    // Hugging: offered the row's 362, the text breaks into 320 and 50 — its box is 320 wide.
    let hugging = screen(&mut tree, 362.0, 400.0, start_row());
    let a = words.add(&mut tree, hugging, Style::default(), &PARAGRAPH, false, true);
    // Not hugging (CSS): its max-content 380 shrinks to the 362 it has — the box is 362 wide.
    let filling = screen(&mut tree, 362.0, 400.0, start_row());
    let b = words.add(&mut tree, filling, Style::default(), &PARAGRAPH, false, false);
    // Centred, the narrower box sits in the middle: (362 - 320) / 2 = 21.
    let centred = screen(&mut tree, 362.0, 400.0, Style { justify_content: JustifyContent::Center, ..start_row() });
    let c = words.add(&mut tree, centred, Style::default(), &PARAGRAPH, false, true);
    tree.compute_layout(&mut words);
    assert_eq!(rect(&tree, a), [0.0, 0.0, 320.0, 40.0]);
    assert_eq!(rect(&tree, b), [0.0, 0.0, 362.0, 40.0]);
    assert_eq!(rect(&tree, c), [21.0, 0.0, 320.0, 40.0]);
}

#[test]
fn a_hugging_ellipsized_line_is_as_wide_as_what_it_kept() {
    let mut tree = Tree::new();
    let mut words = Words::default();
    // 80 + 80 + 80 = 260 on one line, in 242: two words and the ellipsis fit, 80 + 10 + 80 + 12.
    let hugging = screen(&mut tree, 242.0, 100.0, start_row());
    let a = words.add(&mut tree, hugging, Style::default(), &[80.0; 3], true, true);
    // Not hugging: 260 shrinks to 242, and the line is clipped inside the full 242.
    let filling = screen(&mut tree, 242.0, 100.0, start_row());
    let b = words.add(&mut tree, filling, Style::default(), &[80.0; 3], true, false);
    tree.compute_layout(&mut words);
    assert_eq!(rect(&tree, a), [0.0, 0.0, 182.0, 20.0]);
    assert_eq!(rect(&tree, b), [0.0, 0.0, 242.0, 20.0]);
}

#[test]
fn what_fixes_a_width_wins_over_hugging() {
    let mut tree = Tree::new();
    let mut words = Words::default();
    // A definite width: 300, the words broken within it (210 and 160: two lines).
    let row = screen(&mut tree, 362.0, 400.0, start_row());
    let sized = words.add(&mut tree, row, wide(300.0), &PARAGRAPH, false, true);
    // A stretch across a column: the column's 362.
    let col = screen(&mut tree, 362.0, 400.0, column());
    let stretched = words.add(&mut tree, col, Style::default(), &PARAGRAPH, false, true);
    // A flex-grow share: the base 320 grows by the 42 left, to 362.
    let row = screen(&mut tree, 362.0, 400.0, start_row());
    let grown = words.add(&mut tree, row, Style { flex_grow: 1.0, ..Style::default() }, &PARAGRAPH, false, true);
    tree.compute_layout(&mut words);
    assert_eq!(rect(&tree, sized), [0.0, 0.0, 300.0, 40.0]);
    assert_eq!(rect(&tree, stretched), [0.0, 0.0, 362.0, 40.0]);
    assert_eq!(rect(&tree, grown), [0.0, 0.0, 362.0, 40.0]);
}

#[test]
fn a_sibling_follows_a_hugging_text_at_its_reported_width() {
    let mut tree = Tree::new();
    let mut words = Words::default();
    let row = screen(&mut tree, 362.0, 400.0, start_row());
    words.add(&mut tree, row, Style::default(), &PARAGRAPH, false, true);
    let next = child(&mut tree, row, sized(20.0, 20.0));
    tree.compute_layout(&mut words);
    assert_eq!(rect(&tree, next), [320.0, 0.0, 20.0, 20.0]);
}

#[test]
fn a_content_sized_row_hugs_its_hugging_text() {
    let mut tree = Tree::new();
    let mut words = Words::default();
    // A row of auto width at the start of a column: its max-content contribution is the text's
    // width within the 362 it may take — 320 — so the row is 320 wide. Not hugging, it would be
    // fit-content: max-content 380 over 362, so 362.
    let col = screen(&mut tree, 362.0, 400.0, Style { align_items: AlignItems::FlexStart, ..column() });
    let row = child(&mut tree, col, Style::default());
    let text = words.add(&mut tree, row, Style::default(), &PARAGRAPH, false, true);
    let other = child(&mut tree, col, Style::default());
    let filling = words.add(&mut tree, other, Style::default(), &PARAGRAPH, false, false);
    tree.compute_layout(&mut words);
    assert_eq!(rect(&tree, row), [0.0, 0.0, 320.0, 40.0]);
    assert_eq!(rect(&tree, text), [0.0, 0.0, 320.0, 40.0]);
    assert_eq!(rect(&tree, other)[2], 362.0);
    assert_eq!(rect(&tree, filling)[2], 362.0);
}

#[test]
fn a_hugging_text_alone_in_a_row_is_measured_once() {
    // CSS: its max-content (380), its min-content (the line overflows), its height at 362: three
    // questions. Hugging: its width within 362, which then answers the height at that width too.
    let count = |hug: bool| {
        let mut tree = Tree::new();
        let mut words = Words::default();
        let row = screen(&mut tree, 362.0, 400.0, start_row());
        words.add(&mut tree, row, Style::default(), &PARAGRAPH, false, hug);
        tree.compute_layout(&mut words);
        words.calls.len()
    };
    assert_eq!(count(false), 3);
    assert_eq!(count(true), 1);
}

#[test]
fn turning_hug_on_and_off_lays_the_text_out_again() {
    let mut tree = Tree::new();
    let mut words = Words::default();
    let row = screen(&mut tree, 362.0, 400.0, start_row());
    let text = words.add(&mut tree, row, Style::default(), &PARAGRAPH, false, false);
    tree.compute_layout(&mut words);
    assert_eq!(rect(&tree, text)[2], 362.0);

    tree.set_hug(text, true);
    tree.compute_layout(&mut words);
    let changes = tree.changes().to_vec();
    assert!(changes.iter().any(|change| change.node == text && change.resized));
    assert_eq!(rect(&tree, text)[2], 320.0);

    tree.set_hug(text, true);
    tree.compute_layout(&mut words);
    assert!(tree.changes().is_empty(), "the same flag again changes nothing");
    tree.set_hug(text, false);
    tree.compute_layout(&mut words);
    assert_eq!(rect(&tree, text)[2], 362.0);
}

fn max_width(width: f32) -> Style {
    Style { max_size: Size::new(px(width), Length::Auto), ..Style::default() }
}

// Under a max of 176 the paragraph breaks as 100 / 100 / 100 + 50 = 160: three lines, 60 tall. A
// hugging text is offered at most its max and keeps its longest line, 160; a text that does not
// hug is cut to the max and fills it, 176, broken the same way.

#[test]
fn a_hugging_text_cut_by_its_max_keeps_its_longest_line() {
    let mut tree = Tree::new();
    let mut words = Words::default();
    // As a row's main size: a base of 160, within the max.
    let row = screen(&mut tree, 362.0, 400.0, start_row());
    let in_row = words.add(&mut tree, row, max_width(176.0), &PARAGRAPH, false, true);
    let row = screen(&mut tree, 362.0, 400.0, start_row());
    let in_row_off = words.add(&mut tree, row, max_width(176.0), &PARAGRAPH, false, false);
    // As a column's cross size, the column's items at the start.
    let col = screen(&mut tree, 362.0, 400.0, Style { align_items: AlignItems::FlexStart, ..column() });
    let in_col = words.add(&mut tree, col, max_width(176.0), &PARAGRAPH, false, true);
    let col = screen(&mut tree, 362.0, 400.0, Style { align_items: AlignItems::FlexStart, ..column() });
    let in_col_off = words.add(&mut tree, col, max_width(176.0), &PARAGRAPH, false, false);
    // As an absolute box with an open width.
    let host = screen(&mut tree, 362.0, 400.0, Style::default());
    let floating = absolute(insets(Some(0.0), Some(0.0), None, None), max_width(176.0));
    let absolute_hug = words.add(&mut tree, host, floating.clone(), &PARAGRAPH, false, true);
    let host = screen(&mut tree, 362.0, 400.0, Style::default());
    let absolute_off = words.add(&mut tree, host, floating, &PARAGRAPH, false, false);
    tree.compute_layout(&mut words);
    assert_eq!(rect(&tree, in_row), [0.0, 0.0, 160.0, 60.0]);
    assert_eq!(rect(&tree, in_row_off), [0.0, 0.0, 176.0, 60.0]);
    assert_eq!(rect(&tree, in_col), [0.0, 0.0, 160.0, 60.0]);
    assert_eq!(rect(&tree, in_col_off), [0.0, 0.0, 176.0, 60.0]);
    assert_eq!(rect(&tree, absolute_hug), [0.0, 0.0, 160.0, 60.0]);
    assert_eq!(rect(&tree, absolute_off), [0.0, 0.0, 176.0, 60.0]);
}

#[test]
fn a_centred_hugging_text_under_its_max_is_centred_as_its_longest_line() {
    let mut tree = Tree::new();
    let mut words = Words::default();
    let centred = || Style { align_items: AlignItems::Center, ..column() };
    // (362 - 160) / 2 = 101; not hugging, (362 - 176) / 2 = 93.
    let col = screen(&mut tree, 362.0, 400.0, centred());
    let hug = words.add(&mut tree, col, max_width(176.0), &PARAGRAPH, false, true);
    let col = screen(&mut tree, 362.0, 400.0, centred());
    let off = words.add(&mut tree, col, max_width(176.0), &PARAGRAPH, false, false);
    // A stretch still wins: stretched to the column, cut by the max, it fills the 176.
    let col = screen(&mut tree, 362.0, 400.0, column());
    let stretched = words.add(&mut tree, col, max_width(176.0), &PARAGRAPH, false, true);
    tree.compute_layout(&mut words);
    assert_eq!(rect(&tree, hug), [101.0, 0.0, 160.0, 60.0]);
    assert_eq!(rect(&tree, off), [93.0, 0.0, 176.0, 60.0]);
    assert_eq!(rect(&tree, stretched), [0.0, 0.0, 176.0, 60.0]);
}

#[test]
fn a_min_wins_over_a_hug_narrower_than_it() {
    let mut tree = Tree::new();
    let mut words = Words::default();
    // Hugging, the text would be 320 (100 + 100 + 100, then 50); its min is 340, so it is 340, and
    // within 340 it still breaks as 320 and 50: two lines — in a row and at a column's start.
    let at_least = Style { min_size: Size::new(px(340.0), Length::Auto), ..Style::default() };
    let row = screen(&mut tree, 362.0, 400.0, start_row());
    let in_row = words.add(&mut tree, row, at_least.clone(), &PARAGRAPH, false, true);
    let col = screen(&mut tree, 362.0, 400.0, Style { align_items: AlignItems::FlexStart, ..column() });
    let in_col = words.add(&mut tree, col, at_least, &PARAGRAPH, false, true);
    tree.compute_layout(&mut words);
    assert_eq!(rect(&tree, in_row), [0.0, 0.0, 340.0, 40.0]);
    assert_eq!(rect(&tree, in_col), [0.0, 0.0, 340.0, 40.0]);
}
