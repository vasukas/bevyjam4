use bevy::prelude::*;

pub struct GameplayPlugins;

impl Plugin for GameplayPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(());
    }
}
