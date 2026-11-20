//! Synthetic trees: deterministic (a seeded SplitMix64), written in Yoga's CAPTURE format, so the
//! Yoga harness and this one load the very same files. A measured leaf carries one record — its
//! natural size, asked with both modes undefined — and the text model of `measure.rs` answers the
//! rest on both sides.
//!
//! - `typical`: a phone screen (390 × 844 @3x): a header row, a scrolling feed of sections of cards
//!   (an avatar row with an absolute badge, wrapping body text, an optional image, a wrapped row of
//!   chips, a row of buttons), a tab bar;
//! - `deep`: deep and narrow, a few spines of nested columns, each level one leaf beside the next
//!   (depth 100 at 1k nodes, 500 at 10k); `deep-shrink` (growth studies only) sizes the spines by
//!   their content;
//! - `wide`: wide and flat, one scrolling column with n − 1 leaves (wrapping texts and boxes).

use serde_json::{Value, json};

pub const VIEWPORT: (f32, f32) = (390.0, 844.0);
pub const SCALE: f32 = 3.0;

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    fn unit(&mut self) -> f32 {
        (self.next() >> 40) as f32 / (1u64 << 24) as f32
    }

    /// A value in `lo..hi`, rounded to the 1/3 px grid of a 3x screen.
    fn range(&mut self, lo: f32, hi: f32) -> f32 {
        ((lo + (hi - lo) * self.unit()) * 3.0).round() / 3.0
    }

    fn int(&mut self, lo: usize, hi_inclusive: usize) -> usize {
        lo + (self.next() % (hi_inclusive - lo + 1) as u64) as usize
    }

    fn chance(&mut self, p: f32) -> bool {
        self.unit() < p
    }
}

fn px(value: f32) -> Value {
    json!({ "unit": "px", "value": value })
}

fn pct(value: f32) -> Value {
    json!({ "unit": "pct", "value": value })
}

struct Gen {
    rng: Rng,
    count: usize,
    config: Value,
}

impl Gen {
    fn new(seed: u64) -> Self {
        Self { rng: Rng(seed), count: 0, config: json!({ "errata": "none", "point-scale-factor": SCALE }) }
    }

    /// (`json!` would deep-copy `children` — quadratic on the deep tree — so they are moved in.)
    fn node(&mut self, style: Value, children: Vec<Value>) -> Value {
        self.count += 1;
        let mut node = json!({ "config": self.config, "node": null });
        node["style"] = style;
        node["children"] = Value::Array(children);
        node
    }

    fn boxed(&mut self, style: Value) -> Value {
        self.node(style, Vec::new())
    }

    /// A measured leaf (a text) of a natural width in `lo..hi` and a line of 16 or 20 px.
    fn text(&mut self, style: Value, lo: f32, hi: f32) -> Value {
        self.count += 1;
        let width = self.rng.range(lo, hi);
        let height = if self.rng.chance(0.7) { 16.0 } else { 20.0 };
        let mut leaf = json!({
            "config": self.config,
            "node": { "measure-funcs": [{
                "width": null, "width-mode": "undefined", "height": null, "height-mode": "undefined",
                "output-width": width, "output-height": height, "duration-ns": 0
            }] },
        });
        leaf["style"] = style;
        leaf
    }

    fn icon(&mut self, size: f32) -> Value {
        self.boxed(json!({ "width": px(size), "height": px(size) }))
    }

    fn capture(&self, tree: Value) -> Value {
        let mut capture = json!({
            "layout-inputs": { "available-width": VIEWPORT.0, "available-height": VIEWPORT.1, "owner-direction": "ltr" },
        });
        capture["tree"] = tree;
        capture
    }

    fn card(&mut self) -> Value {
        let avatar = self.boxed(json!({ "width": px(40.0), "height": px(40.0), "margin-right": px(8.0) }));
        let name = self.text(json!({}), 60.0, 200.0);
        let subtitle = self.text(json!({ "margin-top": px(2.0) }), 80.0, 320.0);
        let names = self.node(json!({ "flex-grow": 1.0, "flex-shrink": 1.0 }), vec![name, subtitle]);
        let badge = self.boxed(json!({
            "position-type": "absolute", "position-top": px(0.0), "position-right": px(0.0),
            "width": px(12.0), "height": px(12.0)
        }));
        let head = self.node(json!({ "flex-direction": "row", "align-items": "center" }), vec![avatar, names, badge]);
        let body = self.text(json!({ "margin-top": px(8.0) }), 200.0, 1400.0);
        let mut children = vec![head, body];
        if self.rng.chance(0.5) {
            children.push(self.boxed(json!({ "width": pct(100.0), "height": px(160.0), "margin-top": px(8.0) })));
        }
        let mut chips = Vec::new();
        for _ in 0..self.rng.int(2, 8) {
            let label = self.text(json!({}), 24.0, 90.0);
            chips.push(self.node(
                json!({ "padding-vertical": px(4.0), "padding-horizontal": px(8.0), "margin-all": px(4.0), "border-all": px(1.0) }),
                vec![label],
            ));
        }
        children.push(self.node(json!({ "flex-direction": "row", "flex-wrap": "wrap", "margin-top": px(8.0) }), chips));
        let mut buttons = Vec::new();
        for _ in 0..3 {
            let icon = self.icon(16.0);
            let label = self.text(json!({ "margin-left": px(4.0) }), 30.0, 80.0);
            buttons.push(self.node(json!({ "flex-direction": "row", "align-items": "center", "padding-all": px(8.0) }), vec![icon, label]));
        }
        children.push(self.node(
            json!({ "flex-direction": "row", "justify-content": "space-between", "margin-top": px(8.0) }),
            buttons,
        ));
        self.node(json!({ "margin-bottom": px(12.0), "padding-all": px(12.0), "border-all": px(1.0) }), children)
    }
}

