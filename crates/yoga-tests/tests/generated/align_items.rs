// @generated — do not edit. Written by the Yoga importer (crates/yoga-tests/src/bin/import.rs):
//     cargo run -p souple-yoga-tests --bin import -- <path to yoga>
// From Yoga 3.2.1, tests/generated/YGAlignItemsTest.cpp (MIT, Copyright (c) Meta Platforms, Inc. and affiliates).

use crate::support::*;

#[test]
fn align_items_stretch() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.height = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 100.0, 10.0]),
    ]);
}

#[test]
fn align_items_center() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.align_items = AlignItems::Center;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(10.0);
        s.size.height = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [45.0, 0.0, 10.0, 10.0]),
    ]);
}

#[test]
fn align_items_flex_start() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.align_items = AlignItems::FlexStart;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(10.0);
        s.size.height = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 10.0, 10.0]),
    ]);
}

#[test]
fn align_items_flex_end() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.align_items = AlignItems::FlexEnd;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(10.0);
        s.size.height = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [90.0, 0.0, 10.0, 10.0]),
    ]);
}

#[test]
fn align_baseline() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.align_items = AlignItems::Baseline;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 50.0, 50.0]),
        (root_child1, [50.0, 30.0, 50.0, 20.0]),
    ]);
}

#[test]
fn align_baseline_child() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.align_items = AlignItems::Baseline;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child1_child0 = t.child(root_child1, "root_child1_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 50.0, 50.0]),
        (root_child1, [50.0, 40.0, 50.0, 20.0]),
        (root_child1_child0, [0.0, 0.0, 50.0, 10.0]),
    ]);
}

