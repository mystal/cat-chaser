use std::rc::Rc;

use cgmath::{self, Matrix4};
use cgmath::prelude::*;
use entities::Camera;
use midgar::{Midgar, Surface};
use midgar::graphics::shape::ShapeRenderer;
use midgar::graphics::sprite::{DrawTexture, MagnifySamplerFilter, Sprite, SpriteDrawParams, SpriteRenderer};

use config;
use world::*;

pub struct GameRenderer {
    projection: Matrix4<f32>,
    sprite: SpriteRenderer,
    shape: ShapeRenderer,
}

impl GameRenderer {
    pub fn new(midgar: &Midgar) -> Self {
        let projection = cgmath::ortho(-(config::SCREEN_SIZE.x as f32 / 2.0), config::SCREEN_SIZE.x as f32 / 2.0,
                                       config::SCREEN_SIZE.y as f32 / 2.0, -(config::SCREEN_SIZE.y as f32 / 2.0),
                                       -1.0, 1.0);

        GameRenderer {
            projection: projection,
            sprite: SpriteRenderer::new(midgar.graphics().display(), projection),
            shape: ShapeRenderer::new(midgar.graphics().display(), projection),
        }
    }

    pub fn render(&mut self, midgar: &Midgar, dt: f32, world: &GameWorld, camera: &Camera) {
        // set the camera view
        let camera_pos = camera.pos.extend(0.0);
        let view = cgmath::Matrix4::look_at(cgmath::Point3::from_vec(camera_pos),
                                            cgmath::Point3::new(0.0, 0.0, -1.0) + camera_pos,
                                            cgmath::vec3(0.0, 1.0, 0.0));

        let combined = self.projection * view;
        self.sprite.set_projection_matrix(combined);
        self.shape.set_projection_matrix(combined);

        // Get framebuffer target.
        let mut target = midgar.graphics().display().draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        // Draw cat box.
        let white = [1.0, 1.0, 1.0];
        let grey = [0.5, 0.5, 0.5];
        let black = [0.0, 0.0, 0.0];
        let blue_violet = [138.0 / 255.0, 43.0 / 255.0, 226.0 / 255.0];
        self.shape.draw_filled_rect(world.cat_box().pos.x, world.cat_box().pos.y,
                                    world.cat_box().size.x, world.cat_box().size.y,
                                    white, &mut target);
        self.shape.draw_filled_rect(world.cat_box().pos.x, world.cat_box().pos.y,
                                    world.cat_box().size.x - 4.0, world.cat_box().size.y - 4.0,
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
