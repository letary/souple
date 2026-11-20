//! The style of a node: plain data, CSS flexbox names, CSS initial values.
//!
//! An embedder with other defaults (a column, no shrink) writes them itself.

use crate::{Edges, Length, Size, VarId, VarSet};

/// CSS `display`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum Display {
    /// A flex container (and a flex item of its parent). There is no other box kind.
    #[default]
    Flex,
    /// No box: the node and its subtree take no space and are laid out at 0 × 0.
    None,
}

/// CSS `position`. There is no `static`: every box is positioned (the README's "Limits").
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum Position {
    /// In flow; `inset` shifts the box without moving anything else.
    #[default]
    Relative,
    /// Out of flow, placed by `inset` against the parent's padding box.
    Absolute,
}

/// CSS `flex-direction`: the main axis, and which way it runs.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FlexDirection {
    /// Left to right.
    #[default]
    Row,
    /// Top to bottom.
    Column,
    /// Right to left.
    RowReverse,
    /// Bottom to top.
    ColumnReverse,
}

/// CSS `flex-wrap`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FlexWrap {
    /// One line, whatever overflows.
    #[default]
    NoWrap,
    /// A new line before an item that would overflow the main size; lines stack cross-start to
    /// cross-end.
    Wrap,
    /// As `Wrap`, the lines stacked the other way.
    WrapReverse,
}

/// CSS `justify-content`: how a line's free main space is shared out.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum JustifyContent {
    /// Packed at the main start.
    #[default]
    FlexStart,
    /// Packed at the main end.
    FlexEnd,
    /// Packed in the middle.
    Center,
    /// The space between the items; at the start when there is none to share (one item, an
    /// overflow).
    SpaceBetween,
    /// Half a share at each end, a share between items; centred while there is room.
    SpaceAround,
    /// Equal space at the ends and between items; centred while there is room.
    SpaceEvenly,
}

/// CSS `align-items` (and, as `Some`, `align-self`): where an item sits across its line.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum AlignItems {
    /// As tall (or wide) as the line, if its cross size is auto.
    #[default]
    Stretch,
    /// At the line's cross start.
    FlexStart,
    /// At the line's cross end.
    FlexEnd,
    /// In the middle of the line.
    Center,
    /// The first baselines lined up (in a row; elsewhere `FlexStart`).
    Baseline,
}

/// CSS `align-content`: how a multi-line container's lines share its free cross space.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum AlignContent {
    /// The lines grow to fill it.
    #[default]
    Stretch,
    /// Packed at the cross start.
    FlexStart,
    /// Packed at the cross end.
    FlexEnd,
    /// Packed in the middle.
    Center,
    /// The space between the lines.
    SpaceBetween,
    /// Half a share at each end, a share between lines.
    SpaceAround,
    /// Equal space at the ends and between lines.
    SpaceEvenly,
}

/// CSS `box-sizing`: which box `size`, `min_size`, `max_size` and `aspect_ratio` measure.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BoxSizing {
    /// `width` / `height` size the content box.
    #[default]
    ContentBox,
    /// `width` / `height` size the border box.
    BorderBox,
}

/// CSS `overflow`. The engine clips and scrolls nothing itself; this is what layout reads of it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum Overflow {
    /// The content may show past the box.
    #[default]
    Visible,
    /// Clipped. As in CSS, anything but `Visible` makes the automatic minimum size 0.
    Hidden,
    /// A scroll container: its `content_size` is what scrolls.
    Scroll,
}

/// Everything the layout reads of a node, under CSS's names, with CSS's initial values as the
/// [`Default`]. Plain data: a field written is a field read at the next pass
/// ([`crate::Tree::set_style`], [`crate::Tree::update_style`]).
///
/// Any number is accepted: a negative size, padding, border or gap is 0, a negative flex factor 0,
/// an aspect ratio that is not a positive finite number none; NaN and infinities lay out to
/// something finite (the README's "Hostile input").
#[derive(Clone, Debug, PartialEq)]
pub struct Style {
    /// `display`.
    pub display: Display,
    /// `position`.
    pub position: Position,
    /// `box-sizing`.
    pub box_sizing: BoxSizing,
    /// `overflow`.
    pub overflow: Overflow,

    /// `flex-direction`.
    pub flex_direction: FlexDirection,
    /// `flex-wrap`.
    pub flex_wrap: FlexWrap,
    /// `justify-content`.
    pub justify_content: JustifyContent,
    /// `align-items`: where the children sit across their line.
    pub align_items: AlignItems,
    /// `align-self`. `None` = `auto`: the parent's `align_items`.
    pub align_self: Option<AlignItems>,
    /// `align-content`.
    pub align_content: AlignContent,

