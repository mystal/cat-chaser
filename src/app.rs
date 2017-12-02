use midgar::{self, KeyCode, Midgar, Surface};

use world::GameWorld;
use renderer::GameRenderer;

pub struct GameApp {
    world: GameWorld,
    renderer: GameRenderer,
}

impl midgar::App for GameApp {
    fn create(midgar: &Midgar) -> Self {
        GameApp {
            world: GameWorld::new(),
            renderer: GameRenderer::new(midgar),
        }
    }

    fn step(&mut self, midgar: &mut Midgar) {
        if midgar.input().was_key_pressed(KeyCode::Escape) {
            midgar.set_should_exit();
            return;
        }

        let dt = midgar.time().delta_time() as f32;

        self.world.update(midgar, dt);

        self.renderer.render(midgar, dt, &self.world);
    }
}
