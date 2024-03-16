use bevy::prelude::*;
use bevy_ui_dsl::*;

use crate::{
    AppState,
    cats::{Cat, CatState},
    ui::classes::*,
};

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Playing), setup_hud)
            .add_systems(OnExit(AppState::Playing), destroy_hud)
            .add_systems(Update, update_cat_tracker.run_if(in_state(AppState::Playing)));
    }
}

#[derive(Component)]
struct HudRoot;

#[derive(Component)]
struct CatTracker;

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
    cats_q: Query<&Cat>,
) {
    // TODO: Update on CatState changes instead of every frame?
    let cats = cats_q.iter().count();
    let cats_in_pen = cats_q.iter()
        .filter(|cat| cat.state == CatState::InPen)
        .count();
    for mut tracker_text in tracker_q.iter_mut()  {
        tracker_text.sections[0].value = format!("{:02}/{:02}", cats_in_pen, cats);
    }
}
