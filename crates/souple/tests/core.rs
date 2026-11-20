//! The engine around the algorithm: the tree, the variable scopes, what a pass has to do.
//! (The boxes themselves are the fixture suite's.)

use souple::{EditError, Length, NoMeasure, NodeId, Size, Style, Tree, VarId, Viewport};

const EM: VarId = VarId::new(0);
const VW: VarId = VarId::new(1);

fn width(length: Length) -> Style {
    Style { size: Size::new(length, Length::Auto), ..Style::default() }
}

fn screen(tree: &mut Tree) -> NodeId {
    let root = tree.create_node();
    tree.set_viewport(root, Viewport::size(400.0, 800.0));
    root
}

fn children(tree: &Tree, node: NodeId) -> Vec<NodeId> {
    tree.children(node).collect()
}

#[test]
fn insert_moves_and_orders() {
    let mut tree = Tree::new();
    let parent = tree.create_node();
    let [a, b, c] = [tree.create_node(), tree.create_node(), tree.create_node()];
    tree.append(parent, a);
    tree.append(parent, c);
    tree.insert(parent, b, Some(c));
    assert_eq!(children(&tree, parent), [a, b, c]);

    // A move within the parent is the same call.
    tree.insert(parent, c, Some(a));
    assert_eq!(children(&tree, parent), [c, a, b]);
    tree.append(parent, c);
    assert_eq!(children(&tree, parent), [a, b, c]);

    // A move to another parent.
    let other = tree.create_node();
    tree.append(other, b);
    assert_eq!(children(&tree, parent), [a, c]);
    assert_eq!(tree.parent(b), Some(other));

    tree.detach(a);
    assert_eq!(children(&tree, parent), [c]);
    assert_eq!(tree.parent(a), None);
}

#[test]
fn destroy_frees_the_subtree_and_stales_its_ids() {
    let mut tree = Tree::new();
    let parent = tree.create_node();
    let child = tree.create_node();
    let grandchild = tree.create_node();
    tree.append(parent, child);
    tree.append(child, grandchild);
    tree.destroy(child);
    assert!(!tree.is_alive(child) && !tree.is_alive(grandchild));
    assert_eq!(tree.child_count(parent), 0);

    // The slot is reused under another generation: the old id stays stale.
    let reused = tree.create_node();
    assert_ne!(reused, grandchild);
    assert_ne!(reused, child);
    assert!(tree.is_alive(reused) && !tree.is_alive(child));
}

#[test]
#[should_panic(expected = "under itself")]
fn a_cycle_is_refused() {
    let mut tree = Tree::new();
    let parent = tree.create_node();
    let child = tree.create_node();
    tree.append(parent, child);
    tree.append(child, parent);
}

#[test]
fn a_variable_reads_the_nearest_definition() {
    let mut tree = Tree::new();
    let root = tree.create_node();
    let middle = tree.create_node();
    let leaf = tree.create_node();
    tree.append(root, middle);
    tree.append(middle, leaf);

    assert_eq!(tree.var(leaf, EM), 0.0, "undefined reads 0");
    tree.set_var(root, EM, Some(16.0));
    assert_eq!(tree.var(leaf, EM), 16.0);
    tree.set_var(middle, EM, Some(20.0));
    assert_eq!(tree.var(leaf, EM), 20.0);
    assert_eq!(tree.var(root, EM), 16.0);
    tree.set_var(middle, EM, None);
    assert_eq!(tree.var(leaf, EM), 16.0);
}

#[test]
fn a_hosted_root_reads_through_its_host() {
    let mut tree = Tree::new();
    let root = screen(&mut tree);
    let host = tree.create_node();
    tree.append(root, host);
    tree.set_var(root, VW, Some(4.0));

    let page = tree.create_node();
    tree.set_viewport(page, Viewport::host(host));
    assert_eq!(tree.var(page, VW), 4.0);

    tree.destroy(host);
    assert_eq!(tree.var(page, VW), 0.0, "a root whose host is gone has no viewport and no chain");
}

