use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

use crate::{
    AppState, GAME_SIZE,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnExit(AppState::Loading), spawn_camera)
            .add_systems(OnEnter(AppState::Playing), scale_camera);
    }
}

fn spawn_camera(
    mut commands: Commands,
) {
    // Spawn camera.
    commands.spawn(Camera2dBundle::default());
}

fn scale_camera(
    mut camera_q: Query<&mut OrthographicProjection, With<Camera>>,
) {
    for mut projection in camera_q.iter_mut() {
        projection.scaling_mode = ScalingMode::Fixed {
            width: GAME_SIZE.x as f32,
            height: GAME_SIZE.y as f32,
        };
    }
}
