use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

use crate::{
    AppState, SCREEN_SIZE, WORLD_SIZE,
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
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::Fixed {
        width: SCREEN_SIZE.x as f32,
        height: SCREEN_SIZE.y as f32,
    };
    commands.spawn(camera);
}

fn scale_camera(
    mut camera_q: Query<&mut OrthographicProjection, With<Camera>>,
) {
    for mut projection in camera_q.iter_mut() {
        projection.scaling_mode = ScalingMode::Fixed {
            width: WORLD_SIZE.x as f32,
            height: WORLD_SIZE.y as f32,
        };
    }
}
