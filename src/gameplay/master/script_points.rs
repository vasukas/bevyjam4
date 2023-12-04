use super::level::spawn::GameObjectBundle;
use crate::gameplay::objects::player::Player;
use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;

/// Marker used by scripts
#[derive(Component, Serialize, Deserialize, Clone, Default, Debug)]
#[serde(default)]
pub struct ScriptPoint {
    pub id: String,
}

pub struct ScriptsPlugin;

impl Plugin for ScriptsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, on_point_added);
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
