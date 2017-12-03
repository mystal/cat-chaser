use std::rc::Rc;

use cgmath::{self, Matrix4};
use cgmath::prelude::*;
use entities::{Camera, DogState};
use midgar::{Midgar, Surface};
use midgar::graphics::animation::{Animation, PlayMode};
use midgar::graphics::shape::ShapeRenderer;
use midgar::graphics::text::{self, Font, TextRenderer};
use midgar::graphics::sprite::{DrawTexture, MagnifySamplerFilter, SamplerWrapFunction, SpriteDrawParams, SpriteRenderer};
use midgar::graphics::texture::TextureRegion;

use config;
use entities::Facing;
use world::*;

pub struct GameRenderer<'a> {
    projection: Matrix4<f32>,
    sprite: SpriteRenderer,
    shape: ShapeRenderer,
    text: TextRenderer,

    start_menu: TextureRegion,
    how_to_play: TextureRegion,

    background: TextureRegion,
    cat_box: TextureRegion,
    basic_cat_walk_animation: Animation,
    basic_cat_idle_animation: Animation,
    basic_cat_walk_time: f32,
    wizard_dog_idle_animation: Animation,
    wizard_dog_idle_time: f32,
    wizard_dog_run_animation: Animation,
    // TODO: Move this to Dog to start the animation at the right time.
    wizard_dog_run_time: f32,

    font: Font<'a>,

    game_time: f32,
}

impl<'a> GameRenderer<'a> {
    pub fn new(midgar: &Midgar) -> Self {
        // Load textures.
        let start_menu = {
            let texture = Rc::new(midgar.graphics().load_texture("assets/start_menu_background.png", false));
            TextureRegion::new(texture)
        };
        let how_to_play = {
            let texture = Rc::new(midgar.graphics().load_texture("assets/how_to_play.png", false));
            TextureRegion::new(texture)
        };
        let background = {
            let texture = Rc::new(midgar.graphics().load_texture("assets/hardwood_floor.png", false));
            TextureRegion::with_sub_field(texture, (0, 0), (config::SCREEN_SIZE.x, config::SCREEN_SIZE.y))
        };
        let cat_box = {
            let texture = Rc::new(midgar.graphics().load_texture("assets/cat_box.png", false));
            TextureRegion::new(texture)
        };
        let (basic_cat_walk, basic_cat_walk_alt) = {
            let texture = Rc::new(midgar.graphics().load_texture("assets/basic_cat_walk.png", false));
            (TextureRegion::with_sub_field(texture.clone(), (0, 0), (32, 32)),
             TextureRegion::with_sub_field(texture.clone(), (32, 0), (32, 32)))
        };
        let mut basic_cat_walk_animation = Animation::new(0.2, &[basic_cat_walk.clone(), basic_cat_walk_alt.clone()])
            .unwrap();
        basic_cat_walk_animation.play_mode = PlayMode::Loop;
        let basic_cat_idle = {
            let texture = Rc::new(midgar.graphics().load_texture("assets/basic_cat_idle.png", false));
            TextureRegion::split(texture, (32, 32))
        };
        let mut basic_cat_idle_animation = Animation::new(0.2, &basic_cat_idle)
            .unwrap();
        basic_cat_idle_animation.play_mode = PlayMode::Loop;
        let wizard_dog_idle = {
            let texture = Rc::new(midgar.graphics().load_texture("assets/wizard_dog_idle.png", false));
            TextureRegion::split(texture, (32, 32))
        };
        let mut wizard_dog_idle_animation = Animation::new(0.2, &wizard_dog_idle)
            .unwrap();
        wizard_dog_idle_animation.play_mode = PlayMode::Loop;
        let wizard_dog_run = {
            let texture = Rc::new(midgar.graphics().load_texture("assets/wizard_dog_run.png", false));
            TextureRegion::split(texture, (32, 32))
        };
        let mut wizard_dog_run_animation = Animation::new(0.1, &wizard_dog_run)
            .unwrap();
        wizard_dog_run_animation.play_mode = PlayMode::Loop;

        let projection = cgmath::ortho(-(config::GAME_SIZE.x as f32 / 2.0), config::GAME_SIZE.x as f32 / 2.0,
                                       config::GAME_SIZE.y as f32 / 2.0, -(config::GAME_SIZE.y as f32 / 2.0),
                                       -1.0, 1.0);

        GameRenderer {
            projection: projection,
            sprite: SpriteRenderer::new(midgar.graphics().display(), projection),
            shape: ShapeRenderer::new(midgar.graphics().display(), projection),
            text: TextRenderer::new(midgar.graphics().display()),

            start_menu,
            how_to_play,

            background,
            cat_box,
            basic_cat_walk_animation,
            basic_cat_idle_animation,
            basic_cat_walk_time: 0.0,
            wizard_dog_idle_animation,
            wizard_dog_idle_time: 0.0,
            wizard_dog_run_animation,
            wizard_dog_run_time: 0.0,

            font: text::load_font_from_path("assets/fonts/Kenney Pixel.ttf"),

            game_time: 0.0,
        }
    }