#[test]
fn a_pass_is_needed_only_when_something_changed() {
    let mut tree = Tree::new();
    let root = screen(&mut tree);
    let child = tree.create_node();
    tree.append(root, child);
    tree.set_style(child, width(Length::Var(EM, 2.0)));
    tree.set_var(root, EM, Some(16.0));
    assert!(tree.needs_layout());
    tree.compute_layout(&mut NoMeasure);
    assert!(!tree.needs_layout());

    // The same style, the same value: nothing.
    tree.set_style(child, width(Length::Var(EM, 2.0)));
    tree.set_var(root, EM, Some(16.0));
    assert!(!tree.needs_layout());

    // A variable nobody reads: resolved away without a layout.
    tree.set_var(root, VW, Some(4.0));
    tree.compute_layout(&mut NoMeasure);
    assert!(!tree.needs_layout());

    // A variable that is read: a layout.
    tree.set_var(root, EM, Some(20.0));
    assert!(tree.needs_layout());
    tree.compute_layout(&mut NoMeasure);

    // A change that leaves the computed value alone (2em shadowed by the child's own em).
    tree.set_var(child, EM, Some(20.0));
    tree.set_var(root, EM, Some(99.0));
    tree.compute_layout(&mut NoMeasure);
    assert!(!tree.needs_layout());
}

#[test]
fn a_root_without_a_viewport_asks_for_no_pass() {
    let mut tree = Tree::new();
    let detached = tree.create_node();
    let child = tree.create_node();
    tree.append(detached, child);
    tree.set_style(child, width(Length::Px(10.0)));
    assert!(!tree.needs_layout());
}

#[test]
fn the_root_takes_its_viewport_and_reports_the_change_once() {
    let mut tree = Tree::new();
    let root = screen(&mut tree);
    tree.compute_layout(&mut NoMeasure);
    let changes = tree.changes().to_vec();
    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0].node, root);
    assert!(changes[0].resized && !changes[0].moved);
    assert_eq!(tree.layout(root).rect.width, 400.0);

    tree.set_viewport(root, Viewport::size(400.0, 800.0));
    tree.compute_layout(&mut NoMeasure);
    assert!(tree.changes().is_empty(), "the same viewport is no change");
    tree.set_viewport(root, Viewport::size(300.0, 800.0));
    tree.compute_layout(&mut NoMeasure);
    assert_eq!(tree.changes().len(), 1);
}

#[test]
fn a_variable_change_moves_only_what_reads_it() {
    let mut tree = Tree::new();
    let root = screen(&mut tree);
    let [sized, sibling] = [tree.create_node(), tree.create_node()];
    tree.append(root, sized);
    tree.append(root, sibling);
    tree.set_var(root, EM, Some(10.0));
    tree.set_style(sized, width(Length::Var(EM, 2.0)));
    tree.set_style(sibling, width(Length::Px(50.0)));
    tree.compute_layout(&mut NoMeasure);
    assert_eq!(tree.layout(sized).rect.width, 20.0);
    assert_eq!(tree.layout(sibling).rect.x, 20.0);

    tree.set_var(root, EM, Some(15.0));
    tree.compute_layout(&mut NoMeasure);
    let changed: Vec<NodeId> = tree.changes().iter().map(|change| change.node).collect();
    assert_eq!(tree.layout(sized).rect.width, 30.0);
    assert_eq!(tree.layout(sibling).rect.x, 30.0);
    assert_eq!(changed, [sized, sibling], "the root's box is as it was");
}

