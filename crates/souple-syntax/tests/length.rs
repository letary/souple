//! The length grammar: every form of a value, `calc()` and its relatives, the whitespace rule, the
//! units table, and each error. Expected values are written by hand; a formula's expected value is
//! the postfix program, built by `Formula::build` (which folds it the same way).

use souple::{Formula, FormulaError, Length, Op, VarId};
use souple_syntax::{ErrorKind, ParseError, Units, parse_length};

const EM: VarId = VarId::new(0);
const VW: VarId = VarId::new(1);
const SAFE_TOP: VarId = VarId::new(2);
const SAFE_BOTTOM: VarId = VarId::new(3);

fn units() -> Units {
    Units::new().unit("em", EM).unit("vw", VW).keyword("safe-top", SAFE_TOP).keyword("safe-bottom", SAFE_BOTTOM)
}

fn parse(text: &str) -> Length {
    parse_length(text, &units()).unwrap_or_else(|error| panic!("`{text}`: {error}"))
}

fn error(text: &str) -> ParseError {
    match parse_length(text, &units()) {
        Ok(length) => panic!("`{text}` parsed as {length:?}"),
        Err(error) => error,
    }
}

/// The error's kind and offset.
fn fails(text: &str) -> (ErrorKind, usize) {
    let error = error(text);
    (error.kind, error.offset)
}

/// The length a postfix program builds.
fn calc(ops: &[Op]) -> Length {
    Formula::build(ops).unwrap()
}

/// The program of a parsed formula, after folding.
fn ops(text: &str) -> Vec<Op> {
    match parse(text) {
        Length::Calc(formula) => formula.ops().to_vec(),
        other => panic!("`{text}` is {other:?}, not a formula"),
    }
}

fn expected(what: &str) -> ErrorKind {
    ErrorKind::Expected(what.to_string())
}

#[test]
fn plain_values() {
    assert_eq!(parse("auto"), Length::Auto);
    assert_eq!(parse("10px"), Length::Px(10.0));
    assert_eq!(parse("50%"), Length::Percent(50.0));
    assert_eq!(parse("2em"), Length::Var(EM, 2.0));
    assert_eq!(parse("10vw"), Length::Var(VW, 10.0));
    assert_eq!(parse("safe-top"), Length::Var(SAFE_TOP, 1.0));
    assert_eq!(parse("-5px"), Length::Px(-5.0));
    assert_eq!(parse("+5px"), Length::Px(5.0));
    assert_eq!(parse("-2.5%"), Length::Percent(-2.5));
    assert_eq!(parse("-0.5em"), Length::Var(EM, -0.5));
}

#[test]
fn a_bare_number_is_px() {
    assert_eq!(parse("0"), Length::Px(0.0));
    assert_eq!(parse("10"), Length::Px(10.0));
    assert_eq!(parse("-3.5"), Length::Px(-3.5));
}

#[test]
fn number_forms() {
    assert_eq!(parse(".5px"), Length::Px(0.5));
    assert_eq!(parse("+.5"), Length::Px(0.5));
    assert_eq!(parse("-.5em"), Length::Var(EM, -0.5));
    assert_eq!(parse("007px"), Length::Px(7.0));
    assert_eq!(parse("1e2px"), Length::Px(100.0));
    assert_eq!(parse("1E2"), Length::Px(100.0));
    assert_eq!(parse("2.5e+1%"), Length::Percent(25.0));
    assert_eq!(parse("1e-1px"), Length::Px(0.1));
    // `e` starts an exponent only before digits: `1em` is one em, `1e3em` a thousand.
    assert_eq!(parse("1em"), Length::Var(EM, 1.0));
    assert_eq!(parse("1e3em"), Length::Var(EM, 1000.0));
}

