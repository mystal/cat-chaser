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
    pub we_win: bool,
}

impl GameWorld {
    pub fn new() -> Self {
        let level = Level {
            cat_box: CatBox {
                pos: cgmath::vec2(100.0, 100.0),
                size: cgmath::vec2(60.0, 60.0),
            },
            num_cats: 10,
            bounds: cgmath::vec2(config::GAME_SIZE.x, config::GAME_SIZE.y),
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
            we_win: false,
        }
    }

    pub fn update(&mut self, midgar: &Midgar, dt: f32) {
        // TODO: consider moving this into a poll input method
        // TODO: Clamp dog to level bounds.
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

        // Cats move or run!
        for cat in &mut self.cats {
            match cat.update_state(&self.dog, &self.level.cat_box) {
                CatState::Idle => { cat.idle(&self.level.bounds, dt) },
                CatState::InPen => { cat.in_pen(&self.level.bounds, dt) },
                CatState::Flee => {
                    let dir = &cat.pos - self.dog.pos;
                    cat.flee(&self.level.bounds, &dir, dt)
                },
                CatState::Jittering => {
                    cat.jitter(&self.level.bounds, dt)
                }
                CatState::Annoyed => {
                    cat.annoyed(&self.level.bounds, dt)
                }
                _ => {},
            }
        }

        if !self.we_win {
            // Check win condition!
            let mut we_win = true;
            for cat in &self.cats {
                if !self.level.cat_box.in_bounds(&cat.pos) {
                    we_win = false;
                    break;
                }
            }

            if we_win {
                println!("YOU WON");
                self.we_win = true;
            }
        }
    }

    pub fn cat_box(&self) -> &CatBox {
        &self.level.cat_box
    }
}
