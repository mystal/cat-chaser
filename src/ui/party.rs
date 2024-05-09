use bevy::prelude::*;
use bevy_asepritesheet::prelude::*;

use crate::{
    WORLD_SIZE,
    assets::GameAssets,
    cats::{self, CatKind},
    dog,
    game::GameState,
};

const NUM_CATS: u32 = 60;
const PARTY_ITEM_SPEED: f32 = 30.0;

pub struct PartyPlugin;

impl Plugin for PartyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Victory), spawn_party)
            .add_systems(OnExit(GameState::Victory), destroy_party)
            .add_systems(Update, update_cats.run_if(in_state(GameState::Victory)));
    }
}

// TODO: Cat

#[derive(Component)]
struct PartyCat {
    kind: CatKind,
}

#[derive(Component)]
struct PartyDog;

fn spawn_party(
    mut commands: Commands,
    assets: Res<GameAssets>,
) {
    // Spawn doggo!
    commands.spawn((
        Name::new("PartyDog"),
        PartyDog,
        AnimatedSpriteBundle {
            animator: SpriteAnimator::from_anim(dog::RUN_ANIM),
            sprite_bundle: SpriteSheetBundle {
                transform: Transform::from_translation(Vec3::new(0.0, 20.0, 200.0))
                    .with_scale(Vec3::new(7.0, 7.0, 1.0)),
                ..default()
            },
            spritesheet: assets.wizard_dog.clone(),
            ..default()
        },
    ));

    // Spawn cats!
    for i in 0..NUM_CATS {
        let kind = CatKind::random();
        let angle = 360.0 * fastrand::f32();
        let x = (WORLD_SIZE.x as f32 * fastrand::f32()) - WORLD_SIZE.x as f32 / 2.0;
        let y = ((WORLD_SIZE.y + 80) as f32 * fastrand::f32()) + WORLD_SIZE.y as f32 / 2.0;
        let z = 100.0 + i as f32;
        let spritesheet = match kind {
            CatKind::Basic => &assets.basic_cat,
            CatKind::Kitten => &assets.kitten,
            CatKind::Chonk => &assets.fat_cat,
        }.clone();

        commands.spawn((
            Name::new("PartyCat!"),
            PartyCat {
                kind,
            },
            AnimatedSpriteBundle {
                animator: SpriteAnimator::from_anim(cats::IDLE_ANIM),
                sprite_bundle: SpriteSheetBundle {
                    sprite: Sprite {
                        color: cats::random_cat_color(),
                        flip_x: fastrand::bool(),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(x, y, z))
                        .with_rotation(Quat::from_rotation_z(angle.to_radians()))
                        .with_scale(Vec3::new(1.5, 1.5, 1.0)),
                    ..default()
                },
                spritesheet,
                ..default()
            },
        ));
    }
}

fn destroy_party(
    mut commands: Commands,
    cat_q: Query<Entity, With<PartyCat>>,
    dog_q: Query<Entity, With<PartyDog>>,
) {
    // Despawn cats.
    for cat in cat_q.iter() {
        commands.entity(cat).despawn_recursive();
    }

    // Despawn dog.
    for dog in dog_q.iter() {
        commands.entity(dog).despawn_recursive();
    }
}

fn update_cats(
    time: Res<Time>,
    mut cat_q: Query<(&PartyCat, &mut Transform)>,
) {
    let dt = time.delta_seconds();
    for (cat, mut transform) in cat_q.iter_mut() {
        // Update rotation
        let rotation = match cat.kind {
            CatKind::Basic => 180.0 * dt,
            CatKind::Kitten => 360.0 * dt,
            CatKind::Chonk => 90.0 * dt,
        };
        transform.rotate_local_z(-rotation.to_radians());

        // Update position.
        transform.translation.y -= PARTY_ITEM_SPEED * dt;

        // Wrap around once hit the bottom of the screen.
        if transform.translation.y < -(WORLD_SIZE.y as f32 / 2.0) - 50.0 {
            let angle = 360.0 * fastrand::f32();
            let x = (WORLD_SIZE.x as f32 * fastrand::f32()) - WORLD_SIZE.x as f32 / 2.0;
            let y = (WORLD_SIZE.y as f32 / 2.0) + 50.0;
            transform.rotation = Quat::from_rotation_z(angle.to_radians());
            transform.translation.x = x;
            transform.translation.y = y;
        }
    }
}
