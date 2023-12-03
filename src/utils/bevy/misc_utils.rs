//! Various small utilities for [`bevy`]` crate.

use bevy::{
    ecs::{query::ReadOnlyWorldQuery, system::SystemParam},
    prelude::*,
};
use std::time::Duration;

/// Iterate over all children of the entity, their children, and so on.
///
/// Order in which entities are passed to callback is unspecified.
pub fn iterate_children_recursively(
    root: Entity,
    children: &Query<&Children>,
    mut callback: impl FnMut(Entity),
) {
    let mut entities = vec![root];
    while let Some(entity) = entities.pop() {
        callback(entity);
        if let Ok(children) = children.get(entity) {
            entities.extend(children.iter());
        }
    }
}

/// Helper methods for [`Timer`]
pub trait ExtendedTimer {
    /// Run timer once
    fn once(duration: Duration) -> Self;

    /// How much time elapsed since start, normalized to `[0; 1]` range
    fn t_elapsed(&self) -> f32;
}

impl ExtendedTimer for Timer {
    fn once(duration: Duration) -> Self {
        Self::new(duration, TimerMode::Once)
    }

    fn t_elapsed(&self) -> f32 {
        self.elapsed_secs() / self.duration().as_secs_f32()
    }
}

/// Helper methods for [`Gizmos`]
pub trait ExtendedGizmos {
    fn arrow(&mut self, start: Vec3, delta: Vec3, color: Color);
}

impl ExtendedGizmos for Gizmos<'_> {
    fn arrow(&mut self, start: Vec3, delta: Vec3, color: Color) {
        let tip_length = 0.25;

        let end = start + delta;
        self.line(start, end, color);

        let tip_length = tip_length * delta.length();
        let rotation = Quat::from_rotation_arc(Vec3::X, delta.try_normalize().unwrap_or(Vec3::X));
        let tips = [
            Vec3::new(-1., 1., 0.),
            Vec3::new(-1., 0., 1.),
            Vec3::new(-1., -1., 0.),
            Vec3::new(-1., 0., -1.),
            Vec3::new(-1., 0.5, 0.5),
            Vec3::new(-1., 0.5, -0.5),
            Vec3::new(-1., -0.5, 0.5),
            Vec3::new(-1., -0.5, -0.5),
        ];
        let tips = tips.map(|v| rotation * (v.normalize() * tip_length) + end);
        for v in tips {
            self.line(end, v, color);
        }
    }
}

/// Immediate transform propagation.
///
/// **At the moment this doesn't work with hierarchies!**
#[derive(SystemParam)]
pub struct ImmediateTransformUpdate<'w, 's, Filter: ReadOnlyWorldQuery + 'static = ()> {
    pub transform: Query<'w, 's, (&'static mut Transform, Has<Parent>), Filter>,
    pub global: Query<'w, 's, &'static mut GlobalTransform, Filter>,
}

impl<'w, 's, Filter: ReadOnlyWorldQuery + 'static> ImmediateTransformUpdate<'w, 's, Filter> {
    /// Silently fails on errors
    pub fn update_inplace(&mut self, entity: Entity, new_transform: impl FnOnce(&mut Transform)) {
        let Ok(result) = self.transform.get(entity) else { return; };
        let mut transform = *result.0;

        new_transform(&mut transform);

        self.update(entity, transform);
    }

    /// Silently fails on errors
    pub fn update(&mut self, entity: Entity, new_transform: Transform) {
        let Ok((mut transform, has_parent)) = self.transform.get_mut(entity) else { return; };

        if has_parent {
            error!("ImmediateTransformUpdate used for entity which has parent! ignoring!");
            return;
        }

        *transform = new_transform;

        if let Ok(mut global) = self.global.get_mut(entity) {
            *global = new_transform.into();
        }
    }
}
