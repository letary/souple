//! `souple-bench`: Souple on Yoga's captured trees and on synthetic ones, in the scenarios the
//! engine was designed to make cheap. See README.md.
//!
//! ```text
//! souple-bench gen [DIR]                                   the synthetic trees, as capture JSON (default out/trees)
//! souple-bench run [--seconds S] [--out FILE] [--rects DIR] [FILE|DIR ...]
//! souple-bench report SOUPLE.json [YOGA.json] [--rects DIR] [--md FILE]
//! ```

mod harness;
mod measure;
mod plan;
mod report;
mod scenarios;
mod synth;

use std::path::{Path, PathBuf};

use serde_json::{Value, json};

use harness::Budget;

fn bench_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Yoga's captures: `$YOGA_CAPTURES`, else the lecodes-next checkout beside this repo.
fn default_captures() -> PathBuf {
    std::env::var_os("YOGA_CAPTURES").map(PathBuf::from).unwrap_or_else(|| {
        bench_dir().join("../../../lecodes-next/engines/ui/libs/yoga/benchmark/captures")
    })
}

fn json_files(paths: &[PathBuf]) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for path in paths {
        if path.is_dir() {
            let mut list: Vec<PathBuf> = std::fs::read_dir(path)
                .unwrap_or_else(|e| panic!("{}: {e}", path.display()))
                .map(|e| e.unwrap().path())
                .filter(|p| p.extension().is_some_and(|x| x == "json"))
                .collect();
            list.sort();
            files.extend(list);
        } else {
            files.push(path.clone());
        }
    }
    files
}

fn read_json(path: &Path) -> Value {
    let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let mut de = serde_json::Deserializer::from_str(&text);
    de.disable_recursion_limit();
    de.into_iter::<Value>().next().expect("a JSON value").unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

fn machine() -> String {
    std::env::var("BENCH_MACHINE")
        .or_else(|_| std::env::var("PROCESSOR_IDENTIFIER"))
        .unwrap_or_else(|_| "unknown machine".into())
}

fn build_flags() -> String {
    let profile = if cfg!(debug_assertions) { "DEBUG (numbers meaningless)" } else { "release" };
    // rustup names the toolchain it ran cargo with; a build without rustup says so.
    let toolchain = option_env!("RUSTUP_TOOLCHAIN").unwrap_or("(toolchain unknown)");
    format!(
        "rustc {toolchain}, {profile}: opt-level 3, lto = true, codegen-units = 1, panic = abort, {}-{} (no target-cpu)",
        std::env::consts::ARCH,
        std::env::consts::OS,
    )
}

fn main() {
    // The deep synthetic trees recurse thousands of levels (in the engine, and in serde_json's
    // drop): everything runs on a thread with a large stack. yoga/bench.exe links /STACK the same.
    let child = std::thread::Builder::new().stack_size(1 << 30).spawn(real_main).unwrap();
    child.join().unwrap();
}

fn real_main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("gen") => generate(&args[1..]),
        Some("run") => run(&args[1..]),
        Some("report") => report_cmd(&args[1..]),
        _ => {
            eprintln!("usage: souple-bench gen [DIR] | run [--seconds S] [--out FILE] [--rects DIR] [FILE|DIR ...] | report SOUPLE.json [YOGA.json] [--rects DIR] [--md FILE]");
            std::process::exit(2);
        }
    }
}

/// `gen [DIR]`: every synthetic tree; `gen DIR KIND N`: one (`typical`, `deep`, `deep-shrink`,
/// `wide`) of about N nodes, as DIR/KIND-N.json — for growth studies.
fn generate(args: &[String]) {
    let dir = args.first().map(PathBuf::from).unwrap_or_else(|| bench_dir().join("out/trees"));
    std::fs::create_dir_all(&dir).unwrap();
    let trees = match (args.get(1), args.get(2)) {
        (Some(kind), Some(n)) => {
            let n: usize = n.parse().expect("N");
            vec![(format!("{kind}-{n}"), synth::one(kind, n))]
        }
        _ => synth::all(),
    };
    for (name, tree) in trees {
        let path = dir.join(format!("{name}.json"));
        let file = std::io::BufWriter::new(std::fs::File::create(&path).unwrap());
        serde_json::to_writer(file, &tree).unwrap();
        println!("{}", path.display());
    }
}

fn run(args: &[String]) {
    let mut budget = Budget { seconds: 0.5, min_samples: 30, max_samples: 20_000 };
    let mut out = bench_dir().join("out/souple.json");
    let mut rects = Some(bench_dir().join("out/rects"));
    let mut inputs = Vec::new();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--seconds" => {
                i += 1;
                budget.seconds = args[i].parse().expect("--seconds S");
            }
            "--min-samples" => {
                i += 1;
                budget.min_samples = args[i].parse().expect("--min-samples N");
            }
            "--out" => {
                i += 1;
                out = PathBuf::from(&args[i]);
            }
            "--rects" => {
                i += 1;
                rects = Some(PathBuf::from(&args[i]));
            }
            "--no-rects" => rects = None,
            path => inputs.push(PathBuf::from(path)),
        }
        i += 1;
    }
    if inputs.is_empty() {
        inputs.push(default_captures());
        inputs.push(bench_dir().join("out/trees"));
    }
    if let Some(dir) = &rects {
        std::fs::create_dir_all(dir).unwrap();
    }

    let mut trees = Vec::new();
    for file in json_files(&inputs) {
        let name = file.file_stem().unwrap().to_string_lossy().to_string();
        let capture = read_json(&file);
        let plan = plan::load(&name, &capture);
        drop(capture);
        eprintln!(
            "{name}: {} nodes, {} measured, depth {}, scale {}",
            plan.nodes.len(), plan.measured(), plan.max_depth(), plan.scale
        );
        let result = scenarios::run(&plan, &budget, rects.as_deref());
        for s in &result.scenarios {
            eprintln!(
                "  {:<16} median {:>10.2} µs  p95 {:>10.2} µs  ({}×{}) {:?}",
                s.name, s.stats.median, s.stats.p95, s.stats.samples, s.stats.batch, s.counters
            );
        }
        trees.push(scenarios::tree_json(&plan, &result));
    }
    let file = json!({
        "engine": "souple",
        "machine": machine(),
        "build": build_flags(),
        "budget_seconds": budget.seconds,
        "sizes": {
            "Style": std::mem::size_of::<souple::Style>(),
            "Length": std::mem::size_of::<souple::Length>(),
            "NodeLayout": std::mem::size_of::<souple::NodeLayout>(),
        },
        "trees": trees,
    });
    if let Some(parent) = out.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    std::fs::write(&out, serde_json::to_string_pretty(&file).unwrap()).unwrap();
    eprintln!("wrote {}", out.display());
}

fn report_cmd(args: &[String]) {
    let mut files = Vec::new();
    let mut rects = None;
    let mut md = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--rects" => {
                i += 1;
                rects = Some(PathBuf::from(&args[i]));
            }
            "--md" => {
                i += 1;
                md = Some(PathBuf::from(&args[i]));
            }
            path => files.push(PathBuf::from(path)),
        }
        i += 1;
    }
    let souple = read_json(files.first().expect("report SOUPLE.json [YOGA.json]"));
    let yoga = files.get(1).map(|p| read_json(p));
    let text = report::markdown(&souple, yoga.as_ref(), rects.as_deref());
    match md {
        Some(path) => std::fs::write(path, &text).unwrap(),
        None => print!("{text}"),
    }
}
