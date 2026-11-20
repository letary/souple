//! Declarations: a CSS property and its value text → the `Style` fields they set.
//!
//! The match in [`apply_declaration`] is THE list of properties, one arm each, so a field added to
//! `Style` is added there. Every value is read whole before anything is assigned — the arm's right
//! side runs first, and an error returns from it — so a failed declaration leaves the style as it
//! was.
//!
//! What each length property takes besides a length is its CSS definition ([`LengthKind`]): `auto`
//! for sizes, margins and insets, `none` for a maximum, `normal` for a gap, negative values only
//! for margins and insets. The shorthands follow CSS too: `margin: 1px 2px` is top/bottom 1px and
//! left/right 2px, `flex: 1` is `1 1 0%`, an omitted part of `flex-flow` is its initial value.

use souple::{
    AlignContent, AlignItems, BoxSizing, Display, Edges, FlexDirection, FlexWrap, JustifyContent, Length, Overflow,
    Position, Style,
};

use crate::error::{ErrorKind, ParseError};
use crate::length;
use crate::token::{Cursor, TokenKind};
use crate::units::Units;

/// Applies one CSS declaration to a style: `apply_declaration(&mut style, "flex-direction", "row", &units)`.
///
/// The property is named in CSS kebab-case (ASCII case-insensitive) or in camelCase
/// (`flexDirection`). An unknown property is [`ErrorKind::UnknownProperty`] whatever its value; an
/// unknown property or an invalid value leaves the style untouched.
pub fn apply_declaration(style: &mut Style, property: &str, value: &str, units: &Units) -> Result<(), ParseError> {
    use LengthKind::{Gap, Max, Offset, Size, Spacing};

    let value = Value { text: value, units };
    match css_name(property.trim()).as_str() {
        "display" => style.display = value.read(|r| r.keyword(DISPLAY))?,
        "position" => style.position = value.read(|r| r.keyword(POSITION))?,
        "box-sizing" => style.box_sizing = value.read(|r| r.keyword(BOX_SIZING))?,
        "overflow" => style.overflow = value.read(|r| r.keyword(OVERFLOW))?,
        "flex-direction" => style.flex_direction = value.read(|r| r.keyword(FLEX_DIRECTION))?,
        "flex-wrap" => style.flex_wrap = value.read(|r| r.keyword(FLEX_WRAP))?,
        "justify-content" => style.justify_content = value.read(|r| r.keyword(JUSTIFY_CONTENT))?,
        "align-items" => style.align_items = value.read(|r| r.keyword(ALIGN_ITEMS))?,
        "align-self" => style.align_self = value.read(|r| r.keyword(ALIGN_SELF))?,
        "align-content" => style.align_content = value.read(|r| r.keyword(ALIGN_CONTENT))?,

        "flex-grow" => style.flex_grow = value.read(Reader::factor)?,
        "flex-shrink" => style.flex_shrink = value.read(Reader::factor)?,
        "flex-basis" => style.flex_basis = value.read(|r| r.length(Size))?,
        "aspect-ratio" => style.aspect_ratio = value.read(Reader::aspect_ratio)?,

        "width" => style.size.width = value.read(|r| r.length(Size))?,
        "height" => style.size.height = value.read(|r| r.length(Size))?,
        "min-width" => style.min_size.width = value.read(|r| r.length(Size))?,
        "min-height" => style.min_size.height = value.read(|r| r.length(Size))?,
        "max-width" => style.max_size.width = value.read(|r| r.length(Max))?,
        "max-height" => style.max_size.height = value.read(|r| r.length(Max))?,

        "margin-top" => style.margin.top = value.read(|r| r.length(Offset))?,
        "margin-right" => style.margin.right = value.read(|r| r.length(Offset))?,
        "margin-bottom" => style.margin.bottom = value.read(|r| r.length(Offset))?,
        "margin-left" => style.margin.left = value.read(|r| r.length(Offset))?,
        "padding-top" => style.padding.top = value.read(|r| r.length(Spacing))?,
        "padding-right" => style.padding.right = value.read(|r| r.length(Spacing))?,
        "padding-bottom" => style.padding.bottom = value.read(|r| r.length(Spacing))?,
        "padding-left" => style.padding.left = value.read(|r| r.length(Spacing))?,
        "border-top-width" => style.border.top = value.read(|r| r.length(Spacing))?,
        "border-right-width" => style.border.right = value.read(|r| r.length(Spacing))?,
        "border-bottom-width" => style.border.bottom = value.read(|r| r.length(Spacing))?,
        "border-left-width" => style.border.left = value.read(|r| r.length(Spacing))?,
        "top" => style.inset.top = value.read(|r| r.length(Offset))?,
        "right" => style.inset.right = value.read(|r| r.length(Offset))?,
        "bottom" => style.inset.bottom = value.read(|r| r.length(Offset))?,
        "left" => style.inset.left = value.read(|r| r.length(Offset))?,
        "row-gap" => style.row_gap = value.read(|r| r.length(Gap))?,
        "column-gap" => style.column_gap = value.read(|r| r.length(Gap))?,

        "margin" => style.margin = value.read(|r| r.edges(Offset))?,
        "padding" => style.padding = value.read(|r| r.edges(Spacing))?,
        "border-width" => style.border = value.read(|r| r.edges(Spacing))?,
        "inset" => style.inset = value.read(|r| r.edges(Offset))?,
        "gap" => (style.row_gap, style.column_gap) = value.read(Reader::gap)?,
        "flex" => (style.flex_grow, style.flex_shrink, style.flex_basis) = value.read(Reader::flex)?,
        "flex-flow" => (style.flex_direction, style.flex_wrap) = value.read(Reader::flex_flow)?,

        _ => return Err(ParseError::new(0, ErrorKind::UnknownProperty(property.to_string()))),
    }
    Ok(())
}

