// @generated — do not edit. Written by the Yoga importer (crates/yoga-tests/src/bin/import.rs):
//     cargo run -p souple-yoga-tests --bin import -- <path to yoga>
// From Yoga 3.2.1, tests/generated/YGDisplayTest.cpp (MIT, Copyright (c) Meta Platforms, Inc. and affiliates).

use crate::support::*;

#[test]
fn display_none() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.display = Display::None;
        s.flex_grow = 1.0;
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child1, [0.0, 0.0, 0.0, 0.0]),
    ]);
}

#[test]
fn display_none_fixed_size() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.display = Display::None;
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child1, [0.0, 0.0, 0.0, 0.0]),
    ]);
}

#[test]
fn display_none_with_margin() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.display = Display::None;
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
        s.margin.left = Length::Px(10.0);
        s.margin.top = Length::Px(10.0);
        s.margin.right = Length::Px(10.0);
        s.margin.bottom = Length::Px(10.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_grow = 1.0;
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 0.0, 0.0]),
        (root_child1, [0.0, 0.0, 100.0, 100.0]),
    ]);
}

#[test]
fn display_none_with_child() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.display = Display::None;
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
    });
    let root_child1_child0 = t.child(root_child1, "root_child1_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
        s.size.width = Length::Px(20.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 50.0, 100.0]),
        (root_child1, [0.0, 0.0, 0.0, 0.0]),
        (root_child1_child0, [0.0, 0.0, 0.0, 0.0]),
        (root_child2, [50.0, 0.0, 50.0, 100.0]),
    ]);
}

#[test]
fn display_none_with_position() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.display = Display::None;
        s.flex_grow = 1.0;
        s.inset.top = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child1, [0.0, 0.0, 0.0, 0.0]),
    ]);
}

#[test]
fn display_none_with_position_absolute() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.display = Display::None;
        s.position = Position::Absolute;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 0.0, 0.0]),
    ]);
}

#[test]
#[ignore = "not planned: display: contents"]
fn display_contents() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |_| {
        // untranslated: YGNodeStyleSetDisplay(root_child0, YGDisplayContents);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 0.0, 0.0]),
        (root_child0_child0, [0.0, 0.0, 50.0, 10.0]),
        (root_child0_child1, [50.0, 0.0, 50.0, 20.0]),
    ]);
}

#[test]
#[ignore = "not planned: display: contents"]
fn display_contents_fixed_size() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        // untranslated: YGNodeStyleSetDisplay(root_child0, YGDisplayContents);
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 0.0, 0.0]),
        (root_child0_child0, [0.0, 0.0, 50.0, 10.0]),
        (root_child0_child1, [50.0, 0.0, 50.0, 20.0]),
    ]);
}

#[test]
#[ignore = "not planned: display: contents"]
fn display_contents_with_margin() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        // untranslated: YGNodeStyleSetDisplay(root_child0, YGDisplayContents);
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
        s.margin.left = Length::Px(10.0);
        s.margin.top = Length::Px(10.0);
        s.margin.right = Length::Px(10.0);
        s.margin.bottom = Length::Px(10.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_grow = 1.0;
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 0.0, 0.0]),
        (root_child1, [0.0, 0.0, 100.0, 100.0]),
    ]);
}

#[test]
#[ignore = "not planned: display: contents"]
fn display_contents_with_padding() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        // untranslated: YGNodeStyleSetDisplay(root_child0, YGDisplayContents);
        s.padding.left = Length::Px(10.0);
        s.padding.top = Length::Px(10.0);
        s.padding.right = Length::Px(10.0);
        s.padding.bottom = Length::Px(10.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 0.0, 0.0]),
        (root_child0_child0, [0.0, 0.0, 50.0, 10.0]),
        (root_child0_child1, [50.0, 0.0, 50.0, 20.0]),
    ]);
}

#[test]
#[ignore = "not planned: display: contents"]
fn display_contents_with_position() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        // untranslated: YGNodeStyleSetDisplay(root_child0, YGDisplayContents);
        s.inset.top = Length::Px(10.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 0.0, 0.0]),
        (root_child0_child0, [0.0, 0.0, 50.0, 10.0]),
        (root_child0_child1, [50.0, 0.0, 50.0, 20.0]),
    ]);
}

#[test]
#[ignore = "not planned: display: contents"]
fn display_contents_with_position_absolute() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        // untranslated: YGNodeStyleSetDisplay(root_child0, YGDisplayContents);
        s.position = Position::Absolute;
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 0.0, 0.0]),
        (root_child0_child0, [0.0, 0.0, 50.0, 10.0]),
        (root_child0_child1, [50.0, 0.0, 50.0, 20.0]),
    ]);
}

#[test]
#[ignore = "not planned: display: contents"]
fn display_contents_nested() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |_| {
        // untranslated: YGNodeStyleSetDisplay(root_child0, YGDisplayContents);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |_| {
        // untranslated: YGNodeStyleSetDisplay(root_child0_child0, YGDisplayContents);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child0_child0_child1 = t.child(root_child0_child0, "root_child0_child0_child1", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 0.0, 0.0]),
        (root_child0_child0, [0.0, 0.0, 0.0, 0.0]),
        (root_child0_child0_child0, [0.0, 0.0, 50.0, 10.0]),
        (root_child0_child0_child1, [50.0, 0.0, 50.0, 20.0]),
    ]);
}

#[test]
#[ignore = "not planned: display: contents"]
fn display_contents_with_siblings() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
        s.size.height = Length::Px(30.0);
    });
    let root_child1 = t.child(root, "root_child1", |_| {
        // untranslated: YGNodeStyleSetDisplay(root_child1, YGDisplayContents);
    });
    let root_child1_child0 = t.child(root_child1, "root_child1_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child1_child1 = t.child(root_child1, "root_child1_child1", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
        s.size.height = Length::Px(30.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 25.0, 30.0]),
        (root_child1, [0.0, 0.0, 0.0, 0.0]),
        (root_child1_child0, [25.0, 0.0, 25.0, 10.0]),
        (root_child1_child1, [50.0, 0.0, 25.0, 20.0]),
        (root_child2, [75.0, 0.0, 25.0, 30.0]),
    ]);
}
