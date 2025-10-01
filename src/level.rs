use std::ops::Deref;

use bevy::prelude::*;
use serde::Deserialize;

use crate::{
    WORLD_SIZE,
    assets::GameAssets,
    cats::{self, CAT_BOUNDS, Cat},
    dog::{self, Dog},
    game::{CatBox, GameState},
};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CurrentLevel>()
            .init_resource::<Levels>()
            .add_systems(OnEnter(GameState::Playing), spawn_next_level);
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct LevelCats {
    pub basic: u8,
    pub kitten: u8,
    pub chonk: u8,
}

#[derive(Debug, Default, Deserialize, Resource, Asset, TypePath)]
pub struct Levels(Vec<LevelCats>);

impl Deref for Levels {
    type Target = Vec<LevelCats>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Default, Resource)]
pub struct CurrentLevel {
    pub index: usize,
    pub cats: LevelCats,
    pub cats_herded: u8,
}

fn spawn_next_level(
    mut commands: Commands,
    assets: Res<GameAssets>,
    levels: Res<Levels>,
    mut current_level: ResMut<CurrentLevel>,
    cats_q: Query<Entity, With<Cat>>,
    dog_q: Query<Entity, With<Dog>>,
    catbox_q: Query<&Transform, With<CatBox>>,
) {
    // Despawn all cats.
    for entity in cats_q.iter() {
        commands.entity(entity).despawn();
    }

    // Despawn dog.
    for entity in dog_q.iter() {
        commands.entity(entity).despawn();
    }

    // Spawn a new dog.
    let catbox_pos = catbox_q.single()
        .map(|t| t.translation.truncate())
        .unwrap_or_default();
    commands.spawn(dog::dog(catbox_pos, assets.wizard_dog.clone()));

    // Then spawn new cats.
    let level_index = if current_level.index + 1 < levels.len() {
        current_level.index + 1
    } else {
        1
    };
    let Some(level_cats) = levels.get(level_index) else {
        *current_level = CurrentLevel::default();
        error!("Could not load level {}. Levels list length: {}", level_index, levels.len());
        return;
    };

    // Spawn cats in random locations.
    let random_location = || {
        loop {
            let x = (fastrand::f32() - 0.5) * (WORLD_SIZE.x as f32 - (CAT_BOUNDS * 2.0));
            let y = (fastrand::f32() - 0.5) * (WORLD_SIZE.y as f32 - (CAT_BOUNDS * 2.0));
            let pos = Vec2::new(x, y);
            if pos.distance_squared(catbox_pos) > 80.0 * 80.0 {
                break pos;
            }
        }
    };
    for _ in 0..level_cats.basic {
        commands.spawn(cats::basic_cat(random_location(), assets.basic_cat.clone()));
    }
    for _ in 0..level_cats.kitten {
        commands.spawn(cats::kitten_cat(random_location(), assets.kitten.clone()));
    }
    for _ in 0..level_cats.chonk {
        commands.spawn(cats::chonk_cat(random_location(), assets.fat_cat.clone()));
    }

    // Set CurrentLevel info.
    current_level.index = level_index;
    current_level.cats = level_cats.clone();
    current_level.cats_herded = 0;
}
