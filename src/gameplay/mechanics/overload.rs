use bevy::prelude::*;

#[derive(Component)]
pub struct OverloadSource {
    pub power: f32,
}

pub struct OverloadPlugin;

impl Plugin for OverloadPlugin {
    fn build(&self, app: &mut App) {
        //
    }
}
