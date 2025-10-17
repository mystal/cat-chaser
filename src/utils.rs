use bevy::prelude::*;

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update_blink_visibility);
    }
}

#[derive(Component)]
pub struct Blink {
    timer: Timer,
}

impl Blink {
    pub fn from_seconds(blink_time: f32, start_enabled: bool) -> Self {
        let mut timer = Timer::from_seconds(blink_time, TimerMode::Repeating);
        if !start_enabled {
            timer.pause();
        }
        Self {
            timer,
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        if enabled {
            self.timer.reset();
            self.timer.unpause();
        } else {
            self.timer.pause();
        }
    }

    pub fn enable(&mut self) {
        self.set_enabled(true);
    }

    pub fn disable(&mut self) {
        self.set_enabled(false);
    }

    pub fn is_enabled(&self) -> bool {
        !self.timer.is_paused()
    }
}

fn update_blink_visibility(
    time: Res<Time>,
    mut blink_q: Query<(&mut Blink, &mut Visibility)>,
) {
    let dt = time.delta();
    for (mut blink, mut vis) in blink_q.iter_mut() {
        if !blink.is_enabled() {
            *vis = Visibility::Inherited;
            continue;
        }

        if blink.timer.tick(dt).just_finished() {
            *vis = match *vis {
                Visibility::Inherited | Visibility::Visible => Visibility::Hidden,
                Visibility::Hidden => Visibility::Visible,
            };
        }
    }
}
