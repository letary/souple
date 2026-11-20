//! The Yoga importer: turns Yoga's generated gtest files into Rust tests on the engine.
//!
//! ```text
//! cargo run -p souple-yoga-tests --bin import -- <path to a Yoga checkout>
//! ```
//!
//! It reads `<yoga>/tests/generated/*.cpp` and rewrites `crates/yoga-tests/tests/generated/`: one
//! module per Yoga file and a `mod.rs` that lists them (with the counts), all compiled into the one
//! test binary `tests/yoga.rs`. The tests it writes build on `tests/support/mod.rs`.
//!
//! # The input it relies on
//!
//! Yoga's `gentest` writes these files from HTML fixtures, so every test has one shape, one statement
//! per line. Plain string handling reads it:
//!
//! ```text
//! TEST(YogaTest, <name>) {
//!   GTEST_SKIP();                                          optional: Yoga itself fails the test
//!   YGConfigRef config = YGConfigNew();
//!   YGConfigSet…(config, …);                               optional
//!   YGNodeRef <node> = YGNodeNewWithConfig(config);        per node, the root first, then
//!   YGNodeStyleSet…(<node>, …);                            its style
//!   YGNodeInsertChild(<parent>, <node>, <index>);          its place: parents placed first, in index order
//!   YGNodeSetContext(<node>, (void*)"<text>");             optional, on a leaf: its text, and
//!   YGNodeSetMeasureFunc(<node>, &…::IntrinsicSizeMeasure);  the measurer that reads it
//!   YGNodeCalculateLayout(root, <width>, <height>, YGDirectionLTR);
//!   ASSERT_FLOAT_EQ(<value>, YGNodeLayoutGet{Left,Top,Width,Height}(<node>));   four per node
//!   YGNodeCalculateLayout(root, <width>, <height>, YGDirectionRTL);  the RTL half: not read
//!   …
//! }
//! ```
//!
//! A shape other than this (a child placed before its parent, an edit after the layout pass, an
//! assertion missing) stops the importer with the file and line: the translation would be wrong.
//!
//! # What it does with it
//!
//! Yoga's setters are resolved the way Yoga resolves them for an LTR pass, and the test sets the
//! result on the engine's typed style: per physical edge, `Start` over `Left` over `Horizontal` over
//! `All` (and `End` for the right, `Vertical` for top and bottom); `Row` / `Column` gutters over `All`;
//! `flex` into grow, shrink and basis. Only the LTR expectations are asserted (the engine has no RTL).
//! A leaf measured by Yoga's `IntrinsicSizeMeasure` becomes `t.text(node, "<its text>")`: a
//! measured node, sized by the harness's port of that measurer.
//!
//! A line the engine has no counterpart for (`position: static`, `display: contents`, …) makes the
//! test `#[ignore]`d with that reason — under its category of the crate README's "Limits"
//! ([`LEFT_OUT`]) — and the line is kept in the test as a comment. A line the importer does not
//! know does the same and prints a warning: nothing is dropped quietly. A test the engine answers
//! differently by decision is ignored too, "by design": the [`KNOWN_DIFFERENCES`] table.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::{env, fs, process};

/// The tests the engine answers differently ON PURPOSE: `(Yoga's test name, why)`. Each entry is a
/// decision taken about the engine, not a gap in it, so its test is generated as
/// `#[ignore = "by design: <why>"]` instead of staying red. This table is the only place such
/// decisions live. Keep the why to one line (it is the ignore message and a row of the counts); the
/// comment above an entry, and the READMEs, carry the longer story. An entry that names no test is
/// reported as a warning: the test was renamed or removed upstream.
const KNOWN_DIFFERENCES: &[(&str, &str)] = &[
    // The engine snaps each box edge to the pixel grid by its ABSOLUTE position, so neighbours share
    // an edge and a host that sums parent offsets never drifts. Yoga, and Chrome's captured
    // `offsetTop`, round each box's offset from its parent instead. The two part when a parent sits
    // at a fraction of a px: this root is at top 0.3.
    (
        "rounding_fractial_input_3",
        "pixel snapping by absolute position; Yoga rounds each offset from its parent (root_child1: top 65, not 64)",
    ),
];

/// The features the engine leaves out, and the reason a test that needs one is ignored for: the
/// categories of the souple README's "Limits", word for word. A feature missing from the table is
/// ignored under its plain CSS name (`direction: rtl`), to be given a category when a Yoga version
/// first uses it.
const LEFT_OUT: &[(&str, &str)] = &[
    ("position: static", "by design: every box is positioned (no position: static)"),
    ("display: contents", "not planned: display: contents"),
];

/// The reason of a test Yoga itself skips (`GTEST_SKIP`).
const SKIPPED_BY_YOGA: &str = "skipped by Yoga: GTEST_SKIP, Yoga fails it too";

/// A test needs `feature` (`property: value`), which the engine does not have.
fn lacks(feature: String) -> Skip {
    Skip::Lacks(LEFT_OUT.iter().find(|(name, _)| *name == feature).map_or(feature, |(_, why)| why.to_string()))
}

/// The `Style` fields a test writes, in `Style`'s own order: the order a node's lines come out in.
const FIELDS: &[&str] = &[
    "display", "position", "box_sizing", "overflow",
    "flex_direction", "flex_wrap", "justify_content", "align_items", "align_self", "align_content",
    "flex_grow", "flex_shrink", "flex_basis",
    "size.width", "size.height", "min_size.width", "min_size.height", "max_size.width", "max_size.height",
    "aspect_ratio",
    "margin.left", "margin.top", "margin.right", "margin.bottom",
    "padding.left", "padding.top", "padding.right", "padding.bottom",
    "border.left", "border.top", "border.right", "border.bottom",
    "inset.left", "inset.top", "inset.right", "inset.bottom",
    "row_gap", "column_gap",
];