    pub fn render(&mut self, midgar: &Midgar, dt: f32, world: &GameWorld, camera: &Camera) {
        self.game_time += dt;

        // Get framebuffer target.
        let mut target = midgar.graphics().display().draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        let draw_params = SpriteDrawParams::new()
            .magnify_filter(MagnifySamplerFilter::Nearest)
            .alpha(true);

        match world.game_state {
            GameState::StartMenu => {
                let projection = cgmath::ortho(0.0, config::SCREEN_SIZE.x as f32,
                                               config::SCREEN_SIZE.y as f32, 0.0,
                                               -1.0, 1.0);
                self.sprite.set_projection_matrix(projection);
                // Draw start menu splash screen!
                self.sprite.draw(&self.start_menu.draw(config::SCREEN_SIZE.x as f32 / 2.0, config::SCREEN_SIZE.y as f32 / 2.0),
                                 draw_params, &mut target);
                // Draw blinking text!
                if self.game_time.fract() < 0.5 {
                    self.text.draw_text("Press Enter to play!", &self.font, [0.0, 0.0, 0.0],
                                        40, 452.0, 542.0, 500, &projection, &mut target);
                    self.text.draw_text("Press Enter to play!", &self.font, [1.0, 1.0, 1.0],
                                        40, 450.0, 540.0, 500, &projection, &mut target);
                }
            },
            GameState::HowToPlay => {
                let projection = cgmath::ortho(0.0, config::SCREEN_SIZE.x as f32,
                                               config::SCREEN_SIZE.y as f32, 0.0,
                                               -1.0, 1.0);
                self.sprite.set_projection_matrix(projection);
                // Draw how to play splash screen!
                self.sprite.draw(&self.how_to_play.draw(config::SCREEN_SIZE.x as f32 / 2.0, config::SCREEN_SIZE.y as f32 / 2.0),
                                 draw_params, &mut target);
                // Draw corgi idle animation
                self.wizard_dog_idle_time += dt;
                let mut sprite = self.wizard_dog_idle_animation.current_key_frame(self.wizard_dog_idle_time)
                    .draw(670.0, 50.0);
                sprite.set_scale(cgmath::vec2(4.0, 4.0));
                self.sprite.draw(&sprite, draw_params, &mut target);
                // Draw cat animations
                sprite = self.basic_cat_idle_animation.current_key_frame(self.game_time)
                    .draw(380.0, 340.0);
                sprite.set_scale(cgmath::vec2(3.0, 3.0));
                self.sprite.draw(&sprite, draw_params, &mut target);

                // Draw blinking text!
                if self.game_time.fract() < 0.5 {
                    self.text.draw_text("Press Enter to play!", &self.font, [0.0, 0.0, 0.0],
                                        40, 452.0, 542.0, 500, &projection, &mut target);
                    self.text.draw_text("Press Enter to play!", &self.font, [1.0, 1.0, 1.0],
                                        40, 450.0, 540.0, 500, &projection, &mut target);
                }
            },
            GameState::Running | GameState::Won | GameState::GameOver => {
                self.draw_world(dt, world, camera, &mut target);
                self.draw_ui(dt, world, &mut target);
            },
        }

        target.finish().unwrap();
    }

