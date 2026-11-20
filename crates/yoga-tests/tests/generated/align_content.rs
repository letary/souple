// @generated — do not edit. Written by the Yoga importer (crates/yoga-tests/src/bin/import.rs):
//     cargo run -p souple-yoga-tests --bin import -- <path to yoga>
// From Yoga 3.2.1, tests/generated/YGAlignContentTest.cpp (MIT, Copyright (c) Meta Platforms, Inc. and affiliates).

use crate::support::*;

#[test]
fn align_content_flex_start_nowrap() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(140.0);
        s.size.height = Length::Px(120.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 140.0, 120.0]),
        (root_child0, [0.0, 0.0, 50.0, 10.0]),
        (root_child1, [50.0, 0.0, 50.0, 10.0]),
    ]);
}

#[test]
fn align_content_flex_start_wrap() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.size.width = Length::Px(140.0);
        s.size.height = Length::Px(120.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child4 = t.child(root, "root_child4", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 140.0, 120.0]),
        (root_child0, [0.0, 0.0, 50.0, 10.0]),
        (root_child1, [50.0, 0.0, 50.0, 10.0]),
        (root_child2, [0.0, 10.0, 50.0, 10.0]),
        (root_child3, [50.0, 10.0, 50.0, 10.0]),
        (root_child4, [0.0, 20.0, 50.0, 10.0]),
    ]);
}

#[test]
fn align_content_flex_start_wrap_singleline() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.size.width = Length::Px(140.0);
        s.size.height = Length::Px(120.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 140.0, 120.0]),
        (root_child0, [0.0, 0.0, 50.0, 10.0]),
        (root_child1, [50.0, 0.0, 50.0, 10.0]),
    ]);
}

#[test]
fn align_content_flex_start_wrapped_negative_space() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(320.0);
        s.size.height = Length::Px(320.0);
        s.border.left = Length::Px(60.0);
        s.border.top = Length::Px(60.0);
        s.border.right = Length::Px(60.0);
        s.border.bottom = Length::Px(60.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.justify_content = JustifyContent::Center;
        s.size.height = Length::Px(10.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.size.width = Length::Percent(80.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Percent(80.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child0_child2 = t.child(root_child0, "root_child0_child2", |s| {
        s.size.width = Length::Percent(80.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 320.0, 320.0]),
        (root_child0, [60.0, 60.0, 200.0, 10.0]),
        (root_child0_child0, [20.0, 0.0, 160.0, 20.0]),
        (root_child0_child1, [20.0, 20.0, 160.0, 20.0]),
        (root_child0_child2, [20.0, 40.0, 160.0, 20.0]),
    ]);
}

#[test]
fn align_content_flex_start_wrapped_negative_space_gap() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(320.0);
        s.size.height = Length::Px(320.0);
        s.border.left = Length::Px(60.0);
        s.border.top = Length::Px(60.0);
        s.border.right = Length::Px(60.0);
        s.border.bottom = Length::Px(60.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.justify_content = JustifyContent::Center;
        s.size.height = Length::Px(10.0);
        s.row_gap = Length::Px(10.0);
        s.column_gap = Length::Px(10.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.size.width = Length::Percent(80.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Percent(80.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child0_child2 = t.child(root_child0, "root_child0_child2", |s| {
        s.size.width = Length::Percent(80.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 320.0, 320.0]),
        (root_child0, [60.0, 60.0, 200.0, 10.0]),
        (root_child0_child0, [20.0, 0.0, 160.0, 20.0]),
        (root_child0_child1, [20.0, 30.0, 160.0, 20.0]),
        (root_child0_child2, [20.0, 60.0, 160.0, 20.0]),
    ]);
}

#[test]
fn align_content_flex_start_without_height_on_children() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_wrap = FlexWrap::Wrap;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(50.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child4 = t.child(root, "root_child4", |s| {
        s.size.width = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 50.0, 0.0]),
        (root_child1, [0.0, 0.0, 50.0, 10.0]),
        (root_child2, [0.0, 10.0, 50.0, 0.0]),
        (root_child3, [0.0, 10.0, 50.0, 10.0]),
        (root_child4, [0.0, 20.0, 50.0, 0.0]),
    ]);
}

#[test]
fn align_content_flex_start_with_flex() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_wrap = FlexWrap::Wrap;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(120.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_basis = Length::Percent(0.0);
        s.size.width = Length::Px(50.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_grow = 1.0;
        s.flex_basis = Length::Percent(0.0);
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(50.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
        s.size.width = Length::Px(50.0);
    });
    let root_child4 = t.child(root, "root_child4", |s| {
        s.size.width = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 120.0]),
        (root_child0, [0.0, 0.0, 50.0, 40.0]),
        (root_child1, [0.0, 40.0, 50.0, 40.0]),
        (root_child2, [0.0, 80.0, 50.0, 0.0]),
        (root_child3, [0.0, 80.0, 50.0, 40.0]),
        (root_child4, [0.0, 120.0, 50.0, 0.0]),
    ]);
}

#[test]
fn align_content_flex_end_nowrap() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.align_content = AlignContent::FlexEnd;
        s.size.width = Length::Px(140.0);
        s.size.height = Length::Px(120.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 140.0, 120.0]),
        (root_child0, [0.0, 0.0, 50.0, 10.0]),
        (root_child1, [50.0, 0.0, 50.0, 10.0]),
    ]);
}

#[test]
fn align_content_flex_end_wrap() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.align_content = AlignContent::FlexEnd;
        s.size.width = Length::Px(140.0);
        s.size.height = Length::Px(120.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child4 = t.child(root, "root_child4", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 140.0, 120.0]),
        (root_child0, [0.0, 90.0, 50.0, 10.0]),
        (root_child1, [50.0, 90.0, 50.0, 10.0]),
        (root_child2, [0.0, 100.0, 50.0, 10.0]),
        (root_child3, [50.0, 100.0, 50.0, 10.0]),
        (root_child4, [0.0, 110.0, 50.0, 10.0]),
    ]);
}