#[test]
fn a_hosted_root_follows_its_hosts_content_box() {
    let mut tree = Tree::new();
    let root = screen(&mut tree);
    let host = tree.create_node();
    tree.append(root, host);
    tree.set_style(
        host,
        Style {
            size: Size::new(Length::Px(200.0), Length::Px(100.0)),
            padding: souple::Edges::all(Length::Px(10.0)),
            ..Style::default()
        },
    );
    let page = tree.create_node();
    let fill = tree.create_node();
    tree.append(page, fill);
    tree.set_style(fill, width(Length::Percent(50.0)));
    tree.set_viewport(page, Viewport::host(host));
    tree.compute_layout(&mut NoMeasure);
    // The page is the host's content box (the default box-sizing sizes it: 200 × 100), inside the padding.
    assert_eq!((tree.layout(host).rect.width, tree.layout(host).rect.height), (220.0, 120.0));
    assert_eq!((tree.layout(page).rect.width, tree.layout(page).rect.height), (200.0, 100.0));
    assert_eq!(tree.layout(fill).rect.width, 100.0);

    // The host resized: the page follows in the same pass.
    tree.update_style(host, |style| style.size.width = Length::Px(300.0));
    tree.compute_layout(&mut NoMeasure);
    assert_eq!(tree.layout(page).rect.width, 300.0);
    assert_eq!(tree.layout(fill).rect.width, 150.0);
}

#[test]
fn an_edit_under_a_hidden_subtree_still_reaches_the_root() {
    use souple::Display;
    let mut tree = Tree::new();
    let root = screen(&mut tree);
    let hidden = tree.create_node();
    let inner = tree.create_node();
    tree.append(root, hidden);
    tree.append(hidden, inner);
    tree.set_style(inner, width(Length::Px(40.0)));
    tree.update_style(hidden, |style| style.display = Display::None);
    tree.compute_layout(&mut NoMeasure);

    // No pass reaches `inner` while it is hidden, so it stays dirty — a later edit of it must
    // still mark the ancestors a pass cleaned.
    tree.set_style(inner, width(Length::Px(50.0)));
    tree.compute_layout(&mut NoMeasure);
    tree.set_style(inner, width(Length::Px(60.0)));
    tree.update_style(hidden, |style| style.display = Display::Flex);
    tree.compute_layout(&mut NoMeasure);
    assert_eq!(tree.layout(inner).rect.width, 60.0);
    assert_eq!(tree.layout(hidden).rect.width, 60.0);
}

#[test]
fn a_tree_built_children_first_resolves_its_variables() {
    let mut tree = Tree::new();
    // Leaf first, then each parent around it: every insert queues the subtree for a resolve.
    let leaf = tree.create_node();
    tree.set_style(leaf, width(Length::Var(EM, 2.0)));
    let middle = tree.create_node();
    tree.append(middle, leaf);
    tree.set_var(middle, VW, Some(1.0));
    let outer = tree.create_node();
    tree.append(outer, middle);
    let root = screen(&mut tree);
    tree.append(root, outer);
    tree.set_var(root, EM, Some(8.0));
    tree.compute_layout(&mut NoMeasure);
    assert_eq!(tree.layout(leaf).rect.width, 16.0);

    // A definition appearing between the root and the leaf shadows the root's.
    tree.set_var(outer, EM, Some(10.0));
    tree.compute_layout(&mut NoMeasure);
    assert_eq!(tree.layout(leaf).rect.width, 20.0);
    tree.set_var(root, EM, Some(99.0));
    tree.compute_layout(&mut NoMeasure);
    assert!(tree.changes().is_empty());
}

#[test]
fn a_move_by_whole_pixels_reports_only_the_box_that_moved() {
    let mut tree = Tree::new();
    let root = screen(&mut tree);
    let [first, second, inside] = [tree.create_node(), tree.create_node(), tree.create_node()];
    tree.append(root, first);
    tree.append(root, second);
    tree.append(second, inside);
    tree.set_style(first, width(Length::Px(10.0)));
    tree.set_style(inside, width(Length::Px(5.5)));
    tree.compute_layout(&mut NoMeasure);

    tree.set_style(first, width(Length::Px(30.0)));
    tree.compute_layout(&mut NoMeasure);
    let changed: Vec<NodeId> = tree.changes().iter().map(|change| change.node).collect();
    // `inside` keeps its place in `second`: its published box is relative.
    assert_eq!(changed, [first, second]);
    assert_eq!(tree.layout(second).rect.x, 30.0);

    // A move by half a pixel shifts what is snapped under it.
    tree.set_style(first, width(Length::Px(30.5)));
    tree.compute_layout(&mut NoMeasure);
    assert_eq!(tree.layout(second).rect.x, 31.0);
    assert_eq!(tree.layout(inside).rect.width, 5.0, "30.5 + 5.5 = 36, from 31");
}

