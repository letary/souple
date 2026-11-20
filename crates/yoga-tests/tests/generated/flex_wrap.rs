// @generated — do not edit. Written by the Yoga importer (crates/yoga-tests/src/bin/import.rs):
//     cargo run -p souple-yoga-tests --bin import -- <path to yoga>
// From Yoga 3.2.1, tests/generated/YGFlexWrapTest.cpp (MIT, Copyright (c) Meta Platforms, Inc. and affiliates).

use crate::support::*;

#[test]
fn wrap_column() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_wrap = FlexWrap::Wrap;
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(30.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(30.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(30.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(30.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 60.0, 100.0]),
        (root_child0, [0.0, 0.0, 30.0, 30.0]),
        (root_child1, [0.0, 30.0, 30.0, 30.0]),
        (root_child2, [0.0, 60.0, 30.0, 30.0]),
        (root_child3, [30.0, 0.0, 30.0, 30.0]),
    ]);
}

#[test]
fn wrap_row() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.size.width = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(30.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(30.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(30.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(30.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 60.0]),
        (root_child0, [0.0, 0.0, 30.0, 30.0]),
        (root_child1, [30.0, 0.0, 30.0, 30.0]),
        (root_child2, [60.0, 0.0, 30.0, 30.0]),
        (root_child3, [0.0, 30.0, 30.0, 30.0]),
    ]);
}

#[test]
fn wrap_row_align_items_flex_end() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.align_items = AlignItems::FlexEnd;
        s.size.width = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(30.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(30.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 60.0]),
        (root_child0, [0.0, 20.0, 30.0, 10.0]),
        (root_child1, [30.0, 10.0, 30.0, 20.0]),
        (root_child2, [60.0, 0.0, 30.0, 30.0]),
        (root_child3, [0.0, 30.0, 30.0, 30.0]),
    ]);
}

#[test]
fn wrap_row_align_items_center() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.align_items = AlignItems::Center;
        s.size.width = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(30.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(30.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 60.0]),
        (root_child0, [0.0, 10.0, 30.0, 10.0]),
        (root_child1, [30.0, 5.0, 30.0, 20.0]),
        (root_child2, [60.0, 0.0, 30.0, 30.0]),
        (root_child3, [0.0, 30.0, 30.0, 30.0]),
    ]);
}

#[test]
fn flex_wrap_children_with_min_main_overriding_flex_basis() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.size.width = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_basis = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.min_size.width = Length::Px(55.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_basis = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.min_size.width = Length::Px(55.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 55.0, 50.0]),
        (root_child1, [0.0, 50.0, 55.0, 50.0]),
    ]);
}

#[test]
fn flex_wrap_wrap_to_child_height() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.align_items = AlignItems::FlexStart;
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.size.width = Length::Px(100.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 200.0]),
        (root_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child1, [0.0, 100.0, 100.0, 100.0]),
    ]);
}

#[test]
fn flex_wrap_align_stretch_fits_one_row() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.size.width = Length::Px(150.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(50.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 150.0, 100.0]),
        (root_child0, [0.0, 0.0, 50.0, 0.0]),
        (root_child1, [50.0, 0.0, 50.0, 0.0]),
    ]);
}

#[test]
fn wrap_reverse_row_align_content_flex_start() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::WrapReverse;
        s.size.width = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(30.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(40.0);
    });
    let root_child4 = t.child(root, "root_child4", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 80.0]),
        (root_child0, [0.0, 70.0, 30.0, 10.0]),
        (root_child1, [30.0, 60.0, 30.0, 20.0]),
        (root_child2, [60.0, 50.0, 30.0, 30.0]),
        (root_child3, [0.0, 10.0, 30.0, 40.0]),
        (root_child4, [30.0, 0.0, 30.0, 50.0]),
    ]);
}

#[test]
fn wrap_reverse_row_align_content_center() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::WrapReverse;
        s.align_content = AlignContent::Center;
        s.size.width = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(30.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(40.0);
    });
    let root_child4 = t.child(root, "root_child4", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 80.0]),
        (root_child0, [0.0, 70.0, 30.0, 10.0]),
        (root_child1, [30.0, 60.0, 30.0, 20.0]),
        (root_child2, [60.0, 50.0, 30.0, 30.0]),
        (root_child3, [0.0, 10.0, 30.0, 40.0]),
        (root_child4, [30.0, 0.0, 30.0, 50.0]),
    ]);
}

