pub mod beam;
pub mod central;

use crate::path::Point;
use std::{f64::consts::TAU, time::Duration};

pub trait Stepper {
    fn min_step_interval(&self) -> Duration;

    fn target_step(&self, target: Point) -> eyre::Result<TargetStep>;

    fn delta_steps(&self, current: CurrentStep, target: TargetStep) -> eyre::Result<DeltaSteps> {
        Ok(DeltaSteps(target.0 - current.0))
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct CurrentStep(pub i16);

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct DeltaSteps(pub i16);

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct TargetStep(pub i16);

struct Motor {
    steps_per_rev: i16,

    /// in radians per second
    max_velocity: f64,
}

impl Motor {
    fn min_step_interval(self) -> Duration {
        Duration::from_secs_f64(1.0 / self.steps_per_sec())
    }

    fn steps_per_sec(self) -> f64 {
        let rev_per_sec = self.max_velocity / TAU;
        f64::from(self.steps_per_rev) * rev_per_sec
    }
}
