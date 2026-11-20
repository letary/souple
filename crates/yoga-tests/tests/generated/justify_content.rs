// @generated — do not edit. Written by the Yoga importer (crates/yoga-tests/src/bin/import.rs):
//     cargo run -p souple-yoga-tests --bin import -- <path to yoga>
// From Yoga 3.2.1, tests/generated/YGJustifyContentTest.cpp (MIT, Copyright (c) Meta Platforms, Inc. and affiliates).

use crate::support::*;

#[test]
fn justify_content_row_flex_start() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(102.0);
        s.size.height = Length::Px(102.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(10.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(10.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 102.0, 102.0]),
        (root_child0, [0.0, 0.0, 10.0, 102.0]),
        (root_child1, [10.0, 0.0, 10.0, 102.0]),
        (root_child2, [20.0, 0.0, 10.0, 102.0]),
    ]);
}

#[test]
fn justify_content_row_flex_end() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.justify_content = JustifyContent::FlexEnd;
        s.size.width = Length::Px(102.0);
        s.size.height = Length::Px(102.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(10.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(10.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 102.0, 102.0]),
        (root_child0, [72.0, 0.0, 10.0, 102.0]),
        (root_child1, [82.0, 0.0, 10.0, 102.0]),
        (root_child2, [92.0, 0.0, 10.0, 102.0]),
    ]);
}

#[test]
fn justify_content_row_center() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.justify_content = JustifyContent::Center;
        s.size.width = Length::Px(102.0);
        s.size.height = Length::Px(102.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(10.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(10.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 102.0, 102.0]),
        (root_child0, [36.0, 0.0, 10.0, 102.0]),
        (root_child1, [46.0, 0.0, 10.0, 102.0]),
        (root_child2, [56.0, 0.0, 10.0, 102.0]),
    ]);
}

#[test]
fn justify_content_row_space_between() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.justify_content = JustifyContent::SpaceBetween;
        s.size.width = Length::Px(102.0);
        s.size.height = Length::Px(102.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(10.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(10.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 102.0, 102.0]),
        (root_child0, [0.0, 0.0, 10.0, 102.0]),
        (root_child1, [46.0, 0.0, 10.0, 102.0]),
        (root_child2, [92.0, 0.0, 10.0, 102.0]),
    ]);
}

#[test]
fn justify_content_row_space_around() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.justify_content = JustifyContent::SpaceAround;
        s.size.width = Length::Px(102.0);
        s.size.height = Length::Px(102.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(10.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(10.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 102.0, 102.0]),
        (root_child0, [12.0, 0.0, 10.0, 102.0]),
        (root_child1, [46.0, 0.0, 10.0, 102.0]),
        (root_child2, [80.0, 0.0, 10.0, 102.0]),
    ]);
}

#[test]
fn justify_content_column_flex_start() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(102.0);
        s.size.height = Length::Px(102.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.height = Length::Px(10.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.height = Length::Px(10.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.height = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 102.0, 102.0]),
        (root_child0, [0.0, 0.0, 102.0, 10.0]),
        (root_child1, [0.0, 10.0, 102.0, 10.0]),
        (root_child2, [0.0, 20.0, 102.0, 10.0]),
    ]);
}

#[test]
fn justify_content_column_flex_end() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.justify_content = JustifyContent::FlexEnd;
        s.size.width = Length::Px(102.0);
        s.size.height = Length::Px(102.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.height = Length::Px(10.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.height = Length::Px(10.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.height = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 102.0, 102.0]),
        (root_child0, [0.0, 72.0, 102.0, 10.0]),
        (root_child1, [0.0, 82.0, 102.0, 10.0]),
        (root_child2, [0.0, 92.0, 102.0, 10.0]),
    ]);
}