/// A phone screen of about `n` nodes.
pub fn typical(n: usize, seed: u64) -> Value {
    let mut g = Gen::new(seed);
    let left = g.icon(24.0);
    let title = g.text(json!({ "flex-grow": 1.0, "flex-shrink": 1.0, "margin-horizontal": px(12.0) }), 80.0, 240.0);
    let right = g.icon(24.0);
    let header = g.node(
        json!({
            "flex-direction": "row", "height": px(56.0), "padding-horizontal": px(16.0),
            "align-items": "center", "justify-content": "space-between"
        }),
        vec![left, title, right],
    );
    // The tab bar is 13 nodes and the root 1; cards (about 30 nodes) fill the rest, 8 per section.
    let mut sections = Vec::new();
    while g.count + 2 + 13 + 30 <= n {
        let mut items = vec![g.text(json!({ "margin-bottom": px(8.0) }), 80.0, 200.0)];
        while items.len() <= 8 && g.count + 2 + 13 + 30 <= n {
            items.push(g.card());
        }
        sections.push(g.node(json!({ "padding-bottom": px(8.0) }), items));
    }
    let content = g.node(json!({ "flex-grow": 1.0, "overflow": "scroll", "padding-all": px(12.0) }), sections);
    let mut tabs = Vec::new();
    for _ in 0..4 {
        let icon = g.icon(24.0);
        let label = g.text(json!({ "margin-top": px(2.0) }), 30.0, 70.0);
        tabs.push(g.node(json!({ "align-items": "center", "flex-grow": 1.0, "padding-top": px(8.0) }), vec![icon, label]));
    }
    let tabbar = g.node(
        json!({ "flex-direction": "row", "height": px(64.0), "justify-content": "space-around", "border-top": px(1.0) }),
        tabs,
    );
    let root = g.node(json!({}), vec![header, content, tabbar]);
    g.capture(root)
}

/// Deep and narrow: a root column holding `chains` spines (5 up to 2k nodes, else 10), each a stack
/// of nested columns with one leaf beside the next level — depth ≈ n / (2 × chains): 100 at 1k,
/// 500 at 10k. `shrink`: every third level aligns its items to the start and every third to the
/// centre, so the spine below is sized by its content (growth studies only: both engines are
/// exponential in its depth); else every level stretches, the default.
pub fn deep(n: usize, seed: u64, shrink: bool) -> Value {
    let mut g = Gen::new(seed);
    let chains = if n <= 2_000 { 5 } else { 10 };
    let chains = chains.min((n - 1) / 2).max(1);
    let levels = (n - 1) / chains / 2;
    let mut spines = Vec::with_capacity(chains);
    for _ in 0..chains {
        let mut below = g.text(json!({}), 40.0, 300.0);
        for level in (0..levels.saturating_sub(1)).rev() {
            let leaf = if level % 2 == 0 { g.text(json!({}), 40.0, 300.0) } else { g.icon(8.0) };
            let style = match level % 3 {
                1 if shrink => json!({ "align-items": "flex-start" }),
                2 if shrink => json!({ "align-items": "center" }),
                _ => json!({}),
            };
            below = g.node(style, vec![leaf, below]);
        }
        spines.push(g.node(json!({}), vec![below]));
    }
    let root = g.node(json!({ "overflow": "scroll" }), spines);
    g.capture(root)
}

/// One scrolling column of `n - 1` leaves: wrapping texts and boxes.
pub fn wide(n: usize, seed: u64) -> Value {
    let mut g = Gen::new(seed);
    let mut children = Vec::with_capacity(n);
    for i in 0..n - 1 {
        children.push(if i % 2 == 0 {
            g.text(json!({ "margin-horizontal": px(8.0) }), 40.0, 900.0)
        } else {
            let height = g.rng.range(24.0, 48.0);
            if i % 3 == 0 {
                g.boxed(json!({ "height": px(height), "width": pct(50.0), "margin-vertical": px(2.0) }))
            } else {
                g.boxed(json!({ "height": px(height), "margin-vertical": px(2.0) }))
            }
        });
    }
    let root = g.node(json!({ "overflow": "scroll" }), children);
    g.capture(root)
}

/// Every synthetic tree, by name.
pub fn all() -> Vec<(String, Value)> {
    let mut trees = Vec::new();
    for (label, n) in [("1k", 1_000), ("10k", 10_000)] {
        trees.push((format!("typical-{label}"), typical(n, 0x5eed_0001)));
        trees.push((format!("deep-{label}"), deep(n, 0x5eed_0002, false)));
        trees.push((format!("wide-{label}"), wide(n, 0x5eed_0003)));
    }
    trees
}

/// One tree of a kind, for growth studies (`gen DIR KIND N`).
pub fn one(kind: &str, n: usize) -> Value {
    match kind {
        "typical" => typical(n, 0x5eed_0001),
        "deep" => deep(n, 0x5eed_0002, false),
        "deep-shrink" => deep(n, 0x5eed_0002, true),
        "wide" => wide(n, 0x5eed_0003),
        other => panic!("unknown kind {other}"),
    }
}
