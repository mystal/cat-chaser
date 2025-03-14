use bevy::prelude::*;

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
    gamepads: Res<Gamepads>,
    pad_buttons: Res<ButtonInput<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
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

    // Store results in player input components.
    for mut input in player_q.iter_mut() {
        input.movement = movement;
        input.bark = bark;
    }
}
