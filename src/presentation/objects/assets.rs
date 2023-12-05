use crate::utils::plugins::load_assets::LoadedTrackedAssets;
use crate::utils::plugins::load_assets::TrackAssets;
use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Resource)]
pub struct ObjectAssets {
    pub model_jimbo: ModelAsset,

    pub scene_floor: Handle<Scene>,
    pub scene_wall: Handle<Scene>,

    /// 1x1x1 cube
    pub mesh_cube: Handle<Mesh>,
}

pub struct ModelAsset {
    gltf: Handle<Gltf>,
    scene: Handle<Scene>,
    animations: HashMap<String, Handle<AnimationClip>>,
}

impl ModelAsset {
    pub fn scene(&self) -> Handle<Scene> {
        self.scene.clone()
    }

    pub fn animation(&self, name: &str) -> Handle<AnimationClip> {
        self.animations.get(name).cloned().unwrap_or_default()
    }

    fn new(gltf: Handle<Gltf>) -> Self {
        Self {
            gltf,
            scene: default(),
            animations: default(),
        }
    }
}

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_assets)
            .add_systems(First, load_models.run_if(on_event::<LoadedTrackedAssets>()));
    }
}

fn load_assets(mut track: TrackAssets, mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands.insert_resource(ObjectAssets {
        model_jimbo: ModelAsset::new(track.load_and_track("models/jimbo.gltf")),

        scene_floor: track.load_and_track("models/floor.gltf#Scene0"),
        scene_wall: track.load_and_track("models/wall.gltf#Scene0"),

        mesh_cube: meshes.add(shape::Cube::default().into()),
    });
}

fn load_models(mut assets: ResMut<ObjectAssets>, gltfs: Res<Assets<Gltf>>) {
    let load_model = |model: &mut ModelAsset| {
        let gltf = gltfs.get(&model.gltf).unwrap();
        model.scene = gltf.default_scene.clone().unwrap();
        model.animations = gltf.named_animations.clone();
    };

    load_model(&mut assets.model_jimbo);
}
