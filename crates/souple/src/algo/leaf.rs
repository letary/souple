//! The boxes without a flex layout of their own: a MEASURED leaf (text, an image — sized by the
//! embedder's [`Measurer`]) and an EMPTY box (no children: its content is 0 × 0).
//!
//! The rules:
//! - the Measurer is asked in content-box px, and only about an axis the box does not know: with
//!   both axes known it is not called for the size — only for a baseline the parent reads
//!   (`Reads::Baseline`), and then only when no answer at that content width is cached (the
//!   question is the known size; `Cache::answer_at`). A leaf whose baseline nobody reads is not
//!   measured at a known size at all;
//! - it is asked through the leaf's cache (`super::measure`): a question an earlier answer implies
//!   is not asked again;
//! - an auto size is the content's, within min and max; when that width is not the one the content
//!   was laid out in, the content is asked again at that width, so its height is the height of the
//!   text actually laid out there: when the max cut it under what the content asked for — except in
//!   a leaf that HUGS, which is offered at most its max in the first place and keeps what it is
//!   answered (`hug_limit`) — and, where the height is read, when the room offered did not hold the
//!   answer (a word longer than the line: "aa aa bbbbb" offered 30 px is three lines 50 wide, and
//!   at 50 two);
//! - with an aspect ratio, the axis the content did not fix follows the ratio;
//! - a measured leaf's children are not laid out: they are hidden (0 × 0).

use super::model::BoxModel;
use super::{finish, held, hide, measure, Input, Mode, Output, Reads};
use crate::{Available, MeasureInput, Measurer, NodeId, Size, Style, Tree};

/// The node's own known border box: what the parent fixed, else its definite style size, with the
/// aspect ratio filling the other axis.
fn known(model: &BoxModel, input: &Input) -> Size<Option<f32>> {
    model.with_aspect(Size::new(input.known().width.or(model.size.width), input.known().height.or(model.size.height)))
}

/// The content width a HUGGING leaf may take at most: its own `max-width` — read even where a
/// content question sets its size properties aside (a row's flex base size, a contribution), since
/// for a leaf that hugs the max is a limit on the offer, not a width to fill (`flex::content_width`).
fn hug_limit(style: &Style, input: &Input, model: &BoxModel) -> Option<f32> {
    let max = style.max_size.width.px(input.parent.width)?.max(0.0);
    Some(if model.border_box { (max - model.pb().width).max(0.0) } else { max })
}

pub(super) fn measured(tree: &mut Tree, node: NodeId, input: &Input, measurer: &mut dyn Measurer) -> Output {
    let style = tree.styles[node.slot()].computed();
    let model = BoxModel::resolve(style, input.parent, input.content);
    let pb = model.pb();
    let known = known(&model, input);
    let hug_limit = if tree.meta[node.slot()].hug { hug_limit(style, input, &model) } else { None };

    let content_top = model.border.top + model.padding.top;
    let (size, baseline) = match (known.width, known.height) {
        // The parent fixed the box: nothing to measure, unless it reads the baseline. That is the
        // content's at this width — an answer there, or the Measurer asked at the known size — so
        // it does not hang on what the leaf happened to be asked before.
        (Some(width), Some(height)) => {
            let baseline = if input.reads == Reads::Baseline {
                let content = Size::new((width - pb.width).max(0.0), (height - pb.height).max(0.0));
                match tree.cache[node.slot()].answer_at(content.width) {
                    Some(answer) => answer.baseline,
                    None => {
                        let question = MeasureInput {
                            known: Size::new(Some(content.width), Some(content.height)),
                            available: Size::new(Available::Definite(content.width), Available::Definite(content.height)),
                        };
                        measure(tree, node, question, measurer).baseline
                    }
                }
            } else {
                None
            };
            (Size::new(width, height), baseline.map(|baseline| content_top + baseline))
        }
        _ => {
            let content_known =
                Size::new(known.width.map(|w| (w - pb.width).max(0.0)), known.height.map(|h| (h - pb.height).max(0.0)));
            let offered = |horizontal: bool| match content_known.get(horizontal) {
                Some(px) => Available::Definite(px),
                None => input.available().get(horizontal).shrink(model.margin.sum(horizontal) + pb.get(horizontal)),
            };
            let mut available = Size::new(offered(true), offered(false));
            if let (None, Some(limit)) = (content_known.width, hug_limit) {
                available.width = match available.width {
                    Available::Definite(offer) => Available::Definite(offer.min(limit)),
                    Available::MaxContent => Available::Definite(limit),
                    Available::MinContent => Available::MinContent,
                };
            }
            let mut out = measure(tree, node, MeasureInput { known: content_known, available }, measurer);

            let width = known.width.unwrap_or_else(|| model.clamp(true, out.size.width + pb.width));
            let cut = width < out.size.width + pb.width;
            let overflowed = input.reads != Reads::Width && !held(available.width, out.size.width);
            if known.width.is_none() && known.height.is_none() && (cut || overflowed) {
                let at = (width - pb.width).max(0.0);
                let question = MeasureInput {
                    known: Size::new(Some(at), None),
                    available: Size::new(Available::Definite(at), available.height),
                };
                out = measure(tree, node, question, measurer);
            }
            let height = known.height.unwrap_or_else(|| match model.aspect {
                Some(_) => model.clamp(false, model.transfer(true, width)),
                None => model.clamp(false, out.size.height + pb.height),
            });
            (Size::new(width, height), out.baseline.map(|baseline| content_top + baseline))
        }
    };

    if input.mode == Mode::Layout {
        let mut child = tree.links[node.slot()].first_child;
        while let Some(id) = child {
            hide(tree, id);
            child = tree.links[id.slot()].next;
        }
        finish(tree, node, &model, size, Size::new(0.0, 0.0));
    }
    Output { size, baseline }
}

pub(super) fn empty(tree: &mut Tree, node: NodeId, input: &Input) -> Output {
    let model = BoxModel::resolve(tree.styles[node.slot()].computed(), input.parent, input.content);
    let pb = model.pb();
    let known = known(&model, input);
    let width = known.width.unwrap_or_else(|| model.clamp(true, pb.width));
    let height = known.height.unwrap_or_else(|| match model.aspect {
        Some(_) => model.clamp(false, model.transfer(true, width)),
        None => model.clamp(false, pb.height),
    });
    let size = Size::new(width, height);
    if input.mode == Mode::Layout {
        finish(tree, node, &model, size, Size::new(0.0, 0.0));
    }
    Output { size, baseline: None }
}
