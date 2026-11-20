//! Declarations: every property and shorthand of `Style`, the camelCase names, each property
//! kind's CSS rules, and that a failed declaration changes nothing. Expected styles are the
//! default with the fields set by hand.

use std::fmt::Debug;

use souple::{
    AlignContent, AlignItems, BoxSizing, Display, Edges, FlexDirection, FlexWrap, Formula, JustifyContent, Length,
    Op, Overflow, Position, Style, VarId,
};
use souple_syntax::{ErrorKind, ParseError, Units, apply_declaration};

const EM: VarId = VarId::new(0);
const SAFE_TOP: VarId = VarId::new(1);

fn units() -> Units {
    Units::new().unit("em", EM).keyword("safe-top", SAFE_TOP)
}

/// The default style with one declaration applied.
fn with(property: &str, value: &str) -> Style {
    let mut style = Style::default();
    apply(&mut style, property, value);
    style
}

fn apply(style: &mut Style, property: &str, value: &str) {
    apply_declaration(style, property, value, &units()).unwrap_or_else(|error| panic!("{property}: {value}: {error}"));
}

/// The error of a declaration on the default style, which it must leave untouched.
fn error(property: &str, value: &str) -> ParseError {
    let mut style = Style::default();
    let error = apply_declaration(&mut style, property, value, &units()).expect_err(&format!("{property}: {value}"));
    assert_eq!(style, Style::default(), "{property}: {value} failed and changed the style");
    error
}

fn fails(property: &str, value: &str) -> (ErrorKind, usize) {
    let error = error(property, value);
    (error.kind, error.offset)
}

fn expected(what: &str) -> ErrorKind {
    ErrorKind::Expected(what.to_string())
}

fn px(value: f32) -> Length {
    Length::Px(value)
}

fn edges(top: Length, right: Length, bottom: Length, left: Length) -> Edges<Length> {
    Edges { left, top, right, bottom }
}

fn check_keywords<T: PartialEq + Debug + Copy>(property: &str, field: fn(&Style) -> T, cases: &[(&str, T)]) {
    for &(value, expected) in cases {
        assert_eq!(field(&with(property, value)), expected, "{property}: {value}");
        assert_eq!(field(&with(property, &value.to_uppercase())), expected, "{property}: {value} in capitals");
    }
}

#[test]
fn keyword_properties() {
    check_keywords("display", |s| s.display, &[("flex", Display::Flex), ("none", Display::None)]);
    check_keywords("position", |s| s.position, &[("relative", Position::Relative), ("absolute", Position::Absolute)]);
    check_keywords(
        "box-sizing",
        |s| s.box_sizing,
        &[("content-box", BoxSizing::ContentBox), ("border-box", BoxSizing::BorderBox)],
    );
    check_keywords(
        "overflow",
        |s| s.overflow,
        &[("visible", Overflow::Visible), ("hidden", Overflow::Hidden), ("scroll", Overflow::Scroll)],
    );
    check_keywords(
        "flex-direction",
        |s| s.flex_direction,
        &[
            ("row", FlexDirection::Row),
            ("row-reverse", FlexDirection::RowReverse),
            ("column", FlexDirection::Column),
            ("column-reverse", FlexDirection::ColumnReverse),
        ],
    );
    check_keywords(
        "flex-wrap",
        |s| s.flex_wrap,
        &[("nowrap", FlexWrap::NoWrap), ("wrap", FlexWrap::Wrap), ("wrap-reverse", FlexWrap::WrapReverse)],
    );
    check_keywords(
        "justify-content",
        |s| s.justify_content,
        &[
            ("flex-start", JustifyContent::FlexStart),
            ("flex-end", JustifyContent::FlexEnd),
            ("center", JustifyContent::Center),
            ("space-between", JustifyContent::SpaceBetween),
            ("space-around", JustifyContent::SpaceAround),
            ("space-evenly", JustifyContent::SpaceEvenly),
        ],
    );
    let align_items = [
        ("stretch", AlignItems::Stretch),
        ("flex-start", AlignItems::FlexStart),
        ("flex-end", AlignItems::FlexEnd),
        ("center", AlignItems::Center),
        ("baseline", AlignItems::Baseline),
    ];
    check_keywords("align-items", |s| s.align_items, &align_items);
    let align_self: Vec<(&str, Option<AlignItems>)> =
        align_items.iter().map(|&(name, value)| (name, Some(value))).chain([("auto", None)]).collect();
    check_keywords("align-self", |s| s.align_self, &align_self);
    check_keywords(
        "align-content",
        |s| s.align_content,
        &[
            ("stretch", AlignContent::Stretch),
            ("flex-start", AlignContent::FlexStart),
            ("flex-end", AlignContent::FlexEnd),
            ("center", AlignContent::Center),
            ("space-between", AlignContent::SpaceBetween),
            ("space-around", AlignContent::SpaceAround),
            ("space-evenly", AlignContent::SpaceEvenly),
        ],
    );
    // `align-self: auto` puts the parent's alignment back.
    let mut style = with("align-self", "center");
    apply(&mut style, "align-self", "auto");
    assert_eq!(style.align_self, None);
}