    fn draw_world<S: Surface>(&mut self, dt: f32, world: &GameWorld, camera: &Camera, target: &mut S) {
        // set the camera view
        let camera_pos = camera.pos.extend(0.0);
        let view = cgmath::Matrix4::look_at(cgmath::Point3::from_vec(camera_pos),
                                            cgmath::Point3::new(0.0, 0.0, -1.0) + camera_pos,
                                            cgmath::vec3(0.0, 1.0, 0.0));

        let combined = self.projection * view;
        self.sprite.set_projection_matrix(combined);
        self.shape.set_projection_matrix(combined);

        let draw_params = SpriteDrawParams::new()
            .magnify_filter(MagnifySamplerFilter::Nearest)
            .alpha(true);

        // Background
        let pos = self.background.size();
        let mut sprite = self.background.draw(pos.x as f32 / 2.0, pos.y as f32 / 2.0);
        sprite.set_scale(cgmath::vec2(2.0, 2.0));
        self.sprite.draw(&sprite,
                        SpriteDrawParams::new()
                            .magnify_filter(MagnifySamplerFilter::Nearest)
                            .alpha(true)
                            .wrap_function(SamplerWrapFunction::Repeat),
                        target);

        // Draw cat box.
        self.sprite.draw(&self.cat_box.draw(world.cat_box().pos.x, world.cat_box().pos.y),
                         draw_params, target);

        // Draw cats!
        self.basic_cat_walk_time += dt;
        for cat in &world.cats {
            let mut sprite = self.basic_cat_walk_animation.current_key_frame(self.basic_cat_walk_time)
                .draw(cat.pos.x, cat.pos.y);
            sprite.set_flip_x(cat.facing == Facing::Right);
            sprite.set_color(cgmath::Vector3::new(1.0, 1.0 - cat.normalized_jitter(), 1.0 - cat.normalized_jitter()));
            self.sprite.draw(&sprite, draw_params, target);
        }

        // Draw dog, woof.
        self.wizard_dog_idle_time += dt;
        self.wizard_dog_run_time += dt;

        match world.dog.dog_state {
            DogState::Chasing | DogState::Blinking(true) => {
                let mut sprite = if world.dog.vel.is_zero() {
                    self.wizard_dog_idle_animation.current_key_frame(self.wizard_dog_idle_time)
                        .draw(world.dog.pos.x, world.dog.pos.y)
                } else {
                    self.wizard_dog_run_animation.current_key_frame(self.wizard_dog_run_time)
                        .draw(world.dog.pos.x, world.dog.pos.y)
                };
                sprite.set_flip_x(world.dog.facing == Facing::Right);
                self.sprite.draw(&sprite, draw_params, target);
            }
            DogState::Blinking(false) => {}
        }
    }


    fn draw_ui<S: Surface>(&mut self, _dt: f32, world: &GameWorld, target: &mut S) {
        let projection = cgmath::ortho(0.0, config::SCREEN_SIZE.x as f32,
                                       config::SCREEN_SIZE.y as f32, 0.0,
                                       -1.0, 1.0);
        // Draw score!
        let score_text = format!("{:02}/{:02}", world.cats_scored, world.level.num_cats);
        self.text.draw_text(&score_text, &self.font, [0.0, 0.0, 0.0],
                            40, 702.0, 7.0, 800, &projection, target);
        self.text.draw_text(&score_text, &self.font, [1.0, 1.0, 1.0],
                            40, 700.0, 5.0, 800, &projection, target);
        match world.game_state {
            GameState::Running => {
            },
            GameState::Won => {
                // TODO: Draw won text!
                let text = "Cats corralled!\nPress N to start the next level";
                self.text.draw_text(text, &self.font, [0.0, 0.0, 0.0],
                                    40, 252.0, 502.0, 800, &projection, target);
                self.text.draw_text(text, &self.font, [1.0, 1.0, 1.0],
                                    40, 250.0, 500.0, 800, &projection, target);
            },
            GameState::GameOver => {
                // TODO: Draw lose text!
            },
            _ => {},
        }
    }
}
