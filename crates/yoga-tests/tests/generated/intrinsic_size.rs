// @generated — do not edit. Written by the Yoga importer (crates/yoga-tests/src/bin/import.rs):
//     cargo run -p souple-yoga-tests --bin import -- <path to yoga>
// From Yoga 3.2.1, tests/generated/YGIntrinsicSizeTest.cpp (MIT, Copyright (c) Meta Platforms, Inc. and affiliates).

use crate::support::*;

#[test]
fn contains_inner_text_long_word() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.align_items = AlignItems::FlexStart;
        s.size.width = Length::Px(2000.0);
        s.size.height = Length::Px(2000.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::Row;
    });
    t.text(root_child0, "LoremipsumdolorsitametconsecteturadipiscingelitSedeleifasdfettortoracauctorFuscerhoncusipsumtemporerosaliquamconsequatPraesentsoda");
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 2000.0, 2000.0]),
        (root_child0, [0.0, 0.0, 1300.0, 10.0]),
    ]);
}

#[test]
fn contains_inner_text_no_width_no_height() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.align_items = AlignItems::FlexStart;
        s.size.width = Length::Px(2000.0);
        s.size.height = Length::Px(2000.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::Row;
    });
    t.text(root_child0, "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed eleifasd et tortor ac auctor. Integer at volutpat libero, sed elementum dui interdum id. Aliquam consectetur massa vel neque aliquet, quis consequat risus fringilla. Fusce rhoncus ipsum tempor eros aliquam, vel tempus metus ullamcorper. Nam at nulla sed tellus vestibulum fringilla vel sit amet ligula. Proin velit lectus, euismod sit amet quam vel ultricies dolor, vitae finibus lorem ipsum. Pellentesque molestie at mi sit amet dictum. Donec vehicula lacinia felis sit amet consectetur. Praesent sodales enim sapien, sed varius ipsum pellentesque vel. Aenean eu mi eu justo tincidunt finibus vel sit amet ipsum. Sed bibasdum purus vel ipsum sagittis, quis fermentum dolor lobortis. Etiam vulputate eleifasd lectus vel varius. Phasellus imperdiet lectus sit amet ipsum egestas, ut bibasdum ipsum malesuada. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; Sed mollis eros sit amet elit porttitor, vel venenatis turpis venenatis. Nulla tempus tortor at eros efficitur, sit amet dapibus ipsum malesuada. Ut at mauris sed nunc malesuada convallis. Duis id sem vel magna varius eleifasd vel at est. Donec eget orci a ipsum tempor lobortis. Sed at consectetur ipsum.");
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 2000.0, 2000.0]),
        (root_child0, [0.0, 0.0, 2000.0, 70.0]),
    ]);
}

#[test]
fn contains_inner_text_no_width_no_height_long_word_in_paragraph() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.align_items = AlignItems::FlexStart;
        s.size.width = Length::Px(2000.0);
        s.size.height = Length::Px(2000.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::Row;
    });
    t.text(root_child0, "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed eleifasd et tortor ac auctor. Integer at volutpat libero, sed elementum dui interdum id. Aliquam consectetur massa vel neque aliquet, quis consequat risus fringilla. Fusce rhoncus ipsum tempor eros aliquam, vel tempus metus ullamcorper. Nam at nulla sed tellus vestibulum fringilla vel sit amet ligula. Proin velit lectus, euismod sit amet quam vel ultricies dolor, vitae finibus loremipsumloremipsumloremipsumloremipsumloremipsumloremipsumloremipsumloremipsumloremipsumloremipsumloremipsumloremipsumloremipsumlorem Pellentesque molestie at mi sit amet dictum. Donec vehicula lacinia felis sit amet consectetur. Praesent sodales enim sapien, sed varius ipsum pellentesque vel. Aenean eu mi eu justo tincidunt finibus vel sit amet ipsum. Sed bibasdum purus vel ipsum sagittis, quis fermentum dolor lobortis. Etiam vulputate eleifasd lectus vel varius. Phasellus imperdiet lectus sit amet ipsum egestas, ut bibasdum ipsum malesuada. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; Sed mollis eros sit amet elit porttitor, vel venenatis turpis venenatis. Nulla tempus tortor at eros efficitur, sit amet dapibus ipsum malesuada. Ut at mauris sed nunc malesuada convallis. Duis id sem vel magna varius eleifasd vel at est. Donec eget orci a ipsum tempor lobortis. Sed at consectetur ipsum.");
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 2000.0, 2000.0]),
        (root_child0, [0.0, 0.0, 2000.0, 70.0]),
    ]);
}

