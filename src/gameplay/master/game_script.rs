use super::level::current::CurrentLevel;
use super::level::current::LevelLoaded;
use bevy::prelude::*;

pub struct GameScriptPlugin;

impl Plugin for GameScriptPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, on_level_loaded.run_if(on_event::<LevelLoaded>()));
    }
}

fn on_level_loaded(
    mut level_loaded: EventReader<LevelLoaded>,
    mut current_level: ResMut<CurrentLevel>,
) {
    let Some(loaded) = level_loaded.read().last() else { return; };

    match loaded.id.as_str() {
        "ground_zero" => (),
        "two" => {
            current_level.allow_starfield = true;
        }
        "load_bay" => (),
        _ => warn!("no script for level {}", loaded.id),
    }
}
