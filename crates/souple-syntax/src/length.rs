//! Lengths: one value as text → a [`Length`]; `calc()` and its relatives → a [`Formula`].
//!
//! A math function is read by recursive descent straight into the postfix [`Op`] program the core
//! evaluates, and [`Formula::build`] alone judges the result: the types (`1px + 2` is refused),
//! a division by zero, the constant folding (`calc(1px + 2px)` comes back as `Px(3)`). This file
//! knows only the grammar — CSS Values 4:
//!
//! ```text
//! sum     = product ( ( " + " | " - " ) product )*      whitespace on both sides of + and -
//! product = value ( ( "*" | "/" ) value )*
//! value   = number | dimension | percentage | keyword | "(" sum ")" | function
//! function = calc( sum ) | min( sum#) | max( sum# ) | clamp( sum, sum, sum )
//! ```
//!
//! Inside a formula a bare number is a NUMBER, a factor or a divisor (`calc(2 * 10px)`); outside,
//! the embedder's convention makes it px (`width: 10`).

use souple::{Formula, FormulaError, Length, Op};

use crate::error::{ErrorKind, ParseError};
use crate::token::{Cursor, Token, TokenKind};
use crate::units::Units;

/// One length, the whole text: `auto`, `10px`, `10` (px), `50%`, `2em`, `safe-top`,
/// `calc(100% - 2em)`, `min(…)`, `max(…)`, `clamp(…)`.
pub fn parse_length(input: &str, units: &Units) -> Result<Length, ParseError> {
    let mut cursor = Cursor::new(input)?;
    let length = length(&mut cursor, units)?;
    cursor.end()?;
    Ok(length)
}

/// One length at the cursor: the declarations read their components with it.
pub(crate) fn length(cursor: &mut Cursor, units: &Units) -> Result<Length, ParseError> {
    let token = cursor.next()?;
    match token.kind {
        TokenKind::Ident(name) if name.eq_ignore_ascii_case("auto") => Ok(Length::Auto),
        TokenKind::Function(name) => {
            let mut calc = Calc { cursor, units, ops: Vec::new(), depth: 0 };
            calc.function(name, token.start)?;
            Formula::build(&calc.ops).map_err(|error| ParseError::new(token.start, ErrorKind::Formula(error)))
        }
        _ => match operand(token, units)? {
            // The embedder's convention: a bare number is px (CSS takes only a bare `0`).
            Some(Op::Num(px) | Op::Px(px)) => Ok(Length::Px(px)),
            Some(Op::Percent(percent)) => Ok(Length::Percent(percent)),
            Some(Op::Var(var, factor)) => Ok(Length::Var(var, factor)),
            _ => Err(ParseError::expected(token.start, "a length")),
        },
    }
}

/// The op a value token pushes. A bare number is `Op::Num`: a factor in a formula, px at the top
/// level (the caller's choice). `None`: the token is not a value (a parenthesis, an operator).
fn operand(token: Token, units: &Units) -> Result<Option<Op>, ParseError> {
    Ok(Some(match token.kind {
        TokenKind::Number(number) => Op::Num(number),
        TokenKind::Percentage(percent) => Op::Percent(percent),
        TokenKind::Dimension(value, unit) if unit.eq_ignore_ascii_case("px") => Op::Px(value),
        TokenKind::Dimension(value, unit) => match units.find_unit(unit) {
            Some(var) => Op::Var(var, value),
            None => {
                let at = token.start + token.text.len() - unit.len();
                return Err(ParseError::new(at, ErrorKind::UnknownUnit(unit.to_string())));
            }
        },
        TokenKind::Ident(name) => match units.find_keyword(name) {
            Some(var) => Op::Var(var, 1.0),
            None => return Err(ParseError::new(token.start, ErrorKind::UnknownKeyword(name.to_string()))),
        },
        _ => return Ok(None),
    }))
}

#[derive(Clone, Copy, PartialEq)]
enum Function {
    Calc,
    Min,
    Max,
    Clamp,
}