#[test]
fn case_and_surrounding_whitespace() {
    assert_eq!(parse("AUTO"), Length::Auto);
    assert_eq!(parse("10PX"), Length::Px(10.0));
    assert_eq!(parse("2Em"), Length::Var(EM, 2.0));
    assert_eq!(parse("SAFE-TOP"), Length::Var(SAFE_TOP, 1.0));
    assert_eq!(parse("CALC(1PX + 2Px)"), Length::Px(3.0));
    assert_eq!(parse("Min(1px, 2px)"), Length::Px(1.0));
    assert_eq!(parse("  10px \t\n"), Length::Px(10.0));
    assert_eq!(parse(" calc( 1px + 2px ) "), Length::Px(3.0));
}

#[test]
fn calc_values() {
    assert_eq!(parse("calc(10px)"), Length::Px(10.0));
    assert_eq!(parse("calc(50%)"), Length::Percent(50.0));
    assert_eq!(parse("calc(2em)"), Length::Var(EM, 2.0));
    assert_eq!(parse("calc(1px + 2px)"), Length::Px(3.0));
    assert_eq!(parse("calc(10px - 4px)"), Length::Px(6.0));
    assert_eq!(parse("calc(2 * 10px)"), Length::Px(20.0));
    assert_eq!(parse("calc(10px * 2)"), Length::Px(20.0));
    assert_eq!(parse("calc(10px / 4)"), Length::Px(2.5));
    assert_eq!(parse("calc(2.5 * -2px)"), Length::Px(-5.0));
    assert_eq!(parse("calc(100% - 10px)"), calc(&[Op::Percent(100.0), Op::Px(10.0), Op::Sub]));
    assert_eq!(parse("calc(2em + 10vw)"), calc(&[Op::Var(EM, 2.0), Op::Var(VW, 10.0), Op::Add]));
    assert_eq!(parse("calc(100% - safe-top)"), calc(&[Op::Percent(100.0), Op::Var(SAFE_TOP, 1.0), Op::Sub]));
    assert_eq!(parse("calc(-1 * safe-top)"), calc(&[Op::Num(-1.0), Op::Var(SAFE_TOP, 1.0), Op::Mul]));
}

#[test]
fn precedence() {
    assert_eq!(parse("calc(1px + 2px * 3)"), Length::Px(7.0));
    assert_eq!(parse("calc((1px + 2px) * 3)"), Length::Px(9.0));
    assert_eq!(parse("calc(10px + 6px / 2)"), Length::Px(13.0));
    assert_eq!(parse("calc((10px + 6px) / 2)"), Length::Px(8.0));
    assert_eq!(parse("calc((1px + 2px) * (3 - 1))"), Length::Px(6.0));
    // The product folds inside the formula: one subtraction is left.
    assert_eq!(ops("calc(100% - 2px * 3)"), [Op::Percent(100.0), Op::Px(6.0), Op::Sub]);
    // A percent times a number is one operand.
    assert_eq!(ops("calc(10px - 100% * 2)"), [Op::Px(10.0), Op::Percent(200.0), Op::Sub]);
    assert_eq!(ops("calc(100% * 2 - 10px)"), [Op::Percent(200.0), Op::Px(10.0), Op::Sub]);
}

#[test]
fn left_associativity() {
    // Right to left would read 9px, 3px, 36px and 2px.
    assert_eq!(parse("calc(10px - 4px - 3px)"), Length::Px(3.0));
    assert_eq!(parse("calc(10px - 4px + 3px)"), Length::Px(9.0));
    assert_eq!(parse("calc(24px / 2 / 3)"), Length::Px(4.0));
    assert_eq!(parse("calc(12px / 2 * 3)"), Length::Px(18.0));
    // With a percent the sums do not fold, and the program shows the order.
    assert_eq!(ops("calc(100% - 10px - 5px)"), [Op::Percent(100.0), Op::Px(10.0), Op::Sub, Op::Px(5.0), Op::Sub]);
    assert_eq!(parse("calc(100% / 2 / 5)"), Length::Percent(10.0));
}

