// @generated — do not edit. Written by the Yoga importer (crates/yoga-tests/src/bin/import.rs):
//     cargo run -p souple-yoga-tests --bin import -- <path to yoga>
// From Yoga 3.2.1, tests/generated/YGFlexDirectionTest.cpp (MIT, Copyright (c) Meta Platforms, Inc. and affiliates).

use crate::support::*;

#[test]
fn flex_direction_column_no_height() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
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
        (root, [0.0, 0.0, 100.0, 30.0]),
        (root_child0, [0.0, 0.0, 100.0, 10.0]),
        (root_child1, [0.0, 10.0, 100.0, 10.0]),
        (root_child2, [0.0, 20.0, 100.0, 10.0]),
    ]);
}

#[test]
fn flex_direction_row_no_width() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.height = Length::Px(100.0);
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
        (root, [0.0, 0.0, 30.0, 100.0]),
        (root_child0, [0.0, 0.0, 10.0, 100.0]),
        (root_child1, [10.0, 0.0, 10.0, 100.0]),
        (root_child2, [20.0, 0.0, 10.0, 100.0]),
    ]);
}

#[test]
fn flex_direction_column() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
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
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 100.0, 10.0]),
        (root_child1, [0.0, 10.0, 100.0, 10.0]),
        (root_child2, [0.0, 20.0, 100.0, 10.0]),
    ]);
}

#[test]
fn flex_direction_row() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
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
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 10.0, 100.0]),
        (root_child1, [10.0, 0.0, 10.0, 100.0]),
        (root_child2, [20.0, 0.0, 10.0, 100.0]),
    ]);
}

#[test]
fn flex_direction_column_reverse() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::ColumnReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
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
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 90.0, 100.0, 10.0]),
        (root_child1, [0.0, 80.0, 100.0, 10.0]),
        (root_child2, [0.0, 70.0, 100.0, 10.0]),
    ]);
}

#[test]
fn flex_direction_row_reverse() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::RowReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
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
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [90.0, 0.0, 10.0, 100.0]),
        (root_child1, [80.0, 0.0, 10.0, 100.0]),
        (root_child2, [70.0, 0.0, 10.0, 100.0]),
    ]);
}

#[test]
fn flex_direction_row_reverse_margin_left() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::RowReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.margin.left = Length::Px(100.0);
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
        (root, [100.0, 0.0, 100.0, 100.0]),
        (root_child0, [90.0, 0.0, 10.0, 100.0]),
        (root_child1, [80.0, 0.0, 10.0, 100.0]),
        (root_child2, [70.0, 0.0, 10.0, 100.0]),
    ]);
}

#[test]
fn flex_direction_row_reverse_margin_start() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::RowReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.margin.left = Length::Px(100.0);
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
        (root, [100.0, 0.0, 100.0, 100.0]),
        (root_child0, [90.0, 0.0, 10.0, 100.0]),
        (root_child1, [80.0, 0.0, 10.0, 100.0]),
        (root_child2, [70.0, 0.0, 10.0, 100.0]),
    ]);
}

#[test]
fn flex_direction_row_reverse_margin_right() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::RowReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.margin.right = Length::Px(100.0);
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
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [90.0, 0.0, 10.0, 100.0]),
        (root_child1, [80.0, 0.0, 10.0, 100.0]),
        (root_child2, [70.0, 0.0, 10.0, 100.0]),
    ]);
}

#[test]
fn flex_direction_row_reverse_margin_end() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::RowReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.margin.right = Length::Px(100.0);
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
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [90.0, 0.0, 10.0, 100.0]),
        (root_child1, [80.0, 0.0, 10.0, 100.0]),
        (root_child2, [70.0, 0.0, 10.0, 100.0]),
    ]);
}

#[test]
fn flex_direction_column_reverse_margin_top() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::ColumnReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.margin.top = Length::Px(100.0);
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
        (root, [0.0, 100.0, 100.0, 100.0]),
        (root_child0, [0.0, 100.0, 10.0, 0.0]),
        (root_child1, [0.0, 100.0, 10.0, 0.0]),
        (root_child2, [0.0, 100.0, 10.0, 0.0]),
    ]);
}

#[test]
fn flex_direction_column_reverse_margin_bottom() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::ColumnReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.margin.bottom = Length::Px(100.0);
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
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 100.0, 10.0, 0.0]),
        (root_child1, [0.0, 100.0, 10.0, 0.0]),
        (root_child2, [0.0, 100.0, 10.0, 0.0]),
    ]);
}