/// The enum setters: `(setter, field, CSS property, Yoga's value prefix, [(Yoga value, Rust value)])`.
/// A Yoga value missing from its list is one the engine does not have.
type Keyword = (&'static str, &'static str, &'static str, &'static str, &'static [(&'static str, &'static str)]);
const KEYWORDS: &[Keyword] = &[
    ("FlexDirection", "flex_direction", "flex-direction", "YGFlexDirection", &[
        ("Column", "FlexDirection::Column"),
        ("ColumnReverse", "FlexDirection::ColumnReverse"),
        ("Row", "FlexDirection::Row"),
        ("RowReverse", "FlexDirection::RowReverse"),
    ]),
    ("FlexWrap", "flex_wrap", "flex-wrap", "YGWrap", &[
        ("NoWrap", "FlexWrap::NoWrap"),
        ("Wrap", "FlexWrap::Wrap"),
        ("WrapReverse", "FlexWrap::WrapReverse"),
    ]),
    ("JustifyContent", "justify_content", "justify-content", "YGJustify", &[
        ("FlexStart", "JustifyContent::FlexStart"),
        ("FlexEnd", "JustifyContent::FlexEnd"),
        ("Center", "JustifyContent::Center"),
        ("SpaceBetween", "JustifyContent::SpaceBetween"),
        ("SpaceAround", "JustifyContent::SpaceAround"),
        ("SpaceEvenly", "JustifyContent::SpaceEvenly"),
    ]),
    ("AlignItems", "align_items", "align-items", "YGAlign", &[
        ("Stretch", "AlignItems::Stretch"),
        ("FlexStart", "AlignItems::FlexStart"),
        ("FlexEnd", "AlignItems::FlexEnd"),
        ("Center", "AlignItems::Center"),
        ("Baseline", "AlignItems::Baseline"),
    ]),
    ("AlignSelf", "align_self", "align-self", "YGAlign", &[
        ("Auto", "None"),
        ("Stretch", "Some(AlignItems::Stretch)"),
        ("FlexStart", "Some(AlignItems::FlexStart)"),
        ("FlexEnd", "Some(AlignItems::FlexEnd)"),
        ("Center", "Some(AlignItems::Center)"),
        ("Baseline", "Some(AlignItems::Baseline)"),
    ]),
    ("AlignContent", "align_content", "align-content", "YGAlign", &[
        ("Stretch", "AlignContent::Stretch"),
        ("FlexStart", "AlignContent::FlexStart"),
        ("FlexEnd", "AlignContent::FlexEnd"),
        ("Center", "AlignContent::Center"),
        ("SpaceBetween", "AlignContent::SpaceBetween"),
        ("SpaceAround", "AlignContent::SpaceAround"),
        ("SpaceEvenly", "AlignContent::SpaceEvenly"),
    ]),
    ("PositionType", "position", "position", "YGPositionType", &[
        ("Relative", "Position::Relative"),
        ("Absolute", "Position::Absolute"),
    ]),
    ("Overflow", "overflow", "overflow", "YGOverflow", &[
        ("Visible", "Overflow::Visible"),
        ("Hidden", "Overflow::Hidden"),
        ("Scroll", "Overflow::Scroll"),
    ]),
    ("Display", "display", "display", "YGDisplay", &[
        ("Flex", "Display::Flex"),
        ("None", "Display::None"),
    ]),
    ("BoxSizing", "box_sizing", "box-sizing", "YGBoxSizing", &[
        ("BorderBox", "BoxSizing::BorderBox"),
        ("ContentBox", "BoxSizing::ContentBox"),
    ]),
];

/// The size setters: `(setter, field, CSS property)`. Each comes plain (points) and with the unit
/// suffixes below.
const DIMENSIONS: &[(&str, &str, &str)] = &[
    ("Width", "size.width", "width"),
    ("Height", "size.height", "height"),
    ("MinWidth", "min_size.width", "min-width"),
    ("MinHeight", "min_size.height", "min-height"),
    ("MaxWidth", "max_size.width", "max-width"),
    ("MaxHeight", "max_size.height", "max-height"),
    ("FlexBasis", "flex_basis", "flex-basis"),
];

/// The suffixes of `YGNodeStyleSet<Property><Unit>`. The intrinsic keywords have no engine value.
const UNITS: &[&str] = &["Percent", "Auto", "MaxContent", "FitContent", "Stretch"];

/// The edge setters: `(setter, field, the units Yoga has a setter for)`.
const EDGE_PROPERTIES: [(&str, &str, &[&str]); 4] = [
    ("Margin", "margin", &["", "Percent", "Auto"]),
    ("Padding", "padding", &["", "Percent"]),
    ("Border", "border", &[""]),
    ("Position", "inset", &["", "Percent", "Auto"]),
];

/// Yoga's `YGEdge`, in its order: the slots an edge setter writes.
const EDGES: [&str; 9] = ["Left", "Top", "Right", "Bottom", "Start", "End", "Horizontal", "Vertical", "All"];
const LEFT: usize = 0;
const TOP: usize = 1;
const RIGHT: usize = 2;
const BOTTOM: usize = 3;
const START: usize = 4;
const END: usize = 5;
const HORIZONTAL: usize = 6;
const VERTICAL: usize = 7;
const ALL: usize = 8;

/// Which slots decide a physical edge, the first one set winning: Yoga's `computeLeftEdge` & co. in an
/// LTR pass, where `Start` is the left and `End` the right.
const SIDES: [(&str, &[usize]); 4] = [
    ("left", &[START, LEFT, HORIZONTAL, ALL]),
    ("top", &[TOP, VERTICAL, ALL]),
    ("right", &[END, RIGHT, HORIZONTAL, ALL]),
    ("bottom", &[BOTTOM, VERTICAL, ALL]),
];

/// Yoga's `YGGutter`, in its order.
const GUTTERS: [&str; 3] = ["Column", "Row", "All"];

/// The reason of a test with a line the importer does not know (also printed as a warning).
const UNKNOWN: &str = "not understood by the importer:";

fn main() {
    let Some(yoga) = env::args_os().nth(1).map(PathBuf::from) else {
        eprintln!("usage: cargo run -p souple-yoga-tests --bin import -- <path to yoga>");
        process::exit(2);
    };
    let version = yoga_version(&yoga);
    let input = yoga.join("tests").join("generated");
    let mut sources: Vec<PathBuf> = fs::read_dir(&input)
        .unwrap_or_else(|error| exit(&format!("{}: {error}", input.display())))
        .filter_map(|entry| Some(entry.ok()?.path()))
        .filter(|path| path.extension().is_some_and(|extension| extension == "cpp"))
        .collect();
    sources.sort();
    if sources.is_empty() {
        exit(&format!("{}: no .cpp files", input.display()));
    }

    let output = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("generated");
    fs::create_dir_all(&output).unwrap_or_else(|error| exit(&format!("{}: {error}", output.display())));
    // Every module is written again, so one whose Yoga file is gone must not linger.
    for entry in fs::read_dir(&output).into_iter().flatten().flatten() {
        if entry.path().extension().is_some_and(|extension| extension == "rs") {
            fs::remove_file(entry.path()).unwrap_or_else(|error| exit(&format!("{}: {error}", entry.path().display())));
        }
    }

    let mut summary = Summary::default();
    let mut modules = Vec::new();
    let mut unmatched: Vec<&str> = KNOWN_DIFFERENCES.iter().map(|(name, _)| *name).collect();
    for source in &sources {
        let file = source.file_name().expect("a listed file has a name").to_string_lossy().into_owned();
        let text = fs::read_to_string(source).unwrap_or_else(|error| exit(&format!("{}: {error}", source.display())));
        let tests = parse(&file, &text);
        unmatched.retain(|name| !tests.iter().any(|test| test.name == *name));
        let module = module_name(&file);
        let mut code = header(&version, &format!("tests/generated/{file}"));
        code.push_str("\nuse crate::support::*;\n");
        for test in &tests {
            code.push('\n');
            emit_test(&mut code, test);
        }
        write(&output.join(format!("{module}.rs")), &code);
        summary.add(&file, &tests);
        modules.push(module);
    }
    for name in unmatched {
        summary.warnings.push(format!("KNOWN_DIFFERENCES names `{name}`, which no Yoga test has: drop the entry"));
    }

    let table = summary.markdown(&version);
    let mut code = header(&version, "tests/generated/*.cpp");
    code.push('\n');
    for line in table.lines() {
        code.push_str(format!("//! {line}").trim_end());
        code.push('\n');
    }
    code.push('\n');
    modules.sort();
    for module in &modules {
        writeln!(code, "mod {module};").unwrap();
    }
    write(&output.join("mod.rs"), &code);

    println!("wrote {} modules to {}\n", modules.len(), output.display());
    print!("{table}");
    for warning in &summary.warnings {
        eprintln!("warning: {warning}");
    }
}

// ── reading ──────────────────────────────────────────────────────────────────────────────────

/// A length as Yoga's setters give it.
#[derive(Clone, Copy)]
enum Len {
    Px(f64),
    Percent(f64),
    Auto,
}

impl Len {
    fn rust(self) -> String {
        match self {
            Self::Px(value) => format!("Length::Px({})", float(value)),
            Self::Percent(value) => format!("Length::Percent({})", float(value)),
            Self::Auto => "Length::Auto".to_string(),
        }
    }
}

/// Why a line was not translated.
enum Skip {
    /// The engine has no counterpart: the test is ignored for this reason.
    Lacks(String),
    /// The importer does not know the call: ignored, and a warning.
    Unknown,
    /// The line is not of the shape the importer relies on: it stops.
    Malformed(String),
}

impl From<String> for Skip {
    fn from(message: String) -> Self {
        Self::Malformed(message)
    }
}

impl From<&str> for Skip {
    fn from(message: &str) -> Self {
        Self::Malformed(message.to_string())
    }
}

/// A node as Yoga's setters left it: the slots Yoga resolves at layout time are kept as Yoga has them.
#[derive(Default)]
struct Node {
    name: String,
    root: bool,
    parent: Option<usize>,
    children: usize,
    /// Fields set outright: index in [`FIELDS`] → Rust value.
    fields: BTreeMap<usize, String>,
    /// Per [`EDGE_PROPERTIES`], per [`EDGES`] slot.
    edges: [[Option<Len>; 9]; 4],
    /// Per [`GUTTERS`] slot.
    gaps: [Option<Len>; 3],
    flex: Option<f64>,
    flex_grow: Option<f64>,
    flex_shrink: Option<f64>,
    flex_basis: Option<Len>,
    /// `YGNodeSetContext`: the text Yoga's test measurer reads.
    text: Option<String>,
    /// `YGNodeSetMeasureFunc(…, IntrinsicSizeMeasure)`: a leaf sized by its text.
    measured: bool,
    /// The node's lines the engine has no counterpart for, kept as comments.
    untranslated: Vec<String>,
}

struct Test {
    name: String,
    /// Why the test is ignored, each once, in the order met; none = it runs.
    reasons: Vec<String>,
    /// Untranslated lines that belong to no node (a config call), kept as comments.
    untranslated: Vec<String>,
    warnings: Vec<String>,
    nodes: Vec<Node>,
    /// The nodes in the order they joined the tree, the root first: the order they are written in.
    order: Vec<usize>,
    /// The LTR pass's width and height, once its line is read.
    viewport: Option<[Option<f64>; 2]>,
    /// The RTL pass began: nothing more is read.
    done: bool,
    /// Per asserted node: left, top, width, height.
    expected: Vec<(usize, [Option<f64>; 4])>,
}

fn parse(file: &str, text: &str) -> Vec<Test> {
    let mut tests = Vec::new();
    let mut current: Option<Test> = None;
    for (index, line) in text.lines().enumerate() {
        if let Err(message) = parse_line(line, &mut current, &mut tests) {
            exit(&format!("{file}:{}: {message}", index + 1));
        }
    }
    if current.is_some() {
        exit(&format!("{file}: the last TEST is not closed"));
    }
    tests
}

fn parse_line(raw: &str, current: &mut Option<Test>, tests: &mut Vec<Test>) -> Result<(), String> {
    let line = raw.trim();
    if let Some(rest) = line.strip_prefix("TEST(") {
        let name = rest.strip_prefix("YogaTest, ").and_then(|rest| rest.strip_suffix(") {"));
        let name = name.ok_or("a TEST line of an unknown shape")?;
        if current.is_some() {
            return Err("a TEST inside a TEST".to_string());
        }
        *current = Some(Test::new(name));
    } else if raw == "}" {
        // The end of a test: a closing brace in the first column. Outside a test it closes nothing.
        if let Some(mut test) = current.take() {
            test.check()?;
            if let Some((_, why)) = KNOWN_DIFFERENCES.iter().find(|(name, _)| *name == test.name) {
                test.ignore(format!("by design: {why}"));
            }
            tests.push(test);
        }
    } else if let Some(test) = current.as_mut().filter(|test| !line.is_empty() && !test.done) {
        test.read(line)?;
    }
    Ok(())
}

impl Test {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            reasons: Vec::new(),
            untranslated: Vec::new(),
            warnings: Vec::new(),
            nodes: Vec::new(),
            order: Vec::new(),
            viewport: None,
            done: false,
            expected: Vec::new(),
        }
    }

    fn ignore(&mut self, reason: String) {
        if !self.reasons.contains(&reason) {
            self.reasons.push(reason);
        }
    }

    fn unknown(&mut self, function: &str) {
        self.ignore(format!("{UNKNOWN} `{function}`"));
        self.warnings.push(format!("`{function}` is not understood: the test is ignored"));
    }

    fn node(&self, name: Option<&&str>) -> Result<usize, String> {
        let name = name.ok_or("a call without its node")?;
        self.nodes.iter().position(|node| node.name == *name).ok_or_else(|| format!("`{name}` is not a node"))
    }

    fn read(&mut self, line: &str) -> Result<(), String> {
        match line {
            "GTEST_SKIP();" => {
                self.ignore(SKIPPED_BY_YOGA.to_string());
                return Ok(());
            }
            "YGConfigRef config = YGConfigNew();" => return Ok(()),
            _ => {}
        }
        if let Some(rest) = line.strip_prefix("YGNodeRef ") {
            let (name, value) = rest.split_once(" = ").ok_or("a YGNodeRef line of an unknown shape")?;
            if value != "YGNodeNewWithConfig(config);" {
                return Err(format!("`{name}` is not made by YGNodeNewWithConfig(config)"));
            }
            if self.viewport.is_some() || self.node(Some(&name)).is_ok() {
                return Err(format!("`{name}` is made twice, or after the layout pass"));
            }
            let root = self.nodes.is_empty();
            self.nodes.push(Node { name: name.to_string(), root, ..Node::default() });
            if root {
                self.order.push(0);
            }
            return Ok(());
        }

        let (function, args) = call(line)?;
        match function {
            "YGNodeInsertChild" => self.insert(&args),
            "YGNodeCalculateLayout" => self.calculate(&args),
            "ASSERT_FLOAT_EQ" => self.expect(&args),
            _ if self.viewport.is_some() => {
                Err(format!("`{function}` after the layout pass: the importer expects every edit before it"))
            }
            _ if function.starts_with("YGConfigSet") => {
                match config_reason(function, &args) {
                    Some(reason) => self.ignore(reason),
                    None => self.unknown(function),
                }
                self.untranslated.push(line.to_string());
                Ok(())
            }
            _ if function.starts_with("YGNode") => {
                let index = self.node(args.first())?;
                match self.nodes[index].apply(function, &args[1..]) {
                    Ok(()) => return Ok(()),
                    Err(Skip::Lacks(reason)) => self.ignore(reason),
                    Err(Skip::Unknown) => self.unknown(function),
                    Err(Skip::Malformed(message)) => return Err(message),
                }
                self.nodes[index].untranslated.push(line.to_string());
                Ok(())
            }
            _ => {
                self.unknown(function);
                self.untranslated.push(line.to_string());
                Ok(())
            }
        }
    }

    fn insert(&mut self, args: &[&str]) -> Result<(), String> {
        let parent = self.node(args.first())?;
        let child = self.node(args.get(1))?;
        let index: usize = args.get(2).and_then(|index| index.parse().ok()).ok_or("an insert without its index")?;
        if !self.order.contains(&parent) {
            return Err(format!("`{}` gets a child before it has a place in the tree", self.nodes[parent].name));
        }
        if self.order.contains(&child) {
            return Err(format!("`{}` is inserted twice, or is the root", self.nodes[child].name));
        }
        if self.nodes[parent].children != index {
            return Err(format!("a child inserted at {index} where the next index is {}", self.nodes[parent].children));
        }
        self.nodes[parent].children += 1;
        self.nodes[child].parent = Some(parent);
        self.order.push(child);
        Ok(())
    }

    fn calculate(&mut self, args: &[&str]) -> Result<(), String> {
        let [node, width, height, direction] = arity(args)?;
        if self.node(Some(&node))? != 0 {
            return Err("a layout of a node other than the root".to_string());
        }
        let size = [number(width)?, number(height)?];
        match direction {
            "YGDirectionLTR" if self.viewport.is_none() => self.viewport = Some(size),
            "YGDirectionRTL" if self.viewport.is_some() => self.done = true,
            _ => return Err(format!("a {direction} pass where one LTR and then one RTL pass are expected")),
        }
        Ok(())
    }

    fn expect(&mut self, args: &[&str]) -> Result<(), String> {
        if self.viewport.is_none() {
            return Err("an assertion before the layout pass".to_string());
        }
        let [value, getter] = arity(args)?;
        let value = number(value)?.ok_or("an expected YGUndefined")?;
        let (getter, getter_args) = call(getter)?;
        let field = ["YGNodeLayoutGetLeft", "YGNodeLayoutGetTop", "YGNodeLayoutGetWidth", "YGNodeLayoutGetHeight"]
            .iter()
            .position(|known| *known == getter)
            .ok_or_else(|| format!("an assertion on `{getter}`"))?;
        let node = self.node(getter_args.first())?;
        let at = match self.expected.iter().position(|(asserted, _)| *asserted == node) {
            Some(at) => at,
            None => {
                self.expected.push((node, [None; 4]));
                self.expected.len() - 1
            }
        };
        if self.expected[at].1[field].replace(value).is_some() {
            return Err(format!("`{getter}({})` is asserted twice", self.nodes[node].name));
        }
        Ok(())
    }

    /// The test is complete: every node placed, every asserted box whole.
    fn check(&self) -> Result<(), String> {
        if self.viewport.is_none() || self.expected.is_empty() {
            return Err(format!("`{}` has no LTR pass, or asserts nothing", self.name));
        }
        if let Some(node) = self.nodes.iter().find(|node| !node.root && node.parent.is_none()) {
            return Err(format!("`{}` is never inserted", node.name));
        }
        if let Some((node, _)) = self.expected.iter().find(|(_, values)| values.contains(&None)) {
            return Err(format!("`{}` lacks one of its four assertions", self.nodes[*node].name));
        }
        // IntrinsicSizeMeasure reads the node's context as its text: without one, Yoga's own test
        // would read a null pointer.
        if let Some(node) = self.nodes.iter().find(|node| node.measured && node.text.is_none()) {
            return Err(format!("`{}` is measured but has no text", node.name));
        }
        Ok(())
    }
}