#[test]
fn keyword_errors() {
    assert_eq!(fails("display", "grid"), (expected("one of `flex`, `none`"), 0));
    assert_eq!(fails("display", "  block"), (expected("one of `flex`, `none`"), 2));
    assert_eq!(fails("display", ""), (ErrorKind::UnexpectedEnd, 0));
    assert_eq!(fails("display", "flex none"), (expected("the end of the value"), 5));
    assert_eq!(fails("position", "10px"), (expected("one of `relative`, `absolute`"), 0));
    let align = "one of `stretch`, `flex-start`, `flex-end`, `center`, `baseline`";
    assert_eq!(fails("align-items", "auto"), (expected(align), 0), "`auto` is `align-self`'s only");
    assert_eq!(fails("flex-wrap", "#"), (ErrorKind::UnexpectedChar('#'), 0));
}

#[test]
fn number_properties() {
    assert_eq!(with("flex-grow", "2").flex_grow, 2.0);
    assert_eq!(with("flex-grow", ".5").flex_grow, 0.5);
    assert_eq!(with("flex-shrink", "0").flex_shrink, 0.0);
    assert_eq!(with("flex-shrink", "3").flex_shrink, 3.0);
    assert_eq!(fails("flex-grow", "-1"), (ErrorKind::Negative, 0));
    assert_eq!(fails("flex-shrink", " -0.5"), (ErrorKind::Negative, 1));
    assert_eq!(fails("flex-grow", "1px"), (expected("a number"), 0));
    assert_eq!(fails("flex-grow", "auto"), (expected("a number"), 0));
    assert_eq!(fails("flex-grow", "1 2"), (expected("the end of the value"), 2));
    assert_eq!(fails("flex-shrink", ""), (ErrorKind::UnexpectedEnd, 0));
}

#[test]
fn aspect_ratio() {
    assert_eq!(with("aspect-ratio", "16 / 9").aspect_ratio, Some(16.0 / 9.0));
    assert_eq!(with("aspect-ratio", "16/9").aspect_ratio, Some(16.0 / 9.0));
    assert_eq!(with("aspect-ratio", "1.5").aspect_ratio, Some(1.5));
    let mut style = with("aspect-ratio", "2");
    apply(&mut style, "aspect-ratio", "AUTO");
    assert_eq!(style.aspect_ratio, None);
    // A ratio with a zero is degenerate: CSS makes it `auto`.
    for degenerate in ["0", "1 / 0", "0 / 1"] {
        let mut style = with("aspect-ratio", "2");
        apply(&mut style, "aspect-ratio", degenerate);
        assert_eq!(style.aspect_ratio, None, "{degenerate}");
    }
    assert_eq!(fails("aspect-ratio", "-1"), (ErrorKind::Negative, 0));
    assert_eq!(fails("aspect-ratio", "16 / -9"), (ErrorKind::Negative, 5));
    assert_eq!(fails("aspect-ratio", "16px"), (expected("a number"), 0));
    assert_eq!(fails("aspect-ratio", "16 /"), (ErrorKind::UnexpectedEnd, 4));
    assert_eq!(fails("aspect-ratio", "16 / 9 / 2"), (expected("the end of the value"), 7));
    assert_eq!(fails("aspect-ratio", "16 9"), (expected("the end of the value"), 3));
}

