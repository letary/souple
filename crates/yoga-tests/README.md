# souple-yoga-tests

The conformance suite of `souple`: Yoga's generated flexbox tests, translated to the engine's
typed API. Their expected boxes were captured from Chrome, so they are CSS's answers, not Yoga's
opinion. All of them build one test binary, `tests/yoga.rs`.

```
src/bin/import.rs    the importer: Yoga's tests/generated/*.cpp → tests/generated/*.rs
tests/support/       the harness the generated tests use: yoga_default(), Fixture, Fixture::check,
                     and Yoga's fake text measurer
tests/generated/     the importer's output, one module per Yoga file. Never edit it by hand.
```

## Run

```
cargo test -p souple-yoga-tests --test yoga                       # everything
cargo test -p souple-yoga-tests --test yoga margin_start          # every test whose path has it
cargo test -p souple-yoga-tests --test yoga -- --exact generated::margin::margin_start
cargo test -p souple-yoga-tests --test yoga -- --ignored static   # the ignored ones, to see how far off they are
```

A failure lists every box of the test that differs, not only the first one:

```
2 of 2 boxes differ from Yoga's [left, top, width, height]:
  root                     expected [0.0, 0.0, 100.0, 100.0] got [0.0, 0.0, 0.0, 0.0]   (width, height)
  root_child0              expected [0.0, 0.0, 100.0, 100.0] got [0.0, 0.0, 0.0, 0.0]   (width, height)
```

## Regenerate

```
cargo run -p souple-yoga-tests --bin import -- <path to a Yoga checkout>
```

The tests were generated from Yoga 3.2.1 (a checkout that used to sit at
`../lecodes-next/engines/ui/libs/yoga`, since removed there; any 3.2.1 checkout will do). The importer rewrites
`tests/generated/` completely and prints the counts below. It refuses input whose shape differs from
what its header comment describes. A call it does not know marks the test ignored and prints a
warning. `cargo test -p souple-yoga-tests --bin import` tests its precedence rules.

## How a Yoga test becomes a Rust test

- Every node starts from `yoga_default()`, Yoga's defaults: column, `flex-shrink: 0`,
  `align-content: flex-start`, `border-box`. They match the stylesheet Chrome rendered the fixtures
  with. The minimum size stays CSS's `auto`, which Chrome had and Yoga lacks.
- Setters resolve the way Yoga resolves them in an LTR pass. `Start` / `End` become left / right, and
  each physical edge is decided by `Start` over `Left` over `Horizontal` over `All`. Gutters do the
  same (`Row` / `Column` over `All`), and so does `flex` (grow, shrink and basis).
- `YGNodeCalculateLayout(root, w, h)` becomes the root's viewport, with `YGUndefined` as `None`. Every
  Yoga 3.2.1 test passes `YGUndefined` × `YGUndefined`, so every root is content-sized or sized by its
  own style.
- The root's `position: absolute` is dropped. The fixtures use it only so each root sits on its own in
  the page, and Yoga lays a root out the same whatever its position type.
- Only the LTR expectations are asserted. The engine has no RTL.
- A leaf with text (`YGNodeSetContext` + `YGNodeSetMeasureFunc(…, IntrinsicSizeMeasure)`) becomes
  `t.text(node, "<text>")`: a measured node, sized by the harness's port of Yoga's
  `IntrinsicSizeMeasure` (`tests/util/TestUtil.cpp`), a fixed-pitch fake of the fixtures' Ahem font:
  10 px per character, 10 px per line, greedy breaks at spaces. It is ported quirks included, since
  it is the measurer Yoga passes these tests with. The engine's question becomes Yoga's measure modes
  (both in content-box px): a known axis is "exactly", a `Definite` space "at most", `MaxContent`
  "undefined". Yoga has no min-content: a `MinContent` width is the narrowest wrap (the longest
  word), a `MinContent` height the undefined one.

## Ignored tests

A test that uses something the engine does not have is generated anyway, with
`#[ignore = "<reason>"]`. Its untranslated lines are kept as comments. Counts for Yoga 3.2.1 (the
importer keeps the current ones at the top of `tests/generated/mod.rs`):

Yoga 3.2.1: 548 tests in 25 files. 457 run and 91 are ignored.

| Ignored because | Tests | |
|---|---:|---|
| by design: every box is positioned (no position: static) | 64 | The engine has relative and absolute only, by decision. |
| skipped by Yoga: GTEST_SKIP, Yoga fails it too | 17 | They are fully translated, so `--ignored` runs them for real. |
| not planned: display: contents | 9 | |
| by design: pixel snapping by absolute position; Yoga rounds each offset from its parent (…) | 1 | A decision, not a gap: see below. |

The reasons are the categories of the souple README's "Limits", word for word: the importer takes
them from its `LEFT_OUT` table (a feature the engine leaves out), `KNOWN_DIFFERENCES` (below) and
`SKIPPED_BY_YOGA`, so `cargo test` output explains itself. No Yoga 3.2.1 test sets a direction, an
experimental feature, an errata or an intrinsic size keyword (`max-content`, `fit-content`,
`stretch`). The importer would ignore each one under its plain CSS name (`width: max-content`, …),
to be given a category in `LEFT_OUT` when a Yoga version first uses it.

## Known differences

A test the engine answers differently on purpose is listed in `KNOWN_DIFFERENCES` at the top of
`src/bin/import.rs`: the test's name and a one-line why. The importer generates it with
`#[ignore = "by design: <why>"]`, so the suite stays green and the decision stays visible
(`--ignored` still runs it, to see by how much). That table is the only place such a decision is
taken; this README only describes the entries. A new decision is a new entry there, with a comment
on its why. An entry that matches no test makes the importer warn.

| Test | Why |
|---|---|
| `rounding::rounding_fractial_input_3` | **Rounding.** The engine snaps every box edge to the pixel grid by its ABSOLUTE position, so neighbours share an edge and a host that sums parent offsets never drifts. Yoga and Chrome's captured `offsetTop` round each box's offset from its parent. The two part when a parent sits at a fraction of a px: this root is at top 0.3, and `root_child1` is published at top 65 where Yoga expects 64. |

One more convention is Yoga's and not the algorithm's, and the engine follows it, so it fails
nothing and needs no entry:

- **The root's own margin and inset.** Yoga places a root by its margin plus its relative offset.
  14 running tests expect that (for example `justify_content_row_min_width_and_margin`, a root at
  x = 100).

## Attribution

The tests are translated from [Yoga](https://github.com/facebook/yoga)'s `tests/generated`, which
Yoga's `gentest` writes from its `gentest/fixtures`. Yoga is released under the MIT license:

```
MIT License

Copyright (c) Facebook, Inc. and its affiliates.

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```
