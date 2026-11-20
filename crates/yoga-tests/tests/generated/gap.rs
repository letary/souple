// @generated — do not edit. Written by the Yoga importer (crates/yoga-tests/src/bin/import.rs):
//     cargo run -p souple-yoga-tests --bin import -- <path to yoga>
// From Yoga 3.2.1, tests/generated/YGGapTest.cpp (MIT, Copyright (c) Meta Platforms, Inc. and affiliates).

use crate::support::*;

#[test]
fn column_gap_flexible() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(80.0);
        s.size.height = Length::Px(100.0);
        s.row_gap = Length::Px(20.0);
        s.column_gap = Length::Px(10.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 80.0, 100.0]),
        (root_child0, [0.0, 0.0, 20.0, 100.0]),
        (root_child1, [30.0, 0.0, 20.0, 100.0]),
        (root_child2, [60.0, 0.0, 20.0, 100.0]),
    ]);
}

#[test]
fn column_gap_inflexible() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(80.0);
        s.size.height = Length::Px(100.0);
        s.column_gap = Length::Px(10.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(20.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(20.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 80.0, 100.0]),
        (root_child0, [0.0, 0.0, 20.0, 100.0]),
        (root_child1, [30.0, 0.0, 20.0, 100.0]),
        (root_child2, [60.0, 0.0, 20.0, 100.0]),
    ]);
}

#[test]
fn column_gap_mixed_flexible() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(80.0);
        s.size.height = Length::Px(100.0);
        s.column_gap = Length::Px(10.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(20.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 80.0, 100.0]),
        (root_child0, [0.0, 0.0, 20.0, 100.0]),
        (root_child1, [30.0, 0.0, 20.0, 100.0]),
        (root_child2, [60.0, 0.0, 20.0, 100.0]),
    ]);
}

#[test]
fn column_gap_child_margins() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(80.0);
        s.size.height = Length::Px(100.0);
        s.column_gap = Length::Px(10.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
        s.margin.left = Length::Px(2.0);
        s.margin.right = Length::Px(2.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
        s.margin.left = Length::Px(10.0);
        s.margin.right = Length::Px(10.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
        s.margin.left = Length::Px(15.0);
        s.margin.right = Length::Px(15.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 80.0, 100.0]),
        (root_child0, [2.0, 0.0, 2.0, 100.0]),
        (root_child1, [26.0, 0.0, 2.0, 100.0]),
        (root_child2, [63.0, 0.0, 2.0, 100.0]),
    ]);
}

#[test]
fn column_row_gap_wrapping() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.size.width = Length::Px(80.0);
        s.row_gap = Length::Px(20.0);
        s.column_gap = Length::Px(10.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child4 = t.child(root, "root_child4", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child5 = t.child(root, "root_child5", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child6 = t.child(root, "root_child6", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child7 = t.child(root, "root_child7", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child8 = t.child(root, "root_child8", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 80.0, 100.0]),
        (root_child0, [0.0, 0.0, 20.0, 20.0]),
        (root_child1, [30.0, 0.0, 20.0, 20.0]),
        (root_child2, [60.0, 0.0, 20.0, 20.0]),
        (root_child3, [0.0, 40.0, 20.0, 20.0]),
        (root_child4, [30.0, 40.0, 20.0, 20.0]),
        (root_child5, [60.0, 40.0, 20.0, 20.0]),
        (root_child6, [0.0, 80.0, 20.0, 20.0]),
        (root_child7, [30.0, 80.0, 20.0, 20.0]),
        (root_child8, [60.0, 80.0, 20.0, 20.0]),
    ]);
}

#[test]
fn column_gap_start_index() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.size.width = Length::Px(80.0);
        s.row_gap = Length::Px(20.0);
        s.column_gap = Length::Px(10.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 80.0, 20.0]),
        (root_child0, [0.0, 0.0, 20.0, 20.0]),
        (root_child1, [0.0, 0.0, 20.0, 20.0]),
        (root_child2, [30.0, 0.0, 20.0, 20.0]),
        (root_child3, [60.0, 0.0, 20.0, 20.0]),
    ]);
}

#[test]
fn column_gap_justify_flex_start() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.column_gap = Length::Px(10.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(20.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(20.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 20.0, 100.0]),
        (root_child1, [30.0, 0.0, 20.0, 100.0]),
        (root_child2, [60.0, 0.0, 20.0, 100.0]),
    ]);
}

