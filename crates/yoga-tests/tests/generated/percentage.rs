// @generated — do not edit. Written by the Yoga importer (crates/yoga-tests/src/bin/import.rs):
//     cargo run -p souple-yoga-tests --bin import -- <path to yoga>
// From Yoga 3.2.1, tests/generated/YGPercentageTest.cpp (MIT, Copyright (c) Meta Platforms, Inc. and affiliates).

use crate::support::*;

#[test]
fn percentage_width_height() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Percent(30.0);
        s.size.height = Length::Percent(30.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 60.0, 60.0]),
    ]);
}

#[test]
fn percentage_position_left_top() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(400.0);
        s.size.height = Length::Px(400.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Percent(45.0);
        s.size.height = Length::Percent(55.0);
        s.inset.left = Length::Percent(10.0);
        s.inset.top = Length::Percent(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 400.0, 400.0]),
        (root_child0, [40.0, 80.0, 180.0, 220.0]),
    ]);
}

#[test]
fn percentage_position_bottom_right() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(500.0);
        s.size.height = Length::Px(500.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Percent(55.0);
        s.size.height = Length::Percent(15.0);
        s.inset.right = Length::Percent(20.0);
        s.inset.bottom = Length::Percent(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 500.0, 500.0]),
        (root_child0, [-100.0, -50.0, 275.0, 75.0]),
    ]);
}

#[test]
fn percentage_flex_basis() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_basis = Length::Percent(50.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_grow = 1.0;
        s.flex_basis = Length::Percent(25.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 125.0, 200.0]),
        (root_child1, [125.0, 0.0, 75.0, 200.0]),
    ]);
}

#[test]
fn percentage_flex_basis_cross() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_basis = Length::Percent(50.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_grow = 1.0;
        s.flex_basis = Length::Percent(25.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 200.0, 125.0]),
        (root_child1, [0.0, 125.0, 200.0, 75.0]),
    ]);
}

#[test]
#[ignore = "skipped by Yoga: GTEST_SKIP, Yoga fails it too"]
fn percentage_flex_basis_cross_min_height() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
        s.min_size.height = Length::Percent(60.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_grow = 2.0;
        s.min_size.height = Length::Percent(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 200.0, 120.0]),
        (root_child1, [0.0, 120.0, 200.0, 80.0]),
    ]);
}

#[test]
fn percentage_flex_basis_main_max_height() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_basis = Length::Percent(10.0);
        s.max_size.height = Length::Percent(60.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_grow = 4.0;
        s.flex_basis = Length::Percent(10.0);
        s.max_size.height = Length::Percent(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 52.0, 120.0]),
        (root_child1, [52.0, 0.0, 148.0, 40.0]),
    ]);
}

#[test]
fn percentage_flex_basis_cross_max_height() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_basis = Length::Percent(10.0);
        s.max_size.height = Length::Percent(60.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_grow = 4.0;
        s.flex_basis = Length::Percent(10.0);
        s.max_size.height = Length::Percent(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 200.0, 120.0]),
        (root_child1, [0.0, 120.0, 200.0, 40.0]),
    ]);
}

#[test]
fn percentage_flex_basis_main_max_width() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_basis = Length::Percent(15.0);
        s.max_size.width = Length::Percent(60.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_grow = 4.0;
        s.flex_basis = Length::Percent(10.0);
        s.max_size.width = Length::Percent(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 120.0, 200.0]),
        (root_child1, [120.0, 0.0, 40.0, 200.0]),
    ]);
}

#[test]
fn percentage_flex_basis_cross_max_width() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_basis = Length::Percent(10.0);
        s.max_size.width = Length::Percent(60.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_grow = 4.0;
        s.flex_basis = Length::Percent(15.0);
        s.max_size.width = Length::Percent(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 120.0, 50.0]),
        (root_child1, [0.0, 50.0, 40.0, 150.0]),
    ]);
}

#[test]
fn percentage_flex_basis_main_min_width() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_basis = Length::Percent(15.0);
        s.min_size.width = Length::Percent(60.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_grow = 4.0;
        s.flex_basis = Length::Percent(10.0);
        s.min_size.width = Length::Percent(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 120.0, 200.0]),
        (root_child1, [120.0, 0.0, 80.0, 200.0]),
    ]);
}

