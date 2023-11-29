use std::time::Duration;

pub trait DurationDivF32 {
    // it's never gonna be stabilized, is it?
    fn div_duration_f32_fr(&self, other: Duration) -> f32;
}

impl DurationDivF32 for Duration {
    fn div_duration_f32_fr(&self, other: Duration) -> f32 {
        self.as_secs_f32() / other.as_secs_f32()
    }
}
