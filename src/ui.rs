use bevy::prelude::*;

mod classes;
mod hud;
mod menus;

use crate::window::WindowState;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(hud::HudPlugin)
            .add_plugins(menus::MenusPlugin)
            .add_systems(Startup, setup_ui);
    }
}

fn setup_ui(
    mut ui_scale: ResMut<UiScale>,
    window_state: Res<WindowState>,
) {
    ui_scale.0 = 3.0 * window_state.scale as f32;
}
