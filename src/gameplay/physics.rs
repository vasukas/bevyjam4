use bevy::prelude::*;

pub use bevy_rapier2d::prelude::*;

#[derive(Bundle)]
pub struct TypicalBody {
    pub body: RigidBody,
    pub collider: Collider,
    pub friction: Friction,
    pub restitution: Restitution,
    pub mass: ColliderMassProperties,
}

impl TypicalBody {
    pub fn new(collider: Collider) -> Self {
        Self {
            body: RigidBody::Dynamic,
            collider,
            friction: default(),
            restitution: default(),
            mass: ColliderMassProperties::Mass(5.),
        }
    }

    pub fn new_ball(radius: f32) -> Self {
        Self::new(Collider::ball(radius))
    }

    pub fn new_box(half_extents: Vec2) -> Self {
        Self::new(Collider::cuboid(half_extents.x, half_extents.y))
    }

    pub fn mass(mut self, mass: f32) -> Self {
        self.mass = ColliderMassProperties::Mass(mass);
        self
    }

    pub fn friction(mut self, coefficient: f32) -> Self {
        self.friction.coefficient = coefficient;
        self
    }

    pub fn restitution(mut self, coefficient: f32) -> Self {
        self.restitution = Restitution {
            coefficient,
            combine_rule: CoefficientCombineRule::Min,
        };
        self
    }

    pub fn lock_rotation(self) -> impl Bundle {
        (self, LockedAxes::ROTATION_LOCKED)
    }
}

/// How bodies interact
#[derive(Clone, Copy)]
pub enum PhysicsType {
    /// Impenetrable by anything
    Wall,
    /// Collide only with walls
    WallOnly,

    /// Most level objects
    Object,

    Enemy,
    Projectile,

    /// Collides only with itself
    Overload,

    GravityPull,
    Conveyor,
}

impl PhysicsType {
    pub fn groups(self) -> CollisionGroups {
        let wall = Group::GROUP_1;
        let object = Group::GROUP_2;
        let enemy = Group::GROUP_3;
        let projectile = Group::GROUP_4;
        let wall_only = Group::GROUP_5;
        let overload = Group::GROUP_6;
        let gravity_pull = Group::GROUP_7;
        let conveyor = Group::GROUP_8;

        let (memberships, filters) = match self {
            PhysicsType::Wall => (wall, Group::all()),
            PhysicsType::WallOnly => (wall_only, wall),
            PhysicsType::Object => (object, Group::all()),
            PhysicsType::Enemy => (enemy, Group::all()),
            PhysicsType::Projectile => (projectile, wall | object | gravity_pull),
            PhysicsType::Overload => (overload, overload),
            PhysicsType::GravityPull => (Group::all(), object | projectile),
            // PhysicsType::Conveyor => (conveyor, object | enemy),
            PhysicsType::Conveyor => (conveyor, object), // TODO: fix convetors
        };

        CollisionGroups {
            memberships,
            filters,
        }
    }

    pub fn filter(self) -> QueryFilter<'static> {
        QueryFilter::new().groups(self.groups())
    }
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .insert_resource(RapierConfiguration {
                gravity: Vec2::ZERO,
                ..default()
            });

        #[cfg(feature = "dev_build")]
        app.add_plugins(RapierDebugRenderPlugin {
            enabled: false,
            mode: DebugRenderMode::COLLIDER_SHAPES
                | DebugRenderMode::SOLVER_CONTACTS
                | DebugRenderMode::CONTACTS,
            ..default()
        });
    }
}
