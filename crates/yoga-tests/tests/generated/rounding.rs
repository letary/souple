// @generated — do not edit. Written by the Yoga importer (crates/yoga-tests/src/bin/import.rs):
//     cargo run -p souple-yoga-tests --bin import -- <path to yoga>
// From Yoga 3.2.1, tests/generated/YGRoundingTest.cpp (MIT, Copyright (c) Meta Platforms, Inc. and affiliates).

use crate::support::*;

#[test]
fn rounding_flex_basis_flex_grow_row_width_of_100() {
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
        s.flex_grow = 1.0;
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.flex_grow = 1.0;
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 33.0, 100.0]),
        (root_child1, [33.0, 0.0, 34.0, 100.0]),
        (root_child2, [67.0, 0.0, 33.0, 100.0]),
    ]);
}

#[test]
fn rounding_flex_basis_flex_grow_row_prime_number_width() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(113.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_grow = 1.0;
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.flex_grow = 1.0;
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.flex_grow = 1.0;
    });
    let root_child4 = t.child(root, "root_child4", |s| {
        s.flex_grow = 1.0;
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 113.0, 100.0]),
        (root_child0, [0.0, 0.0, 23.0, 100.0]),
        (root_child1, [23.0, 0.0, 22.0, 100.0]),
        (root_child2, [45.0, 0.0, 23.0, 100.0]),
        (root_child3, [68.0, 0.0, 22.0, 100.0]),
        (root_child4, [90.0, 0.0, 23.0, 100.0]),
    ]);
}

#[test]
fn rounding_flex_basis_flex_shrink_row() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(101.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Px(100.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_basis = Length::Px(25.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.flex_basis = Length::Px(25.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 101.0, 100.0]),
        (root_child0, [0.0, 0.0, 51.0, 100.0]),
        (root_child1, [51.0, 0.0, 25.0, 100.0]),
        (root_child2, [76.0, 0.0, 25.0, 100.0]),
    ]);
}

#[test]
fn rounding_flex_basis_overrides_main_size() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(113.0);
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
        (root, [0.0, 0.0, 100.0, 113.0]),
        (root_child0, [0.0, 0.0, 100.0, 64.0]),
        (root_child1, [0.0, 64.0, 100.0, 25.0]),
        (root_child2, [0.0, 89.0, 100.0, 24.0]),
    ]);
}

#[test]
fn rounding_total_fractial() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(87.4);
        s.size.height = Length::Px(113.4);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 0.7;
        s.flex_basis = Length::Px(50.3);
        s.size.height = Length::Px(20.3);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_grow = 1.6;
        s.size.height = Length::Px(10.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.flex_grow = 1.1;
        s.size.height = Length::Px(10.7);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 87.0, 113.0]),
        (root_child0, [0.0, 0.0, 87.0, 59.0]),
        (root_child1, [0.0, 59.0, 87.0, 30.0]),
        (root_child2, [0.0, 89.0, 87.0, 24.0]),
    ]);
}

#[test]
fn rounding_total_fractial_nested() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(87.4);
        s.size.height = Length::Px(113.4);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 0.7;
        s.flex_basis = Length::Px(50.3);
        s.size.height = Length::Px(20.3);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_basis = Length::Px(0.3);
        s.size.height = Length::Px(9.9);
        s.inset.bottom = Length::Px(13.3);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.flex_grow = 4.0;
        s.flex_basis = Length::Px(0.3);
        s.size.height = Length::Px(1.1);
        s.inset.top = Length::Px(13.3);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_grow = 1.6;
        s.size.height = Length::Px(10.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.flex_grow = 1.1;
        s.size.height = Length::Px(10.7);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 87.0, 113.0]),
        (root_child0, [0.0, 0.0, 87.0, 59.0]),
        (root_child0_child0, [0.0, -13.0, 87.0, 12.0]),
        (root_child0_child1, [0.0, 25.0, 87.0, 47.0]),
        (root_child1, [0.0, 59.0, 87.0, 30.0]),
        (root_child2, [0.0, 89.0, 87.0, 24.0]),
    ]);
}

#[test]
fn rounding_fractial_input_1() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(113.4);
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
        (root, [0.0, 0.0, 100.0, 113.0]),
        (root_child0, [0.0, 0.0, 100.0, 64.0]),
        (root_child1, [0.0, 64.0, 100.0, 25.0]),
        (root_child2, [0.0, 89.0, 100.0, 24.0]),
    ]);
}

