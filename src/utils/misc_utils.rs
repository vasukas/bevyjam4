//! Miscellaneous utilities: very small or hard to fit in particular category

use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::Serialize;
use std::collections::BTreeMap;
use std::time::Duration;

/// Inverts color most contrasting to the one give. Alpha is unchanged.
pub fn invert_color(color: Color) -> Color {
    let mut color = color.as_hsla();
    match &mut color {
        Color::Hsla {
            hue,
            saturation,
            lightness,
            ..
        } => {
            if *lightness < 0.25 {
                *lightness = 1. - *lightness
            } else if *saturation < 0.25 {
                *saturation = 1. - *saturation
            } else {
                *hue = (*hue + 180.) % 360.
            }
        }
        _ => unreachable!(),
    }
    color
}

/// div_duration_f32 is still unstable
pub trait DurationDivF32 {
    fn div_dur_f32(&self, rhs: Duration) -> f32;
}

impl DurationDivF32 for Duration {
    fn div_dur_f32(&self, rhs: Duration) -> f32 {
        self.as_secs_f32() / rhs.as_secs_f32()
    }
}

/// Serialize hashmaps as ordered maps!
///
/// Use attribute `#[serde(serialize_with = "serde_sorted_map")]`
pub fn serde_sorted_map<S: serde::Serializer, K: Serialize + Ord, V: Serialize>(
    value: &HashMap<K, V>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let mut items: Vec<(_, _)> = value.iter().collect();
    items.sort_by(|a, b| a.0.cmp(&b.0));
    BTreeMap::from_iter(items).serialize(serializer)
}

/// Weird helper methods for [`EventReader`]
pub trait ExtendedEventReader<E> {
    /// Panics if there is more than one event
    fn read_single(&mut self, system_name: &str) -> Option<&E>;
}

impl<'w, 's, E: Event> ExtendedEventReader<E> for EventReader<'w, 's, E> {
    fn read_single(&mut self, system_name: &str) -> Option<&E> {
        let mut any = None;
        for event in self.read() {
            if any.is_some() {
                panic!("expected single event for {system_name}")
            }
            any = Some(event);
        }
        any
    }
}

/// Weird helper methods for [`Time`]
pub trait ExtendedTime {
    /// Returns true at specified period. First period starts at zero-o-clock plus offset.
    fn is_tick(&self, period: Duration, offset: Duration) -> bool;
}

impl<T: Default> ExtendedTime for Time<T> {
    fn is_tick(&self, period: Duration, offset: Duration) -> bool {
        let period = period.as_secs_f64();
        let now = self.elapsed_seconds_f64() + offset.as_secs_f64();
        let current = now / period;
        let previous = (now - self.delta_seconds_f64()) / period;
        current as isize != previous as isize
    }
}
