# Changelog

The `souple` crate (`crates/souple`). `souple-syntax`, the bench and the conformance tests are not
published.

## 0.2.0 — unreleased

### Breaking

- `Viewport::Node` / `Viewport::node` are `Viewport::Host` / `Viewport::host`: a root shown inside a
  host node.
- `NodeLayout` says which of its numbers are snapped: `rect` and `content_size` (snapped to the
  device grid) as before, and the resolved margins, padding and border widths — unrounded — under
  `unsnapped`, a `BoxEdges` (`layout.padding` → `layout.unsnapped.padding`).
- `Change::inner` is `Change::box_model`.
- `Length::resolve(base)` is `Length::resolve(base, value_of)`, and `resolve_or_zero` likewise: a
  variable's value is read from `value_of` (as `Formula::eval` takes it), so a `Var` length — or a
  formula over variables — resolves. It debug-asserted, and answered 0 for the variable in release.
- `MAX_VARS` is a `usize` (it was a `u8`).
- The refused edits' panics read `Tree::insert: <the reason>` (the `EditError`'s `Display`), one
  text per reason.

### Added

- `Tree::try_insert`, `Tree::try_append`, `Tree::try_set_viewport`, `Tree::try_set_scope_host`:
  the edit, or `Err(EditError)` — and then the tree is exactly as it was. The plain forms are these
  and a panic. `EditError` (`#[non_exhaustive]`, `Display`, `std::error::Error`): `Destroyed` (a
  stale `NodeId`), `Cycle`, `HasViewport`, `NotARoot`, `NotASibling`.
- `Tree::edit_style(node, |style| -> bool)`: an edit of the style where it is, the closure answering
  whether it changed anything — no copy and no comparison of the whole style (`update_style` copies
  it, `set_style` compares it). Measured in LeCodes' UI engine, one property per call: a style edit
  94 → 29 ns, the style writes of a 1,000-node screen −19%.
- `Length::calc(&[Op])`: the documented way to make a formula length (`Formula::build` is the same
  call).
- `Debug` for `Tree`: `{:?}` a bounded dump (the counts; each root with its subtree to 3 levels and
  64 node lines in all, what is left out counted), `{:#?}` every node — its box, its flags, its
  links, the fields of its style that differ from the default.
- `BoxEdges`: a node's unrounded margins, padding and border widths (`NodeLayout::unsnapped`).

### Changed

- `Tree::child_count` is O(1): each node counts its children (it walked them). An embedder that
  appends at `child_count` — LeCodes' runtime does at every insert — no longer walks the list: on a
  10,000-node screen whose list has 1,315 children, its adopts 6.8 → 2.6 ms.
- `set_style` and `edit_style` share the restyle (what the style reads, what the next pass lays out):
  no change of behaviour.

## 0.1.0 — 2026-11-20

The first release: an incremental CSS flexbox layout engine — a tree of boxes, lengths kept live
over scoped variables (`em`, `vw`, a safe-area inset are the embedder's numbered variables),
measured leaves with a cached `Measurer`, pixel snapping by absolute position, and the boxes that
changed as each pass's output. Yoga's conformance suite passes but for the documented differences
(the README's "Limits").
