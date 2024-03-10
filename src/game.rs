use bevy::{prelude::*, render::camera::ScalingMode};

use crate::{
    GAME_SIZE, AppState,
    assets::GameAssets,
    dog::{DogBundle, DogPlugin},
    physics::{ColliderBundle, groups},
};

#[derive(Component)]
pub struct CatBox;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(DogPlugin)
            .add_systems(OnEnter(AppState::Playing), setup_game);
    }
}

fn setup_game(
    mut commands: Commands,
    assets: Res<GameAssets>,
) {
    debug!("Setup game");

    // Spawn camera.
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::Fixed {
        width: GAME_SIZE.x as f32,
        height: GAME_SIZE.y as f32,
    };
    commands.spawn(camera_bundle);

    // Spawn dog.
    commands.spawn(DogBundle::new(Vec2::ZERO, assets.wizard_dog.clone()));

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
}