#[test]
fn wrap_reverse_row_single_line_different_size() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::WrapReverse;
        s.size.width = Length::Px(300.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(30.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(40.0);
    });
    let root_child4 = t.child(root, "root_child4", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 300.0, 50.0]),
        (root_child0, [0.0, 40.0, 30.0, 10.0]),
        (root_child1, [30.0, 30.0, 30.0, 20.0]),
        (root_child2, [60.0, 20.0, 30.0, 30.0]),
        (root_child3, [90.0, 10.0, 30.0, 40.0]),
        (root_child4, [120.0, 0.0, 30.0, 50.0]),
    ]);
}

#[test]
fn wrap_reverse_row_align_content_stretch() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::WrapReverse;
        s.align_content = AlignContent::Stretch;
        s.size.width = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(30.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(40.0);
    });
    let root_child4 = t.child(root, "root_child4", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 80.0]),
        (root_child0, [0.0, 70.0, 30.0, 10.0]),
        (root_child1, [30.0, 60.0, 30.0, 20.0]),
        (root_child2, [60.0, 50.0, 30.0, 30.0]),
        (root_child3, [0.0, 10.0, 30.0, 40.0]),
        (root_child4, [30.0, 0.0, 30.0, 50.0]),
    ]);
}

#[test]
fn wrap_reverse_row_align_content_space_around() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::WrapReverse;
        s.align_content = AlignContent::SpaceAround;
        s.size.width = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(30.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(40.0);
    });
    let root_child4 = t.child(root, "root_child4", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 80.0]),
        (root_child0, [0.0, 70.0, 30.0, 10.0]),
        (root_child1, [30.0, 60.0, 30.0, 20.0]),
        (root_child2, [60.0, 50.0, 30.0, 30.0]),
        (root_child3, [0.0, 10.0, 30.0, 40.0]),
        (root_child4, [30.0, 0.0, 30.0, 50.0]),
    ]);
}

#[test]
fn wrap_reverse_column_fixed_size() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_wrap = FlexWrap::WrapReverse;
        s.align_items = AlignItems::Center;
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(10.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(30.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(40.0);
    });
    let root_child4 = t.child(root, "root_child4", |s| {
        s.size.width = Length::Px(30.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 100.0]),
        (root_child0, [170.0, 0.0, 30.0, 10.0]),
        (root_child1, [170.0, 10.0, 30.0, 20.0]),
        (root_child2, [170.0, 30.0, 30.0, 30.0]),
        (root_child3, [170.0, 60.0, 30.0, 40.0]),
        (root_child4, [140.0, 0.0, 30.0, 50.0]),
    ]);
}

#[test]
fn wrapped_row_within_align_items_center() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.align_items = AlignItems::Center;
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.size.width = Length::Px(150.0);
        s.size.height = Length::Px(80.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Px(80.0);
        s.size.height = Length::Px(80.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 200.0, 160.0]),
        (root_child0_child0, [0.0, 0.0, 150.0, 80.0]),
        (root_child0_child1, [0.0, 80.0, 80.0, 80.0]),
    ]);
}

#[test]
fn wrapped_row_within_align_items_flex_start() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.align_items = AlignItems::FlexStart;
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.size.width = Length::Px(150.0);
        s.size.height = Length::Px(80.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Px(80.0);
        s.size.height = Length::Px(80.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 200.0, 160.0]),
        (root_child0_child0, [0.0, 0.0, 150.0, 80.0]),
        (root_child0_child1, [0.0, 80.0, 80.0, 80.0]),
    ]);
}

#[test]
fn wrapped_row_within_align_items_flex_end() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.align_items = AlignItems::FlexEnd;
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.size.width = Length::Px(150.0);
        s.size.height = Length::Px(80.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Px(80.0);
        s.size.height = Length::Px(80.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 200.0, 160.0]),
        (root_child0_child0, [0.0, 0.0, 150.0, 80.0]),
        (root_child0_child1, [0.0, 80.0, 80.0, 80.0]),
    ]);
}

#[test]
fn wrapped_column_max_height() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_wrap = FlexWrap::Wrap;
        s.justify_content = JustifyContent::Center;
        s.align_items = AlignItems::Center;
        s.align_content = AlignContent::Center;
        s.size.width = Length::Px(700.0);
        s.size.height = Length::Px(500.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(500.0);
        s.max_size.height = Length::Px(200.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
        s.margin.left = Length::Px(20.0);
        s.margin.top = Length::Px(20.0);
        s.margin.right = Length::Px(20.0);
        s.margin.bottom = Length::Px(20.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 700.0, 500.0]),
        (root_child0, [250.0, 30.0, 100.0, 200.0]),
        (root_child1, [200.0, 250.0, 200.0, 200.0]),
        (root_child2, [420.0, 200.0, 100.0, 100.0]),
    ]);
}

