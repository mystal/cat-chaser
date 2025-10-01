use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_egui::input::egui_wants_any_input;

use crate::{
    SCREEN_SIZE, AppState,
    assets::GameAssets,
    cats,
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
            .add_systems(Update, handle_menu_input.run_if(not(egui_wants_any_input)));
    }
}

#[derive(Component)]
pub struct MenuRoot;

fn show_start(
    mut commands: Commands,
    assets: Res<GameAssets>,
) {
    let start_image = ImageNode {
        image: assets.start_menu.clone(),
        ..default()
    };
    let start_text = (
        Blink::from_seconds(0.5, true),
        Text("Press Enter to play!".into()),
        TextColor(Color::WHITE),
        TextFont {
            font: assets.font.clone(),
            font_size: 13.0,
            ..default()
        },
        TextShadow {
            offset: Vec2::splat(0.8),
            color: Color::BLACK,
        },
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(4.0),
            bottom: Val::Px(6.0),
            ..default()
        },
    );
    commands.spawn((
        MenuRoot,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        children![
            start_image,
            start_text,
        ],
    ));
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
        commands.entity(entity).despawn();
    }
}

fn show_credits(
    mut commands: Commands,
    assets: Res<GameAssets>,
) {
    // Spawn background for the menu.
    commands.spawn((
        Name::new("BackgroundSprite"),
        Sprite {
            color: Color::WHITE,
            custom_size: Some(SCREEN_SIZE.as_vec2()),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -1.0),
    ));

    // Spawn animated sprites in world.
    commands.spawn((
        Name::new("LindaSprite"),
        Transform::from_xyz(-170.0, 200.0, 0.0)
            .with_scale(Vec3::new(3.0, 3.0, 1.0)),
        Sprite {
            flip_x: true,
            ..default()
        },
        AseAnimation {
            aseprite: assets.basic_cat.clone(),
            animation: Animation::default()
                .with_tag("jam"),
            ..default()
        },
    ));
    commands.spawn((
        Name::new("MorganSprite"),
        Transform::from_xyz(-170.0, 100.0, 0.0)
            .with_scale(Vec3::new(3.0, 3.0, 1.0)),
        Sprite {
            flip_x: true,
            ..default()
        },
        AseAnimation {
            aseprite: assets.kitten.clone(),
            animation: Animation::default()
                .with_tag("scarf"),
            ..default()
        },
    ));
    commands.spawn((
        Name::new("JustinSprite"),
        Transform::from_xyz(-170.0, 0.0, 0.0)
            .with_scale(Vec3::new(3.0, 3.0, 1.0)),
        Sprite {
            flip_x: true,
            ..default()
        },
        AseAnimation {
            aseprite: assets.basic_cat.clone(),
            animation: Animation::default()
                .with_tag("spin"),
            ..default()
        },
    ));
    commands.spawn((
        Name::new("GabeSprite"),
        Transform::from_xyz(-170.0, -100.0, 0.0)
            .with_scale(Vec3::new(3.0, 3.0, 1.0)),
        Sprite {
            flip_x: true,
            ..default()
        },
        AseAnimation {
            aseprite: assets.wizard_dog.clone(),
            animation: Animation::default()
                .with_tag("run_back"),
            ..default()
        },
    ));
    commands.spawn((
        Name::new("ThamindaSprite"),
        Transform::from_xyz(-170.0, -200.0, 0.0)
            .with_scale(Vec3::new(3.0, 3.0, 1.0)),
        Sprite {
            flip_x: true,
            ..default()
        },
        AseAnimation {
            aseprite: assets.fox.clone(),
            animation: Animation::default()
                .with_tag("fox"),
            ..default()
        },
    ));

    let return_text = (
        Name::new("ReturnText"),
        Blink::from_seconds(0.5, true),
        Text("Press Tab to return!".into()),
        TextColor(Color::BLACK),
        TextFont {
            font: assets.font.clone(),
            font_size: 13.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(4.0),
            bottom: Val::Px(6.0),
            ..default()
        },
    );
    commands.spawn((
        MenuRoot,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        children![
            credits_name_text("Linda Cai", vec2(105.0, 34.0), assets.font.clone()),
            credits_name_text("Morgan Tenney", vec2(105.0, 68.0), assets.font.clone()),
            credits_name_text("Justin Hamilton", vec2(105.0, 98.0), assets.font.clone()),
            credits_name_text("Gabriel Martinez", vec2(105.0, 132.0), assets.font.clone()),
            credits_name_text("Music by Thaminda Edirisooriya", vec2(105.0, 163.0), assets.font.clone()),
            return_text,
        ],
    ));
}

