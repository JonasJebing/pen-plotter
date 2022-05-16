use super::{Motor, Stepper, TargetStep};
use crate::{angle::degrees_to_radians, named::Named, path::Point};
use eyre::{eyre, WrapErr};
use std::{ops::RangeInclusive, time::Duration};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BeamStepper {
    total_steps: i16,
    range: (f64, f64),
    min_step_interval: Duration,
}

impl BeamStepper {
    fn range(self) -> RangeInclusive<f64> {
        self.range.0..=self.range.1
    }
}

impl Named for BeamStepper {
    fn name(&self) -> &'static str {
        "beam stepper"
    }
}

impl Stepper for BeamStepper {
    fn min_step_interval(&self) -> Duration {
        self.min_step_interval
    }

    fn target_step(&self, target: Point) -> eyre::Result<TargetStep> {
        let target_distance = target.location.coords.magnitude();
        if !self.range().contains(&target_distance) {
            return Err(eyre!("invalid target distance {target_distance}"));
        }
        let step_distance = target_distance - self.range.0;
        let range_distance = self.range.1 - self.range.0;
        let step_fraction = step_distance / range_distance;
        let total_steps = f64::from(self.total_steps);
        let target_step = (total_steps * step_fraction).round() as i32;
        let target_step = i16::try_from(target_step).wrap_err_with(|| {
            format!("failed to convert target step {target_step} to i16. target: {target:?}")
        })?;
        Ok(TargetStep(target_step))
    }
}

impl Default for BeamStepper {
    fn default() -> Self {
        Builder::default().build()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Builder {
    range: (i16, i16),
    output_per_rev: i16,
    steps_per_rev: i16,

    /// in radians
    max_velocity: f64,
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            range: (10, 80),
            output_per_rev: 10,
            steps_per_rev: 400,
            max_velocity: degrees_to_radians(60.0),
        }
    }
}

impl Builder {
    fn build(self) -> BeamStepper {
        BeamStepper {
            total_steps: self.total_steps(),
            range: self.range_f64(),
            min_step_interval: self.min_step_interval(),
        }
    }

    fn total_steps(self) -> i16 {
        let range_distance = self.range.1 - self.range.0;
        range_distance * self.steps_per_rev / self.output_per_rev
    }

    fn range_f64(self) -> (f64, f64) {
        map_array_pair(self.range, Into::into)
    }

    fn min_step_interval(self) -> Duration {
        self.motor().min_step_interval()
    }

    fn motor(self) -> Motor {
        Motor {
            steps_per_rev: self.steps_per_rev,
            max_velocity: self.max_velocity,
        }
    }
}

fn map_array_pair<T, U>((a, b): (T, T), mut f: impl FnMut(T) -> U) -> (U, U) {
    (f(a), f(b))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Point2;

    #[test]
    fn default() {
        let expected = BeamStepper {
            total_steps: 2800,
            range: (10.0, 80.0),
            min_step_interval: Duration::from_millis(15),
        };
        assert_eq!(BeamStepper::default(), expected);
    }

    #[test]
    fn default_bit_rate() {
        let message_size = 32.0;
        let bit_rate = Builder::default().motor().steps_per_sec() * message_size;
        assert!(bit_rate <= 9600.0, "actual = {bit_rate}");
        assert_eq!(bit_rate, 2133.333333333333);
    }

    /// Use this instead of `BeamStepper::default()`
    /// so your test doesn't need to be adjusted when the default changes.
    const TEST_STEPPER: BeamStepper = BeamStepper {
        total_steps: 1000,
        range: (20.0, 80.0),
        min_step_interval: Duration::from_millis(0),
    };

    #[test]
    fn target_step() {
        let target = Point {
            location: Point2::new(30.0, 40.0),
            ..Default::default()
        };
        let actual = TEST_STEPPER.target_step(target).unwrap().0;
        assert_eq!(actual, 500);
    }

    #[test]
    fn target_out_of_reach() {
        let target = Point {
            location: Point2::new(100.0, 100.0),
            ..Default::default()
        };
        let _ = TEST_STEPPER.target_step(target).unwrap_err();
    }
}