#[test]
fn min_max_clamp() {
    assert_eq!(parse("min(10px, 50%)"), calc(&[Op::Px(10.0), Op::Percent(50.0), Op::Min(2)]));
    assert_eq!(parse("max(10px, 50%)"), calc(&[Op::Px(10.0), Op::Percent(50.0), Op::Max(2)]));
    assert_eq!(parse("max(1px)"), Length::Px(1.0));
    assert_eq!(parse("min(3px, 1px, 2px)"), Length::Px(1.0));
    assert_eq!(parse("max(3px, 1px, 2px)"), Length::Px(3.0));
    assert_eq!(parse("clamp(10px, 50%, 30px)"), calc(&[Op::Px(10.0), Op::Percent(50.0), Op::Px(30.0), Op::Clamp]));
    assert_eq!(parse("clamp(10px, 5px, 30px)"), Length::Px(10.0));
    assert_eq!(parse("clamp(10px, 50px, 30px)"), Length::Px(30.0));
    assert_eq!(parse("min(1px + 2px, 2px * 2)"), Length::Px(3.0));
    assert_eq!(parse(" min( 1px , 2px ) "), Length::Px(1.0));
    assert_eq!(parse("CLAMP(1px, 2px, 3px)"), Length::Px(2.0));
}

#[test]
fn nesting() {
    assert_eq!(parse("calc(calc(1px + 2px) * 2)"), Length::Px(6.0));
    // A nested calc() may be a number; only the whole formula must be a length.
    assert_eq!(parse("calc(2 * calc(3) * 1px)"), Length::Px(6.0));
    assert_eq!(parse("calc(((1px)))"), Length::Px(1.0));
    assert_eq!(parse("calc(min(10px, 5px) + 1px)"), Length::Px(6.0));
    assert_eq!(
        parse("min(calc(100% - 10px), max(50px, 10%))"),
        calc(&[Op::Percent(100.0), Op::Px(10.0), Op::Sub, Op::Px(50.0), Op::Percent(10.0), Op::Max(2), Op::Min(2)])
    );
    assert_eq!(
        parse("clamp(10px, calc(50% + 1em), 300px)"),
        calc(&[Op::Px(10.0), Op::Percent(50.0), Op::Var(EM, 1.0), Op::Add, Op::Px(300.0), Op::Clamp])
    );
    // 16 levels — calc() and 15 parentheses — is the limit, and fits.
    let deepest = format!("calc({}1px{})", "(".repeat(15), ")".repeat(15));
    assert_eq!(parse(&deepest), Length::Px(1.0));
}

#[test]
fn plus_and_minus_need_whitespace_on_both_sides() {
    assert_eq!(parse("calc(1px + 2px)"), Length::Px(3.0));
    assert_eq!(parse("calc(1px   +   2px)"), Length::Px(3.0));
    assert_eq!(parse("calc(1px\t-\n2px)"), Length::Px(-1.0));
    // A signed number after a spaced operator is a value.
    assert_eq!(parse("calc(1px - -2px)"), Length::Px(3.0));
    assert_eq!(parse("calc(-2px + 1px)"), Length::Px(-1.0));

    assert_eq!(fails("calc(1px+2px)"), (ErrorKind::OperatorSpacing, 8));
    assert_eq!(fails("calc(1px +2px)"), (ErrorKind::OperatorSpacing, 9));
    assert_eq!(fails("calc(1px+ 2px)"), (ErrorKind::OperatorSpacing, 8));
    assert_eq!(fails("calc(1px -2px)"), (ErrorKind::OperatorSpacing, 9));
    assert_eq!(fails("calc(1px +-2px)"), (ErrorKind::OperatorSpacing, 9));
    assert_eq!(fails("calc(1px -(2px))"), (ErrorKind::OperatorSpacing, 9));
    assert_eq!(fails("calc(2 -1)"), (ErrorKind::OperatorSpacing, 7));
    assert_eq!(fails("min(1px, 2px -1px)"), (ErrorKind::OperatorSpacing, 13));
    // Without the spaces a hyphen is part of a name: here, a unit `px-2px`.
    assert_eq!(fails("calc(1px-2px)"), (ErrorKind::UnknownUnit("px-2px".into()), 6));
}