#[test]
fn column_gap_justify_center() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.justify_content = JustifyContent::Center;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.column_gap = Length::Px(10.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(20.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(20.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [10.0, 0.0, 20.0, 100.0]),
        (root_child1, [40.0, 0.0, 20.0, 100.0]),
        (root_child2, [70.0, 0.0, 20.0, 100.0]),
    ]);
}

#[test]
fn column_gap_justify_flex_end() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.justify_content = JustifyContent::FlexEnd;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.column_gap = Length::Px(10.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(20.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(20.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [20.0, 0.0, 20.0, 100.0]),
        (root_child1, [50.0, 0.0, 20.0, 100.0]),
        (root_child2, [80.0, 0.0, 20.0, 100.0]),
    ]);
}

#[test]
fn column_gap_justify_space_between() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.justify_content = JustifyContent::SpaceBetween;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.column_gap = Length::Px(10.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(20.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(20.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 20.0, 100.0]),
        (root_child1, [40.0, 0.0, 20.0, 100.0]),
        (root_child2, [80.0, 0.0, 20.0, 100.0]),
    ]);
}

#[test]
fn column_gap_justify_space_around() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.justify_content = JustifyContent::SpaceAround;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.column_gap = Length::Px(10.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(20.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(20.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [3.0, 0.0, 20.0, 100.0]),
        (root_child1, [40.0, 0.0, 20.0, 100.0]),
        (root_child2, [77.0, 0.0, 20.0, 100.0]),
    ]);
}

#[test]
fn column_gap_justify_space_evenly() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.justify_content = JustifyContent::SpaceEvenly;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.column_gap = Length::Px(10.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(20.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(20.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [5.0, 0.0, 20.0, 100.0]),
        (root_child1, [40.0, 0.0, 20.0, 100.0]),
        (root_child2, [75.0, 0.0, 20.0, 100.0]),
    ]);
}

#[test]
fn column_gap_wrap_align_flex_start() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.row_gap = Length::Px(20.0);
        s.column_gap = Length::Px(10.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child4 = t.child(root, "root_child4", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child5 = t.child(root, "root_child5", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 20.0, 20.0]),
        (root_child1, [30.0, 0.0, 20.0, 20.0]),
        (root_child2, [60.0, 0.0, 20.0, 20.0]),
        (root_child3, [0.0, 40.0, 20.0, 20.0]),
        (root_child4, [30.0, 40.0, 20.0, 20.0]),
        (root_child5, [60.0, 40.0, 20.0, 20.0]),
    ]);
}

#[test]
fn column_gap_wrap_align_center() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.align_content = AlignContent::Center;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.row_gap = Length::Px(20.0);
        s.column_gap = Length::Px(10.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child4 = t.child(root, "root_child4", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child5 = t.child(root, "root_child5", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 20.0, 20.0, 20.0]),
        (root_child1, [30.0, 20.0, 20.0, 20.0]),
        (root_child2, [60.0, 20.0, 20.0, 20.0]),
        (root_child3, [0.0, 60.0, 20.0, 20.0]),
        (root_child4, [30.0, 60.0, 20.0, 20.0]),
        (root_child5, [60.0, 60.0, 20.0, 20.0]),
    ]);
}

#[test]
fn column_gap_wrap_align_flex_end() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.align_content = AlignContent::FlexEnd;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.row_gap = Length::Px(20.0);
        s.column_gap = Length::Px(10.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child4 = t.child(root, "root_child4", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child5 = t.child(root, "root_child5", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 40.0, 20.0, 20.0]),
        (root_child1, [30.0, 40.0, 20.0, 20.0]),
        (root_child2, [60.0, 40.0, 20.0, 20.0]),
        (root_child3, [0.0, 80.0, 20.0, 20.0]),
        (root_child4, [30.0, 80.0, 20.0, 20.0]),
        (root_child5, [60.0, 80.0, 20.0, 20.0]),
    ]);
}

#[test]
fn column_gap_wrap_align_space_between() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.align_content = AlignContent::SpaceBetween;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.row_gap = Length::Px(20.0);
        s.column_gap = Length::Px(10.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child4 = t.child(root, "root_child4", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child5 = t.child(root, "root_child5", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 20.0, 20.0]),
        (root_child1, [30.0, 0.0, 20.0, 20.0]),
        (root_child2, [60.0, 0.0, 20.0, 20.0]),
        (root_child3, [0.0, 80.0, 20.0, 20.0]),
        (root_child4, [30.0, 80.0, 20.0, 20.0]),
        (root_child5, [60.0, 80.0, 20.0, 20.0]),
    ]);
}

