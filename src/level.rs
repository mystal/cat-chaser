use cgmath::{self, MetricSpace, Vector2};
use rand;
use rand::distributions::{IndependentSample, Range};
use config;

use entities::*;

pub const MAX_LEVEL: u32 = 4;

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
            2 => 5,
            3 => 10,
            4 => 20,
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
                speed: 2.0,
                size: cgmath::vec2(30.0, 30.0),
                state: CatState::Idle,
                velocity: cgmath::vec2(1.0, 0.0),
                rw_radius: 0.1,
                rw_theta: 0.0,
            });
        }
        cats
    }
}
