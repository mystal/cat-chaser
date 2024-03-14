use std::ops::Deref;

use bevy::prelude::*;
use serde::Deserialize;

use crate::{
    AppState,
    assets::GameAssets,
    cats::{Cat, CatBundle},
    dog::{Dog, DogBundle},
    game::CatBox,
};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CurrentLevel>()
            .add_event::<NextLevelEvent>()
            .add_systems(Update, spawn_next_level.run_if(in_state(AppState::Playing)))
            .add_systems(Update, debug_next_level.run_if(in_state(AppState::Playing)));
    }
}

#[derive(Default, Event)]
pub struct NextLevelEvent;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct LevelCats {
    pub basic: u8,
    pub kitten: u8,
    pub chonk: u8,
}

#[derive(Debug, Deserialize, Asset, TypePath)]
pub struct Levels(Vec<LevelCats>);

impl Deref for Levels {
    type Target = Vec<LevelCats>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Default, Resource)]
pub struct CurrentLevel {
    pub index: u8,
    pub cats: LevelCats,
    pub cats_herded: u8,
}

fn spawn_next_level(
    mut commands: Commands,
    mut next_level: EventReader<NextLevelEvent>,
    assets: Res<GameAssets>,
    level_assets: Res<Assets<Levels>>,
    mut current_level: ResMut<CurrentLevel>,
    cats_q: Query<Entity, With<Cat>>,
    dog_q: Query<Entity, With<Dog>>,
    catbox_q: Query<&GlobalTransform, With<CatBox>>,
) {
    if next_level.is_empty() {
        return;
    }
    next_level.clear();

    // Despawn all cats.
    for entity in cats_q.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Despawn dog.
    for entity in dog_q.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Spawn a new dog.
    let dog_pos = catbox_q.get_single()
        .map(|t| t.translation().truncate())
        .unwrap_or_default();
    commands.spawn(DogBundle::new(dog_pos, assets.wizard_dog.clone()));

    // Then spawn new cats.
    let levels = level_assets.get(&assets.levels).unwrap();
    let level_index = current_level.index + 1;
    let level_cats = levels.get(level_index as usize).unwrap();

    // TODO: Spawn in random locations.
    for _ in 0..level_cats.basic {
        commands.spawn(CatBundle::basic(Vec2::new(-100.0, -30.0), assets.basic_cat.clone()));
    }
    for _ in 0..level_cats.kitten {
        commands.spawn(CatBundle::kitten(Vec2::new(0.0, -30.0), assets.kitten.clone()));
    }
    for _ in 0..level_cats.chonk {
        commands.spawn(CatBundle::chonk(Vec2::new(100.0, -30.0), assets.fat_cat.clone()));
    }

    // Set CurrentLevel info.
    current_level.index = level_index;
    current_level.cats = level_cats.clone();
    current_level.cats_herded = 0;
}

fn debug_next_level(
    mut next_level: EventWriter<NextLevelEvent>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Tab) {
        next_level.send_default();
    }
}