#[test]
fn column_gap_wrap_align_space_around() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.align_content = AlignContent::SpaceAround;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.row_gap = Length::Px(20.0);
        s.column_gap = Length::Px(10.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child4 = t.child(root, "root_child4", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child5 = t.child(root, "root_child5", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 10.0, 20.0, 20.0]),
        (root_child1, [30.0, 10.0, 20.0, 20.0]),
        (root_child2, [60.0, 10.0, 20.0, 20.0]),
        (root_child3, [0.0, 70.0, 20.0, 20.0]),
        (root_child4, [30.0, 70.0, 20.0, 20.0]),
        (root_child5, [60.0, 70.0, 20.0, 20.0]),
    ]);
}

#[test]
fn column_gap_wrap_align_stretch() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.align_content = AlignContent::Stretch;
        s.size.width = Length::Px(300.0);
        s.size.height = Length::Px(300.0);
        s.column_gap = Length::Px(5.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
        s.min_size.width = Length::Px(60.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_grow = 1.0;
        s.min_size.width = Length::Px(60.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.flex_grow = 1.0;
        s.min_size.width = Length::Px(60.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.flex_grow = 1.0;
        s.min_size.width = Length::Px(60.0);
    });
    let root_child4 = t.child(root, "root_child4", |s| {
        s.flex_grow = 1.0;
        s.min_size.width = Length::Px(60.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 300.0, 300.0]),
        (root_child0, [0.0, 0.0, 71.0, 150.0]),
        (root_child1, [76.0, 0.0, 72.0, 150.0]),
        (root_child2, [153.0, 0.0, 71.0, 150.0]),
        (root_child3, [229.0, 0.0, 71.0, 150.0]),
        (root_child4, [0.0, 150.0, 300.0, 150.0]),
    ]);
}

#[test]
fn column_gap_determines_parent_width() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.height = Length::Px(100.0);
        s.column_gap = Length::Px(10.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(10.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(20.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(30.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 80.0, 100.0]),
        (root_child0, [0.0, 0.0, 10.0, 100.0]),
        (root_child1, [20.0, 0.0, 20.0, 100.0]),
        (root_child2, [50.0, 0.0, 30.0, 100.0]),
    ]);
}

#[test]
fn row_gap_align_items_stretch() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.align_content = AlignContent::Stretch;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(200.0);
        s.row_gap = Length::Px(20.0);
        s.column_gap = Length::Px(10.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(20.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(20.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(20.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.size.width = Length::Px(20.0);
    });
    let root_child4 = t.child(root, "root_child4", |s| {
        s.size.width = Length::Px(20.0);
    });
    let root_child5 = t.child(root, "root_child5", |s| {
        s.size.width = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 200.0]),
        (root_child0, [0.0, 0.0, 20.0, 90.0]),
        (root_child1, [30.0, 0.0, 20.0, 90.0]),
        (root_child2, [60.0, 0.0, 20.0, 90.0]),
        (root_child3, [0.0, 110.0, 20.0, 90.0]),
        (root_child4, [30.0, 110.0, 20.0, 90.0]),
        (root_child5, [60.0, 110.0, 20.0, 90.0]),
    ]);
}

#[test]
fn row_gap_align_items_end() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.align_items = AlignItems::FlexEnd;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(200.0);
        s.row_gap = Length::Px(20.0);
        s.column_gap = Length::Px(10.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(20.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(20.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(20.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.size.width = Length::Px(20.0);
    });
    let root_child4 = t.child(root, "root_child4", |s| {
        s.size.width = Length::Px(20.0);
    });
    let root_child5 = t.child(root, "root_child5", |s| {
        s.size.width = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 200.0]),
        (root_child0, [0.0, 0.0, 20.0, 0.0]),
        (root_child1, [30.0, 0.0, 20.0, 0.0]),
        (root_child2, [60.0, 0.0, 20.0, 0.0]),
        (root_child3, [0.0, 20.0, 20.0, 0.0]),
        (root_child4, [30.0, 20.0, 20.0, 0.0]),
        (root_child5, [60.0, 20.0, 20.0, 0.0]),
    ]);
}

#[test]
fn row_gap_column_child_margins() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(200.0);
        s.row_gap = Length::Px(10.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
        s.margin.top = Length::Px(2.0);
        s.margin.bottom = Length::Px(2.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
        s.margin.top = Length::Px(10.0);
        s.margin.bottom = Length::Px(10.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
        s.margin.top = Length::Px(15.0);
        s.margin.bottom = Length::Px(15.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 200.0]),
        (root_child0, [0.0, 2.0, 100.0, 42.0]),
        (root_child1, [0.0, 66.0, 100.0, 42.0]),
        (root_child2, [0.0, 143.0, 100.0, 42.0]),
    ]);
}

