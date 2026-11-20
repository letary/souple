// @generated — do not edit. Written by the Yoga importer (crates/yoga-tests/src/bin/import.rs):
//     cargo run -p souple-yoga-tests --bin import -- <path to yoga>
// From Yoga 3.2.1, tests/generated/YGStaticPositionTest.cpp (MIT, Copyright (c) Meta Platforms, Inc. and affiliates).

use crate::support::*;

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_insets_have_no_effect_left_top() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.inset.left = Length::Px(50.0);
        s.inset.top = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 100.0, 100.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_insets_have_no_effect_right_bottom() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.inset.right = Length::Px(50.0);
        s.inset.bottom = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [0.0, 0.0, 100.0, 100.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_absolute_child_insets_relative_to_positioned_ancestor() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.margin.left = Length::Px(100.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.inset.left = Length::Px(50.0);
        s.inset.top = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 200.0, 200.0]),
        (root_child0_child0, [100.0, 0.0, 100.0, 100.0]),
        (root_child0_child0_child0, [-50.0, 50.0, 50.0, 50.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_absolute_child_insets_relative_to_positioned_ancestor_row_reverse() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::RowReverse;
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.inset.left = Length::Px(50.0);
        s.inset.top = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 200.0, 200.0]),
        (root_child0_child0, [100.0, 0.0, 100.0, 100.0]),
        (root_child0_child0_child0, [-50.0, 50.0, 50.0, 50.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn column_reverse_static_position_absolute_child_insets_relative_to_positioned_ancestor_row_reverse() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::RowReverse;
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.flex_direction = FlexDirection::ColumnReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.inset.left = Length::Px(50.0);
        s.inset.top = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 200.0, 200.0]),
        (root_child0_child0, [100.0, 0.0, 100.0, 100.0]),
        (root_child0_child0_child0, [-50.0, 50.0, 50.0, 50.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_absolute_child_insets_relative_to_positioned_ancestor_row() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.inset.top = Length::Px(50.0);
        s.inset.right = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 200.0, 200.0]),
        (root_child0_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0_child0, [100.0, 50.0, 50.0, 50.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn column_reverse_static_position_absolute_child_insets_relative_to_positioned_ancestor_row() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.flex_direction = FlexDirection::ColumnReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.inset.top = Length::Px(50.0);
        s.inset.right = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 200.0, 200.0]),
        (root_child0_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0_child0, [100.0, 50.0, 50.0, 50.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_absolute_child_insets_relative_to_positioned_ancestor_column_reverse() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::ColumnReverse;
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.inset.top = Length::Px(50.0);
        s.inset.right = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 200.0, 200.0]),
        (root_child0_child0, [0.0, 100.0, 100.0, 100.0]),
        (root_child0_child0_child0, [100.0, -50.0, 50.0, 50.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn column_reverse_static_position_absolute_child_insets_relative_to_positioned_ancestor_column_reverse() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::ColumnReverse;
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.flex_direction = FlexDirection::ColumnReverse;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.inset.top = Length::Px(50.0);
        s.inset.right = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 200.0, 200.0]),
        (root_child0_child0, [0.0, 100.0, 100.0, 100.0]),
        (root_child0_child0_child0, [100.0, -50.0, 50.0, 50.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_absolute_child_insets_relative_to_positioned_ancestor_deep() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.margin.left = Length::Px(100.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.margin.left = Length::Px(100.0);
    });
    let root_child0_child0_child0_child0 = t.child(root_child0_child0_child0, "root_child0_child0_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.margin.left = Length::Px(100.0);
    });
    let root_child0_child0_child0_child0_child0 = t.child(root_child0_child0_child0_child0, "root_child0_child0_child0_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0_child0_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.margin.left = Length::Px(100.0);
    });
    let root_child0_child0_child0_child0_child0_child0 = t.child(root_child0_child0_child0_child0_child0, "root_child0_child0_child0_child0_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.inset.left = Length::Px(50.0);
        s.inset.top = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 200.0, 200.0]),
        (root_child0_child0, [100.0, 0.0, 100.0, 100.0]),
        (root_child0_child0_child0, [100.0, 0.0, 100.0, 100.0]),
        (root_child0_child0_child0_child0, [100.0, 0.0, 100.0, 100.0]),
        (root_child0_child0_child0_child0_child0, [100.0, 0.0, 100.0, 100.0]),
        (root_child0_child0_child0_child0_child0_child0, [-350.0, 50.0, 50.0, 50.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_absolute_child_width_percentage() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Percent(50.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 200.0, 200.0]),
        (root_child0_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0_child0, [0.0, 0.0, 100.0, 50.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_relative_child_width_percentage() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.size.width = Length::Percent(50.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 200.0, 200.0]),
        (root_child0_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0_child0, [0.0, 0.0, 50.0, 50.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_static_child_width_percentage() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Percent(50.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 200.0, 200.0]),
        (root_child0_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0_child0, [0.0, 0.0, 50.0, 50.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_absolute_child_height_percentage() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Percent(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 200.0, 200.0]),
        (root_child0_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0_child0, [0.0, 0.0, 50.0, 100.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_relative_child_height_percentage() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Percent(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 200.0, 200.0]),
        (root_child0_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0_child0, [0.0, 0.0, 50.0, 50.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_static_child_height_percentage() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Percent(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 200.0, 200.0]),
        (root_child0_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0_child0, [0.0, 0.0, 50.0, 50.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_absolute_child_left_percentage() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.inset.left = Length::Percent(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 200.0, 200.0]),
        (root_child0_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0_child0, [100.0, 0.0, 50.0, 50.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_relative_child_left_percentage() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.inset.left = Length::Percent(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 200.0, 200.0]),
        (root_child0_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0_child0, [50.0, 0.0, 50.0, 50.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_static_child_left_percentage() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.inset.left = Length::Percent(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 200.0, 200.0]),
        (root_child0_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0_child0, [0.0, 0.0, 50.0, 50.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_absolute_child_right_percentage() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.inset.right = Length::Percent(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 200.0, 200.0]),
        (root_child0_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0_child0, [50.0, 0.0, 50.0, 50.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_relative_child_right_percentage() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.inset.right = Length::Percent(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 200.0, 200.0]),
        (root_child0_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0_child0, [-50.0, 0.0, 50.0, 50.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_static_child_right_percentage() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.inset.right = Length::Percent(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 200.0, 200.0]),
        (root_child0_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0_child0, [0.0, 0.0, 50.0, 50.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_absolute_child_top_percentage() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.inset.top = Length::Percent(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 200.0, 200.0]),
        (root_child0_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0_child0, [0.0, 100.0, 50.0, 50.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_relative_child_top_percentage() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.inset.top = Length::Percent(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 200.0, 200.0]),
        (root_child0_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0_child0, [0.0, 50.0, 50.0, 50.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_static_child_top_percentage() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.inset.top = Length::Percent(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 200.0, 200.0]),
        (root_child0_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0_child0, [0.0, 0.0, 50.0, 50.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_absolute_child_bottom_percentage() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.inset.bottom = Length::Percent(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 200.0, 200.0]),
        (root_child0_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0_child0, [0.0, 50.0, 50.0, 50.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_relative_child_bottom_percentage() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.inset.bottom = Length::Percent(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 200.0, 200.0]),
        (root_child0_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0_child0, [0.0, -50.0, 50.0, 50.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_static_child_bottom_percentage() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.inset.bottom = Length::Percent(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 200.0, 200.0]),
        (root_child0_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0_child0, [0.0, 0.0, 50.0, 50.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_absolute_child_margin_percentage() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.margin.left = Length::Percent(50.0);
        s.margin.top = Length::Percent(50.0);
        s.margin.right = Length::Percent(50.0);
        s.margin.bottom = Length::Percent(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 200.0, 200.0]),
        (root_child0_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0_child0, [100.0, 100.0, 50.0, 50.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_relative_child_margin_percentage() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.margin.left = Length::Percent(50.0);
        s.margin.top = Length::Percent(50.0);
        s.margin.right = Length::Percent(50.0);
        s.margin.bottom = Length::Percent(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 200.0, 200.0]),
        (root_child0_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0_child0, [50.0, 50.0, 50.0, 50.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_static_child_margin_percentage() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.margin.left = Length::Percent(50.0);
        s.margin.top = Length::Percent(50.0);
        s.margin.right = Length::Percent(50.0);
        s.margin.bottom = Length::Percent(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 200.0, 200.0]),
        (root_child0_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0_child0, [50.0, 50.0, 50.0, 50.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_absolute_child_padding_percentage() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.padding.left = Length::Percent(50.0);
        s.padding.top = Length::Percent(50.0);
        s.padding.right = Length::Percent(50.0);
        s.padding.bottom = Length::Percent(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 200.0, 200.0]),
        (root_child0_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0_child0, [0.0, 0.0, 200.0, 200.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_relative_child_padding_percentage() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.padding.left = Length::Percent(50.0);
        s.padding.top = Length::Percent(50.0);
        s.padding.right = Length::Percent(50.0);
        s.padding.bottom = Length::Percent(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 200.0, 200.0]),
        (root_child0_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0_child0, [0.0, 0.0, 100.0, 100.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_static_child_padding_percentage() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
        s.padding.left = Length::Percent(50.0);
        s.padding.top = Length::Percent(50.0);
        s.padding.right = Length::Percent(50.0);
        s.padding.bottom = Length::Percent(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 200.0, 200.0]),
        (root_child0_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0_child0, [0.0, 0.0, 100.0, 100.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_absolute_child_border_percentage() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 200.0, 200.0]),
        (root_child0_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0_child0, [0.0, 0.0, 50.0, 50.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_relative_child_border_percentage() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 200.0, 200.0]),
        (root_child0_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0_child0, [0.0, 0.0, 50.0, 50.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_static_child_border_percentage() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [0.0, 0.0, 200.0, 200.0]),
        (root_child0_child0, [0.0, 0.0, 100.0, 100.0]),
        (root_child0_child0_child0, [0.0, 0.0, 50.0, 50.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_absolute_child_containing_block_padding_box() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(400.0);
        s.size.height = Length::Px(400.0);
        s.padding.left = Length::Px(100.0);
        s.padding.top = Length::Px(100.0);
        s.padding.right = Length::Px(100.0);
        s.padding.bottom = Length::Px(100.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Percent(50.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 400.0, 400.0]),
        (root_child0, [0.0, 0.0, 400.0, 400.0]),
        (root_child0_child0, [100.0, 100.0, 100.0, 100.0]),
        (root_child0_child0_child0, [0.0, 0.0, 200.0, 50.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_relative_child_containing_block_padding_box() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(400.0);
        s.size.height = Length::Px(400.0);
        s.padding.left = Length::Px(100.0);
        s.padding.top = Length::Px(100.0);
        s.padding.right = Length::Px(100.0);
        s.padding.bottom = Length::Px(100.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.size.width = Length::Percent(50.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 400.0, 400.0]),
        (root_child0, [0.0, 0.0, 400.0, 400.0]),
        (root_child0_child0, [100.0, 100.0, 100.0, 100.0]),
        (root_child0_child0_child0, [0.0, 0.0, 50.0, 50.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_static_child_containing_block_padding_box() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(400.0);
        s.size.height = Length::Px(400.0);
        s.padding.left = Length::Px(100.0);
        s.padding.top = Length::Px(100.0);
        s.padding.right = Length::Px(100.0);
        s.padding.bottom = Length::Px(100.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Percent(50.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 400.0, 400.0]),
        (root_child0, [0.0, 0.0, 400.0, 400.0]),
        (root_child0_child0, [100.0, 100.0, 100.0, 100.0]),
        (root_child0_child0_child0, [0.0, 0.0, 50.0, 50.0]),
    ]);
}

#[test]
fn static_position_absolute_child_containing_block_content_box() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(400.0);
        s.size.height = Length::Px(400.0);
        s.padding.left = Length::Px(100.0);
        s.padding.top = Length::Px(100.0);
        s.padding.right = Length::Px(100.0);
        s.padding.bottom = Length::Px(100.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Percent(50.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 400.0, 400.0]),
        (root_child0, [0.0, 0.0, 400.0, 400.0]),
        (root_child0_child0, [100.0, 100.0, 200.0, 50.0]),
    ]);
}

#[test]
fn static_position_relative_child_containing_block_content_box() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(400.0);
        s.size.height = Length::Px(400.0);
        s.padding.left = Length::Px(100.0);
        s.padding.top = Length::Px(100.0);
        s.padding.right = Length::Px(100.0);
        s.padding.bottom = Length::Px(100.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.size.width = Length::Percent(50.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 400.0, 400.0]),
        (root_child0, [0.0, 0.0, 400.0, 400.0]),
        (root_child0_child0, [100.0, 100.0, 100.0, 50.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_static_child_containing_block_content_box() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(400.0);
        s.size.height = Length::Px(400.0);
        s.padding.left = Length::Px(100.0);
        s.padding.top = Length::Px(100.0);
        s.padding.right = Length::Px(100.0);
        s.padding.bottom = Length::Px(100.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Percent(50.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 400.0, 400.0]),
        (root_child0, [0.0, 0.0, 400.0, 400.0]),
        (root_child0_child0, [100.0, 100.0, 100.0, 50.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_containing_block_padding_and_border() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(400.0);
        s.size.height = Length::Px(400.0);
        s.padding.left = Length::Px(9.0);
        s.padding.top = Length::Px(8.0);
        s.padding.right = Length::Px(1.0);
        s.padding.bottom = Length::Px(4.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(5.0);
        s.border.right = Length::Px(7.0);
        s.border.bottom = Length::Px(4.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Percent(41.0);
        s.size.height = Length::Percent(61.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 400.0, 400.0]),
        (root_child0, [0.0, 0.0, 400.0, 400.0]),
        (root_child0_child0, [11.0, 13.0, 100.0, 100.0]),
        (root_child0_child0_child0, [0.0, 0.0, 160.0, 239.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_amalgamation() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(500.0);
        s.size.height = Length::Px(500.0);
        s.margin.left = Length::Px(4.0);
        s.margin.top = Length::Px(5.0);
        s.margin.right = Length::Px(9.0);
        s.margin.bottom = Length::Px(1.0);
        s.padding.left = Length::Px(2.0);
        s.padding.top = Length::Px(9.0);
        s.padding.right = Length::Px(11.0);
        s.padding.bottom = Length::Px(13.0);
        s.border.left = Length::Px(5.0);
        s.border.top = Length::Px(6.0);
        s.border.right = Length::Px(7.0);
        s.border.bottom = Length::Px(8.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
        s.margin.left = Length::Px(8.0);
        s.margin.top = Length::Px(6.0);
        s.margin.right = Length::Px(3.0);
        s.margin.bottom = Length::Px(9.0);
        s.padding.left = Length::Px(1.0);
        s.padding.top = Length::Px(7.0);
        s.padding.right = Length::Px(9.0);
        s.padding.bottom = Length::Px(4.0);
        s.border.left = Length::Px(8.0);
        s.border.top = Length::Px(10.0);
        s.border.right = Length::Px(2.0);
        s.border.bottom = Length::Px(1.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Percent(41.0);
        s.size.height = Length::Percent(63.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
        s.inset.left = Length::Px(2.0);
        s.inset.right = Length::Px(12.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 513.0, 506.0]),
        (root_child0, [4.0, 5.0, 500.0, 500.0]),
        (root_child0_child0, [15.0, 21.0, 200.0, 200.0]),
        (root_child0_child0_child0, [1.0, 29.0, 200.0, 306.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_no_position_amalgamation() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(500.0);
        s.size.height = Length::Px(500.0);
        s.margin.left = Length::Px(4.0);
        s.margin.top = Length::Px(5.0);
        s.margin.right = Length::Px(9.0);
        s.margin.bottom = Length::Px(1.0);
        s.padding.left = Length::Px(2.0);
        s.padding.top = Length::Px(9.0);
        s.padding.right = Length::Px(11.0);
        s.padding.bottom = Length::Px(13.0);
        s.border.left = Length::Px(5.0);
        s.border.top = Length::Px(6.0);
        s.border.right = Length::Px(7.0);
        s.border.bottom = Length::Px(8.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
        s.margin.left = Length::Px(8.0);
        s.margin.top = Length::Px(6.0);
        s.margin.right = Length::Px(3.0);
        s.margin.bottom = Length::Px(9.0);
        s.padding.left = Length::Px(1.0);
        s.padding.top = Length::Px(7.0);
        s.padding.right = Length::Px(9.0);
        s.padding.bottom = Length::Px(4.0);
        s.border.left = Length::Px(8.0);
        s.border.top = Length::Px(10.0);
        s.border.right = Length::Px(2.0);
        s.border.bottom = Length::Px(1.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Percent(41.0);
        s.size.height = Length::Percent(63.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 513.0, 506.0]),
        (root_child0, [4.0, 5.0, 500.0, 500.0]),
        (root_child0_child0, [15.0, 21.0, 200.0, 200.0]),
        (root_child0_child0_child0, [18.0, 29.0, 200.0, 306.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_zero_for_inset_amalgamation() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(500.0);
        s.size.height = Length::Px(500.0);
        s.margin.left = Length::Px(4.0);
        s.margin.top = Length::Px(5.0);
        s.margin.right = Length::Px(9.0);
        s.margin.bottom = Length::Px(1.0);
        s.padding.left = Length::Px(2.0);
        s.padding.top = Length::Px(9.0);
        s.padding.right = Length::Px(11.0);
        s.padding.bottom = Length::Px(13.0);
        s.border.left = Length::Px(5.0);
        s.border.top = Length::Px(6.0);
        s.border.right = Length::Px(7.0);
        s.border.bottom = Length::Px(8.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
        s.margin.left = Length::Px(8.0);
        s.margin.top = Length::Px(6.0);
        s.margin.right = Length::Px(3.0);
        s.margin.bottom = Length::Px(9.0);
        s.padding.left = Length::Px(1.0);
        s.padding.top = Length::Px(7.0);
        s.padding.right = Length::Px(9.0);
        s.padding.bottom = Length::Px(4.0);
        s.border.left = Length::Px(8.0);
        s.border.top = Length::Px(10.0);
        s.border.right = Length::Px(2.0);
        s.border.bottom = Length::Px(1.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Percent(41.0);
        s.size.height = Length::Percent(63.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
        s.inset.left = Length::Percent(0.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 513.0, 506.0]),
        (root_child0, [4.0, 5.0, 500.0, 500.0]),
        (root_child0_child0, [15.0, 21.0, 200.0, 200.0]),
        (root_child0_child0_child0, [-1.0, 29.0, 200.0, 306.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_start_inset_amalgamation() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(500.0);
        s.size.height = Length::Px(500.0);
        s.margin.left = Length::Px(4.0);
        s.margin.top = Length::Px(5.0);
        s.margin.right = Length::Px(9.0);
        s.margin.bottom = Length::Px(1.0);
        s.padding.left = Length::Px(2.0);
        s.padding.top = Length::Px(9.0);
        s.padding.right = Length::Px(11.0);
        s.padding.bottom = Length::Px(13.0);
        s.border.left = Length::Px(5.0);
        s.border.top = Length::Px(6.0);
        s.border.right = Length::Px(7.0);
        s.border.bottom = Length::Px(8.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
        s.margin.left = Length::Px(8.0);
        s.margin.top = Length::Px(6.0);
        s.margin.right = Length::Px(3.0);
        s.margin.bottom = Length::Px(9.0);
        s.padding.left = Length::Px(1.0);
        s.padding.top = Length::Px(7.0);
        s.padding.right = Length::Px(9.0);
        s.padding.bottom = Length::Px(4.0);
        s.border.left = Length::Px(8.0);
        s.border.top = Length::Px(10.0);
        s.border.right = Length::Px(2.0);
        s.border.bottom = Length::Px(1.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Percent(41.0);
        s.size.height = Length::Percent(63.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
        s.inset.left = Length::Px(12.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 513.0, 506.0]),
        (root_child0, [4.0, 5.0, 500.0, 500.0]),
        (root_child0_child0, [15.0, 21.0, 200.0, 200.0]),
        (root_child0_child0_child0, [11.0, 29.0, 200.0, 306.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_end_inset_amalgamation() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(500.0);
        s.size.height = Length::Px(500.0);
        s.margin.left = Length::Px(4.0);
        s.margin.top = Length::Px(5.0);
        s.margin.right = Length::Px(9.0);
        s.margin.bottom = Length::Px(1.0);
        s.padding.left = Length::Px(2.0);
        s.padding.top = Length::Px(9.0);
        s.padding.right = Length::Px(11.0);
        s.padding.bottom = Length::Px(13.0);
        s.border.left = Length::Px(5.0);
        s.border.top = Length::Px(6.0);
        s.border.right = Length::Px(7.0);
        s.border.bottom = Length::Px(8.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
        s.margin.left = Length::Px(8.0);
        s.margin.top = Length::Px(6.0);
        s.margin.right = Length::Px(3.0);
        s.margin.bottom = Length::Px(9.0);
        s.padding.left = Length::Px(1.0);
        s.padding.top = Length::Px(7.0);
        s.padding.right = Length::Px(9.0);
        s.padding.bottom = Length::Px(4.0);
        s.border.left = Length::Px(8.0);
        s.border.top = Length::Px(10.0);
        s.border.right = Length::Px(2.0);
        s.border.bottom = Length::Px(1.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Percent(41.0);
        s.size.height = Length::Percent(63.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
        s.inset.right = Length::Px(4.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 513.0, 506.0]),
        (root_child0, [4.0, 5.0, 500.0, 500.0]),
        (root_child0_child0, [15.0, 21.0, 200.0, 200.0]),
        (root_child0_child0_child0, [270.0, 29.0, 200.0, 306.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_row_reverse_amalgamation() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.margin.left = Length::Px(4.0);
        s.margin.top = Length::Px(5.0);
        s.margin.right = Length::Px(9.0);
        s.margin.bottom = Length::Px(1.0);
        s.padding.left = Length::Px(2.0);
        s.padding.top = Length::Px(9.0);
        s.padding.right = Length::Px(11.0);
        s.padding.bottom = Length::Px(13.0);
        s.border.left = Length::Px(5.0);
        s.border.top = Length::Px(6.0);
        s.border.right = Length::Px(7.0);
        s.border.bottom = Length::Px(8.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.flex_direction = FlexDirection::RowReverse;
        s.margin.left = Length::Px(8.0);
        s.margin.top = Length::Px(6.0);
        s.margin.right = Length::Px(3.0);
        s.margin.bottom = Length::Px(9.0);
        s.padding.left = Length::Px(1.0);
        s.padding.top = Length::Px(7.0);
        s.padding.right = Length::Px(9.0);
        s.padding.bottom = Length::Px(4.0);
        s.border.left = Length::Px(8.0);
        s.border.top = Length::Px(10.0);
        s.border.right = Length::Px(2.0);
        s.border.bottom = Length::Px(1.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.height = Length::Percent(12.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
    });
    let root_child0_child0_child0_child0 = t.child(root_child0_child0_child0, "root_child0_child0_child0_child0", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(50.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 69.0, 79.0]),
        (root_child0, [4.0, 5.0, 56.0, 73.0]),
        (root_child0_child0, [15.0, 21.0, 20.0, 22.0]),
        (root_child0_child0_child0, [-128.0, 29.0, 133.0, 23.0]),
        (root_child0_child0_child0_child0, [16.0, 16.0, 100.0, 50.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_column_reverse_amalgamation() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.margin.left = Length::Px(4.0);
        s.margin.top = Length::Px(5.0);
        s.margin.right = Length::Px(9.0);
        s.margin.bottom = Length::Px(1.0);
        s.padding.left = Length::Px(2.0);
        s.padding.top = Length::Px(9.0);
        s.padding.right = Length::Px(11.0);
        s.padding.bottom = Length::Px(13.0);
        s.border.left = Length::Px(5.0);
        s.border.top = Length::Px(6.0);
        s.border.right = Length::Px(7.0);
        s.border.bottom = Length::Px(8.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.flex_direction = FlexDirection::ColumnReverse;
        s.margin.left = Length::Px(8.0);
        s.margin.top = Length::Px(6.0);
        s.margin.right = Length::Px(3.0);
        s.margin.bottom = Length::Px(9.0);
        s.padding.left = Length::Px(1.0);
        s.padding.top = Length::Px(7.0);
        s.padding.right = Length::Px(9.0);
        s.padding.bottom = Length::Px(4.0);
        s.border.left = Length::Px(8.0);
        s.border.top = Length::Px(10.0);
        s.border.right = Length::Px(2.0);
        s.border.bottom = Length::Px(1.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Percent(21.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
    });
    let root_child0_child0_child0_child0 = t.child(root_child0_child0_child0, "root_child0_child0_child0_child0", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(50.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 69.0, 79.0]),
        (root_child0, [4.0, 5.0, 56.0, 73.0]),
        (root_child0_child0, [15.0, 21.0, 20.0, 22.0]),
        (root_child0_child0_child0, [18.0, -82.0, 20.0, 92.0]),
        (root_child0_child0_child0_child0, [16.0, 16.0, 100.0, 50.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_justify_flex_start_amalgamation() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.margin.left = Length::Px(4.0);
        s.margin.top = Length::Px(5.0);
        s.margin.right = Length::Px(9.0);
        s.margin.bottom = Length::Px(1.0);
        s.padding.left = Length::Px(2.0);
        s.padding.top = Length::Px(9.0);
        s.padding.right = Length::Px(11.0);
        s.padding.bottom = Length::Px(13.0);
        s.border.left = Length::Px(5.0);
        s.border.top = Length::Px(6.0);
        s.border.right = Length::Px(7.0);
        s.border.bottom = Length::Px(8.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.margin.left = Length::Px(8.0);
        s.margin.top = Length::Px(6.0);
        s.margin.right = Length::Px(3.0);
        s.margin.bottom = Length::Px(9.0);
        s.padding.left = Length::Px(1.0);
        s.padding.top = Length::Px(7.0);
        s.padding.right = Length::Px(9.0);
        s.padding.bottom = Length::Px(4.0);
        s.border.left = Length::Px(8.0);
        s.border.top = Length::Px(10.0);
        s.border.right = Length::Px(2.0);
        s.border.bottom = Length::Px(1.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Percent(21.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
    });
    let root_child0_child0_child0_child0 = t.child(root_child0_child0_child0, "root_child0_child0_child0_child0", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(50.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
    });
    let root_child0_child0_child1 = t.child(root_child0_child0, "root_child0_child0_child1", |s| {
        s.size.width = Length::Percent(10.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
    });
    let root_child0_child0_child1_child0 = t.child(root_child0_child0_child1, "root_child0_child0_child1_child0", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(50.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
    });
    let root_child0_child0_child2 = t.child(root_child0_child0, "root_child0_child0_child2", |s| {
        s.size.width = Length::Percent(10.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
    });
    let root_child0_child0_child2_child0 = t.child(root_child0_child0_child2, "root_child0_child0_child2_child0", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(50.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 215.0, 301.0]),
        (root_child0, [4.0, 5.0, 202.0, 295.0]),
        (root_child0_child0, [15.0, 21.0, 166.0, 244.0]),
        (root_child0_child0_child0, [18.0, 29.0, 40.0, 92.0]),
        (root_child0_child0_child0_child0, [16.0, 16.0, 100.0, 50.0]),
        (root_child0_child0_child1, [18.0, 29.0, 20.0, 92.0]),
        (root_child0_child0_child1_child0, [16.0, 16.0, 100.0, 50.0]),
        (root_child0_child0_child2, [18.0, 140.0, 20.0, 92.0]),
        (root_child0_child0_child2_child0, [16.0, 16.0, 100.0, 50.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_justify_flex_start_position_set_amalgamation() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.margin.left = Length::Px(4.0);
        s.margin.top = Length::Px(5.0);
        s.margin.right = Length::Px(9.0);
        s.margin.bottom = Length::Px(1.0);
        s.padding.left = Length::Px(2.0);
        s.padding.top = Length::Px(9.0);
        s.padding.right = Length::Px(11.0);
        s.padding.bottom = Length::Px(13.0);
        s.border.left = Length::Px(5.0);
        s.border.top = Length::Px(6.0);
        s.border.right = Length::Px(7.0);
        s.border.bottom = Length::Px(8.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.margin.left = Length::Px(8.0);
        s.margin.top = Length::Px(6.0);
        s.margin.right = Length::Px(3.0);
        s.margin.bottom = Length::Px(9.0);
        s.padding.left = Length::Px(1.0);
        s.padding.top = Length::Px(7.0);
        s.padding.right = Length::Px(9.0);
        s.padding.bottom = Length::Px(4.0);
        s.border.left = Length::Px(8.0);
        s.border.top = Length::Px(10.0);
        s.border.right = Length::Px(2.0);
        s.border.bottom = Length::Px(1.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Percent(21.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
        s.inset.right = Length::Px(30.0);
    });
    let root_child0_child0_child0_child0 = t.child(root_child0_child0_child0, "root_child0_child0_child0_child0", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(50.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
    });
    let root_child0_child0_child1 = t.child(root_child0_child0, "root_child0_child0_child1", |s| {
        s.size.width = Length::Percent(10.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
    });
    let root_child0_child0_child1_child0 = t.child(root_child0_child0_child1, "root_child0_child0_child1_child0", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(50.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
    });
    let root_child0_child0_child2 = t.child(root_child0_child0, "root_child0_child0_child2", |s| {
        s.size.width = Length::Percent(10.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
    });
    let root_child0_child0_child2_child0 = t.child(root_child0_child0_child2, "root_child0_child0_child2_child0", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(50.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 215.0, 301.0]),
        (root_child0, [4.0, 5.0, 202.0, 295.0]),
        (root_child0_child0, [15.0, 21.0, 166.0, 244.0]),
        (root_child0_child0_child0, [106.0, 29.0, 40.0, 92.0]),
        (root_child0_child0_child0_child0, [16.0, 16.0, 100.0, 50.0]),
        (root_child0_child0_child1, [18.0, 29.0, 20.0, 92.0]),
        (root_child0_child0_child1_child0, [16.0, 16.0, 100.0, 50.0]),
        (root_child0_child0_child2, [18.0, 140.0, 20.0, 92.0]),
        (root_child0_child0_child2_child0, [16.0, 16.0, 100.0, 50.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_no_definite_size_amalgamation() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.margin.left = Length::Px(4.0);
        s.margin.top = Length::Px(5.0);
        s.margin.right = Length::Px(9.0);
        s.margin.bottom = Length::Px(1.0);
        s.padding.left = Length::Px(2.0);
        s.padding.top = Length::Px(9.0);
        s.padding.right = Length::Px(11.0);
        s.padding.bottom = Length::Px(13.0);
        s.border.left = Length::Px(5.0);
        s.border.top = Length::Px(6.0);
        s.border.right = Length::Px(7.0);
        s.border.bottom = Length::Px(8.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.margin.left = Length::Px(8.0);
        s.margin.top = Length::Px(6.0);
        s.margin.right = Length::Px(3.0);
        s.margin.bottom = Length::Px(9.0);
        s.padding.left = Length::Px(1.0);
        s.padding.top = Length::Px(7.0);
        s.padding.right = Length::Px(9.0);
        s.padding.bottom = Length::Px(4.0);
        s.border.left = Length::Px(8.0);
        s.border.top = Length::Px(10.0);
        s.border.right = Length::Px(2.0);
        s.border.bottom = Length::Px(1.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.position = Position::Absolute;
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
        s.inset.left = Length::Percent(23.0);
    });
    let root_child0_child0_child0_child0 = t.child(root_child0_child0_child0, "root_child0_child0_child0_child0", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(50.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 69.0, 79.0]),
        (root_child0, [4.0, 5.0, 56.0, 73.0]),
        (root_child0_child0, [15.0, 21.0, 20.0, 22.0]),
        (root_child0_child0_child0, [9.0, 29.0, 133.0, 92.0]),
        (root_child0_child0_child0_child0, [16.0, 16.0, 100.0, 50.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_both_insets_set_amalgamation() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.margin.left = Length::Px(4.0);
        s.margin.top = Length::Px(5.0);
        s.margin.right = Length::Px(9.0);
        s.margin.bottom = Length::Px(1.0);
        s.padding.left = Length::Px(2.0);
        s.padding.top = Length::Px(9.0);
        s.padding.right = Length::Px(11.0);
        s.padding.bottom = Length::Px(13.0);
        s.border.left = Length::Px(5.0);
        s.border.top = Length::Px(6.0);
        s.border.right = Length::Px(7.0);
        s.border.bottom = Length::Px(8.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.margin.left = Length::Px(8.0);
        s.margin.top = Length::Px(6.0);
        s.margin.right = Length::Px(3.0);
        s.margin.bottom = Length::Px(9.0);
        s.padding.left = Length::Px(1.0);
        s.padding.top = Length::Px(7.0);
        s.padding.right = Length::Px(9.0);
        s.padding.bottom = Length::Px(4.0);
        s.border.left = Length::Px(8.0);
        s.border.top = Length::Px(10.0);
        s.border.right = Length::Px(2.0);
        s.border.bottom = Length::Px(1.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.position = Position::Absolute;
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
        s.inset.left = Length::Percent(23.0);
        s.inset.right = Length::Px(13.0);
    });
    let root_child0_child0_child0_child0 = t.child(root_child0_child0_child0, "root_child0_child0_child0_child0", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(50.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 69.0, 79.0]),
        (root_child0, [4.0, 5.0, 56.0, 73.0]),
        (root_child0_child0, [15.0, 21.0, 20.0, 22.0]),
        (root_child0_child0_child0, [9.0, 29.0, 20.0, 92.0]),
        (root_child0_child0_child0_child0, [16.0, 16.0, 100.0, 50.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_justify_center_amalgamation() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.margin.left = Length::Px(4.0);
        s.margin.top = Length::Px(5.0);
        s.margin.right = Length::Px(9.0);
        s.margin.bottom = Length::Px(1.0);
        s.padding.left = Length::Px(2.0);
        s.padding.top = Length::Px(9.0);
        s.padding.right = Length::Px(11.0);
        s.padding.bottom = Length::Px(13.0);
        s.border.left = Length::Px(5.0);
        s.border.top = Length::Px(6.0);
        s.border.right = Length::Px(7.0);
        s.border.bottom = Length::Px(8.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.justify_content = JustifyContent::Center;
        s.margin.left = Length::Px(8.0);
        s.margin.top = Length::Px(6.0);
        s.margin.right = Length::Px(3.0);
        s.margin.bottom = Length::Px(9.0);
        s.padding.left = Length::Px(1.0);
        s.padding.top = Length::Px(7.0);
        s.padding.right = Length::Px(9.0);
        s.padding.bottom = Length::Px(4.0);
        s.border.left = Length::Px(8.0);
        s.border.top = Length::Px(10.0);
        s.border.right = Length::Px(2.0);
        s.border.bottom = Length::Px(1.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Percent(21.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
    });
    let root_child0_child0_child0_child0 = t.child(root_child0_child0_child0, "root_child0_child0_child0_child0", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(50.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
    });
    let root_child0_child0_child1 = t.child(root_child0_child0, "root_child0_child0_child1", |s| {
        s.size.width = Length::Percent(10.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
    });
    let root_child0_child0_child1_child0 = t.child(root_child0_child0_child1, "root_child0_child0_child1_child0", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(50.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
    });
    let root_child0_child0_child2 = t.child(root_child0_child0, "root_child0_child0_child2", |s| {
        s.size.width = Length::Percent(10.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
    });
    let root_child0_child0_child2_child0 = t.child(root_child0_child0_child2, "root_child0_child0_child2_child0", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(50.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 215.0, 301.0]),
        (root_child0, [4.0, 5.0, 202.0, 295.0]),
        (root_child0_child0, [15.0, 21.0, 166.0, 244.0]),
        (root_child0_child0_child0, [18.0, 85.0, 40.0, 92.0]),
        (root_child0_child0_child0_child0, [16.0, 16.0, 100.0, 50.0]),
        (root_child0_child0_child1, [18.0, 29.0, 20.0, 92.0]),
        (root_child0_child0_child1_child0, [16.0, 16.0, 100.0, 50.0]),
        (root_child0_child0_child2, [18.0, 140.0, 20.0, 92.0]),
        (root_child0_child0_child2_child0, [16.0, 16.0, 100.0, 50.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_justify_flex_end_amalgamation() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.margin.left = Length::Px(4.0);
        s.margin.top = Length::Px(5.0);
        s.margin.right = Length::Px(9.0);
        s.margin.bottom = Length::Px(1.0);
        s.padding.left = Length::Px(2.0);
        s.padding.top = Length::Px(9.0);
        s.padding.right = Length::Px(11.0);
        s.padding.bottom = Length::Px(13.0);
        s.border.left = Length::Px(5.0);
        s.border.top = Length::Px(6.0);
        s.border.right = Length::Px(7.0);
        s.border.bottom = Length::Px(8.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.justify_content = JustifyContent::FlexEnd;
        s.margin.left = Length::Px(8.0);
        s.margin.top = Length::Px(6.0);
        s.margin.right = Length::Px(3.0);
        s.margin.bottom = Length::Px(9.0);
        s.padding.left = Length::Px(1.0);
        s.padding.top = Length::Px(7.0);
        s.padding.right = Length::Px(9.0);
        s.padding.bottom = Length::Px(4.0);
        s.border.left = Length::Px(8.0);
        s.border.top = Length::Px(10.0);
        s.border.right = Length::Px(2.0);
        s.border.bottom = Length::Px(1.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Percent(21.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
    });
    let root_child0_child0_child0_child0 = t.child(root_child0_child0_child0, "root_child0_child0_child0_child0", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(50.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
    });
    let root_child0_child0_child1 = t.child(root_child0_child0, "root_child0_child0_child1", |s| {
        s.size.width = Length::Percent(10.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
    });
    let root_child0_child0_child1_child0 = t.child(root_child0_child0_child1, "root_child0_child0_child1_child0", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(50.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
    });
    let root_child0_child0_child2 = t.child(root_child0_child0, "root_child0_child0_child2", |s| {
        s.size.width = Length::Percent(10.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
    });
    let root_child0_child0_child2_child0 = t.child(root_child0_child0_child2, "root_child0_child0_child2_child0", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(50.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 215.0, 301.0]),
        (root_child0, [4.0, 5.0, 202.0, 295.0]),
        (root_child0_child0, [15.0, 21.0, 166.0, 244.0]),
        (root_child0_child0_child0, [18.0, 140.0, 40.0, 92.0]),
        (root_child0_child0_child0_child0, [16.0, 16.0, 100.0, 50.0]),
        (root_child0_child0_child1, [18.0, 29.0, 20.0, 92.0]),
        (root_child0_child0_child1_child0, [16.0, 16.0, 100.0, 50.0]),
        (root_child0_child0_child2, [18.0, 140.0, 20.0, 92.0]),
        (root_child0_child0_child2_child0, [16.0, 16.0, 100.0, 50.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_align_flex_start_amalgamation() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.margin.left = Length::Px(4.0);
        s.margin.top = Length::Px(5.0);
        s.margin.right = Length::Px(9.0);
        s.margin.bottom = Length::Px(1.0);
        s.padding.left = Length::Px(2.0);
        s.padding.top = Length::Px(9.0);
        s.padding.right = Length::Px(11.0);
        s.padding.bottom = Length::Px(13.0);
        s.border.left = Length::Px(5.0);
        s.border.top = Length::Px(6.0);
        s.border.right = Length::Px(7.0);
        s.border.bottom = Length::Px(8.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.align_items = AlignItems::FlexStart;
        s.margin.left = Length::Px(8.0);
        s.margin.top = Length::Px(6.0);
        s.margin.right = Length::Px(3.0);
        s.margin.bottom = Length::Px(9.0);
        s.padding.left = Length::Px(1.0);
        s.padding.top = Length::Px(7.0);
        s.padding.right = Length::Px(9.0);
        s.padding.bottom = Length::Px(4.0);
        s.border.left = Length::Px(8.0);
        s.border.top = Length::Px(10.0);
        s.border.right = Length::Px(2.0);
        s.border.bottom = Length::Px(1.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Percent(21.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
    });
    let root_child0_child0_child0_child0 = t.child(root_child0_child0_child0, "root_child0_child0_child0_child0", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(50.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
    });
    let root_child0_child0_child1 = t.child(root_child0_child0, "root_child0_child0_child1", |s| {
        s.size.width = Length::Percent(10.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
    });
    let root_child0_child0_child1_child0 = t.child(root_child0_child0_child1, "root_child0_child0_child1_child0", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(50.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
    });
    let root_child0_child0_child2 = t.child(root_child0_child0, "root_child0_child0_child2", |s| {
        s.size.width = Length::Percent(10.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
    });
    let root_child0_child0_child2_child0 = t.child(root_child0_child0_child2, "root_child0_child0_child2_child0", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(50.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 215.0, 301.0]),
        (root_child0, [4.0, 5.0, 202.0, 295.0]),
        (root_child0_child0, [15.0, 21.0, 166.0, 244.0]),
        (root_child0_child0_child0, [18.0, 29.0, 40.0, 92.0]),
        (root_child0_child0_child0_child0, [16.0, 16.0, 100.0, 50.0]),
        (root_child0_child0_child1, [18.0, 29.0, 20.0, 92.0]),
        (root_child0_child0_child1_child0, [16.0, 16.0, 100.0, 50.0]),
        (root_child0_child0_child2, [18.0, 140.0, 20.0, 92.0]),
        (root_child0_child0_child2_child0, [16.0, 16.0, 100.0, 50.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_align_center_amalgamation() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.margin.left = Length::Px(4.0);
        s.margin.top = Length::Px(5.0);
        s.margin.right = Length::Px(9.0);
        s.margin.bottom = Length::Px(1.0);
        s.padding.left = Length::Px(2.0);
        s.padding.top = Length::Px(9.0);
        s.padding.right = Length::Px(11.0);
        s.padding.bottom = Length::Px(13.0);
        s.border.left = Length::Px(5.0);
        s.border.top = Length::Px(6.0);
        s.border.right = Length::Px(7.0);
        s.border.bottom = Length::Px(8.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.align_items = AlignItems::Center;
        s.margin.left = Length::Px(8.0);
        s.margin.top = Length::Px(6.0);
        s.margin.right = Length::Px(3.0);
        s.margin.bottom = Length::Px(9.0);
        s.padding.left = Length::Px(1.0);
        s.padding.top = Length::Px(7.0);
        s.padding.right = Length::Px(9.0);
        s.padding.bottom = Length::Px(4.0);
        s.border.left = Length::Px(8.0);
        s.border.top = Length::Px(10.0);
        s.border.right = Length::Px(2.0);
        s.border.bottom = Length::Px(1.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Percent(21.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
    });
    let root_child0_child0_child0_child0 = t.child(root_child0_child0_child0, "root_child0_child0_child0_child0", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(50.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
    });
    let root_child0_child0_child1 = t.child(root_child0_child0, "root_child0_child0_child1", |s| {
        s.size.width = Length::Percent(10.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
    });
    let root_child0_child0_child1_child0 = t.child(root_child0_child0_child1, "root_child0_child0_child1_child0", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(50.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
    });
    let root_child0_child0_child2 = t.child(root_child0_child0, "root_child0_child0_child2", |s| {
        s.size.width = Length::Percent(10.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
    });
    let root_child0_child0_child2_child0 = t.child(root_child0_child0_child2, "root_child0_child0_child2_child0", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(50.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 215.0, 301.0]),
        (root_child0, [4.0, 5.0, 202.0, 295.0]),
        (root_child0_child0, [15.0, 21.0, 166.0, 244.0]),
        (root_child0_child0_child0, [65.0, 29.0, 39.0, 92.0]),
        (root_child0_child0_child0_child0, [16.0, 16.0, 100.0, 50.0]),
        (root_child0_child0_child1, [75.0, 29.0, 20.0, 92.0]),
        (root_child0_child0_child1_child0, [16.0, 16.0, 100.0, 50.0]),
        (root_child0_child0_child2, [75.0, 140.0, 20.0, 92.0]),
        (root_child0_child0_child2_child0, [16.0, 16.0, 100.0, 50.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_align_flex_end_amalgamation() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.margin.left = Length::Px(4.0);
        s.margin.top = Length::Px(5.0);
        s.margin.right = Length::Px(9.0);
        s.margin.bottom = Length::Px(1.0);
        s.padding.left = Length::Px(2.0);
        s.padding.top = Length::Px(9.0);
        s.padding.right = Length::Px(11.0);
        s.padding.bottom = Length::Px(13.0);
        s.border.left = Length::Px(5.0);
        s.border.top = Length::Px(6.0);
        s.border.right = Length::Px(7.0);
        s.border.bottom = Length::Px(8.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.align_items = AlignItems::FlexEnd;
        s.margin.left = Length::Px(8.0);
        s.margin.top = Length::Px(6.0);
        s.margin.right = Length::Px(3.0);
        s.margin.bottom = Length::Px(9.0);
        s.padding.left = Length::Px(1.0);
        s.padding.top = Length::Px(7.0);
        s.padding.right = Length::Px(9.0);
        s.padding.bottom = Length::Px(4.0);
        s.border.left = Length::Px(8.0);
        s.border.top = Length::Px(10.0);
        s.border.right = Length::Px(2.0);
        s.border.bottom = Length::Px(1.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Percent(21.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
    });
    let root_child0_child0_child0_child0 = t.child(root_child0_child0_child0, "root_child0_child0_child0_child0", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(50.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
    });
    let root_child0_child0_child1 = t.child(root_child0_child0, "root_child0_child0_child1", |s| {
        s.size.width = Length::Percent(10.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
    });
    let root_child0_child0_child1_child0 = t.child(root_child0_child0_child1, "root_child0_child0_child1_child0", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(50.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
    });
    let root_child0_child0_child2 = t.child(root_child0_child0, "root_child0_child0_child2", |s| {
        s.size.width = Length::Percent(10.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
    });
    let root_child0_child0_child2_child0 = t.child(root_child0_child0_child2, "root_child0_child0_child2_child0", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(50.0);
        s.margin.left = Length::Px(9.0);
        s.margin.top = Length::Px(12.0);
        s.margin.right = Length::Px(4.0);
        s.margin.bottom = Length::Px(7.0);
        s.padding.left = Length::Px(5.0);
        s.padding.top = Length::Px(3.0);
        s.padding.right = Length::Px(8.0);
        s.padding.bottom = Length::Px(10.0);
        s.border.left = Length::Px(2.0);
        s.border.top = Length::Px(1.0);
        s.border.right = Length::Px(5.0);
        s.border.bottom = Length::Px(9.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 215.0, 301.0]),
        (root_child0, [4.0, 5.0, 202.0, 295.0]),
        (root_child0_child0, [15.0, 21.0, 166.0, 244.0]),
        (root_child0_child0_child0, [111.0, 29.0, 40.0, 92.0]),
        (root_child0_child0_child0_child0, [16.0, 16.0, 100.0, 50.0]),
        (root_child0_child0_child1, [131.0, 29.0, 20.0, 92.0]),
        (root_child0_child0_child1_child0, [16.0, 16.0, 100.0, 50.0]),
        (root_child0_child0_child2, [131.0, 140.0, 20.0, 92.0]),
        (root_child0_child0_child2_child0, [16.0, 16.0, 100.0, 50.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_static_root() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        // untranslated: YGNodeStyleSetPositionType(root, YGPositionTypeStatic);
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(200.0);
        s.padding.left = Length::Px(6.0);
        s.padding.top = Length::Px(1.0);
        s.padding.right = Length::Px(11.0);
        s.padding.bottom = Length::Px(4.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Percent(50.0);
        s.size.height = Length::Percent(50.0);
        s.margin.left = Length::Px(12.0);
        s.margin.top = Length::Px(11.0);
        s.margin.right = Length::Px(15.0);
        s.margin.bottom = Length::Px(1.0);
        s.padding.left = Length::Px(3.0);
        s.padding.top = Length::Px(7.0);
        s.padding.right = Length::Px(5.0);
        s.padding.bottom = Length::Px(4.0);
        s.border.left = Length::Px(4.0);
        s.border.top = Length::Px(3.0);
        s.border.right = Length::Px(2.0);
        s.border.bottom = Length::Px(1.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 200.0]),
        (root_child0, [18.0, 12.0, 50.0, 100.0]),
    ]);
}

#[test]
#[ignore = "by design: every box is positioned (no position: static)"]
fn static_position_absolute_child_multiple() {
    let mut t = Fixture::new();
    let root = t.root("root", |_| {});
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(400.0);
        s.size.height = Length::Px(400.0);
        s.padding.left = Length::Px(100.0);
        s.padding.top = Length::Px(100.0);
        s.padding.right = Length::Px(100.0);
        s.padding.bottom = Length::Px(100.0);
    });
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child0, YGPositionTypeStatic);
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Percent(10.0);
        s.size.height = Length::Px(50.0);
    });
    let root_child0_child1 = t.child(root_child0, "root_child0_child1", |s| {
        // untranslated: YGNodeStyleSetPositionType(root_child0_child1, YGPositionTypeStatic);
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
    });
    let root_child0_child1_child0 = t.child(root_child0_child1, "root_child0_child1_child0", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Percent(50.0);
        s.size.height = Length::Px(50.0);
    });
    let root_child0_child1_child1 = t.child(root_child0_child1, "root_child0_child1_child1", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Percent(50.0);
        s.size.height = Length::Px(50.0);
    });
    let root_child0_child2 = t.child(root_child0, "root_child0_child2", |s| {
        s.position = Position::Absolute;
        s.size.width = Length::Px(25.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 400.0, 400.0]),
        (root_child0, [0.0, 0.0, 400.0, 400.0]),
        (root_child0_child0, [100.0, 100.0, 100.0, 100.0]),
        (root_child0_child0_child0, [0.0, 0.0, 40.0, 50.0]),
        (root_child0_child1, [100.0, 200.0, 100.0, 100.0]),
        (root_child0_child1_child0, [0.0, 0.0, 200.0, 50.0]),
        (root_child0_child1_child1, [0.0, 0.0, 200.0, 50.0]),
        (root_child0_child2, [100.0, 100.0, 25.0, 50.0]),
    ]);
}