#[test]
fn flex_direction_row_reverse_padding_left() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::RowReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.padding.left = Length::Px(100.0);
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
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [90.0, 0.0, 10.0, 100.0]),
        (root_child1, [80.0, 0.0, 10.0, 100.0]),
        (root_child2, [70.0, 0.0, 10.0, 100.0]),
    ]);
}

#[test]
fn flex_direction_row_reverse_padding_start() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::RowReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.padding.left = Length::Px(100.0);
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
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [90.0, 0.0, 10.0, 100.0]),
        (root_child1, [80.0, 0.0, 10.0, 100.0]),
        (root_child2, [70.0, 0.0, 10.0, 100.0]),
    ]);
}

#[test]
fn flex_direction_row_reverse_padding_right() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::RowReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.padding.right = Length::Px(100.0);
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
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [-10.0, 0.0, 10.0, 100.0]),
        (root_child1, [-20.0, 0.0, 10.0, 100.0]),
        (root_child2, [-30.0, 0.0, 10.0, 100.0]),
    ]);
}

#[test]
fn flex_direction_row_reverse_padding_end() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::RowReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.padding.right = Length::Px(100.0);
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
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [-10.0, 0.0, 10.0, 100.0]),
        (root_child1, [-20.0, 0.0, 10.0, 100.0]),
        (root_child2, [-30.0, 0.0, 10.0, 100.0]),
    ]);
}

#[test]
fn flex_direction_column_reverse_padding_top() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::ColumnReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.padding.top = Length::Px(100.0);
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
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 100.0, 10.0, 0.0]),
        (root_child1, [0.0, 100.0, 10.0, 0.0]),
        (root_child2, [0.0, 100.0, 10.0, 0.0]),
    ]);
}

#[test]
fn flex_direction_column_reverse_padding_bottom() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::ColumnReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.padding.bottom = Length::Px(100.0);
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
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 10.0, 0.0]),
        (root_child1, [0.0, 0.0, 10.0, 0.0]),
        (root_child2, [0.0, 0.0, 10.0, 0.0]),
    ]);
}

#[test]
fn flex_direction_row_reverse_border_left() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::RowReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.border.left = Length::Px(100.0);
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
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [90.0, 0.0, 10.0, 100.0]),
        (root_child1, [80.0, 0.0, 10.0, 100.0]),
        (root_child2, [70.0, 0.0, 10.0, 100.0]),
    ]);
}

#[test]
fn flex_direction_row_reverse_border_start() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::RowReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.border.left = Length::Px(100.0);
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
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [90.0, 0.0, 10.0, 100.0]),
        (root_child1, [80.0, 0.0, 10.0, 100.0]),
        (root_child2, [70.0, 0.0, 10.0, 100.0]),
    ]);
}

#[test]
fn flex_direction_row_reverse_border_right() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::RowReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.border.right = Length::Px(100.0);
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
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [-10.0, 0.0, 10.0, 100.0]),
        (root_child1, [-20.0, 0.0, 10.0, 100.0]),
        (root_child2, [-30.0, 0.0, 10.0, 100.0]),
    ]);
}

#[test]
fn flex_direction_row_reverse_border_end() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::RowReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.border.right = Length::Px(100.0);
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
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [-10.0, 0.0, 10.0, 100.0]),
        (root_child1, [-20.0, 0.0, 10.0, 100.0]),
        (root_child2, [-30.0, 0.0, 10.0, 100.0]),
    ]);
}

#[test]
fn flex_direction_column_reverse_border_top() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::ColumnReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.border.top = Length::Px(100.0);
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
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 100.0, 10.0, 0.0]),
        (root_child1, [0.0, 100.0, 10.0, 0.0]),
        (root_child2, [0.0, 100.0, 10.0, 0.0]),
    ]);
}

#[test]
fn flex_direction_column_reverse_border_bottom() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::ColumnReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.border.bottom = Length::Px(100.0);
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
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 10.0, 0.0]),
        (root_child1, [0.0, 0.0, 10.0, 0.0]),
        (root_child2, [0.0, 0.0, 10.0, 0.0]),
    ]);
}