#[test]
fn contains_inner_text_fixed_width() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.align_items = AlignItems::FlexStart;
        s.size.width = Length::Px(2000.0);
        s.size.height = Length::Px(2000.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(100.0);
    });
    t.text(root_child0, "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed eleifasd et tortor ac auctor. Integer at volutpat libero, sed elementum dui interdum id. Aliquam consectetur massa vel neque aliquet, quis consequat risus fringilla. Fusce rhoncus ipsum tempor eros aliquam, vel tempus metus ullamcorper. Nam at nulla sed tellus vestibulum fringilla vel sit amet ligula. Proin velit lectus, euismod sit amet quam vel ultricies dolor, vitae finibus lorem ipsum. Pellentesque molestie at mi sit amet dictum. Donec vehicula lacinia felis sit amet consectetur. Praesent sodales enim sapien, sed varius ipsum pellentesque vel. Aenean eu mi eu justo tincidunt finibus vel sit amet ipsum. Sed bibasdum purus vel ipsum sagittis, quis fermentum dolor lobortis. Etiam vulputate eleifasd lectus vel varius. Phasellus imperdiet lectus sit amet ipsum egestas, ut bibasdum ipsum malesuada. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; Sed mollis eros sit amet elit porttitor, vel venenatis turpis venenatis. Nulla tempus tortor at eros efficitur, sit amet dapibus ipsum malesuada. Ut at mauris sed nunc malesuada convallis. Duis id sem vel magna varius eleifasd vel at est. Donec eget orci a ipsum tempor lobortis. Sed at consectetur ipsum.");
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 2000.0, 2000.0]),
        (root_child0, [0.0, 0.0, 100.0, 1290.0]),
    ]);
}

#[test]
fn contains_inner_text_no_width_fixed_height() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.align_items = AlignItems::FlexStart;
        s.size.width = Length::Px(2000.0);
        s.size.height = Length::Px(2000.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.height = Length::Px(20.0);
    });
    t.text(root_child0, "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed eleifasd et tortor ac auctor. Integer at volutpat libero, sed elementum dui interdum id. Aliquam consectetur massa vel neque aliquet, quis consequat risus fringilla. Fusce rhoncus ipsum tempor eros aliquam, vel tempus metus ullamcorper. Nam at nulla sed tellus vestibulum fringilla vel sit amet ligula. Proin velit lectus, euismod sit amet quam vel ultricies dolor, vitae finibus lorem ipsum. Pellentesque molestie at mi sit amet dictum. Donec vehicula lacinia felis sit amet consectetur. Praesent sodales enim sapien, sed varius ipsum pellentesque vel. Aenean eu mi eu justo tincidunt finibus vel sit amet ipsum. Sed bibasdum purus vel ipsum sagittis, quis fermentum dolor lobortis. Etiam vulputate eleifasd lectus vel varius. Phasellus imperdiet lectus sit amet ipsum egestas, ut bibasdum ipsum malesuada. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; Sed mollis eros sit amet elit porttitor, vel venenatis turpis venenatis. Nulla tempus tortor at eros efficitur, sit amet dapibus ipsum malesuada. Ut at mauris sed nunc malesuada convallis. Duis id sem vel magna varius eleifasd vel at est. Donec eget orci a ipsum tempor lobortis. Sed at consectetur ipsum.");
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 2000.0, 2000.0]),
        (root_child0, [0.0, 0.0, 2000.0, 20.0]),
    ]);
}

