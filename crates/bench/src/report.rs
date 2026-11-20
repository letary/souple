//! The tables: Souple's result file merged with Yoga's (`yoga/bench.cpp` writes the same shape),
//! plus how far the two engines' boxes agree on each tree.

use std::fmt::Write as _;
use std::path::Path;

use serde_json::Value;

/// The scenarios, in the order the tables show them.
pub const ORDER: &[(&str, &str)] = &[
    ("build", "build (top-down)"),
    ("build_set_style", "build (Tree::new, set_style)"),
    ("build_bottom_up", "build (bottom-up)"),
    ("layout", "full layout"),
    ("build_layout", "build + layout"),
    ("layout_min_zero", "full layout, unset min = 0"),
    ("leaf_resize", "one leaf: width"),
    ("leaf_dirty", "one leaf: measured, dirty"),
    ("viewport", "viewport resize"),
    ("var_tree_layout", "first layout of the variable tree"),
    ("var_tree_layout_bottom_up", "the same, built bottom-up"),
    ("var", "variable change (10% read it)"),
    ("var_unread", "variable change (no reader)"),
    ("tween", "tween, per pass"),
    ("tween_60", "tween, 60 passes"),
    ("noop", "no-op pass"),
];

fn trees(file: &Value) -> Vec<&Value> {
    file["trees"].as_array().map(|a| a.iter().collect()).unwrap_or_default()
}

fn find<'a>(file: Option<&'a Value>, name: &str) -> Option<&'a Value> {
    file?["trees"].as_array()?.iter().find(|t| t["name"] == name)
}

fn us(value: &Value) -> String {
    match value.as_f64() {
        Some(v) if v >= 1000.0 => format!("{v:.0}"),
        Some(v) if v >= 10.0 => format!("{v:.1}"),
        Some(v) => format!("{v:.2}"),
        None => "–".into(),
    }
}

/// Nodes whose box (x, y, w, h relative to the parent) differs by more than 1 px, of those compared.
fn agreement(rects: &Path, name: &str, variant: &str) -> Option<(usize, usize)> {
    let read = |engine: &str| -> Option<Vec<[f32; 4]>> {
        let text = std::fs::read_to_string(rects.join(format!("{name}.{engine}.txt"))).ok()?;
        Some(
            text.lines()
                .map(|line| {
                    let v: Vec<f32> = line.split_whitespace().map(|x| x.parse().unwrap_or(f32::NAN)).collect();
                    [v[0], v[1], v[2], v[3]]
                })
                .collect(),
        )
    };
    let (a, b) = (read(variant)?, read("yoga")?);
    if a.len() != b.len() {
        return None;
    }
    let differ = a.iter().zip(&b).filter(|(a, b)| a.iter().zip(b.iter()).any(|(x, y)| (x - y).abs() > 1.0 || (x - y).is_nan())).count();
    Some((differ, a.len()))
}

