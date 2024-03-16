use bevy::prelude::*;

mod classes;
mod hud;
mod menus;

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
) {
    ui_scale.0 = 4.0;
}