#[test]
fn star_and_slash_need_no_whitespace() {
    assert_eq!(parse("calc(2*3px)"), Length::Px(6.0));
    assert_eq!(parse("calc(2 *3px)"), Length::Px(6.0));
    assert_eq!(parse("calc(6px/2)"), Length::Px(3.0));
    assert_eq!(parse("calc(6px/ 2)"), Length::Px(3.0));
}

#[test]
fn hyphenated_keywords_next_to_operators() {
    let top = Op::Var(SAFE_TOP, 1.0);
    let bottom = Op::Var(SAFE_BOTTOM, 1.0);
    assert_eq!(parse("calc(safe-top - 1px)"), calc(&[top, Op::Px(1.0), Op::Sub]));
    assert_eq!(parse("calc(1px - safe-top)"), calc(&[Op::Px(1.0), top, Op::Sub]));
    assert_eq!(parse("calc(safe-top + safe-bottom)"), calc(&[top, bottom, Op::Add]));
    assert_eq!(parse("calc(safe-top*2)"), calc(&[top, Op::Num(2.0), Op::Mul]));
    assert_eq!(parse("calc(2*safe-top)"), calc(&[Op::Num(2.0), top, Op::Mul]));
    assert_eq!(parse("calc(safe-top/2)"), calc(&[top, Op::Num(2.0), Op::Div]));
    assert_eq!(parse("max(safe-top, 8px)"), calc(&[top, Op::Px(8.0), Op::Max(2)]));
    // Without spaces the hyphen and what follows belong to the name.
    assert_eq!(fails("calc(safe-top-1px)"), (ErrorKind::UnknownKeyword("safe-top-1px".into()), 5));
    assert_eq!(fails("calc(safe-top -1px)"), (ErrorKind::OperatorSpacing, 14));
    assert_eq!(fails("calc(100%-safe-top)"), (ErrorKind::OperatorSpacing, 9));
    assert_eq!(fails("calc(100% -safe-top)"), (ErrorKind::OperatorSpacing, 10));
    // CSS has no unary minus: a leading hyphen is part of the name.
    assert_eq!(fails("calc(-safe-top)"), (ErrorKind::UnknownKeyword("-safe-top".into()), 5));
    assert_eq!(fails("-safe-top"), (ErrorKind::UnknownKeyword("-safe-top".into()), 0));
}

#[test]
fn unexpected_char() {
    assert_eq!(fails("#fff"), (ErrorKind::UnexpectedChar('#'), 0));
    assert_eq!(fails("10px;"), (ErrorKind::UnexpectedChar(';'), 4));
    // `5.` is no number: the digits after the point are required.
    assert_eq!(fails("5.px"), (ErrorKind::UnexpectedChar('.'), 1));
    assert_eq!(fails("10 %"), (ErrorKind::UnexpectedChar('%'), 3));
    assert_eq!(fails("calc(1px # 2px)"), (ErrorKind::UnexpectedChar('#'), 9));
}

#[test]
fn unexpected_end() {
    assert_eq!(fails(""), (ErrorKind::UnexpectedEnd, 0));
    assert_eq!(fails("   "), (ErrorKind::UnexpectedEnd, 3));
    assert_eq!(fails("calc("), (ErrorKind::UnexpectedEnd, 5));
    assert_eq!(fails("calc(1px"), (ErrorKind::UnexpectedEnd, 8));
    assert_eq!(fails("calc(1px +"), (ErrorKind::UnexpectedEnd, 10));
    assert_eq!(fails("calc(1px * "), (ErrorKind::UnexpectedEnd, 11));
    assert_eq!(fails("min(1px,"), (ErrorKind::UnexpectedEnd, 8));
    assert_eq!(fails("calc((1px)"), (ErrorKind::UnexpectedEnd, 10));
}

