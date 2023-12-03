use bevy::prelude::*;

/// Parameters controlling in-game time
#[derive(Resource, Default)]
pub struct TimeMaster {
    pub in_menu: bool,
    pub in_editor: bool,
}

impl TimeMaster {
    fn factor(&self) -> f64 {
        if self.in_menu || self.in_editor {
            0.
        } else {
            1.
        }
    }
}

pub struct TimeMasterPlugin;

impl Plugin for TimeMasterPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TimeMaster>().add_systems(
            Last,
            update_time_factor.run_if(resource_changed::<TimeMaster>()),
        );
    }
}

fn update_time_factor(control: Res<TimeMaster>, mut time: ResMut<Time<Virtual>>) {
    time.set_relative_speed_f64(control.factor());
}
