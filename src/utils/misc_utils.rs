//! Miscellaneous utilities: very small or hard to fit in particular category

use bevy::prelude::*;

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
