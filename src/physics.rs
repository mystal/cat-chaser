use std::ops::{Deref, DerefMut};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
pub use bevy_rapier2d::{
    prelude::{ActiveEvents, ActiveCollisionTypes},
    geometry::Group,
};

pub mod groups {
    use bevy_rapier2d::geometry::Group;

    pub const CATBOX: Group = Group::GROUP_1;
    pub const DOG: Group = Group::GROUP_2;
    pub const CAT: Group = Group::GROUP_3;

    #[allow(unused)]
    pub const ALL: Group = Group::ALL;
    #[allow(unused)]
    pub const NONE: Group = Group::NONE;
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(32.0))
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
    // TODO: Take into account level bounds and clamp to them.
    for (velocity, mut transform, bounds) in movement_q.iter_mut() {
        transform.translation += velocity.inner.extend(0.0) * time.delta_seconds();
        // TODO: Move bounds clamping to a different system that uses change detection of transform.
        if let Some(bounds) = bounds {
            transform.translation.x = transform.translation.x.clamp(bounds.min.x, bounds.max.x);
            transform.translation.y = transform.translation.y.clamp(bounds.min.y, bounds.max.y);
        }
    }
}

#[derive(Bundle)]
pub struct ColliderBundle {
    shape: Collider,
    layers: CollisionGroups,
    sensor: Sensor,
    active_events: ActiveEvents,
    collision_types: ActiveCollisionTypes,
}

impl ColliderBundle {
    pub fn circle(radius: f32, memberships: Group, filters: Group) -> Self {
        Self {
            shape: Collider::ball(radius),
            layers: CollisionGroups::new(memberships, filters),
            sensor: Sensor,
            active_events: ActiveEvents::COLLISION_EVENTS,
            collision_types: ActiveCollisionTypes::default() | ActiveCollisionTypes::STATIC_STATIC,
        }
    }

    pub fn rect(size: Vec2, memberships: Group, filters: Group) -> Self {
        let half_extents = size / 2.0;
        Self {
            shape: Collider::cuboid(half_extents.x, half_extents.y),
            layers: CollisionGroups::new(memberships, filters),
            sensor: Sensor,
            active_events: ActiveEvents::COLLISION_EVENTS,
            collision_types: ActiveCollisionTypes::default() | ActiveCollisionTypes::STATIC_STATIC,
        }
    }
}
