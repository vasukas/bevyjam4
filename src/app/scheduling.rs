use bevy::prelude::*;
use bevy::transform::TransformSystem;

/// Sets in [`PostUpdate`].
///
/// Commands are applied between these sets.
#[derive(SystemSet, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum SpawnSet {
    /// Events which start spawning
    Events,
    /// Spawn root components (i.e. Player)
    Roots,
    /// Spawn controller components (i.e. PlayerKinematic)
    Controllers,
    /// Spawn detail components (i.e. PlayerFootsteps)
    Details,
}

impl SpawnSet {
    /// First set of this bunch
    pub fn first() -> Self {
        Self::Events
    }
}

pub struct SchedulingPlugin;

impl Plugin for SchedulingPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            PostUpdate,
            (
                SpawnSet::Events,
                SpawnSet::Roots,
                SpawnSet::Controllers,
                SpawnSet::Details,
            )
                .chain()
                .after(TransformSystem::TransformPropagate),
        )
        .add_systems(
            PostUpdate,
            (
                apply_deferred
                    .after(SpawnSet::Events)
                    .before(SpawnSet::Roots),
                apply_deferred
                    .after(SpawnSet::Roots)
                    .before(SpawnSet::Controllers),
                apply_deferred
                    .after(SpawnSet::Controllers)
                    .before(SpawnSet::Details),
            ),
        );
    }
}
