use bevy::prelude::*;

// mod classes;
mod hud;
mod menus;
mod party;

use crate::window::{WindowState, update_window_state};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                hud::HudPlugin,
                menus::MenusPlugin,
                party::PartyPlugin,
            ))
            .add_systems(PostUpdate, update_ui_scale.after(update_window_state));
    }
}

fn update_ui_scale(
    mut ui_scale: ResMut<UiScale>,
    window_state: Res<WindowState>,
) {
    if 3.0 * window_state.scale as f32 != ui_scale.0 {
        // Make the UI match current viewport scaling.
        ui_scale.0 = 3.0 * window_state.scale as f32;
    }
}