#[test]
fn percentage_flex_basis_cross_min_width() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_basis = Length::Percent(10.0);
        s.min_size.width = Length::Percent(60.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_grow = 4.0;
        s.flex_basis = Length::Percent(15.0);
        s.min_size.width = Length::Percent(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 200.0, 50.0]),
        (root_child1, [0.0, 50.0, 200.0, 150.0]),
    ]);
}

#[test]
fn percentage_multiple_nested_with_padding_margin_and_percentage_values() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_basis = Length::Percent(10.0);
        s.min_size.width = Length::Percent(60.0);
        s.margin.left = Length::Px(5.0);
        s.margin.top = Length::Px(5.0);
        s.margin.right = Length::Px(5.0);
        s.margin.bottom = Length::Px(5.0);
        s.padding.left = Length::Px(3.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(3.0);
        s.padding.bottom = Length::Px(3.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.size.width = Length::Percent(50.0);
        s.margin.left = Length::Px(5.0);
        s.margin.top = Length::Px(5.0);
        s.margin.right = Length::Px(5.0);
        s.margin.bottom = Length::Px(5.0);
        s.padding.left = Length::Percent(3.0);
        s.padding.top = Length::Percent(3.0);
        s.padding.right = Length::Percent(3.0);
        s.padding.bottom = Length::Percent(3.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.size.width = Length::Percent(45.0);
        s.margin.left = Length::Percent(5.0);
        s.margin.top = Length::Percent(5.0);
        s.margin.right = Length::Percent(5.0);
        s.margin.bottom = Length::Percent(5.0);
        s.padding.left = Length::Px(3.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(3.0);
        s.padding.bottom = Length::Px(3.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_grow = 4.0;
        s.flex_basis = Length::Percent(15.0);
        s.min_size.width = Length::Percent(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [5.0, 5.0, 190.0, 48.0]),
        (root_child0_child0, [8.0, 8.0, 92.0, 25.0]),
        (root_child0_child0_child0, [10.0, 10.0, 36.0, 6.0]),
        (root_child1, [0.0, 58.0, 200.0, 142.0]),
    ]);
}

#[test]
fn percentage_margin_should_calculate_based_only_on_width() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
        s.margin.left = Length::Percent(10.0);
        s.margin.top = Length::Percent(10.0);
        s.margin.right = Length::Percent(10.0);
        s.margin.bottom = Length::Percent(10.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.size.width = Length::Px(10.0);
        s.size.height = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 100.0]),
        (root_child0, [20.0, 20.0, 160.0, 60.0]),
        (root_child0_child0, [0.0, 0.0, 10.0, 10.0]),
    ]);
}

#[test]
fn percentage_padding_should_calculate_based_only_on_width() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
        s.padding.left = Length::Percent(10.0);
        s.padding.top = Length::Percent(10.0);
        s.padding.right = Length::Percent(10.0);
        s.padding.bottom = Length::Percent(10.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.size.width = Length::Px(10.0);
        s.size.height = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 100.0]),
        (root_child0, [0.0, 0.0, 200.0, 100.0]),
        (root_child0_child0, [20.0, 20.0, 10.0, 10.0]),
    ]);
}

#[test]
fn percentage_absolute_position() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(10.0);
        s.size.height = Length::Px(10.0);
        s.inset.left = Length::Percent(30.0);
        s.inset.top = Length::Percent(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 100.0]),
        (root_child0, [60.0, 10.0, 10.0, 10.0]),
    ]);
}

#[test]
fn percentage_width_height_undefined_parent_size() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Percent(50.0);
        s.size.height = Length::Percent(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 0.0, 0.0]),
        (root_child0, [0.0, 0.0, 0.0, 0.0]),
    ]);
}

#[test]
fn percent_within_flex_grow() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(350.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(100.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_grow = 1.0;
    });
    let root_child1_child0 = t.child(root_child1, "root_child1_child0", |s| {
        s.size.width = Length::Percent(100.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Px(100.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 350.0, 100.0]),
        (root_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child1, [100.0, 0.0, 150.0, 100.0]),
        (root_child1_child0, [0.0, 0.0, 150.0, 0.0]),
        (root_child2, [250.0, 0.0, 100.0, 100.0]),
    ]);
}

#[test]
fn percentage_container_in_wrapping_container() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.justify_content = JustifyContent::Center;
        s.align_items = AlignItems::Center;
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0 = t.child(root, "root_child0", |_| {});
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.flex_direction = FlexDirection::Row;
        s.justify_content = JustifyContent::Center;
        s.size.width = Length::Percent(100.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    let root_child0_child0_child1 = t.child(root_child0_child0, "root_child0_child0_child1", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [50.0, 75.0, 100.0, 50.0]),
        (root_child0_child0, [0.0, 0.0, 100.0, 50.0]),
        (root_child0_child0_child0, [0.0, 0.0, 50.0, 50.0]),
        (root_child0_child0_child1, [50.0, 0.0, 50.0, 50.0]),
    ]);
}

