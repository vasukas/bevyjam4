use super::master::level::data::HALF_TILE;
use super::master::level::data::TILE_SIZE;
use crate::utils::bevy::commands::FallibleCommands;
use crate::utils::bevy::misc_utils::ExtendedTimer;
use crate::utils::math_algorithms::lerp;
use crate::utils::random::RandomVec;
use bevy::prelude::*;
use leafwing_input_manager::orientation::Orientation;
use leafwing_input_manager::orientation::Rotation;
use std::f32::consts::TAU;
use std::time::Duration;

/// Tile which position belongs to
pub fn pos_to_tile(mut pos: Vec2) -> IVec2 {
    if pos.x < 0. {
        pos.x -= 1.
    }
    if pos.y < 0. {
        pos.y -= 1.
    }

    let tile = (pos / TILE_SIZE).floor().as_ivec2();
    tile
}

/// Center of the tile
pub fn tile_center(tile: IVec2) -> Vec2 {
    tile.as_vec2() * TILE_SIZE + HALF_TILE
}

/// Center of tile which position belongs to
pub fn pos_to_tile_center(pos: Vec2) -> Vec2 {
    tile_center(pos_to_tile(pos))
}

/// Rotation from 2D direction (may be non-normalized)
pub fn rotation_from_dir(dir: Vec2) -> Quat {
    dir.try_normalize()
        .map(|dir| {
            let angle = -dir.angle_between(Vec2::X);
            Quat::from_rotation_z(angle)
        })
        .unwrap_or_default()
}

//

/// Gradually rotates entity in target direction
#[derive(Component)]
pub struct RotateToTarget {
    pub target_dir: Vec2,
    pub rotation_speed: f32,
}

impl RotateToTarget {
    pub fn new(rotation_speed: f32) -> Self {
        Self {
            target_dir: Vec2::Y,
            rotation_speed,
        }
    }

    pub fn new_from_time(seconds_full_360: f32) -> Self {
        Self::new(TAU / seconds_full_360)
    }

    pub fn random(mut self) -> Self {
        self.target_dir = Vec2::random_dir();
        self
    }
}

/// Interpolate (linearly) from current to target transform, once (then this component is removed)
#[derive(Component, Default)]
pub struct InterpolateTransformOnce {
    target_pos: Option<Vec3>,
    target_rotation: Option<Quat>,
    target_scale: Option<Vec3>,

    timer: Timer,
    start: Option<Transform>,
}

impl InterpolateTransformOnce {
    pub fn new(duration: Duration) -> Self {
        Self {
            timer: Timer::once(duration),
            ..default()
        }
    }

    pub fn pos(mut self, target: Vec3) -> Self {
        self.target_pos = target.into();
        self
    }

    pub fn rotation(mut self, target: Quat) -> Self {
        self.target_rotation = target.into();
        self
    }

    pub fn scale(mut self, target: Vec3) -> Self {
        self.target_scale = target.into();
        self
    }
}

/// Entity will be despawned after that time (uses virtual time)
#[derive(Component)]
pub struct Lifetime(pub Duration);

/// The plugin
pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (rotate_to_target, interpolate_transform_once, despawn_after),
        );
    }
}

pub fn rotate_to_target(mut entities: Query<(&mut Transform, &RotateToTarget)>, time: Res<Time>) {
    for (mut transform, target) in entities.iter_mut() {
        // TODO: why negative? needed for player character
        let target_angle = -target.target_dir.angle_between(Vec2::X);
        let target_rot = Quat::from_rotation_z(target_angle);

        let max_delta = target.rotation_speed * time.delta_seconds();
        let max_rotation = Rotation::from_radians(max_delta);

        transform
            .rotation
            .rotate_towards(target_rot, Some(max_rotation));
    }
}

fn interpolate_transform_once(
    mut entities: Query<(Entity, &mut Transform, &mut InterpolateTransformOnce)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut transform, mut data) in entities.iter_mut() {
        let t = match data.timer.tick(time.delta()).finished() {
            true => {
                commands.try_remove::<InterpolateTransformOnce>(entity);
                1.
            }
            false => data.timer.t_elapsed(),
        };

        let start = *data.start.get_or_insert(*transform);

        if let Some(target) = data.target_pos {
            transform.translation = lerp(start.translation, target, t);
        }
        if let Some(target) = data.target_rotation {
            transform.rotation = start.rotation.slerp(target, t);
        }
        if let Some(target) = data.target_scale {
            transform.scale = lerp(start.scale, target, t);
        }
    }
}

#[derive(Component)]
struct DespawnAt(Duration);

fn despawn_after(
    new: Query<(Entity, &Lifetime), Added<Lifetime>>,
    entities: Query<(Entity, &DespawnAt)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, after) in new.iter() {
        match after.0.checked_sub(time.delta()) {
            Some(left) => commands.try_insert(entity, DespawnAt(time.elapsed() + left)),
            None => commands.try_despawn_recursive(entity),
        }
    }

    for (entity, at) in entities.iter() {
        if time.elapsed() >= at.0 {
            commands.try_despawn_recursive(entity);
        }
    }
}
