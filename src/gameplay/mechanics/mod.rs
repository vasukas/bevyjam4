use bevy::prelude::*;

pub mod ai;
pub mod damage;
pub mod movement;
pub mod overload;

/// In [`Update`]
#[derive(SystemSet, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MechanicSet {
    /// Processes external input or sensor data
    Input,

    /// Creates events
    Action,

    /// Reacts to events
    Reaction,

    PostReaction,
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
                MechanicSet::PostReaction,
            )
                .chain(),
        )
        .add_plugins((
            movement::MovementPlugin,
            damage::DamagePlugin,
            ai::AiPlugin,
            overload::OverloadPlugin,
        ));
    }
}