impl Node {
    fn set(&mut self, field: &str, value: Option<String>) {
        let index = FIELDS.iter().position(|known| *known == field).expect("a field of FIELDS");
        match value {
            Some(value) => self.fields.insert(index, value),
            // Yoga's `YGUndefined`: the property is unset again.
            None => self.fields.remove(&index),
        };
    }

    /// One `YGNodeStyleSet*` / `YGNodeSet*` call on this node.
    fn apply(&mut self, function: &str, args: &[&str]) -> Result<(), Skip> {
        let Some(setter) = function.strip_prefix("YGNodeStyleSet") else {
            // A leaf whose size is its text. gentest writes the text as the node's context and
            // Yoga's tests/util/TestUtil.cpp measures it; tests/support has the same measurer.
            return match function {
                "YGNodeSetContext" => {
                    let [context] = arity(args)?;
                    self.text = Some(text_literal(context)?);
                    Ok(())
                }
                "YGNodeSetMeasureFunc" => match arity(args)? {
                    ["&facebook::yoga::test::IntrinsicSizeMeasure"] => {
                        self.measured = true;
                        Ok(())
                    }
                    _ => Err(Skip::Unknown),
                },
                _ => Err(Skip::Unknown),
            };
        };

        if let Some(&(_, field, css, prefix, values)) = KEYWORDS.iter().find(|keyword| keyword.0 == setter) {
            let [value] = arity(args)?;
            let name = value.strip_prefix(prefix).ok_or_else(|| format!("`{value}` is not a {prefix} value"))?;
            let rust = values.iter().find(|(yoga, _)| *yoga == name).map(|(_, rust)| rust.to_string());
            let rust = rust.ok_or_else(|| lacks(format!("{css}: {}", words(name, '-'))))?;
            // Every fixture's root is `position: absolute`, only so that it sits on its own in the
            // page. Yoga lays a root out the same whatever its position type (it places a root by its
            // margin and its relative offset), so the root keeps the engine's `relative`.
            if !(field == "position" && self.root) {
                self.set(field, Some(rust));
            }
            return Ok(());
        }

        if setter == "Direction" {
            let [value] = arity(args)?;
            return match value {
                // The pass is LTR: LTR (or inherit) on a node changes nothing.
                "YGDirectionLTR" | "YGDirectionInherit" => Ok(()),
                "YGDirectionRTL" => Err(lacks("direction: rtl".to_string())),
                _ => Err(Skip::Malformed(format!("`{value}` is not a direction"))),
            };
        }

        let (base, unit) = UNITS
            .iter()
            .find_map(|unit| Some((setter.strip_suffix(unit)?, *unit)))
            .unwrap_or((setter, ""));

        if let Some(&(_, field, css)) = DIMENSIONS.iter().find(|dimension| dimension.0 == base) {
            let length = match unit {
                "" => number(arity::<1>(args)?[0])?.map(Len::Px),
                "Percent" => number(arity::<1>(args)?[0])?.map(Len::Percent),
                "Auto" => {
                    arity::<0>(args)?;
                    Some(Len::Auto)
                }
                keyword => return Err(lacks(format!("{css}: {}", words(keyword, '-')))),
            };
            if field == "flex_basis" {
                // Resolved with `flex` when the node is written.
                self.flex_basis = length;
            } else {
                self.set(field, length.map(Len::rust));
            }
            return Ok(());
        }

        if let Some(property) = EDGE_PROPERTIES.iter().position(|(yoga, _, units)| *yoga == base && units.contains(&unit)) {
            let edge = args.first().and_then(|edge| edge.strip_prefix("YGEdge"));
            let edge = EDGES.iter().position(|known| Some(*known) == edge).ok_or("an edge setter without its edge")?;
            self.edges[property][edge] = match unit {
                "Auto" => {
                    arity::<1>(args)?;
                    Some(Len::Auto)
                }
                "Percent" => number(arity::<2>(args)?[1])?.map(Len::Percent),
                _ => number(arity::<2>(args)?[1])?.map(Len::Px),
            };
            return Ok(());
        }

        match (base, unit) {
            ("Gap", "" | "Percent") => {
                let [gutter, value] = arity(args)?;
                let gutter = gutter.strip_prefix("YGGutter");
                let gutter = GUTTERS.iter().position(|known| Some(*known) == gutter).ok_or("a gap without its gutter")?;
                let value = number(value)?;
                self.gaps[gutter] = if unit == "Percent" { value.map(Len::Percent) } else { value.map(Len::Px) };
            }
            ("Flex", "") => self.flex = number(arity::<1>(args)?[0])?,
            ("FlexGrow", "") => self.flex_grow = number(arity::<1>(args)?[0])?,
            ("FlexShrink", "") => self.flex_shrink = number(arity::<1>(args)?[0])?,
            ("AspectRatio", "") => {
                // gentest copies the CSS value, so the argument is a C++ division (`1 / 1`) as often as
                // a number. Yoga stores a degenerate ratio (0, infinite) as none: `Style::setAspectRatio`.
                let [ratio] = arity(args)?;
                let ratio = match ratio.split_once(" / ") {
                    Some((width, height)) => number(width)?.zip(number(height)?).map(|(width, height)| width / height),
                    None => number(ratio)?,
                };
                let ratio = ratio.filter(|ratio| *ratio != 0.0 && ratio.is_finite());
                self.set("aspect_ratio", Some(ratio.map_or("None".to_string(), |ratio| format!("Some({})", float(ratio)))));
            }
            _ => return Err(Skip::Unknown),
        }
        Ok(())
    }

