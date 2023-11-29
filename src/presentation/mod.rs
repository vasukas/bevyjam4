use bevy::prelude::*;

mod menu;

pub struct PresentationPlugins;

impl Plugin for PresentationPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((menu::MenuPlugins,));
    }
}
