use super::data::LevelData;
use super::spawn::DespawnGameObjects;
use super::spawn::SpawnObject;
use crate::app::scheduling::SpawnSet;
use crate::utils::misc_utils::ExtendedEventReader;
use bevy::asset::AssetLoader;
use bevy::asset::AsyncReadExt as _;
use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy::utils::BoxedFuture;
use thiserror::Error;

/// Change current level
#[derive(Event, Debug)]
pub enum LevelCommand {
    /// Load level with specified ID and re-spawn all game objects
    Load(String),

    /// Re-spawn all objects for current level
    Reload,

    /// Despawn all game objects
    Unload,

    /// Save current level to it's own file
    Save,
}

#[derive(Resource, Default)]
pub struct CurrentLevel {
    pub id: String,
    pub data: LevelData,
}

/// Sent when [`LevelCommand::Load`] is completed.
#[derive(Event)]
pub struct LevelLoaded {
    pub id: String,
    pub name: String,
}

pub struct CurrentPlugin;

impl Plugin for CurrentPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentLevel>()
            .add_event::<LevelCommand>()
            .add_event::<LevelLoaded>()
            .init_asset::<LevelData>()
            .init_asset_loader::<LevelLoader>()
            .add_systems(
                PostUpdate,
                (
                    execute_level_commands
                        .before(SpawnSet::first())
                        .run_if(on_event::<LevelCommand>()),
                    complete_pending_level_load.run_if(resource_exists::<PendingLevelLoad>()),
                ),
            );
    }
}

/// Optional resource
#[derive(Resource)]
struct PendingLevelLoad {
    handle: Handle<LevelData>,
    id: String,
}

fn complete_pending_level_load(
    mut spawn_object: EventWriter<SpawnObject>,
    mut commands: Commands,
    pending: Res<PendingLevelLoad>,
    mut levels: ResMut<Assets<LevelData>>,
    mut current: ResMut<CurrentLevel>,
    asset_server: Res<AssetServer>,
    mut loaded: EventWriter<LevelLoaded>,
) {
    let level = match levels.remove(&pending.handle) {
        Some(level) => level,
        None => {
            let Some(LoadState::Failed) = asset_server.get_load_state(&pending.handle) else { return; };
            error!("Failed to load level");
            LevelData::default()
        }
    };

    for (id, object) in level.objects() {
        spawn_object.send(SpawnObject {
            id,
            object: object.clone(),
        });
    }

    loaded.send(LevelLoaded {
        id: pending.id.clone(),
        name: level.name.clone(),
    });

    current.data = level;

    commands.remove_resource::<PendingLevelLoad>();
}

fn execute_level_commands(
    mut level_commands: EventReader<LevelCommand>,
    mut spawn_object: EventWriter<SpawnObject>,
    mut despawn_cmd: EventWriter<DespawnGameObjects>,
    mut current: ResMut<CurrentLevel>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if let Some(command) = level_commands.read_single("execute_level_commands") {
        info!("execute_level_commands: {command:?}");

        let (despawn, spawn) = match command {
            LevelCommand::Load(id) => {
                commands.insert_resource(PendingLevelLoad {
                    handle: asset_server.load(format!("levels/{}.level", id)),
                    id: id.clone(),
                });
                current.id = id.clone();
                (true, false)
            }
            LevelCommand::Reload => (true, true),
            LevelCommand::Unload => (true, false),
            LevelCommand::Save => {
                #[cfg(not(target_arch = "wasm32"))]
                std::fs::write(
                    format!("assets/levels/{}.level", current.id),
                    ron::ser::to_string_pretty(&current.data, default()).unwrap(),
                )
                .unwrap();
                (false, false)
            }
        };

        if despawn {
            despawn_cmd.send_default()
        }

        if spawn {
            for (id, object) in current.data.objects() {
                spawn_object.send(SpawnObject {
                    id,
                    object: object.clone(),
                });
            }
        }
    }
}

#[derive(Default)]
struct LevelLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum CustomAssetLoaderError {
    /// An [IO](std::io) Error
    #[error("Could load shader: {0}")]
    Io(#[from] std::io::Error),
    /// A [RON](ron) Error
    #[error("Could not parse RON: {0}")]
    RonSpannedError(#[from] ron::error::SpannedError),
}

impl AssetLoader for LevelLoader {
    type Asset = LevelData;
    type Settings = ();
    type Error = CustomAssetLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        _settings: &'a (),
        _load_context: &'a mut bevy::asset::LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let custom_asset = ron::de::from_bytes::<Self::Asset>(&bytes)?;
            Ok(custom_asset)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["level"]
    }
}
