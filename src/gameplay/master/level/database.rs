use bevy::prelude::*;
use std::collections::BTreeMap;

/// Info about all levels
#[derive(Resource)]
pub struct LevelDatabase {
    pub meta: BTreeMap<String, LevelMeta>,
}

impl LevelDatabase {
    /// First level in new game
    pub fn id_first(&self) -> String {
        ID_FIRST.to_string()
    }

    /// Name shown to player
    pub fn get_name<'a>(&'a self, id: &'a str) -> &'a str {
        self.meta
            .get(id)
            .map(|meta| meta.name.as_str())
            .unwrap_or(id)
    }
}

/// Metadata for a level
#[derive(Clone)]
pub struct LevelMeta {
    /// Name shown to player
    pub name: String,
}

pub struct DatabasePlugin;

impl Plugin for DatabasePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelDatabase {
            meta: [(
                ID_FIRST.to_string(),
                LevelMeta {
                    name: "Ground 0".to_string(),
                },
            )]
            .into(),
        });
    }
}

const ID_FIRST: &str = "ground_zero";
