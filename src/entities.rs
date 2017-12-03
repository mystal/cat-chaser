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

#[derive(Clone, Copy)]
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
        let half_size = self.size * 0.5;
        let (min_x, max_x) = (half_size.x, bounds.x as f32 - half_size.x);
        let (min_y, may_y) = (half_size.y, bounds.y as f32 - half_size.y);

        // Clamp new_pos to min and max values.
        let mut new_pos = self.pos + change;
        new_pos.x = if new_pos.x < min_x {
            min_x
        } else if new_pos.x > max_x {
            max_x
        } else {
            new_pos.x
        };
        new_pos.y = if new_pos.y < min_y {
            min_y
        } else if new_pos.y > may_y {
            may_y
        } else {
            new_pos.y
        };

        self.pos = new_pos;
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
