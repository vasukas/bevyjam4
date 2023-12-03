#![allow(dead_code)] // TODO: temporary (used only for debug)

use crate::utils::bevy_egui::*;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy::window::PrimaryWindow;
use bevy_egui::EguiSet;
use itertools::Itertools;

#[derive(SystemParam)]
pub struct AdvancedGizmos<'w> {
    data: ResMut<'w, AdvancedGizmosData>,
}

impl<'w> AdvancedGizmos<'w> {
    /// Draw text at world position
    pub fn world_text(&mut self, pos: Vec2, text: impl AsRef<str>) {
        self.data.world_text.push((pos, text.as_ref().to_string()))
    }

    /// Draw text at world position of the entity.
    ///
    /// Multiple calls for same entity will append text.
    pub fn entity_text(&mut self, entity: Entity, text: impl AsRef<str>) {
        *self.data.entity_text.entry(entity).or_default() += text.as_ref()
    }
}

pub struct AdvancedGizmosPlugin;

impl Plugin for AdvancedGizmosPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AdvancedGizmosData>().add_systems(
            PostUpdate,
            draw_advanced_gizmos.before(EguiSet::ProcessOutput),
        );
    }
}

#[derive(Resource, Default)]
struct AdvancedGizmosData {
    world_text: Vec<(Vec2, String)>,
    entity_text: HashMap<Entity, String>,
}

fn draw_advanced_gizmos(
    mut egui_ctx: EguiContexts,
    mut gizmos: ResMut<AdvancedGizmosData>,
    cameras: Query<(&GlobalTransform, &Camera)>,
    primary_window: Query<(), With<PrimaryWindow>>,
    transforms: Query<&GlobalTransform>,
) {
    let Some((camera_transform, camera)) = cameras.iter().sorted_by_key(|v| v.1.order).last() else {
        *gizmos = default();
        return;
    };

    if primary_window.is_empty() {
        // Don't panic on exiting app.
        // This happens only when ctx_mut() is used in PostUpdate after window was closed.
        return;
    }
    let painter = egui_ctx.ctx_mut().debug_painter();

    let text_at_pos = |pos: Vec2, text| {
        painter.debug_text(
            pos.to_egui_pos(),
            egui::Align2::LEFT_TOP,
            Color::WHITE.to_egui(),
            text,
        );
    };

    for (pos, text) in gizmos.world_text.drain(..) {
        let pos = camera.world_to_viewport(camera_transform, pos.extend(0.));
        let Some(pos) = pos else { continue; };

        text_at_pos(pos, text);
    }

    for (entity, text) in gizmos.entity_text.drain() {
        let Ok(global_transform) = transforms.get(entity) else { continue; };
        let pos = camera.world_to_viewport(camera_transform, global_transform.translation());
        let Some(pos) = pos else { continue; };

        text_at_pos(pos, text);
    }
}
