use bevy::asset::AssetPath;
use bevy::asset::DependencyLoadState;
use bevy::asset::LoadState;
use bevy::asset::UntypedAssetId;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy::utils::HashSet;

/// Use this instead of [`AssetServer`] for loading all assets before game starts.
///
/// Loads assets and starts tracking it's loading state.
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

/// Sent when all assets are loaded, only once per application run!
#[derive(Event)]
pub struct LoadedAllAssets;

#[derive(States, Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum AllAssetsState {
    /// Initial state - assets are still loading
    #[default]
    StillLoading,

    /// All assets are loaded - state will remain set for the rest of run time
    LoadingDone,
}

/// Information about how loading goes
#[derive(Resource, Default)]
pub struct AssetLoadInfo {
    pub errors: usize,
}

///
pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LoadingAssets>()
            .init_resource::<AssetLoadInfo>()
            .add_event::<LoadedAllAssets>()
            .add_state::<AllAssetsState>()
            .add_systems(
                First,
                send_assets_loaded_event.run_if(in_state(AllAssetsState::StillLoading)),
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
    mut event: EventWriter<LoadedAllAssets>,
    mut state: ResMut<NextState<AllAssetsState>>,
    mut info: ResMut<AssetLoadInfo>,
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
        event.send(LoadedAllAssets);
        state.set(AllAssetsState::LoadingDone);
    }
}
