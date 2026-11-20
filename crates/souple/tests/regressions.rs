//! The bugs the randomized test (`fuzz.rs`) found, one test each, in the smallest tree that shows
//! them — and two in the measure cache, which its text (words of one width) cannot show.

use souple::{
    AlignItems, Available, Edges, FlexDirection, JustifyContent, Length, MeasureInput, MeasureOutput, NoMeasure,
    NodeId, NodeLayout, Size, Style, Tree, Viewport,
};

fn every_number(layout: &NodeLayout) -> [f32; 18] {
    let NodeLayout { rect, margin, padding, border, content_size } = *layout;
    let e = |e: Edges<f32>| [e.left, e.top, e.right, e.bottom];
    let [ml, mt, mr, mb] = e(margin);
    let [pl, pt, pr, pb] = e(padding);
    let [bl, bt, br, bb] = e(border);
    [rect.x, rect.y, rect.width, rect.height, ml, mt, mr, mb, pl, pt, pr, pb, bl, bt, br, bb, content_size.width, content_size.height]
}

#[test]
fn a_huge_scale_publishes_a_finite_box() {
    // The engine snapped 768 px at a scale of f32::MAX to an infinite width.
    let mut tree = Tree::new();
    let root = tree.create_node_with_style(Style { justify_content: JustifyContent::SpaceAround, ..Style::default() });
    tree.set_scale(f32::MAX);
    tree.set_viewport(root, Viewport::size(768.0, 390.0));
    tree.compute_layout(&mut NoMeasure);
    assert!(every_number(tree.layout(root)).iter().all(|n| n.is_finite()), "{:?}", tree.layout(root));
}

#[test]
fn hostile_lengths_publish_a_finite_box() {
    let mut tree = Tree::new();
    let root = tree.create_node();
    tree.set_viewport(root, Viewport::size(400.0, 400.0));
    let child = tree.create_node_with_style(Style {
        size: Size::new(Length::Px(f32::INFINITY), Length::Px(3e38)),
        margin: Edges::all(Length::Px(f32::NAN)),
        padding: Edges::all(Length::Px(f32::INFINITY)),
        border: Edges::all(Length::Px(f32::MAX)),
        flex_grow: f32::NAN,
        ..Style::default()
    });
    tree.append(root, child);
    let grandchild = tree.create_node_with_style(Style { margin: Edges::all(Length::Px(-1e30)), ..Style::default() });
    tree.append(child, grandchild);
    tree.compute_layout(&mut NoMeasure);
    for node in [root, child, grandchild] {
        assert!(every_number(tree.layout(node)).iter().all(|n| n.is_finite()), "{:?}", tree.layout(node));
    }
}

#[test]
fn a_scale_that_is_not_a_positive_finite_number_is_ignored() {
    // It panicked (0, a negative or NaN), or snapped every box to NaN (infinity).
    let mut tree = Tree::new();
    let root = tree.create_node();
    tree.set_viewport(root, Viewport::size(100.0, 100.0));
    let child = tree.create_node_with_style(Style { size: Size::new(Length::Px(10.25), Length::Px(10.0)), ..Style::default() });
    tree.append(root, child);
    tree.set_scale(2.0);
    for scale in [0.0, -0.0, -2.0, f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        tree.set_scale(scale);
    }
    tree.compute_layout(&mut NoMeasure);
    assert_eq!(tree.layout(child).rect.width, 10.5, "still the half-pixel grid of scale 2");
}

#[test]
fn a_viewport_axis_that_is_not_finite_is_unbounded() {
    let content = |viewport: Viewport| {
        let mut tree = Tree::new();
        let root = tree.create_node();
        let child = tree.create_node_with_style(Style { size: Size::new(Length::Px(50.0), Length::Px(20.0)), ..Style::default() });
        tree.append(root, child);
        tree.set_viewport(root, viewport);
        tree.compute_layout(&mut NoMeasure);
        // The same viewport again is no change: a NaN equals no NaN, and was laid out every pass.
        tree.set_viewport(root, viewport);
        assert!(!tree.needs_layout());
        tree.layout(root).rect
    };
    let unbounded = content(Viewport::unbounded());
    assert_eq!(content(Viewport::Size(Size::new(Some(f32::NAN), Some(f32::INFINITY)))), unbounded);
    assert_eq!(content(Viewport::Size(Size::new(Some(f32::NEG_INFINITY), None))), unbounded);
}