#[test]
fn justify_content_column_center() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.justify_content = JustifyContent::Center;
        s.size.width = Length::Px(102.0);
        s.size.height = Length::Px(102.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.height = Length::Px(10.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.height = Length::Px(10.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.height = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 102.0, 102.0]),
        (root_child0, [0.0, 36.0, 102.0, 10.0]),
        (root_child1, [0.0, 46.0, 102.0, 10.0]),
        (root_child2, [0.0, 56.0, 102.0, 10.0]),
    ]);
}

#[test]
fn justify_content_column_space_between() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.justify_content = JustifyContent::SpaceBetween;
        s.size.width = Length::Px(102.0);
        s.size.height = Length::Px(102.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.height = Length::Px(10.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.height = Length::Px(10.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.height = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 102.0, 102.0]),
        (root_child0, [0.0, 0.0, 102.0, 10.0]),
        (root_child1, [0.0, 46.0, 102.0, 10.0]),
        (root_child2, [0.0, 92.0, 102.0, 10.0]),
    ]);
}

#[test]
fn justify_content_column_space_around() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.justify_content = JustifyContent::SpaceAround;
        s.size.width = Length::Px(102.0);
        s.size.height = Length::Px(102.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.height = Length::Px(10.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.height = Length::Px(10.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.height = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 102.0, 102.0]),
        (root_child0, [0.0, 12.0, 102.0, 10.0]),
        (root_child1, [0.0, 46.0, 102.0, 10.0]),
        (root_child2, [0.0, 80.0, 102.0, 10.0]),
    ]);
}

#[test]
fn justify_content_row_min_width_and_margin() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.justify_content = JustifyContent::Center;
        s.min_size.width = Length::Px(50.0);
        s.margin.left = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [100.0, 0.0, 50.0, 20.0]),
        (root_child0, [15.0, 0.0, 20.0, 20.0]),
    ]);
}

#[test]
fn justify_content_row_max_width_and_margin() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.justify_content = JustifyContent::Center;
        s.size.width = Length::Px(100.0);
        s.max_size.width = Length::Px(80.0);
        s.margin.left = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [100.0, 0.0, 80.0, 20.0]),
        (root_child0, [30.0, 0.0, 20.0, 20.0]),
    ]);
}

#[test]
fn justify_content_column_min_height_and_margin() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.justify_content = JustifyContent::Center;
        s.min_size.height = Length::Px(50.0);
        s.margin.top = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 100.0, 20.0, 50.0]),
        (root_child0, [0.0, 15.0, 20.0, 20.0]),
    ]);
}

#[test]
fn justify_content_column_max_height_and_margin() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.justify_content = JustifyContent::Center;
        s.size.height = Length::Px(100.0);
        s.max_size.height = Length::Px(80.0);
        s.margin.top = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 100.0, 20.0, 80.0]),
        (root_child0, [0.0, 30.0, 20.0, 20.0]),
    ]);
}

#[test]
fn justify_content_column_space_evenly() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.justify_content = JustifyContent::SpaceEvenly;
        s.size.width = Length::Px(102.0);
        s.size.height = Length::Px(102.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.height = Length::Px(10.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.height = Length::Px(10.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.height = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 102.0, 102.0]),
        (root_child0, [0.0, 18.0, 102.0, 10.0]),
        (root_child1, [0.0, 46.0, 102.0, 10.0]),
        (root_child2, [0.0, 74.0, 102.0, 10.0]),
    ]);
}

#[test]
fn justify_content_row_space_evenly() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.justify_content = JustifyContent::SpaceEvenly;
        s.size.width = Length::Px(102.0);
        s.size.height = Length::Px(102.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.height = Length::Px(10.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.height = Length::Px(10.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.height = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 102.0, 102.0]),
        (root_child0, [26.0, 0.0, 0.0, 10.0]),
        (root_child1, [51.0, 0.0, 0.0, 10.0]),
        (root_child2, [77.0, 0.0, 0.0, 10.0]),
    ]);
}