#[test]
fn align_content_flex_end_wrap_singleline() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.align_content = AlignContent::FlexEnd;
        s.size.width = Length::Px(140.0);
        s.size.height = Length::Px(120.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 140.0, 120.0]),
        (root_child0, [0.0, 110.0, 50.0, 10.0]),
        (root_child1, [50.0, 110.0, 50.0, 10.0]),
    ]);
}

#[test]
fn align_content_flex_end_wrapped_negative_space() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(320.0);
        s.size.height = Length::Px(320.0);
        s.border.left = Length::Px(60.0);
        s.border.top = Length::Px(60.0);
        s.border.right = Length::Px(60.0);
        s.border.bottom = Length::Px(60.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.justify_content = JustifyContent::Center;
        s.align_content = AlignContent::FlexEnd;
        s.size.height = Length::Px(10.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.size.width = Length::Percent(80.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Percent(80.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child0_child2 = t.child(root_child0, "root_child0_child2", |s| {
        s.size.width = Length::Percent(80.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 320.0, 320.0]),
        (root_child0, [60.0, 60.0, 200.0, 10.0]),
        (root_child0_child0, [20.0, -50.0, 160.0, 20.0]),
        (root_child0_child1, [20.0, -30.0, 160.0, 20.0]),
        (root_child0_child2, [20.0, -10.0, 160.0, 20.0]),
    ]);
}

#[test]
fn align_content_flex_end_wrapped_negative_space_gap() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(320.0);
        s.size.height = Length::Px(320.0);
        s.border.left = Length::Px(60.0);
        s.border.top = Length::Px(60.0);
        s.border.right = Length::Px(60.0);
        s.border.bottom = Length::Px(60.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.justify_content = JustifyContent::Center;
        s.align_content = AlignContent::FlexEnd;
        s.size.height = Length::Px(10.0);
        s.row_gap = Length::Px(10.0);
        s.column_gap = Length::Px(10.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.size.width = Length::Percent(80.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Percent(80.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child0_child2 = t.child(root_child0, "root_child0_child2", |s| {
        s.size.width = Length::Percent(80.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 320.0, 320.0]),
        (root_child0, [60.0, 60.0, 200.0, 10.0]),
        (root_child0_child0, [20.0, -70.0, 160.0, 20.0]),
        (root_child0_child1, [20.0, -40.0, 160.0, 20.0]),
        (root_child0_child2, [20.0, -10.0, 160.0, 20.0]),
    ]);
}

#[test]
fn align_content_center_nowrap() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.align_content = AlignContent::Center;
        s.size.width = Length::Px(140.0);
        s.size.height = Length::Px(120.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 140.0, 120.0]),
        (root_child0, [0.0, 0.0, 50.0, 10.0]),
        (root_child1, [50.0, 0.0, 50.0, 10.0]),
    ]);
}

#[test]
fn align_content_center_wrap() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.align_content = AlignContent::Center;
        s.size.width = Length::Px(140.0);
        s.size.height = Length::Px(120.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child4 = t.child(root, "root_child4", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 140.0, 120.0]),
        (root_child0, [0.0, 45.0, 50.0, 10.0]),
        (root_child1, [50.0, 45.0, 50.0, 10.0]),
        (root_child2, [0.0, 55.0, 50.0, 10.0]),
        (root_child3, [50.0, 55.0, 50.0, 10.0]),
        (root_child4, [0.0, 65.0, 50.0, 10.0]),
    ]);
}

#[test]
fn align_content_center_wrap_singleline() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.align_content = AlignContent::Center;
        s.size.width = Length::Px(140.0);
        s.size.height = Length::Px(120.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 140.0, 120.0]),
        (root_child0, [0.0, 55.0, 50.0, 10.0]),
        (root_child1, [50.0, 55.0, 50.0, 10.0]),
    ]);
}

#[test]
fn align_content_center_wrapped_negative_space() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(320.0);
        s.size.height = Length::Px(320.0);
        s.border.left = Length::Px(60.0);
        s.border.top = Length::Px(60.0);
        s.border.right = Length::Px(60.0);
        s.border.bottom = Length::Px(60.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.justify_content = JustifyContent::Center;
        s.align_content = AlignContent::Center;
        s.size.height = Length::Px(10.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.size.width = Length::Percent(80.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Percent(80.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child0_child2 = t.child(root_child0, "root_child0_child2", |s| {
        s.size.width = Length::Percent(80.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 320.0, 320.0]),
        (root_child0, [60.0, 60.0, 200.0, 10.0]),
        (root_child0_child0, [20.0, -25.0, 160.0, 20.0]),
        (root_child0_child1, [20.0, -5.0, 160.0, 20.0]),
        (root_child0_child2, [20.0, 15.0, 160.0, 20.0]),
    ]);
}

#[test]
fn align_content_center_wrapped_negative_space_gap() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(320.0);
        s.size.height = Length::Px(320.0);
        s.border.left = Length::Px(60.0);
        s.border.top = Length::Px(60.0);
        s.border.right = Length::Px(60.0);
        s.border.bottom = Length::Px(60.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.justify_content = JustifyContent::Center;
        s.align_content = AlignContent::Center;
        s.size.height = Length::Px(10.0);
        s.row_gap = Length::Px(10.0);
        s.column_gap = Length::Px(10.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.size.width = Length::Percent(80.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Percent(80.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child0_child2 = t.child(root_child0, "root_child0_child2", |s| {
        s.size.width = Length::Percent(80.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 320.0, 320.0]),
        (root_child0, [60.0, 60.0, 200.0, 10.0]),
        (root_child0_child0, [20.0, -35.0, 160.0, 20.0]),
        (root_child0_child1, [20.0, -5.0, 160.0, 20.0]),
        (root_child0_child2, [20.0, 25.0, 160.0, 20.0]),
    ]);
}

#[test]
fn align_content_space_between_nowrap() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.align_content = AlignContent::SpaceBetween;
        s.size.width = Length::Px(140.0);
        s.size.height = Length::Px(120.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 140.0, 120.0]),
        (root_child0, [0.0, 0.0, 50.0, 10.0]),
        (root_child1, [50.0, 0.0, 50.0, 10.0]),
    ]);
}