#[test]
fn contains_inner_text_fixed_width_fixed_height() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.align_items = AlignItems::FlexStart;
        s.size.width = Length::Px(2000.0);
        s.size.height = Length::Px(2000.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(50.0);
        s.size.height = Length::Px(20.0);
    });
    t.text(root_child0, "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed eleifasd et tortor ac auctor. Integer at volutpat libero, sed elementum dui interdum id. Aliquam consectetur massa vel neque aliquet, quis consequat risus fringilla. Fusce rhoncus ipsum tempor eros aliquam, vel tempus metus ullamcorper. Nam at nulla sed tellus vestibulum fringilla vel sit amet ligula. Proin velit lectus, euismod sit amet quam vel ultricies dolor, vitae finibus lorem ipsum. Pellentesque molestie at mi sit amet dictum. Donec vehicula lacinia felis sit amet consectetur. Praesent sodales enim sapien, sed varius ipsum pellentesque vel. Aenean eu mi eu justo tincidunt finibus vel sit amet ipsum. Sed bibasdum purus vel ipsum sagittis, quis fermentum dolor lobortis. Etiam vulputate eleifasd lectus vel varius. Phasellus imperdiet lectus sit amet ipsum egestas, ut bibasdum ipsum malesuada. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; Sed mollis eros sit amet elit porttitor, vel venenatis turpis venenatis. Nulla tempus tortor at eros efficitur, sit amet dapibus ipsum malesuada. Ut at mauris sed nunc malesuada convallis. Duis id sem vel magna varius eleifasd vel at est. Donec eget orci a ipsum tempor lobortis. Sed at consectetur ipsum.");
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 2000.0, 2000.0]),
        (root_child0, [0.0, 0.0, 50.0, 20.0]),
    ]);
}

#[test]
fn contains_inner_text_max_width_max_height() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.align_items = AlignItems::FlexStart;
        s.size.width = Length::Px(2000.0);
        s.size.height = Length::Px(2000.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::Row;
        s.max_size.width = Length::Px(50.0);
        s.max_size.height = Length::Px(20.0);
    });
    t.text(root_child0, "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed eleifasd et tortor ac auctor. Integer at volutpat libero, sed elementum dui interdum id. Aliquam consectetur massa vel neque aliquet, quis consequat risus fringilla. Fusce rhoncus ipsum tempor eros aliquam, vel tempus metus ullamcorper. Nam at nulla sed tellus vestibulum fringilla vel sit amet ligula. Proin velit lectus, euismod sit amet quam vel ultricies dolor, vitae finibus lorem ipsum. Pellentesque molestie at mi sit amet dictum. Donec vehicula lacinia felis sit amet consectetur. Praesent sodales enim sapien, sed varius ipsum pellentesque vel. Aenean eu mi eu justo tincidunt finibus vel sit amet ipsum. Sed bibasdum purus vel ipsum sagittis, quis fermentum dolor lobortis. Etiam vulputate eleifasd lectus vel varius. Phasellus imperdiet lectus sit amet ipsum egestas, ut bibasdum ipsum malesuada. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; Sed mollis eros sit amet elit porttitor, vel venenatis turpis venenatis. Nulla tempus tortor at eros efficitur, sit amet dapibus ipsum malesuada. Ut at mauris sed nunc malesuada convallis. Duis id sem vel magna varius eleifasd vel at est. Donec eget orci a ipsum tempor lobortis. Sed at consectetur ipsum.");
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 2000.0, 2000.0]),
        (root_child0, [0.0, 0.0, 50.0, 20.0]),
    ]);
}

