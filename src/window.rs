use std::fs;
use std::path::Path;

use bevy::prelude::*;
use bevy::app::AppExit;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::window::PrimaryWindow;
use serde::{Deserialize, Serialize};

use crate::DEFAULT_SCALE;

pub const WINDOW_TITLE: &str = "Cat Chaser";
const WINDOW_STATE_FILENAME: &str = "window_state.ron";

// pub fn window_plugin() -> WindowPlugin {
// }

#[derive(Clone, Debug, Deserialize, Serialize, Resource)]
pub struct WindowState {
    #[serde(default)]
    pub position: WindowPosition,
    #[serde(default)]
    pub scale: u8,
}

impl Default for WindowState {
    fn default() -> Self {
        Self {
            position: WindowPosition::Automatic,
            scale: DEFAULT_SCALE,
        }
    }
}

pub fn load_window_state() -> WindowState {
    if Path::new(WINDOW_STATE_FILENAME).is_file() {
        // TODO: Log errors if these fail and return default.
        let window_state_str = fs::read_to_string(WINDOW_STATE_FILENAME)
            .expect("Could not read window state file");
        ron::from_str(&window_state_str)
            .expect("Could not deserialize window state")
    } else {
        default()
    }
}

#[derive(Resource)]
struct LogFpsTimer(Timer);

impl Default for LogFpsTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1.0, TimerMode::Repeating))
    }
}

pub struct WindowPlugin {
    saved_window_state: WindowState,
}

impl WindowPlugin {
    pub fn new(saved_window_state: WindowState) -> Self {
        Self {
            saved_window_state,
        }
    }
}

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(self.saved_window_state.clone())
            .add_systems(PostUpdate, update_window_state);
        #[cfg(not(target_arch = "wasm32"))]
        {
            app.add_systems(Last, save_window_state_on_exit.run_if(on_event::<AppExit>()));

            app
                .insert_resource(LogFpsTimer::default())
                .add_systems(PostUpdate, log_fps_in_window_title.after(update_window_state));
        }
    }
}

fn update_window_state(
    mut window_state: ResMut<WindowState>,
    window_q: Query<&Window, (With<PrimaryWindow>, Changed<Window>)>,
) {
    if let Ok(window) = window_q.get_single() {
        window_state.position = window.position;
    }
}

fn log_fps_in_window_title(
    mut window_q: Query<&mut Window, With<PrimaryWindow>>,
    mut log_fps_timer: ResMut<LogFpsTimer>,
    diagnostics: Res<DiagnosticsStore>,
    time: Res<Time>,
) {
    if !log_fps_timer.0.tick(time.delta()).just_finished() {
        return;
    }

    if let Ok(mut window) = window_q.get_single_mut() {
        if let (Some(fps), Some(frame_time)) = (
            diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS).and_then(|f| f.smoothed()),
            diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME).and_then(|f| f.smoothed()),
        ) {
            window.title = format!("{} - {:.2} fps ({:.2} ms)", WINDOW_TITLE, fps, frame_time);
        }
    }
}

fn save_window_state_on_exit(
    window_state: Res<WindowState>,
) {
    info!("Saving window state");

    let pretty_config = ron::ser::PrettyConfig::default();
    let state_str = ron::ser::to_string_pretty(&*window_state, pretty_config)
        .expect("Could not serialize window state");
    fs::write(WINDOW_STATE_FILENAME, state_str)
        .expect("Could not write window state to file");
}
