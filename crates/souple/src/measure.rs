use crate::{NodeId, Size};

/// The space a box is offered along one axis.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Available {
    /// This many px.
    Definite(f32),
    /// As narrow as the content allows (every soft wrap taken).
    MinContent,
    /// As wide as the content wants (no wrap).
    MaxContent,
}

/// The question a measured leaf is asked. All sizes are CONTENT sizes in logical px: the padding and
/// the border are the engine's.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MeasureInput {
    /// An axis the layout already fixed; the answer on it is ignored.
    pub known: Size<Option<f32>>,
    /// The space offered on the axes that are not known. A question about the WIDTH offers no
    /// height (`MaxContent`): text is measured by the width it is offered, and a height plays no
    /// part in a width. A height may be offered with a question about the height, at a known width
    /// (the cross size of an item in a row of known height).
    pub available: Size<Available>,
}

/// A measured leaf's answer.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MeasureOutput {
    /// The content's size, in logical px (the padding and the border are added by the engine).
    pub size: Size<f32>,
    /// The first baseline, from the top of the content box; `None` = the bottom of the box.
    pub baseline: Option<f32>,
}

/// The embedder's side of the content-sized leaves (text, images, inputs): the nodes marked with
/// [`crate::Tree::set_measured`]. The engine caches the answers per node until the node is marked
/// dirty ([`crate::Tree::mark_dirty`]), so an implementation need not.
///
/// THE CONTRACT the cache relies on — an answer is reused for a question it implies:
///
/// - offered MORE room than the content took, or less room that still holds it, the content is
///   laid out the same and the answer is the same;
/// - a known size equal to one the content answered in room that HELD it (unbounded, or a definite
///   offer at least that large) gives that same layout. An answer the room did not hold is the
///   content squeezed, not the content at its size — a min-content answer, or one wider than its
///   offer (a word longer than the line) — and is not reused at a known size: "a bb cccccc" at
///   min-content is three lines 60 px wide, and at a known width of 60 two, so the engine asks;
/// - content sits at the top of its box: its first baseline at a width does not depend on the
///   box's height. A box whose two sizes are both known is not measured; when its baseline is
///   read, it is the one of an answer at that width, and only without one is the measurer asked,
///   at the known size.
///
/// Text does this by nature (it breaks the same in any width that fits it). A measurer whose
/// answer depends on anything else must mark the node dirty when that changes.
///
/// A width is measured without a height: a question about the width offers none (see
/// [`MeasureInput::available`]), so a measurer cannot clip its content to an offered height there.
/// A height LIMIT — a `max-height`, a line clamp — is the measurer's own to apply, from what it
/// knows of the node; the engine does not pass one down for it.
///
/// Offered a width, answer the width the content TAKES within it — a wrapped text's longest line,
/// what an ellipsized line kept — not the offer: a leaf that hugs ([`crate::Tree::set_hug`]) is
/// exactly that wide, and so is any leaf the engine sizes by an offer (a column item that is not
/// stretched, an absolute box with an open width).
///
/// Any number is a valid answer as far as the engine's safety goes: a NaN, an infinity or a
/// negative size lays out to something finite (and a negative size is no size), it never panics.
///
/// A closure `FnMut(NodeId, MeasureInput) -> MeasureOutput` is a measurer.
pub trait Measurer {
    /// The size of `node`'s content under `input`.
    fn measure(&mut self, node: NodeId, input: MeasureInput) -> MeasureOutput;
}

impl<F: FnMut(NodeId, MeasureInput) -> MeasureOutput> Measurer for F {
    fn measure(&mut self, node: NodeId, input: MeasureInput) -> MeasureOutput {
        self(node, input)
    }
}

/// A measurer for a tree without measured leaves: every one is 0 × 0.
pub struct NoMeasure;

impl Measurer for NoMeasure {
    fn measure(&mut self, _: NodeId, _: MeasureInput) -> MeasureOutput {
        MeasureOutput::default()
    }
}