pub fn markdown(souple: &Value, yoga: Option<&Value>, rects: Option<&Path>) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "## Machine and build\n");
    for (label, file) in [("Souple", Some(souple)), ("Yoga", yoga)] {
        if let Some(file) = file {
            let _ = writeln!(out, "- {label}: {} — {}", file["machine"].as_str().unwrap_or("?"), file["build"].as_str().unwrap_or("?"));
        }
    }

    let _ = writeln!(out, "\n## Trees\n");
    let _ = writeln!(out, "| tree | nodes | measured | depth | errata | static / RTL / contents | other scale | boxes off Yoga's by >1px | same, unset min = 0 | var readers |");
    let _ = writeln!(out, "|---|---:|---:|---:|---|---|---:|---:|---:|---:|");
    for tree in trees(souple) {
        let name = tree["name"].as_str().unwrap_or("?");
        let notes = &tree["notes"];
        let agree = |variant: &str| {
            rects
                .and_then(|r| agreement(r, name, variant))
                .map_or("–".to_string(), |(d, n)| format!("{d} ({:.1}%)", 100.0 * d as f64 / n as f64))
        };
        let (agree, agree_zero) = (agree("souple"), agree("souple-min-zero"));
        let _ = writeln!(
            out,
            "| {name} | {} | {} | {} | {} | {} / {} / {} | {} | {agree} | {agree_zero} | {} |",
            tree["nodes"], tree["measured"], tree["depth"], notes["errata"].as_str().unwrap_or(""),
            notes["static_position"], notes["rtl"], notes["display_contents"], notes["other_scale"], tree["var_users"],
        );
        if let Some(y) = find(yoga, name)
            && y["picks"] != tree["picks"]
        {
            let _ = writeln!(out, "\n**{name}: the harnesses picked different nodes: souple {} yoga {}**\n", tree["picks"], y["picks"]);
        }
    }

    let _ = writeln!(out, "\n## Medians, µs (Souple / Yoga)\n");
    let summary = [
        "layout", "build_layout", "leaf_resize", "leaf_dirty", "viewport", "var", "var_unread", "tween", "tween_60", "noop",
    ];
    let _ = write!(out, "| tree |");
    for key in summary {
        let label = ORDER.iter().find(|(k, _)| *k == key).unwrap().1;
        let _ = write!(out, " {label} |");
    }
    let _ = writeln!(out);
    let _ = writeln!(out, "|---|{}", "---:|".repeat(summary.len()));
    for tree in trees(souple) {
        let name = tree["name"].as_str().unwrap_or("?");
        let y = find(yoga, name);
        let _ = write!(out, "| {name} |");
        for key in summary {
            let s = us(&tree["scenarios"][key]["median"]);
            let yv = y.map(|y| &y["scenarios"][key]["median"]).filter(|v| !v.is_null());
            match yv {
                Some(v) => {
                    let _ = write!(out, " {s} / {} |", us(v));
                }
                None => {
                    let _ = write!(out, " {s} |");
                }
            }
        }
        let _ = writeln!(out);
    }

    let _ = writeln!(out, "\n## Every scenario: median, p95 (µs), samples × batch\n");
    let _ = writeln!(out, "| tree | scenario | Souple median | Souple p95 | n×b | Yoga median | Yoga p95 | n×b | Souple ÷ Yoga |");
    let _ = writeln!(out, "|---|---|---:|---:|---:|---:|---:|---:|---:|");
    for tree in trees(souple) {
        let name = tree["name"].as_str().unwrap_or("?");
        let y = find(yoga, name);
        for (key, label) in ORDER {
            let s = &tree["scenarios"][*key];
            if s.is_null() {
                continue;
            }
            let ys = y.map(|y| &y["scenarios"][*key]).filter(|v| !v.is_null());
            let ratio = match (s["median"].as_f64(), ys.and_then(|v| v["median"].as_f64())) {
                (Some(a), Some(b)) if b > 0.0 => format!("{:.2}", a / b),
                _ => "–".into(),
            };
            let nb = |v: &Value| format!("{}×{}", v["samples"], v["batch"]);
            let _ = writeln!(
                out,
                "| {name} | {label} | {} | {} | {} | {} | {} | {} | {ratio} |",
                us(&s["median"]), us(&s["p95"]), nb(s),
                ys.map_or("–".into(), |v| us(&v["median"])), ys.map_or("–".into(), |v| us(&v["p95"])),
                ys.map_or("–".into(), nb),
            );
        }
    }

    let _ = writeln!(out, "\n## Work per pass (averaged over 20 passes; full layout: one pass)\n");
    let _ = writeln!(out, "Souple: Measurer calls, published boxes that changed. Yoga: measure calls, and its own LayoutData counters for the pass — layouts / measures computed, layouts / measures served from cache.\n");
    let _ = writeln!(out, "| tree | scenario | Souple calls | Souple calls at captured cost, µs | Souple changed | Yoga calls | Yoga calls at captured cost, µs | Yoga layouts | Yoga measures | Yoga cached layouts | Yoga cached measures |");
    let _ = writeln!(out, "|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|");
    for tree in trees(souple) {
        let name = tree["name"].as_str().unwrap_or("?");
        let y = find(yoga, name);
        for (key, label) in ORDER {
            let c = &tree["scenarios"][*key]["counters"];
            let yc = y.map(|y| &y["scenarios"][*key]["counters"]);
            if c.as_object().is_none_or(|m| m.is_empty()) && yc.is_none_or(|v| v.is_null()) {
                continue;
            }
            let num = |v: &Value| {
                v.as_f64().map_or("–".to_string(), |x| if x.fract() == 0.0 || x >= 100.0 { format!("{x:.0}") } else { format!("{x:.1}") })
            };
            let yv = |k: &str| yc.map_or("–".to_string(), |c| num(&c[k]));
            let _ = writeln!(
                out,
                "| {name} | {label} | {} | {} | {} | {} | {} | {} | {} | {} | {} |",
                num(&c["measure_calls"]), num(&c["measure_us_captured"]), num(&c["changed_nodes"]),
                yv("measure_calls"), yv("measure_us_captured"), yv("layouts"), yv("measures"), yv("cached_layouts"), yv("cached_measures"),
            );
        }
    }
    out
}
