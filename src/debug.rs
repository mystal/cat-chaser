use avian2d::debug_render::*;
use bevy::prelude::*;
use bevy_egui::{egui, input::egui_wants_any_keyboard_input, EguiContextSettings, EguiContexts, EguiPrimaryContextPass};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                bevy_egui::EguiPlugin::default(),
                WorldInspectorPlugin::default().run_if(show_world_inspector),
                PhysicsDebugPlugin::default(),
            ))
            .insert_gizmo_config(
                PhysicsGizmos::default(),
                GizmoConfig {
                    enabled: false,
                    ..default()
                },
            )

            .insert_resource(DebugState::default())
            .add_systems(EguiPrimaryContextPass,
                debug_menu_bar.run_if(debug_ui_enabled)
            )
            .add_systems(Update, (
                toggle_debug_ui.run_if(not(egui_wants_any_keyboard_input)),
                toggle_physics_debug_render.run_if(not(egui_wants_any_keyboard_input)),
                set_ui_scale_factor,
            ));
    }
}

#[derive(Resource)]
struct DebugState {
    enabled: bool,
    show_world_inspector: bool,
}

impl Default for DebugState {
    fn default() -> Self {
        Self {
            enabled: false,
            show_world_inspector: false,
        }
    }
}

fn debug_ui_enabled(
    debug_ui: Res<DebugState>,
) -> bool {
    debug_ui.enabled
}

fn show_world_inspector(
    debug_ui: Res<DebugState>,
) -> bool {
    debug_ui.enabled && debug_ui.show_world_inspector
}

fn debug_menu_bar(
    mut debug_state: ResMut<DebugState>,
    mut gizmo_config_store: ResMut<GizmoConfigStore>,
    mut egui_ctx: EguiContexts,
) {
    let ctx = egui_ctx.ctx_mut().unwrap();
    let (gizmo_config, _) = gizmo_config_store.config_mut::<PhysicsGizmos>();

    egui::TopBottomPanel::top("debug_panel")
        .show(ctx, |ui| {
            // NOTE: An egui bug makes clicking on the menu bar not report wants_pointer_input,
            // which means it'll register as a click in game.
            // https://github.com/emilk/egui/issues/2606
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("Debug", |ui| {
                    ui.checkbox(&mut debug_state.show_world_inspector, "World Inspector");
                    ui.checkbox(&mut gizmo_config.enabled, "Debug Physics Render");
                    // ui.checkbox(&mut debug_state.place_entity_mode, "Place Entity Mode");
                });
            });
        });
}

fn toggle_debug_ui(
    keys: Res<ButtonInput<KeyCode>>,
    mut debug_state: ResMut<DebugState>,
) {
    if keys.just_pressed(KeyCode::Backspace) {
        debug_state.enabled = !debug_state.enabled;
    }
}

fn toggle_physics_debug_render(
    keys: Res<ButtonInput<KeyCode>>,
    mut gizmo_config_store: ResMut<GizmoConfigStore>,
) {
    if keys.just_pressed(KeyCode::Digit0) {
        let (gizmo_config, _) = gizmo_config_store.config_mut::<PhysicsGizmos>();
        gizmo_config.enabled = !gizmo_config.enabled;
    }
}

fn set_ui_scale_factor(
    mut windows: Query<&mut EguiContextSettings, (With<Camera>, Added<EguiContextSettings>)>,
) {
    for mut egui_settings in windows.iter_mut() {
        // TODO: Make this configurable, since it'll depend on the screen.
        // TODO: Default to guessed scale factor?
        egui_settings.scale_factor = 1.5;
    }
}
