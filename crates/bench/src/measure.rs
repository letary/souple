//! The measured leaves: Yoga's recorded measure-function results, replayed.
//!
//! A capture records, per measured node, the questions Yoga asked and the answers it got (`measure-
//! funcs`: width + mode, height + mode → output size). Souple asks different questions (another
//! algorithm, CSS's min-content queries), so a pure replay would miss. Each leaf is therefore a
//! [`MeasureSpec`]: an EXACT replay first (the same modes and the same numbers as a record), else a
//! small text model fitted to the records. `yoga/bench.cpp` implements the very same rules
//! (`Spec::answer`), so both engines see the same leaves at the same cost: an O(1) lookup of the
//! node's spec, a scan of its 0–2 records, a few flops. No sleeping (Yoga's own harness sleeps for
//! the recorded duration of each call), no text shaping.
//!
//! The model: a block of text whose natural (unwrapped) size is `natural`. Offered a width under its
//! natural width it wraps into `ceil(natural_width / width)` lines of the natural height — unless the
//! records show it is a single truncating line (asked narrower, it answered no taller). Its
//! min-content width (a Souple-only question) is `min(natural_width, 4 × natural_height)`: about one
//! long word. Heights offered are ignored, as a text measure does.

use souple::{Available, MeasureInput, MeasureOutput, Measurer, NodeId, Size};

/// A measure mode, Yoga's three plus CSS's min-content (which Yoga never asks).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Undefined,
    Exactly,
    AtMost,
    MinContent,
}

impl Mode {
    pub fn parse(text: &str) -> Mode {
        match text {
            "exactly" => Mode::Exactly,
            "at-most" => Mode::AtMost,
            "undefined" => Mode::Undefined,
            other => panic!("unknown measure mode {other}"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Record {
    pub width: f32,
    pub width_mode: Mode,
    pub height: f32,
    pub height_mode: Mode,
    pub out_width: f32,
    pub out_height: f32,
    /// What the call cost when it was captured (`duration-ns`): reported, never spent.
    pub duration_ns: f64,
}

#[derive(Clone, Debug)]
pub struct MeasureSpec {
    pub records: Vec<Record>,
    pub natural: (f32, f32),
    pub single_line: bool,
    /// The mean captured duration of a call: what a real measure of this leaf cost on the device.
    pub cost_ns: f64,
}

impl MeasureSpec {
    pub fn new(records: Vec<Record>) -> Self {
        // The natural size: the widest answer to a question that did not fix the width; else the
        // widest answer at all; with no record, Yoga's harness's fallback, 10 × 10.
        let widest = |fixed_allowed: bool| {
            let mut best: Option<&Record> = None;
            for record in &records {
                if !fixed_allowed && record.width_mode == Mode::Exactly {
                    continue;
                }
                if best.is_none_or(|b| record.out_width > b.out_width) {
                    best = Some(record);
                }
            }
            best
        };
        let natural = widest(false).or_else(|| widest(true)).map_or((10.0, 10.0), |r| (r.out_width, r.out_height));
        let single_line =
            records.iter().any(|r| r.out_width < natural.0 - 0.5 && r.out_height <= natural.1 + 0.01);
        let cost_ns =
            if records.is_empty() { 0.0 } else { records.iter().map(|r| r.duration_ns).sum::<f64>() / records.len() as f64 };
        Self { records, natural, single_line, cost_ns }
    }

    /// The answer to one question. `bump` is added to the height: a "content changed" toggle.
    #[inline]
    pub fn answer(&self, width: f32, width_mode: Mode, height: f32, height_mode: Mode, bump: f32) -> (f32, f32) {
        for r in &self.records {
            if r.width_mode == width_mode
                && r.height_mode == height_mode
                && (width_mode == Mode::Undefined || r.width == width)
                && (height_mode == Mode::Undefined || r.height == height)
            {
                return (r.out_width, r.out_height + bump);
            }
        }
        let (natural_width, natural_height) = self.natural;
        let w = match width_mode {
            Mode::Exactly => width,
            Mode::AtMost => natural_width.min(width),
            Mode::Undefined => natural_width,
            Mode::MinContent => natural_width.min(4.0 * natural_height),
        };
        let lines = if self.single_line || w >= natural_width - 0.01 { 1.0 } else { (natural_width / w.max(1.0)).ceil() };
        (w, natural_height * lines + bump)
    }
}

/// The Souple side: a [`Measurer`] over the specs of one tree. A fresh tree's nodes are created in
/// preorder, so a node's slot (the low half of `NodeId::to_bits`) is its preorder index.
pub struct Replay<'a> {
    pub specs: &'a [MeasureSpec],
    /// Preorder index → index into `specs` (`u32::MAX` = not measured).
    pub spec_of: &'a [u32],
    pub calls: u64,
    /// The captured cost of the calls made (Σ `cost_ns`): what they would have cost on the device.
    pub cost_ns: f64,
    /// (spec index, height added): the leaf whose content the one-leaf scenario changes.
    pub bump: Option<(u32, f32)>,
}

impl<'a> Replay<'a> {
    pub fn new(specs: &'a [MeasureSpec], spec_of: &'a [u32]) -> Self {
        Self { specs, spec_of, calls: 0, cost_ns: 0.0, bump: None }
    }

    pub fn reset(&mut self) {
        self.calls = 0;
        self.cost_ns = 0.0;
    }
}

fn axis(known: Option<f32>, available: Available) -> (f32, Mode) {
    match (known, available) {
        (Some(px), _) => (px, Mode::Exactly),
        (None, Available::Definite(px)) => (px, Mode::AtMost),
        (None, Available::MaxContent) => (f32::NAN, Mode::Undefined),
        (None, Available::MinContent) => (f32::NAN, Mode::MinContent),
    }
}

impl Measurer for Replay<'_> {
    #[inline]
    fn measure(&mut self, node: NodeId, input: MeasureInput) -> MeasureOutput {
        self.calls += 1;
        let slot = (node.to_bits() & 0xffff_ffff) as usize;
        let spec = self.spec_of[slot];
        let bump = match self.bump {
            Some((bumped, by)) if bumped == spec => by,
            _ => 0.0,
        };
        let (width, width_mode) = axis(input.known.width, input.available.width);
        let (height, height_mode) = axis(input.known.height, input.available.height);
        let spec = &self.specs[spec as usize];
        self.cost_ns += spec.cost_ns;
        let (w, h) = spec.answer(width, width_mode, height, height_mode, bump);
        MeasureOutput { size: Size::new(w, h), baseline: None }
    }
}