#[test]
fn expected_something_else() {
    assert_eq!(fails("10px 20px"), (expected("the end of the value"), 5));
    // Arithmetic needs a math function.
    assert_eq!(fails("1px + 2px"), (expected("the end of the value"), 4));
    assert_eq!(fails("(1px)"), (expected("a length"), 0));
    assert_eq!(fails(")"), (expected("a length"), 0));
    assert_eq!(fails("calc()"), (expected("a number, a length or a percentage"), 5));
    assert_eq!(fails("min()"), (expected("a number, a length or a percentage"), 4));
    assert_eq!(fails("calc(1px * )"), (expected("a number, a length or a percentage"), 11));
    assert_eq!(fails("calc(1px 2px)"), (expected("an operator or `)`"), 9));
    assert_eq!(fails("calc(1px, 2px)"), (expected("an operator or `)`"), 8));
    assert_eq!(fails("min(1px 2px)"), (expected("an operator, `,` or `)`"), 8));
    assert_eq!(fails("calc((1px 2px))"), (expected("an operator or `)`"), 10));
    assert_eq!(fails("calc(1px))"), (expected("the end of the value"), 9));
}

#[test]
fn invalid_number() {
    assert_eq!(fails("1e39px"), (ErrorKind::InvalidNumber, 0));
    assert_eq!(fails("calc(1px * 1e39)"), (ErrorKind::InvalidNumber, 11));
}

#[test]
fn unknown_names() {
    assert_eq!(fails("10foo"), (ErrorKind::UnknownUnit("foo".into()), 2));
    assert_eq!(fails("10px-5px"), (ErrorKind::UnknownUnit("px-5px".into()), 2));
    assert_eq!(fails("calc(1qq)"), (ErrorKind::UnknownUnit("qq".into()), 6));
    assert_eq!(fails("banana"), (ErrorKind::UnknownKeyword("banana".into()), 0));
    assert_eq!(fails("none"), (ErrorKind::UnknownKeyword("none".into()), 0));
    assert_eq!(fails("é"), (ErrorKind::UnknownKeyword("é".into()), 0));
    assert_eq!(fails("calc(1px + foo)"), (ErrorKind::UnknownKeyword("foo".into()), 11));
    // `auto` is a value of a property, not a number a formula can compute with.
    assert_eq!(fails("calc(auto)"), (ErrorKind::UnknownKeyword("auto".into()), 5));
    // A space between a function's name and `(` makes the name a keyword.
    assert_eq!(fails("calc (1px)"), (ErrorKind::UnknownKeyword("calc".into()), 0));
    assert_eq!(fails("foo(1px)"), (ErrorKind::UnknownFunction("foo".into()), 0));
    assert_eq!(fails("calc(1px + bar(2px))"), (ErrorKind::UnknownFunction("bar".into()), 11));
}

#[test]
fn argument_count() {
    assert_eq!(fails("clamp(1px, 2px)"), (ErrorKind::ArgumentCount("clamp"), 0));
    assert_eq!(fails("clamp(1px, 2px, 3px, 4px)"), (ErrorKind::ArgumentCount("clamp"), 0));
    assert_eq!(fails("calc(1px + clamp(1px))"), (ErrorKind::ArgumentCount("clamp"), 11));
    // `Op::Min` / `Op::Max` count their arguments in a byte.
    let many = format!("{}1px)", "1px, ".repeat(255));
    assert_eq!(fails(&format!("min({many}")), (ErrorKind::ArgumentCount("min"), 0));
    assert_eq!(fails(&format!("MAX({many}")), (ErrorKind::ArgumentCount("max"), 0));
}

#[test]
fn the_formula_judges_types() {
    let mismatch = ErrorKind::Formula(FormulaError::Type);
    assert_eq!(fails("calc(10px + 2)"), (mismatch.clone(), 0));
    assert_eq!(fails("calc(2 + 10px)"), (mismatch.clone(), 0));
    assert_eq!(fails("calc(2 * 3 + 4px)"), (mismatch.clone(), 0));
    assert_eq!(fails("calc(1px * 2px)"), (mismatch.clone(), 0));
    assert_eq!(fails("calc(10% * 1em)"), (mismatch.clone(), 0));
    assert_eq!(fails("calc(2 / 1px)"), (mismatch.clone(), 0));
    assert_eq!(fails("min(1px, 2)"), (mismatch.clone(), 0));
    assert_eq!(fails("clamp(1, 2, 3)"), (mismatch.clone(), 0));
    // A bare number is no length, whatever the parentheses.
    assert_eq!(fails("calc(3)"), (mismatch.clone(), 0));
    assert_eq!(fails("calc(1 + 2)"), (mismatch.clone(), 0));
    // The error points at the outer function: the formula is judged whole.
    assert_eq!(fails("  calc(1px + min(1px, 2))"), (mismatch, 2));
}

