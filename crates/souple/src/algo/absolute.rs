//! The absolutely positioned children of a flex container: out of flow, laid out after the flex
//! items, once the container's own box is final.
//!
//! The rules (css-position-3, css-flexbox-1 §4.1):
//! - the containing block is the parent's PADDING box: insets are measured from its edges, percents
//!   (sizes and insets) are of its size, margin and padding percents of its width;
//! - a size left auto is fixed by the two insets when both are set; otherwise it is the content's,
//!   shrink-to-fit in the space the set insets leave;
//! - with both insets and the size fixed, `auto` margins share what is left (centring); an inline
//!   (horizontal) pair never goes negative at the start;
//! - an axis whose insets are both auto takes the STATIC position: where the box would be as the
//!   container's sole flex item — `justify-content` on the main axis, `align-self` on the cross axis
//!   (stretch and baseline as flex-start, the space distributions as their single-item fallbacks).

use super::axis::Axes;
use super::axis::Side::{Bottom, Left, Right, Top};
use super::model::BoxModel;
use super::{ask, compute, Input, Mode, Reads};
use crate::{
    AlignItems, Available, Display, Edges, JustifyContent, Tree, Measurer, NodeId, Position, Size,
};

/// What the absolute children need of their container.
pub(super) struct Frame {
    /// The container's border box.
    pub size: Size<f32>,
    pub border: Edges<f32>,
    pub padding: Edges<f32>,
    pub axes: Axes,
    pub justify: JustifyContent,
    pub align_items: AlignItems,
}

/// Lays out and places every absolute child of `node`; grows `extent` (the far edge of the
/// children's margin boxes, from the padding box's origin) by each.
pub(super) fn layout_children(
    tree: &mut Tree,
    node: NodeId,
    frame: &Frame,
    extent: &mut Size<f32>,
    measurer: &mut dyn Measurer,
) {
    let mut child = tree.links[node.slot()].first_child;
    while let Some(id) = child {
        child = tree.links[id.slot()].next;
        let style = &tree.styles[id.slot()].computed();
        if style.position == Position::Absolute && style.display != Display::None {
            place(tree, id, frame, extent, measurer);
        }
    }
}

fn place(tree: &mut Tree, node: NodeId, frame: &Frame, extent: &mut Size<f32>, measurer: &mut dyn Measurer) {
    let slot = node.slot();
    let padding_box = Size::new(
        (frame.size.width - frame.border.horizontal()).max(0.0),
        (frame.size.height - frame.border.vertical()).max(0.0),
    );
    let block = Size::new(Some(padding_box.width), Some(padding_box.height));
    let style = &tree.styles[slot].computed();
    let model = BoxModel::resolve(style, block, None);
    let inset = Edges {
        left: style.inset.left.px(block.width),
        right: style.inset.right.px(block.width),
        top: style.inset.top.px(block.height),
        bottom: style.inset.bottom.px(block.height),
    };
    let align = match style.align_self.unwrap_or(frame.align_items) {
        AlignItems::Stretch | AlignItems::Baseline => AlignItems::FlexStart,
        other => other,
    };

    // The size: the style's, else the insets', else the content's in the space the insets leave.
    let from_insets = |horizontal: bool| {
        let (start, end) = if horizontal { (inset.left, inset.right) } else { (inset.top, inset.bottom) };
        let space = padding_box.get(horizontal) - start? - end? - model.margin.sum(horizontal);
        Some(model.clamp(horizontal, space))
    };
    let known = model.with_aspect(Size::new(
        model.size.width.or_else(|| from_insets(true)),
        model.size.height.or_else(|| from_insets(false)),
    ));
    let room = |horizontal: bool| {
        let (start, end) = if horizontal { (inset.left, inset.right) } else { (inset.top, inset.bottom) };
        Available::Definite((padding_box.get(horizontal) - start.unwrap_or(0.0) - end.unwrap_or(0.0)).max(0.0))
    };
    let input = Input {
        ask: ask(known, Size::new(room(true), room(false))),
        parent: block,
        mode: Mode::Layout,
        content: None,
        reads: Reads::Size,
        // A height from the style or the insets is definite; one left to the content is not
        // (`compute` sizes it first and marks it so).
        height_definite: true,
    };
    let size = compute(tree, node, input, measurer).size;

    let mut margin = model.margin;
    let position = |horizontal: bool, margin: &mut Edges<f32>| -> f32 {
        let (start, end) = if horizontal { (inset.left, inset.right) } else { (inset.top, inset.bottom) };
        let (start_side, end_side) = if horizontal { (Left, Right) } else { (Top, Bottom) };
        let border_start = if horizontal { frame.border.left } else { frame.border.top };
        let length = size.get(horizontal);
        match (start, end) {
            (Some(start), Some(end)) => {
                let free = padding_box.get(horizontal) - start - end - length - margin.sum(horizontal);
                let auto = (model.margin_auto.get(start_side), model.margin_auto.get(end_side));
                match auto {
                    (true, true) => {
                        let half = if horizontal && free < 0.0 { 0.0 } else { free / 2.0 };
                        margin.set(start_side, half);
                        margin.set(end_side, free - half);
                    }
                    (true, false) => margin.set(start_side, free),
                    (false, true) => margin.set(end_side, free),
                    (false, false) => {}
                }
                border_start + start + margin.get(start_side)
            }
            (Some(start), None) => border_start + start + margin.get(start_side),
            (None, Some(end)) => border_start + padding_box.get(horizontal) - end - margin.get(end_side) - length,
            (None, None) => static_position(frame, horizontal, length, margin, align),
        }
    };
    let x = position(true, &mut margin);
    let y = position(false, &mut margin);

    let rect = &mut tree.raw[slot].rect;
    rect.x = x;
    rect.y = y;
    extent.width = extent.width.max(x - frame.border.left + size.width + margin.right);
    extent.height = extent.height.max(y - frame.border.top + size.height + margin.bottom);
}

/// The border-box origin on one axis of a box placed as the container's sole flex item: in the
/// content box, by `justify-content` on the main axis and `align` on the cross axis.
fn static_position(frame: &Frame, horizontal: bool, length: f32, margin: &Edges<f32>, align: AlignItems) -> f32 {
    let axes = frame.axes;
    let main = horizontal == axes.row;
    let pb = frame.padding.add(&frame.border);
    let content = (frame.size.get(horizontal) - pb.sum(horizontal)).max(0.0);
    let outer = length + margin.sum(horizontal);
    let free = content - outer;
    let reverse = if main { axes.main_reverse } else { axes.cross_reverse };
    let share = if main {
        match frame.justify {
            JustifyContent::FlexStart | JustifyContent::SpaceBetween => 0.0,
            JustifyContent::FlexEnd => 1.0,
            JustifyContent::Center | JustifyContent::SpaceAround | JustifyContent::SpaceEvenly => 0.5,
        }
    } else {
        match align {
            AlignItems::FlexEnd => 1.0,
            AlignItems::Center => 0.5,
            _ => 0.0,
        }
    };
    // From the logical start edge to the margin box, then the physical flip of a reversed axis.
    let logical = free * share;
    let margin_box = if reverse { content - logical - outer } else { logical };
    let (content_start, margin_before) = if horizontal { (pb.left, margin.left) } else { (pb.top, margin.top) };
    content_start + margin_box + margin_before
}
