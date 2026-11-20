//! CSS value text for the [`souple`] layout engine.
//!
//! The core (`souple`) knows no strings: a style is typed, a length is px, percent, a
//! variable or a formula. This crate is the front end that turns what an author writes into those
//! values:
//!
//! ```text
//! "calc(100% - 2em)"      token.rs        tokens: numbers with their units, names, operators
//!   │                     length.rs       one length; a math function → postfix Ops → Formula::build
//!   ▼
//! Length::Calc(…)
//!
//! "margin", "4px auto"    declaration.rs  a property name → the Style fields it sets
//! ```
//!
//! What a unit or a keyword like `safe-top` means is the embedder's: it numbers its variables and
//! names them in a [`Units`] table. The type rules of `calc()` are the core's: [`Formula::build`]
//! judges every formula, this crate only reads the grammar.
//!
//! [`Formula::build`]: souple::Formula::build

#![warn(missing_docs)]
#![forbid(unsafe_code)]

mod declaration;
mod error;
mod length;
mod token;
mod units;

pub use declaration::apply_declaration;
pub use error::{ErrorKind, ParseError};
pub use length::parse_length;
pub use units::Units;
