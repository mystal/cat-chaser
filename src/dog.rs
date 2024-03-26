use bevy::prelude::*;
use bevy_asepritesheet::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};
use bevy_rapier2d::prelude::RapierContext;

use crate::{
    WORLD_SIZE, AppState,
    assets::SfxAssets,
    cats::{Cat, CatState},
    input::PlayerInput,
    physics::{self, groups, ColliderBundle, MovementBounds, Velocity},
    utils::Blink,
};

// TODO: Kinda sucks to hard-code these, but I'm too lazy to figure out how to pipe in them right
// now.
pub const IDLE_ANIM: AnimHandle = AnimHandle::from_index(1);
pub const RUN_ANIM: AnimHandle = AnimHandle::from_index(0);

pub struct DogPlugin;

impl Plugin for DogPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                (tick_recovery, check_dog_hit).before(dog_movement).chain(),
                dog_movement.before(physics::update_movement),
                dog_animation.after(dog_movement),
                dog_bark,
            ).run_if(in_state(AppState::Playing)));
    }
}

#[derive(Component)]
pub struct Dog {
    speed: f32,
    recovery_timer: Timer,
}

impl Dog {
    fn start_recovery(&mut self) {
        self.recovery_timer.reset();
        self.recovery_timer.unpause();
    }

    pub fn is_recovering(&self) -> bool {
        !self.recovery_timer.paused() && !self.recovery_timer.finished()
    }
}

#[derive(Bundle)]
pub struct DogBundle {
    name: Name,
    dog: Dog,
    sprite: AnimatedSpriteBundle,
    velocity: Velocity,
    collider: ColliderBundle,
    input: PlayerInput,
    bounds: MovementBounds,
    blink: Blink,
}

impl DogBundle {
    pub fn new(pos: Vec2, spritesheet: Handle<Spritesheet>) -> Self {
        let mut recovery_timer = Timer::from_seconds(0.5, TimerMode::Once);
        recovery_timer.pause();
        Self {
            name: Name::new("Dog"),
            dog: Dog {
                speed: 150.0,
                recovery_timer,
            },
            sprite: AnimatedSpriteBundle {
                animator: SpriteAnimator::from_anim(IDLE_ANIM),
                spritesheet,
                sprite_bundle: SpriteSheetBundle {
                    sprite: Sprite {
                        flip_x: true,
                        ..default()
                    },
                    transform: Transform::from_translation(pos.extend(3.0)),
                    ..default()
                },
                ..default()
            },
            velocity: Velocity::default(),
            collider: ColliderBundle::rect(Vec2::new(30.0, 30.0), groups::DOG, groups::CAT),
            input: PlayerInput::default(),
            bounds: MovementBounds {
                min: -(WORLD_SIZE.as_vec2() / 2.0) + Vec2::new(0.0, 0.0),
                max: (WORLD_SIZE.as_vec2() / 2.0) - Vec2::new(0.0, 0.0),
            },
            blink: Blink::from_seconds(0.05, false),
        }
    }
}

fn dog_movement(
    mut dog_q: Query<(&Dog, &PlayerInput, &mut Velocity)>,
) {
    for (dog, input, mut velocity) in dog_q.iter_mut() {
        velocity.inner = input.movement * dog.speed;
    }
}

fn check_dog_hit(
    audio: Res<Audio>,
    sounds: Res<SfxAssets>,
    mut dog_q: Query<(Entity, &mut Dog, &mut Blink)>,
    cat_q: Query<&Cat, Without<Dog>>,
    rapier_ctx: Res<RapierContext>,
) {
    for (entity, mut dog, mut blink) in dog_q.iter_mut() {
        if dog.is_recovering() {
            continue;
        }

        for (collider1, collider2, intersecting) in rapier_ctx.intersection_pairs_with(entity) {
            if !intersecting {
                // TODO: Is this right? Confusing API.
                continue;
            }

            if let Ok(cat) = cat_q.get(collider1) {
                if matches!(cat.state, CatState::Cannonballing { .. }) {
                    dog.start_recovery();
                    blink.enable();
                    audio.play(sounds.dog_yip.clone());
                    break;
                }
            }
            if let Ok(cat) = cat_q.get(collider2) {
                if matches!(cat.state, CatState::Cannonballing { .. }) {
                    dog.start_recovery();
                    blink.enable();
                    audio.play(sounds.dog_yip.clone());
                    break;
                }
            }
        }
    }
}

fn tick_recovery(
    time: Res<Time>,
    mut dog_q: Query<(&mut Dog, &mut Blink)>,
) {
    let dt = time.delta();
    for (mut dog, mut blink) in dog_q.iter_mut() {
        if dog.recovery_timer.tick(dt).just_finished() {
            dog.recovery_timer.pause();
            blink.disable();
        }
    }
}

fn dog_animation(
    mut dog_q: Query<(&mut SpriteAnimator, &mut Sprite, &Velocity), With<Dog>>,
) {
    // Update which animation is playing based on movement.
    for (mut animator, mut sprite, velocity) in dog_q.iter_mut() {
        if **velocity == Vec2::ZERO {
            if !animator.is_cur_anim(IDLE_ANIM) {
                animator.set_anim(IDLE_ANIM);
            }
        } else {
            if !animator.is_cur_anim(RUN_ANIM) {
                animator.set_anim(RUN_ANIM);
            }
            if velocity.x != 0.0 {
                sprite.flip_x = velocity.x > 0.0;
            }
        }
    }
}

fn dog_bark(
    audio: Res<Audio>,
    sfx: Res<SfxAssets>,
    dog_q: Query<&PlayerInput, With<Dog>>,
) {
    let bark = dog_q.get_single()
        .map(|input| input.bark)
        .unwrap_or(false);
    if bark {
        audio.play(sfx.dog_woof.clone());
    }
}
