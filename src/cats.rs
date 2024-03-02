use bevy::prelude::*;
use bevy_aseprite::{Aseprite, AsepriteBundle, anim::AsepriteAnimation};

use crate::{
    GAME_SIZE,
    physics::{groups, ColliderBundle, MovementBounds, Velocity},
};

pub struct CatsPlugin;

impl Plugin for CatsPlugin {
    fn build(&self, app: &mut App) {
    }
}

#[derive(Bundle)]
struct CatBundle {
    name: Name,
    sprite: AsepriteBundle,
    velocity: Velocity,
    collider: ColliderBundle,
    bounds: MovementBounds,
}

impl CatBundle {
    fn new(name: &'static str, pos: Vec2, sprite: Handle<Aseprite>) -> Self {
        Self {
            name: Name::new(name),
            sprite: AsepriteBundle {
                aseprite: sprite,
                animation: AsepriteAnimation::from("idle"),
                transform: Transform::from_translation(pos.extend(1.0)),
                ..default()
            },
            velocity: Velocity::default(),
            collider: ColliderBundle::rect(Vec2::new(30.0, 30.0), groups::CAT, groups::DOG | groups::CATBOX),
            bounds: MovementBounds {
                min: -(GAME_SIZE.as_vec2() / 2.0) + Vec2::new(0.0, 0.0),
                max: (GAME_SIZE.as_vec2() / 2.0) - Vec2::new(0.0, 0.0),
            },
        }
    }
}

#[derive(Component)]
pub struct BasicCat;

#[derive(Bundle)]
pub struct BasicCatBundle {
    cat: CatBundle,
}

impl BasicCatBundle {
    pub fn new(pos: Vec2, sprite: Handle<Aseprite>) -> Self {
        Self {
            cat: CatBundle::new("BasicCat", pos, sprite),
        }
    }
}

#[derive(Component)]
pub struct KittenCat;

#[derive(Bundle)]
pub struct KittenBundle {
    cat: CatBundle,
}

impl KittenBundle {
    pub fn new(pos: Vec2, sprite: Handle<Aseprite>) -> Self {
        Self {
            cat: CatBundle::new("Kitten", pos, sprite),
        }
    }
}

#[derive(Component)]
pub struct ChonkCat;

#[derive(Bundle)]
pub struct ChonkCatBundle {
    cat: CatBundle,
}

impl ChonkCatBundle {
    pub fn new(pos: Vec2, sprite: Handle<Aseprite>) -> Self {
        Self {
            cat: CatBundle::new("ChonkCat", pos, sprite),
        }
    }
}
