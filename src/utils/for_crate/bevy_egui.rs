use bevy::prelude::*;

pub use bevy_egui::{egui, EguiContexts};

/// Convert [`Color`]
pub trait BevyEguiColor {
    fn to_egui(self) -> egui::Color32;
    fn from_egui(color: egui::Color32) -> Self;
}

impl BevyEguiColor for Color {
    fn to_egui(self) -> egui::Color32 {
        let v = self.as_rgba_f32().map(|v| (v * 255.).clamp(0., 255.) as u8);
        egui::Color32::from_rgba_unmultiplied(v[0], v[1], v[2], v[3])
    }

    fn from_egui(color: egui::Color32) -> Self {
        let v = color.to_srgba_unmultiplied().map(|v| v as f32 / 255.);
        Color::rgba(v[0], v[1], v[2], v[3])
    }
}

/// Convert [`Vec2`]
pub trait BevyEguiVec2 {
    fn to_egui(self) -> egui::Vec2;
    fn to_egui_pos(self) -> egui::Pos2;
}

impl BevyEguiVec2 for Vec2 {
    fn to_egui(self) -> egui::Vec2 {
        egui::vec2(self.x, self.y)
    }
    fn to_egui_pos(self) -> egui::Pos2 {
        egui::pos2(self.x, self.y)
    }
}

/// Popup window description
pub struct EguiPopup<'a> {
    /// Unique ID (unique only within current frame)
    pub name: &'a str,

    /// Which part of the screen to anchor to
    pub anchor: egui::Align2,

    /// Draw frame and background or not
    pub background: bool,

    /// Draw order
    pub order: egui::Order,

    /// Receive input. If false, mouse events will go to UI elements with lower order
    pub interactable: bool,
}

impl Default for EguiPopup<'_> {
    fn default() -> Self {
        Self {
            name: default(),
            anchor: egui::Align2::CENTER_CENTER,
            background: true,
            order: egui::Order::Middle,
            interactable: true,
        }
    }
}

impl<'a> EguiPopup<'a> {
    pub fn show(self, ctx: &mut egui::Context, add_contents: impl FnOnce(&mut egui::Ui)) {
        egui::Area::new(self.name.to_string())
            .anchor(self.anchor, egui::Vec2::ZERO)
            .order(self.order)
            .interactable(self.interactable)
            .show(ctx, |ui| {
                if self.background {
                    egui::Frame::popup(&ui.style()).show(ui, add_contents);
                } else {
                    (add_contents)(ui)
                }
            });
    }
}
