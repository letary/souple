//! A tree to benchmark, engine-neutral: Yoga's capture JSON read into a preorder list of nodes with
//! Souple styles, measure specs and the facts the scenarios pick nodes by.
//!
//! The style mapping mirrors Yoga's own loader (`benchmark/Benchmark.cpp`, `setStylesFromJson`)
//! key by key — the same keys, the same accepted units — on top of YOGA's defaults, and resolves
//! what Souple does not have the way Yoga resolves it for an LTR tree:
//! - edges: `start` > `left` > `horizontal` > `all` (`end` > `right` for the right edge), `top` >
//!   `vertical` > `all` (yoga/style/Style.h, `computeLeftEdge` …);
//! - `flex: f` (no `flex-grow`/`-shrink`/`-basis`): grow `f` when positive, shrink `-f` when
//!   negative, basis 0 when positive (yoga/node/Node.cpp, `resolveFlexGrow` …, `processFlexBasis`);
//! - an unset minimum: [`Plan::styles`] gives CSS's `auto` (`MinSize::Auto`, as crates/yoga-tests'
//!   `yoga_default` — the scenarios' mapping: Yoga ignores a `flex-basis` along an indefinite main
//!   size and sizes by content, which the automatic minimum reproduces) or Yoga's 0 (`Zero`);
//! - the root's viewport pin (`min == max == the available size`) becomes the viewport (`load`).
//!
//! What Souple lacks — `position: static`, RTL, `display: contents` — is approximated and COUNTED
//! in [`Notes`]. The captures use none of the three; Yoga's `errata` (all four captures say "all")
//! cannot be mapped at all and are reported as a string.

use std::collections::BTreeMap;

use serde_json::Value;
use souple::{
    AlignContent, AlignItems, BoxSizing, Display, Edges, FlexDirection, FlexWrap, JustifyContent, Length, Overflow,
    Position, Size, Style,
};

use crate::measure::{MeasureSpec, Mode, Record};

/// What an unset minimum size is.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MinSize {
    /// Yoga's: 0.
    Zero,
    /// CSS's `auto`: the automatic minimum of a flex item (costs a min-content query when it matters).
    Auto,
}

pub struct PlanNode {
    pub parent: Option<u32>,
    pub depth: u32,
    /// Souple's style on Yoga's semantics, with an unset minimum as 0 (see [`Plan::styles`]).
    pub style: Style,
    /// Which edges / sizes of `style.min_size` were unset (they become `Auto` under `MinSize::Auto`).
    pub min_unset: (bool, bool),
    pub spec: Option<u32>,
    pub children: u32,
    /// Nodes in the subtree, the node included.
    pub subtree: u32,
    /// `display: none` on the node or an ancestor.
    pub hidden: bool,
}

#[derive(Default, Debug)]
pub struct Notes {
    pub static_position: usize,
    pub rtl: usize,
    pub display_contents: usize,
    /// `key=value` pairs mapped to a near value (none expected; listed if any).
    pub approximated: BTreeMap<String, usize>,
    /// Style keys Yoga's loader ignores (so this one does too).
    pub ignored_keys: BTreeMap<String, usize>,
    pub errata: String,
    /// Nodes whose config's point scale differs from the one Souple uses (Souple has one per tree).
    pub other_scale: usize,
    /// Root axes pinned to the viewport by `min == max` and turned into the viewport (see `load`).
    pub viewport_pin: usize,
}

pub struct Plan {
    pub name: String,
    pub width: f32,
    pub height: f32,
    /// The point scale factor most nodes' configs give: Souple's `set_scale`.
    pub scale: f32,
    pub nodes: Vec<PlanNode>,
    pub specs: Vec<MeasureSpec>,
    /// Preorder index → spec index, `u32::MAX` = not measured.
    pub spec_of: Vec<u32>,
    pub notes: Notes,
}

impl Plan {
    pub fn measured(&self) -> usize {
        self.specs.len()
    }

    pub fn max_depth(&self) -> u32 {
        self.nodes.iter().map(|n| n.depth).max().unwrap_or(0)
    }

    /// The styles for a minimum-size policy.
    pub fn styles(&self, min: MinSize) -> Vec<Style> {
        self.nodes
            .iter()
            .map(|node| {
                let mut style = node.style.clone();
                if min == MinSize::Auto {
                    if node.min_unset.0 {
                        style.min_size.width = Length::Auto;
                    }
                    if node.min_unset.1 {
                        style.min_size.height = Length::Auto;
                    }
                }
                style
            })
            .collect()
    }

