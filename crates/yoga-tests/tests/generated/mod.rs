// @generated — do not edit. Written by the Yoga importer (crates/yoga-tests/src/bin/import.rs):
//     cargo run -p souple-yoga-tests --bin import -- <path to yoga>
// From Yoga 3.2.1, tests/generated/*.cpp (MIT, Copyright (c) Meta Platforms, Inc. and affiliates).

//! Yoga 3.2.1: 548 tests in 25 files — 457 run, 91 ignored.
//!
//! | Ignored because | Tests |
//! |---|---:|
//! | by design: every box is positioned (no position: static) | 64 |
//! | skipped by Yoga: GTEST_SKIP, Yoga fails it too | 17 |
//! | not planned: display: contents | 9 |
//! | by design: pixel snapping by absolute position; Yoga rounds each offset from its parent (root_child1: top 65, not 64) | 1 |
//!
//! A test ignored for several reasons counts under each.

mod absolute_position;
mod align_content;
mod align_items;
mod align_self;
mod android_news_feed;
mod aspect_ratio;
mod auto;
mod border;
mod box_sizing;
mod dimension;
mod display;
mod display_contents;
mod flex;
mod flex_direction;
mod flex_wrap;
mod gap;
mod intrinsic_size;
mod justify_content;
mod margin;
mod min_max_dimension;
mod padding;
mod percentage;
mod rounding;
mod size_overflow;
mod static_position;
