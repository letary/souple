# souple

A flexbox layout engine for UI trees that change. You keep a tree of boxes in it, write typed
styles, and call one pass per frame. The pass lays out only what changed and answers exactly the
boxes whose published layout moved.

- **CSS flexbox**, what Chrome does: the automatic minimum size, `flex-basis`, wrapping,
  `align-content`, baselines, absolute children, `aspect-ratio`, `box-sizing`, percent and
  `calc()` lengths. Checked against Yoga's Chrome-derived fixtures (457 of them pass).
- **Live lengths.** `em`, `vw` or a safe-area inset are variables you number and define on nodes;
  a length reads the nearest definition up the tree, and a new value re-lays out only the
  subtrees that read it.
- **Measured leaves.** Text and images are sized by a `Measurer` of yours, asked as rarely as the
  content allows (its answers are cached per node).
- **Pixel snapping** to the device grid, and a **changed-set** as the output of every pass.
- No dependencies, no `unsafe`, no strings (CSS text is the `souple-syntax` crate's).

## Quick start

```rust
use souple::{Edges, FlexDirection, Length, MeasureInput, MeasureOutput, NodeId, Rect, Size, Style, Tree, Viewport};

let mut tree = Tree::new();

// A screen: a column with 16 px of padding, a title, then a row of two buttons sharing the width.
let screen = tree.create_node_with_style(Style {
    flex_direction: FlexDirection::Column,
    padding: Edges::all(Length::Px(16.0)),
    ..Style::default()
});
let title = tree.create_node();
let row = tree.create_node_with_style(Style {
    margin: Edges { top: Length::Px(12.0), ..Edges::all(Length::ZERO) },
    column_gap: Length::Px(8.0),
    ..Style::default()
});
let button = Style { flex_grow: 1.0, size: Size::new(Length::Auto, Length::Px(44.0)), ..Style::default() };
let ok = tree.create_node_with_style(button.clone());
let cancel = tree.create_node_with_style(button);
tree.append(screen, title);
tree.append(screen, row);
tree.append(row, ok);
tree.append(row, cancel);

// The title is text: a measured leaf, sized by the embedder's measurer (a closure will do).
tree.set_measured(title, true);
let mut measure = |_node: NodeId, input: MeasureInput| MeasureOutput {
    size: Size::new(input.known.width.unwrap_or(120.0), 24.0),
    baseline: Some(18.0),
};

// A root is laid out once it has a viewport: here a phone screen, in logical px.
tree.set_viewport(screen, Viewport::size(390.0, 844.0));
tree.compute_layout(&mut measure);
assert_eq!(tree.changes().len(), 5, "the first pass publishes every node");

// A box is relative to its parent's border box, snapped to the device grid.
assert_eq!(tree.layout(title).rect, Rect { x: 16.0, y: 16.0, width: 358.0, height: 24.0 });
assert_eq!(tree.layout(cancel).rect, Rect { x: 183.0, y: 0.0, width: 175.0, height: 44.0 });

// Edit, then pass again: only what changed is laid out, and only what moved is reported.
tree.update_style(row, |style| style.column_gap = Length::Px(10.0));
tree.compute_layout(&mut measure);
for change in tree.changes() {
    let _rect = tree.layout(change.node).rect; // move the view of `change.node` there
}
assert_eq!(tree.layout(cancel).rect.x, 184.0);
```

## The concepts, in the order you need them

**Nodes.** A `Tree` owns a forest. `create_node` (or `create_node_with_style`, a builder's way in)
makes a detached node; `insert(parent, child, before)` / `append` puts it under a parent — and
MOVES it if it had one; `detach` makes it a root again; `destroy` frees it and its subtree;
`child_count` is kept, not counted. A `NodeId` is a slot and a generation: a destroyed node's id
goes stale, and every method that takes one panics on it rather than touch another node
(`is_alive` asks first). Every node is a flex container and a flex item at once. `{:?}` of a tree
is a screenful of it (each root and its first levels, what is left out counted); `{:#?}` is every
node.

**Edits that may be refused.** An insert, a viewport and a scope host may make no loop of parents
and hosts, and the tree refuses one that would — finding it is a walk up the chains, which an
embedder cannot cheaply do first. Each has a `try_` form (`try_insert`, `try_append`,
`try_set_viewport`, `try_set_scope_host`) that answers `Err(EditError)` — `Cycle`, `HasViewport`,
`NotARoot`, `NotASibling`, `Destroyed` — and then changes nothing; the plain form panics with the
same reason. An embedder whose edits come from an app or a host (and that runs with
`panic = "abort"`) calls the `try_` forms and refuses the request.

**Styles.** A `Style` is plain data under CSS's names, with CSS's initial values as its `Default`:
a row, `flex-shrink: 1`, `align-items: stretch`, `box-sizing: content-box`, auto sizes and
minimums. Write a whole one (`set_style`) or edit fields (`update_style(node, |s| ...)`); the same
style again costs a comparison and lays out nothing. An embedder that writes one property at a
time edits the style where it is (`edit_style(node, |s| ...)`, its closure answering whether it
changed anything): no copy, no comparison of the whole style.

**Measured leaves.** `set_measured(node, true)` makes a node content-sized: its size comes from the
`Measurer` passed to the pass, asked in content-box px with what the layout knows (`known`) and
offers (`available`) on each axis. The engine caches the answers per node and reuses one for any
question it implies: more room than the content took, or less that still holds it, and a known
size equal to an answer the room offered held — never a min-content answer, nor one wider than its
offer, which is the content squeezed and is asked again at the known size. A leaf whose two sizes
are known is not measured, unless its baseline is read and no answer at that width is cached (the
contract is on `Measurer`). `mark_dirty(node)` says the content changed.
By default a leaf follows CSS — a wrapping text in a row is as wide as the room it has. `set_hug`
makes a leaf as wide as its content says it is (its longest line, its ellipsized line), as Yoga,
React Native, SwiftUI and Flutter size text.

**Roots and viewports.** A root is laid out only once it has a viewport: `Viewport::size(w, h)`,
`::width(w)` (any height: a list item), `::height(h)`, `::unbounded()`, or `Viewport::host(host)` —
the content box of a node in another tree (a pager's page, a dialog over its screen), laid out
after its host in the same pass. `clear_viewport` takes it away; the boxes stay as they were.
`set_scale` sets the device px per logical px the boxes are snapped to (1 until set).

**The pass and its output.** `compute_layout(&mut measurer)` resolves what changed, lays it out
and snaps the boxes. Then `changes()` is a `&[Change]`: each node whose published layout differs
from the last pass's, once (a node published for the first time always). It stays readable until
the next pass, alongside `layout(node)` — the way to walk it while reading the tree. `layout(node)`
is the box (`NodeLayout`): the border box relative to the parent and the content size (what a
scroll container scrolls), both snapped to the device grid, and under `unsnapped` the resolved
margins, padding and border widths, unrounded (`Change::box_model`: those or the content size
moved). A pass with nothing to do changes
nothing and costs nothing; `needs_layout` says whether one would, and `needs_layout_at(node)` asks
the same of one subtree — whether anything under it changed, for an embedder that lays a root out
on demand.

**Lengths, formulas and variables.** A `Length` is `Auto`, `Px`, `Percent` of the property's base,
`Var(var, factor)` — a multiple of a variable — or `Calc`, a formula of `+ - * /`, `min`, `max`,
`clamp` built once by `Length::calc` (validated and constant-folded; `calc(10px + 5px)` comes back
as `Px(15.0)`). `resolve(base, value_of)` is a length in px, its variables read from `value_of`
(`Tree::var` where it is read, or the embedder's own numbers). A variable is a number defined on a node for its subtree (`set_var`) or for the
whole tree (`set_global`), read by the nearest definition up the scope chain: the parents, then a
hosted root's host (or the scope host of `set_scope_host`, for a root the embedder lays out on its
own). Ids are the embedder's numbering, `VarId::new(0)` to `VarId::new(63)`: one tree tells
`MAX_VARS` = 64 variables apart, because every node keeps the set it reads as one 64-bit word —
which is what lets a new value re-resolve only the styles that read it.

```rust
use souple::{Length, NoMeasure, Op, Size, Style, Tree, VarId, Viewport};

// The engine knows no units: `em` and `vw` are variables the embedder numbers.
const EM: VarId = VarId::new(0);
const VW: VarId = VarId::new(1);

let mut tree = Tree::new();
let screen = tree.create_node();
let card = tree.create_node_with_style(Style {
    size: Size::new(Length::Var(VW, 50.0), Length::Var(EM, 3.0)), // 50vw × 3em
    ..Style::default()
});
tree.append(screen, card);
tree.set_global(VW, 3.9); // 1vw, wherever no node defines it
tree.set_var(screen, EM, Some(16.0)); // 1em, for the screen and everything under it
tree.set_viewport(screen, Viewport::size(390.0, 844.0));
tree.compute_layout(&mut NoMeasure);
assert_eq!(tree.layout(card).rect.height, 48.0);

// A new value lays out again only what reads it.
tree.set_var(screen, EM, Some(20.0));
tree.compute_layout(&mut NoMeasure);
assert_eq!(tree.changes().len(), 1, "the card");

// calc(100% - 2em), in postfix.
let inner = Length::calc(&[Op::Percent(100.0), Op::Var(EM, 2.0), Op::Sub]).unwrap();
assert!(matches!(inner, Length::Calc(_)));
assert_eq!(inner.resolve(Some(300.0), &|_| 16.0), Some(268.0));
assert_eq!(Length::calc(&[Op::Px(10.0), Op::Px(5.0), Op::Add]), Ok(Length::Px(15.0)));
```

## Hostile input

Any number the engine reads may be anything — in a style, a formula, a variable, a viewport, the
scale, a measurer's answer: NaN, ±∞, ±1e30, −0, a subnormal, a negative size, gap or flex factor.
For every one of them **the engine does not panic, every pass ends, and every published number is
finite.** A negative size, padding, border or gap is 0, a negative flex factor 0, an aspect ratio
that is not a positive finite number none; a viewport axis that is not finite is unbounded; a scale
that is not a positive finite number is ignored; a NaN that reaches a published box is 0 and an
overflow saturates at the largest finite `f32`. Beyond that, what a degenerate tree lays out to is
unspecified: finite, not meaningful. Past 2^24 device px from its root (16,777,216 px at scale 1) an
`f32` no longer holds every whole pixel, and boxes there are only as exact as that.

What panics is a programming error, and each case is documented on its method: a stale `NodeId`
and a `VarId` of 64 or more. The edits the tree refuses — a viewport or a scope host on a node
that has a parent, inserting a root that has a viewport, a node inserted under itself or before a
node that is not a child of the parent, a root hosted inside itself through any chain of parents,
viewport hosts and scope hosts — panic in their plain form and are an `Err(EditError)` in their
`try_` form, which changes nothing. A formula that cannot be built is an `Err`, not a panic.

## Limits

These are the library's design:

- **Every box is positioned.** There is no `position: static`: a relative box's insets shift it, an
  absolute box's place it, and an absolute child's containing block is its parent's padding box.
  This is a decision, not a gap: the 64 conformance tests for `static` are ignored as
  `by design: every box is positioned (no position: static)`.
- **No `display: contents`.** Not planned for now; its 9 conformance tests are ignored as
  `not planned: display: contents`.
- **`set_hug` is for measured leaves.** A container does not shrink to its wrapped content: a
  wrapping row is as wide as the room it was given.
- **Pixel snapping.** Every edge is snapped to the nearest device pixel by its absolute position, so
  neighbours share an edge and a sum of parent offsets never drifts; a published length is the
  difference of two snapped edges. A tie rounds toward +∞ (−0.5 px to 0, 0.5 to 1), so a subtree
  moved by whole pixels keeps its edges, and a pass does not snap it again. An edge within float
  error of a tie (an absolute position is a sum of offsets in `f32`) may still keep the pixel it had
  where a pass from scratch, summing the offsets at the new place, lands on the other side of the
  tie: one device pixel off (a published offset or size, the difference of two such edges, two).
  A measured leaf's far edges go out as far as its measured size
  takes from its snapped near edges: never less than it measured (a text never loses the fraction
  of a pixel its last word needs), and no further. Yoga rounds each box's offset from its parent
  instead; the two part when a parent sits at a fraction of a pixel, in one conformance test,
  `rounding::rounding_fractial_input_3`, ignored as
  `by design: pixel snapping by absolute position; Yoga rounds each offset from its parent (root_child1: top 65, not 64)`.
- **The other ignored conformance tests** are the 17 Yoga skips itself:
  `skipped by Yoga: GTEST_SKIP, Yoga fails it too`.
- **Not here:** right-to-left, grid, `position: fixed | sticky`, the intrinsic size keywords
  (`max-content`, `fit-content`).

## Performance

The repository's `crates/bench` runs the engine on Yoga's captured trees and on synthetic ones. On
its synthetic phone screens (a feed of cards, 37% of the nodes measured text, 390 × 844 at scale 3;
release build with LTO, a measurer that is a lookup, one run on an AMD Ryzen 9 9900X under Windows
11), the medians:

| | 981 nodes | 9 997 nodes |
|---|---:|---:|
| the first layout | 386 µs | 4.05 ms |
| a pass after one leaf's width changed | 5.5 µs | 12.1 µs |
| a pass after one text's content changed (`mark_dirty`) | 5.4 µs | 11.9 µs |
| a pass after the viewport grew 1 px | 172 µs | 1.81 ms |
| a pass with nothing to do | 1 ns | 1 ns |

## License

MIT.
