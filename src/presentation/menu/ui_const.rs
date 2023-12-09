use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::EguiSettings;

/// Screen size independent values. I think.
#[derive(SystemParam)]
pub struct UiConst<'w, 's> {
    window: Query<'w, 's, &'static Window, With<PrimaryWindow>>,
    egui_settings: Res<'w, EguiSettings>,
}

impl<'w, 's> UiConst<'w, 's> {
    /// Scale for constants in code
    pub fn scale(&self) -> f32 {
        let dev_window_height = 720.;
        let window_height = self
            .window
            .get_single()
            .map(|window| window.height())
            .unwrap_or(dev_window_height);
        let k_window = window_height / dev_window_height;

        let dev_scale = 2.;
        let scale = self.egui_settings.scale_factor as f32;
        let k_scale = scale / dev_scale;

        k_window * k_scale
    }

    /// Scale set in [`EguiSettings`]
    pub fn egui_scale_factor(&self) -> f32 {
        self.egui_settings.scale_factor as f32
    }
}
