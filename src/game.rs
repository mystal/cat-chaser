use bevy::prelude::*;

use crate::{
    AppState, GAME_SIZE,
    assets::GameAssets,
    dog::DogPlugin,
    level::NextLevelEvent,
    physics::{groups, ColliderBundle},
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(DogPlugin)
            .add_systems(OnEnter(AppState::Playing), setup_game);
    }
}

#[derive(Component)]
pub struct CatBox;

fn setup_game(
    mut commands: Commands,
    mut next_level: EventWriter<NextLevelEvent>,
    assets: Res<GameAssets>,
) {
    debug!("Setup game");

    // Spawn floor.
    let floor_bundle = SpriteBundle {
        sprite: Sprite {
            custom_size: Some(GAME_SIZE.as_vec2() / 2.0),
            ..default()
        },
        texture: assets.floor.clone(),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, -1.0),
            scale: Vec3::new(2.0, 2.0, 1.0),
            ..default()
        },
        ..default()
    };
    commands.spawn((
        Name::new("Floor"),
        floor_bundle,
        ImageScaleMode::Tiled {
            tile_x: true,
            tile_y: true,
            stretch_value: 1.0,
        }
    ));

    // Spawn cat_box.
    commands.spawn((
        CatBox,
        Name::new("CatBox"),
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(-100.0, 50.0, -0.5)),
            texture: assets.cat_box.clone(),
            ..default()
        },
        ColliderBundle::rect((60.0, 60.0).into(), groups::CATBOX, groups::CAT),
    ));

    next_level.send_default();
}
