pub mod log_plugin;
pub mod userdata_plugin;

use bevy::prelude::*;

/// All utility plugins except for log plugin - as it must be added before any other plugins,
/// while others may depend on bevy plugins.
pub struct UtilPlugins;

impl Plugin for UtilPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((userdata_plugin::UserdataPlugin,));
    }
}