#[test]
fn align_content_space_between_wrap() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.align_content = AlignContent::SpaceBetween;
        s.size.width = Length::Px(140.0);
        s.size.height = Length::Px(120.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child4 = t.child(root, "root_child4", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 140.0, 120.0]),
        (root_child0, [0.0, 0.0, 50.0, 10.0]),
        (root_child1, [50.0, 0.0, 50.0, 10.0]),
        (root_child2, [0.0, 55.0, 50.0, 10.0]),
        (root_child3, [50.0, 55.0, 50.0, 10.0]),
        (root_child4, [0.0, 110.0, 50.0, 10.0]),
    ]);
}

#[test]
fn align_content_space_between_wrap_singleline() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.align_content = AlignContent::SpaceBetween;
        s.size.width = Length::Px(140.0);
        s.size.height = Length::Px(120.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 140.0, 120.0]),
        (root_child0, [0.0, 0.0, 50.0, 10.0]),
        (root_child1, [50.0, 0.0, 50.0, 10.0]),
    ]);
}

#[test]
fn align_content_space_between_wrapped_negative_space() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(320.0);
        s.size.height = Length::Px(320.0);
        s.border.left = Length::Px(60.0);
        s.border.top = Length::Px(60.0);
        s.border.right = Length::Px(60.0);
        s.border.bottom = Length::Px(60.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.justify_content = JustifyContent::Center;
        s.align_content = AlignContent::SpaceBetween;
        s.size.height = Length::Px(10.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.size.width = Length::Percent(80.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Percent(80.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child0_child2 = t.child(root_child0, "root_child0_child2", |s| {
        s.size.width = Length::Percent(80.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 320.0, 320.0]),
        (root_child0, [60.0, 60.0, 200.0, 10.0]),
        (root_child0_child0, [20.0, 0.0, 160.0, 20.0]),
        (root_child0_child1, [20.0, 20.0, 160.0, 20.0]),
        (root_child0_child2, [20.0, 40.0, 160.0, 20.0]),
    ]);
}

#[test]
fn align_content_space_between_wrapped_negative_space_row_reverse() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(320.0);
        s.size.height = Length::Px(320.0);
        s.border.left = Length::Px(60.0);
        s.border.top = Length::Px(60.0);
        s.border.right = Length::Px(60.0);
        s.border.bottom = Length::Px(60.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::RowReverse;
        s.flex_wrap = FlexWrap::Wrap;
        s.justify_content = JustifyContent::Center;
        s.align_content = AlignContent::SpaceBetween;
        s.size.height = Length::Px(10.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.size.width = Length::Percent(80.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Percent(80.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child0_child2 = t.child(root_child0, "root_child0_child2", |s| {
        s.size.width = Length::Percent(80.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 320.0, 320.0]),
        (root_child0, [60.0, 60.0, 200.0, 10.0]),
        (root_child0_child0, [20.0, 0.0, 160.0, 20.0]),
        (root_child0_child1, [20.0, 20.0, 160.0, 20.0]),
        (root_child0_child2, [20.0, 40.0, 160.0, 20.0]),
    ]);
}

#[test]
fn align_content_space_between_wrapped_negative_space_gap() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(320.0);
        s.size.height = Length::Px(320.0);
        s.border.left = Length::Px(60.0);
        s.border.top = Length::Px(60.0);
        s.border.right = Length::Px(60.0);
        s.border.bottom = Length::Px(60.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.justify_content = JustifyContent::Center;
        s.align_content = AlignContent::SpaceBetween;
        s.size.height = Length::Px(10.0);
        s.row_gap = Length::Px(10.0);
        s.column_gap = Length::Px(10.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.size.width = Length::Percent(80.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Percent(80.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child0_child2 = t.child(root_child0, "root_child0_child2", |s| {
        s.size.width = Length::Percent(80.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 320.0, 320.0]),
        (root_child0, [60.0, 60.0, 200.0, 10.0]),
        (root_child0_child0, [20.0, 0.0, 160.0, 20.0]),
        (root_child0_child1, [20.0, 30.0, 160.0, 20.0]),
        (root_child0_child2, [20.0, 60.0, 160.0, 20.0]),
    ]);
}

#[test]
fn align_content_space_around_nowrap() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.align_content = AlignContent::SpaceAround;
        s.size.width = Length::Px(140.0);
        s.size.height = Length::Px(120.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 140.0, 120.0]),
        (root_child0, [0.0, 0.0, 50.0, 10.0]),
        (root_child1, [50.0, 0.0, 50.0, 10.0]),
    ]);
}

#[test]
fn align_content_space_around_wrap() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.align_content = AlignContent::SpaceAround;
        s.size.width = Length::Px(140.0);
        s.size.height = Length::Px(120.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child4 = t.child(root, "root_child4", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 140.0, 120.0]),
        (root_child0, [0.0, 15.0, 50.0, 10.0]),
        (root_child1, [50.0, 15.0, 50.0, 10.0]),
        (root_child2, [0.0, 55.0, 50.0, 10.0]),
        (root_child3, [50.0, 55.0, 50.0, 10.0]),
        (root_child4, [0.0, 95.0, 50.0, 10.0]),
    ]);
}

#[test]
fn align_content_space_around_wrap_singleline() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.align_content = AlignContent::SpaceAround;
        s.size.width = Length::Px(140.0);
        s.size.height = Length::Px(120.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 140.0, 120.0]),
        (root_child0, [0.0, 55.0, 50.0, 10.0]),
        (root_child1, [50.0, 55.0, 50.0, 10.0]),
    ]);
}

