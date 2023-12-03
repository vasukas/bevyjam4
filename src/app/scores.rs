use crate::utils::plugins::userdata_plugin::Userdata;
use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;

/// Player highscores and other progression data
#[derive(Resource, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Scores {
    /// ID of the last level played
    pub level: Option<ScoresLastLevel>,
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
            .add_systems(Update, save_scores.run_if(resource_changed::<Scores>()));
    }
}

fn load_scores(mut scores: ResMut<Scores>, userdata: Res<Userdata>) {
    *scores = userdata.read_and_update(USERDATA_NAME);
}

fn save_scores(scores: Res<Scores>, userdata: Res<Userdata>) {
    userdata.write(USERDATA_NAME, &*scores);
}