    /// The node's style lines: what was set outright, then the slots Yoga resolves at layout time,
    /// resolved here the same way — in `FIELDS` order.
    fn assignments(&self) -> Vec<String> {
        let mut fields = self.fields.clone();
        let mut put = |field: &str, value: String| {
            fields.insert(FIELDS.iter().position(|known| *known == field).expect("a field of FIELDS"), value);
        };
        for (property, (_, field, _)) in EDGE_PROPERTIES.iter().enumerate() {
            for (side, slots) in SIDES {
                if let Some(length) = slots.iter().find_map(|slot| self.edges[property][*slot]) {
                    put(&format!("{field}.{side}"), length.rust());
                }
            }
        }
        let [column, row, all] = self.gaps;
        if let Some(gap) = column.or(all) {
            put("column_gap", gap.rust());
        }
        if let Some(gap) = row.or(all) {
            put("row_gap", gap.rust());
        }
        // `flex` as Yoga reads it without web defaults (Node::resolveFlexGrow, resolveFlexShrink,
        // processFlexBasis): a positive flex grows from a 0 basis, a negative one shrinks; the
        // explicit properties win.
        let flex = self.flex.unwrap_or(0.0);
        if let Some(grow) = self.flex_grow.or((flex > 0.0).then_some(flex)) {
            put("flex_grow", float(grow));
        }
        if let Some(shrink) = self.flex_shrink.or((flex < 0.0).then_some(-flex)) {
            put("flex_shrink", float(shrink));
        }
        match self.flex_basis {
            Some(length @ (Len::Px(_) | Len::Percent(_))) => put("flex_basis", length.rust()),
            _ if flex > 0.0 => put("flex_basis", Len::Px(0.0).rust()),
            Some(Len::Auto) => put("flex_basis", Len::Auto.rust()),
            None => {}
        }
        fields.into_iter().map(|(index, value)| format!("s.{} = {value};", FIELDS[index])).collect()
    }
}