#[test]
fn a_node_created_with_its_style_is_the_node_set_style_makes() {
    // The same tree twice: styled after the fact, and created styled (in a tree with its room
    // reserved, one slot of it freed and reused) — one of the styles reads a variable.
    let styles = [width(Length::Px(120.0)), width(Length::Var(EM, 2.0)), width(Length::Percent(25.0))];
    let mut set = Tree::new();
    let set_root = screen(&mut set);
    set.set_var(set_root, EM, Some(16.0));
    let set_nodes: Vec<NodeId> = styles
        .iter()
        .map(|style| {
            let node = set.create_node();
            set.append(set_root, node);
            set.set_style(node, style.clone());
            node
        })
        .collect();

    let mut created = Tree::with_capacity(8);
    let created_root = screen(&mut created);
    created.set_var(created_root, EM, Some(16.0));
    // A freed slot: the first node created styled reuses it.
    let freed = created.create_node();
    created.destroy(freed);
    let created_nodes: Vec<NodeId> = styles
        .iter()
        .map(|style| {
            let node = created.create_node_with_style(style.clone());
            created.append(created_root, node);
            node
        })
        .collect();

    set.compute_layout(&mut NoMeasure);
    created.compute_layout(&mut NoMeasure);
    for (a, b) in set_nodes.iter().zip(&created_nodes) {
        assert_eq!(set.style(*a), created.style(*b));
        assert_eq!(set.layout(*a), created.layout(*b));
    }
    assert_eq!(created.layout(created_nodes[1]).rect.width, 32.0, "2em of 16");
}

#[test]
fn a_global_is_read_where_no_node_defines_the_variable() {
    let mut tree = Tree::new();
    let root = screen(&mut tree);
    let [plain, shadowed] = [tree.create_node(), tree.create_node()];
    tree.append(root, plain);
    tree.append(root, shadowed);
    tree.set_style(plain, width(Length::Var(VW, 10.0)));
    tree.set_style(shadowed, width(Length::Var(VW, 10.0)));
    tree.set_var(shadowed, VW, Some(1.0));
    // A root no viewport lays out reads it too.
    let loose = tree.create_node();
    tree.set_style(loose, width(Length::Var(VW, 1.0)));

    tree.set_global(VW, 4.0);
    assert!(tree.needs_layout());
    tree.compute_layout(&mut NoMeasure);
    assert_eq!(tree.layout(plain).rect.width, 40.0);
    assert_eq!(tree.layout(shadowed).rect.width, 10.0, "a definition shadows the global");
    assert_eq!(tree.var(loose, VW), 4.0);

    tree.set_global(VW, 4.0);
    assert!(!tree.needs_layout(), "the same value again is no change");
    tree.set_global(VW, 5.0);
    tree.compute_layout(&mut NoMeasure);
    let changed: Vec<NodeId> = tree.changes().iter().map(|change| change.node).collect();
    assert_eq!(tree.layout(plain).rect.width, 50.0);
    assert_eq!(changed, [plain, shadowed], "the shadowed reader only moves");
}

