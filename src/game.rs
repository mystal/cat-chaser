use avian2d::prelude::Collider;
use bevy::prelude::*;

use crate::{
    AppState, WORLD_SIZE,
    assets::GameAssets,
    cats::{self, Cat, CatState},
    dog::DogPlugin,
    level::{CurrentLevel, Levels},
    physics::{collider, GameLayer},
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum GameState {
    #[default]
    None,
    Playing,
    LevelClear,
    Victory,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<GameState>()
            .add_plugins(DogPlugin)
            .init_resource::<CatStats>()
            .add_systems(OnEnter(AppState::Playing), setup_game)
            .add_systems(Update, (
                (
                    update_cat_stats,
                    check_state_change,
                ).after(cats::update_cats).chain(),
            ).run_if(in_state(AppState::Playing)));
    }
}

#[derive(Default, Resource)]
pub struct CatStats {
    total: u32,
    in_pen: u32,
}

impl CatStats {
    pub fn total(&self) -> u32 {
        self.total
    }

    pub fn in_pen(&self) -> u32 {
        self.in_pen
    }

    pub fn all_penned(&self) -> bool {
        self.total > 0 && self.in_pen == self.total
    }
}

#[derive(Component)]
pub struct CatBox;

fn setup_game(
    mut commands: Commands,
    mut next_game_state: ResMut<NextState<GameState>>,
    assets: Res<GameAssets>,
) {
    debug!("Setup game");

    // Spawn floor.
    commands.spawn((
        Name::new("Floor"),
        Sprite {
            image: assets.floor.clone(),
            custom_size: Some(WORLD_SIZE.as_vec2() / 2.0),
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,
                tile_y: true,
                stretch_value: 1.0,
            },
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -1.0)
            .with_scale(Vec3::new(2.0, 2.0, 1.0)),
    ));

    // Spawn cat_box.
    commands.spawn((
        CatBox,
        Name::new("CatBox"),
        Sprite::from_image(assets.cat_box.clone()),
        Transform::from_xyz(0.0, 0.0, -0.5),
        collider(Collider::rectangle(60.0, 60.0), GameLayer::CatBox, GameLayer::Cat),
    ));

    next_game_state.set(GameState::Playing);
}

pub fn update_cat_stats(
    mut cat_stats: ResMut<CatStats>,
    cats_q: Query<&Cat>,
) {
    // TODO: Update on CatState changes instead of every frame?
    cat_stats.total = cats_q.iter().count() as u32;
    cat_stats.in_pen = cats_q.iter()
        .filter(|cat| cat.state == CatState::InPen)
        .count()
        as u32;
}

pub fn check_state_change(
    game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    keys: Res<ButtonInput<KeyCode>>,
    cat_stats: Res<CatStats>,
    current_level: Res<CurrentLevel>,
    levels: Res<Levels>,
) {
    match game_state.get() {
        GameState::Playing => {
            let level_clear = cat_stats.all_penned() || keys.just_pressed(KeyCode::Tab);
            if !level_clear {
                return;
            }
            debug!("Level clear!");
            if current_level.index + 1 >= levels.len() {
                next_game_state.set(GameState::Victory);
            } else {
                next_game_state.set(GameState::LevelClear);
            }
        }
        _ => {
            if keys.just_pressed(KeyCode::Enter) {
                next_game_state.set(GameState::Playing);
            }
        }
    }
}
