use midgar::Midgar;

use cgmath::{self, Vector2, InnerSpace};
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

pub enum CatType {
    Basic,
}

pub enum CatState {
    Flee,
    Idle,
    InPen
}

pub struct Cat {
    pub pos: Vector2<f32>,
    pub cat_type: CatType,
    pub radius: f32,
    pub speed: f32,
    pub size: Vector2<f32>,
}

impl Cat {
    fn get_state(&self, dog: &Dog, cat_box: &CatBox) -> CatState {
        let dog_to_cat = self.pos - dog.pos;

        match &self.cat_type {
            _ => { },
        }

        if cat_box.in_bounds(&self.pos) {
            return CatState::InPen;
        }
        if dog_to_cat.magnitude() < self.radius {
            return CatState::Flee;
        }

        CatState::Idle
    }

    fn flee(&mut self, bounds: &Vector2<f32>, dir: &Vector2<f32>) {
        match &self.cat_type {
            _ => { },
        }

        let speed = self.speed;
        self.try_move(bounds, dir.normalize() * speed);
    }

    fn idle(&mut self, bounds: &Vector2<f32>) {

    }

    fn in_pen(&mut self, bounds: &Vector2<f32>) {
        // TODO: wander in random direction
        // self.pos = self.pos;
    }

    fn try_move(&mut self, bounds: &Vector2<f32>, change: Vector2<f32>) {
        let br = self.pos + self.size;

        if br.x + change.x > bounds.x {
            self.pos.x = bounds.x - self.size.x;
        } else if self.pos.x + change.x < 0.0 {
            self.pos.x = 0.0;
        } else {
            self.pos.x = self.pos.x + change.x;
        }

        if br.y + change.y > bounds.y {
            self.pos.y = bounds.y - self.size.y;
        } else if self.pos.y + change.y < 0.0 {
            self.pos.y = 0.0;
        } else {
            self.pos.y = self.pos.y + change.y;
        }
    }
}

pub struct CatBox {
    pub pos: Vector2<f32>,
    pub size: Vector2<f32>,
}

impl CatBox {
    pub fn in_bounds(&self, point: &Vector2<f32>) -> bool {
        let br = self.pos + self.size;
        point.x >= self.pos.x && point.x <= br.x && point.y >= br.y && point.y <= self.pos.y
    }
}

pub struct GameWorld {
    pub dog: Dog,
    pub cats: Vec<Cat>,
    pub cat_box: CatBox,
    pub bounds: Vector2<f32>,
}

impl GameWorld {
    pub fn new() -> Self {
        let cats = vec![
            Cat {
                pos: cgmath::vec2(400.0, 400.0),
                cat_type: CatType::Basic,
                radius: 20.0,
                speed: 1.0,
                size: cgmath::vec2(30.0, 30.0),
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
            bounds: cgmath::vec2(config::SCREEN_SIZE.x as f32, config::SCREEN_SIZE.y as f32),
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
            match cat.get_state(&self.dog, &self.cat_box) {
                CatState::Idle => { cat.idle(&self.bounds) },
                CatState::InPen => { cat.in_pen(&self.bounds) },
                CatState::Flee => {
                    let dir = &cat.pos - self.dog.pos;
                    cat.flee(&self.bounds, &dir)
                },
                _ => {},
            }
        }

        // TODO: Check win condition!
        let mut we_win = true;
        for cat in &self.cats {
            if !self.cat_box.in_bounds(&cat.pos) {
                we_win = false;
                break;
            }
        }

        if we_win {
            println!("YOU WON");
        }
    }
}
