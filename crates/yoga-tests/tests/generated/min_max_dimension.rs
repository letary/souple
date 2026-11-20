// @generated — do not edit. Written by the Yoga importer (crates/yoga-tests/src/bin/import.rs):
//     cargo run -p souple-yoga-tests --bin import -- <path to yoga>
// From Yoga 3.2.1, tests/generated/YGMinMaxDimensionTest.cpp (MIT, Copyright (c) Meta Platforms, Inc. and affiliates).

use crate::support::*;

#[test]
fn max_width() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.height = Length::Px(10.0);
        s.max_size.width = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 50.0, 10.0]),
    ]);
}

#[test]
fn max_height() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(10.0);
        s.max_size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 10.0, 50.0]),
    ]);
}

#[test]
#[ignore = "skipped by Yoga: GTEST_SKIP, Yoga fails it too"]
fn min_height() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
        s.min_size.height = Length::Px(60.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_grow = 1.0;
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 100.0, 60.0]),
        (root_child1, [0.0, 60.0, 100.0, 40.0]),
    ]);
}

#[test]
#[ignore = "skipped by Yoga: GTEST_SKIP, Yoga fails it too"]
fn min_width() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
        s.min_size.width = Length::Px(60.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_grow = 1.0;
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 60.0, 100.0]),
        (root_child1, [60.0, 0.0, 40.0, 100.0]),
    ]);
}

#[test]
fn justify_content_min_max() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.justify_content = JustifyContent::Center;
        s.size.width = Length::Px(100.0);
        s.min_size.height = Length::Px(100.0);
        s.max_size.height = Length::Px(200.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(60.0);
        s.size.height = Length::Px(60.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 20.0, 60.0, 60.0]),
    ]);
}

#[test]
fn align_items_min_max() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.align_items = AlignItems::Center;
        s.size.height = Length::Px(100.0);
        s.min_size.width = Length::Px(100.0);
        s.max_size.width = Length::Px(200.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(60.0);
        s.size.height = Length::Px(60.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [20.0, 0.0, 60.0, 60.0]),
    ]);
}

#[test]
fn justify_content_overflow_min_max() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.justify_content = JustifyContent::Center;
        s.min_size.height = Length::Px(100.0);
        s.max_size.height = Length::Px(110.0);
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
        (root, [0.0, 0.0, 50.0, 110.0]),
        (root_child0, [0.0, -20.0, 50.0, 50.0]),
        (root_child1, [0.0, 30.0, 50.0, 50.0]),
        (root_child2, [0.0, 80.0, 50.0, 50.0]),
    ]);
}

#[test]
fn flex_grow_to_min() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.min_size.height = Length::Px(100.0);
        s.max_size.height = Length::Px(500.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_shrink = 1.0;
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 100.0, 50.0]),
        (root_child1, [0.0, 50.0, 100.0, 50.0]),
    ]);
}

#[test]
fn flex_grow_in_at_most_container() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.align_items = AlignItems::FlexStart;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::Row;
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_basis = Length::Px(0.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 0.0, 0.0]),
        (root_child0_child0, [0.0, 0.0, 0.0, 0.0]),
    ]);
}

#[test]
fn flex_grow_child() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_basis = Length::Px(0.0);
        s.size.height = Length::Px(100.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 0.0, 100.0]),
        (root_child0, [0.0, 0.0, 0.0, 100.0]),
    ]);
}

#[test]
fn flex_grow_within_constrained_min_max_column() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.min_size.height = Length::Px(100.0);
        s.max_size.height = Length::Px(200.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 0.0, 100.0]),
        (root_child0, [0.0, 0.0, 0.0, 50.0]),
        (root_child1, [0.0, 50.0, 0.0, 50.0]),
    ]);
}

#[test]
fn flex_grow_within_max_width() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::Row;
        s.max_size.width = Length::Px(100.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.flex_grow = 1.0;
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 100.0]),
        (root_child0, [0.0, 0.0, 100.0, 20.0]),
        (root_child0_child0, [0.0, 0.0, 100.0, 20.0]),
    ]);
}

#[test]
fn flex_grow_within_constrained_max_width() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::Row;
        s.max_size.width = Length::Px(300.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.flex_grow = 1.0;
        s.size.height = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 100.0]),
        (root_child0, [0.0, 0.0, 200.0, 20.0]),
        (root_child0_child0, [0.0, 0.0, 200.0, 20.0]),
    ]);
}

