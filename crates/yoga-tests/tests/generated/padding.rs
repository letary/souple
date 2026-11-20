// @generated — do not edit. Written by the Yoga importer (crates/yoga-tests/src/bin/import.rs):
//     cargo run -p souple-yoga-tests --bin import -- <path to yoga>
// From Yoga 3.2.1, tests/generated/YGPaddingTest.cpp (MIT, Copyright (c) Meta Platforms, Inc. and affiliates).

use crate::support::*;

#[test]
fn padding_no_size() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.padding.left = Length::Px(10.0);
        s.padding.top = Length::Px(10.0);
        s.padding.right = Length::Px(10.0);
        s.padding.bottom = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 20.0, 20.0]),
    ]);
}

#[test]
fn padding_container_match_child() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.padding.left = Length::Px(10.0);
        s.padding.top = Length::Px(10.0);
        s.padding.right = Length::Px(10.0);
        s.padding.bottom = Length::Px(10.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(10.0);
        s.size.height = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 30.0, 30.0]),
        (root_child0, [10.0, 10.0, 10.0, 10.0]),
    ]);
}

#[test]
fn padding_flex_child() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.padding.left = Length::Px(10.0);
        s.padding.top = Length::Px(10.0);
        s.padding.right = Length::Px(10.0);
        s.padding.bottom = Length::Px(10.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_grow = 1.0;
        s.size.width = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [10.0, 10.0, 10.0, 80.0]),
    ]);
}

#[test]
fn padding_stretch_child() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.padding.left = Length::Px(10.0);
        s.padding.top = Length::Px(10.0);
        s.padding.right = Length::Px(10.0);
        s.padding.bottom = Length::Px(10.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.height = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [10.0, 10.0, 80.0, 10.0]),
    ]);
}

#[test]
fn padding_center_child() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.justify_content = JustifyContent::Center;
        s.align_items = AlignItems::Center;
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.padding.left = Length::Px(10.0);
        s.padding.right = Length::Px(20.0);
        s.padding.bottom = Length::Px(20.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(10.0);
        s.size.height = Length::Px(10.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 100.0, 100.0]),
        (root_child0, [40.0, 35.0, 10.0, 10.0]),
    ]);
}

#[test]
fn child_with_padding_align_end() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.justify_content = JustifyContent::FlexEnd;
        s.align_items = AlignItems::FlexEnd;
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Px(100.0);
        s.size.height = Length::Px(100.0);
        s.padding.left = Length::Px(20.0);
        s.padding.top = Length::Px(20.0);
        s.padding.right = Length::Px(20.0);
        s.padding.bottom = Length::Px(20.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [100.0, 100.0, 100.0, 100.0]),
    ]);
}

#[test]
fn physical_and_relative_edge_defined() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.size.width = Length::Px(200.0);
        s.size.height = Length::Px(200.0);
        s.padding.left = Length::Px(20.0);
        s.padding.right = Length::Px(50.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.size.width = Length::Percent(100.0);
        s.size.height = Length::Px(50.0);
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 200.0, 200.0]),
        (root_child0, [20.0, 0.0, 130.0, 50.0]),
    ]);
}
