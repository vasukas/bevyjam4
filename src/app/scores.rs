use crate::gameplay::master::level::current::LevelLoaded;
use crate::utils::plugins::userdata_plugin::Userdata;
use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;

/// Player highscores and other progression data
#[derive(Resource, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Scores {
    /// ID of the last level played
    pub last_level: Option<ScoresLastLevel>,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ScoresLastLevel {
    pub id: String,
    pub name: String,
}

const USERDATA_NAME: &str = "scores";

pub struct ScoresPlugin;

impl Plugin for ScoresPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Scores>()
            .add_systems(PreStartup, load_scores)
            .add_systems(
                PostUpdate,
                (
                    (update_level.run_if(on_event::<LevelLoaded>()),),
                    save_scores.run_if(resource_changed::<Scores>()),
                )
                    .chain(),
            );
    }
}

fn load_scores(mut scores: ResMut<Scores>, userdata: Res<Userdata>) {
    *scores = userdata.read_and_update(USERDATA_NAME);
}

fn save_scores(scores: Res<Scores>, userdata: Res<Userdata>) {
    userdata.write(USERDATA_NAME, &*scores);
}

fn update_level(mut scores: ResMut<Scores>, mut level_loaded: EventReader<LevelLoaded>) {
    if let Some(loaded) = level_loaded.read().last() {
        scores.last_level = Some(ScoresLastLevel {
            id: loaded.id.clone(),
            name: loaded.name.clone(),
        });
    }
}
