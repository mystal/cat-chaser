use bevy::prelude::*;
use bevy_ui_dsl::*;

use crate::{
    AppState,
    game::{self, CatStats},
    level::NextLevelEvent,
    ui::classes::*,
};

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Playing), setup_hud)
            .add_systems(OnExit(AppState::Playing), destroy_hud)
            .add_systems(Update, (
                update_cat_tracker,
                update_next_level_prompt,
            ).run_if(in_state(AppState::Playing)).after(game::check_start_next_level));
    }
}

#[derive(Component)]
struct HudRoot;

#[derive(Component)]
struct CatTracker;

#[derive(Component)]
struct NextLevelText;

fn setup_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    rooti(c_root, &asset_server, &mut commands, HudRoot, |p| {
        node(c_cat_tracker, p, |p| {
            image(c_cat_face, p);
            // TODO: Add a drop shadow to the text.
            texti("00/00", c_tracker_text, c_font_tracker, CatTracker, p);
        });
        node(c_next_level, p, |p| {
            // TODO: Add a drop shadow to the text.
            texti("Cats Corralled!\nPress Enter to start the next level", c_next_level_text, c_font_next_level, NextLevelText, p);
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
    mut next_level: EventReader<NextLevelEvent>,
    cat_stats: Res<CatStats>,
    mut next_level_q: Query<&mut Visibility, With<NextLevelText>>,
) {
    if !cat_stats.is_changed() {
        return;
    }

    if !next_level.is_empty() {
        // Toggle visibility for next level prompt.
        for mut vis in next_level_q.iter_mut()  {
            *vis = Visibility::Hidden;
        }
    } else if cat_stats.all_penned() {
        // Toggle visibility for next level prompt.
        for mut vis in next_level_q.iter_mut()  {
            *vis = Visibility::Inherited;
        }
    }
    next_level.clear();
}
