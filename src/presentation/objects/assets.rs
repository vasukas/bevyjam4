use crate::utils::plugins::load_assets::TrackAssets;
use bevy::prelude::*;

#[derive(Resource)]
pub struct ObjectAssets {
    pub model_jimbo: Handle<Scene>,
    pub model_floor: Handle<Scene>,
    pub model_wall: Handle<Scene>,
}

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_assets);
    }
}

fn load_assets(mut track: TrackAssets, mut commands: Commands) {
    commands.insert_resource(ObjectAssets {
        model_jimbo: track.load_and_track("models/jimbo.glb#Scene0"),
        model_floor: track.load_and_track("models/floor.glb#Scene0"),
        model_wall: track.load_and_track("models/wall.glb#Scene0"),
    });
}