#[test]
fn align_content_space_around_wrapped_negative_space() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(320.0);
        s.size.height = Length::Px(320.0);
        s.border.left = Length::Px(60.0);
        s.border.top = Length::Px(60.0);
        s.border.right = Length::Px(60.0);
        s.border.bottom = Length::Px(60.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.justify_content = JustifyContent::Center;
        s.align_content = AlignContent::SpaceAround;
        s.size.height = Length::Px(10.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.size.width = Length::Percent(80.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Percent(80.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child0_child2 = t.child(root_child0, "root_child0_child2", |s| {
        s.size.width = Length::Percent(80.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 320.0, 320.0]),
        (root_child0, [60.0, 60.0, 200.0, 10.0]),
        (root_child0_child0, [20.0, 0.0, 160.0, 20.0]),
        (root_child0_child1, [20.0, 20.0, 160.0, 20.0]),
        (root_child0_child2, [20.0, 40.0, 160.0, 20.0]),
    ]);
}

#[test]
fn align_content_space_around_wrapped_negative_space_row_reverse() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(320.0);
        s.size.height = Length::Px(320.0);
        s.border.left = Length::Px(60.0);
        s.border.top = Length::Px(60.0);
        s.border.right = Length::Px(60.0);
        s.border.bottom = Length::Px(60.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::RowReverse;
        s.flex_wrap = FlexWrap::Wrap;
        s.justify_content = JustifyContent::Center;
        s.align_content = AlignContent::SpaceAround;
        s.size.height = Length::Px(10.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.size.width = Length::Percent(80.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Percent(80.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child0_child2 = t.child(root_child0, "root_child0_child2", |s| {
        s.size.width = Length::Percent(80.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 320.0, 320.0]),
        (root_child0, [60.0, 60.0, 200.0, 10.0]),
        (root_child0_child0, [20.0, 0.0, 160.0, 20.0]),
        (root_child0_child1, [20.0, 20.0, 160.0, 20.0]),
        (root_child0_child2, [20.0, 40.0, 160.0, 20.0]),
    ]);
}

#[test]
fn align_content_space_around_wrapped_negative_space_gap() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(320.0);
        s.size.height = Length::Px(320.0);
        s.border.left = Length::Px(60.0);
        s.border.top = Length::Px(60.0);
        s.border.right = Length::Px(60.0);
        s.border.bottom = Length::Px(60.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.justify_content = JustifyContent::Center;
        s.align_content = AlignContent::SpaceAround;
        s.size.height = Length::Px(10.0);
        s.row_gap = Length::Px(10.0);
        s.column_gap = Length::Px(10.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.size.width = Length::Percent(80.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Percent(80.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child0_child2 = t.child(root_child0, "root_child0_child2", |s| {
        s.size.width = Length::Percent(80.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 320.0, 320.0]),
        (root_child0, [60.0, 60.0, 200.0, 10.0]),
        (root_child0_child0, [20.0, 0.0, 160.0, 20.0]),
        (root_child0_child1, [20.0, 30.0, 160.0, 20.0]),
        (root_child0_child2, [20.0, 60.0, 160.0, 20.0]),
    ]);
}

#[test]
fn align_content_space_evenly_nowrap() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.align_content = AlignContent::SpaceEvenly;
        s.size.width = Length::Px(140.0);
        s.size.height = Length::Px(120.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 140.0, 120.0]),
        (root_child0, [0.0, 0.0, 50.0, 10.0]),
        (root_child1, [50.0, 0.0, 50.0, 10.0]),
    ]);
}

#[test]
fn align_content_space_evenly_wrap() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.align_content = AlignContent::SpaceEvenly;
        s.size.width = Length::Px(140.0);
        s.size.height = Length::Px(120.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child4 = t.child(root, "root_child4", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 140.0, 120.0]),
        (root_child0, [0.0, 23.0, 50.0, 10.0]),
        (root_child1, [50.0, 23.0, 50.0, 10.0]),
        (root_child2, [0.0, 55.0, 50.0, 10.0]),
        (root_child3, [50.0, 55.0, 50.0, 10.0]),
        (root_child4, [0.0, 88.0, 50.0, 10.0]),
    ]);
}

#[test]
fn align_content_space_evenly_wrap_singleline() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.align_content = AlignContent::SpaceEvenly;
        s.size.width = Length::Px(140.0);
        s.size.height = Length::Px(120.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 140.0, 120.0]),
        (root_child0, [0.0, 55.0, 50.0, 10.0]),
        (root_child1, [50.0, 55.0, 50.0, 10.0]),
    ]);
}

