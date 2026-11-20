//! Main and cross: the flex algorithm is written once, along abstract axes, and these helpers map
//! them onto width / height and the four physical edges.
//!
//! The algorithm places everything in LOGICAL coordinates — distances from the main-start and
//! cross-start edges of the content box — and turns them physical once, at the end. A reversed
//! axis (`row-reverse`, `column-reverse`, `wrap-reverse`) is only that last flip plus the edges
//! below; no step of the algorithm has a reversed branch of its own. LTR and horizontal writing
//! only: a row's main-start is the left edge, a column's cross-start the left edge too.

use crate::{Available, Edges, FlexDirection, FlexWrap, Size};

/// One physical axis.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Axis {
    Horizontal,
    Vertical,
}

impl Axis {
    pub fn of(horizontal: bool) -> Self {
        if horizontal { Axis::Horizontal } else { Axis::Vertical }
    }

    pub fn is_horizontal(self) -> bool {
        self == Axis::Horizontal
    }
}

/// One physical edge of a box.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Side {
    Left,
    Top,
    Right,
    Bottom,
}

/// The flex container's axes.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Axes {
    /// The main axis is horizontal.
    pub row: bool,
    /// `row-reverse` / `column-reverse`: main-start is the right / bottom edge.
    pub main_reverse: bool,
    /// `wrap-reverse`: cross-start is the bottom / right edge.
    pub cross_reverse: bool,
}

impl Axes {
    pub fn new(direction: FlexDirection, wrap: FlexWrap) -> Self {
        Self {
            row: matches!(direction, FlexDirection::Row | FlexDirection::RowReverse),
            main_reverse: matches!(direction, FlexDirection::RowReverse | FlexDirection::ColumnReverse),
            cross_reverse: wrap == FlexWrap::WrapReverse,
        }
    }

    pub fn main_start(self) -> Side {
        match (self.row, self.main_reverse) {
            (true, false) => Side::Left,
            (true, true) => Side::Right,
            (false, false) => Side::Top,
            (false, true) => Side::Bottom,
        }
    }

    pub fn main_end(self) -> Side {
        self.main_start().opposite()
    }

    pub fn cross_start(self) -> Side {
        match (self.row, self.cross_reverse) {
            (true, false) => Side::Top,
            (true, true) => Side::Bottom,
            (false, false) => Side::Left,
            (false, true) => Side::Right,
        }
    }

    pub fn cross_end(self) -> Side {
        self.cross_start().opposite()
    }
}

impl Side {
    fn opposite(self) -> Self {
        match self {
            Side::Left => Side::Right,
            Side::Right => Side::Left,
            Side::Top => Side::Bottom,
            Side::Bottom => Side::Top,
        }
    }
}

impl<T: Copy> Size<T> {
    /// The value on the main axis of a container whose main axis is horizontal when `row`.
    pub(crate) fn main(self, row: bool) -> T {
        if row { self.width } else { self.height }
    }

    pub(crate) fn cross(self, row: bool) -> T {
        if row { self.height } else { self.width }
    }

    pub(crate) fn from_axes(row: bool, main: T, cross: T) -> Self {
        if row { Size::new(main, cross) } else { Size::new(cross, main) }
    }

    /// The value on one physical axis.
    pub(crate) fn get(self, horizontal: bool) -> T {
        if horizontal { self.width } else { self.height }
    }
}

impl<T: Copy> Edges<T> {
    pub(crate) fn get(&self, side: Side) -> T {
        match side {
            Side::Left => self.left,
            Side::Top => self.top,
            Side::Right => self.right,
            Side::Bottom => self.bottom,
        }
    }

    pub(crate) fn set(&mut self, side: Side, value: T) {
        match side {
            Side::Left => self.left = value,
            Side::Top => self.top = value,
            Side::Right => self.right = value,
            Side::Bottom => self.bottom = value,
        }
    }
}

impl Edges<f32> {
    pub(crate) fn horizontal(&self) -> f32 {
        self.left + self.right
    }

    pub(crate) fn vertical(&self) -> f32 {
        self.top + self.bottom
    }

    /// The two edges of one physical axis, summed.
    pub(crate) fn sum(&self, horizontal: bool) -> f32 {
        if horizontal { self.horizontal() } else { self.vertical() }
    }

    pub(crate) fn add(&self, other: &Edges<f32>) -> Edges<f32> {
        Edges {
            left: self.left + other.left,
            top: self.top + other.top,
            right: self.right + other.right,
            bottom: self.bottom + other.bottom,
        }
    }
}

impl Available {
    /// The space left once `px` of it is taken (a margin, a padding): a min- or max-content
    /// constraint stays what it is.
    pub(crate) fn shrink(self, px: f32) -> Self {
        match self {
            Available::Definite(space) => Available::Definite((space - px).max(0.0)),
            other => other,
        }
    }
}
