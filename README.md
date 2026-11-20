# souple

A flexbox layout engine in Rust for UI trees that change: a tree of boxes with generational ids,
CSS flexbox, lengths that stay live over scoped variables, measured leaves, pixel snapping, and the
boxes that changed as the output of every pass. The library and what it promises:
[`crates/souple/README.md`](crates/souple/README.md).

## Crates

| crate | what | published |
|---|---|---|
| `crates/souple` | the engine; no dependencies | yes |
| `crates/souple-syntax` | CSS text → the engine's typed lengths and declarations | yes, after `souple` |
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