#[test]
fn flex_direction_row_reverse_pos_left() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::RowReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.inset.left = Length::Px(100.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.size.width = Length::Px(10.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Px(10.0);
    });
    let root_child0_child2 = t.child(root_child0, "root_child0_child2", |s| {
        s.size.width = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [100.0, 0.0, 100.0, 100.0]),
        (root_child0_child0, [90.0, 0.0, 10.0, 100.0]),
        (root_child0_child1, [80.0, 0.0, 10.0, 100.0]),
        (root_child0_child2, [70.0, 0.0, 10.0, 100.0]),
    ]);
}

#[test]
fn flex_direction_row_reverse_pos_start() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::RowReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.inset.left = Length::Px(100.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.size.width = Length::Px(10.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Px(10.0);
    });
    let root_child0_child2 = t.child(root_child0, "root_child0_child2", |s| {
        s.size.width = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [100.0, 0.0, 100.0, 100.0]),
        (root_child0_child0, [90.0, 0.0, 10.0, 100.0]),
        (root_child0_child1, [80.0, 0.0, 10.0, 100.0]),
        (root_child0_child2, [70.0, 0.0, 10.0, 100.0]),
    ]);
}

#[test]
fn flex_direction_row_reverse_pos_right() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::RowReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.inset.right = Length::Px(100.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.size.width = Length::Px(10.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Px(10.0);
    });
    let root_child0_child2 = t.child(root_child0, "root_child0_child2", |s| {
        s.size.width = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [-100.0, 0.0, 100.0, 100.0]),
        (root_child0_child0, [90.0, 0.0, 10.0, 100.0]),
        (root_child0_child1, [80.0, 0.0, 10.0, 100.0]),
        (root_child0_child2, [70.0, 0.0, 10.0, 100.0]),
    ]);
}

#[test]
fn flex_direction_row_reverse_pos_end() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::RowReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.inset.right = Length::Px(100.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.size.width = Length::Px(10.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Px(10.0);
    });
    let root_child0_child2 = t.child(root_child0, "root_child0_child2", |s| {
        s.size.width = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [-100.0, 0.0, 100.0, 100.0]),
        (root_child0_child0, [90.0, 0.0, 10.0, 100.0]),
        (root_child0_child1, [80.0, 0.0, 10.0, 100.0]),
        (root_child0_child2, [70.0, 0.0, 10.0, 100.0]),
    ]);
}

#[test]
fn flex_direction_column_reverse_pos_top() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::ColumnReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.inset.top = Length::Px(100.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.size.width = Length::Px(10.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Px(10.0);
    });
    let root_child0_child2 = t.child(root_child0, "root_child0_child2", |s| {
        s.size.width = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 100.0, 100.0, 100.0]),
        (root_child0_child0, [0.0, 100.0, 10.0, 0.0]),
        (root_child0_child1, [0.0, 100.0, 10.0, 0.0]),
        (root_child0_child2, [0.0, 100.0, 10.0, 0.0]),
    ]);
}

#[test]
fn flex_direction_column_reverse_pos_bottom() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::ColumnReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.inset.bottom = Length::Px(100.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.size.width = Length::Px(10.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Px(10.0);
    });
    let root_child0_child2 = t.child(root_child0, "root_child0_child2", |s| {
        s.size.width = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, -100.0, 100.0, 100.0]),
        (root_child0_child0, [0.0, 100.0, 10.0, 0.0]),
        (root_child0_child1, [0.0, 100.0, 10.0, 0.0]),
        (root_child0_child2, [0.0, 100.0, 10.0, 0.0]),
    ]);
}

#[test]
fn flex_direction_row_reverse_inner_pos_left() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::RowReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(10.0);
        s.size.height = Length::Px(10.0);
        s.inset.left = Length::Px(10.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Px(10.0);
    });
    let root_child0_child2 = t.child(root_child0, "root_child0_child2", |s| {
        s.size.width = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0, [10.0, 0.0, 10.0, 10.0]),
        (root_child0_child1, [90.0, 0.0, 10.0, 100.0]),
        (root_child0_child2, [80.0, 0.0, 10.0, 100.0]),
    ]);
}

#[test]
fn flex_direction_row_reverse_inner_pos_right() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::RowReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(10.0);
        s.size.height = Length::Px(10.0);
        s.inset.right = Length::Px(10.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Px(10.0);
    });
    let root_child0_child2 = t.child(root_child0, "root_child0_child2", |s| {
        s.size.width = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0, [80.0, 0.0, 10.0, 10.0]),
        (root_child0_child1, [90.0, 0.0, 10.0, 100.0]),
        (root_child0_child2, [80.0, 0.0, 10.0, 100.0]),
    ]);
}

