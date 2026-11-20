# souple-bench

Souple against Yoga, on the same machine, the same trees and the same measured leaves, in the
scenarios Souple was designed to make cheap: a full layout, a one-leaf change, a viewport resize, a
variable change, a per-frame tween, a pass with nothing to do.

```
bash crates/bench/run-all.sh          # everything (~20 min): both harnesses, both runs, the tables
```

`run-all.sh` builds this crate (`cargo build -p souple-bench --release`) and the Yoga harness
(`yoga/build.bat`: MSVC and clang-cl, into `yoga-build/`, git-ignored), writes the synthetic trees
(`out/trees/`), runs Yoga (clang-cl, then MSVC), then Souple, one at a time, and writes
`out/results.md` (Souple vs clang-cl Yoga) and `out/results-msvc.md`. Everything under `out/` is
git-ignored. The steps by hand:

```
cargo build -p souple-bench --release
target/release/souple-bench gen [DIR]                         # the synthetic trees (default out/trees)
target/release/souple-bench gen DIR KIND N                    # one: typical | deep | deep-shrink | wide
crates/bench/yoga/build.bat [YOGA_ROOT]                       # → yoga-build/{msvc/Release,clang}/yoga-bench.exe
yoga-bench --seconds 0.5 --out out/yoga.json --rects out/rects CAPTURES_DIR out/trees   [--errata none]
target/release/souple-bench run --seconds 0.5 --out out/souple.json --rects out/rects CAPTURES_DIR out/trees
target/release/souple-bench report out/souple.json out/yoga.json --rects out/rects [--md FILE]
```

