use crate::gameplay::master::script_points::EnemySpawner;
use crate::gameplay::master::script_points::ScriptPoint;
use crate::gameplay::objects::terrain::TerrainFloor;
use crate::gameplay::objects::terrain::TerrainLight;
use crate::gameplay::objects::terrain::TerrainWall;
use crate::utils::misc_utils::serde_sorted_map;
use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::Deserialize;
use serde::Serialize;
use std::f32::consts::FRAC_PI_2;
use std::f32::consts::PI;

/// Size of a tile
pub const TILE_SIZE: f32 = 2.;

/// Half of size of a tile
pub const HALF_TILE: f32 = TILE_SIZE / 2.;

/// First level in new game
pub const FIRST_LEVEL_ID: &str = "ground_zero";

/// Level data
#[derive(Clone, Serialize, Deserialize, Default, Asset, TypePath)]
#[serde(default)]
pub struct LevelData {
    #[serde(serialize_with = "serde_sorted_map")]
    objects: HashMap<LevelObjectId, LevelObject>,
    last_object_id: u64,

    /// Name shown to player
    pub name: String,
}

impl LevelData {
    pub fn add_object(&mut self, object: LevelObject) -> LevelObjectId {
        self.last_object_id = self.last_object_id.checked_add(1).unwrap();
        let id = LevelObjectId(self.last_object_id);
        self.objects.insert(id, object);
        id
    }

    pub fn remove_object(&mut self, id: LevelObjectId) {
        self.objects.remove(&id);
    }

    pub fn get_object_mut(&mut self, id: LevelObjectId) -> Option<&mut LevelObject> {
        self.objects.get_mut(&id)
    }

    pub fn objects(&self) -> impl Iterator<Item = (LevelObjectId, &LevelObject)> {
        self.objects.iter().map(|v| (*v.0, v.1))
    }

    pub fn get_object(&self, id: LevelObjectId) -> Option<&LevelObject> {
        self.objects.get(&id)
    }
}

/// Unique ID - doesn't get used again **in single executable run**.
///
/// Spawned together with level objects, can be used to find objects in editor.
#[derive(
    Component,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Debug,
    Serialize,
    Deserialize,
)]
pub struct LevelObjectId(u64);

///
#[derive(Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct LevelObject {
    /// For tile-aligned objects: always center position
    pub pos: Vec2,
    pub align: LevelAlign,
    pub data: LevelObjectData,
}

impl LevelObject {
    pub fn transform(&self) -> Transform {
        let pos = self.pos + self.align.offset();
        let rotation = Quat::from_rotation_z(self.align.rotation_angle());
        Transform::from_translation(pos.extend(0.)).with_rotation(rotation)
    }
}

/// To which edge of the tile object sticks
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize, Default)]
pub enum LevelAlign {
    Top,
    Bottom,
    Left,
    Right,

    #[default]
    #[serde(other)]
    Center,
}

impl LevelAlign {
    /// From center position to tile edge
    pub fn offset(self) -> Vec2 {
        match self {
            LevelAlign::Center => Vec2::ZERO,
            LevelAlign::Top => Vec2::Y * HALF_TILE,
            LevelAlign::Bottom => Vec2::NEG_Y * HALF_TILE,
            LevelAlign::Left => Vec2::NEG_X * HALF_TILE,
            LevelAlign::Right => Vec2::X * HALF_TILE,
        }
    }

    pub fn rotation_angle(self) -> f32 {
        match self {
            LevelAlign::Center => 0.,
            LevelAlign::Top => 0.,
            LevelAlign::Bottom => PI,
            LevelAlign::Left => FRAC_PI_2,
            LevelAlign::Right => -FRAC_PI_2,
        }
    }

    pub fn symbol(self) -> &'static str {
        match self {
            LevelAlign::Center => ".",
            LevelAlign::Top => "^",
            LevelAlign::Bottom => "v",
            LevelAlign::Left => "<",
            LevelAlign::Right => ">",
        }
    }
}

/// Spawns all required components when added to the entity.
///
/// Changes and removal are ignored.
#[derive(Component, Clone, Serialize, Deserialize, Default, Debug)]
pub enum LevelObjectData {
    ScriptPoint(ScriptPoint),
    EnemySpawner(EnemySpawner),

    TerrainWall(TerrainWall),
    TerrainFloor(TerrainFloor),
    TerrainLight(TerrainLight),

    #[default]
    #[serde(other)]
    None,
}
