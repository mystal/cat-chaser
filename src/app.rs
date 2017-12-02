use midgar::{self, KeyCode, Midgar, Surface};

pub struct GameApp {
}

impl midgar::App for GameApp {
    fn create(midgar: &Midgar) -> Self {
        GameApp {
        }
    }

    fn step(&mut self, midgar: &mut Midgar) {
        if midgar.input().was_key_pressed(KeyCode::Escape) {
            midgar.set_should_exit();
            return;
        }

        let mut target = midgar.graphics().display().draw();

        target.clear_color(0.0, 0.3, 0.7, 1.0);

        target.finish()
            .expect("target.finish() failed");
    }
}