#[test]
fn flex_direction_col_reverse_inner_pos_top() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::ColumnReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(10.0);
        s.size.height = Length::Px(10.0);
        s.inset.top = Length::Px(10.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Px(10.0);
    });
    let root_child0_child2 = t.child(root_child0, "root_child0_child2", |s| {
        s.size.width = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0, [0.0, 10.0, 10.0, 10.0]),
        (root_child0_child1, [0.0, 100.0, 10.0, 0.0]),
        (root_child0_child2, [0.0, 100.0, 10.0, 0.0]),
    ]);
}

#[test]
fn flex_direction_col_reverse_inner_pos_bottom() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::ColumnReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(10.0);
        s.size.height = Length::Px(10.0);
        s.inset.bottom = Length::Px(10.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Px(10.0);
    });
    let root_child0_child2 = t.child(root_child0, "root_child0_child2", |s| {
        s.size.width = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0, [0.0, 80.0, 10.0, 10.0]),
        (root_child0_child1, [0.0, 100.0, 10.0, 0.0]),
        (root_child0_child2, [0.0, 100.0, 10.0, 0.0]),
    ]);
}

#[test]
#[ignore = "skipped by Yoga: GTEST_SKIP, Yoga fails it too"]
fn flex_direction_row_reverse_inner_pos_start() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::RowReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(10.0);
        s.size.height = Length::Px(10.0);
        s.inset.left = Length::Px(10.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Px(10.0);
    });
    let root_child0_child2 = t.child(root_child0, "root_child0_child2", |s| {
        s.size.width = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0, [10.0, 0.0, 10.0, 10.0]),
        (root_child0_child1, [90.0, 0.0, 10.0, 100.0]),
        (root_child0_child2, [80.0, 0.0, 10.0, 100.0]),
    ]);
}

#[test]
#[ignore = "skipped by Yoga: GTEST_SKIP, Yoga fails it too"]
fn flex_direction_row_reverse_inner_pos_end() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::RowReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(10.0);
        s.size.height = Length::Px(10.0);
        s.inset.right = Length::Px(10.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Px(10.0);
    });
    let root_child0_child2 = t.child(root_child0, "root_child0_child2", |s| {
        s.size.width = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0, [80.0, 0.0, 10.0, 10.0]),
        (root_child0_child1, [90.0, 0.0, 10.0, 100.0]),
        (root_child0_child2, [80.0, 0.0, 10.0, 100.0]),
    ]);
}

#[test]
fn flex_direction_row_reverse_inner_margin_left() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::RowReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(10.0);
        s.size.height = Length::Px(10.0);
        s.margin.left = Length::Px(10.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Px(10.0);
    });
    let root_child0_child2 = t.child(root_child0, "root_child0_child2", |s| {
        s.size.width = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0, [90.0, 0.0, 10.0, 10.0]),
        (root_child0_child1, [90.0, 0.0, 10.0, 100.0]),
        (root_child0_child2, [80.0, 0.0, 10.0, 100.0]),
    ]);
}

#[test]
fn flex_direction_row_reverse_inner_margin_right() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::RowReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(10.0);
        s.size.height = Length::Px(10.0);
        s.margin.right = Length::Px(10.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Px(10.0);
    });
    let root_child0_child2 = t.child(root_child0, "root_child0_child2", |s| {
        s.size.width = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0, [80.0, 0.0, 10.0, 10.0]),
        (root_child0_child1, [90.0, 0.0, 10.0, 100.0]),
        (root_child0_child2, [80.0, 0.0, 10.0, 100.0]),
    ]);
}

#[test]
fn flex_direction_col_reverse_inner_margin_top() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::ColumnReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(10.0);
        s.size.height = Length::Px(10.0);
        s.margin.top = Length::Px(10.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Px(10.0);
    });
    let root_child0_child2 = t.child(root_child0, "root_child0_child2", |s| {
        s.size.width = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0, [0.0, 90.0, 10.0, 10.0]),
        (root_child0_child1, [0.0, 100.0, 10.0, 0.0]),
        (root_child0_child2, [0.0, 100.0, 10.0, 0.0]),
    ]);
}