#[test]
fn the_formula_judges_division_by_zero() {
    let zero = ErrorKind::Formula(FormulaError::DivisionByZero);
    assert_eq!(fails("calc(1px / 0)"), (zero.clone(), 0));
    assert_eq!(fails("calc(100% / 0)"), (zero.clone(), 0));
    assert_eq!(fails("calc(1px / (2 - 2))"), (zero, 0));
}

#[test]
fn too_deep() {
    let deep = ErrorKind::Formula(FormulaError::TooDeep);
    // calc() and 16 parentheses: the 17th level, at the 16th parenthesis.
    let nested = format!("calc({}1px{})", "(".repeat(16), ")".repeat(16));
    assert_eq!(fails(&nested), (deep.clone(), 20));
    // Text from anywhere: the reader stops at the limit rather than recursing to a stack overflow.
    let hostile = format!("calc({}1px{})", "(".repeat(100_000), ")".repeat(100_000));
    assert_eq!(error(&hostile).kind, deep);
    // Seventeen values on the evaluator's stack at once: the formula's own limit.
    let wide = format!("min({}1%)", "1%, ".repeat(16));
    assert_eq!(fails(&wide), (deep, 0));
}

#[test]
fn the_units_table() {
    let em = Units::new().unit("EM", EM);
    assert_eq!(parse_length("2em", &em), Ok(Length::Var(EM, 2.0)), "a unit is matched in any case");
    let redefined = Units::new().unit("em", EM).unit("Em", VW);
    assert_eq!(parse_length("2em", &redefined), Ok(Length::Var(VW, 2.0)), "a unit defined again is replaced");
    // A unit and a keyword of the same name are separate tables.
    let both = Units::new().unit("vh", EM).keyword("vh", VW);
    assert_eq!(parse_length("2vh", &both), Ok(Length::Var(EM, 2.0)));
    assert_eq!(parse_length("vh", &both), Ok(Length::Var(VW, 1.0)));
    // Without a table only px and percent are known.
    let none = Units::new();
    assert_eq!(parse_length("2em", &none).unwrap_err().kind, ErrorKind::UnknownUnit("em".into()));
    assert_eq!(parse_length("safe-top", &none).unwrap_err().kind, ErrorKind::UnknownKeyword("safe-top".into()));
}

#[test]
#[should_panic(expected = "the core's own unit")]
fn px_is_not_a_unit_of_the_table() {
    let _ = Units::new().unit("PX", EM);
}

#[test]
#[should_panic(expected = "a unit is a CSS name")]
fn a_unit_is_a_name() {
    let _ = Units::new().unit("2x", EM);
}

#[test]
#[should_panic(expected = "a keyword is a CSS name")]
fn a_keyword_is_a_name() {
    let _ = Units::new().keyword("safe top", SAFE_TOP);
}

#[test]
#[should_panic(expected = "`auto` is every length property's own keyword")]
fn auto_is_not_a_keyword_of_the_table() {
    let _ = Units::new().keyword("auto", SAFE_TOP);
}

#[test]
fn messages() {
    assert_eq!(error("10foo").to_string(), "unknown unit `foo` at byte 2");
    assert_eq!(error("calc(1px+2px)").to_string(), "`+` and `-` need whitespace on both sides at byte 8");
    assert_eq!(error("calc(1px / 0)").to_string(), "division by zero at byte 0");
    assert_eq!(error("clamp(1px)").to_string(), "`clamp()` takes 3 arguments at byte 0");
    let error: Box<dyn std::error::Error> = Box::new(error(""));
    assert_eq!(error.to_string(), "the value ends too early at byte 0");
}
