use bevy::prelude::*;

pub mod load_assets;
pub mod log_plugin;
pub mod userdata_plugin;
pub mod ron_asset;

/// All utility plugins except for log plugin - as it must be added before any other plugins,
/// while others may depend on bevy plugins.
pub struct UtilPlugin;

impl Plugin for UtilPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            userdata_plugin::UserdataPlugin,
            load_assets::LoadAssetsPlugin,
        ));
    }
}
