use bevy::prelude::*;

pub struct MechanicsPlugin;

impl Plugin for MechanicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(());
    }
}