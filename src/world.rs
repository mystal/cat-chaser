use midgar::Midgar;

use cgmath::{self, Vector2};
use midgar::{self, KeyCode};

use config;
use entities::*;
use level::Level;

const MOVE_SPEED: f32 = 150.0;

pub struct GameWorld {
    pub level: Level,
    pub dog: Dog,
    pub cats: Vec<Cat>,
}

impl GameWorld {
    pub fn new() -> Self {
        let level = Level {
            cat_box: CatBox {
                pos: cgmath::vec2(200.0, 200.0),
                size: cgmath::vec2(80.0, 80.0),
            },
            num_cats: 10,
            bounds: config::SCREEN_SIZE,
        };
        let dog = Dog {
            pos: level.cat_box.pos,
            left_key: KeyCode::Left,
            right_key: KeyCode::Right,
            up_key: KeyCode::Up,
            down_key: KeyCode::Down,
        };
        let cats = level.generate_cats();

        GameWorld {
            level,
            dog,
            cats,
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

    pub fn cat_box(&self) -> &CatBox {
        &self.level.cat_box
    }
}
