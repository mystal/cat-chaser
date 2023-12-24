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
    keys: Res<Input<KeyCode>>,
    gamepads: Res<Gamepads>,
    pad_buttons: Res<Input<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
    mut egui_ctx: EguiContexts,
    mut player_q: Query<&mut PlayerInput>,
) {
    if player_q.is_empty() {
        return;
    }

    let mut movement = Vec2::ZERO;
    let mut bark = false;

    // Read input from first gamepad.
    if let Some(gamepad) = gamepads.iter().next() {
        // Movement
        let move_x = GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX);
        let move_y = GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY);
        if let (Some(x), Some(y)) = (axes.get(move_x), axes.get(move_y)) {
            let tmp = Vec2::new(x, y);
            // TODO: See if we can configure the deadzone using Bevy's APIs.
            if tmp.length() > 0.1 {
                movement = tmp;
            }
        }

        // Shoot
        let bark_button = GamepadButton::new(gamepad, GamepadButtonType::South);
        bark |= pad_buttons.pressed(bark_button);
    }

    // Read input from mouse/keyboard.
    // Movement
    if movement == Vec2::ZERO && !egui_ctx.ctx_mut().wants_keyboard_input() {
        let x = (keys.pressed(KeyCode::D) as i8 - keys.pressed(KeyCode::A) as i8) as f32;
        let y = (keys.pressed(KeyCode::W) as i8 - keys.pressed(KeyCode::S) as i8) as f32;
        movement = Vec2::new(x, y).normalize_or_zero();
    }

    // Bark
    bark |= keys.pressed(KeyCode::Space) && !egui_ctx.ctx_mut().wants_keyboard_input();

    // Store results in player input components.
    for mut input in player_q.iter_mut() {
        input.movement = movement;
        input.bark = bark;
    }
}