#[test]
#[should_panic(expected = "hosted inside itself")]
fn a_cycle_through_a_shadowed_scope_host_is_refused() {
    // A root's scope host is not followed while it has a `Viewport::Node`, so the check looked past
    // it: the loop below was accepted and closed the moment the viewport went — and every walk up
    // the scope chain (`var`, the next pass) never ended.
    let mut tree = Tree::new();
    let tree_of = |tree: &mut Tree| {
        let root = tree.create_node();
        let child = tree.create_node();
        tree.append(root, child);
        (root, child)
    };
    let (a, a1) = tree_of(&mut tree);
    let (_b, h1) = tree_of(&mut tree);
    let (c, c1) = tree_of(&mut tree);
    tree.set_viewport(a, Viewport::node(h1));
    tree.set_scope_host(a, Some(c1));
    // a → c1 → c → a1 → a: refused now (it was accepted).
    tree.set_scope_host(c, Some(a1));
    tree.clear_viewport(a);
    tree.var(a1, souple::VarId::new(0));
}

/// The published layouts of `nodes` in `incremental`, and in a tree built by `build` from scratch.
fn same_as_from_scratch(incremental: &Tree, nodes: &[NodeId], build: impl Fn(&mut Tree) -> Vec<NodeId>) {
    let mut fresh = Tree::new();
    let fresh_nodes = build(&mut fresh);
    fresh.compute_layout(&mut NoMeasure);
    for (a, b) in nodes.iter().zip(&fresh_nodes) {
        assert_eq!(incremental.layout(*a), fresh.layout(*b));
    }
}

#[test]
fn a_subtree_moved_by_whole_pixels_across_zero_publishes_the_same_boxes_as_a_fresh_tree() {
    // `inner` sits at y -0.5, a tie. Rounded away from zero it went to -1; its wrapper moved down
    // 1 px, `inner` to 0.5, which rounds to 1 — and the publish walk, which skips a subtree moved by
    // whole pixels, left `inner` at the -1 it had: one device px from a layout from scratch. A tie
    // now rounds toward +∞ (-0.5 to 0, 0.5 to 1), the same pixel wherever the subtree sits, so the
    // skipped subtree is exact: `inner` keeps its box, is not reported, and matches a fresh tree.
    let build = |spacer_height: f32| {
        move |tree: &mut Tree| {
            let root = tree.create_node_with_style(Style { flex_direction: FlexDirection::Column, ..Style::default() });
            tree.set_viewport(root, Viewport::size(100.0, 100.0));
            let spacer = tree.create_node_with_style(Style { size: Size::new(Length::Auto, Length::Px(spacer_height)), ..Style::default() });
            let wrapper = tree.create_node_with_style(Style { size: Size::new(Length::Px(10.0), Length::Px(10.0)), ..Style::default() });
            let inner = tree.create_node_with_style(Style {
                margin: Edges { top: Length::Px(-0.5), ..Edges::all(Length::ZERO) },
                size: Size::new(Length::Px(5.0), Length::Px(5.0)),
                ..Style::default()
            });
            tree.append(root, spacer);
            tree.append(root, wrapper);
            tree.append(wrapper, inner);
            vec![spacer, wrapper, inner]
        }
    };
    let mut tree = Tree::new();
    let nodes = build(0.0)(&mut tree);
    tree.compute_layout(&mut NoMeasure);
    assert_eq!(tree.layout(nodes[2]).rect.y, 0.0, "the tie at -0.5 rounds up, to 0");
    // Down across zero, further down, back up across it again: whole pixels each time.
    for height in [1.0, 3.0, 0.0, 2.0] {
        tree.update_style(nodes[0], |style| style.size.height = Length::Px(height));
        tree.compute_layout(&mut NoMeasure);
        assert!(tree.changes().iter().all(|change| change.node != nodes[2]), "`inner` keeps its box at {height}");
        assert_eq!(tree.layout(nodes[2]).rect.y, 0.0);
        same_as_from_scratch(&tree, &nodes, build(height));
    }
}

