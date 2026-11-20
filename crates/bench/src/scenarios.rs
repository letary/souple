//! The scenarios, on Souple. `yoga/bench.cpp` runs the same ones on Yoga, step for step; the names
//! are the keys of both result files.
//!
//! | key | what one sample is |
//! |---|---|
//! | `build` | a fresh tree from the plan, top-down in preorder (Yoga's loader order), the builder's way: room for every node reserved (`Tree::with_capacity`), each node created with its style (`create_node_with_style`), set measured, appended |
//! | `build_set_style` | the same with neither: `Tree::new`, `create_node`, set measured, append, then `set_style` (Souple only) |
//! | `build_bottom_up` | the same tree built children first (Souple only — see the README) |
//! | `layout` | `compute_layout` of that fresh tree (build excluded) |
//! | `build_layout` | both, measured in the same iteration |
//! | `layout_min_zero` | `layout` with Yoga's 0 for every unset minimum instead of CSS's `auto` (Souple only) |
//! | `leaf_resize` | the deepest leaf's width 10 ↔ 11 px, then a pass |
//! | `leaf_dirty` | the deepest measured leaf's content changes (its height ±1 px), `mark_dirty`, a pass |
//! | `viewport` | the root's viewport width moves by 1 px (a triangle over ±32 px), a pass; its work counted over one whole period of the triangle |
//! | `var_tree_layout` | the first `compute_layout` of the tree `var` runs on, built top-down (its reference) (Souple only) |
//! | `var_tree_layout_bottom_up` | the same, the tree built bottom-up (Souple only) |
//! | `var` | 10% of the nodes read `VW`; `set_var(root, VW)` toggles by 1%, a pass (Souple only) |
//! | `var_unread` | the same tree; a variable NO node reads is set, a pass (Souple only) |
//! | `tween` | one pass of a 60-pass width tween (1 px per pass) of a container holding ~5% of the tree |
//! | `tween_60` | the 60 passes together |
//! | `noop` | `compute_layout` with nothing changed |

use std::hint::black_box;
use std::path::Path;

use serde_json::{Value, json};
use souple::{Length, NodeId, Style, Tree, VarId, Viewport};

use crate::harness::{Budget, Sampler, Stats, bench, elapsed_us};
use crate::measure::Replay;
use crate::plan::{MinSize, Plan};

/// Passes over which the per-pass counters are averaged (untimed).
const COUNT_PASSES: usize = 20;
/// The viewport's counters are averaged over one whole period of its triangle: what a pass finds
/// in its caches depends on where in the wave it is (near a turn it revisits recent widths).
const VIEWPORT_PASSES: usize = 4 * RESIZE_AMPLITUDE as usize;
/// The tween's width: `TWEEN_FROM`..`TWEEN_FROM + 60`, back and forth.
pub const TWEEN_FROM: f32 = 200.0;
pub const TWEEN_FRAMES: usize = 60;
/// The viewport triangle's amplitude, px.
pub const RESIZE_AMPLITUDE: i64 = 32;

pub const VW: VarId = VarId::new(0);
pub const UNREAD: VarId = VarId::new(1);

pub struct Scenario {
    pub name: &'static str,
    pub stats: Stats,
    pub counters: Vec<(&'static str, f64)>,
}

impl Scenario {
    fn new(name: &'static str, stats: Stats) -> Self {
        Self { name, stats, counters: Vec::new() }
    }

