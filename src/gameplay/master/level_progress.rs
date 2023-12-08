use super::level::current::LevelLoaded;
use crate::gameplay::objects::player::PlayerEvent;
use bevy::prelude::*;

/// Sent when level exit is unlocked
#[derive(Event)]
pub struct ExitUnlocked;

pub struct LevelProgressPlugin;

impl Plugin for LevelProgressPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LevelProgressState>()
            .add_event::<ExitUnlocked>()
            .add_systems(
                Update,
                (
                    on_level.run_if(on_event::<LevelLoaded>()),
                    on_player.run_if(on_event::<PlayerEvent>()),
                ),
            );
    }
}

#[derive(Resource, Default)]
struct LevelProgressState {
    exit_unlocked: bool,
}

fn on_level(mut state: ResMut<LevelProgressState>, mut loaded: EventReader<LevelLoaded>) {
    state.exit_unlocked = false;
}

fn on_player(
    mut player_events: EventReader<PlayerEvent>,
    mut unlocked_events: EventWriter<ExitUnlocked>,
    mut state: ResMut<LevelProgressState>,
) {
    for event in player_events.read() {
        match event {
            PlayerEvent::ReachedExitElevator => {
                if !state.exit_unlocked {
                    state.exit_unlocked = true;
                    unlocked_events.send(ExitUnlocked);
                }
            }
            #[allow(unreachable_patterns)] // flock off
            _ => (),
        }
    }
}
