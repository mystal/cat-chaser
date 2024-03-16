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
    pub fn from_seconds(blink_time: f32) -> Self {
        Self {
            timer: Timer::from_seconds(blink_time, TimerMode::Repeating),
        }
    }
}

fn update_blink_visibility(
    time: Res<Time>,
    mut blink_q: Query<(&mut Blink, &mut Visibility)>,
) {
    let dt = time.delta();
    for (mut blink, mut vis) in blink_q.iter_mut() {
        if blink.timer.tick(dt).just_finished() {
            *vis = match *vis {
                Visibility::Inherited | Visibility::Visible => Visibility::Hidden,
                Visibility::Hidden => Visibility::Visible,
            };
        }
    }
}