#[test]
fn row_gap_row_wrap_child_margins() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(200.0);
        s.row_gap = Length::Px(10.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(60.0);
        s.margin.top = Length::Px(2.0);
        s.margin.bottom = Length::Px(2.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(60.0);
        s.margin.top = Length::Px(10.0);
        s.margin.bottom = Length::Px(10.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(60.0);
        s.margin.top = Length::Px(15.0);
        s.margin.bottom = Length::Px(15.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 200.0]),
        (root_child0, [0.0, 2.0, 60.0, 0.0]),
        (root_child1, [0.0, 24.0, 60.0, 0.0]),
        (root_child2, [0.0, 59.0, 60.0, 0.0]),
    ]);
}

#[test]
fn row_gap_determines_parent_height() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.row_gap = Length::Px(10.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.height = Length::Px(10.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.height = Length::Px(20.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.height = Length::Px(30.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 80.0]),
        (root_child0, [0.0, 0.0, 100.0, 10.0]),
        (root_child1, [0.0, 20.0, 100.0, 20.0]),
        (root_child2, [0.0, 50.0, 100.0, 30.0]),
    ]);
}

#[test]
fn row_gap_percent_wrapping() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.size.width = Length::Px(300.0);
        s.size.height = Length::Px(700.0);
        s.padding.left = Length::Px(10.0);
        s.padding.top = Length::Px(10.0);
        s.padding.right = Length::Px(10.0);
        s.padding.bottom = Length::Px(10.0);
        s.row_gap = Length::Percent(10.0);
        s.column_gap = Length::Percent(10.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child4 = t.child(root, "root_child4", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 300.0, 700.0]),
        (root_child0, [10.0, 10.0, 100.0, 100.0]),
        (root_child1, [138.0, 10.0, 100.0, 100.0]),
        (root_child2, [10.0, 178.0, 100.0, 100.0]),
        (root_child3, [138.0, 178.0, 100.0, 100.0]),
        (root_child4, [10.0, 346.0, 100.0, 100.0]),
    ]);
}

#[test]
fn row_gap_percent_determines_parent_height() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.size.width = Length::Px(300.0);
        s.row_gap = Length::Percent(10.0);
        s.column_gap = Length::Percent(10.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child4 = t.child(root, "root_child4", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 300.0, 300.0]),
        (root_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child1, [130.0, 0.0, 100.0, 100.0]),
        (root_child2, [0.0, 100.0, 100.0, 100.0]),
        (root_child3, [130.0, 100.0, 100.0, 100.0]),
        (root_child4, [0.0, 200.0, 100.0, 100.0]),
    ]);
}

#[test]
fn row_gap_percent_wrapping_with_both_content_padding_and_item_padding() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.size.width = Length::Px(300.0);
        s.size.height = Length::Px(700.0);
        s.padding.left = Length::Px(10.0);
        s.padding.top = Length::Px(10.0);
        s.padding.right = Length::Px(10.0);
        s.padding.bottom = Length::Px(10.0);
        s.row_gap = Length::Percent(10.0);
        s.column_gap = Length::Percent(10.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.padding.left = Length::Px(10.0);
        s.padding.top = Length::Px(10.0);
        s.padding.right = Length::Px(10.0);
        s.padding.bottom = Length::Px(10.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.padding.left = Length::Px(10.0);
        s.padding.top = Length::Px(10.0);
        s.padding.right = Length::Px(10.0);
        s.padding.bottom = Length::Px(10.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.padding.left = Length::Px(10.0);
        s.padding.top = Length::Px(10.0);
        s.padding.right = Length::Px(10.0);
        s.padding.bottom = Length::Px(10.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.padding.left = Length::Px(10.0);
        s.padding.top = Length::Px(10.0);
        s.padding.right = Length::Px(10.0);
        s.padding.bottom = Length::Px(10.0);
    });
    let root_child4 = t.child(root, "root_child4", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.padding.left = Length::Px(10.0);
        s.padding.top = Length::Px(10.0);
        s.padding.right = Length::Px(10.0);
        s.padding.bottom = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 300.0, 700.0]),
        (root_child0, [10.0, 10.0, 100.0, 100.0]),
        (root_child1, [138.0, 10.0, 100.0, 100.0]),
        (root_child2, [10.0, 178.0, 100.0, 100.0]),
        (root_child3, [138.0, 178.0, 100.0, 100.0]),
        (root_child4, [10.0, 346.0, 100.0, 100.0]),
    ]);
}

