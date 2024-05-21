use bevy::prelude::*;
use bevy_asepritesheet::prelude::*;
use bevy_ui_dsl::*;

use crate::{
    SCREEN_SIZE, AppState,
    assets::GameAssets,
    cats,
    dog,
    ui::classes::*,
    utils::Blink,
};

const CAT_JAM_ANIM: AnimHandle = AnimHandle::from_index(3);
const KITTEN_SCARF_ANIM: AnimHandle = AnimHandle::from_index(2);
const CAT_SPIN_ANIM: AnimHandle = AnimHandle::from_index(4);
const DOG_RUN_BACK_ANIM: AnimHandle = AnimHandle::from_index(2);
const FOX_ANIM: AnimHandle = AnimHandle::from_index(0);

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
        AnimatedSpriteBundle {
            animator: SpriteAnimator::from_anim(CAT_JAM_ANIM),
            sprite_bundle: SpriteSheetBundle {
                sprite: Sprite {
                    flip_x: true,
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(-170.0, 200.0, 0.0))
                    .with_scale(Vec3::new(3.0, 3.0, 1.0)),
                ..default()
            },
            spritesheet: assets.basic_cat.clone(),
            ..default()
        },
    ));
    commands.spawn((
        Name::new("MorganSprite"),
        AnimatedSpriteBundle {
            animator: SpriteAnimator::from_anim(KITTEN_SCARF_ANIM),
            sprite_bundle: SpriteSheetBundle {
                sprite: Sprite {
                    flip_x: true,
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(-170.0, 100.0, 0.0))
                    .with_scale(Vec3::new(3.0, 3.0, 1.0)),
                ..default()
            },
            spritesheet: assets.kitten.clone(),
            ..default()
        },
    ));
    commands.spawn((
        Name::new("JustinSprite"),
        AnimatedSpriteBundle {
            animator: SpriteAnimator::from_anim(CAT_SPIN_ANIM),
            sprite_bundle: SpriteSheetBundle {
                sprite: Sprite {
                    flip_x: true,
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(-170.0, 0.0, 0.0))
                    .with_scale(Vec3::new(3.0, 3.0, 1.0)),
                ..default()
            },
            spritesheet: assets.basic_cat.clone(),
            ..default()
        },
    ));
    commands.spawn((
        Name::new("GabeSprite"),
        AnimatedSpriteBundle {
            animator: SpriteAnimator::from_anim(DOG_RUN_BACK_ANIM),
            sprite_bundle: SpriteSheetBundle {
                sprite: Sprite {
                    flip_x: true,
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(-170.0, -100.0, 0.0))
                    .with_scale(Vec3::new(3.0, 3.0, 1.0)),
                ..default()
            },
            spritesheet: assets.wizard_dog.clone(),
            ..default()
        },
    ));
    commands.spawn((
        Name::new("ThamindaSprite"),
        AnimatedSpriteBundle {
            animator: SpriteAnimator::from_anim(FOX_ANIM),
            sprite_bundle: SpriteSheetBundle {
                sprite: Sprite {
                    flip_x: true,
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(-170.0, -200.0, 0.0))
                    .with_scale(Vec3::new(3.0, 3.0, 1.0)),
                ..default()
            },
            spritesheet: assets.fox.clone(),
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
        AnimatedSpriteBundle {
            animator: SpriteAnimator::from_anim(dog::IDLE_ANIM),
            sprite_bundle: SpriteSheetBundle {
                transform: Transform::from_translation(Vec3::new(270.0, 240.0, 0.0))
                    .with_scale(Vec3::new(4.0, 4.0, 1.0)),
                ..default()
            },
            spritesheet: assets.wizard_dog.clone(),
            ..default()
        },
    ));

    // ... and cats.
    // TODO: Scaling them 3x doesn't look good at this resolution. Get artifacts :/
    let color = Color::rgb_from_array(cats::CAT_COLORS[0]);
    commands.spawn((
        Name::new("BasicCat"),
        AnimatedSpriteBundle {
            animator: SpriteAnimator::from_anim(cats::IDLE_ANIM),
            sprite_bundle: SpriteSheetBundle {
                sprite: Sprite {
                    color,
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(-30.0, -25.0, 0.0))
                    .with_scale(Vec3::new(3.0, 3.0, 1.0)),
                ..default()
            },
            spritesheet: assets.basic_cat.clone(),
            ..default()
        },
    ));
    let color = Color::rgb_from_array(cats::CAT_COLORS[3]);
    commands.spawn((
        Name::new("FatCat"),
        AnimatedSpriteBundle {
            animator: SpriteAnimator::from_anim(cats::IDLE_ANIM),
            sprite_bundle: SpriteSheetBundle {
                sprite: Sprite {
                    color,
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(70.0, -25.0, 0.0))
                    .with_scale(Vec3::new(3.0, 3.0, 1.0)),
                ..default()
            },
            spritesheet: assets.fat_cat.clone(),
            ..default()
        },
    ));
    let color = Color::rgb_from_array(cats::CAT_COLORS[2]);
    commands.spawn((
        Name::new("Kitten"),
        AnimatedSpriteBundle {
            animator: SpriteAnimator::from_anim(cats::IDLE_ANIM),
            sprite_bundle: SpriteSheetBundle {
                sprite: Sprite {
                    color,
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(165.0, -25.0, 0.0))
                    .with_scale(Vec3::new(3.0, 3.0, 1.0)),
                ..default()
            },
            spritesheet: assets.kitten.clone(),
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