#[test]
fn justify_content_min_width_with_padding_child_width_greater_than_parent() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.align_content = AlignContent::Stretch;
        s.size.width = Length::Px(1000.0);
        s.size.height = Length::Px(1584.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::Row;
        s.align_content = AlignContent::Stretch;
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.flex_direction = FlexDirection::Row;
        s.justify_content = JustifyContent::Center;
        s.align_content = AlignContent::Stretch;
        s.min_size.width = Length::Px(400.0);
        s.padding.left = Length::Px(100.0);
        s.padding.right = Length::Px(100.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.flex_direction = FlexDirection::Row;
        s.align_content = AlignContent::Stretch;
        s.size.width = Length::Px(300.0);
        s.size.height = Length::Px(100.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 1000.0, 1584.0]),
        (root_child0, [0.0, 0.0, 1000.0, 100.0]),
        (root_child0_child0, [0.0, 0.0, 500.0, 100.0]),
        (root_child0_child0_child0, [100.0, 0.0, 300.0, 100.0]),
    ]);
}

#[test]
fn justify_content_min_width_with_padding_child_width_lower_than_parent() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.align_content = AlignContent::Stretch;
        s.size.width = Length::Px(1080.0);
        s.size.height = Length::Px(1584.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::Row;
        s.align_content = AlignContent::Stretch;
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.flex_direction = FlexDirection::Row;
        s.justify_content = JustifyContent::Center;
        s.align_content = AlignContent::Stretch;
        s.min_size.width = Length::Px(400.0);
        s.padding.left = Length::Px(100.0);
        s.padding.right = Length::Px(100.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.flex_direction = FlexDirection::Row;
        s.align_content = AlignContent::Stretch;
        s.size.width = Length::Px(199.0);
        s.size.height = Length::Px(100.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 1080.0, 1584.0]),
        (root_child0, [0.0, 0.0, 1080.0, 100.0]),
        (root_child0_child0, [0.0, 0.0, 400.0, 100.0]),
        (root_child0_child0_child0, [101.0, 0.0, 199.0, 100.0]),
    ]);
}

#[test]
fn justify_content_space_between_indefinite_container_dim_with_free_space() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.align_items = AlignItems::Center;
        s.size.width = Length::Px(300.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::Row;
        s.justify_content = JustifyContent::SpaceBetween;
        s.min_size.width = Length::Px(200.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 300.0, 50.0]),
        (root_child0, [50.0, 0.0, 200.0, 50.0]),
        (root_child0_child0, [0.0, 0.0, 50.0, 50.0]),
        (root_child0_child1, [150.0, 0.0, 50.0, 50.0]),
    ]);
}

#[test]
fn justify_content_flex_start_row_reverse() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::RowReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
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
        (root_child0, [80.0, 0.0, 20.0, 100.0]),
        (root_child1, [60.0, 0.0, 20.0, 100.0]),
        (root_child2, [40.0, 0.0, 20.0, 100.0]),
    ]);
}

#[test]
fn justify_content_flex_end_row_reverse() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::RowReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
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
        (root_child0, [80.0, 0.0, 20.0, 100.0]),
        (root_child1, [60.0, 0.0, 20.0, 100.0]),
        (root_child2, [40.0, 0.0, 20.0, 100.0]),
    ]);
}

#[test]
fn justify_content_overflow_row_flex_start() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(102.0);
        s.size.height = Length::Px(102.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(40.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(40.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(40.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 102.0, 102.0]),
        (root_child0, [0.0, 0.0, 40.0, 102.0]),
        (root_child1, [40.0, 0.0, 40.0, 102.0]),
        (root_child2, [80.0, 0.0, 40.0, 102.0]),
    ]);
}

#[test]
fn justify_content_overflow_row_flex_end() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.justify_content = JustifyContent::FlexEnd;
        s.size.width = Length::Px(102.0);
        s.size.height = Length::Px(102.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(40.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(40.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(40.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 102.0, 102.0]),
        (root_child0, [-18.0, 0.0, 40.0, 102.0]),
        (root_child1, [22.0, 0.0, 40.0, 102.0]),
        (root_child2, [62.0, 0.0, 40.0, 102.0]),
    ]);
}