/// The CSS name of a property: `flexDirection` → `flex-direction`. A name that starts lowercase and
/// has no hyphen is camelCase, where each capital starts a word; any other is CSS spelling, ASCII
/// case-insensitive (`Flex-Direction`, `WIDTH`).
fn css_name(property: &str) -> String {
    let camel = !property.contains('-') && property.starts_with(|c: char| c.is_ascii_lowercase());
    let mut name = String::with_capacity(property.len() + 4);
    for c in property.chars() {
        if camel && c.is_ascii_uppercase() {
            name.push('-');
        }
        name.push(c.to_ascii_lowercase());
    }
    name
}

const DISPLAY: &[(&str, Display)] = &[("flex", Display::Flex), ("none", Display::None)];

const POSITION: &[(&str, Position)] = &[("relative", Position::Relative), ("absolute", Position::Absolute)];

const BOX_SIZING: &[(&str, BoxSizing)] =
    &[("content-box", BoxSizing::ContentBox), ("border-box", BoxSizing::BorderBox)];

const OVERFLOW: &[(&str, Overflow)] =
    &[("visible", Overflow::Visible), ("hidden", Overflow::Hidden), ("scroll", Overflow::Scroll)];

const FLEX_DIRECTION: &[(&str, FlexDirection)] = &[
    ("row", FlexDirection::Row),
    ("row-reverse", FlexDirection::RowReverse),
    ("column", FlexDirection::Column),
    ("column-reverse", FlexDirection::ColumnReverse),
];

const FLEX_WRAP: &[(&str, FlexWrap)] =
    &[("nowrap", FlexWrap::NoWrap), ("wrap", FlexWrap::Wrap), ("wrap-reverse", FlexWrap::WrapReverse)];

const JUSTIFY_CONTENT: &[(&str, JustifyContent)] = &[
    ("flex-start", JustifyContent::FlexStart),
    ("flex-end", JustifyContent::FlexEnd),
    ("center", JustifyContent::Center),
    ("space-between", JustifyContent::SpaceBetween),
    ("space-around", JustifyContent::SpaceAround),
    ("space-evenly", JustifyContent::SpaceEvenly),
];

const ALIGN_ITEMS: &[(&str, AlignItems)] = &[
    ("stretch", AlignItems::Stretch),
    ("flex-start", AlignItems::FlexStart),
    ("flex-end", AlignItems::FlexEnd),
    ("center", AlignItems::Center),
    ("baseline", AlignItems::Baseline),
];

/// `align-items` and `auto`: the parent's.
const ALIGN_SELF: &[(&str, Option<AlignItems>)] = &[
    ("auto", None),
    ("stretch", Some(AlignItems::Stretch)),
    ("flex-start", Some(AlignItems::FlexStart)),
    ("flex-end", Some(AlignItems::FlexEnd)),
    ("center", Some(AlignItems::Center)),
    ("baseline", Some(AlignItems::Baseline)),
];

