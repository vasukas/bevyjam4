use crate::gameplay::master::level::current::CurrentLevel;
use crate::gameplay::master::level::current::LevelLoaded;
use crate::gameplay::master::level_progress::GotoNextLevel;
use crate::utils::plugins::userdata_plugin::Userdata;
use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeSet;

/// Player highscores and other progression data
#[derive(Resource, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Scores {
    /// ID of the last level played
    pub last_level: Option<ScoresLastLevel>,

    /// IDs of completed levels
    pub completed_levels: BTreeSet<String>,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ScoresLastLevel {
    pub id: String,
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
                    update_level.run_if(on_event::<LevelLoaded>()),
                    update_visited_levels.run_if(on_event::<GotoNextLevel>()),
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
        });
    }
}

fn update_visited_levels(
    mut scores: ResMut<Scores>,
    current: Res<CurrentLevel>,
    mut next_level: EventReader<GotoNextLevel>,
) {
    if let Some(next) = next_level.read().last() {
        match &next.id {
            Some(_) => {
                scores.completed_levels.insert(current.id.clone());
            }
            None => {
                // final level completed!
                scores.last_level = None;
            }
        }
    }
}