#[test]
fn a_first_publish_is_a_change_even_of_an_empty_box() {
    let mut tree = Tree::new();
    let root = screen(&mut tree);
    let empty = tree.create_node();
    tree.set_style(empty, Style { position: souple::Position::Absolute, ..Style::default() });
    tree.append(root, empty);
    tree.compute_layout(&mut NoMeasure);
    let changes = tree.changes().to_vec();
    let first = changes.iter().find(|change| change.node == empty).expect("the 0 x 0 box is reported");
    assert!(!first.moved && !first.resized && !first.box_model, "nothing differs from the default box");
    assert_eq!(tree.layout(empty).rect.width, 0.0);
    tree.set_viewport(root, Viewport::size(300.0, 800.0));
    tree.compute_layout(&mut NoMeasure);
    assert!(tree.changes().iter().all(|change| change.node != empty), "and only once");
}

#[test]
fn the_resolved_margins_are_published() {
    let mut tree = Tree::new();
    let root = screen(&mut tree);
    let child = tree.create_node();
    tree.append(root, child);
    tree.set_style(child, Style {
        margin: souple::Edges { left: Length::Auto, top: Length::Px(4.0), right: Length::Percent(10.0), bottom: Length::Px(6.0) },
        size: Size::new(Length::Px(10.0), Length::Px(10.0)),
        ..Style::default()
    });
    tree.compute_layout(&mut NoMeasure);
    let margin = tree.layout(child).unsnapped.margin;
    assert_eq!((margin.left, margin.top, margin.right, margin.bottom), (0.0, 4.0, 40.0, 6.0), "auto is 0, a percent is of the width");
}

#[test]
fn a_size_viewport_taken_and_given_again_keeps_the_layout() {
    let mut tree = Tree::new();
    let root = tree.create_node();
    let child = tree.create_node();
    tree.append(root, child);
    tree.set_var(root, EM, Some(10.0));
    tree.set_style(child, width(Length::Var(EM, 3.0)));
    tree.set_viewport(root, Viewport::size(100.0, 100.0));
    tree.compute_layout(&mut NoMeasure);
    tree.clear_viewport(root);
    // Laid out on demand: the same box again has nothing to redo, and nothing is resolved anew.
    tree.set_viewport(root, Viewport::size(100.0, 100.0));
    tree.compute_layout(&mut NoMeasure);
    assert!(tree.changes().is_empty());
    assert_eq!(tree.layout(child).rect.width, 30.0);
}

#[test]
fn an_infinite_flex_factor_ends_the_pass() {
    // inf / inf is NaN: a flex loop that waited for a violation to freeze an item never ended.
    let mut tree = Tree::new();
    let root = screen(&mut tree);
    for grow in [f32::INFINITY, f32::INFINITY, 1.0] {
        let item = tree.create_node();
        tree.set_style(item, Style { flex_grow: grow, ..Style::default() });
        tree.append(root, item);
    }
    let shrinking = tree.create_node();
    tree.set_style(shrinking, Style { flex_shrink: f32::INFINITY, size: Size::new(Length::Px(900.0), Length::Auto), ..Style::default() });
    tree.append(root, shrinking);
    tree.compute_layout(&mut NoMeasure);
}

#[test]
fn a_scope_host_lends_its_variables_and_nothing_else() {
    let mut tree = Tree::new();
    let root = screen(&mut tree);
    let host = tree.create_node();
    tree.append(root, host);
    tree.set_style(host, width(Length::Px(300.0)));
    tree.set_var(host, EM, Some(10.0));
    // An item root the embedder lays out on its own, reading the host's `em`.
    let item = tree.create_node();
    let leaf = tree.create_node();
    tree.append(item, leaf);
    tree.set_style(leaf, width(Length::Var(EM, 3.0)));
    tree.set_scope_host(item, Some(host));
    assert_eq!(tree.scope_host(item), Some(host));
    assert_eq!(tree.var(leaf, EM), 10.0);
    tree.compute_layout(&mut NoMeasure);
    // No viewport: no pass lays the item out, the host's box is not its box.
    assert_eq!(tree.layout(leaf).rect.width, 0.0);
    tree.set_viewport(item, Viewport::size(100.0, 100.0));
    tree.compute_layout(&mut NoMeasure);
    assert_eq!(tree.layout(leaf).rect.width, 30.0);
    assert_eq!(tree.layout(item).rect.width, 100.0, "the item's box is its own viewport's");

    // A new value at the host re-resolves the hosted styles at the next pass.
    tree.set_var(host, EM, Some(20.0));
    tree.compute_layout(&mut NoMeasure);
    assert_eq!(tree.layout(leaf).rect.width, 60.0);

    // The link gone: the global is read again.
    tree.set_global(EM, 5.0);
    tree.set_scope_host(item, None);
    tree.compute_layout(&mut NoMeasure);
    assert_eq!(tree.layout(leaf).rect.width, 15.0);

    // An insert drops the link (a child's scope is its parent's); a destroyed host drops it too.
    tree.clear_viewport(item);
    tree.set_scope_host(item, Some(host));
    let other = tree.create_node();
    tree.append(root, other);
    tree.append(other, item);
    assert_eq!(tree.scope_host(item), None);
    tree.detach(item);
    tree.set_scope_host(item, Some(host));
    tree.destroy(host);
    assert_eq!(tree.scope_host(item), None);
    assert_eq!(tree.var(leaf, EM), 5.0);
}

