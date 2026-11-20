// @generated — do not edit. Written by the Yoga importer (crates/yoga-tests/src/bin/import.rs):
//     cargo run -p souple-yoga-tests --bin import -- <path to yoga>
// From Yoga 3.2.1, tests/generated/YGAbsolutePositionTest.cpp (MIT, Copyright (c) Meta Platforms, Inc. and affiliates).

use crate::support::*;

#[test]
fn absolute_layout_width_height_start_top() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(10.0);
        s.size.height = Length::Px(10.0);
        s.inset.left = Length::Px(10.0);
        s.inset.top = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [10.0, 10.0, 10.0, 10.0]),
    ]);
}

#[test]
fn absolute_layout_width_height_left_auto_right() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(10.0);
        s.size.height = Length::Px(10.0);
        s.inset.left = Length::Auto;
        s.inset.right = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [80.0, 0.0, 10.0, 10.0]),
    ]);
}

#[test]
fn absolute_layout_width_height_left_right_auto() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(10.0);
        s.size.height = Length::Px(10.0);
        s.inset.left = Length::Px(10.0);
        s.inset.right = Length::Auto;
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [10.0, 0.0, 10.0, 10.0]),
    ]);
}

#[test]
fn absolute_layout_width_height_left_auto_right_auto() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(10.0);
        s.size.height = Length::Px(10.0);
        s.inset.left = Length::Auto;
        s.inset.right = Length::Auto;
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 10.0, 10.0]),
    ]);
}

#[test]
fn absolute_layout_width_height_end_bottom() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(10.0);
        s.size.height = Length::Px(10.0);
        s.inset.right = Length::Px(10.0);
        s.inset.bottom = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [80.0, 80.0, 10.0, 10.0]),
    ]);
}

#[test]
fn absolute_layout_start_top_end_bottom() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.position = Position::Absolute;
        s.inset.left = Length::Px(10.0);
        s.inset.top = Length::Px(10.0);
        s.inset.right = Length::Px(10.0);
        s.inset.bottom = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [10.0, 10.0, 80.0, 80.0]),
    ]);
}

#[test]
fn absolute_layout_width_height_start_top_end_bottom() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(10.0);
        s.size.height = Length::Px(10.0);
        s.inset.left = Length::Px(10.0);
        s.inset.top = Length::Px(10.0);
        s.inset.right = Length::Px(10.0);
        s.inset.bottom = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [10.0, 10.0, 10.0, 10.0]),
    ]);
}

#[test]
fn do_not_clamp_height_of_absolute_node_to_height_of_its_overflow_hidden_parent() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.overflow = Overflow::Hidden;
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.position = Position::Absolute;
        s.inset.left = Length::Px(0.0);
        s.inset.top = Length::Px(0.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 50.0, 50.0]),
        (root_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0, [0.0, 0.0, 100.0, 100.0]),
    ]);
}

#[test]
fn absolute_layout_within_border() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.margin.left = Length::Px(10.0);
        s.margin.top = Length::Px(10.0);
        s.margin.right = Length::Px(10.0);
        s.margin.bottom = Length::Px(10.0);
        s.padding.left = Length::Px(10.0);
        s.padding.top = Length::Px(10.0);
        s.padding.right = Length::Px(10.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(10.0);
        s.border.top = Length::Px(10.0);
        s.border.right = Length::Px(10.0);
        s.border.bottom = Length::Px(10.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.inset.left = Length::Px(0.0);
        s.inset.top = Length::Px(0.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.inset.right = Length::Px(0.0);
        s.inset.bottom = Length::Px(0.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.margin.left = Length::Px(10.0);
        s.margin.top = Length::Px(10.0);
        s.margin.right = Length::Px(10.0);
        s.margin.bottom = Length::Px(10.0);
        s.inset.left = Length::Px(0.0);
        s.inset.top = Length::Px(0.0);
    });
    let root_child3 = t.child(root, "root_child3", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.margin.left = Length::Px(10.0);
        s.margin.top = Length::Px(10.0);
        s.margin.right = Length::Px(10.0);
        s.margin.bottom = Length::Px(10.0);
        s.inset.right = Length::Px(0.0);
        s.inset.bottom = Length::Px(0.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [10.0, 10.0, 100.0, 100.0]),
        (root_child0, [10.0, 10.0, 50.0, 50.0]),
        (root_child1, [40.0, 40.0, 50.0, 50.0]),
        (root_child2, [20.0, 20.0, 50.0, 50.0]),
        (root_child3, [30.0, 30.0, 50.0, 50.0]),
    ]);
}

