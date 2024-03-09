use bevy::prelude::*;
use bevy_asepritesheet::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};

use crate::{
    GAME_SIZE, AppState,
    assets::SfxAssets,
    input::PlayerInput,
    physics::{self, groups, ColliderBundle, MovementBounds, Velocity},
};

// TODO: Kinda sucks to hard-code these, but I'm too lazy to figure out how to pipe in them right
// now.
const IDLE_ANIM: AnimHandle = AnimHandle::from_index(1);
const RUN_ANIM: AnimHandle = AnimHandle::from_index(0);

pub struct DogPlugin;

impl Plugin for DogPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                dog_movement.before(physics::update_movement),
                dog_animation.after(dog_movement),
                dog_bark,
            ).run_if(in_state(AppState::Playing)));
    }
}

#[derive(Component)]
pub struct Dog {
    speed: f32,
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
}

impl DogBundle {
    pub fn new(pos: Vec2, spritesheet: Handle<Spritesheet>) -> Self {
        Self {
            name: Name::new("Dog"),
            dog: Dog {
                speed: 150.0,
            },
            sprite: AnimatedSpriteBundle {
                animator: SpriteAnimator::from_anim(IDLE_ANIM),
                spritesheet,
                sprite_bundle: SpriteSheetBundle {
                    transform: Transform::from_translation(pos.extend(2.0)),
                    ..default()
                },
                ..default()
            },
            velocity: Velocity::default(),
            collider: ColliderBundle::rect(Vec2::new(30.0, 30.0), groups::DOG, groups::CAT),
            input: PlayerInput::default(),
            bounds: MovementBounds {
                min: -(GAME_SIZE.as_vec2() / 2.0) + Vec2::new(0.0, 0.0),
                max: (GAME_SIZE.as_vec2() / 2.0) - Vec2::new(0.0, 0.0),
            },
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

fn dog_animation(
    mut dog_q: Query<(&mut SpriteAnimator, &mut TextureAtlasSprite, &Velocity), With<Dog>>,
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