#[test]
fn a_root_shown_again_after_a_scale_change_is_snapped_at_the_new_scale() {
    // The scale changed and a pass ran while the root had no viewport: that pass cleared the flag
    // that re-snaps every box, and when the root was shown again its cached subtree kept the boxes
    // snapped at the old scale (the leaf at x 1.142857 = 3 / 2.625, not 1 = 3 / 3).
    let build = |tree: &mut Tree| {
        let root = tree.create_node();
        let child = tree.create_node_with_style(Style { padding: Edges::all(Length::Px(1.1)), ..Style::default() });
        let leaf = tree.create_node_with_style(Style { size: Size::new(Length::Px(5.3), Length::Px(5.3)), ..Style::default() });
        tree.append(root, child);
        tree.append(child, leaf);
        vec![root, child, leaf]
    };
    let mut tree = Tree::new();
    let nodes = build(&mut tree);
    tree.set_scale(2.625);
    tree.set_viewport(nodes[0], Viewport::size(400.0, 400.0));
    tree.compute_layout(&mut NoMeasure);
    tree.set_scale(3.0);
    tree.clear_viewport(nodes[0]);
    tree.compute_layout(&mut NoMeasure);
    assert!(tree.needs_layout_at(nodes[0]), "a root without a viewport is dirty after a scale change too");
    tree.set_viewport(nodes[0], Viewport::size(1000.0, 400.0));
    tree.compute_layout(&mut NoMeasure);
    assert_eq!(tree.layout(nodes[2]).rect.x, 1.0);
    same_as_from_scratch(&tree, &nodes, |fresh| {
        let nodes = build(fresh);
        fresh.set_scale(3.0);
        fresh.set_viewport(nodes[0], Viewport::size(1000.0, 400.0));
        nodes
    });
}

#[test]
fn a_box_far_from_the_origin_publishes_an_exact_length() {
    // Each edge snapped in logical px and then subtracted: at y 10 000 an f32 is 0.001 px coarse,
    // and a box of 50 device px at scale 3 was published 16.66699 tall.
    let mut tree = Tree::new();
    let root = tree.create_node_with_style(Style { flex_direction: FlexDirection::Column, ..Style::default() });
    tree.set_scale(3.0);
    tree.set_viewport(root, Viewport::Size(Size::new(Some(400.0), None)));
    let spacer = tree.create_node_with_style(Style { size: Size::new(Length::Auto, Length::Px(10_000.0 + 1.0 / 3.0)), ..Style::default() });
    let item = tree.create_node_with_style(Style { size: Size::new(Length::Auto, Length::Px(50.0 / 3.0)), ..Style::default() });
    tree.append(root, spacer);
    tree.append(root, item);
    tree.compute_layout(&mut NoMeasure);
    assert_eq!(tree.layout(item).rect.height, 50.0 / 3.0);
    assert_eq!(tree.layout(item).rect.y, 30_001.0 / 3.0);
}

// ── measured leaves ──────────────────────────────────────────────────────────────────────────

#[test]
fn a_measured_leafs_baseline_does_not_depend_on_what_it_was_asked_before() {
    // `row` is the first item of `line`, which lines up baselines; its baseline is its leaf's. The
    // leaf fills `row` (grow, stretch): both its sizes are known, so it is not measured for its
    // size. Its baseline was the one the measurer had given AT THAT WIDTH — if it was ever asked
    // there. Laid out first as a column, the leaf was asked at its known width 100: baseline 8. A
    // tree built as a row never asked it there: no baseline, synthesized from the bottom (20) — and
    // `badge` sat 10 px lower than in the tree that has the history. Now a baseline read at a known
    // size with no answer at that width asks the measurer there. CSS: 8, in both.
    let build = |tree: &mut Tree, direction: FlexDirection| {
        let line = tree.create_node_with_style(Style { align_items: AlignItems::Baseline, ..Style::default() });
        tree.set_viewport(line, Viewport::size(200.0, 100.0));
        let row = tree.create_node_with_style(Style {
            flex_direction: direction,
            size: Size::new(Length::Px(100.0), Length::Px(20.0)),
            ..Style::default()
        });
        let leaf = tree.create_node_with_style(Style { flex_grow: 1.0, ..Style::default() });
        let badge = tree.create_node_with_style(Style { size: Size::new(Length::Px(10.0), Length::Px(10.0)), ..Style::default() });
        tree.set_measured(leaf, true);
        tree.append(line, row);
        tree.append(row, leaf);
        tree.append(line, badge);
        vec![line, row, leaf, badge]
    };
    let mut tree = Tree::new();
    let nodes = build(&mut tree, FlexDirection::Column);
    tree.compute_layout(&mut text(&[30.0]));
    tree.update_style(nodes[1], |style| style.flex_direction = FlexDirection::Row);
    tree.compute_layout(&mut text(&[30.0]));
    let mut fresh = Tree::new();
    let fresh_nodes = build(&mut fresh, FlexDirection::Row);
    fresh.compute_layout(&mut text(&[30.0]));
    for (a, b) in nodes.iter().zip(&fresh_nodes) {
        assert_eq!(tree.layout(*a), fresh.layout(*b));
    }
}