#[test]
fn align_baseline_child_multiline() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.align_items = AlignItems::Baseline;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(60.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(25.0);
    });
    let root_child1_child0 = t.child(root_child1, "root_child1_child0", |s| {
        s.size.width = Length::Px(25.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child1_child1 = t.child(root_child1, "root_child1_child1", |s| {
        s.size.width = Length::Px(25.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child1_child2 = t.child(root_child1, "root_child1_child2", |s| {
        s.size.width = Length::Px(25.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child1_child3 = t.child(root_child1, "root_child1_child3", |s| {
        s.size.width = Length::Px(25.0);
        s.size.height = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 50.0, 60.0]),
        (root_child1, [50.0, 40.0, 50.0, 25.0]),
        (root_child1_child0, [0.0, 0.0, 25.0, 20.0]),
        (root_child1_child1, [25.0, 0.0, 25.0, 10.0]),
        (root_child1_child2, [0.0, 20.0, 25.0, 20.0]),
        (root_child1_child3, [25.0, 20.0, 25.0, 10.0]),
    ]);
}

#[test]
fn align_baseline_child_multiline_override() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.align_items = AlignItems::Baseline;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(60.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(25.0);
    });
    let root_child1_child0 = t.child(root_child1, "root_child1_child0", |s| {
        s.size.width = Length::Px(25.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child1_child1 = t.child(root_child1, "root_child1_child1", |s| {
        s.align_self = Some(AlignItems::Baseline);
        s.size.width = Length::Px(25.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child1_child2 = t.child(root_child1, "root_child1_child2", |s| {
        s.size.width = Length::Px(25.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child1_child3 = t.child(root_child1, "root_child1_child3", |s| {
        s.align_self = Some(AlignItems::Baseline);
        s.size.width = Length::Px(25.0);
        s.size.height = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 50.0, 60.0]),
        (root_child1, [50.0, 50.0, 50.0, 25.0]),
        (root_child1_child0, [0.0, 0.0, 25.0, 20.0]),
        (root_child1_child1, [25.0, 0.0, 25.0, 10.0]),
        (root_child1_child2, [0.0, 20.0, 25.0, 20.0]),
        (root_child1_child3, [25.0, 20.0, 25.0, 10.0]),
    ]);
}

#[test]
fn align_baseline_child_multiline_no_override_on_secondline() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.align_items = AlignItems::Baseline;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(60.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(25.0);
    });
    let root_child1_child0 = t.child(root_child1, "root_child1_child0", |s| {
        s.size.width = Length::Px(25.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child1_child1 = t.child(root_child1, "root_child1_child1", |s| {
        s.size.width = Length::Px(25.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child1_child2 = t.child(root_child1, "root_child1_child2", |s| {
        s.size.width = Length::Px(25.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child1_child3 = t.child(root_child1, "root_child1_child3", |s| {
        s.align_self = Some(AlignItems::Baseline);
        s.size.width = Length::Px(25.0);
        s.size.height = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 50.0, 60.0]),
        (root_child1, [50.0, 40.0, 50.0, 25.0]),
        (root_child1_child0, [0.0, 0.0, 25.0, 20.0]),
        (root_child1_child1, [25.0, 0.0, 25.0, 10.0]),
        (root_child1_child2, [0.0, 20.0, 25.0, 20.0]),
        (root_child1_child3, [25.0, 20.0, 25.0, 10.0]),
    ]);
}

#[test]
fn align_baseline_child_top() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.align_items = AlignItems::Baseline;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.inset.top = Length::Px(10.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child1_child0 = t.child(root_child1, "root_child1_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 10.0, 50.0, 50.0]),
        (root_child1, [50.0, 40.0, 50.0, 20.0]),
        (root_child1_child0, [0.0, 0.0, 50.0, 10.0]),
    ]);
}

#[test]
fn align_baseline_child_top2() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.align_items = AlignItems::Baseline;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(20.0);
        s.inset.top = Length::Px(5.0);
    });
    let root_child1_child0 = t.child(root_child1, "root_child1_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 50.0, 50.0]),
        (root_child1, [50.0, 45.0, 50.0, 20.0]),
        (root_child1_child0, [0.0, 0.0, 50.0, 10.0]),
    ]);
}

#[test]
fn align_baseline_double_nested_child() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.align_items = AlignItems::Baseline;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child1_child0 = t.child(root_child1, "root_child1_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(15.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 50.0, 50.0]),
        (root_child0_child0, [0.0, 0.0, 50.0, 20.0]),
        (root_child1, [50.0, 5.0, 50.0, 20.0]),
        (root_child1_child0, [0.0, 0.0, 50.0, 15.0]),
    ]);
}

#[test]
fn align_baseline_column() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.align_items = AlignItems::Baseline;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 50.0, 50.0]),
        (root_child1, [0.0, 50.0, 50.0, 20.0]),
    ]);
}

#[test]
fn align_baseline_child_margin() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.align_items = AlignItems::Baseline;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.margin.left = Length::Px(5.0);
        s.margin.top = Length::Px(5.0);
        s.margin.right = Length::Px(5.0);
        s.margin.bottom = Length::Px(5.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child1_child0 = t.child(root_child1, "root_child1_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
        s.margin.left = Length::Px(1.0);
        s.margin.top = Length::Px(1.0);
        s.margin.right = Length::Px(1.0);
        s.margin.bottom = Length::Px(1.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [5.0, 5.0, 50.0, 50.0]),
        (root_child1, [60.0, 44.0, 50.0, 20.0]),
        (root_child1_child0, [1.0, 1.0, 50.0, 10.0]),
    ]);
}

#[test]
fn align_baseline_child_padding() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.align_items = AlignItems::Baseline;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(5.0);
        s.padding.right = Length::Px(5.0);
        s.padding.bottom = Length::Px(5.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(20.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(5.0);
        s.padding.right = Length::Px(5.0);
        s.padding.bottom = Length::Px(5.0);
    });
    let root_child1_child0 = t.child(root_child1, "root_child1_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [5.0, 5.0, 50.0, 50.0]),
        (root_child1, [55.0, 40.0, 50.0, 20.0]),
        (root_child1_child0, [5.0, 5.0, 50.0, 10.0]),
    ]);
}

#[test]
fn align_baseline_multiline() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.align_items = AlignItems::Baseline;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child1_child0 = t.child(root_child1, "root_child1_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child2_child0 = t.child(root_child2, "root_child2_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 50.0, 50.0]),
        (root_child1, [50.0, 40.0, 50.0, 20.0]),
        (root_child1_child0, [0.0, 0.0, 50.0, 10.0]),
        (root_child2, [0.0, 100.0, 50.0, 20.0]),
        (root_child2_child0, [0.0, 0.0, 50.0, 10.0]),
        (root_child3, [50.0, 60.0, 50.0, 50.0]),
    ]);
}

