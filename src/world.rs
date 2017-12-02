use midgar::Midgar;

use cgmath::{self, Vector2};

use config;

pub struct Dog {
    pub pos: Vector2<f32>,
}

pub struct Cat {
    pub pos: Vector2<f32>,
}

pub struct CatBox {
    pub pos: Vector2<f32>,
    pub size: Vector2<f32>,
}

pub struct GameWorld {
    pub dog: Dog,
    pub cats: Vec<Cat>,
    pub cat_box: CatBox,
    pub bounds: Vector2<u32>,
}

impl GameWorld {
    pub fn new() -> Self {
        let cats = vec![
            Cat {
                pos: cgmath::vec2(400.0, 400.0),
            },
        ];

        GameWorld {
            dog: Dog {
                pos: cgmath::vec2(700.0, 100.0),
            },
            cats,
            cat_box: CatBox {
                pos: cgmath::vec2(200.0, 200.0),
                size: cgmath::vec2(80.0, 80.0),
            },
            bounds: config::SCREEN_SIZE,
        }
    }

    pub fn update(&mut self, midgar: &Midgar, dt: f32) {
    }
}
