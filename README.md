# souple

[![souple on crates.io](https://img.shields.io/crates/v/souple.svg?label=souple)](https://crates.io/crates/souple)
[![souple-syntax on crates.io](https://img.shields.io/crates/v/souple-syntax.svg?label=souple-syntax)](https://crates.io/crates/souple-syntax)
[![docs.rs](https://img.shields.io/docsrs/souple)](https://docs.rs/souple)
[![license: MIT](https://img.shields.io/crates/l/souple.svg)](LICENSE)

A flexbox layout engine in Rust for UI trees that change: a tree of boxes with generational ids,
CSS flexbox, lengths that stay live over scoped variables, measured leaves, pixel snapping, and the
boxes that changed as the output of every pass. The library and what it promises:
[`crates/souple/README.md`](crates/souple/README.md).

## Example

```text
cargo add souple
```

A screen with a header, a body that takes the rest of the height, and a footer:

```rust
use souple::{FlexDirection, Length, NoMeasure, Rect, Size, Style, Tree, Viewport};

let mut tree = Tree::new();

// A column with a 56 px header, a body that grows to fill the rest, and a 48 px footer.
let screen = tree.create_node_with_style(Style {
    flex_direction: FlexDirection::Column,
    ..Style::default()
});
let header = tree.create_node_with_style(Style {
    size: Size::new(Length::Auto, Length::Px(56.0)),
    ..Style::default()
});
let body = tree.create_node_with_style(Style { flex_grow: 1.0, ..Style::default() });
let footer = tree.create_node_with_style(Style {
    size: Size::new(Length::Auto, Length::Px(48.0)),
    ..Style::default()
});
tree.append(screen, header);
tree.append(screen, body);
tree.append(screen, footer);

// Give the root a viewport (a phone screen, in logical px) and lay it out.
tree.set_viewport(screen, Viewport::size(390.0, 844.0));
tree.compute_layout(&mut NoMeasure);

assert_eq!(tree.layout(body).rect, Rect { x: 0.0, y: 56.0, width: 390.0, height: 740.0 });
assert_eq!(tree.layout(footer).rect.y, 796.0);

// Make the header taller and lay out again.
tree.update_style(header, |style| style.size.height = Length::Px(64.0));
tree.compute_layout(&mut NoMeasure);

// Only the boxes that moved are reported: the header and the body. The footer stayed put.
assert_eq!(tree.changes().len(), 2);
assert_eq!(tree.layout(body).rect.height, 732.0);
```

`NoMeasure` is for a tree without content-sized leaves. Text and images are measured by a
`Measurer` of yours (a closure will do), shown in the engine's
[quick start](crates/souple/README.md#quick-start).

Styles can also come from CSS text, with `souple-syntax`:

```rust
use souple::Style;
use souple_syntax::{apply_declaration, Units};

let units = Units::new();
let mut style = Style::default();
apply_declaration(&mut style, "flex-direction", "column", &units)?;
apply_declaration(&mut style, "padding", "16px", &units)?;
apply_declaration(&mut style, "width", "calc(100% - 32px)", &units)?;
```

## Crates

| crate | what | published |
|---|---|---|
| `crates/souple` | the engine; no dependencies | [crates.io](https://crates.io/crates/souple) |
| `crates/souple-syntax` | CSS text → the engine's typed lengths and declarations | [crates.io](https://crates.io/crates/souple-syntax) |
| `crates/yoga-tests` | the conformance suite, generated from Yoga's Chrome-derived fixtures | no |
| `crates/bench` | the engine on Yoga's captured trees and on synthetic ones (against Yoga) | no |

## Build and test

Stable Rust, 1.85 or later (edition 2024).

```text
cargo test --workspace                       # everything: the engine, the syntax, the conformance suite
cargo test -p souple                         # the engine alone
cargo clippy --workspace --all-targets       # warning-free
SOUPLE_FUZZ_CASES=20000 cargo test -p souple --release --test fuzz   # a long run of the randomized test
```

`crates/souple/tests/fuzz.rs` is the randomized test of the engine's promises: random forests,
styles over every field with hostile numbers, random edits between passes, and after every pass a
tree built from scratch to the same state must publish the same boxes, the changed-set must be
exact, and a second pass must do nothing. A failure prints its seed and the shortest sequence of
edits that still fails. `crates/souple/tests/regressions.rs` keeps one test per bug it found.

## The engine's files

| file | owns |
|---|---|
| `tree.rs` | the storage (one array per role, indexed by a node's slot), the node lifecycle, the pass, the pixel snapping and the diff |
| `edits.rs` | the links and the edits |
| `style.rs` | `Style` and its enums |
| `value.rs` | `Length`, `VarId`, `Formula` (build, fold, evaluate) |
| `vars.rs` | the scope chain, authored → computed styles |
| `measure.rs` | the `Measurer` contract |
| `algo/mod.rs` | the recursive question `compute(node, Input) -> Output`, its cache, the root |
| `algo/flex.rs` | the flex container: one function per step of css-flexbox-1 §9, in the spec's order |
| `algo/leaf.rs` · `absolute.rs` · `model.rs` · `axis.rs` | measured and childless boxes · out-of-flow children · the resolved box model · main / cross |

The pipeline, one module per stage:

```text
Style (authored: px, %, variables, formulas)      style.rs, value.rs
  │  vars.rs   substitute the variables in scope
  ▼
Style (computed: px and % only)
  │  algo/     CSS flexbox, in logical px, unrounded
  ▼
NodeLayout (raw)
  │  tree.rs   snap to the device grid, diff against the last pass
  ▼
NodeLayout (published) + [Change]
```

## License

MIT ([`LICENSE`](LICENSE)). The conformance tests are translated from Yoga's (MIT, Meta Platforms):
see `crates/yoga-tests/README.md`.
