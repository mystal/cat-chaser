use bevy::prelude::*;

use crate::{
    AppState,
    assets::GameAssets,
    game::{self, CatStats, GameState},
};

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Playing), setup_hud)
            .add_systems(OnExit(AppState::Playing), destroy_hud)
            .add_systems(Update, (
                update_cat_tracker.after(game::update_cat_stats),
                update_next_level_prompt,
            ).run_if(in_state(AppState::Playing)));
    }
}

#[derive(Component)]
struct HudRoot;

#[derive(Component)]
struct CatTracker;

#[derive(Component)]
struct NextLevelText;

#[derive(Component)]
struct VictoryText;

fn setup_hud(
    mut commands: Commands,
    assets: Res<GameAssets>,
) {
    // Set up Cat Tracker.
    let cat_tracker = (
        Name::new("CatTracker"),
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(4.0),
            top: Val::Px(2.0),
            align_items: AlignItems::Center,
            column_gap: Val::Px(4.0),
            ..default()
        },
        children![
            (
                ImageNode {
                    image: assets.cat_face.clone(),
                    ..default()
                },
                Node {
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
            ),
            (
                CatTracker,
                Text("00/00".into()),
                TextColor(Color::WHITE),
                TextFont {
                    font: assets.font.clone(),
                    font_size: 14.0,
                    ..default()
                },
                TextShadow {
                    offset: Vec2::splat(0.8),
                    color: Color::BLACK,
                },
                Node {
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
            ),
        ],
    );

    // Set up Next Level text.
    let next_level_text = (
        NextLevelText,
        Name::new("NextLevel"),
        Text("Cats Corralled!\nPress Enter to start the next level".into()),
        TextColor(Color::WHITE),
        TextFont {
            font: assets.font.clone(),
            font_size: 12.0,
            ..default()
        },
        TextShadow {
            offset: Vec2::splat(0.8),
            color: Color::BLACK,
        },
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(6.0),
            bottom: Val::Px(6.0),
            ..default()
        },
        Visibility::Hidden,
    );

    // Set up Victory text.
    let victory_text = (
        Name::new("Victory"),
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            bottom: Val::Px(6.0),
            ..default()
        },
        children![(
            VictoryText,
            Text("You are the most magical corgi in all the land!\nPress Enter to start anew!".into()),
            TextColor(Color::WHITE),
            TextFont {
                font: assets.font.clone(),
                font_size: 12.0,
                ..default()
            },
            TextShadow {
                offset: Vec2::splat(0.8),
                color: Color::BLACK,
            },
            Visibility::Hidden,
        )],
    );

    // Spawn HUD root.
    commands.spawn((
        Name::new("HudRoot"),
        HudRoot,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        children![
            cat_tracker,
            next_level_text,
            victory_text,
        ],
    ));
}

fn destroy_hud(
    mut commands: Commands,
    hud_q: Query<Entity, With<HudRoot>>,
) {
    for entity in hud_q.iter() {
        commands.entity(entity).despawn();
    }
}

fn update_cat_tracker(
    mut tracker_q: Query<&mut Text, With<CatTracker>>,
    cat_stats: Res<CatStats>,
) {
    if !cat_stats.is_changed() {
        return;
    }

    for mut tracker_text in tracker_q.iter_mut()  {
        tracker_text.0 = format!("{:02}/{:02}", cat_stats.in_pen(), cat_stats.total());
    }
}

fn update_next_level_prompt(
    game_state: Res<State<GameState>>,
    mut next_level_q: Query<&mut Visibility, (With<NextLevelText>, Without<VictoryText>)>,
    mut victory_q: Query<&mut Visibility, With<VictoryText>>,
) {
    if !game_state.is_changed() {
        return;
    }

    match game_state.get() {
        GameState::LevelClear => {
            for mut vis in next_level_q.iter_mut()  {
                *vis = Visibility::Inherited;
            }
            for mut vis in victory_q.iter_mut()  {
                *vis = Visibility::Hidden;
            }
        },
        GameState::Victory => {
            for mut vis in next_level_q.iter_mut()  {
                *vis = Visibility::Hidden;
            }
            for mut vis in victory_q.iter_mut()  {
                *vis = Visibility::Inherited;
            }
        },
        GameState::None | GameState::Playing => {
            for mut vis in next_level_q.iter_mut()  {
                *vis = Visibility::Hidden;
            }
            for mut vis in victory_q.iter_mut()  {
                *vis = Visibility::Hidden;
            }
        },
    }
}