`CAPTURES_DIR` defaults to `../lecodes-next/engines/ui/libs/yoga/benchmark/captures` (`$YOGA_CAPTURES`);
the Yoga sources are read from that checkout (`-DYOGA_ROOT=` / the bat's argument), never written.
Windows, Git Bash; the Yoga harness needs VS 2019 Build Tools (MSVC 19.29 and its clang-cl 12).

## The scenarios

The same keys in both result files (`src/scenarios.rs`, `yoga/bench.cpp`). Every time is one pass
unless it says otherwise; an edit, where there is one, is inside the timed pass.

| key | one sample | engines |
|---|---|---|
| `build` | a fresh tree from the plan, top-down in preorder, Yoga's loader order: create, measure function, insert, style. Souple builds the way a loader that knows its node count does: `Tree::with_capacity`, each node created with its style (`create_node_with_style`) | both |
| `build_set_style` | the same with the general-purpose calls: `Tree::new` (the arrays grow), `create_node`, then `set_style` once inserted | Souple |
| `build_bottom_up` | the same tree, children styled and linked before their parents | Souple |
| `layout` | the first `compute_layout` / `YGNodeCalculateLayout` of that fresh tree | both |
| `build_layout` | both, from the same iteration | both |
| `layout_min_zero` | `layout` with every unset minimum 0 (Yoga's rule) instead of CSS's `auto` | Souple |
| `leaf_resize` | the deepest leaf's width 10 ↔ 11 px (`update_style` / `YGNodeStyleSetWidth`), a pass | both |
| `leaf_dirty` | the deepest measured leaf's content changes (its measured height ±1 px), `mark_dirty` / `YGNodeMarkDirty`, a pass | both |
| `viewport` | the viewport width moves 1 px (a triangle over ±32 px, never the same twice in a row), a pass | both |
| `var_tree_layout` | the first layout of the variable tree (below), built top-down — `var`'s reference | Souple |
| `var_tree_layout_bottom_up` | the same, built bottom-up | Souple |
| `var` | `set_var(root, VW)` toggling by 1%, a pass; 10% of the nodes read `VW` | Souple |
| `var_unread` | the same tree; a variable that no node reads is set, a pass | Souple |
| `tween` | one pass of a 60-pass tween: a container's width 200 → 260 px (and back), 1 px per pass | both |
| `tween_60` | those 60 passes, together | both |
| `noop` | a pass with nothing changed | both |

The nodes acted on are picked by the same rules on both sides (`Plan::deep_leaf` …, `Plan::*` in
bench.cpp; the report flags a mismatch): the deepest visible leaf, the deepest visible measured
leaf, and for the tween the visible in-flow container (not the root) whose subtree is closest to 5%
of the tree. The variable tree: every node at a preorder index ≡ 5 (mod 10) reads `VW` (1% of the
viewport width) — its px width written as the same number of `vw`, else a left padding of `1vw`.

Per-pass work, averaged over 20 more passes (untimed) — the viewport's over one whole period of its
triangle (128 passes), since what a pass finds in the caches depends on where in the wave it is
(near a turn it revisits recent widths): Measurer calls and published boxes that
changed (`changes()` after the pass) for Souple; measure calls and Yoga's own `LayoutData` (layouts
and measures computed, layouts and measures served from cache) for Yoga, read through its event
subscriber in separate untimed passes. Souple has no counter of nodes recomputed and this crate does
not edit the engine; the counters quoted under "Why the extra questions" come from an instrumented
copy of the engine outside this repo.

## The trees

**Yoga's captures** (`benchmark/captures/*.json`): real trees with every measure-function call the
app made. `src/plan.rs` maps each style key by key exactly as Yoga's loader does
(`benchmark/Benchmark.cpp`, `setStylesFromJson`: the same keys, the same accepted units) onto Yoga's
defaults — column, `flex-shrink: 0`, `align-content: flex-start`, `box-sizing: border-box`,
`position: relative` — and resolves what Souple does not have the way Yoga resolves it for an LTR
tree: `start`/`end`/`horizontal`/`vertical`/`all` edges by Yoga's precedence, the `flex` shorthand by
`resolveFlexGrow`/`resolveFlexShrink`/`processFlexBasis` (`flex: 1` = grow 1, shrink 0, basis 0).

- **Unset minimums** stay CSS's `auto`, as in `crates/yoga-tests`' `yoga_default`. Yoga has no
  automatic minimum, but it ignores a `flex-basis` along an indefinite main size and sizes by content;
  with a minimum of 0 Souple collapses those `flex: 1` items to 0 instead. Measured (the table
  below): `auto` leaves fewer boxes off Yoga's on two captures of four and in total (291 vs 346);
  `layout_min_zero` times the other.
- **The root's viewport pin**: every capture sizes its root as `min == max == the available size`
  (React Native's way). That is the viewport; both harnesses drop the pin and let the root fill the
  viewport — the same box, and a viewport resize now reaches the tree (pinned, it reaches nothing).
- **Not in Souple**: `position: static` (approximated as relative without insets), RTL,
  `display: contents` — counted per tree; the four captures use none. Yoga's `errata` (all four say
  `all`) cannot be mapped; `yoga-bench --errata none` moved Yoga's full layouts by −6% to +12%.
- **Point scale**: Souple has one per tree (the majority of the nodes' configs); Yoga keeps each
  node's (chat-mac and rendering-sample give their root 2× and every other node 1×).
- **How comparable the layouts are**: after the first layout both harnesses dump every box (x, y, w,
  h relative to the parent, snapped) and the report counts the nodes off by more than 1 px.

**Synthetic trees** (`src/synth.rs`, seeded, written in the capture format so both harnesses load the
same files; 390 × 844 @3x, `errata: none`):

- `typical-1k` / `-10k`: a phone screen — a header row, a scrolling feed of sections of cards (an
  avatar row with an absolute badge, wrapping body text, an optional 100%-wide image, a wrapped row of
  chips, a row of buttons), a tab bar;
- `deep-1k` / `-10k`: deep and narrow — 5 (10) spines of nested stretching columns under a root, one
  leaf beside the next level at each, depth 100 (500);
- `wide-1k` / `-10k`: wide and flat — one scrolling column of 999 (9 999) leaves, wrapping texts and
  boxes.

## The measured leaves

`src/measure.rs` and `Spec` in `yoga/bench.cpp`, the same rules: an EXACT replay first (a record with
the same modes and the same numbers, as Yoga's harness matches), else a text model fitted to the
node's records — a block of its natural size that wraps into `ceil(natural width / width)` lines when
offered less, unless the records show a single truncating line; min-content (a Souple-only question)
is `min(natural width, 4 × line height)`. The cost is a lookup on both sides: the node's spec by slot
(Souple) or context pointer (Yoga), a scan of 0–2 records, a few flops. Nothing sleeps — Yoga's own
harness sleeps for each call's recorded `duration-ns` (`std::this_thread::sleep_for`, whose granularity
on Windows is far coarser than those few µs — not measured here) and prints on every replay miss. Yoga asks exactly the captured questions (its call count equals the
records'), so its replay is exact; Souple's questions differ and mostly take the model.

The recorded durations are kept and REPORTED: `measure_us_captured` is Σ over the calls made of the
node's mean captured duration — what those calls cost on the device that made the capture. It is not
spent, and a measurer that caches its text shaping would pay less.

## Timing

`std::time::Instant` / `std::chrono::steady_clock` (both QueryPerformanceCounter, 100 ns ticks). A
warm-up (≥ 3 calls and 20 ms; full layouts: 2 fresh trees), then samples for `--seconds` (0.5) per
scenario: at least 30, at most 20 000 — and once 10× the budget is spent, no more than 3 (an
operation of seconds). An operation under 5 µs is timed in batches (the sample is the batch's mean;
the tables give samples × batch), so its p95 is of batch means. The tween is timed per pass (and per
60), at least 10 runs (3 when a run takes seconds: Yoga on deep-10k). Full layouts build a fresh tree per sample; teardown is untimed.

Builds: Rust release profile of the workspace (opt-level 3, `lto = true`, `codegen-units = 1`,
`panic = abort`); Yoga 3.2.1 with its own release optimisation plus whole-program optimisation
(clang-cl 12: `/O2 -flto=thin`, lld-link; MSVC 19.29: `/O2 /Ob2 /GL /LTCG`), `/GR-`, C++20. No
`target-cpu` on either side. The deep trees nest 500 levels: both run on a 1 GB stack.

## Results

AMD Ryzen 9 9900X (12 cores, nothing else running), Windows 11 Pro 26300, 2026-11-19. Souple at
commit `5a45f99` plus the working-tree edits of that moment (two `max(0.0)` clamps in `algo/flex.rs`
and `algo/model.rs`, nothing on a hot path). Yoga 3.2.1 from lecodes-next, the clang-cl build (the
MSVC build's full layouts are within 1–15% of it, table further down). 0.5 s per scenario. All times in µs.
`out/results.md` is the generated version of these tables.

### The trees

| tree | nodes | measured | depth | errata | static / RTL / contents | other scale | boxes off Yoga's by >1px | same, unset min = 0 | var readers |
|---|---:|---:|---:|---|---|---:|---:|---:|---:|
| chat-mac | 1516 | 139 | 37 | all | 0 / 0 / 0 | 1 | 273 (18.0%) | 257 (17.0%) | 152 |
| feed-android | 559 | 58 | 31 | all | 0 / 0 / 0 | 0 | 0 (0.0%) | 59 (10.6%) | 56 |
| profile-ios | 101 | 13 | 16 | all | 0 / 0 / 0 | 0 | 15 (14.9%) | 29 (28.7%) | 10 |
| rendering-sample-mac | 120 | 24 | 10 | all | 0 / 0 / 0 | 1 | 3 (2.5%) | 1 (0.8%) | 12 |
| deep-10k | 9981 | 2500 | 500 | none | 0 / 0 / 0 | 0 | 0 (0.0%) | 0 (0.0%) | 998 |
| deep-1k | 991 | 250 | 100 | none | 0 / 0 / 0 | 0 | 0 (0.0%) | 0 (0.0%) | 99 |
| typical-10k | 9997 | 3755 | 6 | none | 0 / 0 / 0 | 0 | 25 (0.3%) | 25 (0.3%) | 1000 |
| typical-1k | 981 | 368 | 6 | none | 0 / 0 / 0 | 0 | 0 (0.0%) | 0 (0.0%) | 98 |
| wide-10k | 10000 | 5000 | 1 | none | 0 / 0 / 0 | 0 | 0 (0.0%) | 0 (0.0%) | 1000 |
| wide-1k | 1000 | 500 | 1 | none | 0 / 0 / 0 | 0 | 0 (0.0%) | 0 (0.0%) | 100 |

"boxes off": nodes whose box (relative to the parent) differs from Yoga's by more than 1 px after the
first layout — how far the two engines compute the same layout. The synthetic trees agree almost
exactly; chat-mac's 18% are CSS-vs-Yoga differences (`flex: 1` items, Yoga's errata), so there the
two compare similar, not identical, work.

### Medians, Souple / Yoga

| tree | full layout | build + layout | one leaf: width | one leaf: measured, dirty | viewport resize | variable change (10% read it) | variable change (no reader) | tween, per pass | tween, 60 passes | no-op pass |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| chat-mac | 1805 / 5988 | 2785 / 6263 | 102.9 / 469.8 | 91.0 / 575.3 | 430.2 / 4431 | 3016 | 0.02 | 24.2 / 438.0 | 1475 / 27990 | 0.00 / 0.02 |
| feed-android | 259.5 / 1142 | 402.4 / 1230 | 14.2 / 109.2 | 14.1 / 109.2 | 2.33 / 33.5 | 156.0 | 0.02 | 13.2 / 116.7 | 796.1 / 7072 | 0.00 / 0.02 |
| profile-ios | 42.3 / 211.9 | 62.5 / 217.6 | 7.50 / 23.1 | 10.8 / 113.3 | 10.1 / 45.8 | 25.5 | 0.02 | 10.4 / 103.3 | 627.6 / 6231 | 0.00 / 0.02 |
| rendering-sample-mac | 70.9 / 95.8 | 90.5 / 101.7 | 15.2 / 18.1 | 15.1 / 18.1 | 18.2 / 97.1 | 35.5 | 0.02 | 2.80 / 11.0 | 168.8 / 667.5 | 0.00 / 0.02 |
| deep-10k | 3539 / 511800 | 27287 / 512706 | 244.3 / 51575 | 361.4 / 51683 | 2854 / 593441 | 4028 | 0.02 | 263.1 / 56787 | 17836 / 3429602 | 0.00 / 0.02 |
| deep-1k | 274.8 / 10146 | 702.8 / 10196 | 45.2 / 2052 | 54.9 / 2050 | 250.4 / 11732 | 262.4 | 0.02 | 47.5 / 2337 | 2876 / 140606 | 0.00 / 0.02 |
| typical-10k | 6246 / 23252 | 11333 / 23962 | 109.6 / 593.9 | 11.6 / 594.1 | 1835 / 22612 | 2400 | 0.02 | 58.1 / 1221 | 3562 / 73535 | 0.00 / 0.02 |
| typical-1k | 518.1 / 2260 | 684.8 / 2321 | 15.1 / 65.0 | 5.60 / 65.1 | 167.3 / 2204 | 226.2 | 0.02 | 9.10 / 142.3 | 578.1 / 8640 | 0.00 / 0.02 |
| wide-10k | 2939 / 3784 | 7707 / 4161 | 1082 / 3270 | 1066 / 3299 | 1790 / 5681 | 1344 | 0.02 | 1788 / 5428 | 107936 / 333450 | 0.00 / 0.02 |
| wide-1k | 186.4 / 367.9 | 335.4 / 403.9 | 88.9 / 307.5 | 89.0 / 311.6 | 153.5 / 554.5 | 112.9 | 0.02 | 152.6 / 526.9 | 9138 / 32624 | 0.00 / 0.02 |

### Every scenario: median, p95, samples × batch

| tree | scenario | Souple median | Souple p95 | n×b | Yoga median | Yoga p95 | n×b | Souple ÷ Yoga |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| chat-mac | build (top-down) | 976.6 | 1079 | 170×1 | 265.5 | 363.7 | 79×1 | 3.68 |
| chat-mac | build (bottom-up) | 734.3 | 1050 | 546×1 | – | – | – | – |
| chat-mac | full layout | 1805 | 1851 | 170×1 | 5988 | 6054 | 79×1 | 0.30 |
| chat-mac | build + layout | 2785 | 2887 | 170×1 | 6263 | 6368 | 79×1 | 0.44 |
| chat-mac | full layout, unset min = 0 | 1481 | 1522 | 236×1 | – | – | – | – |
| chat-mac | one leaf: width | 102.9 | 105.9 | 5176×1 | 469.8 | 496.7 | 1056×1 | 0.22 |
| chat-mac | one leaf: measured, dirty | 91.0 | 93.3 | 5471×1 | 575.3 | 585.6 | 868×1 | 0.16 |
| chat-mac | viewport resize | 430.2 | 1923 | 734×1 | 4431 | 4810 | 113×1 | 0.10 |
| chat-mac | first layout of the variable tree | 3352 | 3527 | 122×1 | – | – | – | – |
| chat-mac | the same, built bottom-up | 3953 | 4049 | 112×1 | – | – | – | – |
| chat-mac | variable change (10% read it) | 3016 | 3410 | 164×1 | – | – | – | – |
| chat-mac | variable change (no reader) | 0.02 | 0.02 | 20000×101 | – | – | – | – |
| chat-mac | tween, per pass | 24.2 | 25.7 | 19980×1 | 438.0 | 528.9 | 1080×1 | 0.06 |
| chat-mac | tween, 60 passes | 1475 | 1548 | 333×1 | 27990 | 28296 | 18×1 | 0.05 |
| chat-mac | no-op pass | 0.00 | 0.00 | 20000×179 | 0.02 | 0.02 | 20000×133 | 0.03 |
| feed-android | build (top-down) | 140.2 | 370.7 | 1123×1 | 86.8 | 117.1 | 395×1 | 1.62 |
| feed-android | build (bottom-up) | 104.5 | 122.0 | 4145×1 | – | – | – | – |
| feed-android | full layout | 259.5 | 293.8 | 1123×1 | 1142 | 1198 | 395×1 | 0.23 |
| feed-android | build + layout | 402.4 | 637.9 | 1123×1 | 1230 | 1302 | 395×1 | 0.33 |
| feed-android | full layout, unset min = 0 | 194.5 | 212.3 | 1503×1 | – | – | – | – |
| feed-android | one leaf: width | 14.2 | 14.3 | 20000×1 | 109.2 | 115.8 | 4526×1 | 0.13 |
| feed-android | one leaf: measured, dirty | 14.1 | 14.2 | 20000×1 | 109.2 | 122.6 | 4486×1 | 0.13 |
| feed-android | viewport resize | 2.33 | 2.37 | 20000×3 | 33.5 | 35.6 | 14627×1 | 0.07 |
| feed-android | first layout of the variable tree | 296.4 | 320.9 | 1115×1 | – | – | – | – |
| feed-android | the same, built bottom-up | 419.3 | 447.6 | 913×1 | – | – | – | – |
| feed-android | variable change (10% read it) | 156.0 | 171.6 | 3161×1 | – | – | – | – |
| feed-android | variable change (no reader) | 0.02 | 0.02 | 20000×103 | – | – | – | – |
| feed-android | tween, per pass | 13.2 | 17.3 | 19980×1 | 116.7 | 127.9 | 4200×1 | 0.11 |
| feed-android | tween, 60 passes | 796.1 | 893.6 | 333×1 | 7072 | 7240 | 70×1 | 0.11 |
| feed-android | no-op pass | 0.00 | 0.00 | 20000×180 | 0.02 | 0.02 | 20000×133 | 0.03 |
| profile-ios | build (top-down) | 20.1 | 21.8 | 7624×1 | 5.70 | 6.20 | 2233×1 | 3.53 |
| profile-ios | build (bottom-up) | 15.5 | 21.9 | 20000×1 | – | – | – | – |
| profile-ios | full layout | 42.3 | 46.9 | 7624×1 | 211.9 | 230.0 | 2233×1 | 0.20 |
| profile-ios | build + layout | 62.5 | 68.6 | 7624×1 | 217.6 | 236.7 | 2233×1 | 0.29 |
| profile-ios | full layout, unset min = 0 | 40.1 | 43.0 | 8370×1 | – | – | – | – |
| profile-ios | one leaf: width | 7.50 | 7.60 | 20000×1 | 23.1 | 23.4 | 20000×1 | 0.32 |
| profile-ios | one leaf: measured, dirty | 10.8 | 10.9 | 20000×1 | 113.3 | 128.3 | 4335×1 | 0.10 |
| profile-ios | viewport resize | 10.1 | 10.3 | 20000×1 | 45.8 | 52.7 | 10706×1 | 0.22 |
| profile-ios | first layout of the variable tree | 46.2 | 48.8 | 7292×1 | – | – | – | – |
| profile-ios | the same, built bottom-up | 61.1 | 65.4 | 6250×1 | – | – | – | – |
| profile-ios | variable change (10% read it) | 25.5 | 26.1 | 19277×1 | – | – | – | – |
| profile-ios | variable change (no reader) | 0.02 | 0.02 | 20000×104 | – | – | – | – |
| profile-ios | tween, per pass | 10.4 | 10.6 | 19980×1 | 103.3 | 108.6 | 4800×1 | 0.10 |
| profile-ios | tween, 60 passes | 627.6 | 655.2 | 333×1 | 6231 | 6357 | 80×1 | 0.10 |
| profile-ios | no-op pass | 0.00 | 0.00 | 20000×181 | 0.02 | 0.02 | 20000×128 | 0.03 |
| rendering-sample-mac | build (top-down) | 19.5 | 21.5 | 5318×1 | 5.90 | 6.30 | 4720×1 | 3.31 |
| rendering-sample-mac | build (bottom-up) | 17.5 | 20.6 | 20000×1 | – | – | – | – |
| rendering-sample-mac | full layout | 70.9 | 77.2 | 5318×1 | 95.8 | 100.1 | 4720×1 | 0.74 |
| rendering-sample-mac | build + layout | 90.5 | 99.6 | 5318×1 | 101.7 | 113.7 | 4720×1 | 0.89 |
| rendering-sample-mac | full layout, unset min = 0 | 29.3 | 32.2 | 9997×1 | – | – | – | – |
| rendering-sample-mac | one leaf: width | 15.2 | 16.0 | 20000×1 | 18.1 | 18.3 | 20000×1 | 0.84 |
| rendering-sample-mac | one leaf: measured, dirty | 15.1 | 15.4 | 20000×1 | 18.1 | 19.4 | 20000×1 | 0.83 |
| rendering-sample-mac | viewport resize | 18.2 | 66.7 | 20000×1 | 97.1 | 112.0 | 5012×1 | 0.19 |
| rendering-sample-mac | first layout of the variable tree | 75.1 | 87.9 | 5060×1 | – | – | – | – |
| rendering-sample-mac | the same, built bottom-up | 85.4 | 96.3 | 4693×1 | – | – | – | – |
| rendering-sample-mac | variable change (10% read it) | 35.5 | 37.5 | 14025×1 | – | – | – | – |
| rendering-sample-mac | variable change (no reader) | 0.02 | 0.02 | 20000×103 | – | – | – | – |
| rendering-sample-mac | tween, per pass | 2.80 | 2.90 | 19980×1 | 11.0 | 11.2 | 19980×1 | 0.25 |
| rendering-sample-mac | tween, 60 passes | 168.8 | 195.0 | 333×1 | 667.5 | 693.4 | 333×1 | 0.25 |
| rendering-sample-mac | no-op pass | 0.00 | 0.00 | 20000×181 | 0.02 | 0.02 | 20000×133 | 0.03 |
| deep-10k | build (top-down) | 23733 | 24540 | 30×1 | 926.9 | 1586 | 10×1 | 25.60 |
| deep-10k | build (bottom-up) | 4760 | 5315 | 93×1 | – | – | – | – |
| deep-10k | full layout | 3539 | 3915 | 30×1 | 511800 | 519390 | 10×1 | 0.01 |
| deep-10k | build + layout | 27287 | 28718 | 30×1 | 512706 | 520376 | 10×1 | 0.05 |
| deep-10k | full layout, unset min = 0 | 3224 | 3383 | 56×1 | – | – | – | – |
| deep-10k | one leaf: width | 244.3 | 264.8 | 2013×1 | 51575 | 53618 | 30×1 | 0.00 |
| deep-10k | one leaf: measured, dirty | 361.4 | 423.0 | 1341×1 | 51683 | 52278 | 30×1 | 0.01 |
| deep-10k | viewport resize | 2854 | 2975 | 179×1 | 593441 | 656168 | 9×1 | 0.00 |
| deep-10k | first layout of the variable tree | 6468 | 6869 | 30×1 | – | – | – | – |
| deep-10k | the same, built bottom-up | 184901 | 186243 | 27×1 | – | – | – | – |
| deep-10k | variable change (10% read it) | 4028 | 4184 | 124×1 | – | – | – | – |
| deep-10k | variable change (no reader) | 0.02 | 0.02 | 20000×103 | – | – | – | – |
| deep-10k | tween, per pass | 263.1 | 387.1 | 1680×1 | 56787 | 62114 | 180×1 | 0.00 |
| deep-10k | tween, 60 passes | 17836 | 18085 | 28×1 | 3429602 | 3451140 | 3×1 | 0.01 |
| deep-10k | no-op pass | 0.00 | 0.00 | 20000×181 | 0.02 | 0.02 | 20000×133 | 0.03 |
| deep-1k | build (top-down) | 426.8 | 470.3 | 679×1 | 48.4 | 128.4 | 49×1 | 8.82 |
| deep-1k | build (bottom-up) | 156.5 | 170.5 | 2862×1 | – | – | – | – |
| deep-1k | full layout | 274.8 | 303.2 | 679×1 | 10146 | 10380 | 49×1 | 0.03 |
| deep-1k | build + layout | 702.8 | 795.6 | 679×1 | 10196 | 10516 | 49×1 | 0.07 |
| deep-1k | full layout, unset min = 0 | 256.7 | 276.0 | 1114×1 | – | – | – | – |
| deep-1k | one leaf: width | 45.2 | 49.3 | 10824×1 | 2052 | 2144 | 243×1 | 0.02 |
| deep-1k | one leaf: measured, dirty | 54.9 | 59.7 | 8942×1 | 2050 | 2145 | 243×1 | 0.03 |
| deep-1k | viewport resize | 250.4 | 269.3 | 2041×1 | 11732 | 12954 | 43×1 | 0.02 |
| deep-1k | first layout of the variable tree | 353.9 | 371.0 | 634×1 | – | – | – | – |
| deep-1k | the same, built bottom-up | 1580 | 1622 | 284×1 | – | – | – | – |
| deep-1k | variable change (10% read it) | 262.4 | 279.9 | 1888×1 | – | – | – | – |
| deep-1k | variable change (no reader) | 0.02 | 0.02 | 20000×103 | – | – | – | – |
| deep-1k | tween, per pass | 47.5 | 57.9 | 10260×1 | 2337 | 2477 | 600×1 | 0.02 |
| deep-1k | tween, 60 passes | 2876 | 2965 | 171×1 | 140606 | 140958 | 10×1 | 0.02 |
| deep-1k | no-op pass | 0.00 | 0.00 | 20000×178 | 0.02 | 0.02 | 20000×125 | 0.03 |
| typical-10k | build (top-down) | 5114 | 5262 | 42×1 | 631.2 | 1592 | 30×1 | 8.10 |
| typical-10k | build (bottom-up) | 4865 | 5258 | 92×1 | – | – | – | – |
| typical-10k | full layout | 6246 | 6473 | 42×1 | 23252 | 23648 | 30×1 | 0.27 |
| typical-10k | build + layout | 11333 | 11588 | 42×1 | 23962 | 24969 | 30×1 | 0.47 |
| typical-10k | full layout, unset min = 0 | 5720 | 6563 | 44×1 | – | – | – | – |
| typical-10k | one leaf: width | 109.6 | 132.2 | 4383×1 | 593.9 | 627.1 | 830×1 | 0.18 |
| typical-10k | one leaf: measured, dirty | 11.6 | 11.7 | 20000×1 | 594.1 | 624.3 | 830×1 | 0.02 |
| typical-10k | viewport resize | 1835 | 2054 | 279×1 | 22612 | 23150 | 30×1 | 0.08 |
| typical-10k | first layout of the variable tree | 6378 | 6763 | 41×1 | – | – | – | – |
| typical-10k | the same, built bottom-up | 6129 | 6285 | 43×1 | – | – | – | – |
| typical-10k | variable change (10% read it) | 2400 | 2494 | 208×1 | – | – | – | – |
| typical-10k | variable change (no reader) | 0.02 | 0.02 | 20000×103 | – | – | – | – |
| typical-10k | tween, per pass | 58.1 | 75.8 | 8400×1 | 1221 | 1281 | 600×1 | 0.05 |
| typical-10k | tween, 60 passes | 3562 | 3652 | 140×1 | 73535 | 73978 | 10×1 | 0.05 |
| typical-10k | no-op pass | 0.00 | 0.00 | 20000×181 | 0.02 | 0.02 | 20000×133 | 0.03 |
| typical-1k | build (top-down) | 165.6 | 179.1 | 704×1 | 56.0 | 65.3 | 213×1 | 2.96 |
| typical-1k | build (bottom-up) | 157.9 | 182.1 | 2807×1 | – | – | – | – |
| typical-1k | full layout | 518.1 | 549.5 | 704×1 | 2260 | 2394 | 213×1 | 0.23 |
| typical-1k | build + layout | 684.8 | 721.2 | 704×1 | 2321 | 2448 | 213×1 | 0.30 |
| typical-1k | full layout, unset min = 0 | 474.2 | 508.5 | 747×1 | – | – | – | – |
| typical-1k | one leaf: width | 15.1 | 25.4 | 20000×1 | 65.0 | 70.9 | 7561×1 | 0.23 |
| typical-1k | one leaf: measured, dirty | 5.60 | 5.70 | 20000×1 | 65.1 | 75.5 | 7492×1 | 0.09 |
| typical-1k | viewport resize | 167.3 | 189.9 | 3051×1 | 2204 | 2473 | 225×1 | 0.08 |
| typical-1k | first layout of the variable tree | 537.8 | 563.2 | 691×1 | – | – | – | – |
| typical-1k | the same, built bottom-up | 560.5 | 586.8 | 670×1 | – | – | – | – |
| typical-1k | variable change (10% read it) | 226.2 | 246.0 | 2184×1 | – | – | – | – |
| typical-1k | variable change (no reader) | 0.02 | 0.02 | 20000×103 | – | – | – | – |
| typical-1k | tween, per pass | 9.10 | 17.9 | 19980×1 | 142.3 | 159.9 | 3420×1 | 0.06 |
| typical-1k | tween, 60 passes | 578.1 | 601.6 | 333×1 | 8640 | 8874 | 57×1 | 0.07 |
| typical-1k | no-op pass | 0.00 | 0.00 | 20000×179 | 0.02 | 0.02 | 20000×134 | 0.03 |
| wide-10k | build (top-down) | 4777 | 5475 | 60×1 | 357.9 | 1309 | 72×1 | 13.35 |
| wide-10k | build (bottom-up) | 4740 | 4939 | 95×1 | – | – | – | – |
| wide-10k | full layout | 2939 | 3083 | 60×1 | 3784 | 4148 | 72×1 | 0.78 |
| wide-10k | build + layout | 7707 | 8532 | 60×1 | 4161 | 5278 | 72×1 | 1.85 |
| wide-10k | full layout, unset min = 0 | 2647 | 2796 | 62×1 | – | – | – | – |
| wide-10k | one leaf: width | 1082 | 1147 | 457×1 | 3270 | 3450 | 152×1 | 0.33 |
| wide-10k | one leaf: measured, dirty | 1066 | 1193 | 465×1 | 3299 | 3513 | 151×1 | 0.32 |
| wide-10k | viewport resize | 1790 | 1886 | 281×1 | 5681 | 6986 | 89×1 | 0.32 |
| wide-10k | first layout of the variable tree | 3317 | 3426 | 58×1 | – | – | – | – |
| wide-10k | the same, built bottom-up | 3001 | 3185 | 60×1 | – | – | – | – |
| wide-10k | variable change (10% read it) | 1344 | 1481 | 367×1 | – | – | – | – |
| wide-10k | variable change (no reader) | 0.02 | 0.02 | 20000×104 | – | – | – | – |
| wide-10k | tween, per pass | 1788 | 1930 | 600×1 | 5428 | 7009 | 600×1 | 0.33 |
| wide-10k | tween, 60 passes | 107936 | 109492 | 10×1 | 333450 | 339904 | 10×1 | 0.32 |
| wide-10k | no-op pass | 0.00 | 0.00 | 20000×179 | 0.02 | 0.02 | 20000×133 | 0.03 |
| wide-1k | build (top-down) | 148.7 | 161.9 | 1397×1 | 35.1 | 44.6 | 1114×1 | 4.24 |
| wide-1k | build (bottom-up) | 153.8 | 165.4 | 2886×1 | – | – | – | – |
| wide-1k | full layout | 186.4 | 202.1 | 1397×1 | 367.9 | 393.5 | 1114×1 | 0.51 |
| wide-1k | build + layout | 335.4 | 358.0 | 1397×1 | 403.9 | 434.7 | 1114×1 | 0.83 |
| wide-1k | full layout, unset min = 0 | 154.7 | 172.2 | 1468×1 | – | – | – | – |
| wide-1k | one leaf: width | 88.9 | 101.7 | 5520×1 | 307.5 | 331.4 | 1599×1 | 0.29 |
| wide-1k | one leaf: measured, dirty | 89.0 | 107.2 | 5469×1 | 311.6 | 338.0 | 1571×1 | 0.29 |
| wide-1k | viewport resize | 153.5 | 169.4 | 3258×1 | 554.5 | 752.6 | 874×1 | 0.28 |
| wide-1k | first layout of the variable tree | 207.1 | 228.5 | 1347×1 | – | – | – | – |
| wide-1k | the same, built bottom-up | 192.2 | 211.2 | 1340×1 | – | – | – | – |
| wide-1k | variable change (10% read it) | 112.9 | 122.6 | 4372×1 | – | – | – | – |
| wide-1k | variable change (no reader) | 0.02 | 0.02 | 20000×103 | – | – | – | – |
| wide-1k | tween, per pass | 152.6 | 169.2 | 3240×1 | 526.9 | 686.8 | 900×1 | 0.29 |
| wide-1k | tween, 60 passes | 9138 | 9243 | 54×1 | 32624 | 33003 | 15×1 | 0.28 |
| wide-1k | no-op pass | 0.00 | 0.00 | 20000×181 | 0.02 | 0.02 | 20000×133 | 0.03 |

### Work per pass

Souple: Measurer calls, published boxes that changed. Yoga: measure calls, and its own LayoutData counters for the pass — layouts / measures computed, layouts / measures served from cache.

| tree | scenario | Souple calls | Souple calls at captured cost, µs | Souple changed | Yoga calls | Yoga calls at captured cost, µs | Yoga layouts | Yoga measures | Yoga cached layouts | Yoga cached measures |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| chat-mac | full layout | 736 | 5429 | – | 162 | 1149 | 1373 | 11549 | 139 | 8050 |
| chat-mac | one leaf: width | 0 | 0 | 50 | 0 | 0 | 38 | 854 | 43 | 482 |
| chat-mac | one leaf: measured, dirty | 27.5 | 228 | 2 | 1 | 12.0 | 36 | 1018 | 41 | 813 |
| chat-mac | viewport resize | 372 | 2731 | 227 | 26.3 | 165 | 287 | 8180 | 92.4 | 5811 |
| chat-mac | variable change (10% read it) | 1168 | 8564 | 559 | – | – | – | – | – | – |
| chat-mac | tween, per pass | 0 | 0 | 2 | 2.5 | 16.8 | 36.1 | 774 | 41.6 | 517 |
| feed-android | full layout | 139 | 5069 | – | 57 | 2436 | 503 | 2366 | 56 | 690 |
| feed-android | one leaf: width | 1 | 10.3 | 41 | 1 | 10.3 | 32 | 174 | 22 | 82.8 |
| feed-android | one leaf: measured, dirty | 2 | 20.6 | 26 | 1 | 10.3 | 32 | 174 | 22 | 83.8 |
| feed-android | viewport resize | 0 | 0 | 7 | 0 | 0 | 6 | 11 | 3 | 12 |
| feed-android | variable change (10% read it) | 47.5 | 716 | 138 | – | – | – | – | – | – |
| feed-android | tween, per pass | 0 | 0 | 7 | 0 | 0 | 26 | 174 | 23 | 171 |
| profile-ios | full layout | 19 | 101 | – | 13 | 73.8 | 88 | 391 | 13 | 322 |
| profile-ios | one leaf: width | 0 | 0 | 1 | 0 | 0 | 17 | 23 | 15 | 39 |
| profile-ios | one leaf: measured, dirty | 1 | 4.4 | 2 | 1 | 4.4 | 16 | 210 | 18 | 221 |
| profile-ios | viewport resize | 1.5 | 14.8 | 30 | 2 | 19.8 | 26 | 84.1 | 10 | 40.9 |
| profile-ios | variable change (10% read it) | 2 | 19.2 | 30 | – | – | – | – | – | – |
| profile-ios | tween, per pass | 0 | 0 | 14 | 0 | 0 | 16 | 194 | 17 | 196 |
| rendering-sample-mac | full layout | 79 | 1373 | – | 24 | 725 | 96 | 164 | 24 | 78 |
| rendering-sample-mac | one leaf: width | 1 | 8.2 | 21 | 1 | 8.2 | 10 | 12 | 25 | 24 |
| rendering-sample-mac | one leaf: measured, dirty | 4 | 33 | 21 | 1 | 8.2 | 10 | 12 | 25 | 25 |
| rendering-sample-mac | viewport resize | 30.6 | 237 | 61.3 | 5.6 | 56.1 | 63 | 141 | 52 | 93.5 |
| rendering-sample-mac | variable change (10% read it) | 14 | 98.3 | 21 | – | – | – | – | – | – |
| rendering-sample-mac | tween, per pass | 0 | 0 | 0 | 0 | 0 | 6 | 3 | 11 | 7 |
| deep-10k | full layout | 2500 | 0 | – | 2500 | 0 | 7481 | 1250020 | 2500 | 627510 |
| deep-10k | one leaf: width | 0 | 0 | 1 | 0 | 0 | 501 | 124752 | 507 | 63019 |
| deep-10k | one leaf: measured, dirty | 1 | 0 | 510 | 1 | 0 | 500 | 124753 | 508 | 63018 |
| deep-10k | viewport resize | 1875 | 0 | 7491 | 2500 | 0 | 4991 | 1250020 | 4990 | 627510 |
| deep-10k | variable change (10% read it) | 498 | 0 | 500 | – | – | – | – | – | – |
| deep-10k | tween, per pass | 115 | 0 | 461 | 117 | 0 | 500 | 124868 | 508 | 62902 |
| deep-1k | full layout | 250 | 0 | – | 250 | 0 | 741 | 25010 | 250 | 12755 |
| deep-1k | one leaf: width | 0 | 0 | 1 | 0 | 0 | 101 | 4952 | 102 | 2609 |
| deep-1k | one leaf: measured, dirty | 1 | 0 | 105 | 1 | 0 | 100 | 4953 | 103 | 2608 |
| deep-1k | viewport resize | 250 | 0 | 746 | 250 | 0 | 496 | 25010 | 495 | 12755 |
| deep-1k | variable change (10% read it) | 49 | 0 | 50 | – | – | – | – | – | – |
| deep-1k | tween, per pass | 11.9 | 0 | 41 | 12.1 | 0 | 100 | 4963 | 103 | 2597 |
| typical-10k | full layout | 8790 | 0 | – | 3755 | 0 | 7959 | 32854 | 5470 | 47802 |
| typical-10k | one leaf: width | 1 | 0 | 61 | 1 | 0 | 6 | 15 | 58 | 173 |
| typical-10k | one leaf: measured, dirty | 4 | 0 | 3 | 1 | 0 | 6 | 15 | 58 | 174 |
| typical-10k | viewport resize | 1401 | 0 | 4138 | 3008 | 0 | 5297 | 30775 | 6140 | 48862 |
| typical-10k | variable change (10% read it) | 916 | 0 | 3420 | – | – | – | – | – | – |
| typical-10k | tween, per pass | 55.5 | 0 | 85.5 | 23.3 | 0 | 162 | 801 | 220 | 1582 |
| typical-1k | full layout | 870 | 0 | – | 368 | 0 | 781 | 3199 | 534 | 4639 |
| typical-1k | one leaf: width | 1 | 0 | 23 | 1 | 0 | 6 | 15 | 20 | 59 |
| typical-1k | one leaf: measured, dirty | 4 | 0 | 3 | 1 | 0 | 6 | 15 | 20 | 60 |
| typical-1k | viewport resize | 164 | 0 | 412 | 101 | 0 | 519 | 2800 | 604 | 4947 |
| typical-1k | variable change (10% read it) | 98.5 | 0 | 315 | – | – | – | – | – | – |
| typical-1k | tween, per pass | 7.1 | 0 | 10.9 | 2.8 | 0 | 25 | 112 | 37 | 235 |
| wide-10k | full layout | 5000 | 0 | – | 5000 | 0 | 5000 | 8333 | 5000 | 5000 |
| wide-10k | one leaf: width | 1 | 0 | 10000 | 1 | 0 | 1 | 1 | 9999 | 13331 |
| wide-10k | one leaf: measured, dirty | 1 | 0 | 10000 | 1 | 0 | 1 | 1 | 9999 | 13332 |
| wide-10k | viewport resize | 5000 | 0 | 10000 | 5000 | 0 | 5000 | 8333 | 5000 | 5000 |
| wide-10k | variable change (10% read it) | 1000 | 0 | 1000 | – | – | – | – | – | – |
| wide-10k | tween, per pass | 4583 | 0 | 10000 | 4667 | 0 | 5000 | 7777 | 5000 | 5556 |
| wide-1k | full layout | 500 | 0 | – | 500 | 0 | 500 | 833 | 500 | 500 |
| wide-1k | one leaf: width | 1 | 0 | 1000 | 1 | 0 | 1 | 1 | 999 | 1331 |
| wide-1k | one leaf: measured, dirty | 1 | 0 | 1000 | 1 | 0 | 1 | 1 | 999 | 1332 |
| wide-1k | viewport resize | 375 | 0 | 1000 | 500 | 0 | 500 | 833 | 500 | 500 |
| wide-1k | variable change (10% read it) | 100 | 0 | 100 | – | – | – | – | – | – |
| wide-1k | tween, per pass | 458 | 0 | 1000 | 467 | 0 | 500 | 777 | 500 | 556 |

"calls at captured cost": the calls made, each priced at the captured duration of a call to that
leaf (captures only; the synthetic trees have none) — what the Measurer would have cost on the
capturing device with no cache of its own. chat-mac's full layout: Souple 1 805 + 5 429 µs, Yoga
5 988 + 1 149 µs.

### Yoga: clang-cl vs MSVC

| tree | layout, clang-cl | layout, MSVC | one leaf: width, clang-cl | MSVC | build, clang-cl | MSVC |
|---|---:|---:|---:|---:|---:|---:|
| chat-mac | 5988 | 6068 | 469.8 | 462.5 | 265.5 | 278.5 |
| feed-android | 1142 | 1169 | 109.2 | 103.9 | 86.8 | 97.9 |
| profile-ios | 212 | 215 | 23.1 | 22.3 | 5.7 | 7.5 |
| rendering-sample-mac | 96 | 99 | 18.1 | 16.7 | 5.9 | 7.9 |
| deep-10k | 511800 | 542312 | 51575 | 54408 | 926.9 | 1611 |
| deep-1k | 10146 | 10889 | 2052 | 2190 | 48.4 | 64.3 |
| typical-10k | 23252 | 25674 | 593.9 | 437.4 | 631.2 | 1629 |
| typical-1k | 2260 | 2487 | 65.0 | 50.2 | 56.0 | 74.9 |
| wide-10k | 3784 | 4338 | 3270 | 3394 | 357.9 | 507.0 |
| wide-1k | 368 | 415 | 307.5 | 322.5 | 35.1 | 49.2 |

### Growth with depth

`gen out/growth deep N` / `deep-shrink N` (5 chains, depth ≈ N / 10), 0.02 s per scenario:

| tree | depth | layout, Souple | layout, Yoga | one leaf, Souple | one leaf, Yoga |
|---|---:|---:|---:|---:|---:|
| deep (stretch) | 11 | 26.3 | 142.8 | 6.1 | 32.9 |
| | 21 | 52.8 | 470.0 | 11.5 | 102.0 |
| | 31 | 80.1 | 986.5 | 17.2 | 209.0 |
| | 41 | 106.8 | 1724 | 22.9 | 357.9 |
| | 51 | 138.2 | 2619 | 28.3 | 538.0 |
| deep-shrink | 11 | 87.4 | 284.9 | 19.1 | 61.0 |
| | 21 | 513.7 | 1788 | 103.5 | 365.9 |
| | 31 | 2426 | 7818 | 479.9 | 1571 |
| | 41 | 10790 | 31090 | 2214 | 6511 |
| | 51 | 45114 | 121001 | 8772 | 24297 |

Stretching levels: Souple linear, Yoga quadratic (its measure count grows with depth²). Levels sized
by their content: both exponential at about the same rate (× 4–6 per 10 levels), Souple about 3×
lower.

## What the numbers say

- **Where Souple wins.** Every layout scenario on every tree, with a lookup-cost Measurer. A full
  layout is 1.35–5× faster on the captures and 1.3–4.4× on the typical and wide trees (37–145× on
  the deep ones, where Yoga is quadratic); the incremental passes are 1.2–17× faster on the captures
  (rendering-sample's one-leaf passes only 1.2×) and up to ~200× on the deep trees. Yoga's
  incremental passes have an O(n) floor — `roundLayoutResultsToPixelGrid` walks the whole tree after
  any pass that does work (typical-10k: 6 layouts and 15 measures computed, 594 µs) — and re-measure
  more on the way up (chat-mac, one leaf: 854 measures). Souple's publish walk descends only where
  the algorithm laid nodes out or a node moved. A pass with nothing to do is free in both.
- **Where Souple loses.**
  - *Building*: 1.6–26× slower than Yoga's build; on wide-10k build + layout is a net loss (7.7 ms vs
    4.2 ms). A node is ~1.6 KB (`Style` 432 B kept twice — authored and computed —, the cache 488 B,
    meta 96 B, links 40 B, two layouts 112 B) against Yoga's 576 B; `Tree` cannot reserve capacity,
    so its six arrays reallocate as they grow; `set_style` clones and compares the whole style; and
    every edit (`append`, `set_style`, `set_measured`) walks to the root clearing caches, with no
    early exit by design — the top-down build of deep-10k takes 5.0 M walk steps (23.7 ms; 4.8 ms
    built children-first, `build_bottom_up`).
  - *Measurer calls*: 1.5–4.5× more than Yoga on the captures and 2.3–2.4× on the typical screens
    (chat-mac 736 vs 162; one leaf up to 49 in one layout; a one-leaf content change asks that leaf
    26.5 times per pass where Yoga asks once). With a lookup they are free. Priced at the captured
    cost of a call they outweigh the layout itself, and the full-layout comparison turns: layout +
    measuring, Souple vs Yoga, is 7.2 vs 7.1 ms on chat-mac, 5.3 vs 3.6 ms on feed-android, 1.44 vs
    0.82 ms on rendering-sample — Souple behind on two of four captures — and 0.14 vs 0.29 ms on
    profile-ios. (Assumes a Measurer with no cache of its own.) Part is CSS (min-content questions
    Yoga never asks); most is the cache key — below.
- **Variables.** A change no node reads costs ~20 ns. A change 10% of the nodes read costs 38–90% of
  a full layout of that tree: the readers are spread out, so most of the tree is dirty. The win is
  not the layout — a Yoga host re-setting those styles would leave the same work — it is skipping the
  re-parse. Built children-first, a tree that reads variables resolves each subtree once per
  ancestor (each `insert` queues a whole-subtree resolve, `rescope_later`, and the queue is not
  merged): deep-10k's first layout takes 185 ms, against 6.5 ms built top-down.

### Why the extra questions (evidence: an instrumented copy of the engine)

Counters added to a COPY of `crates/souple` in a scratch directory (this repo's engine is not
edited): runs of the flex algorithm (cache misses) per node, and the inputs of one node's misses.

- Down a stretching chain every container runs exactly twice at every depth. Down a content-sized
  chain the runs per node grow ~1.2× per level (2 at depth 1, 42 at depth 20).
- One such container (depth 10, 8 runs): 5 of its misses are the SAME max-content width question
  (`known` none, `available.width: MaxContent`) that differ only in `available.height` — 188, 196,
  220, 240 px or unbounded. `contributions()` (algo/flex.rs) passes each ancestor's own height space
  down, and `answers()` (algo/mod.rs) keys the cache on the space offered along every open axis, so
  every content-sized ancestor mints new keys for an answer that does not depend on them. The
  chat-mac leaf asked 26.5 times per pass shows the same: of its 27 questions in one pass, 13 are the
  max-content width under 9 different offered heights, 7 the min-content width under 4, 7 at a known
  width (239 px three times); 10 repeat a question of the same pass that the 6-entry round-robin
  cache (`SIZE_ENTRIES`) had evicted.
- A full chat-mac layout: 7 114 container runs and 1 375 leaf runs (16 605 cache lookups, 8 116
  hits), against Yoga's 1 373 layouts + 11 549 measures — fewer computations than Yoga, and cheaper
  ones (~0.21 µs vs ~0.46 µs each).
- The publish walk re-snaps every descendant of a node that moved (snapping is by absolute
  position): typical-10k's one-leaf width change visits 9 994 nodes to publish 61 changes (110 µs);
  the content change that moves nothing visits 64 (12 µs).

## What made the benchmark awkward

- No per-node user data: the Measurer gets a `NodeId`, and the embedder has to map it back to its
  leaf. This crate relies on the low half of `NodeId::to_bits()` being the slot (undocumented); Yoga
  has `YGNodeGetContext`.
- No counters: nothing tells how many nodes a pass recomputed or how the cache did (Yoga publishes
  `LayoutData` per pass). The instrumented copy was the only way to see it.
- A top-down build — the order a loader or a reconciler produces — is the slow order; nothing says so.
- Setting one property is a whole-`Style` round trip (`update_style` clones 432 B, compares, stores
  it twice).
- No `Tree::with_capacity`.
- The captures pin the root with `min == max`; Souple's `Viewport` replaces that, but a host that
  keeps the pin gets a viewport change that silently does nothing.
