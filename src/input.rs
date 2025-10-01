use bevy::prelude::*;
use bevy_egui::EguiContexts;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreUpdate, read_player_input);
    }
}

#[derive(Default, Component, Reflect)]
pub struct PlayerInput {
    pub movement: Vec2,
    pub bark: bool,
}

pub fn read_player_input(
    keys: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    mut player_q: Query<&mut PlayerInput>,
    mut egui_ctx: EguiContexts,
) {
    if player_q.is_empty() {
        return;
    }

    let mut movement = Vec2::ZERO;
    let mut bark = false;

    // Read input from first gamepad.
    // TODO: Somehow keep track of which gamepad the player is using if there are multiple connected.
    if let Some(gamepad) = gamepads.iter().next() {
        // Movement
        if let (Some(x), Some(y)) = (gamepad.get(GamepadAxis::LeftStickX), gamepad.get(GamepadAxis::LeftStickY)) {
            let tmp = Vec2::new(x, y);
            // TODO: See if we can configure the deadzone using Bevy's APIs.
            if tmp.length() > 0.1 {
                movement = tmp;
            }
        }

        // Shoot
        bark |= gamepad.just_pressed(GamepadButton::South);
    }

    // Read input from keyboard.
    let egui_wants_input = egui_ctx.ctx_mut()
        .map(|ctx| ctx.wants_keyboard_input())
        .unwrap_or_default();
    if !egui_wants_input {
        // Movement
        if movement == Vec2::ZERO {
            let right = keys.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);
            let left = keys.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
            let x = (right as i8 - left as i8) as f32;

            let up = keys.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
            let down = keys.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);
            let y = (up as i8 - down as i8) as f32;

            movement = Vec2::new(x, y).normalize_or_zero();
        }

        // Bark
        bark |= keys.just_pressed(KeyCode::Space);
    }

    // Store results in player input components.
    for mut input in player_q.iter_mut() {
        input.movement = movement;
        input.bark = bark;
    }
}