#[test]
fn a_scope_host_inside_the_root_is_refused() {
    let mut tree = Tree::new();
    let item = tree.create_node();
    let inner = tree.create_node();
    tree.append(item, inner);
    let before = format!("{tree:#?}");
    assert_eq!(tree.try_set_scope_host(item, Some(inner)), Err(EditError::Cycle));
    assert_eq!(format!("{tree:#?}"), before, "refused: nothing changed");
}

#[test]
#[should_panic(expected = "Tree::set_scope_host: a node cannot be under itself")]
fn the_plain_form_panics_with_the_refusal() {
    let mut tree = Tree::new();
    let item = tree.create_node();
    let inner = tree.create_node();
    tree.append(item, inner);
    tree.set_scope_host(item, Some(inner));
}

#[test]
fn a_host_inserted_under_its_hosted_root_is_refused() {
    let mut tree = Tree::new();
    let host = tree.create_node();
    let item = tree.create_node();
    tree.set_scope_host(item, Some(host));
    let before = format!("{tree:#?}");
    assert_eq!(tree.try_append(item, host), Err(EditError::Cycle));
    assert_eq!(format!("{tree:#?}"), before, "refused: nothing changed");
}

#[test]
fn a_root_laid_out_on_demand_says_when_it_is_dirty() {
    let mut tree = Tree::new();
    let item = tree.create_node();
    let leaf = tree.create_node();
    tree.append(item, leaf);
    assert!(tree.needs_layout_at(item), "a new tree was never laid out");
    tree.set_viewport(item, Viewport::size(100.0, 100.0));
    tree.compute_layout(&mut NoMeasure);
    tree.clear_viewport(item);
    assert!(!tree.needs_layout_at(item) && !tree.needs_layout_at(leaf));
    tree.set_style(leaf, width(Length::Px(10.0)));
    assert!(tree.needs_layout_at(item) && tree.needs_layout_at(leaf), "an edit marks the chain to the root");
}

