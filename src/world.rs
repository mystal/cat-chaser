use midgar::Midgar;

use cgmath::{self, Vector2};
use midgar::{self, KeyCode};

use config;

const MOVE_SPEED: f32 = 150.0;

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
                left_key: KeyCode::Left,
                right_key: KeyCode::Right,
                up_key: KeyCode::Up,
                down_key: KeyCode::Down,
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
        // TODO: consider moving this into a poll input method
        if midgar.input().is_key_held(self.dog.left_key) && !midgar.input().is_key_held(self.dog.right_key) {
            self.dog.pos.x -= MOVE_SPEED * dt;
        }

        if midgar.input().is_key_held(self.dog.right_key) && !midgar.input().is_key_held(self.dog.left_key) {
            self.dog.pos.x += MOVE_SPEED * dt;
        }

        if midgar.input().is_key_held(self.dog.up_key) && !midgar.input().is_key_held(self.dog.down_key) {
            self.dog.pos.y -= MOVE_SPEED * dt;
        }

        if midgar.input().is_key_held(self.dog.down_key) && !midgar.input().is_key_held(self.dog.up_key) {
            self.dog.pos.y += MOVE_SPEED * dt;
        }

        // TODO: Cats move or run!

        // TODO: Check win condition!
    }
}
