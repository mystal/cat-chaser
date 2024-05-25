use bevy::prelude::*;
use bevy::render::camera::{ScalingMode, Viewport};
use bevy::window::{PrimaryWindow, WindowResized};

use crate::{
    AppState, SCREEN_SIZE, WORLD_SIZE,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnExit(AppState::Loading), spawn_camera)
            .add_systems(OnEnter(AppState::Playing), scale_camera)
            .add_systems(Update, handle_window_resize);
    }
}

fn spawn_camera(
    mut commands: Commands,
    window_q: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_q.single();

    // Spawn camera.
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::Fixed {
        width: SCREEN_SIZE.x as f32,
        height: SCREEN_SIZE.y as f32,
    };
    camera.camera.viewport = compute_viewport(window.physical_width(), window.physical_height());
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

fn handle_window_resize(
    mut resize_events: EventReader<WindowResized>,
    mut camera_q: Query<&mut Camera>,
    window_q: Query<(Entity, &Window), With<PrimaryWindow>>,
) {
    let Ok((window_entity, window)) = window_q.get_single() else {
        return;
    };
    let Ok(mut camera) = camera_q.get_single_mut() else {
        return;
    };

    for e in resize_events.read() {
        if e.window != window_entity {
            continue;
        }

        camera.viewport = compute_viewport(window.physical_width(), window.physical_height());
    }
}

fn compute_viewport_integer(physical_width: u32, physical_height: u32) -> Option<Viewport> {
    let scale_x = physical_width / SCREEN_SIZE.x;
    let scale_y = physical_height / SCREEN_SIZE.y;
    let scale = scale_x.min(scale_y);
    if scale == 0 {
        return None;
    }
    let physical_size = SCREEN_SIZE * scale;
    let physical_position = UVec2::new(
        (physical_width - physical_size.x) / 2,
        (physical_height - physical_size.y) / 2,
    );

    Some(Viewport {
        physical_position,
        physical_size,
        ..default()
    })
}

fn compute_viewport(physical_width: u32, physical_height: u32) -> Option<Viewport> {
    let scale_x = physical_width as f32 / SCREEN_SIZE.x as f32;
    let scale_y = physical_height as f32 / SCREEN_SIZE.y as f32;
    let scale = scale_x.min(scale_y);

    let physical_size = (SCREEN_SIZE.as_vec2() * scale).as_uvec2();
    let physical_position = UVec2::new(
        (physical_width - physical_size.x) / 2,
        (physical_height - physical_size.y) / 2,
    );

    Some(Viewport {
        physical_position,
        physical_size,
        ..default()
    })
}
