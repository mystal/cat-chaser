use bevy::prelude::*;
use bevy_aseprite::{Aseprite, AsepriteBundle, anim::AsepriteAnimation};
use bevy_kira_audio::{Audio, AudioControl};

use crate::{
    GAME_SIZE, AppState,
    assets::SfxAssets,
    input::PlayerInput,
    physics::{self, groups, ColliderBundle, MovementBounds, Velocity},
};

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
    // animated sprite, input
    name: Name,
    dog: Dog,
    sprite: AsepriteBundle,
    velocity: Velocity,
    collider: ColliderBundle,
    input: PlayerInput,
    bounds: MovementBounds,
    /*
    pub facing: Facing,

    pub dog_state: DogState,
    pub hit_time: f32,
    pub hit_frame: u32,

    pub yip_sound: Sound,
    pub woof_sound: Sound,
    */
}

impl DogBundle {
    pub fn new(pos: Vec2, sprite: Handle<Aseprite>) -> Self {
        Self {
            name: Name::new("Dog"),
            dog: Dog {
                speed: 150.0,
                // anim: DogAnim::Idle,
            },
            sprite: AsepriteBundle {
                aseprite: sprite,
                animation: AsepriteAnimation::from("idle_front"),
                transform: Transform::from_translation(pos.extend(2.0)),
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
    mut dog_q: Query<(&mut AsepriteAnimation, &mut TextureAtlasSprite, &Velocity), With<Dog>>,
) {
    // Update which animation is playing based on movement.
    for (mut anim, mut sprite, velocity) in dog_q.iter_mut() {
        // TODO: Trying to debug why the wrong frame is used in run_front.
        // trace!("Dog frame: {}", anim.current_frame());
        if velocity.inner.x == 0.0 {
            if !anim.is_tag("idle_front") {
                *anim = AsepriteAnimation::from("idle_front");
            }
        } else {
            if !anim.is_tag("run_front") {
                *anim = AsepriteAnimation::from("run_front");
            }
            sprite.flip_x = velocity.inner.x > 0.0;
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
