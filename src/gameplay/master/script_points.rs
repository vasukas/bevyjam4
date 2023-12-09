use super::level::spawn::GameObjectBundle;
use crate::gameplay::objects::enemy::Enemy;
use crate::gameplay::objects::player::Player;
use crate::utils::bevy::commands::FallibleCommands;
use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;

/// Marker used by scripts
#[derive(Component, Serialize, Deserialize, Clone, Default, Debug)]
#[serde(default)]
pub struct ScriptPoint {
    pub id: String,
}

/// Marker used by scripts
#[derive(Component, Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub enum EnemySpawner {
    #[default]
    #[serde(other)]
    Regular,
}

///
pub struct ScriptsPlugin;

impl Plugin for ScriptsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (on_point_added, enemy_spawner));
    }
}

fn on_point_added(
    new: Query<(&GlobalTransform, &ScriptPoint), Added<ScriptPoint>>,
    mut commands: Commands,
) {
    for (transform, point) in new.iter() {
        match point.id.as_str() {
            "player" => {
                commands.spawn((
                    GameObjectBundle::new("the player", Transform::from(*transform)),
                    Player::default(),
                ));
            }
            _ => (),
        }
    }
}

fn enemy_spawner(new: Query<(Entity, &EnemySpawner), Added<EnemySpawner>>, mut commands: Commands) {
    for (entity, spawner) in new.iter() {
        match *spawner {
            EnemySpawner::Regular => {
                commands.try_with_children(entity, |parent| {
                    parent.spawn((GameObjectBundle::new("enemy", default()), Enemy));
                });
            }
        }
    }
}
