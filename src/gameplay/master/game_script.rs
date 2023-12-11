use super::level::current::CurrentLevel;
use super::level::current::LevelLoaded;
use crate::app::actions::ActionPrompt;
use crate::app::actions::PlayerActions;
use crate::presentation::DelayedMessage;
use crate::presentation::Message;
use bevy::prelude::*;
use std::time::Duration;

pub struct GameScriptPlugin;

impl Plugin for GameScriptPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, on_level_loaded.run_if(on_event::<LevelLoaded>()));
    }
}

fn on_level_loaded(
    mut level_loaded: EventReader<LevelLoaded>,
    mut messages: EventWriter<DelayedMessage>,
    prompt: ActionPrompt<PlayerActions>,
    mut current_level: ResMut<CurrentLevel>,
) {
    let Some(loaded) = level_loaded.read().last() else { return; };

    match loaded.id.as_str() {
        "01_cells" => {
            messages.send(
                Message::notify("Tutorial", "Look into bottom left corner to see tutorial.")
                    .delay(Duration::from_millis(1200), true),
            );
            messages.send(
                Message::notify(
                    "Tutorial",
                    format!(
                        "Press {} to show objective and controls",
                        prompt.get(PlayerActions::ToggleHelp)
                    ),
                )
                .delay(Duration::from_millis(6000), true),
            );
        }
        "02_connect" => {
            current_level.allow_starfield = true;
        }
        "03_loadbay" => (),
        "04_storage" => {
            current_level.allow_starfield = true;
        }
        "05_process" => (),
        "06_tunnels" => (),
        "07_engine" => (),
        "08_security" => {
            current_level.allow_starfield = true;
        }
        _ => warn!("no script for level {}", loaded.id),
    }
}
