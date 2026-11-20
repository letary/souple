// @generated — do not edit. Written by the Yoga importer (crates/yoga-tests/src/bin/import.rs):
//     cargo run -p souple-yoga-tests --bin import -- <path to yoga>
// From Yoga 3.2.1, tests/generated/YGMarginTest.cpp (MIT, Copyright (c) Meta Platforms, Inc. and affiliates).

use crate::support::*;

#[test]
fn margin_start() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(10.0);
        s.margin.left = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [10.0, 0.0, 10.0, 100.0]),
    ]);
}

#[test]
fn margin_top() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.height = Length::Px(10.0);
        s.margin.top = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 10.0, 100.0, 10.0]),
    ]);
}

#[test]
fn margin_end() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.justify_content = JustifyContent::FlexEnd;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(10.0);
        s.margin.right = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [80.0, 0.0, 10.0, 100.0]),
    ]);
}

#[test]
fn margin_bottom() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.justify_content = JustifyContent::FlexEnd;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.height = Length::Px(10.0);
        s.margin.bottom = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 80.0, 100.0, 10.0]),
    ]);
}

#[test]
fn margin_and_flex_row() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
        s.margin.left = Length::Px(10.0);
        s.margin.right = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [10.0, 0.0, 80.0, 100.0]),
    ]);
}

#[test]
fn margin_and_flex_column() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
        s.margin.top = Length::Px(10.0);
        s.margin.bottom = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 10.0, 100.0, 80.0]),
    ]);
}

#[test]
fn margin_and_stretch_row() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
        s.margin.top = Length::Px(10.0);
        s.margin.bottom = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 10.0, 100.0, 80.0]),
    ]);
}

#[test]
fn margin_and_stretch_column() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
        s.margin.left = Length::Px(10.0);
        s.margin.right = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [10.0, 0.0, 80.0, 100.0]),
    ]);
}

#[test]
fn margin_with_sibling_row() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
        s.margin.right = Length::Px(10.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_grow = 1.0;
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 45.0, 100.0]),
        (root_child1, [55.0, 0.0, 45.0, 100.0]),
    ]);
}

#[test]
fn margin_with_sibling_column() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
        s.margin.bottom = Length::Px(10.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_grow = 1.0;
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 100.0, 45.0]),
        (root_child1, [0.0, 55.0, 100.0, 45.0]),
    ]);
}

#[test]
fn margin_auto_bottom() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.align_items = AlignItems::Center;
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.margin.bottom = Length::Auto;
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [75.0, 0.0, 50.0, 50.0]),
        (root_child1, [75.0, 150.0, 50.0, 50.0]),
    ]);
}

#[test]
fn margin_auto_top() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.align_items = AlignItems::Center;
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.margin.top = Length::Auto;
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [75.0, 100.0, 50.0, 50.0]),
        (root_child1, [75.0, 150.0, 50.0, 50.0]),
    ]);
}

#[test]
fn margin_auto_bottom_and_top() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.align_items = AlignItems::Center;
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.margin.top = Length::Auto;
        s.margin.bottom = Length::Auto;
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [75.0, 50.0, 50.0, 50.0]),
        (root_child1, [75.0, 150.0, 50.0, 50.0]),
    ]);
}

#[test]
fn margin_auto_bottom_and_top_justify_center() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.justify_content = JustifyContent::Center;
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.margin.top = Length::Auto;
        s.margin.bottom = Length::Auto;
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 50.0, 50.0, 50.0]),
        (root_child1, [0.0, 150.0, 50.0, 50.0]),
    ]);
}

#[test]
fn margin_auto_multiple_children_column() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.align_items = AlignItems::Center;
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.margin.top = Length::Auto;
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.margin.top = Length::Auto;
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [75.0, 25.0, 50.0, 50.0]),
        (root_child1, [75.0, 100.0, 50.0, 50.0]),
        (root_child2, [75.0, 150.0, 50.0, 50.0]),
    ]);
}

#[test]
fn margin_auto_multiple_children_row() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.align_items = AlignItems::Center;
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.margin.right = Length::Auto;
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.margin.right = Length::Auto;
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 75.0, 50.0, 50.0]),
        (root_child1, [75.0, 75.0, 50.0, 50.0]),
        (root_child2, [150.0, 75.0, 50.0, 50.0]),
    ]);
}

#[test]
fn margin_auto_left_and_right_column() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.align_items = AlignItems::Center;
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.margin.left = Length::Auto;
        s.margin.right = Length::Auto;
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [50.0, 75.0, 50.0, 50.0]),
        (root_child1, [150.0, 75.0, 50.0, 50.0]),
    ]);
}

#[test]
fn margin_auto_left_and_right() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.margin.left = Length::Auto;
        s.margin.right = Length::Auto;
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [75.0, 0.0, 50.0, 50.0]),
        (root_child1, [0.0, 50.0, 50.0, 50.0]),
    ]);
}

#[test]
fn margin_auto_start_and_end_column() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.align_items = AlignItems::Center;
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.margin.left = Length::Auto;
        s.margin.right = Length::Auto;
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [50.0, 75.0, 50.0, 50.0]),
        (root_child1, [150.0, 75.0, 50.0, 50.0]),
    ]);
}