/// What a length property takes besides a length, as the tests expect it (CSS).
#[derive(Clone, Copy, PartialEq, Debug)]
enum Kind {
    /// `auto`, nothing negative.
    Size,
    /// `none` = Auto, nothing negative.
    Max,
    /// `auto`, negative values.
    Offset,
    /// nothing but a length, nothing negative.
    Spacing,
    /// `normal` = 0, nothing negative.
    Gap,
}

type Field = fn(&mut Style) -> &mut Length;

/// Every length longhand: its name, its field, its kind.
const LENGTHS: &[(&str, Field, Kind)] = &[
    ("flex-basis", |s| &mut s.flex_basis, Kind::Size),
    ("width", |s| &mut s.size.width, Kind::Size),
    ("height", |s| &mut s.size.height, Kind::Size),
    ("min-width", |s| &mut s.min_size.width, Kind::Size),
    ("min-height", |s| &mut s.min_size.height, Kind::Size),
    ("max-width", |s| &mut s.max_size.width, Kind::Max),
    ("max-height", |s| &mut s.max_size.height, Kind::Max),
    ("margin-top", |s| &mut s.margin.top, Kind::Offset),
    ("margin-right", |s| &mut s.margin.right, Kind::Offset),
    ("margin-bottom", |s| &mut s.margin.bottom, Kind::Offset),
    ("margin-left", |s| &mut s.margin.left, Kind::Offset),
    ("padding-top", |s| &mut s.padding.top, Kind::Spacing),
    ("padding-right", |s| &mut s.padding.right, Kind::Spacing),
    ("padding-bottom", |s| &mut s.padding.bottom, Kind::Spacing),
    ("padding-left", |s| &mut s.padding.left, Kind::Spacing),
    ("border-top-width", |s| &mut s.border.top, Kind::Spacing),
    ("border-right-width", |s| &mut s.border.right, Kind::Spacing),
    ("border-bottom-width", |s| &mut s.border.bottom, Kind::Spacing),
    ("border-left-width", |s| &mut s.border.left, Kind::Spacing),
    ("top", |s| &mut s.inset.top, Kind::Offset),
    ("right", |s| &mut s.inset.right, Kind::Offset),
    ("bottom", |s| &mut s.inset.bottom, Kind::Offset),
    ("left", |s| &mut s.inset.left, Kind::Offset),
    ("row-gap", |s| &mut s.row_gap, Kind::Gap),
    ("column-gap", |s| &mut s.column_gap, Kind::Gap),
];

/// The default style with one field set: what a declaration of that one property must give.
fn default_with(field: Field, length: Length) -> Style {
    let mut style = Style::default();
    *field(&mut style) = length;
    style
}

#[test]
fn length_properties_set_their_field_and_nothing_else() {
    let formula = Formula::build(&[Op::Percent(100.0), Op::Var(EM, 1.0), Op::Sub]).unwrap();
    for &(name, field, _) in LENGTHS {
        assert_eq!(with(name, "10px"), default_with(field, px(10.0)), "{name}");
        assert_eq!(with(name, "12"), default_with(field, px(12.0)), "{name}: a bare number is px");
        assert_eq!(with(name, "25%"), default_with(field, Length::Percent(25.0)), "{name}");
        assert_eq!(with(name, "2em"), default_with(field, Length::Var(EM, 2.0)), "{name}");
        assert_eq!(with(name, "safe-top"), default_with(field, Length::Var(SAFE_TOP, 1.0)), "{name}");
        assert_eq!(with(name, "calc(100% - 1em)"), default_with(field, formula.clone()), "{name}");
        assert_eq!(with(name, "calc(1px + 2px)"), default_with(field, px(3.0)), "{name}");
        assert_eq!(fails(name, "10px 20px"), (expected("the end of the value"), 5), "{name}");
        assert_eq!(fails(name, "banana"), (ErrorKind::UnknownKeyword("banana".into()), 0), "{name}");
        assert_eq!(fails(name, "calc(1px + 2)"), (ErrorKind::Formula(souple::FormulaError::Type), 0), "{name}");
    }
}

