use super::data::LevelData;
use super::spawn::DespawnGameObjects;
use super::spawn::SpawnObject;
use crate::app::scheduling::SpawnSet;
use crate::gameplay::master::level_progress::LevelList;
use crate::utils::misc_utils::ExtendedEventReader;
use bevy::prelude::*;

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
}

pub struct CurrentPlugin;

impl Plugin for CurrentPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentLevel>()
            .add_event::<LevelCommand>()
            .add_event::<LevelLoaded>()
            .add_systems(
                PostUpdate,
                (execute_level_commands
                    .before(SpawnSet::first())
                    .run_if(on_event::<LevelCommand>()),),
            );
    }
}

fn execute_level_commands(
    mut level_commands: EventReader<LevelCommand>,
    mut spawn_object: EventWriter<SpawnObject>,
    mut despawn_cmd: EventWriter<DespawnGameObjects>,
    mut current: ResMut<CurrentLevel>,
    levels: Res<LevelList>,
    mut loaded_event: EventWriter<LevelLoaded>,
) {
    if let Some(command) = level_commands.read_single("execute_level_commands") {
        info!("execute_level_commands: {command:?}");

        let (despawn, spawn) = match command {
            LevelCommand::Load(id) => {
                let data = levels.data(&id);

                *current = CurrentLevel {
                    id: id.clone(),
                    data: data.clone(),
                };

                loaded_event.send(LevelLoaded { id: id.clone() });

                (true, true)
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
