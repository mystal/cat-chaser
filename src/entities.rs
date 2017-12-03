use cgmath::{self, Vector2, InnerSpace};
use midgar::{self, KeyCode};

const ANNOYANCE_THRESHOLD: f32 = 100.0;

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

#[derive(Clone, Copy)]
pub enum CatState {
    Flee,
    Idle,
    InPen,
    Annoyed,
}

pub struct Cat {
    pub pos: Vector2<f32>,
    pub cat_type: CatType,
    pub radius: f32,
    pub speed: f32,
    pub size: Vector2<f32>,
    pub annoyance_total: f32,
    pub annoyance_rate: f32,
    pub calming_rate: f32,
    pub state: CatState,
}

impl Cat {
    pub fn update_state(&mut self, dog: &Dog, cat_box: &CatBox) -> CatState {
        let dog_to_cat = self.pos - dog.pos;

        match &self.cat_type {
            _ => { },
        }

        self.state = if cat_box.in_bounds(&self.pos) {
            CatState::InPen
        } else if dog_to_cat.magnitude() < self.radius {
            CatState::Flee
        } else {
            CatState::Idle
        };

        self.state
    }

    pub fn flee(&mut self, bounds: &Vector2<u32>, dir: &Vector2<f32>, dt: f32) {
        match &self.cat_type {
            _ => { },
        }

        let speed = self.speed * dt;
        self.try_move(bounds, dir.normalize() * speed);
        self.increase_annoyance(dt);
    }

    pub fn idle(&mut self, bounds: &Vector2<u32>, dt: f32) {
        self.decrease_annoyance(dt);
    }

    pub fn in_pen(&mut self, bounds: &Vector2<u32>, dt: f32) {
        // TODO: wander in random direction
        // self.pos = self.pos;
        self.decrease_annoyance(dt);
    }

    pub fn annoyed(&mut self, bounds: &Vector2<u32>, dt: f32) {
        println!("ANNOYED!");
    }

    fn try_move(&mut self, bounds: &Vector2<u32>, change: Vector2<f32>) {
        let bottom_right = self.pos + self.size;
        let bound_x = bounds.x as f32;
        let bound_y = bounds.y as f32;

        if bottom_right.x + change.x > bound_x {
            self.pos.x = bound_x - self.size.x;
        } else if self.pos.x + change.x < 0.0 {
            self.pos.x = 0.0;
        } else {
            self.pos.x = self.pos.x + change.x;
        }

        if bottom_right.y + change.y > bound_y {
            self.pos.y = bound_y - self.size.y;
        } else if self.pos.y + change.y < 0.0 {
            self.pos.y = 0.0;
        } else {
            self.pos.y = self.pos.y + change.y;
        }
    }

    fn decrease_annoyance(&mut self, dt: f32) {
        self.annoyance_total -= self.calming_rate * dt;
    }

    fn increase_annoyance(&mut self, dt: f32) {
        self.annoyance_total += self.annoyance_rate * dt;
    }
}

pub struct CatBox {
    pub pos: Vector2<f32>,
    pub size: Vector2<f32>,
}

impl CatBox {
    pub fn in_bounds(&self, point: &Vector2<f32>) -> bool {
        let half_size = self.size * 0.5;
        let top_left = self.pos - half_size;
        let bottom_right = self.pos + half_size;
        point.x >= top_left.x && point.x <= bottom_right.x &&
            point.y >= top_left.y && point.y <= bottom_right.y
    }
}

pub struct Camera {
    pub pos: Vector2<f32>,
    pub bounds: Vector2<f32>,
    pub zoom: i32,
}
