use bevy::prelude::*;

use crate::dog::{DogBundle, DogPlugin};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(DogPlugin)
            .add_systems(Startup, setup_game);
    }
}

fn setup_game(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(DogBundle::new(Vec2::ZERO));
}