/// The reason a config call makes a test ignored; `None` = a call the importer does not know.
fn config_reason(function: &str, args: &[&str]) -> Option<String> {
    let value = |prefix: &str| args.get(1).map(|arg| arg.strip_prefix(prefix).unwrap_or(arg).to_string());
    match function {
        "YGConfigSetExperimentalFeatureEnabled" => Some(format!("experimental feature {}", value("YGExperimentalFeature")?)),
        "YGConfigSetErrata" => Some(format!("errata {}", value("YGErrata")?)),
        "YGConfigSetUseWebDefaults" => Some("Yoga's web defaults".to_string()),
        "YGConfigSetPointScaleFactor" => Some(format!("point scale factor {}", value("")?)),
        _ => None,
    }
}

// ── writing ──────────────────────────────────────────────────────────────────────────────────

fn header(version: &str, source: &str) -> String {
    format!(
        "// @generated — do not edit. Written by the Yoga importer (crates/yoga-tests/src/bin/import.rs):\n\
         //     cargo run -p souple-yoga-tests --bin import -- <path to yoga>\n\
         // From Yoga {version}, {source} (MIT, Copyright (c) Meta Platforms, Inc. and affiliates).\n"
    )
}

fn emit_test(out: &mut String, test: &Test) {
    let used = |index: usize| {
        let node = &test.nodes[index];
        index == 0 || node.children > 0 || node.measured || test.expected.iter().any(|(asserted, _)| *asserted == index)
    };
    let variable = |index: usize| {
        let name = &test.nodes[index].name;
        if used(index) { name.clone() } else { format!("_{name}") }
    };

    writeln!(out, "#[test]").unwrap();
    if !test.reasons.is_empty() {
        writeln!(out, "#[ignore = {:?}]", test.reasons.join("; ")).unwrap();
    }
    writeln!(out, "fn {}() {{", test.name).unwrap();
    writeln!(out, "    let mut t = Fixture::new();").unwrap();
    for line in &test.untranslated {
        writeln!(out, "    // untranslated: {}", clip(line)).unwrap();
    }
    for &index in &test.order {
        let node = &test.nodes[index];
        let call = match node.parent {
            None => format!("t.root(\"{}\"", node.name),
            Some(parent) => format!("t.child({}, \"{}\"", test.nodes[parent].name, node.name),
        };
        let assignments = node.assignments();
        let mut body: Vec<String> = node.untranslated.iter().map(|line| format!("// untranslated: {}", clip(line))).collect();
        body.extend_from_slice(&assignments);
        let parameter = if assignments.is_empty() { "_" } else { "s" };
        if body.is_empty() {
            writeln!(out, "    let {} = {call}, |_| {{}});", variable(index)).unwrap();
        } else {
            writeln!(out, "    let {} = {call}, |{parameter}| {{", variable(index)).unwrap();
            for line in body {
                writeln!(out, "        {line}").unwrap();
            }
            writeln!(out, "    }});").unwrap();
        }
        if node.measured {
            // In full, not clipped like a comment: it is the leaf's content.
            let text = node.text.as_ref().expect("a checked measured node has its text");
            writeln!(out, "    t.text({}, {text:?});", node.name).unwrap();
        }
    }
    let [width, height] = test.viewport.expect("a checked test has its LTR pass");
    let size = |value: Option<f64>| value.map_or("None".to_string(), |value| format!("Some({})", float(value)));
    writeln!(out, "    t.compute_layout({}, {}, {});", test.nodes[0].name, size(width), size(height)).unwrap();
    writeln!(out, "    t.check(&[").unwrap();
    for (node, values) in &test.expected {
        let values: Vec<String> = values.iter().map(|value| float(value.expect("a checked box is whole"))).collect();
        writeln!(out, "        ({}, [{}]),", test.nodes[*node].name, values.join(", ")).unwrap();
    }
    writeln!(out, "    ]);").unwrap();
    writeln!(out, "}}").unwrap();
}

