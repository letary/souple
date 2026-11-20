#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![forbid(unsafe_code)]

// The pipeline, one module per stage:
//
//   Style (authored: px, %, variables, formulas)      style.rs, value.rs
//     │  vars.rs — substitute the variables in scope
//     ▼
//   Style (computed: px and % only)
//     │  algo/ — CSS flexbox, in logical px, unrounded
//     ▼
//   NodeLayout (raw)
//     │  tree.rs — snap to the device grid, diff against the last pass
//     ▼
//   NodeLayout (published) + [Change]
//
// What the engine does NOT know: strings, units by name (`em`, `vw`, `safe-top` are variables the
// embedder numbers), a cascade, text. Text and every other content-sized leaf is a Measurer call.

mod algo;
mod edits;
mod geometry;
mod id;
mod measure;
mod style;
mod tree;
mod value;
mod vars;

pub use geometry::{Edges, Rect, Size};
pub use id::NodeId;
pub use measure::{Available, MeasureInput, MeasureOutput, Measurer, NoMeasure};
pub use style::{
    AlignContent, AlignItems, BoxSizing, Display, FlexDirection, FlexWrap, JustifyContent, Overflow,
    Position, Style,
};
pub use tree::{Change, NodeLayout, Tree, Viewport};
pub use value::{Formula, FormulaError, Length, Op, VarId, VarSet, MAX_VARS};
