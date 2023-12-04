use bevy::asset::AssetPath;
use bevy::asset::DependencyLoadState;
use bevy::asset::LoadState;
use bevy::asset::UntypedAssetId;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy::utils::HashSet;

/// Use this instead of [`AssetServer`] for loading all assets before game starts.
///
/// Should be used only in startup systems!
/// Assets won't be tracked if added in [`AllAssetsState::LoadingDone`] state!
#[derive(SystemParam)]
pub struct TrackAssets<'w> {
    asset_server: Res<'w, AssetServer>,
    loading: ResMut<'w, LoadingAssets>,
}

impl<'w> TrackAssets<'w> {
    pub fn load_and_track<'a, A: Asset>(
        &mut self,
        asset_path: impl Into<AssetPath<'a>>,
    ) -> Handle<A> {
        let handle = self.asset_server.load(asset_path);
        self.loading.ids.insert(handle.id().untyped());
        handle
    }
}

/// Sent when all assets are loaded, only once per application run.
///
/// Sent in [`Last`] schedule.
#[derive(Event)]
pub struct LoadedTrackedAssets;

///
#[derive(States, Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum TrackedAssetsState {
    /// Initial state - assets are still loading
    #[default]
    StillLoading,

    /// All assets are loaded - state will remain set for the rest of run time
    Loaded,
}

/// Information about how loading progress
#[derive(Resource, Default)]
pub struct TrackedAssetsInfo {
    /// How many assets failed to load
    pub errors: usize,
}

//

pub struct LoadAssetsPlugin;

impl Plugin for LoadAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LoadingAssets>()
            .init_resource::<TrackedAssetsInfo>()
            .add_event::<LoadedTrackedAssets>()
            .add_state::<TrackedAssetsState>()
            .add_systems(
                Last,
                send_assets_loaded_event.run_if(in_state(TrackedAssetsState::StillLoading)),
            );
    }
}

#[derive(Resource, Default)]
struct LoadingAssets {
    /// Assets which are still loading
    ids: HashSet<UntypedAssetId>,
}

/// Remove assets which are loaded, send [`AllAssetsLoaded`] and
/// switch to [`AllAssetsState::LoadingDone`] when all assets are loaded.
fn send_assets_loaded_event(
    asset_server: Res<AssetServer>,
    mut tracked_assets: ResMut<LoadingAssets>,
    mut event: EventWriter<LoadedTrackedAssets>,
    mut state: ResMut<NextState<TrackedAssetsState>>,
    mut info: ResMut<TrackedAssetsInfo>,
) {
    // retain only assets which are still loading
    tracked_assets.ids.retain(|id| {
        let path = || {
            asset_server
                .get_path(*id)
                .map(|path| path.to_string())
                .unwrap_or_else(|| "???".to_string())
        };

        match asset_server.get_load_states(*id) {
            Some((load, deps, _)) => match load {
                LoadState::NotLoaded | LoadState::Loading => true,
                LoadState::Loaded => match deps {
                    DependencyLoadState::NotLoaded | DependencyLoadState::Loading => true,
                    DependencyLoadState::Loaded => false,
                    DependencyLoadState::Failed => {
                        info.errors += 1;
                        false
                    }
                },
                LoadState::Failed => {
                    info.errors += 1;
                    false
                }
            },
            None => {
                error!("get_load_state is None for \"{}\"", path());
                false
            }
        }
    });

    if tracked_assets.ids.is_empty() {
        event.send(LoadedTrackedAssets);
        state.set(TrackedAssetsState::Loaded);
    }
}