#[derive(Default)]
struct Summary {
    files: usize,
    tests: usize,
    ignored: usize,
    reasons: BTreeMap<String, usize>,
    warnings: Vec<String>,
}

impl Summary {
    fn add(&mut self, file: &str, tests: &[Test]) {
        self.files += 1;
        for test in tests {
            self.tests += 1;
            self.ignored += usize::from(!test.reasons.is_empty());
            for reason in &test.reasons {
                *self.reasons.entry(reason.clone()).or_default() += 1;
            }
            for warning in &test.warnings {
                self.warnings.push(format!("{file}: {}: {warning}", test.name));
            }
        }
    }

    /// The counts, as Markdown: printed, kept at the top of `generated/mod.rs`, quoted by the README.
    fn markdown(&self, version: &str) -> String {
        let mut reasons: Vec<(&String, &usize)> = self.reasons.iter().collect();
        reasons.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
        let mut out = format!(
            "Yoga {version}: {} tests in {} files — {} run, {} ignored.\n\n",
            self.tests,
            self.files,
            self.tests - self.ignored,
            self.ignored
        );
        out.push_str("| Ignored because | Tests |\n|---|---:|\n");
        for (reason, count) in reasons {
            writeln!(out, "| {reason} | {count} |").unwrap();
        }
        out.push_str("\nA test ignored for several reasons counts under each.\n");
        out
    }
}

