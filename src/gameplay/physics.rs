use bevy::prelude::*;

pub use bevy_rapier2d::prelude::*;

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
