use cgmath::{self, Vector2, InnerSpace};
use midgar::{self, KeyCode};
use rand;
use rand::distributions::{IndependentSample, Range};

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Facing {
    Left,
    Right,
}

const ANNOYANCE_THRESHOLD: f32 = 1.0;
const CANNONBALL_COUNTDOWN: f32 = 1.0;

pub struct Dog {
    pub pos: Vector2<f32>,
    pub vel: Vector2<f32>,
    pub facing: Facing,

    pub left_key: KeyCode, // TODO: consider breaking this out into control struct
    pub right_key: KeyCode,
    pub up_key: KeyCode,
    pub down_key: KeyCode,
}

pub enum CatType {
    Basic,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CatState {
    Flee,
    Idle,
    InPen,
    Jittering,
    Cannonballing,
    Annoyed,
}

pub struct Cat {
    pub pos: Vector2<f32>,
    pub facing: Facing,
    pub cat_type: CatType,
    pub radius: f32,
    pub speed: f32,
    pub size: Vector2<f32>,
    pub annoyance_total: f32,
    pub annoyance_rate: f32,
    pub calming_rate: f32,
    pub state: CatState,
    pub velocity: Vector2<f32>,
    pub rw_radius: f32, // for random walk in idle
    pub rw_theta: f32, // for random walk in idle
    pub jitter_origin: Vector2<f32>,
    pub cannonball_time: f32,
    pub dog_target: Vector2<f32>,
}

impl Cat {
    fn start_cannonballing(&mut self, dog_pos: Vector2<f32>) {
        self.dog_target = (dog_pos - self.pos).normalize();
    }

    fn start_jitter(&mut self) {
        self.jitter_origin = self.pos;
        self.cannonball_time = CANNONBALL_COUNTDOWN;
    }

    pub fn normalized_jitter(&self) -> f32 {
        return self.annoyance_total / ANNOYANCE_THRESHOLD
    }

    pub fn jitter(&mut self, bounds: &Vector2<u32>, dt: f32, dog: &Dog) {
        let mut rng = rand::thread_rng();
        let x_range = Range::new(-1.0, 1.0);
        let y_range = Range::new(-1.0, 1.0);

        let x = x_range.ind_sample(&mut rng);
        let y = y_range.ind_sample(&mut rng);

        self.pos.x = self.jitter_origin.x + x;
        self.pos.y = self.jitter_origin.y + y;

        self.cannonball_countdown(dt, dog);
    }

    pub fn update_state(&mut self, dog: &Dog, cat_box: &CatBox) -> CatState {
        let dog_to_cat = self.pos - dog.pos;

        match &self.cat_type {
            _ => { },
        }

        self.state = if self.state == CatState::Cannonballing {
            CatState::Cannonballing
        } else if self.state == CatState::Jittering && self.cannonball_time <= 0.0 {
            CatState::Cannonballing
        } else if self.state != CatState::Cannonballing && self.annoyance_total >= ANNOYANCE_THRESHOLD {
            CatState::Jittering
        } else if cat_box.in_bounds(&self.pos) {
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

        let speed = self.speed;
        self.velocity = dir.normalize() * speed;
        self.try_move(bounds, dir.normalize() * speed * dt);
        self.increase_annoyance(dt);
    }

    pub fn idle(&mut self, bounds: &Vector2<u32>, dt: f32) {
        let range_theta = Range::new(-0.3, 0.3);
        let mut rng = rand::thread_rng();
        // random update rw_theta
        self.rw_theta = self.rw_theta + range_theta.ind_sample(&mut rng);

        // 'circle' vector by (velocity rotated by theta).normalized * rw_radius
        let t = self.rw_theta;
        let mut v = cgmath::vec2(1.0, 0.0);
        let mut circle_vector = cgmath::vec2(t.cos()*v.x - t.sin()*v.y, t.sin()*v.x + t.cos()*v.y);

        if circle_vector.magnitude() != 0.0 {
            circle_vector = circle_vector.normalize() * self.rw_radius;
        }

        // velocity = (velocity + 'circle' vector).normalized * speed

        if (self.velocity + circle_vector).magnitude() != 0.0 {
            self.velocity = (self.velocity + circle_vector).normalize() * self.speed / 3.0;
        }
        v = self.velocity;
        self.try_move(bounds, v * dt);
        self.decrease_annoyance(dt)
    }

    pub fn in_pen(&mut self, bounds: &Vector2<u32>, dt: f32) {
        // TODO: wander in random direction
        // self.pos = self.pos;
        self.decrease_annoyance(dt);
    }

    pub fn cannonball(&mut self, bounds: &Vector2<u32>, dt: f32) {
        let target = self.dog_target;
        self.try_move(bounds, target * 240.0 * dt);
    }

    pub fn annoyed(&mut self, bounds: &Vector2<u32>, dt: f32) {       
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

    fn decrease_annoyance(&mut self, dt: f32) {
        self.annoyance_total -= self.calming_rate * dt;
        if self.annoyance_total < 0.0 {
            self.annoyance_total = 0.0;
        }
    }

    fn increase_annoyance(&mut self, dt: f32) {
        self.annoyance_total += self.annoyance_rate * dt;
        if (self.annoyance_total >= ANNOYANCE_THRESHOLD) {
            self.start_jitter();
        }
    }

    fn cannonball_countdown(&mut self, dt: f32, dog: &Dog) {
        self.cannonball_time -= dt;
        if self.cannonball_time <= 0.0 {
            self.start_cannonballing(dog.pos);
        }
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