impl Function {
    const NAMES: [(&'static str, Function); 4] =
        [("calc", Function::Calc), ("min", Function::Min), ("max", Function::Max), ("clamp", Function::Clamp)];

    fn named(name: &str) -> Option<(&'static str, Function)> {
        Self::NAMES.into_iter().find(|(known, _)| known.eq_ignore_ascii_case(name))
    }
}

/// The reader of one math function and everything inside it, writing its postfix program.
struct Calc<'c, 'a> {
    cursor: &'c mut Cursor<'a>,
    units: &'c Units,
    ops: Vec<Op>,
    /// The parentheses and functions open around the cursor.
    depth: usize,
}

impl Calc<'_, '_> {
    /// A math function whose `name(` was just read, through its `)`.
    fn function(&mut self, name: &str, start: usize) -> Result<(), ParseError> {
        let (name, function) =
            Function::named(name).ok_or_else(|| ParseError::new(start, ErrorKind::UnknownFunction(name.to_string())))?;
        self.enter(start)?;
        let mut count = 0usize;
        loop {
            self.sum()?;
            count += 1;
            let token = self.cursor.next()?;
            match token.kind {
                TokenKind::Close => break,
                TokenKind::Comma if function != Function::Calc => {}
                _ if function == Function::Calc => return Err(ParseError::expected(token.start, "an operator or `)`")),
                _ => return Err(ParseError::expected(token.start, "an operator, `,` or `)`")),
            }
        }
        self.depth -= 1;
        let op = match (function, u8::try_from(count)) {
            (Function::Calc, _) => return Ok(()),
            (Function::Clamp, _) if count == 3 => Op::Clamp,
            (Function::Min, Ok(count)) => Op::Min(count),
            (Function::Max, Ok(count)) => Op::Max(count),
            _ => return Err(ParseError::new(start, ErrorKind::ArgumentCount(name))),
        };
        self.ops.push(op);
        Ok(())
    }

    /// `a + b - c`: products joined by `+` / `-`, left to right.
    fn sum(&mut self) -> Result<(), ParseError> {
        self.product()?;
        while let Some(token) = self.cursor.peek() {
            let op = match token.kind {
                TokenKind::Plus => Op::Add,
                TokenKind::Minus => Op::Sub,
                // A value glued to its sign right after another value — `1px -2px`, `1px+2px`,
                // `100%-safe-top` — is an operator that lost its spaces: say so, rather than
                // "expected `)`" at a value the author sees as two.
                TokenKind::Number(_) | TokenKind::Percentage(_) | TokenKind::Dimension(..) | TokenKind::Ident(_)
                    if token.text.starts_with(['+', '-']) =>
                {
                    return Err(ParseError::new(token.start, ErrorKind::OperatorSpacing));
                }
                _ => break,
            };
            // The end of the text after the operator is the next value's error, not this one's.
            let unspaced_after = self.cursor.peek_second().is_some_and(|next| !next.spaced);
            if !token.spaced || unspaced_after {
                return Err(ParseError::new(token.start, ErrorKind::OperatorSpacing));
            }
            self.cursor.next()?;
            self.product()?;
            self.ops.push(op);
        }
        Ok(())
    }

    /// `a * b / c`: values joined by `*` / `/`, left to right. No whitespace rule: neither can
    /// be mistaken for a sign.
    fn product(&mut self) -> Result<(), ParseError> {
        self.value()?;
        loop {
            let op = if self.cursor.eat(TokenKind::Star) {
                Op::Mul
            } else if self.cursor.eat(TokenKind::Slash) {
                Op::Div
            } else {
                return Ok(());
            };
            self.value()?;
            self.ops.push(op);
        }
    }

    /// One operand: a number, a length, a keyword, a sum in parentheses, a nested function.
    fn value(&mut self) -> Result<(), ParseError> {
        let token = self.cursor.next()?;
        match token.kind {
            TokenKind::Open => {
                self.enter(token.start)?;
                self.sum()?;
                let close = self.cursor.next()?;
                if close.kind != TokenKind::Close {
                    return Err(ParseError::expected(close.start, "an operator or `)`"));
                }
                self.depth -= 1;
                Ok(())
            }
            TokenKind::Function(name) => self.function(name, token.start),
            _ => match operand(token, self.units)? {
                Some(op) => {
                    self.ops.push(op);
                    Ok(())
                }
                None => Err(ParseError::expected(token.start, "a number, a length or a percentage")),
            },
        }
    }

    /// One more parenthesis or function. Bounded because the reader recurses per level and the
    /// text may come from anywhere: it must not overflow the stack. The evaluator's stack depth
    /// is the bound — a formula nested deeper is not one anybody meant.
    fn enter(&mut self, start: usize) -> Result<(), ParseError> {
        self.depth += 1;
        if self.depth > Formula::MAX_DEPTH {
            return Err(ParseError::new(start, ErrorKind::Formula(FormulaError::TooDeep)));
        }
        Ok(())
    }
}
