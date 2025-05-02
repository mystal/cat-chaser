use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_ui_dsl::*;

use crate::{
    SCREEN_SIZE, AppState,
    assets::GameAssets,
    cats,
    ui::classes::*,
    utils::Blink,
};

pub struct MenusPlugin;

impl Plugin for MenusPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::StartMenu), show_start)
            .add_systems(OnExit(AppState::StartMenu), clear_start)
            .add_systems(OnEnter(AppState::Credits), show_credits)
            .add_systems(OnExit(AppState::Credits), clear_credits)
            .add_systems(OnEnter(AppState::HowToPlay), show_how_to_play)
            .add_systems(OnExit(AppState::HowToPlay), clear_how_to_play)
            .add_systems(Update, handle_menu_input);
    }
}

#[derive(Component)]
pub struct MenuRoot;

fn show_start(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    rooti(c_root, &asset_server, &mut commands, MenuRoot, |p| {
        image(c_start_image, p);

        let blink = Blink::from_seconds(0.5, true);
        // TODO: Add a drop shadow to the text.
        texti("Press Enter to play!", c_start_text, c_font_start, blink, p);
    });
}

fn handle_menu_input(
    keys: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    match app_state.get() {
        AppState::Loading => {}
        AppState::StartMenu => {
            if keys.just_pressed(KeyCode::Tab) {
                next_state.set(AppState::Credits);
            } else if keys.just_pressed(KeyCode::Enter) {
                next_state.set(AppState::HowToPlay);
            }
        }
        AppState::Credits => {
            if keys.any_just_pressed([KeyCode::Tab, KeyCode::Enter]) {
                next_state.set(AppState::StartMenu);
            }
        }
        AppState::HowToPlay => {
            if keys.just_pressed(KeyCode::Enter) {
                next_state.set(AppState::Playing);
            }
        }
        AppState::Playing => {}
    }
}

fn clear_start(
    mut commands: Commands,
    root_q: Query<Entity, With<MenuRoot>>,
) {
    for entity in root_q.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn show_credits(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    assets: Res<GameAssets>,
) {
    // Spawn background for the menu.
    commands.spawn((
        Name::new("BackgroundSprite"),
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(SCREEN_SIZE.as_vec2()),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)),
            ..default()
        },
    ));

    // Spawn animated sprites in world.
    commands.spawn((
        Name::new("LindaSprite"),
        AsepriteAnimationBundle {
            transform: Transform::from_translation(Vec3::new(-170.0, 200.0, 0.0))
                .with_scale(Vec3::new(3.0, 3.0, 1.0)),
            sprite: Sprite {
                flip_x: true,
                ..default()
            },
            aseprite: assets.basic_cat.clone(),
            animation: Animation::default()
                .with_tag("jam"),
            ..default()
        },
    ));
    commands.spawn((
        Name::new("MorganSprite"),
        AsepriteAnimationBundle {
            transform: Transform::from_translation(Vec3::new(-170.0, 100.0, 0.0))
                .with_scale(Vec3::new(3.0, 3.0, 1.0)),
            sprite: Sprite {
                flip_x: true,
                ..default()
            },
            aseprite: assets.kitten.clone(),
            animation: Animation::default()
                .with_tag("scarf"),
            ..default()
        },
    ));
    commands.spawn((
        Name::new("JustinSprite"),
        AsepriteAnimationBundle {
            transform: Transform::from_translation(Vec3::new(-170.0, 0.0, 0.0))
                .with_scale(Vec3::new(3.0, 3.0, 1.0)),
            sprite: Sprite {
                flip_x: true,
                ..default()
            },
            aseprite: assets.basic_cat.clone(),
            animation: Animation::default()
                .with_tag("spin"),
            ..default()
        },
    ));
    commands.spawn((
        Name::new("GabeSprite"),
        AsepriteAnimationBundle {
            transform: Transform::from_translation(Vec3::new(-170.0, -100.0, 0.0))
                .with_scale(Vec3::new(3.0, 3.0, 1.0)),
            sprite: Sprite {
                flip_x: true,
                ..default()
            },
            aseprite: assets.wizard_dog.clone(),
            animation: Animation::default()
                .with_tag("run_back"),
            ..default()
        },
    ));
    commands.spawn((
        Name::new("ThamindaSprite"),
        AsepriteAnimationBundle {
            transform: Transform::from_translation(Vec3::new(-170.0, -200.0, 0.0))
                .with_scale(Vec3::new(3.0, 3.0, 1.0)),
            sprite: Sprite {
                flip_x: true,
                ..default()
            },
            aseprite: assets.fox.clone(),
            animation: Animation::default()
                .with_tag("fox"),
            ..default()
        },
    ));

    let name = Name::new("UiRoot");
    rooti(c_root, &asset_server, &mut commands, (name, MenuRoot), |p| {
        // Spawn credits text in UI.
        text("Linda Cai", c_credits_text_linda, c_font_credits, p);
        text("Morgan Tenney", c_credits_text_morgan, c_font_credits, p);
        text("Justin Hamilton", c_credits_text_justin, c_font_credits, p);
        text("Gabriel Martinez", c_credits_text_gabe, c_font_credits, p);
        text("Music by Thaminda Edirisooriya", c_credits_text_thaminda, c_font_credits, p);

        let name = Name::new("ReturnText");
        let blink = Blink::from_seconds(0.5, true);
        // TODO: Add a drop shadow to the text.
        texti("Press Tab to return!", c_start_text, c_font_how_to_play, (name, blink), p);
    });
}