#[test]
fn each_length_property_takes_its_css_keywords() {
    for &(name, field, kind) in LENGTHS {
        let auto = apply_declaration(&mut Style::default(), name, "auto", &units());
        match kind {
            Kind::Size | Kind::Offset => {
                let mut style = with(name, "5px");
                apply(&mut style, name, "auto");
                assert_eq!(style, default_with(field, Length::Auto), "{name}: auto");
            }
            Kind::Max => assert_eq!(auto.unwrap_err().kind, expected("`none` or a length"), "{name}"),
            Kind::Gap => assert_eq!(auto.unwrap_err().kind, expected("`normal` or a length"), "{name}"),
            Kind::Spacing => assert_eq!(auto.unwrap_err().kind, expected("a length other than `auto`"), "{name}"),
        }
        let none = apply_declaration(&mut Style::default(), name, "NONE", &units());
        if kind == Kind::Max {
            let mut style = with(name, "5px");
            apply(&mut style, name, "NONE");
            assert_eq!(style, Style::default(), "{name}: none is no maximum, Length::Auto");
        } else {
            assert_eq!(none.unwrap_err().kind, ErrorKind::UnknownKeyword("NONE".into()), "{name}");
        }
        let normal = apply_declaration(&mut Style::default(), name, "normal", &units());
        if kind == Kind::Gap {
            let mut style = with(name, "5px");
            apply(&mut style, name, "normal");
            assert_eq!(style, default_with(field, Length::ZERO), "{name}: normal is 0 in a flex container");
        } else {
            assert_eq!(normal.unwrap_err().kind, ErrorKind::UnknownKeyword("normal".into()), "{name}");
        }
    }
}

#[test]
fn only_offsets_take_negative_values() {
    for &(name, field, kind) in LENGTHS {
        for (value, length) in [("-5px", px(-5.0)), ("-10%", Length::Percent(-10.0)), ("-2em", Length::Var(EM, -2.0))] {
            if kind == Kind::Offset {
                assert_eq!(with(name, value), default_with(field, length), "{name}: {value}");
            } else {
                assert_eq!(fails(name, value), (ErrorKind::Negative, 0), "{name}: {value}");
            }
        }
        // CSS judges a math function by what it computes to, and clamps it then: the engine's job.
        assert_eq!(with(name, "calc(-5px)"), default_with(field, px(-5.0)), "{name}");
        assert_eq!(with(name, "-0"), default_with(field, px(-0.0)), "{name}: -0 is no negative");
    }
}

#[test]
fn box_shorthands() {
    let edges_px = |top, right, bottom, left| edges(px(top), px(right), px(bottom), px(left));
    type Edge = fn(&Style) -> Edges<Length>;
    let shorthands: [(&str, Edge); 4] = [
        ("margin", |s| s.margin.clone()),
        ("padding", |s| s.padding.clone()),
        ("border-width", |s| s.border.clone()),
        ("inset", |s| s.inset.clone()),
    ];
    for (name, field) in shorthands {
        assert_eq!(field(&with(name, "1px")), edges_px(1.0, 1.0, 1.0, 1.0), "{name}");
        assert_eq!(field(&with(name, "1px 2px")), edges_px(1.0, 2.0, 1.0, 2.0), "{name}");
        assert_eq!(field(&with(name, "1px 2px 3px")), edges_px(1.0, 2.0, 3.0, 2.0), "{name}");
        assert_eq!(field(&with(name, "1px 2px 3px 4px")), edges_px(1.0, 2.0, 3.0, 4.0), "{name}");
        // Components are split by the tokens, so a math function's spaces stay inside it.
        assert_eq!(field(&with(name, "calc(1px + 2px) 4px")), edges_px(3.0, 4.0, 3.0, 4.0), "{name}");
        let formula = Formula::build(&[Op::Percent(50.0), Op::Var(EM, 1.0), Op::Px(2.0), Op::Max(2), Op::Sub]).unwrap();
        assert_eq!(
            field(&with(name, "calc(50% - max(1em, 2px)) 0")),
            edges(formula.clone(), px(0.0), formula, px(0.0)),
            "{name}"
        );
        assert_eq!(fails(name, "1px 2px 3px 4px 5px"), (expected("the end of the value"), 16), "{name}");
        assert_eq!(fails(name, "1px, 2px"), (expected("a length"), 3), "{name}");
        assert_eq!(fails(name, ""), (ErrorKind::UnexpectedEnd, 0), "{name}");
        assert_eq!(fails(name, "1px calc(1px"), (ErrorKind::UnexpectedEnd, 12), "{name}");
    }
    // The longhands' rules hold in the shorthands.
    assert_eq!(with("margin", "0 auto").margin, edges(px(0.0), Length::Auto, px(0.0), Length::Auto));
    assert_eq!(with("inset", "auto -1px").inset, edges(Length::Auto, px(-1.0), Length::Auto, px(-1.0)));
    assert_eq!(with("margin", "-1px").margin, Edges::all(px(-1.0)));
    assert_eq!(fails("padding", "1px auto"), (expected("a length other than `auto`"), 4));
    assert_eq!(fails("border-width", "1px 2px -3px"), (ErrorKind::Negative, 8));
}

