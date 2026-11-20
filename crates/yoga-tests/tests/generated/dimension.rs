// @generated — do not edit. Written by the Yoga importer (crates/yoga-tests/src/bin/import.rs):
//     cargo run -p souple-yoga-tests --bin import -- <path to yoga>
// From Yoga 3.2.1, tests/generated/YGDimensionTest.cpp (MIT, Copyright (c) Meta Platforms, Inc. and affiliates).

use crate::support::*;

#[test]
fn wrap_child() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 100.0, 100.0]),
    ]);
}

#[test]
fn wrap_grandchild() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |_| {});
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0, [0.0, 0.0, 100.0, 100.0]),
    ]);
}