#[test]
#[ignore = "skipped by Yoga: GTEST_SKIP, Yoga fails it too"]
fn align_baseline_multiline_column() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_wrap = FlexWrap::Wrap;
        s.align_items = AlignItems::Baseline;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(50.0);
    });
    let root_child1_child0 = t.child(root_child1, "root_child1_child0", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(40.0);
        s.size.height = Length::Px(70.0);
    });
    let root_child2_child0 = t.child(root_child2, "root_child2_child0", |s| {
        s.size.width = Length::Px(10.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 50.0, 50.0]),
        (root_child1, [0.0, 50.0, 30.0, 50.0]),
        (root_child1_child0, [0.0, 0.0, 20.0, 20.0]),
        (root_child2, [50.0, 0.0, 40.0, 70.0]),
        (root_child2_child0, [0.0, 0.0, 10.0, 10.0]),
        (root_child3, [50.0, 70.0, 50.0, 20.0]),
    ]);
}

#[test]
#[ignore = "skipped by Yoga: GTEST_SKIP, Yoga fails it too"]
fn align_baseline_multiline_column2() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_wrap = FlexWrap::Wrap;
        s.align_items = AlignItems::Baseline;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(50.0);
    });
    let root_child1_child0 = t.child(root_child1, "root_child1_child0", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(40.0);
        s.size.height = Length::Px(70.0);
    });
    let root_child2_child0 = t.child(root_child2, "root_child2_child0", |s| {
        s.size.width = Length::Px(10.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 50.0, 50.0]),
        (root_child1, [0.0, 50.0, 30.0, 50.0]),
        (root_child1_child0, [0.0, 0.0, 20.0, 20.0]),
        (root_child2, [50.0, 0.0, 40.0, 70.0]),
        (root_child2_child0, [0.0, 0.0, 10.0, 10.0]),
        (root_child3, [50.0, 70.0, 50.0, 20.0]),
    ]);
}

#[test]
fn align_baseline_multiline_row_and_column() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.align_items = AlignItems::Baseline;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    let root_child1_child0 = t.child(root_child1, "root_child1_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child2_child0 = t.child(root_child2, "root_child2_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 50.0, 50.0]),
        (root_child1, [50.0, 40.0, 50.0, 50.0]),
        (root_child1_child0, [0.0, 0.0, 50.0, 10.0]),
        (root_child2, [0.0, 100.0, 50.0, 20.0]),
        (root_child2_child0, [0.0, 0.0, 50.0, 10.0]),
        (root_child3, [50.0, 90.0, 50.0, 20.0]),
    ]);
}

#[test]
fn align_items_center_child_with_margin_bigger_than_parent() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.justify_content = JustifyContent::Center;
        s.align_items = AlignItems::Center;
        s.size.width = Length::Px(52.0);
        s.size.height = Length::Px(52.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.align_items = AlignItems::Center;
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.size.width = Length::Px(52.0);
        s.size.height = Length::Px(52.0);
        s.margin.left = Length::Px(10.0);
        s.margin.right = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 52.0, 52.0]),
        (root_child0, [-10.0, 0.0, 72.0, 52.0]),
        (root_child0_child0, [10.0, 0.0, 52.0, 52.0]),
    ]);
}

#[test]
fn align_items_flex_end_child_with_margin_bigger_than_parent() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.justify_content = JustifyContent::Center;
        s.align_items = AlignItems::Center;
        s.size.width = Length::Px(52.0);
        s.size.height = Length::Px(52.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.align_items = AlignItems::FlexEnd;
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.size.width = Length::Px(52.0);
        s.size.height = Length::Px(52.0);
        s.margin.left = Length::Px(10.0);
        s.margin.right = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 52.0, 52.0]),
        (root_child0, [-10.0, 0.0, 72.0, 52.0]),
        (root_child0_child0, [10.0, 0.0, 52.0, 52.0]),
    ]);
}