#[test]
fn align_content_space_evenly_wrapped_negative_space() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(320.0);
        s.size.height = Length::Px(320.0);
        s.border.left = Length::Px(60.0);
        s.border.top = Length::Px(60.0);
        s.border.right = Length::Px(60.0);
        s.border.bottom = Length::Px(60.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.justify_content = JustifyContent::Center;
        s.align_content = AlignContent::SpaceEvenly;
        s.size.height = Length::Px(10.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.size.width = Length::Percent(80.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Percent(80.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child0_child2 = t.child(root_child0, "root_child0_child2", |s| {
        s.size.width = Length::Percent(80.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 320.0, 320.0]),
        (root_child0, [60.0, 60.0, 200.0, 10.0]),
        (root_child0_child0, [20.0, 0.0, 160.0, 20.0]),
        (root_child0_child1, [20.0, 20.0, 160.0, 20.0]),
        (root_child0_child2, [20.0, 40.0, 160.0, 20.0]),
    ]);
}

#[test]
fn align_content_space_evenly_wrapped_negative_space_gap() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(320.0);
        s.size.height = Length::Px(320.0);
        s.border.left = Length::Px(60.0);
        s.border.top = Length::Px(60.0);
        s.border.right = Length::Px(60.0);
        s.border.bottom = Length::Px(60.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.justify_content = JustifyContent::Center;
        s.align_content = AlignContent::SpaceEvenly;
        s.size.height = Length::Px(10.0);
        s.row_gap = Length::Px(10.0);
        s.column_gap = Length::Px(10.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.size.width = Length::Percent(80.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Percent(80.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child0_child2 = t.child(root_child0, "root_child0_child2", |s| {
        s.size.width = Length::Percent(80.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 320.0, 320.0]),
        (root_child0, [60.0, 60.0, 200.0, 10.0]),
        (root_child0_child0, [20.0, 0.0, 160.0, 20.0]),
        (root_child0_child1, [20.0, 30.0, 160.0, 20.0]),
        (root_child0_child2, [20.0, 60.0, 160.0, 20.0]),
    ]);
}

#[test]
fn align_content_stretch() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_wrap = FlexWrap::Wrap;
        s.align_content = AlignContent::Stretch;
        s.size.width = Length::Px(150.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(50.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.size.width = Length::Px(50.0);
    });
    let root_child4 = t.child(root, "root_child4", |s| {
        s.size.width = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 150.0, 100.0]),
        (root_child0, [0.0, 0.0, 50.0, 0.0]),
        (root_child1, [0.0, 0.0, 50.0, 0.0]),
        (root_child2, [0.0, 0.0, 50.0, 0.0]),
        (root_child3, [0.0, 0.0, 50.0, 0.0]),
        (root_child4, [0.0, 0.0, 50.0, 0.0]),
    ]);
}

#[test]
fn align_content_stretch_row() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.align_content = AlignContent::Stretch;
        s.size.width = Length::Px(150.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(50.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.size.width = Length::Px(50.0);
    });
    let root_child4 = t.child(root, "root_child4", |s| {
        s.size.width = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 150.0, 100.0]),
        (root_child0, [0.0, 0.0, 50.0, 50.0]),
        (root_child1, [50.0, 0.0, 50.0, 50.0]),
        (root_child2, [100.0, 0.0, 50.0, 50.0]),
        (root_child3, [0.0, 50.0, 50.0, 50.0]),
        (root_child4, [50.0, 50.0, 50.0, 50.0]),
    ]);
}

#[test]
fn align_content_stretch_row_with_children() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.align_content = AlignContent::Stretch;
        s.size.width = Length::Px(150.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(50.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.size.width = Length::Px(50.0);
    });
    let root_child4 = t.child(root, "root_child4", |s| {
        s.size.width = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 150.0, 100.0]),
        (root_child0, [0.0, 0.0, 50.0, 50.0]),
        (root_child0_child0, [0.0, 0.0, 50.0, 50.0]),
        (root_child1, [50.0, 0.0, 50.0, 50.0]),
        (root_child2, [100.0, 0.0, 50.0, 50.0]),
        (root_child3, [0.0, 50.0, 50.0, 50.0]),
        (root_child4, [50.0, 50.0, 50.0, 50.0]),
    ]);
}

#[test]
fn align_content_stretch_row_with_flex() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.align_content = AlignContent::Stretch;
        s.size.width = Length::Px(150.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
        s.size.width = Length::Px(50.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(50.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
        s.size.width = Length::Px(50.0);
    });
    let root_child4 = t.child(root, "root_child4", |s| {
        s.size.width = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 150.0, 100.0]),
        (root_child0, [0.0, 0.0, 50.0, 100.0]),
        (root_child1, [50.0, 0.0, 0.0, 100.0]),
        (root_child2, [50.0, 0.0, 50.0, 100.0]),
        (root_child3, [100.0, 0.0, 0.0, 100.0]),
        (root_child4, [100.0, 0.0, 50.0, 100.0]),
    ]);
}

#[test]
fn align_content_stretch_row_with_flex_no_shrink() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.align_content = AlignContent::Stretch;
        s.size.width = Length::Px(150.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
        s.size.width = Length::Px(50.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(50.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.flex_grow = 1.0;
        s.flex_basis = Length::Percent(0.0);
        s.size.width = Length::Px(50.0);
    });
    let root_child4 = t.child(root, "root_child4", |s| {
        s.size.width = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 150.0, 100.0]),
        (root_child0, [0.0, 0.0, 50.0, 100.0]),
        (root_child1, [50.0, 0.0, 0.0, 100.0]),
        (root_child2, [50.0, 0.0, 50.0, 100.0]),
        (root_child3, [100.0, 0.0, 0.0, 100.0]),
        (root_child4, [100.0, 0.0, 50.0, 100.0]),
    ]);
}

#[test]
fn align_content_stretch_row_with_margin() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.align_content = AlignContent::Stretch;
        s.size.width = Length::Px(150.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.margin.left = Length::Px(10.0);
        s.margin.top = Length::Px(10.0);
        s.margin.right = Length::Px(10.0);
        s.margin.bottom = Length::Px(10.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(50.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.size.width = Length::Px(50.0);
        s.margin.left = Length::Px(10.0);
        s.margin.top = Length::Px(10.0);
        s.margin.right = Length::Px(10.0);
        s.margin.bottom = Length::Px(10.0);
    });
    let root_child4 = t.child(root, "root_child4", |s| {
        s.size.width = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 150.0, 100.0]),
        (root_child0, [0.0, 0.0, 50.0, 40.0]),
        (root_child1, [60.0, 10.0, 50.0, 20.0]),
        (root_child2, [0.0, 40.0, 50.0, 40.0]),
        (root_child3, [60.0, 50.0, 50.0, 20.0]),
        (root_child4, [0.0, 80.0, 50.0, 20.0]),
    ]);
}

#[test]
fn align_content_stretch_row_with_padding() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.align_content = AlignContent::Stretch;
        s.size.width = Length::Px(150.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.padding.left = Length::Px(10.0);
        s.padding.top = Length::Px(10.0);
        s.padding.right = Length::Px(10.0);
        s.padding.bottom = Length::Px(10.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(50.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.size.width = Length::Px(50.0);
        s.padding.left = Length::Px(10.0);
        s.padding.top = Length::Px(10.0);
        s.padding.right = Length::Px(10.0);
        s.padding.bottom = Length::Px(10.0);
    });
    let root_child4 = t.child(root, "root_child4", |s| {
        s.size.width = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 150.0, 100.0]),
        (root_child0, [0.0, 0.0, 50.0, 50.0]),
        (root_child1, [50.0, 0.0, 50.0, 50.0]),
        (root_child2, [100.0, 0.0, 50.0, 50.0]),
        (root_child3, [0.0, 50.0, 50.0, 50.0]),
        (root_child4, [50.0, 50.0, 50.0, 50.0]),
    ]);
}

