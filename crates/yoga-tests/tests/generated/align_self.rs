// @generated — do not edit. Written by the Yoga importer (crates/yoga-tests/src/bin/import.rs):
//     cargo run -p souple-yoga-tests --bin import -- <path to yoga>
// From Yoga 3.2.1, tests/generated/YGAlignSelfTest.cpp (MIT, Copyright (c) Meta Platforms, Inc. and affiliates).

use crate::support::*;

#[test]
fn align_self_center() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.align_self = Some(AlignItems::Center);
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
fn align_self_flex_end() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.align_self = Some(AlignItems::FlexEnd);
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
fn align_self_flex_start() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.align_self = Some(AlignItems::FlexStart);
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
fn align_self_flex_end_override_flex_start() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.align_items = AlignItems::FlexStart;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.align_self = Some(AlignItems::FlexEnd);
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
fn align_self_baseline() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.align_self = Some(AlignItems::Baseline);
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.align_self = Some(AlignItems::Baseline);
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
