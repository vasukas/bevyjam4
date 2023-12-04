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
