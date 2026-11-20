// @generated — do not edit. Written by the Yoga importer (crates/yoga-tests/src/bin/import.rs):
//     cargo run -p souple-yoga-tests --bin import -- <path to yoga>
// From Yoga 3.2.1, tests/generated/YGFlexTest.cpp (MIT, Copyright (c) Meta Platforms, Inc. and affiliates).

use crate::support::*;

#[test]
fn flex_basis_flex_grow_column() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_basis = Length::Px(50.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_grow = 1.0;
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 100.0, 75.0]),
        (root_child1, [0.0, 75.0, 100.0, 25.0]),
    ]);
}

#[test]
fn flex_shrink_flex_grow_row() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(500.0);
        s.size.height = Length::Px(500.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_shrink = 1.0;
        s.size.width = Length::Px(500.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_shrink = 1.0;
        s.size.width = Length::Px(500.0);
        s.size.height = Length::Px(100.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 500.0, 500.0]),
        (root_child0, [0.0, 0.0, 250.0, 100.0]),
        (root_child1, [250.0, 0.0, 250.0, 100.0]),
    ]);
}

#[test]
fn flex_shrink_flex_grow_child_flex_shrink_other_child() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(500.0);
        s.size.height = Length::Px(500.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_shrink = 1.0;
        s.size.width = Length::Px(500.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.size.width = Length::Px(500.0);
        s.size.height = Length::Px(100.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 500.0, 500.0]),
        (root_child0, [0.0, 0.0, 250.0, 100.0]),
        (root_child1, [250.0, 0.0, 250.0, 100.0]),
    ]);
}

#[test]
fn flex_basis_flex_grow_row() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_basis = Length::Px(50.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_grow = 1.0;
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 75.0, 100.0]),
        (root_child1, [75.0, 0.0, 25.0, 100.0]),
    ]);
}

#[test]
fn flex_basis_flex_shrink_column() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Px(100.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_basis = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 100.0, 50.0]),
        (root_child1, [0.0, 50.0, 100.0, 50.0]),
    ]);
}

#[test]
fn flex_basis_flex_shrink_row() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Px(100.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_basis = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 50.0, 100.0]),
        (root_child1, [50.0, 0.0, 50.0, 100.0]),
    ]);
}

#[test]
fn flex_shrink_to_zero() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.height = Length::Px(75.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_shrink = 1.0;
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 50.0, 75.0]),
        (root_child0, [0.0, 0.0, 50.0, 50.0]),
        (root_child1, [0.0, 50.0, 50.0, 0.0]),
        (root_child2, [0.0, 50.0, 50.0, 50.0]),
    ]);
}

#[test]
fn flex_basis_overrides_main_size() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_basis = Length::Px(50.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_grow = 1.0;
        s.size.height = Length::Px(10.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.flex_grow = 1.0;
        s.size.height = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 100.0, 60.0]),
        (root_child1, [0.0, 60.0, 100.0, 20.0]),
        (root_child2, [0.0, 80.0, 100.0, 20.0]),
    ]);
}

#[test]
fn flex_grow_shrink_at_most() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |_| {});
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 100.0, 0.0]),
        (root_child0_child0, [0.0, 0.0, 100.0, 0.0]),
    ]);
}

#[test]
fn flex_grow_less_than_factor_one() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(500.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 0.2;
        s.flex_basis = Length::Px(40.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_grow = 0.2;
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.flex_grow = 0.4;
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 500.0]),
        (root_child0, [0.0, 0.0, 200.0, 132.0]),
        (root_child1, [0.0, 132.0, 200.0, 92.0]),
        (root_child2, [0.0, 224.0, 200.0, 184.0]),
    ]);
}