#[test]
fn align_items_center_child_without_margin_bigger_than_parent() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.justify_content = JustifyContent::Center;
        s.align_items = AlignItems::Center;
        s.size.width = Length::Px(52.0);
        s.size.height = Length::Px(52.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.align_items = AlignItems::Center;
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.size.width = Length::Px(72.0);
        s.size.height = Length::Px(72.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 52.0, 52.0]),
        (root_child0, [-10.0, -10.0, 72.0, 72.0]),
        (root_child0_child0, [0.0, 0.0, 72.0, 72.0]),
    ]);
}

#[test]
fn align_items_flex_end_child_without_margin_bigger_than_parent() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.justify_content = JustifyContent::Center;
        s.align_items = AlignItems::Center;
        s.size.width = Length::Px(52.0);
        s.size.height = Length::Px(52.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.align_items = AlignItems::FlexEnd;
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.size.width = Length::Px(72.0);
        s.size.height = Length::Px(72.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 52.0, 52.0]),
        (root_child0, [-10.0, -10.0, 72.0, 72.0]),
        (root_child0_child0, [0.0, 0.0, 72.0, 72.0]),
    ]);
}

#[test]
fn align_center_should_size_based_on_content() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.align_items = AlignItems::Center;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.margin.top = Length::Px(20.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.justify_content = JustifyContent::Center;
        s.flex_shrink = 1.0;
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 20.0, 100.0, 100.0]),
        (root_child0, [40.0, 0.0, 20.0, 20.0]),
        (root_child0_child0, [0.0, 0.0, 20.0, 20.0]),
        (root_child0_child0_child0, [0.0, 0.0, 20.0, 20.0]),
    ]);
}

#[test]
fn align_stretch_should_size_based_on_parent() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.margin.top = Length::Px(20.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.justify_content = JustifyContent::Center;
        s.flex_shrink = 1.0;
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 20.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 100.0, 20.0]),
        (root_child0_child0, [0.0, 0.0, 100.0, 20.0]),
        (root_child0_child0_child0, [0.0, 0.0, 20.0, 20.0]),
    ]);
}

#[test]
fn align_flex_start_with_shrinking_children() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(500.0);
        s.size.height = Length::Px(500.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.align_items = AlignItems::FlexStart;
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 500.0, 500.0]),
        (root_child0, [0.0, 0.0, 500.0, 0.0]),
        (root_child0_child0, [0.0, 0.0, 0.0, 0.0]),
        (root_child0_child0_child0, [0.0, 0.0, 0.0, 0.0]),
    ]);
}

#[test]
fn align_flex_start_with_stretching_children() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(500.0);
        s.size.height = Length::Px(500.0);
    });
    let root_child0 = t.child(root, "root_child0", |_| {});
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 500.0, 500.0]),
        (root_child0, [0.0, 0.0, 500.0, 0.0]),
        (root_child0_child0, [0.0, 0.0, 500.0, 0.0]),
        (root_child0_child0_child0, [0.0, 0.0, 500.0, 0.0]),
    ]);
}

#[test]
fn align_flex_start_with_shrinking_children_with_stretch() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(500.0);
        s.size.height = Length::Px(500.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.align_items = AlignItems::FlexStart;
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 500.0, 500.0]),
        (root_child0, [0.0, 0.0, 500.0, 0.0]),
        (root_child0_child0, [0.0, 0.0, 0.0, 0.0]),
        (root_child0_child0_child0, [0.0, 0.0, 0.0, 0.0]),
    ]);
}

#[test]
fn align_flex_end_with_row_reverse() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_wrap = FlexWrap::Wrap;
        s.align_items = AlignItems::FlexEnd;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(75.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.margin.left = Length::Px(3.0);
        s.margin.right = Length::Px(5.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 75.0]),
        (root_child0, [3.0, 0.0, 50.0, 50.0]),
        (root_child1, [58.0, 0.0, 50.0, 50.0]),
    ]);
}

#[test]
fn align_stretch_with_row_reverse() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_wrap = FlexWrap::Wrap;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(75.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.margin.left = Length::Px(3.0);
        s.margin.right = Length::Px(5.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 75.0]),
        (root_child0, [3.0, 0.0, 50.0, 50.0]),
        (root_child1, [58.0, 0.0, 50.0, 50.0]),
    ]);
}