#[test]
fn row_gap_percent_wrapping_with_both_content_padding() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.size.width = Length::Px(300.0);
        s.size.height = Length::Px(700.0);
        s.padding.left = Length::Px(10.0);
        s.padding.top = Length::Px(10.0);
        s.padding.right = Length::Px(10.0);
        s.padding.bottom = Length::Px(10.0);
        s.row_gap = Length::Percent(10.0);
        s.column_gap = Length::Percent(10.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child4 = t.child(root, "root_child4", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 300.0, 700.0]),
        (root_child0, [10.0, 10.0, 100.0, 100.0]),
        (root_child1, [138.0, 10.0, 100.0, 100.0]),
        (root_child2, [10.0, 178.0, 100.0, 100.0]),
        (root_child3, [138.0, 178.0, 100.0, 100.0]),
        (root_child4, [10.0, 346.0, 100.0, 100.0]),
    ]);
}

#[test]
fn row_gap_percent_wrapping_with_content_margin() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.size.width = Length::Px(300.0);
        s.size.height = Length::Px(700.0);
        s.margin.left = Length::Px(10.0);
        s.margin.top = Length::Px(10.0);
        s.margin.right = Length::Px(10.0);
        s.margin.bottom = Length::Px(10.0);
        s.row_gap = Length::Percent(10.0);
        s.column_gap = Length::Percent(10.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child4 = t.child(root, "root_child4", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [10.0, 10.0, 300.0, 700.0]),
        (root_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child1, [130.0, 0.0, 100.0, 100.0]),
        (root_child2, [0.0, 170.0, 100.0, 100.0]),
        (root_child3, [130.0, 170.0, 100.0, 100.0]),
        (root_child4, [0.0, 340.0, 100.0, 100.0]),
    ]);
}

#[test]
fn row_gap_percent_wrapping_with_content_margin_and_padding() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.size.width = Length::Px(300.0);
        s.size.height = Length::Px(700.0);
        s.margin.left = Length::Px(10.0);
        s.margin.top = Length::Px(10.0);
        s.margin.right = Length::Px(10.0);
        s.margin.bottom = Length::Px(10.0);
        s.padding.left = Length::Px(10.0);
        s.padding.top = Length::Px(10.0);
        s.padding.right = Length::Px(10.0);
        s.padding.bottom = Length::Px(10.0);
        s.row_gap = Length::Percent(10.0);
        s.column_gap = Length::Percent(10.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child4 = t.child(root, "root_child4", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [10.0, 10.0, 300.0, 700.0]),
        (root_child0, [10.0, 10.0, 100.0, 100.0]),
        (root_child1, [138.0, 10.0, 100.0, 100.0]),
        (root_child2, [10.0, 178.0, 100.0, 100.0]),
        (root_child3, [138.0, 178.0, 100.0, 100.0]),
        (root_child4, [10.0, 346.0, 100.0, 100.0]),
    ]);
}

#[test]
fn row_gap_percent_wrapping_with_flexible_content() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(300.0);
        s.size.height = Length::Px(300.0);
        s.row_gap = Length::Percent(10.0);
        s.column_gap = Length::Percent(10.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 300.0, 300.0]),
        (root_child0, [0.0, 0.0, 80.0, 300.0]),
        (root_child1, [110.0, 0.0, 80.0, 300.0]),
        (root_child2, [220.0, 0.0, 80.0, 300.0]),
    ]);
}

#[test]
fn row_gap_percent_wrapping_with_mixed_flexible_content() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(300.0);
        s.size.height = Length::Px(300.0);
        s.row_gap = Length::Percent(10.0);
        s.column_gap = Length::Percent(10.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(10.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Percent(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 300.0, 300.0]),
        (root_child0, [0.0, 0.0, 10.0, 300.0]),
        (root_child1, [40.0, 0.0, 200.0, 300.0]),
        (root_child2, [270.0, 0.0, 30.0, 300.0]),
    ]);
}

#[test]
#[ignore = "skipped by Yoga: GTEST_SKIP, Yoga fails it too"]
fn row_gap_percent_wrapping_with_min_width() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.min_size.width = Length::Px(300.0);
        s.row_gap = Length::Percent(10.0);
        s.column_gap = Length::Percent(10.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child4 = t.child(root, "root_child4", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 300.0, 300.0]),
        (root_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child1, [130.0, 0.0, 100.0, 100.0]),
        (root_child2, [0.0, 100.0, 100.0, 100.0]),
        (root_child3, [130.0, 100.0, 100.0, 100.0]),
        (root_child4, [0.0, 200.0, 100.0, 100.0]),
    ]);
}
