use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{egui, EguiContexts, EguiSettings};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::render::{DebugRenderContext, RapierDebugRenderPlugin};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(EguiSettings {
                // TODO: Make this configurable, since it'll depend on the screen.
                // TODO: Default to guessed scale factor?
                scale_factor: 1.5,
                ..default()
            })
            .add_plugins((
                WorldInspectorPlugin::default().run_if(show_world_inspector),
                RapierDebugRenderPlugin::default().disabled(),
            ))

            .insert_resource(DebugState::default())
            // Run these before game player input because wants_pointer_input will return false
            // otherwise.
            .add_systems(Update, (
                debug_menu_bar.run_if(debug_ui_enabled),
                toggle_debug_ui,
                toggle_physics_debug_render,
            )/*.before(input::read_player_input)*/);
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
    mut debug_physics_ctx: ResMut<DebugRenderContext>,
    mut egui_ctx: EguiContexts,
) {
    let ctx = egui_ctx.ctx_mut();

    egui::TopBottomPanel::top("debug_panel")
        .show(ctx, |ui| {
            // NOTE: An egui bug makes clicking on the menu bar not report wants_pointer_input,
            // which means it'll register as a click in game.
            // https://github.com/emilk/egui/issues/2606
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Debug", |ui| {
                    ui.checkbox(&mut debug_state.show_world_inspector, "World Inspector");
                    ui.checkbox(&mut debug_physics_ctx.enabled, "Debug Physics Render");
                    // ui.checkbox(&mut debug_state.place_entity_mode, "Place Entity Mode");
                });
            });
        });
}

fn toggle_debug_ui(
    keys: Res<ButtonInput<KeyCode>>,
    mut debug_state: ResMut<DebugState>,
    mut egui_ctx: EguiContexts,
) {
    if egui_ctx.ctx_mut().wants_keyboard_input() {
        return;
    }

    if keys.just_pressed(KeyCode::Backspace) {
        debug_state.enabled = !debug_state.enabled;
    }
}

fn toggle_physics_debug_render(
    keys: Res<ButtonInput<KeyCode>>,
    mut egui_ctx: EguiContexts,
    mut debug_render_context: ResMut<DebugRenderContext>,
) {
    if egui_ctx.ctx_mut().wants_keyboard_input() {
        return;
    }

    if keys.just_pressed(KeyCode::Digit0) {
        debug_render_context.enabled = !debug_render_context.enabled;
    }
}