// ── small helpers ────────────────────────────────────────────────────────────────────────────

/// `f(a, b);` → `("f", ["a", "b"])`. An argument may hold a call or a string with commas.
fn call(text: &str) -> Result<(&str, Vec<&str>), String> {
    let malformed = || format!("not a call: `{}`", clip(text));
    let text = text.strip_suffix(';').unwrap_or(text);
    let (function, rest) = text.split_once('(').ok_or_else(malformed)?;
    let inner = rest.strip_suffix(')').ok_or_else(malformed)?;
    if function.is_empty() || !function.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return Err(malformed());
    }
    let mut args = Vec::new();
    let (mut start, mut depth, mut quoted) = (0, 0, false);
    for (at, c) in inner.char_indices() {
        match c {
            '"' => quoted = !quoted,
            '(' if !quoted => depth += 1,
            ')' if !quoted => depth -= 1,
            ',' if !quoted && depth == 0 => {
                args.push(inner[start..at].trim());
                start = at + 1;
            }
            _ => {}
        }
    }
    if !inner.trim().is_empty() {
        args.push(inner[start..].trim());
    }
    Ok((function, args))
}

fn arity<'a, const N: usize>(args: &[&'a str]) -> Result<[&'a str; N], String> {
    args.try_into().map_err(|_| format!("{} arguments where {N} are expected", args.len()))
}

