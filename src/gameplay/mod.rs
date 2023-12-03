use bevy::prelude::*;

pub mod master;
pub mod mechanics;
pub mod objects;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            master::MasterPlugin,
            objects::ObjectsPlugin,
            mechanics::MechanicsPlugin,
        ));
    }
}