#[test]
fn flex_direction_col_reverse_inner_margin_bottom() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::ColumnReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(10.0);
        s.size.height = Length::Px(10.0);
        s.margin.bottom = Length::Px(10.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Px(10.0);
    });
    let root_child0_child2 = t.child(root_child0, "root_child0_child2", |s| {
        s.size.width = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0, [0.0, 80.0, 10.0, 10.0]),
        (root_child0_child1, [0.0, 100.0, 10.0, 0.0]),
        (root_child0_child2, [0.0, 100.0, 10.0, 0.0]),
    ]);
}

#[test]
fn flex_direction_row_reverse_inner_marign_start() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::RowReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(10.0);
        s.size.height = Length::Px(10.0);
        s.margin.left = Length::Px(10.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Px(10.0);
    });
    let root_child0_child2 = t.child(root_child0, "root_child0_child2", |s| {
        s.size.width = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0, [90.0, 0.0, 10.0, 10.0]),
        (root_child0_child1, [90.0, 0.0, 10.0, 100.0]),
        (root_child0_child2, [80.0, 0.0, 10.0, 100.0]),
    ]);
}

#[test]
fn flex_direction_row_reverse_inner_margin_end() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::RowReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(10.0);
        s.size.height = Length::Px(10.0);
        s.margin.right = Length::Px(10.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Px(10.0);
    });
    let root_child0_child2 = t.child(root_child0, "root_child0_child2", |s| {
        s.size.width = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0, [80.0, 0.0, 10.0, 10.0]),
        (root_child0_child1, [90.0, 0.0, 10.0, 100.0]),
        (root_child0_child2, [80.0, 0.0, 10.0, 100.0]),
    ]);
}

#[test]
fn flex_direction_row_reverse_inner_border_left() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::RowReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(10.0);
        s.size.height = Length::Px(10.0);
        s.border.left = Length::Px(10.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Px(10.0);
    });
    let root_child0_child2 = t.child(root_child0, "root_child0_child2", |s| {
        s.size.width = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0, [90.0, 0.0, 10.0, 10.0]),
        (root_child0_child1, [90.0, 0.0, 10.0, 100.0]),
        (root_child0_child2, [80.0, 0.0, 10.0, 100.0]),
    ]);
}

#[test]
fn flex_direction_row_reverse_inner_border_right() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::RowReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(10.0);
        s.size.height = Length::Px(10.0);
        s.border.right = Length::Px(10.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Px(10.0);
    });
    let root_child0_child2 = t.child(root_child0, "root_child0_child2", |s| {
        s.size.width = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0, [90.0, 0.0, 10.0, 10.0]),
        (root_child0_child1, [90.0, 0.0, 10.0, 100.0]),
        (root_child0_child2, [80.0, 0.0, 10.0, 100.0]),
    ]);
}

#[test]
fn flex_direction_col_reverse_inner_border_top() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::ColumnReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(10.0);
        s.size.height = Length::Px(10.0);
        s.border.top = Length::Px(10.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Px(10.0);
    });
    let root_child0_child2 = t.child(root_child0, "root_child0_child2", |s| {
        s.size.width = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0, [0.0, 90.0, 10.0, 10.0]),
        (root_child0_child1, [0.0, 100.0, 10.0, 0.0]),
        (root_child0_child2, [0.0, 100.0, 10.0, 0.0]),
    ]);
}

#[test]
fn flex_direction_col_reverse_inner_border_bottom() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::ColumnReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(10.0);
        s.size.height = Length::Px(10.0);
        s.border.bottom = Length::Px(10.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Px(10.0);
    });
    let root_child0_child2 = t.child(root_child0, "root_child0_child2", |s| {
        s.size.width = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0, [0.0, 90.0, 10.0, 10.0]),
        (root_child0_child1, [0.0, 100.0, 10.0, 0.0]),
        (root_child0_child2, [0.0, 100.0, 10.0, 0.0]),
    ]);
}

#[test]
fn flex_direction_row_reverse_inner_border_start() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::RowReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(10.0);
        s.size.height = Length::Px(10.0);
        s.border.left = Length::Px(10.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Px(10.0);
    });
    let root_child0_child2 = t.child(root_child0, "root_child0_child2", |s| {
        s.size.width = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0, [90.0, 0.0, 10.0, 10.0]),
        (root_child0_child1, [90.0, 0.0, 10.0, 100.0]),
        (root_child0_child2, [80.0, 0.0, 10.0, 100.0]),
    ]);
}

