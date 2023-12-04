use super::MechanicSet;
use crate::gameplay::physics::*;
use bevy::prelude::*;

/// Force-based character controller for dynamic bodies.
///
/// Resets linear [`ExternalForce`] each step!
#[derive(Component)]
pub struct MovementController {
    /// Movement direction, reset each step. May be not normalized.
    pub target_dir: Vec2,

    /// [`Self::target_dir`] is multiplied by this
    pub speed: f32,

    /// Unitless force coefficient - larger it is, more responsive movement is
    pub k_force: f32,
}

impl MovementController {
    pub fn bundle(self) -> impl Bundle {
        (
            self,
            Velocity::default(),
            ExternalForce::default(),
            ReadMassProperties::default(),
        )
    }
}

impl Default for MovementController {
    fn default() -> Self {
        Self {
            target_dir: default(),
            speed: 6.,
            k_force: 150.,
        }
    }
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_controller.in_set(MechanicSet::Action));
    }
}

fn update_controller(
    mut controllers: Query<(
        &mut MovementController,
        &Velocity,
        &mut ExternalForce,
        &ReadMassProperties,
    )>,
    time: Res<Time>,
) {
    for (mut controller, velocity, mut ext_force, mass) in controllers.iter_mut() {
        let target_velocity = std::mem::take(&mut controller.target_dir) * controller.speed;
        let velocity = velocity.linvel;

        let k_force = controller.k_force * controller.speed * time.delta_seconds();

        let force = (target_velocity - velocity) * mass.mass * k_force;
        ext_force.force = force;
    }
}
