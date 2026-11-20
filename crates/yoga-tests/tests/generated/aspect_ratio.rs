// @generated — do not edit. Written by the Yoga importer (crates/yoga-tests/src/bin/import.rs):
//     cargo run -p souple-yoga-tests --bin import -- <path to yoga>
// From Yoga 3.2.1, tests/generated/YGAspectRatioTest.cpp (MIT, Copyright (c) Meta Platforms, Inc. and affiliates).

use crate::support::*;

#[test]
#[ignore = "skipped by Yoga: GTEST_SKIP, Yoga fails it too"]
fn aspect_ratio_does_not_stretch_cross_axis_dim() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(300.0);
        s.size.height = Length::Px(300.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.overflow = Overflow::Scroll;
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.flex_direction = FlexDirection::Row;
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.flex_grow = 2.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
        s.aspect_ratio = Some(1.0);
    });
    let root_child0_child0_child1 = t.child(root_child0_child0, "root_child0_child0_child1", |s| {
        s.size.width = Length::Px(5.0);
    });
    let root_child0_child0_child2 = t.child(root_child0_child0, "root_child0_child0_child2", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
    });
    let root_child0_child0_child2_child0 = t.child(root_child0_child0_child2, "root_child0_child0_child2_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
        s.aspect_ratio = Some(1.0);
    });
    let root_child0_child0_child2_child0_child0 = t.child(root_child0_child0_child2_child0, "root_child0_child0_child2_child0_child0", |s| {
        s.size.width = Length::Px(5.0);
    });
    let root_child0_child0_child2_child0_child1 = t.child(root_child0_child0_child2_child0, "root_child0_child0_child2_child0_child1", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
        s.aspect_ratio = Some(1.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 300.0, 300.0]),
        (root_child0, [0.0, 0.0, 300.0, 300.0]),
        (root_child0_child0, [0.0, 0.0, 300.0, 197.0]),
        (root_child0_child0_child0, [0.0, 0.0, 197.0, 197.0]),
        (root_child0_child0_child1, [197.0, 0.0, 5.0, 197.0]),
        (root_child0_child0_child2, [202.0, 0.0, 98.0, 197.0]),
        (root_child0_child0_child2_child0, [0.0, 0.0, 98.0, 197.0]),
        (root_child0_child0_child2_child0_child0, [0.0, 0.0, 5.0, 0.0]),
        (root_child0_child0_child2_child0_child1, [0.0, 0.0, 98.0, 197.0]),
    ]);
}

#[test]
fn zero_aspect_ratio_behaves_like_auto() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(300.0);
        s.size.height = Length::Px(300.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.aspect_ratio = None;
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 300.0, 300.0]),
        (root_child0, [0.0, 0.0, 50.0, 0.0]),
    ]);
}
