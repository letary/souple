// @generated — do not edit. Written by the Yoga importer (crates/yoga-tests/src/bin/import.rs):
//     cargo run -p souple-yoga-tests --bin import -- <path to yoga>
// From Yoga 3.2.1, tests/generated/YGAndroidNewsFeed.cpp (MIT, Copyright (c) Meta Platforms, Inc. and affiliates).

use crate::support::*;

#[test]
fn android_news_feed() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.align_content = AlignContent::Stretch;
        s.size.width = Length::Px(1080.0);
    });
    let root_child0 = t.child(root, "root_child0", |_| {});
    let root_child0_child0 = t.child(root_child0, "root_child0_child0", |s| {
        s.align_content = AlignContent::Stretch;
    });
    let root_child0_child0_child0 = t.child(root_child0_child0, "root_child0_child0_child0", |s| {
        s.align_content = AlignContent::Stretch;
    });
    let root_child0_child0_child0_child0 = t.child(root_child0_child0_child0, "root_child0_child0_child0_child0", |s| {
        s.flex_direction = FlexDirection::Row;
        s.align_items = AlignItems::FlexStart;
        s.align_content = AlignContent::Stretch;
        s.margin.left = Length::Px(36.0);
        s.margin.top = Length::Px(24.0);
    });
    let root_child0_child0_child0_child0_child0 = t.child(root_child0_child0_child0_child0, "root_child0_child0_child0_child0_child0", |s| {
        s.flex_direction = FlexDirection::Row;
        s.align_content = AlignContent::Stretch;
    });
    let root_child0_child0_child0_child0_child0_child0 = t.child(root_child0_child0_child0_child0_child0, "root_child0_child0_child0_child0_child0_child0", |s| {
        s.align_content = AlignContent::Stretch;
        s.size.width = Length::Px(120.0);
        s.size.height = Length::Px(120.0);
    });
    let root_child0_child0_child0_child0_child1 = t.child(root_child0_child0_child0_child0, "root_child0_child0_child0_child0_child1", |s| {
        s.align_content = AlignContent::Stretch;
        s.flex_shrink = 1.0;
        s.margin.right = Length::Px(36.0);
        s.padding.left = Length::Px(36.0);
        s.padding.top = Length::Px(21.0);
        s.padding.right = Length::Px(36.0);
        s.padding.bottom = Length::Px(18.0);
    });
    let root_child0_child0_child0_child0_child1_child0 = t.child(root_child0_child0_child0_child0_child1, "root_child0_child0_child0_child0_child1_child0", |s| {
        s.flex_direction = FlexDirection::Row;
        s.align_content = AlignContent::Stretch;
        s.flex_shrink = 1.0;
    });
    let root_child0_child0_child0_child0_child1_child1 = t.child(root_child0_child0_child0_child0_child1, "root_child0_child0_child0_child0_child1_child1", |s| {
        s.align_content = AlignContent::Stretch;
        s.flex_shrink = 1.0;
    });
    let root_child0_child0_child1 = t.child(root_child0_child0, "root_child0_child0_child1", |s| {
        s.align_content = AlignContent::Stretch;
    });
    let root_child0_child0_child1_child0 = t.child(root_child0_child0_child1, "root_child0_child0_child1_child0", |s| {
        s.flex_direction = FlexDirection::Row;
        s.align_items = AlignItems::FlexStart;
        s.align_content = AlignContent::Stretch;
        s.margin.left = Length::Px(174.0);
        s.margin.top = Length::Px(24.0);
    });
    let root_child0_child0_child1_child0_child0 = t.child(root_child0_child0_child1_child0, "root_child0_child0_child1_child0_child0", |s| {
        s.flex_direction = FlexDirection::Row;
        s.align_content = AlignContent::Stretch;
    });
    let root_child0_child0_child1_child0_child0_child0 = t.child(root_child0_child0_child1_child0_child0, "root_child0_child0_child1_child0_child0_child0", |s| {
        s.align_content = AlignContent::Stretch;
        s.size.width = Length::Px(72.0);
        s.size.height = Length::Px(72.0);
    });
    let root_child0_child0_child1_child0_child1 = t.child(root_child0_child0_child1_child0, "root_child0_child0_child1_child0_child1", |s| {
        s.align_content = AlignContent::Stretch;
        s.flex_shrink = 1.0;
        s.margin.right = Length::Px(36.0);
        s.padding.left = Length::Px(36.0);
        s.padding.top = Length::Px(21.0);
        s.padding.right = Length::Px(36.0);
        s.padding.bottom = Length::Px(18.0);
    });
    let root_child0_child0_child1_child0_child1_child0 = t.child(root_child0_child0_child1_child0_child1, "root_child0_child0_child1_child0_child1_child0", |s| {
        s.flex_direction = FlexDirection::Row;
        s.align_content = AlignContent::Stretch;
        s.flex_shrink = 1.0;
    });
    let root_child0_child0_child1_child0_child1_child1 = t.child(root_child0_child0_child1_child0_child1, "root_child0_child0_child1_child0_child1_child1", |s| {
        s.align_content = AlignContent::Stretch;
        s.flex_shrink = 1.0;
    });
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 1080.0, 240.0]),
        (root_child0, [0.0, 0.0, 1080.0, 240.0]),
        (root_child0_child0, [0.0, 0.0, 1080.0, 240.0]),
        (root_child0_child0_child0, [0.0, 0.0, 1080.0, 144.0]),
        (root_child0_child0_child0_child0, [36.0, 24.0, 1044.0, 120.0]),
        (root_child0_child0_child0_child0_child0, [0.0, 0.0, 120.0, 120.0]),
        (root_child0_child0_child0_child0_child0_child0, [0.0, 0.0, 120.0, 120.0]),
        (root_child0_child0_child0_child0_child1, [120.0, 0.0, 72.0, 39.0]),
        (root_child0_child0_child0_child0_child1_child0, [36.0, 21.0, 0.0, 0.0]),
        (root_child0_child0_child0_child0_child1_child1, [36.0, 21.0, 0.0, 0.0]),
        (root_child0_child0_child1, [0.0, 144.0, 1080.0, 96.0]),
        (root_child0_child0_child1_child0, [174.0, 24.0, 906.0, 72.0]),
        (root_child0_child0_child1_child0_child0, [0.0, 0.0, 72.0, 72.0]),
        (root_child0_child0_child1_child0_child0_child0, [0.0, 0.0, 72.0, 72.0]),
        (root_child0_child0_child1_child0_child1, [72.0, 0.0, 72.0, 39.0]),
        (root_child0_child0_child1_child0_child1_child0, [36.0, 21.0, 0.0, 0.0]),
        (root_child0_child0_child1_child0_child1_child1, [36.0, 21.0, 0.0, 0.0]),
    ]);
}