#[test]
fn justify_content_overflow_row_center() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.justify_content = JustifyContent::Center;
        s.size.width = Length::Px(102.0);
        s.size.height = Length::Px(102.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(40.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(40.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(40.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 102.0, 102.0]),
        (root_child0, [-9.0, 0.0, 40.0, 102.0]),
        (root_child1, [31.0, 0.0, 40.0, 102.0]),
        (root_child2, [71.0, 0.0, 40.0, 102.0]),
    ]);
}

#[test]
fn justify_content_overflow_row_space_between() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.justify_content = JustifyContent::SpaceBetween;
        s.size.width = Length::Px(102.0);
        s.size.height = Length::Px(102.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(40.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(40.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(40.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 102.0, 102.0]),
        (root_child0, [0.0, 0.0, 40.0, 102.0]),
        (root_child1, [40.0, 0.0, 40.0, 102.0]),
        (root_child2, [80.0, 0.0, 40.0, 102.0]),
    ]);
}

#[test]
fn justify_content_overflow_row_space_around() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.justify_content = JustifyContent::SpaceAround;
        s.size.width = Length::Px(102.0);
        s.size.height = Length::Px(102.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(40.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(40.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(40.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 102.0, 102.0]),
        (root_child0, [0.0, 0.0, 40.0, 102.0]),
        (root_child1, [40.0, 0.0, 40.0, 102.0]),
        (root_child2, [80.0, 0.0, 40.0, 102.0]),
    ]);
}

#[test]
fn justify_content_overflow_row_space_evenly() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.justify_content = JustifyContent::SpaceEvenly;
        s.size.width = Length::Px(102.0);
        s.size.height = Length::Px(102.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(40.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(40.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(40.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 102.0, 102.0]),
        (root_child0, [0.0, 0.0, 40.0, 102.0]),
        (root_child1, [40.0, 0.0, 40.0, 102.0]),
        (root_child2, [80.0, 0.0, 40.0, 102.0]),
    ]);
}

#[test]
#[ignore = "skipped by Yoga: GTEST_SKIP, Yoga fails it too"]
fn justify_content_overflow_row_reverse_space_around() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::RowReverse;
        s.justify_content = JustifyContent::SpaceAround;
        s.size.width = Length::Px(102.0);
        s.size.height = Length::Px(102.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(40.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(40.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(40.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 102.0, 102.0]),
        (root_child0, [80.0, 0.0, 40.0, 102.0]),
        (root_child1, [40.0, 0.0, 40.0, 102.0]),
        (root_child2, [0.0, 0.0, 40.0, 102.0]),
    ]);
}

#[test]
#[ignore = "skipped by Yoga: GTEST_SKIP, Yoga fails it too"]
fn justify_content_overflow_row_reverse_space_evenly() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::RowReverse;
        s.justify_content = JustifyContent::SpaceEvenly;
        s.size.width = Length::Px(102.0);
        s.size.height = Length::Px(102.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(40.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(40.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(40.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 102.0, 102.0]),
        (root_child0, [80.0, 0.0, 40.0, 102.0]),
        (root_child1, [40.0, 0.0, 40.0, 102.0]),
        (root_child2, [0.0, 0.0, 40.0, 102.0]),
    ]);
}

#[test]
fn justify_content_overflow_row_space_evenly_auto_margin() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.justify_content = JustifyContent::SpaceEvenly;
        s.size.width = Length::Px(102.0);
        s.size.height = Length::Px(102.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(40.0);
        s.margin.right = Length::Auto;
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(40.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(40.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 102.0, 102.0]),
        (root_child0, [0.0, 0.0, 40.0, 102.0]),
        (root_child1, [40.0, 0.0, 40.0, 102.0]),
        (root_child2, [80.0, 0.0, 40.0, 102.0]),
    ]);
}
