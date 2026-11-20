//! The tokenizer: value text → tokens, after CSS Syntax 3, cut down to what a layout value uses.
//!
//! Two CSS rules live here rather than in the grammar, because characters decide them:
//!
//! - A sign glued to a number belongs to the number (`-5px` is one token), and a hyphen inside a
//!   name belongs to the name (`safe-top`; `10px-5px` is the number 10 with the unit `px-5px`).
//!   That is why CSS wants whitespace around a binary `+` / `-`: without it the text IS a signed
//!   number or a longer name. Each token records whether whitespace came before it, so the
//!   grammar can insist on the spaces.
//! - A name directly followed by `(` is a function (`calc(`); with a space between, it is a name.
//!
//! The text is read as bytes: every decision is about an ASCII character, and a non-ASCII
//! character is part of a name (as in CSS), so a token never ends inside one.

use crate::error::{ErrorKind, ParseError};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum TokenKind<'a> {
    /// A bare number: `2`, `-0.5`, `1e3`.
    Number(f32),
    /// `50%`: the number.
    Percentage(f32),
    /// A number with a unit, as written: `10px`, `2EM`.
    Dimension(f32, &'a str),
    /// A name: `auto`, `row-reverse`, `safe-top`.
    Ident(&'a str),
    /// A name and its opening parenthesis: `calc(` is `Function("calc")`.
    Function(&'a str),
    Open,
    Close,
    Comma,
    Plus,
    Minus,
    Star,
    Slash,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Token<'a> {
    pub kind: TokenKind<'a>,
    /// The token's text.
    pub text: &'a str,
    /// Its byte offset in the value.
    pub start: usize,
    /// Whitespace comes right before it.
    pub spaced: bool,
}

/// The tokens of one value, read front to back by the grammar.
pub(crate) struct Cursor<'a> {
    tokens: Vec<Token<'a>>,
    next: usize,
    /// The length of the text: where an error at the end points.
    end: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(text: &'a str) -> Result<Self, ParseError> {
        Ok(Self { tokens: tokenize(text)?, next: 0, end: text.len() })
    }

    pub fn peek(&self) -> Option<Token<'a>> {
        self.tokens.get(self.next).copied()
    }

    /// The token after the next one.
    pub fn peek_second(&self) -> Option<Token<'a>> {
        self.tokens.get(self.next + 1).copied()
    }

    /// The next token; the end of the text here is an error.
    pub fn next(&mut self) -> Result<Token<'a>, ParseError> {
        let token = self.peek().ok_or(ParseError::new(self.end, ErrorKind::UnexpectedEnd))?;
        self.next += 1;
        Ok(token)
    }

    /// Where the next token starts, or the end of the text.
    pub fn offset(&self) -> usize {
        self.peek().map_or(self.end, |token| token.start)
    }

    pub fn is_end(&self) -> bool {
        self.next == self.tokens.len()
    }

    /// Nothing is left.
    pub fn end(&self) -> Result<(), ParseError> {
        match self.peek() {
            None => Ok(()),
            Some(token) => Err(ParseError::expected(token.start, "the end of the value")),
        }
    }

    /// Skips the next token if it is this name (ASCII case-insensitive).
    pub fn eat_keyword(&mut self, keyword: &str) -> bool {
        let found = match self.peek().map(|token| token.kind) {
            Some(TokenKind::Ident(name)) => name.eq_ignore_ascii_case(keyword),
            _ => false,
        };
        self.next += found as usize;
        found
    }

    /// Skips the next token if it is this one (an operator or a parenthesis).
    pub fn eat(&mut self, kind: TokenKind) -> bool {
        let found = self.peek().is_some_and(|token| token.kind == kind);
        self.next += found as usize;
        found
    }
}