fn clear_credits(
    mut commands: Commands,
    root_q: Query<Entity, With<MenuRoot>>,
    sprite_q: Query<Entity, With<Sprite>>,
) {
    for entity in root_q.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in sprite_q.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn show_how_to_play(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    assets: Res<GameAssets>,
) {
    // Ideally this would all be done in UI, but UI renders on top of the world...
    // And we don't (yet) have animated sprites in UI.
    commands.spawn(SpriteBundle {
        texture: assets.how_to_play.clone(),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)),
        ..default()
    });

    // Spawn sprites for the dog.
    commands.spawn((
        Name::new("Dog"),
        AsepriteAnimationBundle {
            transform: Transform::from_translation(Vec3::new(270.0, 240.0, 0.0))
                .with_scale(Vec3::new(4.0, 4.0, 1.0)),
            aseprite: assets.wizard_dog.clone(),
            animation: Animation::default()
                .with_tag("idle_front"),
            ..default()
        },
    ));

    // ... and cats.
    // TODO: Scaling them 3x doesn't look good at this resolution. Get artifacts :/
    let color = Color::srgb_from_array(cats::CAT_COLORS[0]);
    commands.spawn((
        Name::new("BasicCat"),
        AsepriteAnimationBundle {
            transform: Transform::from_translation(Vec3::new(-30.0, -25.0, 0.0))
                .with_scale(Vec3::new(3.0, 3.0, 1.0)),
            sprite: Sprite {
                color,
                ..default()
            },
            aseprite: assets.basic_cat.clone(),
            animation: Animation::default()
                .with_tag("idle"),
            ..default()
        },
    ));
    let color = Color::srgb_from_array(cats::CAT_COLORS[3]);
    commands.spawn((
        Name::new("FatCat"),
        AsepriteAnimationBundle {
            transform: Transform::from_translation(Vec3::new(70.0, -25.0, 0.0))
                .with_scale(Vec3::new(3.0, 3.0, 1.0)),
            sprite: Sprite {
                color,
                ..default()
            },
            aseprite: assets.fat_cat.clone(),
            animation: Animation::default()
                .with_tag("idle"),
            ..default()
        },
    ));
    let color = Color::srgb_from_array(cats::CAT_COLORS[2]);
    commands.spawn((
        Name::new("Kitten"),
        AsepriteAnimationBundle {
            transform: Transform::from_translation(Vec3::new(165.0, -25.0, 0.0))
                .with_scale(Vec3::new(3.0, 3.0, 1.0)),
            sprite: Sprite {
                color,
                ..default()
            },
            aseprite: assets.kitten.clone(),
            animation: Animation::default()
                .with_tag("idle"),
            ..default()
        },
    ));

    rooti(c_root, &asset_server, &mut commands, MenuRoot, |p| {
        let blink = Blink::from_seconds(0.5, true);
        // TODO: Add a drop shadow to the text.
        texti("Press Enter to play!", c_start_text, c_font_how_to_play, blink, p);
    });
}

fn clear_how_to_play(
    mut commands: Commands,
    root_q: Query<Entity, With<MenuRoot>>,
    sprite_q: Query<Entity, With<Sprite>>,
) {
    for entity in root_q.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in sprite_q.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