#[test]
fn flex_direction_row_reverse_inner_border_end() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::RowReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(10.0);
        s.size.height = Length::Px(10.0);
        s.border.right = Length::Px(10.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Px(10.0);
    });
    let root_child0_child2 = t.child(root_child0, "root_child0_child2", |s| {
        s.size.width = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0, [90.0, 0.0, 10.0, 10.0]),
        (root_child0_child1, [90.0, 0.0, 10.0, 100.0]),
        (root_child0_child2, [80.0, 0.0, 10.0, 100.0]),
    ]);
}

#[test]
fn flex_direction_row_reverse_inner_padding_left() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::RowReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(10.0);
        s.size.height = Length::Px(10.0);
        s.padding.left = Length::Px(10.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Px(10.0);
    });
    let root_child0_child2 = t.child(root_child0, "root_child0_child2", |s| {
        s.size.width = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0, [90.0, 0.0, 10.0, 10.0]),
        (root_child0_child1, [90.0, 0.0, 10.0, 100.0]),
        (root_child0_child2, [80.0, 0.0, 10.0, 100.0]),
    ]);
}

#[test]
fn flex_direction_row_reverse_inner_padding_right() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::RowReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(10.0);
        s.size.height = Length::Px(10.0);
        s.padding.right = Length::Px(10.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Px(10.0);
    });
    let root_child0_child2 = t.child(root_child0, "root_child0_child2", |s| {
        s.size.width = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0, [90.0, 0.0, 10.0, 10.0]),
        (root_child0_child1, [90.0, 0.0, 10.0, 100.0]),
        (root_child0_child2, [80.0, 0.0, 10.0, 100.0]),
    ]);
}

#[test]
fn flex_direction_col_reverse_inner_padding_top() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::ColumnReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(10.0);
        s.size.height = Length::Px(10.0);
        s.padding.top = Length::Px(10.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Px(10.0);
    });
    let root_child0_child2 = t.child(root_child0, "root_child0_child2", |s| {
        s.size.width = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0, [0.0, 90.0, 10.0, 10.0]),
        (root_child0_child1, [0.0, 100.0, 10.0, 0.0]),
        (root_child0_child2, [0.0, 100.0, 10.0, 0.0]),
    ]);
}

#[test]
fn flex_direction_col_reverse_inner_padding_bottom() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::ColumnReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(10.0);
        s.size.height = Length::Px(10.0);
        s.padding.bottom = Length::Px(10.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Px(10.0);
    });
    let root_child0_child2 = t.child(root_child0, "root_child0_child2", |s| {
        s.size.width = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0, [0.0, 90.0, 10.0, 10.0]),
        (root_child0_child1, [0.0, 100.0, 10.0, 0.0]),
        (root_child0_child2, [0.0, 100.0, 10.0, 0.0]),
    ]);
}

#[test]
fn flex_direction_row_reverse_inner_padding_start() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::RowReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(10.0);
        s.size.height = Length::Px(10.0);
        s.padding.left = Length::Px(10.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Px(10.0);
    });
    let root_child0_child2 = t.child(root_child0, "root_child0_child2", |s| {
        s.size.width = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0, [90.0, 0.0, 10.0, 10.0]),
        (root_child0_child1, [90.0, 0.0, 10.0, 100.0]),
        (root_child0_child2, [80.0, 0.0, 10.0, 100.0]),
    ]);
}

#[test]
fn flex_direction_row_reverse_inner_padding_end() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::RowReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(10.0);
        s.size.height = Length::Px(10.0);
        s.padding.right = Length::Px(10.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Px(10.0);
    });
    let root_child0_child2 = t.child(root_child0, "root_child0_child2", |s| {
        s.size.width = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0, [90.0, 0.0, 10.0, 10.0]),
        (root_child0_child1, [90.0, 0.0, 10.0, 100.0]),
        (root_child0_child2, [80.0, 0.0, 10.0, 100.0]),
    ]);
}

#[test]
fn flex_direction_alternating_with_percent() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(300.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Percent(50.0);
        s.size.height = Length::Percent(50.0);
        s.inset.left = Length::Percent(10.0);
        s.inset.top = Length::Percent(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 300.0]),
        (root_child0, [20.0, 30.0, 100.0, 150.0]),
    ]);
}