#[test]
fn percent_absolute_position() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(60.0);
        s.size.height = Length::Px(50.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.position = Position::Absolute;
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Percent(100.0);
        s.size.height = Length::Px(50.0);
        s.inset.left = Length::Percent(50.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.size.width = Length::Percent(100.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Percent(100.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 60.0, 50.0]),
        (root_child0, [30.0, 0.0, 60.0, 50.0]),
        (root_child0_child0, [0.0, 0.0, 60.0, 50.0]),
        (root_child0_child1, [60.0, 0.0, 60.0, 50.0]),
    ]);
}

#[test]
fn percent_of_minmax_main() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.height = Length::Px(50.0);
        s.min_size.width = Length::Px(60.0);
        s.max_size.width = Length::Px(60.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Percent(50.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 60.0, 50.0]),
        (root_child0, [0.0, 0.0, 30.0, 20.0]),
    ]);
}

#[test]
#[ignore = "skipped by Yoga: GTEST_SKIP, Yoga fails it too"]
fn percent_of_min_main() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.height = Length::Px(50.0);
        s.min_size.width = Length::Px(60.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Percent(50.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 60.0, 50.0]),
        (root_child0, [0.0, 0.0, 30.0, 20.0]),
    ]);
}

#[test]
#[ignore = "skipped by Yoga: GTEST_SKIP, Yoga fails it too"]
fn percent_of_min_main_multiple() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.height = Length::Px(50.0);
        s.min_size.width = Length::Px(60.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Percent(50.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Percent(50.0);
        s.size.height = Length::Px(20.0);
    });
    let root_child2 = t.child(root, "root_child2", |s| {
        s.size.width = Length::Percent(50.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 60.0, 50.0]),
        (root_child0, [0.0, 0.0, 30.0, 20.0]),
        (root_child1, [30.0, 0.0, 30.0, 20.0]),
        (root_child2, [60.0, 0.0, 30.0, 20.0]),
    ]);
}

#[test]
#[ignore = "skipped by Yoga: GTEST_SKIP, Yoga fails it too"]
fn percent_of_max_main() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.height = Length::Px(50.0);
        s.max_size.width = Length::Px(60.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Percent(50.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 0.0, 50.0]),
        (root_child0, [0.0, 0.0, 0.0, 20.0]),
    ]);
}

#[test]
fn percent_of_minmax_cross_stretched() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.height = Length::Px(50.0);
        s.min_size.width = Length::Px(60.0);
        s.max_size.width = Length::Px(60.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Percent(50.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 60.0, 50.0]),
        (root_child0, [0.0, 0.0, 30.0, 20.0]),
    ]);
}

#[test]
fn percent_absolute_of_minmax_cross_stretched() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.height = Length::Px(50.0);
        s.min_size.width = Length::Px(60.0);
        s.max_size.width = Length::Px(60.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Percent(50.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 60.0, 50.0]),
        (root_child0, [0.0, 0.0, 30.0, 20.0]),
    ]);
}

#[test]
fn percent_of_minmax_cross_unstretched() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.align_items = AlignItems::FlexStart;
        s.size.height = Length::Px(50.0);
        s.min_size.width = Length::Px(60.0);
        s.max_size.width = Length::Px(60.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Percent(50.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 60.0, 50.0]),
        (root_child0, [0.0, 0.0, 30.0, 20.0]),
    ]);
}

#[test]
#[ignore = "skipped by Yoga: GTEST_SKIP, Yoga fails it too"]
fn percent_of_min_cross_unstretched() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.align_items = AlignItems::FlexStart;
        s.size.height = Length::Px(50.0);
        s.min_size.width = Length::Px(60.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Percent(50.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 60.0, 50.0]),
        (root_child0, [0.0, 0.0, 30.0, 20.0]),
    ]);
}

#[test]
fn percent_of_max_cross_unstretched() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.align_items = AlignItems::FlexStart;
        s.size.height = Length::Px(50.0);
        s.max_size.width = Length::Px(60.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Percent(50.0);
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 0.0, 50.0]),
        (root_child0, [0.0, 0.0, 0.0, 20.0]),
    ]);
}