#[test]
fn align_content_stretch_row_with_single_row() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.align_content = AlignContent::Stretch;
        s.size.width = Length::Px(150.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 150.0, 100.0]),
        (root_child0, [0.0, 0.0, 50.0, 100.0]),
        (root_child1, [50.0, 0.0, 50.0, 100.0]),
    ]);
}

#[test]
fn align_content_stretch_row_with_fixed_height() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.align_content = AlignContent::Stretch;
        s.size.width = Length::Px(150.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(60.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(50.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.size.width = Length::Px(50.0);
    });
    let root_child4 = t.child(root, "root_child4", |s| {
        s.size.width = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 150.0, 100.0]),
        (root_child0, [0.0, 0.0, 50.0, 80.0]),
        (root_child1, [50.0, 0.0, 50.0, 60.0]),
        (root_child2, [100.0, 0.0, 50.0, 80.0]),
        (root_child3, [0.0, 80.0, 50.0, 20.0]),
        (root_child4, [50.0, 80.0, 50.0, 20.0]),
    ]);
}

#[test]
fn align_content_stretch_row_with_max_height() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.align_content = AlignContent::Stretch;
        s.size.width = Length::Px(150.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.max_size.height = Length::Px(20.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(50.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.size.width = Length::Px(50.0);
    });
    let root_child4 = t.child(root, "root_child4", |s| {
        s.size.width = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 150.0, 100.0]),
        (root_child0, [0.0, 0.0, 50.0, 50.0]),
        (root_child1, [50.0, 0.0, 50.0, 20.0]),
        (root_child2, [100.0, 0.0, 50.0, 50.0]),
        (root_child3, [0.0, 50.0, 50.0, 50.0]),
        (root_child4, [50.0, 50.0, 50.0, 50.0]),
    ]);
}

#[test]
fn align_content_stretch_row_with_min_height() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.align_content = AlignContent::Stretch;
        s.size.width = Length::Px(150.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.min_size.height = Length::Px(80.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(50.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.size.width = Length::Px(50.0);
    });
    let root_child4 = t.child(root, "root_child4", |s| {
        s.size.width = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 150.0, 100.0]),
        (root_child0, [0.0, 0.0, 50.0, 90.0]),
        (root_child1, [50.0, 0.0, 50.0, 90.0]),
        (root_child2, [100.0, 0.0, 50.0, 90.0]),
        (root_child3, [0.0, 90.0, 50.0, 10.0]),
        (root_child4, [50.0, 90.0, 50.0, 10.0]),
    ]);
}

#[test]
fn align_content_stretch_column() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_wrap = FlexWrap::Wrap;
        s.align_content = AlignContent::Stretch;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(150.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.height = Length::Px(50.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
        s.size.height = Length::Px(50.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.height = Length::Px(50.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.size.height = Length::Px(50.0);
    });
    let root_child4 = t.child(root, "root_child4", |s| {
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 150.0]),
        (root_child0, [0.0, 0.0, 50.0, 50.0]),
        (root_child0_child0, [0.0, 0.0, 50.0, 50.0]),
        (root_child1, [0.0, 50.0, 50.0, 0.0]),
        (root_child2, [0.0, 50.0, 50.0, 50.0]),
        (root_child3, [0.0, 100.0, 50.0, 50.0]),
        (root_child4, [50.0, 0.0, 50.0, 50.0]),
    ]);
}

#[test]
fn align_content_stretch_is_not_overriding_align_items() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.align_content = AlignContent::Stretch;
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::Row;
        s.align_items = AlignItems::Center;
        s.align_content = AlignContent::Stretch;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.align_content = AlignContent::Stretch;
        s.size.width = Length::Px(10.0);
        s.size.height = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0, [0.0, 45.0, 10.0, 10.0]),
    ]);
}

#[test]
fn align_content_stretch_with_min_cross_axis() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.align_content = AlignContent::Stretch;
        s.size.width = Length::Px(500.0);
        s.min_size.height = Length::Px(500.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(400.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(400.0);
        s.size.height = Length::Px(200.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 500.0, 500.0]),
        (root_child0, [0.0, 0.0, 400.0, 200.0]),
        (root_child1, [0.0, 250.0, 400.0, 200.0]),
    ]);
}

#[test]
fn align_content_stretch_with_max_cross_axis() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.align_content = AlignContent::Stretch;
        s.size.width = Length::Px(500.0);
        s.max_size.height = Length::Px(500.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(400.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(400.0);
        s.size.height = Length::Px(200.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 500.0, 400.0]),
        (root_child0, [0.0, 0.0, 400.0, 200.0]),
        (root_child1, [0.0, 200.0, 400.0, 200.0]),
    ]);
}

#[test]
fn align_content_stretch_with_max_cross_axis_and_border_padding() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.align_content = AlignContent::Stretch;
        s.size.width = Length::Px(500.0);
        s.max_size.height = Length::Px(500.0);
        s.padding.left = Length::Px(2.0);
        s.padding.top = Length::Px(2.0);
        s.padding.right = Length::Px(2.0);
        s.padding.bottom = Length::Px(2.0);
        s.border.left = Length::Px(5.0);
        s.border.top = Length::Px(5.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(5.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(400.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(400.0);
        s.size.height = Length::Px(200.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 500.0, 414.0]),
        (root_child0, [7.0, 7.0, 400.0, 200.0]),
        (root_child1, [7.0, 207.0, 400.0, 200.0]),
    ]);
}