#[test]
fn wrapped_column_max_height_flex() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_wrap = FlexWrap::Wrap;
        s.justify_content = JustifyContent::Center;
        s.align_items = AlignItems::Center;
        s.align_content = AlignContent::Center;
        s.size.width = Length::Px(700.0);
        s.size.height = Length::Px(500.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(500.0);
        s.max_size.height = Length::Px(200.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
        s.margin.left = Length::Px(20.0);
        s.margin.top = Length::Px(20.0);
        s.margin.right = Length::Px(20.0);
        s.margin.bottom = Length::Px(20.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 700.0, 500.0]),
        (root_child0, [300.0, 0.0, 100.0, 180.0]),
        (root_child1, [250.0, 200.0, 200.0, 180.0]),
        (root_child2, [300.0, 400.0, 100.0, 100.0]),
    ]);
}

#[test]
fn wrap_nodes_with_content_sizing_overflowing_margin() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(500.0);
        s.size.height = Length::Px(500.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.size.width = Length::Px(85.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |_| {});
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.size.width = Length::Px(40.0);
        s.size.height = Length::Px(40.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.margin.right = Length::Px(10.0);
    });
    let root_child0_child1_child0 = t.child(root_child0_child1, "root_child0_child1_child0", |s| {
        s.size.width = Length::Px(40.0);
        s.size.height = Length::Px(40.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 500.0, 500.0]),
        (root_child0, [0.0, 0.0, 85.0, 80.0]),
        (root_child0_child0, [0.0, 0.0, 40.0, 40.0]),
        (root_child0_child0_child0, [0.0, 0.0, 40.0, 40.0]),
        (root_child0_child1, [0.0, 40.0, 40.0, 40.0]),
        (root_child0_child1_child0, [0.0, 0.0, 40.0, 40.0]),
    ]);
}

#[test]
fn wrap_nodes_with_content_sizing_margin_cross() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(500.0);
        s.size.height = Length::Px(500.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.size.width = Length::Px(70.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |_| {});
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.size.width = Length::Px(40.0);
        s.size.height = Length::Px(40.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.margin.top = Length::Px(10.0);
    });
    let root_child0_child1_child0 = t.child(root_child0_child1, "root_child0_child1_child0", |s| {
        s.size.width = Length::Px(40.0);
        s.size.height = Length::Px(40.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 500.0, 500.0]),
        (root_child0, [0.0, 0.0, 70.0, 90.0]),
        (root_child0_child0, [0.0, 0.0, 40.0, 40.0]),
        (root_child0_child0_child0, [0.0, 0.0, 40.0, 40.0]),
        (root_child0_child1, [0.0, 50.0, 40.0, 40.0]),
        (root_child0_child1_child0, [0.0, 0.0, 40.0, 40.0]),
    ]);
}

#[test]
fn wrap_with_min_cross_axis() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.size.width = Length::Px(500.0);
        s.min_size.height = Length::Px(500.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(400.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(400.0);
        s.size.height = Length::Px(200.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 500.0, 500.0]),
        (root_child0, [0.0, 0.0, 400.0, 200.0]),
        (root_child1, [0.0, 200.0, 400.0, 200.0]),
    ]);
}

#[test]
fn wrap_with_max_cross_axis() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.size.width = Length::Px(500.0);
        s.max_size.height = Length::Px(500.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(400.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(400.0);
        s.size.height = Length::Px(200.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 500.0, 400.0]),
        (root_child0, [0.0, 0.0, 400.0, 200.0]),
        (root_child1, [0.0, 200.0, 400.0, 200.0]),
    ]);
}

#[test]
fn nowrap_expands_flexline_box_to_min_cross() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.min_size.height = Length::Px(400.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 0.0, 400.0]),
        (root_child0, [0.0, 0.0, 0.0, 400.0]),
    ]);
}

#[test]
fn wrap_does_not_impose_min_cross_onto_single_flexline() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::Wrap;
        s.min_size.height = Length::Px(400.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Percent(0.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 0.0, 400.0]),
        (root_child0, [0.0, 0.0, 0.0, 0.0]),
    ]);
}
