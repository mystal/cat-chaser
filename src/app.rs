use std::env;

use midgar::{self, KeyCode, Midgar};

use config;
use cgmath;
use entities::Camera;
use world::GameWorld;
use renderer::GameRenderer;
use sounds::{Sounds, AudioController};

pub struct GameApp<'a> {
    camera: Camera,
    world: GameWorld,
    renderer: GameRenderer<'a>,
}

impl<'a> midgar::App for GameApp<'a> {
    fn create(midgar: &Midgar) -> Self {
        let assets_path = env::args().nth(1).unwrap_or("assets".into());
        let mut sounds = Sounds::new(&assets_path);
        sounds.intro_music.set_volume(0.2);
        sounds.intro_music.play();

        GameApp {
            world: GameWorld::new(sounds),
            camera: Camera {
                pos: cgmath::vec2(config::GAME_SIZE.x as f32 / 2.0, config::GAME_SIZE.y as f32 / 2.0),
                bounds: config::GAME_SIZE.cast::<f32>(),
                zoom: 1,
            },
            renderer: GameRenderer::new(midgar, &assets_path),
        }
    }

    fn step(&mut self, midgar: &mut Midgar) {
        if midgar.input().was_key_pressed(KeyCode::Escape) {
            midgar.set_should_exit();
            return;
        }

        let dt = midgar.time().delta_time() as f32;

        if !self.world.sounds.intro_music.is_playing() && !self.world.sounds.background_music.is_playing() {
            self.world.sounds.background_music.set_volume(0.2);
            self.world.sounds.background_music.play();
        }
        self.world.update(midgar, dt);

        self.renderer.render(midgar, dt, &self.world, &self.camera);
    }
}
