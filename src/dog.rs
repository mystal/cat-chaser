use bevy::prelude::*;

use crate::{
    input::PlayerInput,
    physics::{self, groups, ColliderBundle, Velocity},
};

pub struct DogPlugin;

impl Plugin for DogPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, dog_movement.before(physics::update_movement));
    }
}

#[derive(Component)]
pub struct Dog {
    speed: f32,
}

#[derive(Bundle)]
pub struct DogBundle {
    // animated sprite, input
    dog: Dog,
    sprite: SpriteBundle,
    velocity: Velocity,
    collider: ColliderBundle,
    input: PlayerInput,
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
    pub fn new(pos: Vec2) -> Self {
        Self {
            dog: Dog {
                speed: 150.0,
            },
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(32.0, 32.0)),
                    ..default()
                },
                transform: Transform::from_translation(pos.extend(0.0)),
                ..default()
            },
            velocity: Velocity::default(),
            collider: ColliderBundle::rect(Vec2::new(30.0, 30.0), groups::DOG, groups::CAT),
            input: PlayerInput::default(),
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