    // ── the nodes the scenarios act on: the same rules as yoga/bench.cpp (`pick_*`) ─────────────

    /// The deepest visible leaf (no children); the first in preorder on a tie.
    pub fn deep_leaf(&self) -> usize {
        self.deepest(|n| n.children == 0)
    }

    /// The deepest visible measured leaf.
    pub fn deep_measured(&self) -> Option<usize> {
        (!self.specs.is_empty()).then(|| self.deepest(|n| n.spec.is_some()))
    }

    fn deepest(&self, mut pred: impl FnMut(&PlanNode) -> bool) -> usize {
        let mut best: Option<usize> = None;
        for (i, node) in self.nodes.iter().enumerate() {
            if node.hidden || !pred(node) {
                continue;
            }
            if best.is_none_or(|b| node.depth > self.nodes[b].depth) {
                best = Some(i);
            }
        }
        best.unwrap_or(0)
    }

    /// The node a width tween animates: a visible in-flow container (not the root) whose subtree is
    /// closest to 5% of the tree (at least 3 nodes); the first in preorder on a tie.
    pub fn tween_node(&self) -> usize {
        let target = (self.nodes.len() as i64 / 20).max(3);
        let mut best: Option<(i64, usize)> = None;
        for (i, node) in self.nodes.iter().enumerate().skip(1) {
            if node.hidden || node.children == 0 || node.subtree < 3 || node.style.position == Position::Absolute {
                continue;
            }
            let distance = (node.subtree as i64 - target).abs();
            if best.is_none_or(|(d, _)| distance < d) {
                best = Some((distance, i));
            }
        }
        best.map_or(0, |(_, i)| i)
    }
}

/// Yoga's defaults (yoga/style/Style.h with `useWebDefaults` off), as Souple fields.
pub fn yoga_default() -> Style {
    Style {
        flex_direction: FlexDirection::Column,
        flex_shrink: 0.0,
        align_content: AlignContent::FlexStart,
        box_sizing: BoxSizing::BorderBox,
        position: Position::Relative,
        min_size: Size::new(Length::Px(0.0), Length::Px(0.0)),
        ..Style::default()
    }
}

// ── loading ─────────────────────────────────────────────────────────────────────────────────────

pub fn load(name: &str, capture: &Value) -> Plan {
    let inputs = &capture["layout-inputs"];
    let width = inputs["available-width"].as_f64().expect("available-width") as f32;
    let height = inputs["available-height"].as_f64().expect("available-height") as f32;
    let mut notes = Notes::default();
    if inputs["owner-direction"].as_str() == Some("rtl") {
        notes.rtl += 1;
    }

    let mut loader = Loader { nodes: Vec::new(), specs: Vec::new(), notes, scales: BTreeMap::new(), errata: BTreeMap::new() };
    // Iterative preorder walk (the deep synthetic trees nest thousands of levels).
    let mut stack: Vec<(&Value, Option<u32>, u32, bool)> = vec![(&capture["tree"], None, 0, false)];
    while let Some((json, parent, depth, hidden)) = stack.pop() {
        let index = loader.node(json, parent, depth, hidden);
        let hidden = loader.nodes[index].hidden;
        if let Some(children) = json["children"].as_array() {
            loader.nodes[index].children = children.len() as u32;
            for child in children.iter().rev() {
                stack.push((child, Some(index as u32), depth + 1, hidden));
            }
        }
    }
    // Subtree sizes: children follow their parent in preorder, so one reverse sweep adds them up.
    for i in (0..loader.nodes.len()).rev() {
        loader.nodes[i].subtree += 1;
        if let Some(parent) = loader.nodes[i].parent {
            let size = loader.nodes[i].subtree;
            loader.nodes[parent as usize].subtree += size;
        }
    }

    // The captured roots pin their size as `min == max == the available size` (React Native's way of
    // sizing a root). That pin IS the viewport: dropped, the root fills the viewport by itself — the
    // same box — and a viewport change reaches it. yoga/bench.cpp drops it the same way.
    if let Some(root) = loader.nodes.first_mut() {
        let s = &mut root.style;
        if s.min_size.width == Length::Px(width) && s.max_size.width == Length::Px(width) {
            (s.min_size.width, s.max_size.width) = (Length::Px(0.0), Length::Auto);
            root.min_unset.0 = true;
            loader.notes.viewport_pin += 1;
        }
        if s.min_size.height == Length::Px(height) && s.max_size.height == Length::Px(height) {
            (s.min_size.height, s.max_size.height) = (Length::Px(0.0), Length::Auto);
            root.min_unset.1 = true;
            loader.notes.viewport_pin += 1;
        }
    }

    let scale = loader.scales.iter().max_by_key(|(_, count)| **count).map_or(1.0, |(bits, _)| f32::from_bits(*bits));
    loader.notes.other_scale =
        loader.scales.iter().filter(|(bits, _)| f32::from_bits(**bits) != scale).map(|(_, c)| *c).sum();
    loader.notes.errata = loader.errata.keys().cloned().collect::<Vec<_>>().join("+");
    let mut spec_of = vec![u32::MAX; loader.nodes.len()];
    for (i, node) in loader.nodes.iter().enumerate() {
        if let Some(spec) = node.spec {
            spec_of[i] = spec;
        }
    }
    Plan { name: name.to_string(), width, height, scale, nodes: loader.nodes, specs: loader.specs, spec_of, notes: loader.notes }
}

