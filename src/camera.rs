use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

use crate::{
    AppState, GAME_SIZE,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnExit(AppState::Loading), spawn_camera);
    }
}

fn spawn_camera(
    mut commands: Commands,
) {
    // Spawn camera.
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::Fixed {
        width: GAME_SIZE.x as f32,
        height: GAME_SIZE.y as f32,
    };
    commands.spawn(camera_bundle);
}