const ALIGN_CONTENT: &[(&str, AlignContent)] = &[
    ("stretch", AlignContent::Stretch),
    ("flex-start", AlignContent::FlexStart),
    ("flex-end", AlignContent::FlexEnd),
    ("center", AlignContent::Center),
    ("space-between", AlignContent::SpaceBetween),
    ("space-around", AlignContent::SpaceAround),
    ("space-evenly", AlignContent::SpaceEvenly),
];

/// What a length property takes besides a plain length, after its CSS definition.
#[derive(Clone, Copy, PartialEq)]
enum LengthKind {
    /// `width`, `height`, `min-*`, `flex-basis`: `auto`; nothing negative.
    Size,
    /// `max-*`: `none` — no maximum, `Length::Auto` in the core; nothing negative.
    Max,
    /// `margin-*`, `top` / `right` / `bottom` / `left`: `auto`, and negative values.
    Offset,
    /// `padding-*`, `border-*-width`: a length only; nothing negative.
    Spacing,
    /// `row-gap`, `column-gap`: `normal`, which is 0 in a flex container; nothing negative.
    Gap,
}

/// The value text of one declaration, read whole by one of the [`Reader`] methods.
struct Value<'a> {
    text: &'a str,
    units: &'a Units,
}

impl<'a> Value<'a> {
    /// The text tokenized, read with `read`, and nothing left after it. The text is tokenized only
    /// here, once the property is known: an unknown property is reported as one even when its
    /// value is something this crate cannot read (`color: #fff`).
    fn read<T>(&self, read: impl FnOnce(&mut Reader<'a>) -> Result<T, ParseError>) -> Result<T, ParseError> {
        let mut reader = Reader { cursor: Cursor::new(self.text)?, units: self.units };
        let value = read(&mut reader)?;
        reader.cursor.end()?;
        Ok(value)
    }
}

/// Reads the components of a value: a keyword, a number, a length, and the shorthands built of them.
struct Reader<'a> {
    cursor: Cursor<'a>,
    units: &'a Units,
}