#[test]
fn align_content_space_evenly_with_min_cross_axis() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.align_content = AlignContent::SpaceEvenly;
        s.size.width = Length::Px(500.0);
        s.min_size.height = Length::Px(500.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(400.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(400.0);
        s.size.height = Length::Px(200.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 500.0, 500.0]),
        (root_child0, [0.0, 33.0, 400.0, 200.0]),
        (root_child1, [0.0, 267.0, 400.0, 200.0]),
    ]);
}

#[test]
fn align_content_space_evenly_with_max_cross_axis() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.align_content = AlignContent::SpaceEvenly;
        s.size.width = Length::Px(500.0);
        s.max_size.height = Length::Px(500.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(400.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(400.0);
        s.size.height = Length::Px(200.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 500.0, 400.0]),
        (root_child0, [0.0, 0.0, 400.0, 200.0]),
        (root_child1, [0.0, 200.0, 400.0, 200.0]),
    ]);
}

#[test]
fn align_content_space_evenly_with_max_cross_axis_violated() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.align_content = AlignContent::SpaceEvenly;
        s.size.width = Length::Px(500.0);
        s.max_size.height = Length::Px(300.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(400.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(400.0);
        s.size.height = Length::Px(200.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 500.0, 300.0]),
        (root_child0, [0.0, 0.0, 400.0, 200.0]),
        (root_child1, [0.0, 200.0, 400.0, 200.0]),
    ]);
}

#[test]
fn align_content_space_evenly_with_max_cross_axis_violated_padding_and_border() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.align_content = AlignContent::SpaceEvenly;
        s.size.width = Length::Px(500.0);
        s.max_size.height = Length::Px(300.0);
        s.padding.left = Length::Px(2.0);
        s.padding.top = Length::Px(2.0);
        s.padding.right = Length::Px(2.0);
        s.padding.bottom = Length::Px(2.0);
        s.border.left = Length::Px(5.0);
        s.border.top = Length::Px(5.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(5.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(400.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(400.0);
        s.size.height = Length::Px(200.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 500.0, 300.0]),
        (root_child0, [7.0, 7.0, 400.0, 200.0]),
        (root_child1, [7.0, 207.0, 400.0, 200.0]),
    ]);
}

#[test]
fn align_content_space_around_and_align_items_flex_end_with_flex_wrap() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.align_items = AlignItems::FlexEnd;
        s.align_content = AlignContent::SpaceAround;
        s.size.width = Length::Px(300.0);
        s.size.height = Length::Px(300.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(150.0);
        s.size.height = Length::Px(50.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(120.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(120.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 300.0, 300.0]),
        (root_child0, [0.0, 88.0, 150.0, 50.0]),
        (root_child1, [150.0, 38.0, 120.0, 100.0]),
        (root_child2, [0.0, 213.0, 120.0, 50.0]),
    ]);
}

#[test]
fn align_content_space_around_and_align_items_center_with_flex_wrap() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.align_items = AlignItems::Center;
        s.align_content = AlignContent::SpaceAround;
        s.size.width = Length::Px(300.0);
        s.size.height = Length::Px(300.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(150.0);
        s.size.height = Length::Px(50.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(120.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(120.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 300.0, 300.0]),
        (root_child0, [0.0, 63.0, 150.0, 50.0]),
        (root_child1, [150.0, 38.0, 120.0, 100.0]),
        (root_child2, [0.0, 213.0, 120.0, 50.0]),
    ]);
}

#[test]
fn align_content_space_around_and_align_items_flex_start_with_flex_wrap() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.align_items = AlignItems::FlexStart;
        s.align_content = AlignContent::SpaceAround;
        s.size.width = Length::Px(300.0);
        s.size.height = Length::Px(300.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(150.0);
        s.size.height = Length::Px(50.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(120.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(120.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 300.0, 300.0]),
        (root_child0, [0.0, 38.0, 150.0, 50.0]),
        (root_child1, [150.0, 38.0, 120.0, 100.0]),
        (root_child2, [0.0, 213.0, 120.0, 50.0]),
    ]);
}

#[test]
fn align_content_flex_start_stretch_doesnt_influence_line_box_dim() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(400.0);
        s.padding.left = Length::Px(20.0);
        s.padding.top = Length::Px(20.0);
        s.padding.right = Length::Px(20.0);
        s.padding.bottom = Length::Px(20.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.margin.right = Length::Px(20.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
    });
    let root_child1_child0 = t.child(root_child1, "root_child1_child0", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(30.0);
        s.margin.right = Length::Px(20.0);
    });
    let root_child1_child1 = t.child(root_child1, "root_child1_child1", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(30.0);
        s.margin.right = Length::Px(20.0);
    });
    let root_child1_child2 = t.child(root_child1, "root_child1_child2", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(30.0);
        s.margin.right = Length::Px(20.0);
    });
    let root_child1_child3 = t.child(root_child1, "root_child1_child3", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(30.0);
        s.margin.right = Length::Px(20.0);
    });
    let root_child1_child4 = t.child(root_child1, "root_child1_child4", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(30.0);
        s.margin.right = Length::Px(20.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.margin.left = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 400.0, 140.0]),
        (root_child0, [20.0, 20.0, 100.0, 100.0]),
        (root_child1, [140.0, 20.0, 170.0, 100.0]),
        (root_child1_child0, [0.0, 0.0, 30.0, 30.0]),
        (root_child1_child1, [50.0, 0.0, 30.0, 30.0]),
        (root_child1_child2, [100.0, 0.0, 30.0, 30.0]),
        (root_child1_child3, [0.0, 30.0, 30.0, 30.0]),
        (root_child1_child4, [50.0, 30.0, 30.0, 30.0]),
        (root_child2, [330.0, 20.0, 50.0, 50.0]),
    ]);
}

