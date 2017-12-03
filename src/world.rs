use midgar::Midgar;

use cgmath::{self, InnerSpace, Vector2, Zero};
use midgar::{self, KeyCode};

use config;
use entities::*;
use level::{Level, MAX_LEVEL};


const MOVE_SPEED: f32 = 150.0;

#[derive(Clone, Copy, PartialEq)]
pub enum GameState {
    Running,
    Won,
    GameOver,
}

pub struct GameWorld {
    pub game_state: GameState,
    pub level: Level,
    pub dog: Dog,
    pub cats: Vec<Cat>,
}

impl GameWorld {
    pub fn new() -> Self {
        let level = Level::new(1);
        let dog = Dog {
            pos: level.cat_box.pos,
            vel: Vector2::zero(),
            facing: Facing::Left,
            left_key: KeyCode::Left,
            right_key: KeyCode::Right,
            up_key: KeyCode::Up,
            down_key: KeyCode::Down,
        };
        let cats = level.generate_cats();

        GameWorld {
            game_state: GameState::Running,
            level,
            dog,
            cats,
        }
    }

    pub fn update(&mut self, midgar: &Midgar, dt: f32) {
        match self.game_state {
            GameState::Running => self.update_running(midgar, dt),
            GameState::Won => self.update_won(midgar, dt),
            GameState::GameOver => self.update_game_over(midgar, dt),
            _ => {},
        }
    }

    fn restart(&mut self) {
        self.dog.pos = self.level.cat_box.pos;
        let cats = self.level.generate_cats();
        self.cats = cats;
        self.game_state = GameState::Running;
    }

    fn update_game_over(&mut self, midgar: &Midgar, dt: f32) {
        if midgar.input().was_key_pressed(KeyCode::R) {
            self.level = Level::new(1);
            self.restart();
        }
    }

    fn update_won(&mut self, midgar: &Midgar, dt: f32) {
        if midgar.input().was_key_pressed(KeyCode::N) {
            if self.level.level_num > MAX_LEVEL {
                self.game_state = GameState::GameOver;
                return;
            }

            self.level.next_level();
            self.restart();
        }

        self.update_running(midgar, dt);
    }

    fn update_running(&mut self, midgar: &Midgar, dt: f32) {
        if midgar.input().was_key_pressed(KeyCode::R) {
            self.restart();
        }
        // TODO: consider moving this into a poll input method
        // TODO: Clamp dog to level bounds.
        let mut dir = Vector2::zero();
        if midgar.input().is_key_held(self.dog.left_key) && !midgar.input().is_key_held(self.dog.right_key) {
            dir.x -= 1.0;
        }
        if midgar.input().is_key_held(self.dog.right_key) && !midgar.input().is_key_held(self.dog.left_key) {
            dir.x += 1.0;
        }
        if midgar.input().is_key_held(self.dog.up_key) && !midgar.input().is_key_held(self.dog.down_key) {
            dir.y -= 1.0;
        }
        if midgar.input().is_key_held(self.dog.down_key) && !midgar.input().is_key_held(self.dog.up_key) {
            dir.y += 1.0;
        }
        if !dir.is_zero() {
            dir = dir.normalize();
        }
        if dir.x != 0.0 {
            self.dog.facing = if dir.x > 0.0 {
                Facing::Right
            } else {
                Facing::Left
            };
        }
        self.dog.vel = dir * MOVE_SPEED;
        self.dog.pos += self.dog.vel * dt;

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
            if cat.velocity.x != 0.0 {
                cat.facing = if cat.velocity.x > 0.0 {
                    Facing::Right
                } else {
                    Facing::Left
                };
            }
        }

        if self.game_state != GameState::Won {
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
                self.game_state = GameState::Won;
            }
        }
    }

    pub fn cat_box(&self) -> &CatBox {
        &self.level.cat_box
    }
}
