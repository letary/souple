/// A value per axis.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Size<T> {
    /// The horizontal axis.
    pub width: T,
    /// The vertical axis.
    pub height: T,
}

impl<T> Size<T> {
    /// A size of these two values.
    pub const fn new(width: T, height: T) -> Self {
        Self { width, height }
    }
}

/// A value per physical edge.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Edges<T> {
    /// The left edge.
    pub left: T,
    /// The top edge.
    pub top: T,
    /// The right edge.
    pub right: T,
    /// The bottom edge.
    pub bottom: T,
}

impl<T: Clone> Edges<T> {
    /// The same value on all four edges.
    pub fn all(value: T) -> Self {
        Self { left: value.clone(), top: value.clone(), right: value.clone(), bottom: value }
    }
}

/// A box: the origin relative to the parent's border box, and the size of the border box.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Rect {
    /// The left edge, from the parent's left border edge.
    pub x: f32,
    /// The top edge, from the parent's top border edge.
    pub y: f32,
    /// The width of the border box.
    pub width: f32,
    /// The height of the border box.
    pub height: f32,
}
