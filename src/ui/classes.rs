use bevy::prelude::*;

pub fn c_root(b: &mut NodeBundle) {
    b.style.width = Val::Percent(100.);
    b.style.height = Val::Percent(100.);
}

pub fn c_start_image(assets: &AssetServer, b: &mut ImageBundle) {
    b.image = assets.load("ui/start_menu_background.png").into();
}

pub fn c_start_text(_a: &AssetServer, b: &mut TextBundle) {
    let s = &mut b.style;
    s.position_type = PositionType::Absolute;
    s.right = Val::Px(4.0);
    s.bottom = Val::Px(6.0);
}

pub fn c_credits_text_linda(_a: &AssetServer, b: &mut TextBundle) {
    b.style.position_type = PositionType::Absolute;
    b.style.align_self = AlignSelf::Center;
    b.style.left = Val::Px(105.0);
    b.style.top = Val::Px(34.0);
}

pub fn c_credits_text_morgan(_a: &AssetServer, b: &mut TextBundle) {
    b.style.position_type = PositionType::Absolute;
    b.style.align_self = AlignSelf::Center;
    b.style.left = Val::Px(105.0);
    b.style.top = Val::Px(68.0);
}

pub fn c_credits_text_justin(_a: &AssetServer, b: &mut TextBundle) {
    b.style.position_type = PositionType::Absolute;
    b.style.align_self = AlignSelf::Center;
    b.style.left = Val::Px(105.0);
    b.style.top = Val::Px(98.0);
}

pub fn c_credits_text_gabe(_a: &AssetServer, b: &mut TextBundle) {
    b.style.position_type = PositionType::Absolute;
    b.style.align_self = AlignSelf::Center;
    b.style.left = Val::Px(105.0);
    b.style.top = Val::Px(132.0);
}

pub fn c_credits_text_thaminda(_a: &AssetServer, b: &mut TextBundle) {
    b.style.position_type = PositionType::Absolute;
    b.style.align_self = AlignSelf::Center;
    b.style.left = Val::Px(105.0);
    b.style.top = Val::Px(163.0);
}

pub fn c_font_credits(assets: &AssetServer, s: &mut TextStyle) {
    s.font = assets.load("fonts/Kenney Pixel.ttf");
    s.font_size = 11.0;
    s.color = Color::BLACK;
}

pub fn c_font_start(assets: &AssetServer, s: &mut TextStyle) {
    s.font = assets.load("fonts/Kenney Pixel.ttf");
    s.font_size = 13.0;
    s.color = Color::WHITE;
}

pub fn c_font_how_to_play(assets: &AssetServer, s: &mut TextStyle) {
    s.font = assets.load("fonts/Kenney Pixel.ttf");
    s.font_size = 13.0;
    s.color = Color::BLACK;
}

pub fn c_cat_tracker(b: &mut NodeBundle) {
    let s = &mut b.style;
    s.position_type = PositionType::Absolute;
    s.right = Val::Px(4.0);
    s.top = Val::Px(2.0);
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
    s.font = assets.load("fonts/Kenney Pixel.ttf");
    s.font_size = 14.0;
    s.color = Color::WHITE;
}

pub fn c_next_level(b: &mut NodeBundle) {
    let s = &mut b.style;
    s.position_type = PositionType::Absolute;
    s.right = Val::Px(6.0);
    s.bottom = Val::Px(6.0);
}

pub fn c_next_level_text(_a: &AssetServer, b: &mut TextBundle) {
    b.visibility = Visibility::Hidden;
}

pub fn c_font_next_level(assets: &AssetServer, s: &mut TextStyle) {
    s.font = assets.load("fonts/Kenney Pixel.ttf");
    s.font_size = 12.0;
    s.color = Color::WHITE;
}

pub fn c_victory(b: &mut NodeBundle) {
    let s = &mut b.style;
    s.position_type = PositionType::Absolute;
    s.width = Val::Percent(100.0);
    s.justify_content = JustifyContent::Center;
    // s.right = Val::Px(6.0);
    s.bottom = Val::Px(6.0);
}
