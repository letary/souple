//! The embedder's units: the spellings the engine leaves to it.
//!
//! The core knows px and percent. `em`, `vw`, `safe-top` are variables the embedder numbers
//! (`VarId`) and defines on its nodes; this table gives them their names, in two forms:
//!
//! - a UNIT is a suffix after a number: with `unit("em", EM)`, `2em` is `Length::Var(EM, 2.0)`;
//! - a KEYWORD stands alone, as one of it: with `keyword("safe-top", SAFE_TOP)`, `safe-top` is
//!   `Length::Var(SAFE_TOP, 1.0)`, and `calc(-1 * safe-top)` its negative.
//!
//! The tables are a handful of entries each, so they are lists searched in order: no hashing, and
//! the ASCII case-insensitive match CSS asks for costs nothing extra.

use souple::VarId;

use crate::token::is_name_text;

/// The names the embedder gives its variables: unit suffixes (`2em`) and bare keywords
/// (`safe-top`), each a `VarId` of its numbering. Built once, read by every parse.
#[derive(Clone, Debug, Default)]
pub struct Units {
    units: Vec<(Box<str>, VarId)>,
    keywords: Vec<(Box<str>, VarId)>,
}

impl Units {
    /// An empty table: `px` and percents only.
    pub fn new() -> Self {
        Self::default()
    }

    /// A unit suffix: `units.unit("em", EM)` makes `2em` = `Length::Var(EM, 2.0)`. Matched
    /// ASCII case-insensitively; defining a suffix again replaces it.
    ///
    /// # Panics
    /// When the suffix could never be read: it is not a CSS name (`2x`, `%`, empty) or it is
    /// `px`, which the core owns.
    pub fn unit(mut self, suffix: &str, var: VarId) -> Self {
        assert!(is_name_text(suffix), "a unit is a CSS name, so the tokenizer reads it after a number: `{suffix}`");
        assert!(!suffix.eq_ignore_ascii_case("px"), "`px` is the core's own unit");
        define(&mut self.units, suffix, var);
        self
    }

    /// A bare keyword that is a length: `units.keyword("safe-top", SAFE_TOP)` makes `safe-top` =
    /// `Length::Var(SAFE_TOP, 1.0)`. Matched ASCII case-insensitively; defining a name again
    /// replaces it.
    ///
    /// # Panics
    /// When the name could never be read: it is not a CSS name, or it is `auto`, which every
    /// length property owns. (`none` and `normal` are taken only where a property means them: a
    /// keyword with that name is shadowed in `max-*` and the gaps.)
    pub fn keyword(mut self, name: &str, var: VarId) -> Self {
        assert!(is_name_text(name), "a keyword is a CSS name: `{name}`");
        assert!(!name.eq_ignore_ascii_case("auto"), "`auto` is every length property's own keyword");
        define(&mut self.keywords, name, var);
        self
    }

    pub(crate) fn find_unit(&self, suffix: &str) -> Option<VarId> {
        find(&self.units, suffix)
    }

    pub(crate) fn find_keyword(&self, name: &str) -> Option<VarId> {
        find(&self.keywords, name)
    }
}

fn define(table: &mut Vec<(Box<str>, VarId)>, name: &str, var: VarId) {
    match table.iter_mut().find(|(defined, _)| defined.eq_ignore_ascii_case(name)) {
        Some(entry) => entry.1 = var,
        None => table.push((name.into(), var)),
    }
}

fn find(table: &[(Box<str>, VarId)], name: &str) -> Option<VarId> {
    table.iter().find(|(defined, _)| defined.eq_ignore_ascii_case(name)).map(|&(_, var)| var)
}
