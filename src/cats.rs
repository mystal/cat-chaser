use bevy::prelude::*;
use bevy_aseprite::{Aseprite, AsepriteBundle, anim::AsepriteAnimation};

use crate::{
    GAME_SIZE, AppState,
    dog::Dog,
    physics::{self, groups, ColliderBundle, MovementBounds, Velocity},
};

const FLEE_RANGE: f32 = 70.0;
const FLEE_BUFFER: f32 = 10.0;

pub struct CatsPlugin;

impl Plugin for CatsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                update_cats.before(physics::update_movement),
                cat_animation.after(update_cats),
                init_cat_color,
            ).run_if(in_state(AppState::Playing)));
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum CatState {
    #[default]
    Wander,
    Flee,
    Jittering,
    Cannonballing,
    InPen,
}

#[derive(Default, Component)]
pub struct Cat {
    state: CatState,
}

#[derive(Bundle)]
struct CatBundle {
    cat: Cat,
    name: Name,
    sprite: AsepriteBundle,
    velocity: Velocity,
    collider: ColliderBundle,
    bounds: MovementBounds,
}

impl CatBundle {
    fn new(name: &'static str, pos: Vec2, sprite: Handle<Aseprite>) -> Self {
        Self {
            cat: Cat::default(),
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
    basic_cat: BasicCat,
    cat: CatBundle,
}

impl BasicCatBundle {
    pub fn new(pos: Vec2, sprite: Handle<Aseprite>) -> Self {
        Self {
            basic_cat: BasicCat,
            cat: CatBundle::new("BasicCat", pos, sprite),
        }
    }
}

#[derive(Component)]
pub struct KittenCat;

#[derive(Bundle)]
pub struct KittenBundle {
    kitten: KittenCat,
    cat: CatBundle,
}

impl KittenBundle {
    pub fn new(pos: Vec2, sprite: Handle<Aseprite>) -> Self {
        Self {
            kitten: KittenCat,
            cat: CatBundle::new("Kitten", pos, sprite),
        }
    }
}

#[derive(Component)]
pub struct ChonkCat;

#[derive(Bundle)]
pub struct ChonkCatBundle {
    chonk_cat: ChonkCat,
    cat: CatBundle,
}

impl ChonkCatBundle {
    pub fn new(pos: Vec2, sprite: Handle<Aseprite>) -> Self {
        Self {
            chonk_cat: ChonkCat,
            cat: CatBundle::new("ChonkCat", pos, sprite),
        }
    }
}

fn update_cats(
    mut cat_q: Query<(&mut Cat, &Transform, &mut Velocity)>,
    dog_q: Query<&GlobalTransform, (With<Dog>, Without<Cat>)>,
) {
    let dog_pos = dog_q.get_single()
        .map(|trans| trans.translation().truncate())
        .ok();
    for (mut cat, transform, mut velocity) in cat_q.iter_mut() {
        let pos = transform.translation.truncate();
        // TODO: Update state first and then run state logic.
        match cat.state {
            CatState::Wander => {
                // If dog is within a certain radius, switch to Flee.
                if let Some(dog_pos) = dog_pos {
                    if pos.distance_squared(dog_pos) < FLEE_RANGE.powi(2) {
                        cat.state = CatState::Flee;
                    }
                }

                **velocity = Vec2::ZERO;
            },
            CatState::Flee => {
                // If dog is outside certain radius, switch to Wander.
                if let Some(dog_pos) = dog_pos {
                    if pos.distance_squared(dog_pos) > (FLEE_RANGE + FLEE_BUFFER).powi(2) {
                        cat.state = CatState::Wander;
                    }
                }

                if let Some(dog_pos) = dog_pos {
                    let flee_dir = (pos - dog_pos).normalize_or_zero();
                    **velocity = flee_dir * 100.0;
                }
            },
            CatState::Jittering => todo!(),
            CatState::Cannonballing => todo!(),
            CatState::InPen => todo!(),
        }
    }
}

fn cat_animation(
    mut cat_q: Query<(&mut AsepriteAnimation, &mut TextureAtlasSprite, &Cat, &Velocity)>,
) {
    // Update which animation is playing based on state and velocity.
    for (mut anim, mut sprite, cat, velocity) in cat_q.iter_mut() {
        match cat.state {
            CatState::Wander => {
                if velocity.x == 0.0 {
                    if !anim.is_tag("idle") {
                        *anim = AsepriteAnimation::from("idle");
                    }
                } else {
                    if !anim.is_tag("walk") {
                        *anim = AsepriteAnimation::from("walk");
                    }
                    sprite.flip_x = velocity.x > 0.0;
                }
            },
            CatState::Flee => {
                if !anim.is_tag("walk") {
                    *anim = AsepriteAnimation::from("walk");
                }
                if velocity.x != 0.0 {
                    sprite.flip_x = velocity.x > 0.0;
                }
            },
            CatState::Jittering => todo!(),
            CatState::Cannonballing => todo!(),
            CatState::InPen => {
                if !anim.is_tag("idle") {
                    *anim = AsepriteAnimation::from("idle_front");
                }
            },
        }
    }
}

const CAT_COLORS: &[[f32; 3]] = &[
    [203.0 / 255.0, 219.0 / 255.0, 252.0 / 255.0], // The default purple blue
    [189.0 / 255.0, 245.0 / 255.0, 242.0 / 255.0], // Robin's egg blue-ish
    [174.0 / 255.0, 245.0 / 255.0, 184.0 / 255.0], // Pastel green.
    [255.0 / 255.0, 193.0 / 255.0, 229.0 / 255.0], // Not quite but sort of pink.
];

fn init_cat_color(
    mut cat_q: Query<&mut TextureAtlasSprite, (Added<TextureAtlasSprite>, With<Cat>)>,
) {
    for mut sprite in cat_q.iter_mut() {
        dbg!("blah");
        let color = fastrand::choice(CAT_COLORS).unwrap();
        sprite.color = Color::rgb(color[0], color[1], color[2]);
    }
}
