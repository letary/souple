//! What the generated Yoga tests run on: Yoga's default style, a tree that knows its nodes by the
//! names Yoga's tests give them, Yoga's fake text measurer, and a check that reports every box that
//! differs, not the first.

use std::collections::HashMap;
use std::fmt::Write as _;

use souple::{Available, MeasureInput, MeasureOutput, Measurer, NodeId, Size, Tree, Viewport};
pub use souple::{
    AlignContent, AlignItems, BoxSizing, Display, FlexDirection, FlexWrap, JustifyContent, Length, Overflow,
    Position, Style,
};

/// The style every Yoga node starts from. The engine's `Style::default()` is CSS's initial values;
/// Yoga's differ (yoga/style/Style.h, with `useWebDefaults` off as in `YGConfigNew`), and so did the
/// stylesheet Chrome rendered the fixtures with (gentest/test-template.html). The two agree on every
/// field written here.
pub fn yoga_default() -> Style {
    Style {
        flex_direction: FlexDirection::Column,
        flex_shrink: 0.0,
        align_content: AlignContent::FlexStart,
        box_sizing: BoxSizing::BorderBox,
        // Not a Yoga default: Yoga has no automatic minimum (an unset minimum is 0). But the expected
        // boxes are Chrome's, and Chrome had `min-width: auto` — the template does not set it, and
        // no fixture writes a minimum of 0 — so the CSS value stays.
        min_size: Size::new(Length::Auto, Length::Auto),
        ..Style::default()
    }
}

/// A Yoga test's tree on an engine of its own.
pub struct Fixture {
    tree: Tree,
    names: HashMap<NodeId, &'static str>,
    /// The measured leaves and their text: what [`TextMeasure`] reads.
    texts: HashMap<NodeId, &'static str>,
}

impl Fixture {
    pub fn new() -> Self {
        Self { tree: Tree::new(), names: HashMap::new(), texts: HashMap::new() }
    }

    /// `YGNodeSetContext(node, text)` + `YGNodeSetMeasureFunc(node, IntrinsicSizeMeasure)`: the node
    /// becomes a measured leaf, sized as Yoga's test measurer sizes `text`.
    pub fn text(&mut self, node: NodeId, text: &'static str) {
        self.tree.set_measured(node, true);
        self.texts.insert(node, text);
    }

    /// `YGNodeNewWithConfig` and its `YGNodeStyleSet*` calls: a node on Yoga's defaults, then `edit`.
    pub fn root(&mut self, name: &'static str, edit: impl FnOnce(&mut Style)) -> NodeId {
        let mut style = yoga_default();
        edit(&mut style);
        let node = self.tree.create_node();
        self.tree.set_style(node, style);
        self.names.insert(node, name);
        node
    }

    /// [`Fixture::root`] and `YGNodeInsertChild` as the last child of `parent`.
    pub fn child(&mut self, parent: NodeId, name: &'static str, edit: impl FnOnce(&mut Style)) -> NodeId {
        let node = self.root(name, edit);
        self.tree.append(parent, node);
        node
    }

    /// `YGNodeCalculateLayout(root, width, height, YGDirectionLTR)`, `YGUndefined` being `None`.
    pub fn compute_layout(&mut self, root: NodeId, width: Option<f32>, height: Option<f32>) {
        self.tree.set_viewport(root, Viewport::Size(Size::new(width, height)));
        self.tree.compute_layout(&mut TextMeasure { texts: &self.texts });
    }

    /// Yoga's `ASSERT_FLOAT_EQ`s: each node's `[left, top, width, height]` against the published box.
    /// Fails once, listing every box that differs, so one run shows the whole picture of a test.
    #[track_caller]
    pub fn check(&self, expected: &[(NodeId, [f32; 4])]) {
        // The published boxes are snapped to whole px, and Yoga's are whole px too: a difference is
        // never a rounding error, so the tolerance only absorbs float noise.
        let same = |a: f32, b: f32| (a - b).abs() < 1e-3;
        let mut report = String::new();
        let mut wrong = 0;
        for (node, want) in expected {
            let rect = self.tree.layout(*node).rect;
            let got = [rect.x, rect.y, rect.width, rect.height];
            if want.iter().zip(&got).all(|(a, b)| same(*a, *b)) {
                continue;
            }
            wrong += 1;
            let fields: Vec<&str> = ["left", "top", "width", "height"]
                .into_iter()
                .zip(want.iter().zip(&got))
                .filter(|(_, (a, b))| !same(**a, **b))
                .map(|(field, _)| field)
                .collect();
            let _ = writeln!(
                report,
                "  {:<24} expected {:<22} got {:<22} ({})",
                self.names[node],
                format!("{want:?}"),
                format!("{got:?}"),
                fields.join(", "),
            );
        }
        if wrong > 0 {
            panic!("{wrong} of {} boxes differ from Yoga's [left, top, width, height]:\n{report}", expected.len());
        }
    }
}

