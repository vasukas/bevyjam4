//! Random number generation

use super::math_algorithms::dir_vec2;
use bevy::math::*;
use rand::distributions::uniform::SampleUniform;
pub use rand::{thread_rng, Rng};
use std::f32::consts::TAU;

/// Random generation of [`bool`]
pub trait RandomBool {
    /// Randomly returns true with specified probability in `[0; 1]` range
    fn true_with_chance(probability: f32) -> Self;
}

impl RandomBool for bool {
    fn true_with_chance(probability: f32) -> Self {
        thread_rng().gen_bool(probability.into())
    }
}

/// Random generation of math vectors
pub trait RandomVec {
    /// Normalized direction
    fn random_dir() -> Self;
}

impl RandomVec for Vec2 {
    fn random_dir() -> Self {
        dir_vec2((0. ..TAU).random())
    }
}

/// Random value from a range
pub trait RandomRange<T> {
    /// If range is empty, returns start value
    fn random(self) -> T;
}

impl<T: PartialOrd + SampleUniform> RandomRange<T> for std::ops::Range<T> {
    fn random(self) -> T {
        if self.is_empty() {
            return self.start;
        }
        thread_rng().gen_range(self)
    }
}

impl<T: PartialOrd + SampleUniform + Copy> RandomRange<T> for std::ops::RangeInclusive<T> {
    fn random(self) -> T {
        if self.is_empty() {
            return *self.start();
        }
        thread_rng().gen_range(self)
    }
}
