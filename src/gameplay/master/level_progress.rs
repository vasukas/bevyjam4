use super::level::current::CurrentLevel;
use super::level::current::LevelLoaded;
use super::level::data::LevelData;
use crate::gameplay::mechanics::damage::Dead;
use crate::gameplay::objects::player::PlayerEvent;
use crate::utils::plugins::load_assets::LoadedTrackedAssets;
use crate::utils::plugins::load_assets::TrackAssets;
use bevy::prelude::*;
use bevy::utils::HashMap;

/// Sent when level exit is unlocked
#[derive(Event)]
pub struct ExitUnlocked;

/// IDs of all levels in the game
#[derive(Resource)]
pub struct LevelList {
    levels: HashMap<String, LevelInfo>,
    order: Vec<String>,
    first: String,
}

impl LevelList {
    pub fn first(&self) -> String {
        self.first.clone()
    }

    pub fn all(&self) -> impl Iterator<Item = &String> {
        self.order.iter()
    }

    pub fn data(&self, id: &str) -> &LevelData {
        &self
            .levels
            .get(id)
            .expect(&format!("no such level \"{id}\""))
            .data
    }

    pub fn replace_data(&mut self, id: &str, data: LevelData) {
        self.levels
            .get_mut(id)
            .expect(&format!("no such level \"{id}\""))
            .data = data
    }

    pub fn name(&self, id: &str) -> String {
        self.levels
            .get(id)
            .map(|info| info.name.clone())
            .unwrap_or_else(|| id.to_string())
    }

    fn next(&self, id: &str) -> Option<String> {
        self.levels.get(id).and_then(|info| info.next.clone())
    }
}

struct LevelInfo {
    asset: Handle<LevelData>,
    data: LevelData,
    next: Option<String>,
    name: String,
}

/// Must be dealt with to pass the level
#[derive(Component)]
pub struct ImportantEnemy;

/// Sent here to go to next level, handled in menues
#[derive(Event)]
pub struct GotoNextLevel {
    /// If None, last level has been completed!
    pub id: Option<String>,
}

/// The plugin
pub struct LevelProgressPlugin;

impl Plugin for LevelProgressPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LevelProgressState>()
            .add_event::<ExitUnlocked>()
            .add_event::<GotoNextLevel>()
            .add_systems(Startup, load_levels)
            .add_systems(
                First,
                on_loaded_assets.run_if(on_event::<LoadedTrackedAssets>()),
            )
            .add_systems(
                Update,
                (
                    on_level_loaded.run_if(on_event::<LevelLoaded>()),
                    on_player_event.run_if(on_event::<PlayerEvent>()),
                    check_enemies,
                ),
            );
    }
}

fn load_levels(mut commands: Commands, mut track: TrackAssets) {
    let sequence = [("ground_zero", "Ground Zero"), ("two", "Connector")];

    commands.insert_resource(LevelList {
        first: sequence.first().unwrap().0.to_string(),
        order: sequence.iter().map(|v| v.0.to_string()).collect(),
        levels: sequence
            .iter()
            .enumerate()
            .map(|(index, (id, name))| {
                let asset = track.load_and_track(format!("levels/{id}.level"));
                (
                    id.to_string(),
                    LevelInfo {
                        asset,
                        data: default(),
                        next: sequence.get(index + 1).map(|s| s.0.to_string()),
                        name: name.to_string(),
                    },
                )
            })
            .collect(),
    });
}

fn on_loaded_assets(mut levels: ResMut<LevelList>, mut assets: ResMut<Assets<LevelData>>) {
    for (id, level) in levels.levels.iter_mut() {
        if let Some(data) = assets.remove(&level.asset) {
            level.data = data;
        } else {
            error!("Can't load level \"{id}\"!");
        }
    }
}

#[derive(Resource, Default)]
struct LevelProgressState {
    exit_unlocked: bool,
    goto_sent: bool,
}

fn on_level_loaded(mut state: ResMut<LevelProgressState>) {
    *state = default();
}

fn on_player_event(
    mut player_events: EventReader<PlayerEvent>,
    mut events: EventWriter<GotoNextLevel>,
    levels: Res<LevelList>,
    mut state: ResMut<LevelProgressState>,
    current: Res<CurrentLevel>,
) {
    for event in player_events.read() {
        match event {
            PlayerEvent::ReachedExitElevator => {
                if state.exit_unlocked && !state.goto_sent {
                    state.goto_sent = true;

                    let id = levels.next(&current.id);

                    info!("GotoNextLevel: {id:?}");

                    events.send(GotoNextLevel { id });
                }
            }
            #[allow(unreachable_patterns)] // flock off
            _ => (),
        }
    }
}

fn check_enemies(
    enemies: Query<Has<Dead>, With<ImportantEnemy>>,
    mut state: ResMut<LevelProgressState>,
    mut event: EventWriter<ExitUnlocked>,
) {
    if !enemies.is_empty() && !state.exit_unlocked {
        let all_dead = enemies.iter().all(|dead| dead);
        if all_dead {
            state.exit_unlocked = true;
            event.send(ExitUnlocked);
        }
    }
}