fn credits_name_text(name: &str, pos: Vec2, font: Handle<Font>) -> impl Bundle {
    (
        Text(name.into()),
        TextColor(Color::BLACK),
        TextFont {
            font,
            font_size: 11.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            align_self: AlignSelf::Center,
            left: Val::Px(pos.x),
            top: Val::Px(pos.y),
            ..default()
        },
    )
}

fn clear_credits(
    mut commands: Commands,
    root_q: Query<Entity, With<MenuRoot>>,
    sprite_q: Query<Entity, With<Sprite>>,
) {
    for entity in root_q.iter() {
        commands.entity(entity).despawn();
    }
    for entity in sprite_q.iter() {
        commands.entity(entity).despawn();
    }
}

fn show_how_to_play(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    assets: Res<GameAssets>,
) {
    // Ideally this would all be done in UI, but UI renders on top of the world...
    // And we don't (yet) have animated sprites in UI.
    commands.spawn((
        Sprite {
            image: assets.how_to_play.clone(),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -1.0),
    ));

    // Spawn sprites for the dog.
    commands.spawn((
        Name::new("Dog"),
        Transform::from_xyz(270.0, 240.0, 0.0)
            .with_scale(Vec3::new(4.0, 4.0, 1.0)),
        Sprite::default(),
        AseAnimation {
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
        Transform::from_xyz(-30.0, -25.0, 0.0)
            .with_scale(Vec3::new(3.0, 3.0, 1.0)),
        Sprite {
            color,
            ..default()
        },
        AseAnimation {
            aseprite: assets.basic_cat.clone(),
            animation: Animation::default()
                .with_tag("idle"),
            ..default()
        },
    ));
    let color = Color::srgb_from_array(cats::CAT_COLORS[3]);
    commands.spawn((
        Name::new("FatCat"),
        Transform::from_xyz(70.0, -25.0, 0.0)
            .with_scale(Vec3::new(3.0, 3.0, 1.0)),
        Sprite {
            color,
            ..default()
        },
        AseAnimation {
            aseprite: assets.fat_cat.clone(),
            animation: Animation::default()
                .with_tag("idle"),
            ..default()
        },
    ));
    let color = Color::srgb_from_array(cats::CAT_COLORS[2]);
    commands.spawn((
        Name::new("Kitten"),
        Transform::from_xyz(165.0, -25.0, 0.0)
            .with_scale(Vec3::new(3.0, 3.0, 1.0)),
        Sprite {
            color,
            ..default()
        },
        AseAnimation {
            aseprite: assets.kitten.clone(),
            animation: Animation::default()
                .with_tag("idle"),
            ..default()
        },
    ));

    commands.spawn((
        MenuRoot,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        children![(
            Blink::from_seconds(0.5, true),
            Text("Press Enter to play!".into()),
            TextColor(Color::BLACK),
            TextFont {
                font: assets.font.clone(),
                font_size: 13.0,
                ..default()
            },
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(4.0),
                bottom: Val::Px(6.0),
                ..default()
            },
        )],
    ));
}

fn clear_how_to_play(
    mut commands: Commands,
    root_q: Query<Entity, With<MenuRoot>>,
    sprite_q: Query<Entity, With<Sprite>>,
) {
    for entity in root_q.iter() {
        commands.entity(entity).despawn();
    }
    for entity in sprite_q.iter() {
        commands.entity(entity).despawn();
    }
}