impl Reader<'_> {
    fn keyword<T: Copy>(&mut self, table: &[(&str, T)]) -> Result<T, ParseError> {
        match self.try_keyword(table) {
            Some(value) => Ok(value),
            None if self.cursor.is_end() => Err(ParseError::new(self.cursor.offset(), ErrorKind::UnexpectedEnd)),
            None => Err(ParseError::expected(self.cursor.offset(), one_of(table))),
        }
    }

    /// The next token's value when it is one of the table's keywords (then it is read); `None`
    /// reads nothing.
    fn try_keyword<T: Copy>(&mut self, table: &[(&str, T)]) -> Option<T> {
        table.iter().find(|(name, _)| self.cursor.eat_keyword(name)).map(|&(_, value)| value)
    }

    fn at_number(&self) -> bool {
        matches!(self.cursor.peek().map(|token| token.kind), Some(TokenKind::Number(_)))
    }

    /// A number that is not negative: `flex-grow`, `flex-shrink`, a side of `aspect-ratio`.
    fn factor(&mut self) -> Result<f32, ParseError> {
        let token = self.cursor.next()?;
        match token.kind {
            TokenKind::Number(number) if number < 0.0 => Err(ParseError::new(token.start, ErrorKind::Negative)),
            TokenKind::Number(number) => Ok(number),
            _ => Err(ParseError::expected(token.start, "a number")),
        }
    }

    fn length(&mut self, kind: LengthKind) -> Result<Length, ParseError> {
        if kind == LengthKind::Max && self.cursor.eat_keyword("none") {
            return Ok(Length::Auto);
        }
        if kind == LengthKind::Gap && self.cursor.eat_keyword("normal") {
            return Ok(Length::ZERO);
        }
        let start = self.cursor.offset();
        let function = matches!(self.cursor.peek().map(|token| token.kind), Some(TokenKind::Function(_)));
        let length = length::length(&mut self.cursor, self.units)?;
        if length.is_auto() && !matches!(kind, LengthKind::Size | LengthKind::Offset) {
            let expected = match kind {
                LengthKind::Max => "`none` or a length",
                LengthKind::Gap => "`normal` or a length",
                _ => "a length other than `auto`",
            };
            return Err(ParseError::expected(start, expected));
        }
        // CSS refuses a negative value where none makes sense, but judges a math function by what
        // it computes to, clamping it then: `width: calc(-1px)` is valid and 0. So only a literal
        // is refused here, and the engine clamps the rest.
        let negative = matches!(length, Length::Px(v) | Length::Percent(v) | Length::Var(_, v) if v < 0.0);
        if negative && !function && kind != LengthKind::Offset {
            return Err(ParseError::new(start, ErrorKind::Negative));
        }
        Ok(length)
    }

    /// 1 to 4 lengths, CSS's box order: `top right bottom left`; a missing right is the top, a
    /// missing bottom the top, a missing left the right.
    fn edges(&mut self, kind: LengthKind) -> Result<Edges<Length>, ParseError> {
        let mut values = vec![self.length(kind)?];
        while values.len() < 4 && !self.cursor.is_end() {
            values.push(self.length(kind)?);
        }
        let top = values[0].clone();
        let right = values.get(1).unwrap_or(&top).clone();
        let bottom = values.get(2).unwrap_or(&top).clone();
        let left = values.get(3).unwrap_or(&right).clone();
        Ok(Edges { left, top, right, bottom })
    }

    /// `gap`: the row gap, then the column gap (the row gap again when omitted).
    fn gap(&mut self) -> Result<(Length, Length), ParseError> {
        let row = self.length(LengthKind::Gap)?;
        let column = if self.cursor.is_end() { row.clone() } else { self.length(LengthKind::Gap)? };
        Ok((row, column))
    }

    /// `flex`: `none`, or `<grow> <shrink>?` and `<basis>` in either order, each at most once.
    /// The answer is grow, shrink, basis.
    fn flex(&mut self) -> Result<(f32, f32, Length), ParseError> {
        if self.cursor.eat_keyword("none") {
            return Ok((0.0, 0.0, Length::Auto));
        }
        let (mut grow, mut shrink, mut basis) = (None, None, None);
        loop {
            // CSS: a bare number is a flex factor unless two factors came before it. Then it is the
            // basis, in px by the embedder's convention (CSS takes only a bare `0` there).
            if grow.is_none() && self.at_number() {
                grow = Some(self.factor()?);
                if self.at_number() {
                    shrink = Some(self.factor()?);
                }
            } else if basis.is_none() {
                basis = Some(self.length(LengthKind::Size)?);
            } else {
                break; // Both parts are read: what is left is the caller's "expected the end".
            }
            if self.cursor.is_end() {
                break;
            }
        }
        // CSS: an omitted factor is 1 and an omitted basis 0% — `flex: 1` is `1 1 0%`.
        Ok((grow.unwrap_or(1.0), shrink.unwrap_or(1.0), basis.unwrap_or(Length::Percent(0.0))))
    }

    /// `flex-flow`: a direction and a wrap in either order, each at most once; an omitted one is
    /// its initial value (`row`, `nowrap`), as in any CSS shorthand.
    fn flex_flow(&mut self) -> Result<(FlexDirection, FlexWrap), ParseError> {
        let (mut direction, mut wrap) = (None, None);
        loop {
            if direction.is_none()
                && let Some(value) = self.try_keyword(FLEX_DIRECTION)
            {
                direction = Some(value);
            } else if wrap.is_none()
                && let Some(value) = self.try_keyword(FLEX_WRAP)
            {
                wrap = Some(value);
            } else if self.cursor.is_end() {
                return Err(ParseError::new(self.cursor.offset(), ErrorKind::UnexpectedEnd));
            } else {
                let expected = "a `flex-direction` or a `flex-wrap` keyword, each at most once";
                return Err(ParseError::expected(self.cursor.offset(), expected));
            }
            if self.cursor.is_end() {
                break;
            }
        }
        Ok((direction.unwrap_or_default(), wrap.unwrap_or_default()))
    }

    /// `aspect-ratio`: `auto`, a number, or `width / height`.
    fn aspect_ratio(&mut self) -> Result<Option<f32>, ParseError> {
        if self.cursor.eat_keyword("auto") {
            return Ok(None);
        }
        let width = self.factor()?;
        let height = if self.cursor.eat(TokenKind::Slash) { self.factor()? } else { 1.0 };
        // CSS: a ratio with a zero in it is degenerate, and the property behaves as `auto`.
        Ok((width > 0.0 && height > 0.0).then(|| width / height))
    }
}

/// The names of a keyword table, for the error: "one of `row`, `column`".
fn one_of<T>(table: &[(&str, T)]) -> String {
    let names: Vec<String> = table.iter().map(|(name, _)| format!("`{name}`")).collect();
    format!("one of {}", names.join(", "))
}
