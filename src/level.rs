use bevy::prelude::*;

use crate::{
    AppState,
    assets::GameAssets,
    cats::CatBundle,
};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Playing), spawn_level);
    }
}

fn spawn_level(
    mut commands: Commands,
    assets: Res<GameAssets>,
) {
    commands.spawn(CatBundle::basic(Vec2::new(-100.0, -30.0), assets.basic_cat.clone()));
    commands.spawn(CatBundle::kitten(Vec2::new(0.0, -30.0), assets.kitten.clone()));
    commands.spawn(CatBundle::chonk(Vec2::new(100.0, -30.0), assets.fat_cat.clone()));
}