// ── Yoga's text measurer ─────────────────────────────────────────────────────────────────────

/// Yoga's `IntrinsicSizeMeasure` (tests/util/TestUtil.cpp), the measure function of every Yoga test
/// whose leaf holds text: a fixed-pitch stand-in for the Ahem font Chrome rendered the fixtures with
/// (`font: 10px/1 Ahem` in gentest/test-template.html). Every byte is 10 px wide, every line 10 px
/// high, and lines break at spaces.
///
/// It is ported as it is, quirks included, because it is the measurer Yoga passes these tests with
/// (their expectations are Chrome's): an open width is the whole text on one line; a width offered
/// "at most" is taken whole when the text is longer (not narrowed to the longest line that fits); a
/// word wider than the offered width widens the wrap to that word. It reads neither the node's
/// style nor its flex direction (Yoga 3.2.1's does not).
struct TextMeasure<'a> {
    texts: &'a HashMap<NodeId, &'static str>,
}

/// Yoga's `YGMeasureMode` with its size: how a measure function is asked about one axis.
#[derive(Clone, Copy)]
enum Mode {
    Exactly(f32),
    AtMost(f32),
    Undefined,
}

const CHAR_WIDTH: f32 = 10.0;
const LINE_HEIGHT: f32 = 10.0;

impl Measurer for TextMeasure<'_> {
    fn measure(&mut self, node: NodeId, input: MeasureInput) -> MeasureOutput {
        let text = self.texts[&node];
        // The engine's question in Yoga's terms. Both speak in content-box px. A known axis is
        // Yoga's "exactly", the space offered its "at most", max-content its "undefined" (no limit,
        // so no wrap). Yoga never asks for a min-content width; the narrowest this text wraps to is
        // its longest word, and "at most" that width answers exactly that. A text has one height at
        // a given width, so a min-content height is the undefined one.
        let width = match (input.known.width, input.available.width) {
            (Some(width), _) => Mode::Exactly(width),
            (None, Available::Definite(width)) => Mode::AtMost(width),
            (None, Available::MaxContent) => Mode::Undefined,
            (None, Available::MinContent) => Mode::AtMost(longest_word_width(text)),
        };
        let height = match (input.known.height, input.available.height) {
            (Some(height), _) => Mode::Exactly(height),
            (None, Available::Definite(height)) => Mode::AtMost(height),
            (None, Available::MaxContent | Available::MinContent) => Mode::Undefined,
        };
        // A Yoga measure function answers no baseline: Yoga then takes the bottom of the box, as
        // the engine does with `None`.
        MeasureOutput { size: intrinsic_size(text, width, height), baseline: None }
    }
}

/// `IntrinsicSizeMeasure`'s body.
fn intrinsic_size(text: &str, width: Mode, height: Mode) -> Size<f32> {
    let one_line = text.len() as f32 * CHAR_WIDTH;
    let width = match width {
        Mode::Exactly(width) => width,
        Mode::AtMost(width) => one_line.min(width),
        Mode::Undefined => one_line,
    };
    // The text wraps at the width just measured, or at its longest word when that is wider.
    let wrapped = calculate_height(text, longest_word_width(text).max(width));
    let height = match height {
        Mode::Exactly(height) => height,
        Mode::AtMost(height) => wrapped.min(height),
        Mode::Undefined => wrapped,
    };
    Size::new(width, height)
}

/// `longestWordWidth`: the longest run of bytes between spaces.
fn longest_word_width(text: &str) -> f32 {
    text.split(' ').map(str::len).max().unwrap_or(0) as f32 * CHAR_WIDTH
}

/// `calculateHeight`: the height of `text` broken greedily into lines of `width`, each word
/// followed by a space's width.
fn calculate_height(text: &str, width: f32) -> f32 {
    if text.len() as f32 * CHAR_WIDTH <= width {
        return LINE_HEIGHT;
    }
    // The words `getline(stream, word, ' ')` reads: `split`'s, except that a trailing space ends
    // the text rather than starting an empty last word. (Two spaces in a row do make an empty word,
    // in both.)
    let mut words: Vec<&str> = text.split(' ').collect();
    if words.last() == Some(&"") {
        words.pop();
    }
    let mut lines = 1.0;
    let mut line = 0.0;
    for word in words {
        let word = word.len() as f32 * CHAR_WIDTH;
        if word > width {
            // A word wider than the line gets a line of its own. Unreachable from
            // `intrinsic_size`, which never wraps narrower than the longest word; kept to stay
            // Yoga's function.
            if line > 0.0 {
                lines += 1.0;
            }
            lines += 1.0;
            line = 0.0;
        } else if line + word <= width {
            line += word + CHAR_WIDTH;
        } else {
            lines += 1.0;
            line = word + CHAR_WIDTH;
        }
    }
    (if line == 0.0 { lines - 1.0 } else { lines }) * LINE_HEIGHT
}
