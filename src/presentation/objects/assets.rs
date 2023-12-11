use crate::utils::plugins::load_assets::LoadedTrackedAssets;
use crate::utils::plugins::load_assets::TrackAssets;
use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Resource)]
pub struct ObjectAssets {
    // don't forget to add new ones to load_models!
    pub model_jimbo: ModelAsset,
    pub model_tripod: ModelAsset,

    // floors
    pub scene_void_lta: Handle<Scene>,
    pub scene_void_ltb: Handle<Scene>,
    pub scene_void_squ: Handle<Scene>,
    pub scene_void_tri: Handle<Scene>,
    pub scene_floor_hatch: Handle<Scene>,
    pub scene_floor_metals: Handle<Scene>,
    pub scene_floor: Handle<Scene>,

    // walls
    pub scene_wall_computer: Handle<Scene>,
    pub scene_wall_compscreen: Handle<Scene>,
    pub scene_wall_hatch: Handle<Scene>,
    pub scene_wall_ventilation: Handle<Scene>, // NONE
    pub scene_wall_vertpipe: Handle<Scene>,
    pub scene_wall_vertpipe2: Handle<Scene>,
    pub scene_wall_horpipes: Handle<Scene>,
    pub scene_wall: Handle<Scene>,

    // barrels
    pub scene_barrel_red: Handle<Scene>,
    pub scene_barrel_blue: Handle<Scene>,

    // elevators
    pub scene_elevator_enter: Handle<Scene>,
    pub scene_elevator_exit: Handle<Scene>,

    // decor
    pub scene_cell_bars: Handle<Scene>,
    pub scene_cell_melted: Handle<Scene>,
    pub scene_cell_bed: Handle<Scene>,
    pub scene_load_crane: Handle<Scene>,
    pub scene_closed_pipe: Handle<Scene>,
    pub scene_green_pipe: Handle<Scene>,

    // unique
    pub model_engine: ModelAsset, // NONE
    pub model_brain: ModelAsset,  // NONE
    pub model_cannon: ModelAsset, // NONE

    // conveyor
    pub scene_belt: Handle<Scene>,
    pub scene_chute: Handle<Scene>,

    // procedural
    pub scene_star: Handle<Scene>,
    pub scorchmark: Handle<Scene>,

    /// 1x1x1 cube
    pub mesh_cube: Handle<Mesh>,
    /// 0.5-radius UV-sphere
    pub mesh_sphere: Handle<Mesh>,
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
        let anim = self.animations.get(name);
        if anim.is_none() {
            error!("animation \"{name}\" not found");
        }
        anim.cloned().unwrap_or_default()
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
        model_tripod: ModelAsset::new(track.load_and_track("models/tripod.gltf")),

        scene_void_lta: track.load_and_track("models/void_lta.glb#Scene0"),
        scene_void_ltb: track.load_and_track("models/void_ltb.glb#Scene0"),
        scene_void_squ: track.load_and_track("models/void_squ.glb#Scene0"),
        scene_void_tri: track.load_and_track("models/void_tri.glb#Scene0"),
        scene_floor_hatch: track.load_and_track("models/floor_hatch.glb#Scene0"),
        scene_floor_metals: track.load_and_track("models/floor_metals.glb#Scene0"),
        scene_floor: track.load_and_track("models/floor.gltf#Scene0"),

        scene_wall_computer: track.load_and_track("models/wall_computer.glb#Scene0"),
        scene_wall_compscreen: track.load_and_track("models/wall_compscreen.glb#Scene0"),
        scene_wall_hatch: track.load_and_track("models/wall_hatch.glb#Scene0"),
        scene_wall_ventilation: track.load_and_track("models/wall_ventilation.glb#Scene0"),
        scene_wall_vertpipe: track.load_and_track("models/wall_vertpipe.glb#Scene0"),
        scene_wall_vertpipe2: track.load_and_track("models/wall_vertpipe2.glb#Scene0"),
        scene_wall_horpipes: track.load_and_track("models/wall_horpipes.glb#Scene0"),
        scene_wall: track.load_and_track("models/wall.gltf#Scene0"),

        scene_barrel_red: track.load_and_track("models/barrel_red.gltf#Scene0"),
        scene_barrel_blue: track.load_and_track("models/barrel_blue.gltf#Scene0"),

        scene_elevator_enter: track.load_and_track("models/elevator_enter.glb#Scene0"),
        scene_elevator_exit: track.load_and_track("models/elevator_exit.glb#Scene0"),

        scene_cell_bars: track.load_and_track("models/cell_bars.glb#Scene0"),
        scene_cell_melted: track.load_and_track("models/cell_melted.glb#Scene0"),
        scene_cell_bed: track.load_and_track("models/cell_bed.glb#Scene0"),
        scene_load_crane: track.load_and_track("models/load_crane.glb#Scene0"),
        scene_closed_pipe: track.load_and_track("models/closed_pipe.glb#Scene0"),
        scene_green_pipe: track.load_and_track("models/green_pipe.glb#Scene0"),

        model_engine: ModelAsset::new(track.load_and_track("models/engine.gltf")),
        model_brain: ModelAsset::new(track.load_and_track("models/brain.gltf")),
        model_cannon: ModelAsset::new(track.load_and_track("models/cannon.glb")),

        scene_belt: track.load_and_track("models/belt.glb#Scene0"),
        scene_chute: track.load_and_track("models/chute.glb#Scene0"),

        scene_star: track.load_and_track("models/star.glb#Scene0"),
        scorchmark: track.load_and_track("models/scorchmark.glb#Scene0"),

        mesh_cube: meshes.add(shape::Cube::default().into()),
        mesh_sphere: meshes.add(
            shape::UVSphere {
                radius: 0.5,
                sectors: 18,
                stacks: 9,
            }
            .into(),
        ),
    });
}

fn load_models(mut assets: ResMut<ObjectAssets>, gltfs: Res<Assets<Gltf>>) {
    let load_model = |model: &mut ModelAsset| {
        let gltf = gltfs.get(&model.gltf).unwrap();
        model.scene = gltf.default_scene.clone().unwrap();
        model.animations = gltf.named_animations.clone();
    };

    load_model(&mut assets.model_jimbo);
    load_model(&mut assets.model_tripod);
}
