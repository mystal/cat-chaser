use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};
use bevy_rapier2d::prelude::ReadRapierContext;

use crate::{
    WORLD_SIZE, AppState,
    assets::SfxAssets,
    cats::{Cat, CatState},
    input::PlayerInput,
    physics::{self, groups, ColliderBundle, MovementBounds, Velocity},
    utils::Blink,
};

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

pub fn dog(pos: Vec2, aseprite: Handle<Aseprite>) -> impl Bundle {
    let mut recovery_timer = Timer::from_seconds(0.5, TimerMode::Once);
    recovery_timer.pause();
    (
        Name::new("Dog"),
        Dog {
            speed: 150.0,
            recovery_timer,
        },
        Transform::from_translation(pos.extend(3.0)),
        Sprite {
            flip_x: true,
            ..default()
        },
        AseAnimation {
            aseprite,
            animation: Animation::default()
                .with_tag("idle_front"),
        },
        Velocity::default(),
        ColliderBundle::rect(Vec2::new(30.0, 30.0), groups::DOG, groups::CAT),
        PlayerInput::default(),
        MovementBounds {
            min: -(WORLD_SIZE.as_vec2() / 2.0) + Vec2::new(0.0, 0.0),
            max: (WORLD_SIZE.as_vec2() / 2.0) - Vec2::new(0.0, 0.0),
        },
        Blink::from_seconds(0.05, false),
    )
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
    rapier_ctx: ReadRapierContext,
) {
    let rapier_ctx = rapier_ctx.single().unwrap();

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
    mut dog_q: Query<(&mut AseAnimation, &mut Sprite, &Velocity), With<Dog>>,
) {
    // Update which animation is playing based on movement.
    for (mut aseanim, mut sprite, velocity) in dog_q.iter_mut() {
        if **velocity == Vec2::ZERO {
            if aseanim.animation.tag.as_deref() != Some("idle_front") {
                aseanim.animation.play("idle_front", AnimationRepeat::Loop);
            }
        } else {
            if aseanim.animation.tag.as_deref() != Some("run_front") {
                aseanim.animation.play("run_front", AnimationRepeat::Loop);
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
    let bark = dog_q.single()
        .map(|input| input.bark)
        .unwrap_or(false);
    if bark {
        audio.play(sfx.dog_woof.clone());
    }
}