#[test]
fn gap() {
    let style = with("gap", "4px");
    assert_eq!((style.row_gap, style.column_gap), (px(4.0), px(4.0)));
    let style = with("gap", "4px 10%");
    assert_eq!((style.row_gap, style.column_gap), (px(4.0), Length::Percent(10.0)));
    let style = with("gap", "normal 1em");
    assert_eq!((style.row_gap, style.column_gap), (Length::ZERO, Length::Var(EM, 1.0)));
    let style = with("gap", "calc(1px + 1px) 3px");
    assert_eq!((style.row_gap, style.column_gap), (px(2.0), px(3.0)));
    assert_eq!(fails("gap", "1px 2px 3px"), (expected("the end of the value"), 8));
    assert_eq!(fails("gap", "auto"), (expected("`normal` or a length"), 0));
    assert_eq!(fails("gap", "1px -2px"), (ErrorKind::Negative, 4));
    assert_eq!(fails("gap", ""), (ErrorKind::UnexpectedEnd, 0));
}

fn flex(value: &str) -> (f32, f32, Length) {
    let style = with("flex", value);
    (style.flex_grow, style.flex_shrink, style.flex_basis)
}

#[test]
fn flex_shorthand() {
    let zero_percent = Length::Percent(0.0);
    assert_eq!(flex("none"), (0.0, 0.0, Length::Auto));
    assert_eq!(flex("NONE"), (0.0, 0.0, Length::Auto));
    assert_eq!(flex("auto"), (1.0, 1.0, Length::Auto));
    // CSS: `flex: <grow>` is `<grow> 1 0%`.
    assert_eq!(flex("1"), (1.0, 1.0, zero_percent.clone()));
    assert_eq!(flex("2"), (2.0, 1.0, zero_percent.clone()));
    assert_eq!(flex("0"), (0.0, 1.0, zero_percent.clone()));
    assert_eq!(flex("2 3"), (2.0, 3.0, zero_percent.clone()));
    assert_eq!(flex("1 0"), (1.0, 0.0, zero_percent.clone()));
    assert_eq!(flex("2 10px"), (2.0, 1.0, px(10.0)));
    assert_eq!(flex("2 auto"), (2.0, 1.0, Length::Auto));
    assert_eq!(flex("1 50%"), (1.0, 1.0, Length::Percent(50.0)));
    assert_eq!(flex("2 3 10px"), (2.0, 3.0, px(10.0)));
    assert_eq!(flex("1 1 2em"), (1.0, 1.0, Length::Var(EM, 2.0)));
    // A bare number after two factors is the basis.
    assert_eq!(flex("2 3 0"), (2.0, 3.0, px(0.0)));
    assert_eq!(flex("2 3 12"), (2.0, 3.0, px(12.0)));
    // The basis may come first, and alone.
    assert_eq!(flex("10px"), (1.0, 1.0, px(10.0)));
    assert_eq!(flex("10px 2"), (2.0, 1.0, px(10.0)));
    assert_eq!(flex("10px 2 3"), (2.0, 3.0, px(10.0)));
    assert_eq!(flex("auto 2"), (2.0, 1.0, Length::Auto));
    let formula = Formula::build(&[Op::Percent(100.0), Op::Var(EM, 1.0), Op::Sub]).unwrap();
    assert_eq!(flex("1 1 calc(100% - 1em)"), (1.0, 1.0, formula));

    assert_eq!(fails("flex", "1 2 3 4"), (expected("the end of the value"), 6));
    assert_eq!(fails("flex", "10px 20px"), (expected("the end of the value"), 5));
    // The factors are adjacent: no basis between them.
    assert_eq!(fails("flex", "1 10px 2"), (expected("the end of the value"), 7));
    assert_eq!(fails("flex", "none 1"), (expected("the end of the value"), 5));
    assert_eq!(fails("flex", "1 none"), (ErrorKind::UnknownKeyword("none".into()), 2));
    assert_eq!(fails("flex", "-1"), (ErrorKind::Negative, 0));
    assert_eq!(fails("flex", "1 -1"), (ErrorKind::Negative, 2));
    assert_eq!(fails("flex", "1 1 -1px"), (ErrorKind::Negative, 4));
    assert_eq!(fails("flex", ""), (ErrorKind::UnexpectedEnd, 0));
}