fn tokenize(text: &str) -> Result<Vec<Token<'_>>, ParseError> {
    let bytes = text.as_bytes();
    let mut tokens = Vec::new();
    let mut at = 0;
    let mut spaced = false;
    while at < bytes.len() {
        let byte = bytes[at];
        if matches!(byte, b' ' | b'\t' | b'\n' | b'\r' | b'\x0C') {
            spaced = true;
            at += 1;
            continue;
        }
        let start = at;
        let kind = if let Some(len) = number_len(bytes, at) {
            let value = number(&text[at..at + len], start)?;
            at += len;
            if bytes.get(at) == Some(&b'%') {
                at += 1;
                TokenKind::Percentage(value)
            } else if starts_name(bytes, at) {
                let unit = at;
                at = name_end(bytes, at);
                TokenKind::Dimension(value, &text[unit..at])
            } else {
                TokenKind::Number(value)
            }
        } else if starts_name(bytes, at) {
            at = name_end(bytes, at);
            let name = &text[start..at];
            if bytes.get(at) == Some(&b'(') {
                at += 1;
                TokenKind::Function(name)
            } else {
                TokenKind::Ident(name)
            }
        } else {
            at += 1;
            match byte {
                b'(' => TokenKind::Open,
                b')' => TokenKind::Close,
                b',' => TokenKind::Comma,
                b'+' => TokenKind::Plus,
                b'-' => TokenKind::Minus,
                b'*' => TokenKind::Star,
                b'/' => TokenKind::Slash,
                _ => {
                    let char = text[start..].chars().next().unwrap_or('\u{FFFD}');
                    return Err(ParseError::new(start, ErrorKind::UnexpectedChar(char)));
                }
            }
        };
        tokens.push(Token { kind, text: &text[start..at], start, spaced });
        spaced = false;
    }
    Ok(tokens)
}

/// The length of the number that starts at `at`, `None` when none does: an optional sign, digits
/// with an optional fraction or a fraction alone (`.5`, not `5.`), an optional exponent.
fn number_len(bytes: &[u8], at: usize) -> Option<usize> {
    let digit = |i: usize| bytes.get(i).is_some_and(u8::is_ascii_digit);
    let mut end = at;
    if matches!(bytes.get(end), Some(b'+' | b'-')) {
        end += 1;
    }
    let integer = end;
    while digit(end) {
        end += 1;
    }
    if bytes.get(end) == Some(&b'.') && digit(end + 1) {
        end += 1;
        while digit(end) {
            end += 1;
        }
    } else if end == integer {
        return None;
    }
    // An exponent only when digits follow: `1e3` is a thousand, `1em` one em.
    if matches!(bytes.get(end), Some(b'e' | b'E')) {
        let mut exponent = end + 1;
        if matches!(bytes.get(exponent), Some(b'+' | b'-')) {
            exponent += 1;
        }
        if digit(exponent) {
            end = exponent;
            while digit(end) {
                end += 1;
            }
        }
    }
    Some(end - at)
}

/// The value of a number `number_len` measured: Rust's float syntax includes CSS's.
fn number(text: &str, start: usize) -> Result<f32, ParseError> {
    match text.parse::<f32>() {
        Ok(value) if value.is_finite() => Ok(value),
        _ => Err(ParseError::new(start, ErrorKind::InvalidNumber)),
    }
}

fn is_name_start(byte: u8) -> bool {
    byte.is_ascii_alphabetic() || byte == b'_' || byte >= 0x80
}

fn is_name(byte: u8) -> bool {
    is_name_start(byte) || byte.is_ascii_digit() || byte == b'-'
}

/// A name starts at `at`: a letter, or a hyphen before a letter or another hyphen (`-x`, `--x`).
/// A hyphen before a digit is a sign, and `number_len` has taken it first.
fn starts_name(bytes: &[u8], at: usize) -> bool {
    match bytes.get(at) {
        Some(b'-') => bytes.get(at + 1).is_some_and(|&next| is_name_start(next) || next == b'-'),
        Some(&byte) => is_name_start(byte),
        None => false,
    }
}

fn name_end(bytes: &[u8], mut at: usize) -> usize {
    while bytes.get(at).is_some_and(|&byte| is_name(byte)) {
        at += 1;
    }
    at
}

/// The text is one name, as the tokenizer reads names: what a unit or a keyword must be to be found.
pub(crate) fn is_name_text(text: &str) -> bool {
    let bytes = text.as_bytes();
    starts_name(bytes, 0) && name_end(bytes, 0) == bytes.len()
}
