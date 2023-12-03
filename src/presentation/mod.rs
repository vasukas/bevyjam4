use bevy::prelude::*;

mod menu;
mod objects;

pub struct PresentationPlugin;

impl Plugin for PresentationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((menu::MenuPlugin, objects::ObjectsPlugin));
    }
}
