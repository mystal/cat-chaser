use cgmath::{self, Vector2};
use midgar::{self, KeyCode};

pub struct Dog {
    pub pos: Vector2<f32>,
    pub left_key: KeyCode, // TODO: consider breaking this out into control struct
    pub right_key: KeyCode,
    pub up_key: KeyCode,
    pub down_key: KeyCode,
}

pub struct Cat {
    pub pos: Vector2<f32>,
}

pub struct CatBox {
    pub pos: Vector2<f32>,
    pub size: Vector2<f32>,
}