struct Loader {
    nodes: Vec<PlanNode>,
    specs: Vec<MeasureSpec>,
    notes: Notes,
    scales: BTreeMap<u32, usize>,
    errata: BTreeMap<String, usize>,
}

/// A length as Yoga's `unitFromJson` reads it.
#[derive(Clone, Copy, PartialEq)]
enum Unit {
    Undefined,
    Auto,
    Px(f32),
    Pct(f32),
}

fn unit(value: &Value) -> Unit {
    match value.as_str() {
        Some("auto") => return Unit::Auto,
        Some("undefined") => return Unit::Undefined,
        _ => {}
    }
    let number = value["value"].as_f64().map(|v| v as f32);
    match value["unit"].as_str() {
        Some("px") => Unit::Px(number.expect("a px value")),
        Some("pct") => Unit::Pct(number.expect("a pct value")),
        other => panic!("unknown unit {other:?}"),
    }
}

/// The nine edges a Yoga edge property may be set on.
#[derive(Default)]
struct YogaEdges {
    left: Option<Length>,
    top: Option<Length>,
    right: Option<Length>,
    bottom: Option<Length>,
    start: Option<Length>,
    end: Option<Length>,
    horizontal: Option<Length>,
    vertical: Option<Length>,
    all: Option<Length>,
}

impl YogaEdges {
    fn set(&mut self, edge: &str, length: Length) {
        let slot = match edge {
            "left" => &mut self.left,
            "top" => &mut self.top,
            "right" => &mut self.right,
            "bottom" => &mut self.bottom,
            "start" => &mut self.start,
            "end" => &mut self.end,
            "horizontal" => &mut self.horizontal,
            "vertical" => &mut self.vertical,
            "all" => &mut self.all,
            other => panic!("unknown edge {other}"),
        };
        *slot = Some(length);
    }

    /// The physical edges of an LTR node (yoga/style/Style.h, `compute*Edge`); `unset` where none.
    fn resolve(&self, unset: &Length) -> Edges<Length> {
        let pick = |options: [&Option<Length>; 4]| options.into_iter().flatten().next().cloned().unwrap_or(unset.clone());
        Edges {
            left: pick([&self.start, &self.left, &self.horizontal, &self.all]),
            right: pick([&self.end, &self.right, &self.horizontal, &self.all]),
            top: pick([&self.top, &self.vertical, &self.all, &None]),
            bottom: pick([&self.bottom, &self.vertical, &self.all, &None]),
        }
    }

    fn is_empty(&self) -> bool {
        [&self.left, &self.top, &self.right, &self.bottom, &self.start, &self.end, &self.horizontal, &self.vertical, &self.all]
            .iter()
            .all(|e| e.is_none())
    }
}

impl Loader {
    fn approximate(&mut self, what: String) {
        *self.notes.approximated.entry(what).or_default() += 1;
    }