#[test]
fn contains_inner_text_max_width() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.align_items = AlignItems::FlexStart;
        s.size.width = Length::Px(2000.0);
        s.size.height = Length::Px(2000.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::Row;
        s.max_size.width = Length::Px(100.0);
    });
    t.text(root_child0, "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed eleifasd et tortor ac auctor. Integer at volutpat libero, sed elementum dui interdum id. Aliquam consectetur massa vel neque aliquet, quis consequat risus fringilla. Fusce rhoncus ipsum tempor eros aliquam, vel tempus metus ullamcorper. Nam at nulla sed tellus vestibulum fringilla vel sit amet ligula. Proin velit lectus, euismod sit amet quam vel ultricies dolor, vitae finibus lorem ipsum. Pellentesque molestie at mi sit amet dictum. Donec vehicula lacinia felis sit amet consectetur. Praesent sodales enim sapien, sed varius ipsum pellentesque vel. Aenean eu mi eu justo tincidunt finibus vel sit amet ipsum. Sed bibasdum purus vel ipsum sagittis, quis fermentum dolor lobortis. Etiam vulputate eleifasd lectus vel varius. Phasellus imperdiet lectus sit amet ipsum egestas, ut bibasdum ipsum malesuada. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; Sed mollis eros sit amet elit porttitor, vel venenatis turpis venenatis. Nulla tempus tortor at eros efficitur, sit amet dapibus ipsum malesuada. Ut at mauris sed nunc malesuada convallis. Duis id sem vel magna varius eleifasd vel at est. Donec eget orci a ipsum tempor lobortis. Sed at consectetur ipsum.");
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 2000.0, 2000.0]),
        (root_child0, [0.0, 0.0, 100.0, 1290.0]),
    ]);
}

#[test]
fn contains_inner_text_fixed_width_shorter_text() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.align_items = AlignItems::FlexStart;
        s.size.width = Length::Px(2000.0);
        s.size.height = Length::Px(2000.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.width = Length::Px(100.0);
    });
    t.text(root_child0, "Lorem ipsum");
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 2000.0, 2000.0]),
        (root_child0, [0.0, 0.0, 100.0, 20.0]),
    ]);
}

#[test]
fn contains_inner_text_fixed_height_shorter_text() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.align_items = AlignItems::FlexStart;
        s.size.width = Length::Px(2000.0);
        s.size.height = Length::Px(2000.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::Row;
        s.size.height = Length::Px(100.0);
    });
    t.text(root_child0, "Lorem ipsum");
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 2000.0, 2000.0]),
        (root_child0, [0.0, 0.0, 110.0, 100.0]),
    ]);
}

#[test]
fn contains_inner_text_max_height() {
    let mut t = Fixture::new();
    let root = t.root("root", |s| {
        s.align_items = AlignItems::FlexStart;
        s.size.width = Length::Px(2000.0);
        s.size.height = Length::Px(2000.0);
    });
    let root_child0 = t.child(root, "root_child0", |s| {
        s.flex_direction = FlexDirection::Row;
        s.max_size.height = Length::Px(20.0);
    });
    t.text(root_child0, "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed eleifasd et tortor ac auctor. Integer at volutpat libero, sed elementum dui interdum id. Aliquam consectetur massa vel neque aliquet, quis consequat risus fringilla. Fusce rhoncus ipsum tempor eros aliquam, vel tempus metus ullamcorper. Nam at nulla sed tellus vestibulum fringilla vel sit amet ligula. Proin velit lectus, euismod sit amet quam vel ultricies dolor, vitae finibus lorem ipsum. Pellentesque molestie at mi sit amet dictum. Donec vehicula lacinia felis sit amet consectetur. Praesent sodales enim sapien, sed varius ipsum pellentesque vel. Aenean eu mi eu justo tincidunt finibus vel sit amet ipsum. Sed bibasdum purus vel ipsum sagittis, quis fermentum dolor lobortis. Etiam vulputate eleifasd lectus vel varius. Phasellus imperdiet lectus sit amet ipsum egestas, ut bibasdum ipsum malesuada. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; Sed mollis eros sit amet elit porttitor, vel venenatis turpis venenatis. Nulla tempus tortor at eros efficitur, sit amet dapibus ipsum malesuada. Ut at mauris sed nunc malesuada convallis. Duis id sem vel magna varius eleifasd vel at est. Donec eget orci a ipsum tempor lobortis. Sed at consectetur ipsum.");
    t.compute_layout(root, None, None);
    t.check(&[
        (root, [0.0, 0.0, 2000.0, 2000.0]),
        (root_child0, [0.0, 0.0, 2000.0, 20.0]),
    ]);
}