/// A C++ float literal (`50`, `0.3f`, `-10`); `YGUndefined` is `None`.
fn number(text: &str) -> Result<Option<f64>, String> {
    if text == "YGUndefined" {
        return Ok(None);
    }
    let digits = text.strip_suffix('f').unwrap_or(text);
    digits.parse().map(Some).map_err(|_| format!("`{text}` is not a number"))
}

/// A measured node's text: `(void*)"Lorem ipsum"` → `Lorem ipsum`. gentest pastes Chrome's
/// `innerText` between the quotes without escaping it, so a quote or a backslash in it would be a
/// broken literal: refused rather than guessed at.
fn text_literal(context: &str) -> Result<String, String> {
    let text = context.strip_prefix("(void*)\"").and_then(|rest| rest.strip_suffix('"'));
    match text {
        Some(text) if !text.contains(['"', '\\']) => Ok(text.to_string()),
        _ => Err(format!("a context other than a plain string literal: `{}`", clip(context))),
    }
}

/// A Rust f32 literal: `50.0`, `0.3`.
fn float(value: f64) -> String {
    let text = format!("{value}");
    if text.contains('.') { text } else { format!("{text}.0") }
}

/// `SpaceBetween` → `space-between` (or `space_between`).
fn words(name: &str, separator: char) -> String {
    let mut out = String::new();
    for (at, c) in name.chars().enumerate() {
        if c.is_ascii_uppercase() && at > 0 {
            out.push(separator);
        }
        out.push(c.to_ascii_lowercase());
    }
    out
}

/// `YGAlignContentTest.cpp` → `align_content`.
fn module_name(file: &str) -> String {
    let stem = file.strip_suffix(".cpp").unwrap_or(file);
    let stem = stem.strip_prefix("YG").unwrap_or(stem);
    words(stem.strip_suffix("Test").unwrap_or(stem), '_')
}

/// A line quoted in a comment or an error: a long one is cut.
fn clip(line: &str) -> String {
    match line.char_indices().nth(120) {
        Some((at, _)) => format!("{}…", &line[..at]),
        None => line.to_string(),
    }
}

/// Yoga's version, from the file its release script writes it to (`set-version.py`).
fn yoga_version(yoga: &Path) -> String {
    let properties = fs::read_to_string(yoga.join("gradle.properties")).unwrap_or_default();
    let version = properties.lines().find_map(|line| line.strip_prefix("VERSION_NAME="));
    version.map_or("(unknown version)".to_string(), |version| version.trim().to_string())
}

fn write(path: &Path, text: &str) {
    fs::write(path, text).unwrap_or_else(|error| exit(&format!("{}: {error}", path.display())));
}

fn exit(message: &str) -> ! {
    eprintln!("import: {message}");
    process::exit(1);
}

// Yoga 3.2.1's gentest never writes `Horizontal` / `Vertical` / `All` edges or `flex`, so no imported
// test would catch a wrong precedence here: these pin Yoga's rules.
#[cfg(test)]
mod tests {
    use super::*;

    /// The style lines of a node after `calls` (`(setter, arguments after the node)`).
    fn lines(calls: &[(&str, &[&str])]) -> Vec<String> {
        let mut node = Node::default();
        for (function, args) in calls {
            assert!(node.apply(function, args).is_ok(), "{function} translates");
        }
        node.assignments()
    }

    #[test]
    fn an_edge_resolves_as_yoga_does_in_ltr() {
        // Start / End over the physical edge over Horizontal / Vertical over All, in any call order.
        let lines = lines(&[
            ("YGNodeStyleSetMargin", &["YGEdgeLeft", "1"]),
            ("YGNodeStyleSetMargin", &["YGEdgeStart", "2"]),
            ("YGNodeStyleSetMargin", &["YGEdgeAll", "3"]),
            ("YGNodeStyleSetMargin", &["YGEdgeVertical", "4"]),
            ("YGNodeStyleSetMargin", &["YGEdgeBottom", "5"]),
            ("YGNodeStyleSetMargin", &["YGEdgeHorizontal", "6"]),
        ]);
        assert_eq!(lines, [
            "s.margin.left = Length::Px(2.0);",
            "s.margin.top = Length::Px(4.0);",
            "s.margin.right = Length::Px(6.0);",
            "s.margin.bottom = Length::Px(5.0);",
        ]);
    }

    #[test]
    fn flex_resolves_as_yoga_does_without_web_defaults() {
        assert_eq!(lines(&[("YGNodeStyleSetFlex", &["2"])]), ["s.flex_grow = 2.0;", "s.flex_basis = Length::Px(0.0);"]);
        assert_eq!(lines(&[("YGNodeStyleSetFlex", &["-1"])]), ["s.flex_shrink = 1.0;"]);
        // The explicit properties win over `flex`, whichever comes first.
        let explicit = lines(&[
            ("YGNodeStyleSetFlexGrow", &["3"]),
            ("YGNodeStyleSetFlex", &["2"]),
            ("YGNodeStyleSetFlexBasisPercent", &["10"]),
        ]);
        assert_eq!(explicit, ["s.flex_grow = 3.0;", "s.flex_basis = Length::Percent(10.0);"]);
    }
}
