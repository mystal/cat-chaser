use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
pub use bevy_rapier2d::{
    prelude::{ActiveEvents, ActiveCollisionTypes, CollisionEvent},
    geometry::Group,
};

pub mod groups {
    use bevy_rapier2d::geometry::Group;

    pub const CATBOX: Group = Group::GROUP_1;
    pub const DOG: Group = Group::GROUP_2;
    pub const CAT: Group = Group::GROUP_3;

    pub const ALL: Group = Group::ALL;
    pub const NONE: Group = Group::NONE;
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(RapierConfiguration {
                gravity: Vec2::ZERO,
                ..default()
            })
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(32.0))
            .add_systems(Update, update_movement);
    }
}

#[derive(Clone, Copy, Default, Component)]
pub struct Velocity {
    pub inner: Vec2,
}

impl Velocity {
    pub fn new(vel: Vec2) -> Self {
        Self {
            inner: vel,
        }
    }
}

pub fn update_movement(
    time: Res<Time>,
    mut movement_q: Query<(&Velocity, &mut Transform)>,
) {
    // TODO: Take into account level bounds and clamp to them.
    for (velocity, mut transform) in movement_q.iter_mut() {
        transform.translation += velocity.inner.extend(0.0) * time.delta_seconds();
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

