use cgmath::{self, Vector2, InnerSpace};
use midgar::{self, KeyCode};

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
    pub fn get_state(&self, dog: &Dog, cat_box: &CatBox) -> CatState {
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

    pub fn flee(&mut self, bounds: &Vector2<u32>, dir: &Vector2<f32>) {
        match &self.cat_type {
            _ => { },
        }

        let speed = self.speed;
        self.try_move(bounds, dir.normalize() * speed);
    }

    pub fn idle(&mut self, bounds: &Vector2<u32>) {

    }

    pub fn in_pen(&mut self, bounds: &Vector2<u32>) {
        // TODO: wander in random direction
        // self.pos = self.pos;
    }

    fn try_move(&mut self, bounds: &Vector2<u32>, change: Vector2<f32>) {
        let br = self.pos + self.size;
        let bound_x = bounds.x as f32;
        let bound_y = bounds.y as f32;

        if br.x + change.x > bound_x {
            self.pos.x = bound_x - self.size.x;
        } else if self.pos.x + change.x < 0.0 {
            self.pos.x = 0.0;
        } else {
            self.pos.x = self.pos.x + change.x;
        }

        if br.y + change.y > bound_y {
            self.pos.y = bound_y - self.size.y;
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

pub struct Camera {
    pub pos: Vector2<f32>,
    pub bounds: Vector2<f32>,
    pub zoom: i32,
}
