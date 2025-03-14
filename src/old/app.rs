use midgar::{self, KeyCode, Midgar};

use crate::config;
use cgmath;
use crate::entities::Camera;
use crate::world::GameWorld;
use crate::renderer::GameRenderer;
use crate::sounds::{Sounds, AudioController};

pub struct GameApp<'a> {
    camera: Camera,
    world: GameWorld,
    sounds: Sounds,
    renderer: GameRenderer<'a>,
}

impl<'a> midgar::App for GameApp<'a> {
    fn create(midgar: &Midgar) -> Self {
        let mut sounds = Sounds::new();
        sounds.intro_music.set_volume(0.2);
        sounds.intro_music.play();

        GameApp {
            world: GameWorld::new(),
            camera: Camera {
                pos: cgmath::vec2(config::GAME_SIZE.x as f32 / 2.0, config::GAME_SIZE.y as f32 / 2.0),
                bounds: config::GAME_SIZE.cast::<f32>(),
                zoom: 1,
            },
            renderer: GameRenderer::new(midgar),
            sounds,
        }
    }

    fn step(&mut self, midgar: &mut Midgar) {
        if midgar.input().was_key_pressed(KeyCode::Escape) {
            midgar.set_should_exit();
            return;
        }

        let dt = midgar.time().delta_time() as f32;

        if !self.sounds.intro_music.is_playing() && !self.sounds.background_music.is_playing() {
            self.sounds.background_music.set_volume(0.2);
            self.sounds.background_music.play();
        }
        self.world.update(midgar, dt);

        self.renderer.render(midgar, dt, &self.world, &self.camera);
    }
}
