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
) {
    let Some(loaded) = level_loaded.read().last() else { return; };

    match loaded.id.as_str() {
        "ground_zero" => {
            messages.send(
                Message::notify(
                    "Tutorial",
                    format!(
                        "Press {} to show objective and controls",
                        prompt.get(PlayerActions::ToggleHelp)
                    ),
                )
                .delay(Duration::from_millis(1200)),
            );
        }
        _ => warn!("no script for level {}", loaded.id),
    }
}
