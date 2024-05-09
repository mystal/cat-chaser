use bevy::prelude::*;
use bevy_ui_dsl::*;

use crate::{
    AppState,
    game::{self, CatStats, GameState},
    ui::classes::*,
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
    asset_server: Res<AssetServer>,
) {
    rooti(c_root, &asset_server, &mut commands, (HudRoot, Name::new("HudRoot")), |p| {
        nodei(c_cat_tracker, Name::new("CatTracker"), p, |p| {
            image(c_cat_face, p);
            // TODO: Add a drop shadow to the text.
            texti("00/00", c_tracker_text, c_font_tracker, CatTracker, p);
        });
        nodei(c_next_level, Name::new("NextLevel"), p, |p| {
            // TODO: Add a drop shadow to the text.
            texti("Cats Corralled!\nPress Enter to start the next level", c_next_level_text, c_font_next_level, NextLevelText, p);
        });
        nodei(c_victory, Name::new("Victory"), p, |p| {
            // TODO: Add a drop shadow to the text.
            texti("You are the most magical corgi in all the land!\nPress Enter to start anew!", c_next_level_text, c_font_next_level, VictoryText, p);
        });
    });
}

fn destroy_hud(
    mut commands: Commands,
    hud_q: Query<Entity, With<HudRoot>>,
) {
    for entity in hud_q.iter() {
        commands.entity(entity).despawn_recursive();
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
        tracker_text.sections[0].value = format!("{:02}/{:02}", cat_stats.in_pen(), cat_stats.total());
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
