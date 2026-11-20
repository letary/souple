//! A node's box model, resolved: margins, padding, border, and the size properties as BORDER-box
//! px. Both the node (laying itself out) and its parent (placing it as an item) resolve it, from
//! the same computed style and the same containing block, so they always agree.
//!
//! The CSS rules this file owns:
//! - every margin and padding percent — all four edges — is of the containing block's WIDTH;
//! - a percent size, min or max is of the containing block's size on its own axis, and of an
//!   indefinite size it is `auto` (`Length::resolve(None)` answers `None`);
//! - `box-sizing` decides which box `size`, `min_size`, `max_size` and `aspect_ratio` measure; past
//!   this file everything is a border box;
//! - a used size is clamped by max, then by min (the minimum wins a conflict), and is never smaller
//!   than the padding and border;
//! - `aspect_ratio` fills an axis the box does not know from the one it does.

use super::axis::Axis;
use crate::{BoxSizing, Edges, Length, Position, Size, Style};

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct BoxModel {
    /// The margins; an `auto` one is 0 here and flagged in `margin_auto` — the flex algorithm
    /// gives it its share of the free space.
    pub margin: Edges<f32>,
    pub margin_auto: Edges<bool>,
    pub padding: Edges<f32>,
    pub border: Edges<f32>,
    /// The `size` property as a border box; `None` = auto (or a percent of an indefinite size).
    pub size: Size<Option<f32>>,
    /// `None` = no minimum (an `auto` minimum is the flex parent's business, not the node's).
    pub min: Size<Option<f32>>,
    pub max: Size<Option<f32>>,
    /// width / height of the box `box-sizing` names.
    pub aspect: Option<f32>,
    pub border_box: bool,
}

impl BoxModel {
    /// `parent` is the containing block: the percent base. `content` drops the size, min and max
    /// properties on that axis — a question about what the content alone asks for there.
    pub fn resolve(style: &Style, parent: Size<Option<f32>>, content: Option<Axis>) -> Self {
        let base = parent.width;
        let edges = |lengths: &Edges<Length>| Edges {
            left: lengths.left.px_or_zero(base).max(0.0),
            top: lengths.top.px_or_zero(base).max(0.0),
            right: lengths.right.px_or_zero(base).max(0.0),
            bottom: lengths.bottom.px_or_zero(base).max(0.0),
        };
        let padding = edges(&style.padding);
        let border = edges(&style.border);
        let margin = Edges {
            left: style.margin.left.px_or_zero(base),
            top: style.margin.top.px_or_zero(base),
            right: style.margin.right.px_or_zero(base),
            bottom: style.margin.bottom.px_or_zero(base),
        };
        let margin_auto = Edges {
            left: style.margin.left.is_auto(),
            top: style.margin.top.is_auto(),
            right: style.margin.right.is_auto(),
            bottom: style.margin.bottom.is_auto(),
        };
        let border_box = style.box_sizing == BoxSizing::BorderBox;
        let pb = Size::new(padding.horizontal() + border.horizontal(), padding.vertical() + border.vertical());
        let to_border_box = |length: &Length, axis: Axis| {
            if content == Some(axis) {
                return None;
            }
            let horizontal = axis.is_horizontal();
            // A formula may compute below zero; CSS clamps a size at 0.
            let px = length.px(parent.get(horizontal))?.max(0.0);
            Some(if border_box { px } else { px + pb.get(horizontal) })
        };
        let pair = |lengths: &Size<Length>| {
            Size::new(to_border_box(&lengths.width, Axis::Horizontal), to_border_box(&lengths.height, Axis::Vertical))
        };

        let mut model = Self {
            margin,
            margin_auto,
            padding,
            border,
            size: pair(&style.size),
            min: pair(&style.min_size),
            max: pair(&style.max_size),
            aspect: style.aspect_ratio.filter(|ratio| *ratio > 0.0 && ratio.is_finite()),
            border_box,
        };
        model.size.width = model.size.width.map(|width| model.clamp(true, width));
        model.size.height = model.size.height.map(|height| model.clamp(false, height));
        model.size = model.with_aspect(model.size);
        model
    }

    /// The padding plus the border on each axis.
    pub fn pb(&self) -> Size<f32> {
        Size::new(
            self.padding.horizontal() + self.border.horizontal(),
            self.padding.vertical() + self.border.vertical(),
        )
    }

    pub fn padding_border(&self) -> Edges<f32> {
        self.padding.add(&self.border)
    }

    /// A border-box size on one axis, within the node's min and max.
    pub fn clamp(&self, horizontal: bool, value: f32) -> f32 {
        let mut value = value;
        if let Some(max) = self.max.get(horizontal) {
            value = value.min(max);
        }
        if let Some(min) = self.min.get(horizontal) {
            value = value.max(min);
        }
        value.max(self.pb().get(horizontal))
    }

    /// `known` with an axis it lacks filled from the other through the aspect ratio.
    pub fn with_aspect(&self, known: Size<Option<f32>>) -> Size<Option<f32>> {
        if self.aspect.is_none() {
            return known;
        }
        match (known.width, known.height) {
            (Some(width), None) => Size::new(Some(width), Some(self.clamp(false, self.transfer(true, width)))),
            (None, Some(height)) => Size::new(Some(self.clamp(true, self.transfer(false, height))), Some(height)),
            _ => known,
        }
    }

    /// The border-box size on the other axis for `value` on this one. The ratio holds between the
    /// boxes `box-sizing` names, so a content-box ratio leaves the padding and border out.
    pub fn transfer(&self, from_width: bool, value: f32) -> f32 {
        let Some(ratio) = self.aspect else { return value };
        let pb = self.pb();
        match (self.border_box, from_width) {
            (true, true) => value / ratio,
            (true, false) => value * ratio,
            (false, true) => (value - pb.width).max(0.0) / ratio + pb.height,
            (false, false) => (value - pb.height).max(0.0) * ratio + pb.width,
        }
    }
}

/// The visual shift of a `position: relative` box: `left` wins over `right`, `top` over `bottom`.
/// It moves the box after layout and nothing else; a percent is of the containing block, and of an
/// indefinite one it is no shift.
pub(crate) fn relative_offset(style: &Style, parent: Size<Option<f32>>) -> (f32, f32) {
    if style.position != Position::Relative {
        return (0.0, 0.0);
    }
    let inset = &style.inset;
    let x = inset.left.px(parent.width).or_else(|| inset.right.px(parent.width).map(|right| -right));
    let y = inset.top.px(parent.height).or_else(|| inset.bottom.px(parent.height).map(|bottom| -bottom));
    (x.unwrap_or(0.0), y.unwrap_or(0.0))
}