#[test]
fn rounding_fractial_input_2() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(113.6);
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
        (root, [0.0, 0.0, 100.0, 114.0]),
        (root_child0, [0.0, 0.0, 100.0, 65.0]),
        (root_child1, [0.0, 65.0, 100.0, 24.0]),
        (root_child2, [0.0, 89.0, 100.0, 25.0]),
    ]);
}

#[test]
#[ignore = "by design: pixel snapping by absolute position; Yoga rounds each offset from its parent (root_child1: top 65, not 64)"]
fn rounding_fractial_input_3() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(113.4);
        s.inset.top = Length::Px(0.3);
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
        (root, [0.0, 0.0, 100.0, 114.0]),
        (root_child0, [0.0, 0.0, 100.0, 65.0]),
        (root_child1, [0.0, 64.0, 100.0, 24.0]),
        (root_child2, [0.0, 89.0, 100.0, 25.0]),
    ]);
}

#[test]
fn rounding_fractial_input_4() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(113.4);
        s.inset.top = Length::Px(0.7);
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
        (root, [0.0, 1.0, 100.0, 113.0]),
        (root_child0, [0.0, 0.0, 100.0, 64.0]),
        (root_child1, [0.0, 64.0, 100.0, 25.0]),
        (root_child2, [0.0, 89.0, 100.0, 24.0]),
    ]);
}

#[test]
fn rounding_inner_node_controversy_horizontal() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(320.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
        s.size.height = Length::Px(10.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_grow = 1.0;
        s.size.height = Length::Px(10.0);
    });
    let root_child1_child0 = t.child(root_child1, "root_child1_child0", |s| {
        s.flex_grow = 1.0;
        s.size.height = Length::Px(10.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.flex_grow = 1.0;
        s.size.height = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 320.0, 10.0]),
        (root_child0, [0.0, 0.0, 107.0, 10.0]),
        (root_child1, [107.0, 0.0, 106.0, 10.0]),
        (root_child1_child0, [0.0, 0.0, 106.0, 10.0]),
        (root_child2, [213.0, 0.0, 107.0, 10.0]),
    ]);
}

#[test]
fn rounding_inner_node_controversy_vertical() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.height = Length::Px(320.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
        s.size.width = Length::Px(10.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_grow = 1.0;
        s.size.width = Length::Px(10.0);
    });
    let root_child1_child0 = t.child(root_child1, "root_child1_child0", |s| {
        s.flex_grow = 1.0;
        s.size.width = Length::Px(10.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.flex_grow = 1.0;
        s.size.width = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 10.0, 320.0]),
        (root_child0, [0.0, 0.0, 10.0, 107.0]),
        (root_child1, [0.0, 107.0, 10.0, 106.0]),
        (root_child1_child0, [0.0, 0.0, 10.0, 106.0]),
        (root_child2, [0.0, 213.0, 10.0, 107.0]),
    ]);
}

#[test]
fn rounding_inner_node_controversy_combined() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(640.0);
        s.size.height = Length::Px(320.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
        s.size.height = Length::Percent(100.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_grow = 1.0;
        s.size.height = Length::Percent(100.0);
    });
    let root_child1_child0 = t.child(root_child1, "root_child1_child0", |s| {
        s.flex_grow = 1.0;
        s.size.width = Length::Percent(100.0);
    });
    let root_child1_child1 = t.child(root_child1, "root_child1_child1", |s| {
        s.flex_grow = 1.0;
        s.size.width = Length::Percent(100.0);
    });
    let root_child1_child1_child0 = t.child(root_child1_child1, "root_child1_child1_child0", |s| {
        s.flex_grow = 1.0;
        s.size.width = Length::Percent(100.0);
    });
    let root_child1_child2 = t.child(root_child1, "root_child1_child2", |s| {
        s.flex_grow = 1.0;
        s.size.width = Length::Percent(100.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.flex_grow = 1.0;
        s.size.height = Length::Percent(100.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 640.0, 320.0]),
        (root_child0, [0.0, 0.0, 213.0, 320.0]),
        (root_child1, [213.0, 0.0, 214.0, 320.0]),
        (root_child1_child0, [0.0, 0.0, 214.0, 107.0]),
        (root_child1_child1, [0.0, 107.0, 214.0, 106.0]),
        (root_child1_child1_child0, [0.0, 0.0, 214.0, 106.0]),
        (root_child1_child2, [0.0, 213.0, 214.0, 107.0]),
        (root_child2, [427.0, 0.0, 213.0, 320.0]),
    ]);
}
