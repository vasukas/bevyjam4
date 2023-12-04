use bevy::prelude::*;

pub mod movement;

#[derive(SystemSet, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MechanicSet {
    /// Processes external input
    Input,

    /// Creates events
    Action,

    /// Reacts to events
    Reaction,
}

pub struct MechanicsPlugin;

impl Plugin for MechanicsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                MechanicSet::Input,
                MechanicSet::Action,
                MechanicSet::Reaction,
            )
                .chain(),
        )
        .add_plugins((movement::MovementPlugin,));
    }
}