/// Two trees, edited the same way — one by `set_style`, the other in place by `edit_style` — lay
/// out, report and measure the same through every path a restyle takes: a plain length, the same
/// value (answered unchanged, and answered changed), a variable read for the first time and then
/// moved, a variable's factor, a variable dropped for the px it computed to (no layout), and for
/// other px.
#[test]
fn an_edit_in_place_is_the_edit_set_style_makes() {
    use souple::{Available, MeasureInput, MeasureOutput};
    struct Twin {
        tree: Tree,
        root: NodeId,
        box_: NodeId,
        text: NodeId,
    }
    fn twin() -> Twin {
        let mut tree = Tree::new();
        let root = screen(&mut tree);
        let box_ = tree.create_node();
        let text = tree.create_node();
        tree.append(root, box_);
        tree.append(box_, text);
        tree.set_measured(text, true);
        tree.set_var(root, EM, Some(10.0));
        Twin { tree, root, box_, text }
    }
    // A text that wraps: its size depends on the width it is offered.
    let pass = |twin: &mut Twin| {
        let mut calls = 0;
        let mut measure = |_: NodeId, input: MeasureInput| {
            calls += 1;
            let offered = match input.available.width {
                Available::Definite(width) => width,
                _ => 400.0,
            };
            let room = input.known.width.unwrap_or(offered).max(1.0);
            MeasureOutput { size: Size::new(room.min(300.0), (300.0 / room).ceil() * 10.0), baseline: None }
        };
        let needed = twin.tree.needs_layout();
        twin.tree.compute_layout(&mut measure);
        let changes = twin.tree.changes().to_vec();
        (needed, changes, calls, [twin.root, twin.box_, twin.text].map(|n| *twin.tree.layout(n)))
    };
    let (mut set, mut edit) = (twin(), twin());
    assert_eq!(pass(&mut set), pass(&mut edit), "the first pass");

    // (length, what the in-place edit answers: `None` = whether it changed, `Some` = that)
    let steps: [(Length, Option<bool>); 8] = [
        (Length::Px(200.0), None),            // no variable before or after: laid out
        (Length::Px(200.0), None),            // the same value: answered unchanged, nothing to do
        (Length::Px(200.0), Some(true)),      // the same value answered changed: no harm
        (Length::Var(EM, 15.0), None),        // a variable read for the first time: resolved at the pass
        (Length::Var(EM, 12.0), None),        // its factor moved: resolved again
        (Length::Px(120.0), None),            // dropped for the px it computed to: no layout
        (Length::Px(90.0), None),             // and for other px: laid out
        (Length::Var(EM, 9.0), None),         // read again; the variable's moves reach it (below)
    ];
    for (at, (length, answer)) in steps.into_iter().enumerate() {
        set.tree.set_style(set.box_, width(length.clone()));
        edit.tree.edit_style(edit.box_, |style| {
            let changed = style.size.width != length;
            style.size.width = length;
            answer.unwrap_or(changed)
        });
        assert_eq!(set.tree.style(set.box_), edit.tree.style(edit.box_), "step {at}");
        let (a, b) = (pass(&mut set), pass(&mut edit));
        assert_eq!(a.3, b.3, "step {at}: the boxes");
        if answer.is_none() {
            assert_eq!(a, b, "step {at}: what needed a pass, what changed, what was measured");
        } else {
            assert!(b.1.is_empty(), "step {at}: an unchanged style answered changed moves nothing");
        }
        if at == 1 || at == 5 {
            assert!(!b.0 && b.1.is_empty() && b.2 == 0, "step {at}: no pass needed, no change, no measure");
        }
    }
    // The variable the last edit made the box read: a new value reaches it.
    set.tree.set_var(set.root, EM, Some(20.0));
    edit.tree.set_var(edit.root, EM, Some(20.0));
    let (a, b) = (pass(&mut set), pass(&mut edit));
    assert_eq!(a, b);
    assert_eq!(b.3[1].rect.width, 180.0, "9em of 20");
}