#[test]
fn absolute_layout_align_items_and_justify_content_center() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.justify_content = JustifyContent::Center;
        s.align_items = AlignItems::Center;
        s.flex_grow = 1.0;
        s.size.width = Length::Px(110.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(60.0);
        s.size.height = Length::Px(40.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 110.0, 100.0]),
        (root_child0, [25.0, 30.0, 60.0, 40.0]),
    ]);
}

#[test]
fn absolute_layout_align_items_and_justify_content_flex_end() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.justify_content = JustifyContent::FlexEnd;
        s.align_items = AlignItems::FlexEnd;
        s.flex_grow = 1.0;
        s.size.width = Length::Px(110.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(60.0);
        s.size.height = Length::Px(40.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 110.0, 100.0]),
        (root_child0, [50.0, 60.0, 60.0, 40.0]),
    ]);
}

#[test]
fn absolute_layout_justify_content_center() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.justify_content = JustifyContent::Center;
        s.flex_grow = 1.0;
        s.size.width = Length::Px(110.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(60.0);
        s.size.height = Length::Px(40.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 110.0, 100.0]),
        (root_child0, [0.0, 30.0, 60.0, 40.0]),
    ]);
}

#[test]
fn absolute_layout_align_items_center() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.align_items = AlignItems::Center;
        s.flex_grow = 1.0;
        s.size.width = Length::Px(110.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(60.0);
        s.size.height = Length::Px(40.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 110.0, 100.0]),
        (root_child0, [25.0, 0.0, 60.0, 40.0]),
    ]);
}

#[test]
fn absolute_layout_align_items_center_on_child_only() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_grow = 1.0;
        s.size.width = Length::Px(110.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.position = Position::Absolute;
        s.align_self = Some(AlignItems::Center);
        s.size.width = Length::Px(60.0);
        s.size.height = Length::Px(40.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 110.0, 100.0]),
        (root_child0, [25.0, 0.0, 60.0, 40.0]),
    ]);
}

#[test]
fn absolute_layout_align_items_and_justify_content_center_and_top_position() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.justify_content = JustifyContent::Center;
        s.align_items = AlignItems::Center;
        s.flex_grow = 1.0;
        s.size.width = Length::Px(110.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(60.0);
        s.size.height = Length::Px(40.0);
        s.inset.top = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 110.0, 100.0]),
        (root_child0, [25.0, 10.0, 60.0, 40.0]),
    ]);
}

#[test]
fn absolute_layout_align_items_and_justify_content_center_and_bottom_position() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.justify_content = JustifyContent::Center;
        s.align_items = AlignItems::Center;
        s.flex_grow = 1.0;
        s.size.width = Length::Px(110.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(60.0);
        s.size.height = Length::Px(40.0);
        s.inset.bottom = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 110.0, 100.0]),
        (root_child0, [25.0, 50.0, 60.0, 40.0]),
    ]);
}

#[test]
fn absolute_layout_align_items_and_justify_content_center_and_left_position() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.justify_content = JustifyContent::Center;
        s.align_items = AlignItems::Center;
        s.flex_grow = 1.0;
        s.size.width = Length::Px(110.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(60.0);
        s.size.height = Length::Px(40.0);
        s.inset.left = Length::Px(5.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 110.0, 100.0]),
        (root_child0, [5.0, 30.0, 60.0, 40.0]),
    ]);
}

#[test]
fn absolute_layout_align_items_and_justify_content_center_and_right_position() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.justify_content = JustifyContent::Center;
        s.align_items = AlignItems::Center;
        s.flex_grow = 1.0;
        s.size.width = Length::Px(110.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(60.0);
        s.size.height = Length::Px(40.0);
        s.inset.right = Length::Px(5.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 110.0, 100.0]),
        (root_child0, [45.0, 30.0, 60.0, 40.0]),
    ]);
}

#[test]
fn position_root_with_rtl_should_position_withoutdirection() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(52.0);
        s.size.height = Length::Px(52.0);
        s.inset.left = Length::Px(72.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [72.0, 0.0, 52.0, 52.0]),
    ]);
}

#[test]
fn absolute_layout_percentage_bottom_based_on_parent_height() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(10.0);
        s.size.height = Length::Px(10.0);
        s.inset.top = Length::Percent(50.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(10.0);
        s.size.height = Length::Px(10.0);
        s.inset.bottom = Length::Percent(50.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(10.0);
        s.inset.top = Length::Percent(10.0);
        s.inset.bottom = Length::Percent(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 200.0]),
        (root_child0, [0.0, 100.0, 10.0, 10.0]),
        (root_child1, [0.0, 90.0, 10.0, 10.0]),
        (root_child2, [0.0, 20.0, 10.0, 160.0]),
    ]);
}

