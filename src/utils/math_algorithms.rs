//! Various mathematic algorithms

use bevy::math::*;

/// Linear interpolation (interpolant **is not clamped**)
pub fn lerp<T: std::ops::Add<Output = T> + std::ops::Mul<f32, Output = T>>(
    v0: T,
    v1: T,
    t: f32,
) -> T {
    v0 * (1. - t) + v1 * t // more precise than `v0 + t * (v1 - v0)`
}

/// Linearly maps values in input range to output range. May optionally clamp values to the range.
pub fn map_linear_range(
    value: f32,
    in_min: f32,
    in_max: f32,
    out_min: f32,
    out_max: f32,
    clamp: bool,
) -> f32 {
    let t = if (in_max - in_min).abs() < 1e-10 {
        0.
    } else {
        (value - in_min) / (in_max - in_min)
    };
    let t = if clamp { t.clamp(0., 1.) } else { t };
    lerp(out_min, out_max, t)
}

/// Rotate 2D vector
pub fn rotate_vec2(vec: Vec2, angle: f32) -> Vec2 {
    Affine2::from_angle(angle).transform_vector2(vec)
}

/// Normalized direction from angle
pub fn dir_vec2(angle: f32) -> Vec2 {
    rotate_vec2(Vec2::X, angle)
}