/// Each refusal of the `try_` edits names its rule — and leaves the tree as it was: its dump (boxes,
/// dirtiness, links, styles), whether a pass is needed, and what the next pass reports.
#[test]
fn every_refusal_names_its_rule_and_changes_nothing() {
    let mut tree = Tree::new();
    let root = screen(&mut tree);
    let [child, grandchild] = [tree.create_node(), tree.create_node()];
    tree.append(root, child);
    tree.append(child, grandchild);
    let shown = screen(&mut tree); // another live root: it has a viewport
    let loose = tree.create_node(); // a root without one
    let gone = tree.create_node();
    tree.destroy(gone);
    tree.set_style(child, width(Length::Px(50.0)));
    tree.compute_layout(&mut NoMeasure);
    // (what, the edit over [root, child, grandchild, shown, loose, gone], the refusal)
    type Edit = fn(&mut Tree, [NodeId; 6]) -> Result<(), EditError>;
    let refusals: [(&str, Edit, EditError); 9] = [
        ("a node under itself", |t, [_, child, ..]| t.try_append(child, child), EditError::Cycle),
        ("a node under its grandchild", |t, [_, child, grandchild, ..]| t.try_append(grandchild, child), EditError::Cycle),
        ("a root with a viewport as a child", |t, [root, _, _, shown, ..]| t.try_append(root, shown), EditError::HasViewport),
        ("before = the child", |t, [root, _, _, _, loose, _]| t.try_insert(root, loose, Some(loose)), EditError::NotASibling),
        ("before not a child of the parent", |t, [root, _, grandchild, _, loose, _]| t.try_insert(root, loose, Some(grandchild)), EditError::NotASibling),
        ("a viewport on a child", |t, [_, child, ..]| t.try_set_viewport(child, Viewport::size(10.0, 10.0)), EditError::NotARoot),
        ("a root hosted inside itself", |t, [root, _, grandchild, ..]| t.try_set_viewport(root, Viewport::host(grandchild)), EditError::Cycle),
        ("a scope host on a child", |t, [_, child, _, _, loose, _]| t.try_set_scope_host(child, Some(loose)), EditError::NotARoot),
        ("a destroyed node", |t, [root, _, _, _, _, gone]| t.try_append(root, gone), EditError::Destroyed),
    ];
    for (what, edit, error) in refusals {
        let before = (format!("{tree:#?}"), tree.needs_layout());
        assert_eq!(edit(&mut tree, [root, child, grandchild, shown, loose, gone]), Err(error), "{what}");
        assert_eq!((format!("{tree:#?}"), tree.needs_layout()), before, "{what}: the tree is as it was");
        tree.compute_layout(&mut NoMeasure);
        assert!(tree.changes().is_empty(), "{what}: the next pass has nothing to report");
    }
    assert!(EditError::Cycle.to_string().contains("under itself"));
    // And an accepted one changes what it says.
    assert_eq!(tree.try_append(root, loose), Ok(()));
    assert_eq!(tree.parent(loose), Some(root));
}

#[test]
fn a_dump_shows_the_roots_their_boxes_and_what_their_styles_change() {
    let mut tree = Tree::new();
    let root = screen(&mut tree);
    let child = tree.create_node();
    tree.append(root, child);
    tree.set_style(child, Style { padding: souple::Edges::all(Length::Px(4.0)), ..width(Length::Px(120.0)) });
    tree.set_measured(child, true);
    let host = tree.create_node();
    tree.set_viewport(host, Viewport::host(child));
    tree.compute_layout(&mut NoMeasure);
    let dump = format!("{tree:?}");
    assert!(dump.starts_with("Tree { 3 nodes, 2 roots (2 live)"), "{dump}");
    assert!(dump.contains(&format!("n{} @ 0,0 400×800 [viewport 400×800]", root.index())), "{dump}");
    assert!(dump.contains(&format!("  n{} @ 0,0 128×800 measured {{ width: 120px, padding: 4px }}", child.index())), "{dump}");
    assert!(dump.contains(&format!("[hosted in n{}]", child.index())), "{dump}");
}

#[test]
fn a_dump_of_a_large_tree_is_bounded_unless_asked_for_all() {
    let mut tree = Tree::new();
    let root = screen(&mut tree);
    for _ in 0..100 {
        let row = tree.create_node();
        tree.append(root, row);
        for _ in 0..99 {
            let cell = tree.create_node();
            tree.append(row, cell);
        }
    }
    let short = format!("{tree:?}");
    assert!(short.lines().count() < 70 && short.len() < 8_000, "{} lines, {} bytes", short.lines().count(), short.len());
    assert!(short.contains("… 9937 more nodes under"), "{short}");
    let all = format!("{tree:#?}");
    assert_eq!(all.lines().count(), 1 + 10_001, "the summary and every node");
}