/// Text the way a real measurer lays it out: words of their own widths, broken greedily at spaces
/// of 10 px, lines of 10 px.
fn text(words: &'static [f32]) -> impl FnMut(NodeId, MeasureInput) -> MeasureOutput {
    move |_, input| {
        let limit = match (input.known.width, input.available.width) {
            (Some(width), _) | (None, Available::Definite(width)) => Some(width),
            (None, Available::MaxContent) => None,
            (None, Available::MinContent) => Some(0.0),
        };
        let (mut lines, mut line, mut longest) = (0.0f32, 0.0f32, 0.0f32);
        for &word in words {
            if lines == 0.0 {
                (lines, line) = (1.0, word);
            } else if limit.is_some_and(|limit| line + 10.0 + word > limit) {
                (lines, line) = (lines + 1.0, word);
            } else {
                line += 10.0 + word;
            }
            longest = longest.max(line);
        }
        MeasureOutput { size: Size::new(longest, lines * 10.0), baseline: Some(8.0) }
    }
}

#[test]
fn a_text_squeezed_to_its_min_content_width_is_laid_out_at_that_width() {
    // "a bb cccccc" (10, 20 and 60 px) in a 50 px row: the auto minimum keeps it 60 wide, and at 60
    // px it is two lines ("a bb" and "cccccc"), 20 px. The min-content answer (one word a line, 30
    // px) was reused for the question at the known width 60, as the contract's old rule "a known
    // size equal to one the content answered gives the same layout" allowed — which text breaks:
    // an answer serves a known size only where the room it was offered held it.
    let mut tree = Tree::new();
    let row = tree.create_node_with_style(Style {
        size: Size::new(Length::Px(50.0), Length::Px(200.0)),
        align_items: AlignItems::FlexStart,
        ..Style::default()
    });
    tree.set_viewport(row, Viewport::unbounded());
    let leaf = tree.create_node();
    tree.append(row, leaf);
    tree.set_measured(leaf, true);
    tree.compute_layout(&mut text(&[10.0, 20.0, 60.0]));
    assert_eq!(tree.layout(leaf).rect.width, 60.0);
    assert_eq!(tree.layout(leaf).rect.height, 20.0, "the text at its width: two lines");
}

#[test]
fn a_text_wider_than_its_offer_is_laid_out_at_its_own_width() {
    // "aa aa bbbbb" (20, 20, 50 px) offered 30 px: three lines, 50 wide (the long word overflows).
    // Laid out at that width, 50, it is two lines ("aa aa", "bbbbb"), 20 px. The leaf kept the 30
    // of the answer at 30; now it asks again at its own width, and the cache does not answer that
    // from the answer the 30 px did not hold.
    let mut tree = Tree::new();
    let column = tree.create_node_with_style(Style {
        flex_direction: FlexDirection::Column,
        size: Size::new(Length::Px(30.0), Length::Px(200.0)),
        align_items: AlignItems::FlexStart,
        ..Style::default()
    });
    tree.set_viewport(column, Viewport::unbounded());
    let leaf = tree.create_node();
    tree.append(column, leaf);
    tree.set_measured(leaf, true);
    tree.compute_layout(&mut text(&[20.0, 20.0, 50.0]));
    assert_eq!(tree.layout(leaf).rect.width, 50.0);
    assert_eq!(tree.layout(leaf).rect.height, 20.0, "the text at its width: two lines");
}
