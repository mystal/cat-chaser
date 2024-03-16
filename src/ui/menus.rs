use bevy::prelude::*;
use bevy_ui_dsl::*;

use crate::{
    AppState,
    ui::classes::*,
    utils::Blink,
};

pub struct MenusPlugin;

impl Plugin for MenusPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::StartMenu), show_start)
            .add_systems(Update, handle_start_input.run_if(in_state(AppState::StartMenu)))
            .add_systems(OnExit(AppState::StartMenu), clear_start)
            .add_systems(OnEnter(AppState::Credits), show_credits)
            .add_systems(OnExit(AppState::Credits), clear_credits)
            .add_systems(OnEnter(AppState::HowToPlay), show_how_to_play)
            .add_systems(OnExit(AppState::HowToPlay), clear_how_to_play);
    }
}

#[derive(Component)]
pub struct MenuRoot;

fn show_start(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    rooti(c_root, &asset_server, &mut commands, MenuRoot, |p| {
        image(c_start_image, p);

        let blink = Blink::from_seconds(0.5);
        // TODO: Add a drop shadow to the text.
        texti("Press Enter to play!", c_start_text, c_font_start, blink, p);
    });
}

fn handle_start_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keys.just_pressed(KeyCode::Tab) {
        next_state.set(AppState::Credits);
    } else if keys.just_pressed(KeyCode::Enter) {
        next_state.set(AppState::Playing);
    }
}

fn clear_start(
    mut commands: Commands,
    root_q: Query<Entity, With<MenuRoot>>,
) {
    for entity in root_q.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn show_credits(
) {
}

fn clear_credits(
    mut commands: Commands,
    root_q: Query<Entity, With<MenuRoot>>,
) {
    for entity in root_q.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn show_how_to_play(
) {
}

fn clear_how_to_play(
    mut commands: Commands,
    root_q: Query<Entity, With<MenuRoot>>,
) {
    for entity in root_q.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
