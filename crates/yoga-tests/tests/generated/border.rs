// @generated — do not edit. Written by the Yoga importer (crates/yoga-tests/src/bin/import.rs):
//     cargo run -p souple-yoga-tests --bin import -- <path to yoga>
// From Yoga 3.2.1, tests/generated/YGBorderTest.cpp (MIT, Copyright (c) Meta Platforms, Inc. and affiliates).

use crate::support::*;

#[test]
fn border_no_size() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.border.left = Length::Px(10.0);
        s.border.top = Length::Px(10.0);
        s.border.right = Length::Px(10.0);
        s.border.bottom = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 20.0, 20.0]),
    ]);
}

#[test]
fn border_container_match_child() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.border.left = Length::Px(10.0);
        s.border.top = Length::Px(10.0);
        s.border.right = Length::Px(10.0);
        s.border.bottom = Length::Px(10.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(10.0);
        s.size.height = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 30.0, 30.0]),
        (root_child0, [10.0, 10.0, 10.0, 10.0]),
    ]);
}

#[test]
fn border_flex_child() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.border.left = Length::Px(10.0);
        s.border.top = Length::Px(10.0);
        s.border.right = Length::Px(10.0);
        s.border.bottom = Length::Px(10.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
        s.size.width = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [10.0, 10.0, 10.0, 80.0]),
    ]);
}

#[test]
fn border_stretch_child() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.border.left = Length::Px(10.0);
        s.border.top = Length::Px(10.0);
        s.border.right = Length::Px(10.0);
        s.border.bottom = Length::Px(10.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.height = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [10.0, 10.0, 80.0, 10.0]),
    ]);
}

#[test]
fn border_center_child() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.justify_content = JustifyContent::Center;
        s.align_items = AlignItems::Center;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.border.left = Length::Px(10.0);
        s.border.right = Length::Px(20.0);
        s.border.bottom = Length::Px(20.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(10.0);
        s.size.height = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [40.0, 35.0, 10.0, 10.0]),
    ]);
}
