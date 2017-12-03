use cgmath::{self, InnerSpace, MetricSpace, Vector2};
use rand::{self, Rng};
use rand::distributions::{IndependentSample, Range};
use config;

use entities::*;

pub const MAX_LEVEL: u32 = 5;
const BASIC_CAT_ANNOYANCE_RATE: f32 = 0.75;
const BASIC_CAT_CALMING_RATE: f32 = 0.5;
const BASIC_CAT_SPEED: f32 = 150.0;
const BASIC_CAT_RW_RADIUS: f32 = 9.0;

pub struct Level {
    pub cat_box: CatBox,
    pub num_cats: u32,
    pub bounds: Vector2<u32>,
    pub level_num: u32,
}

impl Level {
    pub fn new(level_num: u32) -> Self {
        let num_cats = Level::num_cats_for_level(level_num);

        Level {
            level_num: level_num,
            cat_box: CatBox {
                pos: cgmath::vec2(100.0, 100.0),
                size: cgmath::vec2(60.0, 60.0),
            },
            num_cats: num_cats,
            bounds: cgmath::vec2(config::GAME_SIZE.x, config::GAME_SIZE.y),
        }
    }

    pub fn next_level(&mut self) {
        let next_level_num = self.level_num + 1;
        self.level_num = next_level_num;
        self.num_cats = Level::num_cats_for_level(next_level_num);
    }

    pub fn num_cats_for_level(level_num: u32) -> u32 {
        let num = match level_num {
            1 => 1,
            2 => 3,
            3 => 5,
            4 => 10,
            5 => 20,
            _ => 1,
        };

        num
    }

    pub fn generate_cats(&self) -> Vec<Cat> {
        // Spawn cats a bit away from walls and away from the cat box.
        let cat_box_radius = 80.0;
        let range_x = Range::new(20.0, self.bounds.x as f32 - 20.0);
        let range_y = Range::new(20.0, self.bounds.y as f32 - 20.0);
        let mut rng = rand::thread_rng();

        let mut cats = Vec::new();
        for _ in 0..self.num_cats {
            let mut cat_pos = cgmath::vec2(range_x.ind_sample(&mut rng), range_y.ind_sample(&mut rng));
            // TODO: We should probably try to space out the cats from each other.
            while cat_pos.distance(self.cat_box.pos) < cat_box_radius {
                cat_pos = cgmath::vec2(range_x.ind_sample(&mut rng), range_y.ind_sample(&mut rng));
            }
            cats.push(Cat {
                pos: cat_pos,
                facing: Facing::Left, // TODO: Randomize!
                cat_type: CatType::Basic,
                radius: 70.0,
                speed: BASIC_CAT_SPEED,
                size: cgmath::vec2(30.0, 30.0),
                annoyance_total: 0.0,
                annoyance_rate: BASIC_CAT_ANNOYANCE_RATE,
                calming_rate: BASIC_CAT_CALMING_RATE,
                state: CatState::Idle,
                velocity: cgmath::vec2(rng.gen::<f32>() * 2.0 - 1.0,
                                       rng.gen::<f32>() * 2.0 - 1.0).normalize(),
                rw_radius: BASIC_CAT_RW_RADIUS,
                rw_theta: 0.0,
                jitter_origin: cat_pos,
                targeting_time: 0.0,
                dog_target: cgmath::vec2(0.0, 0.0),
                cannonballing_time: 0.0,
            });
        }
        cats
    }
}
