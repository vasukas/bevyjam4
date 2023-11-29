use bevy::prelude::*;
use bevy_egui::egui;

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