    fn node(&mut self, json: &Value, parent: Option<u32>, depth: u32, parent_hidden: bool) -> usize {
        // The config: one scale per Souple tree (the majority's), errata reported.
        let config = &json["config"];
        let scale = config["point-scale-factor"].as_f64().unwrap_or(1.0) as f32;
        *self.scales.entry(scale.to_bits()).or_default() += 1;
        let errata = config["errata"].as_str().unwrap_or("none").to_string();
        *self.errata.entry(errata).or_default() += 1;

        let mut style = yoga_default();
        let mut min_unset = (true, true);
        let (mut flex, mut flex_grow, mut flex_shrink, mut flex_basis) = (None, None, None, None::<Length>);
        let (mut position, mut padding, mut border, mut margin) =
            (YogaEdges::default(), YogaEdges::default(), YogaEdges::default(), YogaEdges::default());
        let mut position_static = false;

        if let Some(map) = json["style"].as_object() {
            for (key, value) in map {
                let text = value.as_str().unwrap_or("");
                match key.as_str() {
                    "flex-direction" => {
                        style.flex_direction = match text {
                            "row" => FlexDirection::Row,
                            "row-reverse" => FlexDirection::RowReverse,
                            "column" => FlexDirection::Column,
                            "column-reverse" => FlexDirection::ColumnReverse,
                            other => panic!("flex-direction {other}"),
                        }
                    }
                    "justify-content" => {
                        style.justify_content = match text {
                            "flex-start" => JustifyContent::FlexStart,
                            "center" => JustifyContent::Center,
                            "flex-end" => JustifyContent::FlexEnd,
                            "space-between" => JustifyContent::SpaceBetween,
                            "space-around" => JustifyContent::SpaceAround,
                            "space-evenly" => JustifyContent::SpaceEvenly,
                            other => panic!("justify-content {other}"),
                        }
                    }
                    "align-items" => {
                        if text == "auto" {
                            self.approximate("align-items=auto".into());
                        }
                        style.align_items = self.align(key, text).unwrap_or(AlignItems::Stretch)
                    }
                    "align-self" => style.align_self = self.align(key, text),
                    "align-content" => {
                        style.align_content = match text {
                            "flex-start" => AlignContent::FlexStart,
                            "center" => AlignContent::Center,
                            "flex-end" => AlignContent::FlexEnd,
                            "stretch" => AlignContent::Stretch,
                            "space-between" => AlignContent::SpaceBetween,
                            "space-around" => AlignContent::SpaceAround,
                            "space-evenly" => AlignContent::SpaceEvenly,
                            other => {
                                self.approximate(format!("align-content={other}"));
                                AlignContent::FlexStart
                            }
                        }
                    }
                    "flex-wrap" => {
                        style.flex_wrap = match text {
                            "no-wrap" => FlexWrap::NoWrap,
                            "wrap" => FlexWrap::Wrap,
                            "wrap-reverse" => FlexWrap::WrapReverse,
                            other => panic!("flex-wrap {other}"),
                        }
                    }
                    "overflow" => {
                        style.overflow = match text {
                            "visible" => Overflow::Visible,
                            "hidden" => Overflow::Hidden,
                            "scroll" => Overflow::Scroll,
                            other => panic!("overflow {other}"),
                        }
                    }
                    "display" => {
                        style.display = match text {
                            "flex" => Display::Flex,
                            "none" => Display::None,
                            "contents" => {
                                self.notes.display_contents += 1;
                                Display::Flex
                            }
                            other => panic!("display {other}"),
                        }
                    }
                    "position-type" => {
                        style.position = match text {
                            "relative" => Position::Relative,
                            "absolute" => Position::Absolute,
                            "static" => {
                                // Static: in flow, insets ignored. Relative without its insets.
                                position_static = true;
                                Position::Relative
                            }
                            other => panic!("position-type {other}"),
                        }
                    }
                    "flex-grow" => flex_grow = value.as_f64().map(|v| v as f32),
                    "flex-shrink" => flex_shrink = value.as_f64().map(|v| v as f32),
                    "flex" => flex = value.as_f64().map(|v| v as f32),
                    "flex-basis" => match unit(value) {
                        Unit::Auto => flex_basis = Some(Length::Auto),
                        Unit::Px(px) => flex_basis = Some(Length::Px(px)),
                        Unit::Pct(pct) => flex_basis = Some(Length::Percent(pct)),
                        Unit::Undefined => {}
                    },
                    "gap" | "column-gap" | "row-gap" => {
                        if let Unit::Px(px) = unit(value) {
                            if key != "row-gap" {
                                style.column_gap = Length::Px(px);
                            }
                            if key != "column-gap" {
                                style.row_gap = Length::Px(px);
                            }
                        }
                    }
                    "width" | "height" => {
                        let length = match unit(value) {
                            Unit::Auto => Some(Length::Auto),
                            Unit::Px(px) => Some(Length::Px(px)),
                            Unit::Pct(pct) => Some(Length::Percent(pct)),
                            Unit::Undefined => None,
                        };
                        if let Some(length) = length {
                            if key == "width" {
                                style.size.width = length;
                            } else {
                                style.size.height = length;
                            }
                        }
                    }
                    "min-width" | "min-height" | "max-width" | "max-height" => {
                        let length = match unit(value) {
                            Unit::Px(px) => Length::Px(px),
                            Unit::Pct(pct) => Length::Percent(pct),
                            _ => continue,
                        };
                        match key.as_str() {
                            "min-width" => {
                                style.min_size.width = length;
                                min_unset.0 = false;
                            }
                            "min-height" => {
                                style.min_size.height = length;
                                min_unset.1 = false;
                            }
                            "max-width" => style.max_size.width = length,
                            _ => style.max_size.height = length,
                        }
                    }
                    _ if key.starts_with("position-") => match unit(value) {
                        Unit::Px(px) => position.set(&key["position-".len()..], Length::Px(px)),
                        Unit::Pct(pct) => position.set(&key["position-".len()..], Length::Percent(pct)),
                        _ => {}
                    },
                    _ if key.starts_with("padding-") => match unit(value) {
                        Unit::Px(px) => padding.set(&key["padding-".len()..], Length::Px(px)),
                        Unit::Pct(pct) => padding.set(&key["padding-".len()..], Length::Percent(pct)),
                        _ => {}
                    },
                    _ if key.starts_with("border-") => {
                        if let Unit::Px(px) = unit(value) {
                            border.set(&key["border-".len()..], Length::Px(px));
                        }
                    }
                    _ if key.starts_with("margin-") => match unit(value) {
                        Unit::Px(px) => margin.set(&key["margin-".len()..], Length::Px(px)),
                        Unit::Pct(pct) => margin.set(&key["margin-".len()..], Length::Percent(pct)),
                        Unit::Auto => margin.set(&key["margin-".len()..], Length::Auto),
                        Unit::Undefined => {}
                    },
                    other => *self.notes.ignored_keys.entry(other.to_string()).or_default() += 1,
                }
            }
        }

        // Yoga's flex resolution, for a node with an owner (a root's grow / shrink play no part).
        style.flex_grow = flex_grow.or(flex.filter(|f| *f > 0.0)).unwrap_or(0.0);
        style.flex_shrink = flex_shrink.or(flex.filter(|f| *f < 0.0).map(|f| -f)).unwrap_or(0.0);
        style.flex_basis = match flex_basis {
            Some(basis) if !basis.is_auto() => basis,
            _ if flex.is_some_and(|f| f > 0.0) => Length::Px(0.0),
            _ => Length::Auto,
        };
        style.padding = padding.resolve(&Length::ZERO);
        style.border = border.resolve(&Length::ZERO);
        style.margin = margin.resolve(&Length::ZERO);
        if position_static {
            self.notes.static_position += 1;
            if !position.is_empty() {
                self.approximate("position-type=static with insets".into());
            }
        } else {
            style.inset = position.resolve(&Length::Auto);
        }

        // A measured leaf: Yoga sets the measure function whenever the key is there, null included.
        let spec = json["node"].get("measure-funcs").map(|funcs| {
            let records = funcs
                .as_array()
                .map(|list| list.iter().map(record).collect())
                .unwrap_or_default();
            self.specs.push(MeasureSpec::new(records));
            (self.specs.len() - 1) as u32
        });

        let hidden = parent_hidden || style.display == Display::None;
        self.nodes.push(PlanNode { parent, depth, style, min_unset, spec, children: 0, subtree: 0, hidden });
        self.nodes.len() - 1
    }

    fn align(&mut self, key: &str, text: &str) -> Option<AlignItems> {
        match text {
            "auto" => None,
            "flex-start" => Some(AlignItems::FlexStart),
            "center" => Some(AlignItems::Center),
            "flex-end" => Some(AlignItems::FlexEnd),
            "stretch" => Some(AlignItems::Stretch),
            "baseline" => Some(AlignItems::Baseline),
            other => {
                self.approximate(format!("{key}={other}"));
                Some(AlignItems::FlexStart)
            }
        }
    }
}

fn record(json: &Value) -> Record {
    let number = |key: &str| json[key].as_f64().map_or(f32::NAN, |v| v as f32);
    Record {
        width: number("width"),
        width_mode: Mode::parse(json["width-mode"].as_str().expect("width-mode")),
        height: number("height"),
        height_mode: Mode::parse(json["height-mode"].as_str().expect("height-mode")),
        out_width: number("output-width"),
        out_height: number("output-height"),
        duration_ns: json["duration-ns"].as_f64().unwrap_or(0.0),
    }
}
