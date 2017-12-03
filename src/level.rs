use cgmath::{self, InnerSpace, MetricSpace, Vector2};
use rand::{self, Rng};
use rand::distributions::{IndependentSample, Range};
use config;

use entities::*;

pub const MAX_LEVEL: u32 = 5;

pub struct Level {
    pub cat_box: CatBox,
    pub num_cats: u32,
    pub bounds: Vector2<u32>,
    pub level_num: u32,
    pub cats: (u32, u32, u32),
}

impl Level {
    pub fn new(level_num: u32) -> Self {
        let num_cats = Level::num_cats_for_level(level_num);
        let cats = Level::cats_for_level(level_num);

        Level {
            level_num: level_num,
            cat_box: CatBox {
                pos: cgmath::vec2(100.0, 100.0),
                size: cgmath::vec2(60.0, 60.0),
            },
            num_cats: num_cats,
            bounds: cgmath::vec2(config::GAME_SIZE.x, config::GAME_SIZE.y),
            cats: cats,
            
        }
    }

    pub fn next_level(&mut self) {
        let next_level_num = self.level_num + 1;
        self.level_num = next_level_num;
        self.num_cats = Level::num_cats_for_level(next_level_num);
        self.cats = Level::cats_for_level(next_level_num);
    }

    pub fn cats_for_level(level_num: u32) -> (u32, u32, u32) {
        match level_num {
            1 => (1, 0, 0),
            2 => (3, 0, 0),
            3 => (3, 2, 0),
            4 => (5, 3, 2),
            5 => (10, 6, 4),
            _ => (1, 0, 0)
        }
    }

    pub fn num_cats_for_level(level_num: u32) -> u32 {
        let (x, y, z) = Level::cats_for_level(level_num);
        return x + y + z;
    }

    pub fn generate_cats(&self) -> Vec<Cat> {
        // Spawn cats a bit away from walls and away from the cat box.
        let cat_box_radius = 80.0;
        let range_x = Range::new(20.0, self.bounds.x as f32 - 20.0);
        let range_y = Range::new(20.0, self.bounds.y as f32 - 20.0);
        let mut rng = rand::thread_rng();

        let mut cats = Vec::new();
        let mut basic_cats: u32 = 0;
        let mut kittens: u32 = 0;
        let mut fat_cats: u32 = 0;

        let (total_basic, total_kittens, _total_fat) = self.cats;

        for _ in 0..self.num_cats {
            let mut cat_pos = cgmath::vec2(range_x.ind_sample(&mut rng), range_y.ind_sample(&mut rng));
            // TODO: We should probably try to space out the cats from each other.
            while cat_pos.distance(self.cat_box.pos) < cat_box_radius {
                cat_pos = cgmath::vec2(range_x.ind_sample(&mut rng), range_y.ind_sample(&mut rng));
            }
            let vel = cgmath::vec2(rng.gen::<f32>() * 2.0 - 1.0,
                                   rng.gen::<f32>() * 2.0 - 1.0).normalize();

            let cat = if basic_cats < total_basic {
                basic_cats += 1;
                Cat::new_basic_cat(cat_pos, vel)
            } else if kittens < total_kittens {
                kittens += 1;
                Cat::new_kitten(cat_pos, vel)
            } else {
                fat_cats += 1;
                Cat::new_fat_cat(cat_pos, vel)
            };
            cats.push(cat);
        }
        cats
    }
}
