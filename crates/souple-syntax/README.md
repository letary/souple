# souple-syntax

[![crates.io](https://img.shields.io/crates/v/souple-syntax.svg)](https://crates.io/crates/souple-syntax)
[![docs.rs](https://img.shields.io/docsrs/souple-syntax)](https://docs.rs/souple-syntax)
[![license: MIT](https://img.shields.io/crates/l/souple-syntax.svg)](https://github.com/letary/souple/blob/main/LICENSE)

CSS value text → the engine's typed values. The engine (`souple`) knows no strings; this
crate is the only place that reads them. One dependency: the engine.

```
cargo test -p souple-syntax
```

## Use

```rust
use souple::{Length, Style, VarId};
use souple_syntax::{apply_declaration, parse_length, Units};

// The embedder names its variables once.
const EM: VarId = VarId::new(0);
const SAFE_TOP: VarId = VarId::new(1);
let units = Units::new()
    .unit("em", EM)                 // 2em      → Length::Var(EM, 2.0)
    .keyword("safe-top", SAFE_TOP); // safe-top → Length::Var(SAFE_TOP, 1.0)

// One length.
let width = parse_length("calc(100% - 2em)", &units)?;

// One declaration, kebab-case or camelCase.
let mut style = Style::default();
apply_declaration(&mut style, "flex-direction", "column", &units)?;
apply_declaration(&mut style, "paddingTop", "max(safe-top, 12px)", &units)?;
apply_declaration(&mut style, "margin", "8px auto", &units)?;
```

A declaration that fails leaves the style untouched. A `ParseError` carries a byte offset into the
value and an `ErrorKind`; an unknown property is `ErrorKind::UnknownProperty`, so an embedder can
route it elsewhere (a paint property).

## The grammar

| text | value |
|---|---|
| `auto` | `Length::Auto` |
| `12px`, `12`, `.5px`, `1e2px` | px — a bare number is px |
| `50%` | percent |
| `2em`, `safe-top` | a variable, by the `Units` table |
| `calc(100% - 2em)` | `+ - * /`, parentheses, the usual precedence |
| `min(a, b, …)`, `max(…)`, `clamp(min, value, max)` | also inside a `calc()`, and at the top level |

- As in CSS, a binary `+` or `-` needs whitespace on both sides: that is what tells `safe-top` and
  `-2px` from a subtraction. `*` and `/` do not.
- Inside a math function a bare number is a NUMBER (a factor), not px: `calc(2 * 10px)` is 20px,
  `calc(10px + 2)` is a type error.
- The type rules and the constant folding are the engine's (`Formula::build`); this crate reads the
  grammar and passes its verdict on.
- Names of functions, units and keywords match in any case.

## The properties

Every field of `Style`: `display`, `position`, `box-sizing`, `overflow`, `flex-direction`,
`flex-wrap`, `justify-content`, `align-items`, `align-self`, `align-content`, `flex-grow`,
`flex-shrink`, `flex-basis`, `aspect-ratio`, `width`, `height`, `min-*`, `max-*`, `margin-*`,
`padding-*`, `border-*-width`, `top` / `right` / `bottom` / `left`, `row-gap`, `column-gap`.

Shorthands, with CSS's meaning: `margin`, `padding`, `border-width`, `inset` (1–4 values), `gap`
(1–2), `flex` (`flex: 1` is `1 1 0%`), `flex-flow`.

## The files

| file | owns |
|---|---|
| `token.rs` | the tokenizer and the cursor the grammar reads from |
| `length.rs` | `parse_length`: one length; a math function becomes postfix `Op`s |
| `declaration.rs` | `apply_declaration`: one match arm per property — the match IS the list |
| `units.rs` | the `Units` table |
| `error.rs` | `ParseError`, `ErrorKind` |