#[test]
fn flex_flow() {
    let flow = |value: &str| {
        // From a style with both set, so the reset of an omitted part shows.
        let mut style = with("flex-direction", "column");
        apply(&mut style, "flex-wrap", "wrap");
        apply(&mut style, "flex-flow", value);
        (style.flex_direction, style.flex_wrap)
    };
    assert_eq!(flow("row"), (FlexDirection::Row, FlexWrap::NoWrap));
    assert_eq!(flow("column-reverse"), (FlexDirection::ColumnReverse, FlexWrap::NoWrap));
    assert_eq!(flow("wrap-reverse"), (FlexDirection::Row, FlexWrap::WrapReverse));
    assert_eq!(flow("column wrap"), (FlexDirection::Column, FlexWrap::Wrap));
    assert_eq!(flow("wrap column"), (FlexDirection::Column, FlexWrap::Wrap));
    assert_eq!(flow("NOWRAP ROW-REVERSE"), (FlexDirection::RowReverse, FlexWrap::NoWrap));

    let either = expected("a `flex-direction` or a `flex-wrap` keyword, each at most once");
    assert_eq!(fails("flex-flow", "row column"), (either.clone(), 4));
    assert_eq!(fails("flex-flow", "wrap nowrap"), (either.clone(), 5));
    assert_eq!(fails("flex-flow", "diagonal"), (either.clone(), 0));
    assert_eq!(fails("flex-flow", "row 1px"), (either.clone(), 4));
    assert_eq!(fails("flex-flow", "row wrap column"), (either, 9));
    assert_eq!(fails("flex-flow", ""), (ErrorKind::UnexpectedEnd, 0));
}