#[test]
fn align_content_stretch_stretch_does_influence_line_box_dim() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(400.0);
        s.padding.left = Length::Px(20.0);
        s.padding.top = Length::Px(20.0);
        s.padding.right = Length::Px(20.0);
        s.padding.bottom = Length::Px(20.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.margin.right = Length::Px(20.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.align_content = AlignContent::Stretch;
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
    });
    let root_child1_child0 = t.child(root_child1, "root_child1_child0", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(30.0);
        s.margin.right = Length::Px(20.0);
    });
    let root_child1_child1 = t.child(root_child1, "root_child1_child1", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(30.0);
        s.margin.right = Length::Px(20.0);
    });
    let root_child1_child2 = t.child(root_child1, "root_child1_child2", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(30.0);
        s.margin.right = Length::Px(20.0);
    });
    let root_child1_child3 = t.child(root_child1, "root_child1_child3", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(30.0);
        s.margin.right = Length::Px(20.0);
    });
    let root_child1_child4 = t.child(root_child1, "root_child1_child4", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(30.0);
        s.margin.right = Length::Px(20.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.margin.left = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 400.0, 140.0]),
        (root_child0, [20.0, 20.0, 100.0, 100.0]),
        (root_child1, [140.0, 20.0, 170.0, 100.0]),
        (root_child1_child0, [0.0, 0.0, 30.0, 30.0]),
        (root_child1_child1, [50.0, 0.0, 30.0, 30.0]),
        (root_child1_child2, [100.0, 0.0, 30.0, 30.0]),
        (root_child1_child3, [0.0, 50.0, 30.0, 30.0]),
        (root_child1_child4, [50.0, 50.0, 30.0, 30.0]),
        (root_child2, [330.0, 20.0, 50.0, 50.0]),
    ]);
}

#[test]
fn align_content_space_evenly_stretch_does_influence_line_box_dim() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(400.0);
        s.padding.left = Length::Px(20.0);
        s.padding.top = Length::Px(20.0);
        s.padding.right = Length::Px(20.0);
        s.padding.bottom = Length::Px(20.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.margin.right = Length::Px(20.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.align_content = AlignContent::Stretch;
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
    });
    let root_child1_child0 = t.child(root_child1, "root_child1_child0", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(30.0);
        s.margin.right = Length::Px(20.0);
    });
    let root_child1_child1 = t.child(root_child1, "root_child1_child1", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(30.0);
        s.margin.right = Length::Px(20.0);
    });
    let root_child1_child2 = t.child(root_child1, "root_child1_child2", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(30.0);
        s.margin.right = Length::Px(20.0);
    });
    let root_child1_child3 = t.child(root_child1, "root_child1_child3", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(30.0);
        s.margin.right = Length::Px(20.0);
    });
    let root_child1_child4 = t.child(root_child1, "root_child1_child4", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(30.0);
        s.margin.right = Length::Px(20.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.margin.left = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 400.0, 140.0]),
        (root_child0, [20.0, 20.0, 100.0, 100.0]),
        (root_child1, [140.0, 20.0, 170.0, 100.0]),
        (root_child1_child0, [0.0, 0.0, 30.0, 30.0]),
        (root_child1_child1, [50.0, 0.0, 30.0, 30.0]),
        (root_child1_child2, [100.0, 0.0, 30.0, 30.0]),
        (root_child1_child3, [0.0, 50.0, 30.0, 30.0]),
        (root_child1_child4, [50.0, 50.0, 30.0, 30.0]),
        (root_child2, [330.0, 20.0, 50.0, 50.0]),
    ]);
}

#[test]
fn align_content_stretch_and_align_items_flex_end_with_flex_wrap() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.align_items = AlignItems::FlexEnd;
        s.align_content = AlignContent::Stretch;
        s.size.width = Length::Px(300.0);
        s.size.height = Length::Px(300.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.align_self = Some(AlignItems::FlexStart);
        s.size.width = Length::Px(150.0);
        s.size.height = Length::Px(50.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(120.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(120.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 300.0, 300.0]),
        (root_child0, [0.0, 0.0, 150.0, 50.0]),
        (root_child1, [150.0, 75.0, 120.0, 100.0]),
        (root_child2, [0.0, 250.0, 120.0, 50.0]),
    ]);
}

#[test]
fn align_content_stretch_and_align_items_flex_start_with_flex_wrap() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.align_items = AlignItems::FlexStart;
        s.align_content = AlignContent::Stretch;
        s.size.width = Length::Px(300.0);
        s.size.height = Length::Px(300.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.align_self = Some(AlignItems::FlexEnd);
        s.size.width = Length::Px(150.0);
        s.size.height = Length::Px(50.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(120.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(120.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 300.0, 300.0]),
        (root_child0, [0.0, 125.0, 150.0, 50.0]),
        (root_child1, [150.0, 0.0, 120.0, 100.0]),
        (root_child2, [0.0, 175.0, 120.0, 50.0]),
    ]);
}

#[test]
fn align_content_stretch_and_align_items_center_with_flex_wrap() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.align_items = AlignItems::Center;
        s.align_content = AlignContent::Stretch;
        s.size.width = Length::Px(300.0);
        s.size.height = Length::Px(300.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.align_self = Some(AlignItems::FlexEnd);
        s.size.width = Length::Px(150.0);
        s.size.height = Length::Px(50.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(120.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(120.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 300.0, 300.0]),
        (root_child0, [0.0, 125.0, 150.0, 50.0]),
        (root_child1, [150.0, 38.0, 120.0, 100.0]),
        (root_child2, [0.0, 213.0, 120.0, 50.0]),
    ]);
}

#[test]
fn align_content_stretch_and_align_items_stretch_with_flex_wrap() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.align_content = AlignContent::Stretch;
        s.size.width = Length::Px(300.0);
        s.size.height = Length::Px(300.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.align_self = Some(AlignItems::FlexEnd);
        s.size.width = Length::Px(150.0);
        s.size.height = Length::Px(50.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(120.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(120.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 300.0, 300.0]),
        (root_child0, [0.0, 125.0, 150.0, 50.0]),
        (root_child1, [150.0, 0.0, 120.0, 100.0]),
        (root_child2, [0.0, 175.0, 120.0, 50.0]),
    ]);
}
