//! The engine around the algorithm: the tree, the variable scopes, what a pass has to do.
//! (The boxes themselves are the fixture suite's.)

use souple::{Tree, Length, NoMeasure, NodeId, Size, Style, VarId, Viewport};

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
    tree.set_viewport(page, Viewport::node(host));
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
    tree.set_viewport(page, Viewport::node(host));
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
    assert!(!first.moved && !first.resized && !first.inner, "nothing differs from the default box");
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
    let margin = tree.layout(child).margin;
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
#[should_panic(expected = "hosted inside itself")]
fn a_scope_host_inside_the_root_is_refused() {
    let mut tree = Tree::new();
    let item = tree.create_node();
    let inner = tree.create_node();
    tree.append(item, inner);
    tree.set_scope_host(item, Some(inner));
}

#[test]
#[should_panic(expected = "inserted under itself")]
fn a_host_inserted_under_its_hosted_root_is_refused() {
    let mut tree = Tree::new();
    let host = tree.create_node();
    let item = tree.create_node();
    tree.set_scope_host(item, Some(host));
    tree.append(item, host);
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
