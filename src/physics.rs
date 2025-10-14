use std::ops::{Deref, DerefMut};

use bevy::prelude::*;
use avian2d::prelude::*;

#[derive(PhysicsLayer, Default)]
pub enum GameLayer {
    #[default]
    Default,
    CatBox,
    Dog,
    Cat,
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(PhysicsPlugins::default().with_length_unit(32.0))
            .add_systems(Update, update_movement);
    }
}

#[derive(Clone, Copy, Default, Component)]
pub struct Velocity {
    pub inner: Vec2,
}

impl Deref for Velocity {
    type Target = Vec2;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Velocity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Velocity {
    pub fn new(vel: Vec2) -> Self {
        Self {
            inner: vel,
        }
    }
}

// TODO: Consider implementing a new method that enforces min and max.
#[derive(Component)]
pub struct MovementBounds {
    pub min: Vec2,
    pub max: Vec2,
}

pub fn update_movement(
    time: Res<Time>,
    mut movement_q: Query<(&Velocity, &mut Transform, Option<&MovementBounds>)>,
) {
    for (velocity, mut transform, bounds) in movement_q.iter_mut() {
        transform.translation += velocity.inner.extend(0.0) * time.delta_secs();
        // Take into account level bounds and clamp to them.
        // TODO: Move bounds clamping to a different system that uses change detection of transform.
        if let Some(bounds) = bounds {
            transform.translation.x = transform.translation.x.clamp(bounds.min.x, bounds.max.x);
            transform.translation.y = transform.translation.y.clamp(bounds.min.y, bounds.max.y);
        }
    }
}

pub fn collider(
    collider: Collider,
    memberships: impl Into<LayerMask>,
    filters: impl Into<LayerMask>,
) -> impl Bundle {
    (
        collider,
        CollisionLayers::new(memberships, filters),
        Sensor,
    )
}
