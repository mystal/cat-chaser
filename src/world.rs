use midgar::Midgar;

use cgmath::{self, Vector2, InnerSpace};
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
            bounds: cgmath::vec2(config::SCREEN_SIZE.x, config::SCREEN_SIZE.y),
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
        for cat in &mut self.cats {
            match cat.get_state(&self.dog, &self.level.cat_box) {
                CatState::Idle => { cat.idle(&self.level.bounds) },
                CatState::InPen => { cat.in_pen(&self.level.bounds) },
                CatState::Flee => {
                    let dir = &cat.pos - self.dog.pos;
                    cat.flee(&self.level.bounds, &dir)
                },
                _ => {},
            }
        }

        // TODO: Check win condition!
        let mut we_win = true;
        for cat in &self.cats {
            if !self.level.cat_box.in_bounds(&cat.pos) {
                we_win = false;
                break;
            }
        }

        if we_win {
            println!("YOU WON");
        }
    }

    pub fn cat_box(&self) -> &CatBox {
        &self.level.cat_box
    }
}
