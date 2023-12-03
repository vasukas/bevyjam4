use bevy::prelude::*;

mod advanced_gizmos;
mod menu;
mod objects;

pub use advanced_gizmos::AdvancedGizmos;

pub struct PresentationPlugin;

impl Plugin for PresentationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            menu::MenuPlugin,
            objects::ObjectsPlugin,
            advanced_gizmos::AdvancedGizmosPlugin,
        ));
    }
}
