// @generated — do not edit. Written by the Yoga importer (crates/yoga-tests/src/bin/import.rs):
//     cargo run -p souple-yoga-tests --bin import -- <path to yoga>
// From Yoga 3.2.1, tests/generated/YGAutoTest.cpp (MIT, Copyright (c) Meta Platforms, Inc. and affiliates).

use crate::support::*;

#[test]
fn auto_width() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Auto;
        s.size.height = Length::Px(50.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 150.0, 50.0]),
        (root_child0, [0.0, 0.0, 50.0, 50.0]),
        (root_child1, [50.0, 0.0, 50.0, 50.0]),
        (root_child2, [100.0, 0.0, 50.0, 50.0]),
    ]);
}

#[test]
fn auto_height() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Auto;
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 50.0, 150.0]),
        (root_child0, [0.0, 0.0, 50.0, 50.0]),
        (root_child1, [0.0, 50.0, 50.0, 50.0]),
        (root_child2, [0.0, 100.0, 50.0, 50.0]),
    ]);
}

#[test]
fn auto_flex_basis() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(50.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 50.0, 150.0]),
        (root_child0, [0.0, 0.0, 50.0, 50.0]),
        (root_child1, [0.0, 50.0, 50.0, 50.0]),
        (root_child2, [0.0, 100.0, 50.0, 50.0]),
    ]);
}

#[test]
fn auto_position() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(25.0);
        s.size.height = Length::Px(25.0);
        s.inset.right = Length::Auto;
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 50.0, 50.0]),
        (root_child0, [0.0, 0.0, 25.0, 25.0]),
    ]);
}

#[test]
fn auto_margin() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(25.0);
        s.size.height = Length::Px(25.0);
        s.margin.left = Length::Auto;
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 50.0, 50.0]),
        (root_child0, [25.0, 0.0, 25.0, 25.0]),
    ]);
}
