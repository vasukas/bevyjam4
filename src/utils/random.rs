//! Random number generation

pub use rand::{thread_rng, Rng};

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
