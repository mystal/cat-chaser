use midgar::Midgar;

use cgmath::{self, InnerSpace, Vector2, Zero};
use midgar::KeyCode;
use entities::*;
use sounds::{Sounds, AudioController};
use level::{Level, MAX_LEVEL};
use party::Party;

const MOVE_SPEED: f32 = 150.0;

#[derive(Clone, Copy, PartialEq)]
pub enum GameState {
    StartMenu,
    HowToPlay,
    Running,
    Won,
    GameOver,
}

pub struct GameWorld {
    pub game_state: GameState,
    pub level: Level,
    pub sounds: Sounds,
    pub dog: Dog,
    pub cats: Vec<Cat>,
    pub cats_scored: u32,

    pub the_party: Party,
}

impl GameWorld {
    pub fn new() -> Self {
        let level = Level::new(1);
        let mut yip_sound = Sounds::dog_yip();
        yip_sound.set_volume(3.0);
        let dog = Dog {
            pos: level.cat_box.pos,
            vel: Vector2::zero(),
            size: cgmath::Vector2::new(30.0, 30.0),
            facing: Facing::Left,
            left_key: KeyCode::Left,
            right_key: KeyCode::Right,
            up_key: KeyCode::Up,
            down_key: KeyCode::Down,
            dog_state: DogState::Chasing,
            hit_time: 0.0,
            hit_frame: 0,
            yip_sound,
            woof_sound: Sounds::dog_woof(),
        };
        let cats = level.generate_cats();

        GameWorld {
            game_state: GameState::StartMenu,
            level,
            sounds: Sounds::new(),
            dog,
            cats,
            cats_scored: 0,
            the_party: Party::new(),
        }
    }

    pub fn update(&mut self, midgar: &Midgar, dt: f32) {
        match self.game_state {
            GameState::StartMenu => self.update_start_menu(midgar, dt),
            GameState::HowToPlay => self.update_how_to_play(midgar, dt),
            GameState::Running => self.update_running(midgar, dt),
            GameState::Won => self.update_won(midgar, dt),
            GameState::GameOver => self.update_game_over(midgar, dt),
        }
    }

    fn restart(&mut self) {
        self.dog.pos = self.level.cat_box.pos;
        let cats = self.level.generate_cats();
        self.cats = cats;
        self.game_state = GameState::Running;
    }

    fn next_level(&mut self) {
        if self.level.level_num >= MAX_LEVEL {
            self.game_state = GameState::GameOver;
            return;
        }

        self.level.next_level();
        self.restart();
    }

    fn update_start_menu(&mut self, midgar: &Midgar, _dt: f32) {
        if midgar.input().was_key_pressed(KeyCode::Return) {
            self.game_state = GameState::HowToPlay;
        }
    }

    fn update_how_to_play(&mut self, midgar: &Midgar, _dt: f32) {
        if midgar.input().was_key_pressed(KeyCode::Return) {
            self.game_state = GameState::Running;
        }
    }

    fn update_game_over(&mut self, midgar: &Midgar, _dt: f32) {
        if midgar.input().was_key_pressed(KeyCode::R) {
            self.level = Level::new(1);
            self.restart();
        }
    }

    fn update_won(&mut self, midgar: &Midgar, dt: f32) {
        if midgar.input().was_key_pressed(KeyCode::N) {
            self.next_level();
        }

        self.update_running(midgar, dt);
    }

    fn update_running(&mut self, midgar: &Midgar, dt: f32) {
        if midgar.input().was_key_pressed(KeyCode::R) {
            self.restart();
            return;
        }
        if midgar.input().was_key_pressed(KeyCode::Tab) {
            self.next_level();
            return;
        }
        if midgar.input().was_key_pressed(KeyCode::Space) {
            self.dog.woof();
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
        let delta_pos = self.dog.vel * dt;
        self.dog.try_move(&self.level.bounds, delta_pos);

        self.dog.update(dt);

        let mut cats_scored = 0;
        // Cats move or run!
        for cat in &mut self.cats {
            let prev_state = cat.state.clone();
            match cat.update_state(&self.dog, &self.level.cat_box) {
                CatState::Idle => { cat.idle(&self.level.bounds, &self.level.cat_box, dt) },
                CatState::InPen => {
                    cat.in_pen(&self.level.bounds, dt);
                    cats_scored += 1;
                },
                CatState::Flee => {
                    let dir = &cat.pos - self.dog.pos;
                    cat.flee(&self.level.bounds, &dir, dt)
                },
                CatState::Jittering => {
                    cat.jitter(dt, &self.dog)
                }
                CatState::Cannonballing => {
                    cat.cannonball(&self.level.bounds, dt, &mut self.dog)
                }
            }

            if cat.state == CatState::Idle || cat.state == CatState::InPen || cat.state == CatState::Flee {
                // Basic meow
                if cat.meow_time >= cat.meow_interval {
                    cat.meow();
                }
                cat.meow_time += dt;
            } else if prev_state != cat.state {
                // Angry meow
                if cat.state == CatState::Jittering || cat.state == CatState::Cannonballing {
                    cat.meow();
                }
            }

            if cat.velocity.x != 0.0 {
                cat.facing = if cat.velocity.x > 0.0 {
                    Facing::Right
                } else {
                    Facing::Left
                };
            }
        }

        self.cats_scored = cats_scored;

        if self.game_state != GameState::Won {
            // Check win condition!
            if self.cats_scored == self.level.num_cats {
                self.game_state = GameState::Won;
            }
        }
    }

    pub fn cat_box(&self) -> &CatBox {
        &self.level.cat_box
    }
}