#[test]
fn absolute_layout_in_wrap_reverse_column_container() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_wrap = FlexWrap::WrapReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [80.0, 0.0, 20.0, 20.0]),
    ]);
}

#[test]
fn absolute_layout_in_wrap_reverse_row_container() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::WrapReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 80.0, 20.0, 20.0]),
    ]);
}

#[test]
fn absolute_layout_in_wrap_reverse_column_container_flex_end() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_wrap = FlexWrap::WrapReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.position = Position::Absolute;
        s.align_self = Some(AlignItems::FlexEnd);
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 20.0, 20.0]),
    ]);
}

#[test]
fn absolute_layout_in_wrap_reverse_row_container_flex_end() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.flex_wrap = FlexWrap::WrapReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.position = Position::Absolute;
        s.align_self = Some(AlignItems::FlexEnd);
        s.size.width = Length::Px(20.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 20.0, 20.0]),
    ]);
}

#[test]
fn percent_absolute_position_infinite_height() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(300.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(300.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Percent(20.0);
        s.size.height = Length::Percent(20.0);
        s.inset.left = Length::Percent(20.0);
        s.inset.top = Length::Percent(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 300.0, 0.0]),
        (root_child0, [0.0, 0.0, 300.0, 0.0]),
        (root_child1, [60.0, 0.0, 60.0, 0.0]),
    ]);
}

#[test]
fn absolute_layout_percentage_height_based_on_padded_parent() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.padding.top = Length::Px(10.0);
        s.border.top = Length::Px(10.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Percent(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 20.0, 100.0, 45.0]),
    ]);
}

#[test]
fn absolute_layout_percentage_height_based_on_padded_parent_and_align_items_center() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.justify_content = JustifyContent::Center;
        s.align_items = AlignItems::Center;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.padding.top = Length::Px(20.0);
        s.padding.bottom = Length::Px(20.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Percent(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 25.0, 100.0, 50.0]),
    ]);
}

#[test]
fn absolute_layout_padding_left() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
        s.padding.left = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [100.0, 0.0, 50.0, 50.0]),
    ]);
}

#[test]
fn absolute_layout_padding_right() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
        s.padding.right = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 50.0, 50.0]),
    ]);
}

#[test]
fn absolute_layout_padding_top() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
        s.padding.top = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 100.0, 50.0, 50.0]),
    ]);
}

#[test]
fn absolute_layout_padding_bottom() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
        s.padding.bottom = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 50.0, 50.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn absolute_layout_padding() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
        s.margin.left = Length::Px(10.0);
        s.margin.top = Length::Px(10.0);
        s.margin.right = Length::Px(10.0);
        s.margin.bottom = Length::Px(10.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
        s.padding.left = Length::Px(50.0);
        s.padding.top = Length::Px(50.0);
        s.padding.right = Length::Px(50.0);
        s.padding.bottom = Length::Px(50.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 220.0, 220.0]),
        (root_child0, [10.0, 10.0, 200.0, 200.0]),
        (root_child0_child0, [0.0, 0.0, 200.0, 200.0]),
        (root_child0_child0_child0, [50.0, 50.0, 50.0, 50.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn absolute_layout_border() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
        s.margin.left = Length::Px(10.0);
        s.margin.top = Length::Px(10.0);
        s.margin.right = Length::Px(10.0);
        s.margin.bottom = Length::Px(10.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
        s.border.left = Length::Px(10.0);
        s.border.top = Length::Px(10.0);
        s.border.right = Length::Px(10.0);
        s.border.bottom = Length::Px(10.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 220.0, 220.0]),
        (root_child0, [10.0, 10.0, 200.0, 200.0]),
        (root_child0_child0, [0.0, 0.0, 200.0, 200.0]),
        (root_child0_child0_child0, [10.0, 10.0, 50.0, 50.0]),
    ]);
}

#[test]
fn absolute_layout_column_reverse_margin_border() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::ColumnReverse;
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.margin.left = Length::Px(3.0);
        s.margin.right = Length::Px(4.0);
        s.border.left = Length::Px(1.0);
        s.border.right = Length::Px(7.0);
        s.inset.left = Length::Px(5.0);
        s.inset.right = Length::Px(3.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [8.0, 150.0, 50.0, 50.0]),
    ]);
}
