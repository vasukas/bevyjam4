use super::data::LevelObject;
use super::data::LevelObjectData;
use super::data::LevelObjectId;
use crate::app::scheduling::SpawnSet;
use crate::utils::bevy::commands::FallibleCommands;
use bevy::prelude::*;

/// All root game objects must be marked with this (for despawning on restarts, level loads, etc).
#[derive(Component, Default)]
pub struct GameObject;

#[derive(Bundle, Default)]
pub struct GameObjectBundle {
    pub spatial: SpatialBundle,
    pub name: Name,
    pub game_object: GameObject,
}

impl GameObjectBundle {
    pub fn new(name: impl Into<std::borrow::Cow<'static, str>>, transform: Transform) -> Self {
        Self {
            spatial: SpatialBundle {
                transform,
                global_transform: transform.into(),
                ..default()
            },
            name: Name::new(name),
            game_object: GameObject,
        }
    }
}

/// Spawn an object.
///
/// Exposed for use in level editor.
#[derive(Event)]
pub struct SpawnObject {
    pub id: LevelObjectId,
    pub object: LevelObject,
}

/// Despawn all game objects.
///
/// Executed right before [`SpawnObject`].
#[derive(Event, Default)]
pub struct DespawnGameObjects;

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnObject>()
            .add_event::<DespawnGameObjects>()
            .add_systems(
                PostUpdate,
                (
                    spawn_objects.run_if(on_event::<SpawnObject>()),
                    despawn_objects.run_if(on_event::<DespawnGameObjects>()),
                )
                    .in_set(SpawnSet::Events),
            );
    }
}

fn spawn_objects(mut spawn_objects: ResMut<Events<SpawnObject>>, mut commands: Commands) {
    for SpawnObject { id, object } in spawn_objects.drain() {
        let mut entity = commands.spawn((
            GameObjectBundle::new("level object", object.transform()),
            id,
        ));

        match object.data {
            LevelObjectData::None => {
                error!("data type doesn't exist for level object: {id:?}")
            }
            LevelObjectData::ScriptPoint(object) => {
                entity.insert(object);
            }
            LevelObjectData::Elevator(object) => {
                entity.insert(object);
            }
            LevelObjectData::EnemySpawner(object) => {
                entity.insert(object);
            }
            LevelObjectData::Barrel(object) => {
                entity.insert(object);
            }
            LevelObjectData::TerrainWall(object) => {
                entity.insert(object);
            }
            LevelObjectData::TerrainFloor(object) => {
                entity.insert(object);
            }
            LevelObjectData::TerrainLight(object) => {
                entity.insert(object);
            }
        }
    }
}

fn despawn_objects(objects: Query<Entity, With<GameObject>>, mut commands: Commands) {
    for entity in objects.iter() {
        commands.try_despawn_recursive(entity);
    }
}
