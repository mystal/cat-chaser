use bevy::prelude::*;
use bevy_asepritesheet::prelude::*;
use bevy_rapier2d::prelude::RapierContext;

use crate::{
    GAME_SIZE, AppState,
    dog::Dog,
    game::CatBox,
    physics::{self, groups, ColliderBundle, MovementBounds, Velocity},
};

// TODO: Kinda sucks to hard-code these, but I'm too lazy to figure out how to pipe in them right
// now.
const IDLE_ANIM: AnimHandle = AnimHandle::from_index(1);
const WALK_ANIM: AnimHandle = AnimHandle::from_index(0);

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
    sprite: AnimatedSpriteBundle,
    velocity: Velocity,
    collider: ColliderBundle,
    bounds: MovementBounds,
}

impl CatBundle {
    fn new(name: &'static str, pos: Vec2, spritesheet: Handle<Spritesheet>) -> Self {
        Self {
            cat: Cat::default(),
            name: Name::new(name),
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
            collider: ColliderBundle::rect(Vec2::new(30.0, 30.0), groups::CAT, groups::DOG | groups::CATBOX),
            bounds: MovementBounds {
                min: -(GAME_SIZE.as_vec2() / 2.0) + Vec2::new(15.0, 15.0),
                max: (GAME_SIZE.as_vec2() / 2.0) - Vec2::new(15.0, 15.0),
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
    pub fn new(pos: Vec2, spritesheet: Handle<Spritesheet>) -> Self {
        Self {
            basic_cat: BasicCat,
            cat: CatBundle::new("BasicCat", pos, spritesheet),
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
    pub fn new(pos: Vec2, spritesheet: Handle<Spritesheet>) -> Self {
        Self {
            kitten: KittenCat,
            cat: CatBundle::new("Kitten", pos, spritesheet),
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
    pub fn new(pos: Vec2, spritesheet: Handle<Spritesheet>) -> Self {
        Self {
            chonk_cat: ChonkCat,
            cat: CatBundle::new("ChonkCat", pos, spritesheet),
        }
    }
}

fn update_cats(
    rapier_ctx: Res<RapierContext>,
    mut cat_q: Query<(Entity, &mut Cat, &Transform, &mut Velocity)>,
    dog_q: Query<&GlobalTransform, (With<Dog>, Without<Cat>)>,
    cat_box_q: Query<Entity, With<CatBox>>,
) {
    let dog_pos = dog_q.get_single()
        .map(|trans| trans.translation().truncate())
        .ok();
    let cat_box = cat_box_q.get_single().ok();
    for (entity, mut cat, transform, mut velocity) in cat_q.iter_mut() {
        let pos = transform.translation.truncate();

        let in_pen = cat_box.map(|cat_box|
            rapier_ctx.intersection_pair(entity, cat_box) == Some(true))
            .unwrap_or(false);
        let dog_in_range = dog_pos.map(|dog_pos|
            pos.distance_squared(dog_pos) < FLEE_RANGE.powi(2))
            .unwrap_or(false);
        let dog_out_of_range = dog_pos.map(|dog_pos|
            pos.distance_squared(dog_pos) > (FLEE_RANGE + FLEE_BUFFER).powi(2))
            .unwrap_or(false);

        // Update cat state first.
        match cat.state {
            CatState::Wander => {
                if in_pen {
                    cat.state = CatState::InPen;
                } else if dog_in_range {
                    cat.state = CatState::Flee;
                }
            },
            CatState::Flee => {
                if in_pen {
                    cat.state = CatState::InPen;
                } else if dog_out_of_range {
                    cat.state = CatState::Wander;
                }
            },
            CatState::Jittering => todo!(),
            CatState::Cannonballing => todo!(),
            CatState::InPen => {},
        }

        // Perform cat state logic.
        match cat.state {
            CatState::Wander => {
                **velocity = Vec2::ZERO;
            },
            CatState::Flee => {
                if let Some(dog_pos) = dog_pos {
                    let flee_dir = (pos - dog_pos).normalize_or_zero();
                    **velocity = flee_dir * 100.0;
                }
            },
            CatState::Jittering => todo!(),
            CatState::Cannonballing => todo!(),
            CatState::InPen => {
                **velocity = Vec2::ZERO;
            },
        }
    }
}

fn cat_animation(
    mut cat_q: Query<(&mut SpriteAnimator, &mut Sprite, &Cat, &Velocity)>,
) {
    // Update which animation is playing based on state and velocity.
    for (mut animator, mut sprite, cat, velocity) in cat_q.iter_mut() {
        match cat.state {
            CatState::Wander => {
                if **velocity == Vec2::ZERO {
                    if !animator.is_cur_anim(IDLE_ANIM) {
                        animator.set_anim(IDLE_ANIM);
                    }
                } else {
                    if !animator.is_cur_anim(WALK_ANIM) {
                        animator.set_anim(WALK_ANIM);
                    }
                    if velocity.x != 0.0 {
                        sprite.flip_x = velocity.x > 0.0;
                    }
                }
            },
            CatState::Flee => {
                if !animator.is_cur_anim(WALK_ANIM) {
                    animator.set_anim(WALK_ANIM);
                }
                if **velocity != Vec2::ZERO {
                    sprite.flip_x = velocity.x > 0.0;
                }
            },
            CatState::Jittering => todo!(),
            CatState::Cannonballing => todo!(),
            CatState::InPen => {
                if !animator.is_cur_anim(IDLE_ANIM) {
                    animator.set_anim(IDLE_ANIM);
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
    mut cat_q: Query<&mut Sprite, (Added<Sprite>, With<Cat>)>,
) {
    for mut sprite in cat_q.iter_mut() {
        let color = fastrand::choice(CAT_COLORS).unwrap();
        sprite.color = Color::rgb(color[0], color[1], color[2]);
    }
}