    fn with(mut self, counter: &'static str, value: f64) -> Self {
        self.counters.push((counter, value));
        self
    }
}

pub struct Built {
    pub tree: Tree,
    pub ids: Vec<NodeId>,
}

/// Yoga's loader order (`buildTreeFromJson`): each node created, given its measure function,
/// inserted under its parent; then its children. A loader knows how many nodes it reads, so it
/// reserves them; and it has each node's style at hand when it creates the node.
pub fn build_top_down(plan: &Plan, styles: &[Style]) -> Built {
    let mut tree = Tree::with_capacity(plan.nodes.len());
    tree.set_scale(plan.scale);
    let mut ids: Vec<NodeId> = Vec::with_capacity(plan.nodes.len());
    for (i, node) in plan.nodes.iter().enumerate() {
        let id = tree.create_node_with_style(styles[i].clone());
        if node.spec.is_some() {
            tree.set_measured(id, true);
        }
        if let Some(parent) = node.parent {
            tree.append(ids[parent as usize], id);
        }
        ids.push(id);
    }
    tree.set_viewport(ids[0], Viewport::size(plan.width, plan.height));
    Built { tree, ids }
}

/// The same order, the general-purpose way (`build_set_style`): no room reserved, each node
/// created with the default style, then styled once inserted — what an editor that grows a tree
/// node by node does.
pub fn build_set_style(plan: &Plan, styles: &[Style]) -> Built {
    let mut tree = Tree::new();
    tree.set_scale(plan.scale);
    let mut ids: Vec<NodeId> = Vec::with_capacity(plan.nodes.len());
    for (i, node) in plan.nodes.iter().enumerate() {
        let id = tree.create_node();
        if node.spec.is_some() {
            tree.set_measured(id, true);
        }
        if let Some(parent) = node.parent {
            tree.append(ids[parent as usize], id);
        }
        tree.set_style(id, styles[i].clone());
        ids.push(id);
    }
    tree.set_viewport(ids[0], Viewport::size(plan.width, plan.height));
    Built { tree, ids }
}

/// Children before parents: every node styled while detached, then linked from the last node in
/// preorder to the first, each at the front of its parent — whose own parent is not linked yet, so
/// no edit walks further than one node.
pub fn build_bottom_up(plan: &Plan, styles: &[Style]) -> Built {
    let mut tree = Tree::with_capacity(plan.nodes.len());
    tree.set_scale(plan.scale);
    let ids: Vec<NodeId> = styles.iter().map(|style| tree.create_node_with_style(style.clone())).collect();
    for (i, node) in plan.nodes.iter().enumerate() {
        if node.spec.is_some() {
            tree.set_measured(ids[i], true);
        }
    }
    for i in (1..plan.nodes.len()).rev() {
        let parent = ids[plan.nodes[i].parent.unwrap() as usize];
        let first = tree.children(parent).next();
        tree.insert(parent, ids[i], first);
    }
    tree.set_viewport(ids[0], Viewport::size(plan.width, plan.height));
    Built { tree, ids }
}

/// The triangle wave of the viewport scenario: every step exactly 1 px, never the same width twice
/// in a row.
pub fn resize_offset(i: u64) -> f32 {
    let a = RESIZE_AMPLITUDE;
    let p = (i % (4 * a as u64)) as i64;
    (if p < 2 * a { p - a } else { 3 * a - p }) as f32
}

/// The variable tree: every node at a preorder index ≡ 5 (mod 10) reads `VW` (1% of the viewport
/// width) — its px width as the same number of `vw`, else a `1vw` left padding. Returns the count.
pub fn var_styles(plan: &Plan, styles: &[Style]) -> (Vec<Style>, usize) {
    let vw = plan.width / 100.0;
    let mut out = styles.to_vec();
    let mut users = 0;
    for (i, style) in out.iter_mut().enumerate().skip(1) {
        if i % 10 != 5 {
            continue;
        }
        users += 1;
        match style.size.width {
            Length::Px(px) => style.size.width = Length::Var(VW, px / vw),
            _ => style.padding.left = Length::Var(VW, 1.0),
        }
    }
    (out, users)
}

fn write_rects(path: &Path, built: &Built) {
    let mut text = String::new();
    for id in &built.ids {
        let r = built.tree.layout(*id).rect;
        text.push_str(&format!("{} {} {} {}\n", r.x, r.y, r.width, r.height));
    }
    std::fs::write(path, text).expect("write the rects");
}

/// A scenario on a laid-out tree: timed, then `COUNT_PASSES` more passes counted.
fn steady<S>(
    name: &'static str,
    budget: &Budget,
    built: &mut Built,
    replay: &mut Replay,
    state: S,
    pass: impl FnMut(&mut Built, &mut Replay, &mut S) -> usize,
) -> Scenario {
    steady_over(name, COUNT_PASSES, budget, built, replay, state, pass)
}

/// `steady`, its counters averaged over `passes` passes.
fn steady_over<S>(
    name: &'static str,
    passes: usize,
    budget: &Budget,
    built: &mut Built,
    replay: &mut Replay,
    mut state: S,
    mut pass: impl FnMut(&mut Built, &mut Replay, &mut S) -> usize,
) -> Scenario {
    let stats = bench(budget, || {
        black_box(pass(built, replay, &mut state));
    });
    replay.reset();
    let mut changes = 0;
    for _ in 0..passes {
        changes += pass(built, replay, &mut state);
    }
    Scenario::new(name, stats)
        .with("measure_calls", replay.calls as f64 / passes as f64)
        .with("measure_us_captured", replay.cost_ns / 1e3 / passes as f64)
        .with("changed_nodes", changes as f64 / passes as f64)
}

/// `compute_layout` of fresh trees (built bottom-up, untimed).
fn fresh_layout(name: &'static str, plan: &Plan, styles: &[Style], budget: &Budget, replay: &mut Replay) -> Scenario {
    let mut times = Vec::new();
    let mut sampler = Sampler::new(*budget);
    let (mut calls, mut cost) = (0, 0.0);
    while sampler.more() {
        replay.reset();
        let mut b = build_bottom_up(plan, styles);
        let t0 = std::time::Instant::now();
        b.tree.compute_layout(replay);
        black_box(b.tree.changes().len());
        times.push(elapsed_us(t0));
        (calls, cost) = (replay.calls, replay.cost_ns);
        drop(black_box(b));
    }
    Scenario::new(name, Stats::from(&mut times, 1))
        .with("measure_calls", calls as f64)
        .with("measure_us_captured", cost / 1e3)
}

pub struct TreeRun {
    pub scenarios: Vec<Scenario>,
    pub var_users: usize,
}

pub fn run(plan: &Plan, budget: &Budget, rects: Option<&Path>) -> TreeRun {
    // The primary mapping keeps CSS's automatic minimum (crates/yoga-tests' `yoga_default`); the
    // `_min_zero` variant writes Yoga's "an unset minimum is 0" instead (see the README).
    let styles = plan.styles(MinSize::Auto);
    let mut scenarios = Vec::new();
    let mut replay = Replay::new(&plan.specs, &plan.spec_of);

    // ── full layout of a fresh tree ────────────────────────────────────────────────────────────
    {
        let (mut build, mut layout, mut total) = (Vec::new(), Vec::new(), Vec::new());
        let (mut calls, mut cost) = (0, 0.0);
        // Two untimed warm-up rounds, then the budget.
        for _ in 0..2 {
            let mut b = build_top_down(plan, &styles);
            b.tree.compute_layout(&mut replay);
        }
        let mut sampler = Sampler::new(*budget);
        while sampler.more() {
            replay.reset();
            let t0 = std::time::Instant::now();
            let mut b = build_top_down(plan, &styles);
            let t1 = std::time::Instant::now();
            b.tree.compute_layout(&mut replay);
            black_box(b.tree.changes().len());
            let t2 = std::time::Instant::now();
            build.push((t1 - t0).as_secs_f64() * 1e6);
            layout.push((t2 - t1).as_secs_f64() * 1e6);
            total.push((t2 - t0).as_secs_f64() * 1e6);
            (calls, cost) = (replay.calls, replay.cost_ns);
            drop(black_box(b));
        }
        scenarios.push(Scenario::new("build", Stats::from(&mut build, 1)));
        scenarios.push(
            Scenario::new("layout", Stats::from(&mut layout, 1))
                .with("measure_calls", calls as f64)
                .with("measure_us_captured", cost / 1e3),
        );
        scenarios.push(Scenario::new("build_layout", Stats::from(&mut total, 1)));

        let mut set_style = Vec::new();
        let mut sampler = Sampler::new(*budget);
        while sampler.more() {
            let t0 = std::time::Instant::now();
            let b = build_set_style(plan, &styles);
            set_style.push(elapsed_us(t0));
            drop(black_box(b));
        }
        scenarios.push(Scenario::new("build_set_style", Stats::from(&mut set_style, 1)));

        let mut bottom_up = Vec::new();
        let mut sampler = Sampler::new(*budget);
        while sampler.more() {
            let t0 = std::time::Instant::now();
            let b = build_bottom_up(plan, &styles);
            bottom_up.push(elapsed_us(t0));
            drop(black_box(b));
        }
        scenarios.push(Scenario::new("build_bottom_up", Stats::from(&mut bottom_up, 1)));

        let zero_styles = plan.styles(MinSize::Zero);
        scenarios.push(fresh_layout("layout_min_zero", plan, &zero_styles, budget, &mut replay));
        if let Some(dir) = rects {
            let mut b = build_bottom_up(plan, &zero_styles);
            b.tree.compute_layout(&mut replay);
            write_rects(&dir.join(format!("{}.souple-min-zero.txt", plan.name)), &b);
        }
    }

    // ── incremental scenarios on one laid-out tree ─────────────────────────────────────────────
    let mut b = build_bottom_up(plan, &styles);
    b.tree.compute_layout(&mut replay);
    if let Some(dir) = rects {
        write_rects(&dir.join(format!("{}.souple.txt", plan.name)), &b);
    }
    let root = b.ids[0];

    scenarios.push(steady("noop", budget, &mut b, &mut replay, (), |b, r, _| {
        b.tree.compute_layout(r);
        b.tree.changes().len()
    }));

    // 2a. The deepest leaf's width, 10 ↔ 11 px.
    let leaf_index = plan.deep_leaf();
    let leaf = b.ids[leaf_index];
    b.tree.update_style(leaf, |s| s.size.width = Length::Px(10.0));
    b.tree.compute_layout(&mut replay);
    scenarios.push(steady("leaf_resize", budget, &mut b, &mut replay, false, move |b, r, flip| {
        *flip = !*flip;
        let width = if *flip { 11.0 } else { 10.0 };
        b.tree.update_style(leaf, |s| s.size.width = Length::Px(width));
        b.tree.compute_layout(r);
        b.tree.changes().len()
    }));
    b.tree.set_style(leaf, styles[leaf_index].clone());
    b.tree.compute_layout(&mut replay);

    // 2b. The deepest measured leaf's content.
    if let Some(index) = plan.deep_measured() {
        let id = b.ids[index];
        let spec = plan.spec_of[index];
        scenarios.push(steady("leaf_dirty", budget, &mut b, &mut replay, false, move |b, r, flip| {
            *flip = !*flip;
            r.bump = Some((spec, if *flip { 1.0 } else { 0.0 }));
            b.tree.mark_dirty(id);
            b.tree.compute_layout(r);
            b.tree.changes().len()
        }));
        replay.bump = None;
        b.tree.mark_dirty(id);
        b.tree.compute_layout(&mut replay);
    }

    // 3. The viewport.
    let (width, height) = (plan.width, plan.height);
    scenarios.push(steady_over("viewport", VIEWPORT_PASSES, budget, &mut b, &mut replay, 0u64, move |b, r, i| {
        *i += 1;
        b.tree.set_viewport(root, Viewport::size(width + resize_offset(*i), height));
        b.tree.compute_layout(r);
        b.tree.changes().len()
    }));
    b.tree.set_viewport(root, Viewport::size(width, height));
    b.tree.compute_layout(&mut replay);

    // 5. The tween: each pass timed alone, and the 60 as one.
    {
        let index = plan.tween_node();
        let node = b.ids[index];
        b.tree.update_style(node, |s| s.size.width = Length::Px(TWEEN_FROM));
        b.tree.compute_layout(&mut replay);
        let (mut passes, mut runs) = (Vec::new(), Vec::new());
        let (mut calls, mut cost, mut changes, mut counted) = (0u64, 0.0, 0usize, 0usize);
        let mut sampler = Sampler::new(Budget { min_samples: 10, max_samples: budget.max_samples / TWEEN_FRAMES, ..*budget });
        let mut run = 0usize;
        // One untimed run as the warm-up.
        let mut warm = true;
        while warm || sampler.more() {
            replay.reset();
            let mut run_changes = 0;
            let run_start = std::time::Instant::now();
            for frame in 1..=TWEEN_FRAMES {
                let width = if run.is_multiple_of(2) { TWEEN_FROM + frame as f32 } else { TWEEN_FROM + (TWEEN_FRAMES - frame) as f32 };
                let t = std::time::Instant::now();
                b.tree.update_style(node, |s| s.size.width = Length::Px(width));
                b.tree.compute_layout(&mut replay);
                run_changes += b.tree.changes().len();
                if !warm {
                    passes.push(elapsed_us(t));
                }
            }
            if !warm {
                runs.push(elapsed_us(run_start));
                calls += replay.calls;
                cost += replay.cost_ns;
                changes += run_changes;
                counted += TWEEN_FRAMES;
            }
            warm = false;
            run += 1;
        }
        scenarios.push(
            Scenario::new("tween", Stats::from(&mut passes, 1))
                .with("measure_calls", calls as f64 / counted as f64)
                .with("measure_us_captured", cost / 1e3 / counted as f64)
                .with("changed_nodes", changes as f64 / counted as f64),
        );
        scenarios.push(Scenario::new("tween_60", Stats::from(&mut runs, 1)));
        b.tree.set_style(node, styles[index].clone());
        b.tree.compute_layout(&mut replay);
    }

    // 4. Variables, on a tree whose every tenth node reads VW.
    let (styles_var, var_users) = var_styles(plan, &styles);
    // The reference for `var`: the first layout of that tree, built top-down (as `layout`) and
    // bottom-up (each insert of a subtree that reads a variable queues a resolve of that subtree).
    for (name, top_down) in [("var_tree_layout", true), ("var_tree_layout_bottom_up", false)] {
        let mut times = Vec::new();
        let mut sampler = Sampler::new(*budget);
        let (mut calls, mut cost) = (0, 0.0);
        while sampler.more() {
            let mut v = if top_down { build_top_down(plan, &styles_var) } else { build_bottom_up(plan, &styles_var) };
            v.tree.set_var(v.ids[0], VW, Some(plan.width / 100.0));
            v.tree.set_var(v.ids[0], UNREAD, Some(1.0));
            replay.reset();
            let t0 = std::time::Instant::now();
            v.tree.compute_layout(&mut replay);
            black_box(v.tree.changes().len());
            times.push(elapsed_us(t0));
            (calls, cost) = (replay.calls, replay.cost_ns);
            drop(black_box(v));
        }
        scenarios.push(
            Scenario::new(name, Stats::from(&mut times, 1))
                .with("measure_calls", calls as f64)
                .with("measure_us_captured", cost / 1e3),
        );
    }
    let mut v = build_top_down(plan, &styles_var);
    let vw = plan.width / 100.0;
    let var_root = v.ids[0];
    v.tree.set_var(var_root, VW, Some(vw));
    v.tree.set_var(var_root, UNREAD, Some(1.0));
    v.tree.compute_layout(&mut replay);
    scenarios.push(steady("var", budget, &mut v, &mut replay, false, move |v, r, flip| {
        *flip = !*flip;
        v.tree.set_var(var_root, VW, Some(if *flip { vw * 1.01 } else { vw }));
        v.tree.compute_layout(r);
        v.tree.changes().len()
    }));
    scenarios.push(steady("var_unread", budget, &mut v, &mut replay, false, move |v, r, flip| {
        *flip = !*flip;
        v.tree.set_var(var_root, UNREAD, Some(if *flip { 2.0 } else { 1.0 }));
        v.tree.compute_layout(r);
        v.tree.changes().len()
    }));

    TreeRun { scenarios, var_users }
}

pub fn tree_json(plan: &Plan, run: &TreeRun) -> Value {
    let mut scenarios = serde_json::Map::new();
    for s in &run.scenarios {
        let mut entry = s.stats.to_json();
        let counters: serde_json::Map<String, Value> = s.counters.iter().map(|(k, v)| (k.to_string(), json!(v))).collect();
        entry["counters"] = Value::Object(counters);
        scenarios.insert(s.name.to_string(), entry);
    }
    let n = &plan.notes;
    json!({
        "name": plan.name,
        "nodes": plan.nodes.len(),
        "measured": plan.measured(),
        "depth": plan.max_depth(),
        "picks": { "leaf": plan.deep_leaf(), "measured": plan.deep_measured(), "tween": plan.tween_node() },
        "var_users": run.var_users,
        "notes": {
            "static_position": n.static_position, "rtl": n.rtl, "display_contents": n.display_contents,
            "approximated": n.approximated, "ignored_keys": n.ignored_keys, "errata": n.errata,
            "scale": plan.scale, "other_scale": n.other_scale, "viewport_pin": n.viewport_pin,
        },
        "scenarios": scenarios,
    })
}
