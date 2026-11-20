//! What went wrong in a value, and where.
//!
//! One error type for every stage: the tokenizer, the length grammar, the declarations. The kind is
//! an enum so an embedder can act on it (skip an [`ErrorKind::UnknownProperty`] it routes
//! elsewhere) and a test can name it; the offset points into the value text, so a message can show
//! the author the spot.

use std::fmt;

use souple::{Formula, FormulaError};

/// A value (or a property) that could not be read, and where.
#[derive(Clone, Debug, PartialEq)]
pub struct ParseError {
    /// Where in the value text, in bytes. 0 for an unknown property: the name is in the kind.
    pub offset: usize,
    /// What went wrong.
    pub kind: ErrorKind,
}

/// What went wrong in a value.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum ErrorKind {
    /// A character no value starts with: `#`, `;`, a `.` without digits after it.
    UnexpectedChar(char),
    /// The text ends where more was needed: an empty value, `calc(1px`, `calc(1px +`.
    UnexpectedEnd,
    /// The token at the offset is not what the grammar wants there; the text says what would be.
    Expected(String),
    /// A number too large for an `f32`.
    InvalidNumber,
    /// A number's suffix that is neither `px` nor a unit of the [`Units`](crate::Units) table.
    UnknownUnit(String),
    /// A name that is neither `auto` nor a keyword of the [`Units`](crate::Units) table.
    UnknownKeyword(String),
    /// A function other than `calc`, `min`, `max`, `clamp`.
    UnknownFunction(String),
    /// A property that is not a field of `Style` (a paint property, a typo), as it was named.
    UnknownProperty(String),
    /// A `+` or `-` between two values without whitespace on both sides: `1px+2px`, `1px -2px`.
    OperatorSpacing,
    /// `clamp()` with other than 3 arguments, `min()` / `max()` with more than 255. The function.
    ArgumentCount(&'static str),
    /// A negative value for a property that takes none: `width: -1px`, `flex-grow: -1`.
    Negative,
    /// [`Formula::build`] refused the formula: mixed types, a division by zero, too deep.
    Formula(FormulaError),
}

impl ParseError {
    pub(crate) fn new(offset: usize, kind: ErrorKind) -> Self {
        Self { offset, kind }
    }

    pub(crate) fn expected(offset: usize, what: impl Into<String>) -> Self {
        Self::new(offset, ErrorKind::Expected(what.into()))
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::UnknownProperty(_) => write!(f, "{}", self.kind),
            kind => write!(f, "{kind} at byte {}", self.offset),
        }
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedChar(char) => write!(f, "unexpected character `{char}`"),
            Self::UnexpectedEnd => write!(f, "the value ends too early"),
            Self::Expected(what) => write!(f, "expected {what}"),
            Self::InvalidNumber => write!(f, "the number is out of range"),
            Self::UnknownUnit(unit) => write!(f, "unknown unit `{unit}`"),
            Self::UnknownKeyword(name) => write!(f, "unknown keyword `{name}`"),
            Self::UnknownFunction(name) => write!(f, "unknown function `{name}()`"),
            Self::UnknownProperty(name) => write!(f, "unknown property `{name}`"),
            Self::OperatorSpacing => write!(f, "`+` and `-` need whitespace on both sides"),
            Self::ArgumentCount("clamp") => write!(f, "`clamp()` takes 3 arguments"),
            Self::ArgumentCount(function) => write!(f, "`{function}()` takes 1 to 255 arguments"),
            Self::Negative => write!(f, "a negative value is not allowed here"),
            Self::Formula(FormulaError::Type) => write!(
                f,
                "mismatched types: `+`, `-`, `min()`… join lengths, `*` needs a number on one side, \
                 `/` a number on the right, and the result is a length"
            ),
            Self::Formula(FormulaError::DivisionByZero) => write!(f, "division by zero"),
            Self::Formula(FormulaError::TooDeep) => {
                write!(f, "the formula is nested too deeply (the limit is {})", Formula::MAX_DEPTH)
            }
            Self::Formula(FormulaError::Unbalanced) => write!(f, "the formula is unbalanced"),
            // A reason a newer `souple` adds (the enum is non-exhaustive).
            Self::Formula(_) => write!(f, "the formula is not valid"),
        }
    }
}

impl std::error::Error for ParseError {}
