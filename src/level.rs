use cgmath::{self, MetricSpace, Vector2};
use rand;
use rand::distributions::{IndependentSample, Range};

use entities::*;

pub struct Level {
    pub cat_box: CatBox,
    pub num_cats: u32,
    pub bounds: Vector2<u32>,
}

impl Level {
    pub fn generate_cats(&self) -> Vec<Cat> {
        let cat_box_radius = 150.0;
        let range_x = Range::new(0.0, self.bounds.x as f32);
        let range_y = Range::new(0.0, self.bounds.y as f32);
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
                cat_type: CatType::Basic,
                radius: 100.0,
                speed: 2.0,
                size: cgmath::vec2(30.0, 30.0),
            });
        }
        cats
    }
}