    /// `flex-grow`: the share of a line's free space the item takes.
    pub flex_grow: f32,
    /// `flex-shrink`: the share of a line's overflow the item gives up, by its base size.
    pub flex_shrink: f32,
    /// `flex-basis`. `Auto` = the main size property.
    pub flex_basis: Length,

    /// `width` / `height`. `Auto` = sized by the content and the flex rules.
    pub size: Size<Length>,
    /// `min-width` / `min-height`. `Auto` = the automatic minimum of a flex item (its content, 0
    /// for a scroll container).
    pub min_size: Size<Length>,
    /// `max-width` / `max-height`. `Auto` = no maximum.
    pub max_size: Size<Length>,
    /// `aspect-ratio`: width / height. `None` = auto.
    pub aspect_ratio: Option<f32>,

    /// `margin-*`. `Auto` = an auto margin: it takes the free space. A percent is of the
    /// containing block's width, on all four edges.
    pub margin: Edges<Length>,
    /// `padding-*`. A percent is of the containing block's width, on all four edges.
    pub padding: Edges<Length>,
    /// `border-*-width`: space between the margin and the padding, nothing more.
    pub border: Edges<Length>,
    /// `left` / `top` / `right` / `bottom`. `Auto` = that edge does not place the box.
    pub inset: Edges<Length>,

    /// `row-gap`: between the lines of a row, the items of a column.
    pub row_gap: Length,
    /// `column-gap`: between the items of a row, the lines of a column.
    pub column_gap: Length,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            display: Display::Flex,
            position: Position::Relative,
            box_sizing: BoxSizing::ContentBox,
            overflow: Overflow::Visible,
            flex_direction: FlexDirection::Row,
            flex_wrap: FlexWrap::NoWrap,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Stretch,
            align_self: None,
            align_content: AlignContent::Stretch,
            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_basis: Length::Auto,
            size: Size::new(Length::Auto, Length::Auto),
            min_size: Size::new(Length::Auto, Length::Auto),
            max_size: Size::new(Length::Auto, Length::Auto),
            aspect_ratio: None,
            margin: Edges::all(Length::ZERO),
            padding: Edges::all(Length::ZERO),
            border: Edges::all(Length::ZERO),
            inset: Edges::all(Length::Auto),
            row_gap: Length::ZERO,
            column_gap: Length::ZERO,
        }
    }
}

impl Style {
    /// Every length of the style — THE list: a length field added to `Style` is added here and in
    /// `lengths_mut`, and the variables reach it.
    fn lengths(&self) -> [&Length; 25] {
        let Self { size, min_size, max_size, margin, padding, border, inset, .. } = self;
        [
            &self.flex_basis, &self.row_gap, &self.column_gap,
            &size.width, &size.height,
            &min_size.width, &min_size.height,
            &max_size.width, &max_size.height,
            &margin.left, &margin.top, &margin.right, &margin.bottom,
            &padding.left, &padding.top, &padding.right, &padding.bottom,
            &border.left, &border.top, &border.right, &border.bottom,
            &inset.left, &inset.top, &inset.right, &inset.bottom,
        ]
    }

    fn lengths_mut(&mut self) -> [&mut Length; 25] {
        let Self { flex_basis, row_gap, column_gap, size, min_size, max_size, margin, padding, border, inset, .. } = self;
        [
            flex_basis, row_gap, column_gap,
            &mut size.width, &mut size.height,
            &mut min_size.width, &mut min_size.height,
            &mut max_size.width, &mut max_size.height,
            &mut margin.left, &mut margin.top, &mut margin.right, &mut margin.bottom,
            &mut padding.left, &mut padding.top, &mut padding.right, &mut padding.bottom,
            &mut border.left, &mut border.top, &mut border.right, &mut border.bottom,
            &mut inset.left, &mut inset.top, &mut inset.right, &mut inset.bottom,
        ]
    }

    /// The variables the style reads.
    pub fn vars(&self) -> VarSet {
        self.lengths().into_iter().fold(VarSet::EMPTY, |set, length| set.union(length.vars()))
    }

    /// The computed style: the same with every variable replaced by its value (what the engine
    /// does with the values in scope; for an embedder that wants a style's computed form).
    pub fn substitute(&self, value_of: &impl Fn(VarId) -> f32) -> Self {
        let mut computed = self.clone();
        for length in computed.lengths_mut() {
            if !length.vars().is_empty() {
                *length = length.substitute(value_of);
            }
        }
        computed
    }
}