/// Every property: its CSS name, its camelCase name, a value that changes the default style.
const PROPERTIES: &[(&str, &str, &str)] = &[
    ("display", "display", "none"),
    ("position", "position", "absolute"),
    ("box-sizing", "boxSizing", "border-box"),
    ("overflow", "overflow", "scroll"),
    ("flex-direction", "flexDirection", "column"),
    ("flex-wrap", "flexWrap", "wrap"),
    ("justify-content", "justifyContent", "center"),
    ("align-items", "alignItems", "center"),
    ("align-self", "alignSelf", "flex-end"),
    ("align-content", "alignContent", "space-between"),
    ("flex-grow", "flexGrow", "2"),
    ("flex-shrink", "flexShrink", "0"),
    ("flex-basis", "flexBasis", "10px"),
    ("aspect-ratio", "aspectRatio", "2"),
    ("width", "width", "7px"),
    ("height", "height", "7px"),
    ("min-width", "minWidth", "7px"),
    ("min-height", "minHeight", "7px"),
    ("max-width", "maxWidth", "7px"),
    ("max-height", "maxHeight", "7px"),
    ("margin-top", "marginTop", "7px"),
    ("margin-right", "marginRight", "7px"),
    ("margin-bottom", "marginBottom", "7px"),
    ("margin-left", "marginLeft", "7px"),
    ("padding-top", "paddingTop", "7px"),
    ("padding-right", "paddingRight", "7px"),
    ("padding-bottom", "paddingBottom", "7px"),
    ("padding-left", "paddingLeft", "7px"),
    ("border-top-width", "borderTopWidth", "7px"),
    ("border-right-width", "borderRightWidth", "7px"),
    ("border-bottom-width", "borderBottomWidth", "7px"),
    ("border-left-width", "borderLeftWidth", "7px"),
    ("top", "top", "7px"),
    ("right", "right", "7px"),
    ("bottom", "bottom", "7px"),
    ("left", "left", "7px"),
    ("row-gap", "rowGap", "7px"),
    ("column-gap", "columnGap", "7px"),
    ("margin", "margin", "1px 2px"),
    ("padding", "padding", "1px 2px"),
    ("border-width", "borderWidth", "1px 2px"),
    ("inset", "inset", "1px 2px"),
    ("gap", "gap", "3px 4px"),
    ("flex", "flex", "1"),
    ("flex-flow", "flexFlow", "column wrap"),
];

#[test]
fn camel_case_names() {
    for &(css, camel, value) in PROPERTIES {
        let style = with(css, value);
        assert_ne!(style, Style::default(), "{css}: {value} changes the style");
        assert_eq!(with(camel, value), style, "{camel}");
    }
}

#[test]
fn css_names_are_case_insensitive() {
    assert_eq!(with("Flex-Direction", "column"), with("flex-direction", "column"));
    assert_eq!(with("WIDTH", "7px"), with("width", "7px"));
    assert_eq!(with("Width", "7px"), with("width", "7px"));
    assert_eq!(with("BORDER-TOP-WIDTH", "7px"), with("border-top-width", "7px"));
    assert_eq!(with(" width ", "7px"), with("width", "7px"));
}

#[test]
fn unknown_properties() {
    // Whatever the value: an embedder may route the property elsewhere.
    assert_eq!(fails("color", "#fff"), (ErrorKind::UnknownProperty("color".into()), 0));
    assert_eq!(fails("widht", "10px"), (ErrorKind::UnknownProperty("widht".into()), 0));
    assert_eq!(fails("", "10px"), (ErrorKind::UnknownProperty("".into()), 0));
    // camelCase starts lowercase and has no hyphen; nothing else is converted.
    assert_eq!(fails("FlexDirection", "row").0, ErrorKind::UnknownProperty("FlexDirection".into()));
    assert_eq!(fails("flexdirection", "row").0, ErrorKind::UnknownProperty("flexdirection".into()));
    assert_eq!(fails("flex_direction", "row").0, ErrorKind::UnknownProperty("flex_direction".into()));
    assert_eq!(fails("flex-Direction-", "row").0, ErrorKind::UnknownProperty("flex-Direction-".into()));
    assert_eq!(error("colour", "red").to_string(), "unknown property `colour`");
}

#[test]
fn a_failed_declaration_leaves_the_style_untouched() {
    let mut style = Style::default();
    for &(property, _, value) in PROPERTIES {
        apply(&mut style, property, value);
    }
    let before = style.clone();
    let failing = [
        ("margin", "1px 2px banana"),
        ("padding", "1px 2px 3px -4px"),
        ("inset", "1px 2px 3px 4px 5px"),
        ("border-width", "1px #"),
        ("gap", "1px 2px 3px"),
        ("flex", "2 3 4px 5"),
        ("flex", "2 3 -4px"),
        ("flex-flow", "column column"),
        ("aspect-ratio", "16 / x"),
        ("width", "calc(1px + 2)"),
        ("max-width", "auto"),
        ("display", "grid"),
        ("flex-grow", "1 2"),
        ("nope", "1px"),
    ];
    for (property, value) in failing {
        assert!(apply_declaration(&mut style, property, value, &units()).is_err(), "{property}: {value}");
        assert_eq!(style, before, "{property}: {value} failed and changed the style");
    }
}
