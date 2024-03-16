use bevy::prelude::*;

pub fn c_root(b: &mut NodeBundle) {
    b.style.width = Val::Percent(100.);
    b.style.height = Val::Percent(100.);
}

pub fn c_cat_tracker(b: &mut NodeBundle) {
    let s = &mut b.style;
    s.position_type = PositionType::Absolute;
    s.right = Val::Px(4.0);
    s.top = Val::Px(4.0);
    s.align_items = AlignItems::Center;
    s.column_gap = Val::Px(4.0);
}

pub fn c_cat_face(assets: &AssetServer, b: &mut ImageBundle) {
    b.image = assets.load("ui/cat_face.png").into();
    let s = &mut b.style;
    s.flex_direction = FlexDirection::Column;
}

pub fn c_tracker_text(_a: &AssetServer, b: &mut TextBundle) {
    b.style.flex_direction = FlexDirection::Column;
}

pub fn c_font_tracker(assets: &AssetServer, s: &mut TextStyle) {
    s.font = assets.load("fonts/Kenney Pixel.ttf").into();
    s.font_size = 16.;
    s.color = Color::WHITE.into();
}
