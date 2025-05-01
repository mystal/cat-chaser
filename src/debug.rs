use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;
use bevy_egui::{egui, EguiContexts, EguiSettings};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::render::{DebugRenderContext, RapierDebugRenderPlugin};

use crate::input;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                bevy_egui::EguiPlugin,
                WorldInspectorPlugin::default().run_if(show_world_inspector),
                RapierDebugRenderPlugin::default().disabled(),
            ))

            .insert_resource(DebugState::default())
            // Run absorb_egui_inputs after egui reads input, but before the game reads input (and
            // before egui starts a new frame).
            .add_systems(PreUpdate, absorb_egui_inputs
                .after(bevy_egui::systems::process_input_system)
                .before(bevy_egui::EguiSet::BeginPass)
                .before(input::read_player_input))
            .add_systems(Update, (
                debug_menu_bar.run_if(debug_ui_enabled),
                toggle_debug_ui,
                toggle_physics_debug_render,
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

fn absorb_egui_inputs(
    mut contexts: bevy_egui::EguiContexts,
    mut mouse: ResMut<ButtonInput<MouseButton>>,
    mut mouse_wheel: ResMut<Events<MouseWheel>>,
    mut keyboard: ResMut<ButtonInput<KeyCode>>,
) {
    // Clear out all inputs if egui is handling them. That way other game systems don't try to use
    // them.
    let ctx = contexts.ctx_mut();
    if ctx.wants_pointer_input() || ctx.is_pointer_over_area() {
        let modifiers = [
            KeyCode::SuperLeft,
            KeyCode::SuperRight,
            KeyCode::ControlLeft,
            KeyCode::ControlRight,
            KeyCode::AltLeft,
            KeyCode::AltRight,
            KeyCode::ShiftLeft,
            KeyCode::ShiftRight,
        ];

        // Figure out which modifiers are currently held.
        let pressed = modifiers.map(|key| keyboard.pressed(key).then_some(key));

        // Clear out all inputs.
        mouse.reset_all();
        mouse_wheel.clear();
        keyboard.reset_all();

        for key in pressed.into_iter().flatten() {
            keyboard.press(key);
        }
    }
}

fn set_ui_scale_factor(mut windows: Query<&mut EguiSettings, (With<Window>, Added<EguiSettings>)>) {
    for mut egui_settings in windows.iter_mut() {
        // TODO: Make this configurable, since it'll depend on the screen.
        // TODO: Default to guessed scale factor?
        egui_settings.scale_factor = 1.5;
    }
}
