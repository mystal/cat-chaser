use std::rc::Rc;

use cgmath;
use midgar::{Midgar, Surface};
use midgar::graphics::shape::ShapeRenderer;
use midgar::graphics::sprite::{DrawTexture, MagnifySamplerFilter, Sprite, SpriteDrawParams, SpriteRenderer};

use config;
use world::*;

pub struct GameRenderer {
    sprite: SpriteRenderer,
    shape: ShapeRenderer,
}

impl GameRenderer {
    pub fn new(midgar: &Midgar) -> Self {
        let projection = cgmath::ortho(0.0, config::SCREEN_SIZE.x as f32,
                                       config::SCREEN_SIZE.y as f32, 0.0,
                                       -1.0, 1.0);

        GameRenderer {
            sprite: SpriteRenderer::new(midgar.graphics().display(), projection),
            shape: ShapeRenderer::new(midgar.graphics().display(), projection),
        }
    }

    pub fn render(&mut self, midgar: &Midgar, dt: f32, world: &GameWorld) {
        // Get framebuffer target.
        let mut target = midgar.graphics().display().draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        // Draw cat box.
        let white = [1.0, 1.0, 1.0];
        let grey = [0.5, 0.5, 0.5];
        let black = [0.0, 0.0, 0.0];
        let blue_violet = [138.0 / 255.0, 43.0 / 255.0, 226.0 / 255.0];
        self.shape.draw_filled_rect(world.cat_box.pos.x, world.cat_box.pos.y,
                                    world.cat_box.size.x, world.cat_box.size.y,
                                    white, &mut target);
        self.shape.draw_filled_rect(world.cat_box.pos.x + 2.0, world.cat_box.pos.y + 2.0,
                                    world.cat_box.size.x - 4.0, world.cat_box.size.y - 4.0,
                                    black, &mut target);

        // Draw cats!
        for cat in &world.cats {
            self.shape.draw_filled_rect(cat.pos.x, cat.pos.y,
                                        30.0, 30.0,
                                        grey, &mut target);
        }

        // Draw dog, woof.
        self.shape.draw_filled_rect(world.dog.pos.x, world.dog.pos.y,
                                    50.0, 50.0,
                                    blue_violet, &mut target);

        target.finish().unwrap();
    }
}