#[test]
fn flex_root_ignored() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_grow = 1.0;
        s.size.width = Length::Px(100.0);
        s.min_size.height = Length::Px(100.0);
        s.max_size.height = Length::Px(500.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_basis = Length::Px(200.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.height = Length::Px(100.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 300.0]),
        (root_child0, [0.0, 0.0, 100.0, 200.0]),
        (root_child1, [0.0, 200.0, 100.0, 100.0]),
    ]);
}

#[test]
fn flex_grow_root_minimized() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.min_size.height = Length::Px(100.0);
        s.max_size.height = Length::Px(500.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
        s.min_size.height = Length::Px(100.0);
        s.max_size.height = Length::Px(500.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_basis = Length::Px(200.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.height = Length::Px(100.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 300.0]),
        (root_child0, [0.0, 0.0, 100.0, 300.0]),
        (root_child0_child0, [0.0, 0.0, 100.0, 200.0]),
        (root_child0_child1, [0.0, 200.0, 100.0, 100.0]),
    ]);
}

#[test]
fn flex_grow_height_maximized() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(500.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
        s.min_size.height = Length::Px(100.0);
        s.max_size.height = Length::Px(500.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_basis = Length::Px(200.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.height = Length::Px(100.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 500.0]),
        (root_child0, [0.0, 0.0, 100.0, 500.0]),
        (root_child0_child0, [0.0, 0.0, 100.0, 400.0]),
        (root_child0_child1, [0.0, 400.0, 100.0, 100.0]),
    ]);
}

#[test]
fn flex_grow_within_constrained_min_row() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.height = Length::Px(100.0);
        s.min_size.width = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.width = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 50.0, 100.0]),
        (root_child1, [50.0, 0.0, 50.0, 100.0]),
    ]);
}

#[test]
fn flex_grow_within_constrained_min_column() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.min_size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 0.0, 100.0]),
        (root_child0, [0.0, 0.0, 0.0, 50.0]),
        (root_child1, [0.0, 50.0, 0.0, 50.0]),
    ]);
}

#[test]
fn flex_grow_within_constrained_max_row() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(200.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.height = Length::Px(100.0);
        s.max_size.width = Length::Px(100.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Px(100.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        s.size.width = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 100.0]),
        (root_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0, [0.0, 0.0, 50.0, 100.0]),
        (root_child0_child1, [50.0, 0.0, 50.0, 100.0]),
    ]);
}

#[test]
fn flex_grow_within_constrained_max_column() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.max_size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_shrink = 1.0;
        s.flex_basis = Length::Px(100.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 100.0, 50.0]),
        (root_child1, [0.0, 50.0, 100.0, 50.0]),
    ]);
}

#[test]
fn child_min_max_width_flexing() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(120.0);
        s.size.height = Length::Px(50.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
        s.flex_basis = Length::Px(0.0);
        s.min_size.width = Length::Px(60.0);
    });
    let root_child1 = t.child(root, "root_child1", |s| {
        s.flex_grow = 1.0;
        s.flex_basis = Length::Percent(50.0);
        s.max_size.width = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 120.0, 50.0]),
        (root_child0, [0.0, 0.0, 100.0, 50.0]),
        (root_child1, [100.0, 0.0, 20.0, 50.0]),
    ]);
}

#[test]
fn min_width_overrides_width() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(50.0);
        s.min_size.width = Length::Px(100.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 0.0]),
    ]);
}

#[test]
fn max_width_overrides_width() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(200.0);
        s.max_size.width = Length::Px(100.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 0.0]),
    ]);
}

#[test]
fn min_height_overrides_height() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.height = Length::Px(50.0);
        s.min_size.height = Length::Px(100.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 0.0, 100.0]),
    ]);
}

#[test]
fn max_height_overrides_height() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.height = Length::Px(200.0);
        s.max_size.height = Length::Px(100.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 0.0, 100.0]),
    ]);
}

#[test]
fn min_max_percent_no_width_height() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.align_items = AlignItems::FlexStart;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.min_size.width = Length::Percent(10.0);
        s.min_size.height = Length::Percent(10.0);
        s.max_size.width = Length::Percent(10.0);
        s.max_size.height = Length::Percent(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 10.0, 10.0]),
    ]);
}