#[test]
fn margin_auto_start_and_end() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.margin.left = Length::Auto;
        s.margin.right = Length::Auto;
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [75.0, 0.0, 50.0, 50.0]),
        (root_child1, [0.0, 50.0, 50.0, 50.0]),
    ]);
}

#[test]
fn margin_auto_left_and_right_column_and_center() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.align_items = AlignItems::Center;
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.margin.left = Length::Auto;
        s.margin.right = Length::Auto;
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [75.0, 0.0, 50.0, 50.0]),
        (root_child1, [75.0, 50.0, 50.0, 50.0]),
    ]);
}

#[test]
fn margin_auto_left() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.align_items = AlignItems::Center;
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.margin.left = Length::Auto;
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [150.0, 0.0, 50.0, 50.0]),
        (root_child1, [75.0, 50.0, 50.0, 50.0]),
    ]);
}

#[test]
fn margin_auto_right() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.align_items = AlignItems::Center;
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.margin.right = Length::Auto;
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 50.0, 50.0]),
        (root_child1, [75.0, 50.0, 50.0, 50.0]),
    ]);
}

#[test]
fn margin_auto_left_and_right_stretch() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.margin.left = Length::Auto;
        s.margin.right = Length::Auto;
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [50.0, 0.0, 50.0, 50.0]),
        (root_child1, [150.0, 0.0, 50.0, 50.0]),
    ]);
}

#[test]
fn margin_auto_top_and_bottom_stretch() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.margin.top = Length::Auto;
        s.margin.bottom = Length::Auto;
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 50.0, 50.0, 50.0]),
        (root_child1, [0.0, 150.0, 50.0, 50.0]),
    ]);
}

#[test]
fn margin_should_not_be_part_of_max_height() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(250.0);
        s.size.height = Length::Px(250.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.max_size.height = Length::Px(100.0);
        s.margin.top = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 250.0, 250.0]),
        (root_child0, [0.0, 20.0, 100.0, 100.0]),
    ]);
}

#[test]
fn margin_should_not_be_part_of_max_width() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(250.0);
        s.size.height = Length::Px(250.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.max_size.width = Length::Px(100.0);
        s.margin.left = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 250.0, 250.0]),
        (root_child0, [20.0, 0.0, 100.0, 100.0]),
    ]);
}

#[test]
fn margin_auto_left_right_child_bigger_than_parent() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.justify_content = JustifyContent::Center;
        s.size.width = Length::Px(52.0);
        s.size.height = Length::Px(52.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(72.0);
        s.size.height = Length::Px(72.0);
        s.margin.left = Length::Auto;
        s.margin.right = Length::Auto;
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 52.0, 52.0]),
        (root_child0, [0.0, -10.0, 72.0, 72.0]),
    ]);
}

#[test]
fn margin_auto_left_child_bigger_than_parent() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.justify_content = JustifyContent::Center;
        s.size.width = Length::Px(52.0);
        s.size.height = Length::Px(52.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(72.0);
        s.size.height = Length::Px(72.0);
        s.margin.left = Length::Auto;
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 52.0, 52.0]),
        (root_child0, [0.0, -10.0, 72.0, 72.0]),
    ]);
}

#[test]
fn margin_fix_left_auto_right_child_bigger_than_parent() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.justify_content = JustifyContent::Center;
        s.size.width = Length::Px(52.0);
        s.size.height = Length::Px(52.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(72.0);
        s.size.height = Length::Px(72.0);
        s.margin.left = Length::Px(10.0);
        s.margin.right = Length::Auto;
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 52.0, 52.0]),
        (root_child0, [10.0, -10.0, 72.0, 72.0]),
    ]);
}

#[test]
fn margin_auto_left_fix_right_child_bigger_than_parent() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.justify_content = JustifyContent::Center;
        s.size.width = Length::Px(52.0);
        s.size.height = Length::Px(52.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(72.0);
        s.size.height = Length::Px(72.0);
        s.margin.left = Length::Auto;
        s.margin.right = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 52.0, 52.0]),
        (root_child0, [0.0, -10.0, 72.0, 72.0]),
    ]);
}

#[test]
fn margin_auto_top_stretching_child() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.align_items = AlignItems::Center;
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
        s.margin.top = Length::Auto;
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [100.0, 0.0, 0.0, 150.0]),
        (root_child1, [75.0, 150.0, 50.0, 50.0]),
    ]);
}

#[test]
fn margin_auto_left_stretching_child() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.align_items = AlignItems::Center;
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
        s.margin.left = Length::Auto;
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [200.0, 0.0, 0.0, 150.0]),
        (root_child1, [75.0, 150.0, 50.0, 50.0]),
    ]);
}

#[test]
fn margin_auto_overflowing_container() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.align_items = AlignItems::Center;
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(150.0);
        s.margin.bottom = Length::Auto;
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(150.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [75.0, 0.0, 50.0, 150.0]),
        (root_child1, [75.0, 150.0, 50.0, 150.0]),
    ]);
}
