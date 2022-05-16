use super::{DeltaSteps, Motor, Stepper, TargetStep};
use crate::{angle::degrees_to_radians, named::Named, path::Point, vector_2};
use eyre::WrapErr;
use std::{f64::consts::TAU, time::Duration};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CentralStepper {
    steps_per_rev: i16,
    min_step_interval: Duration,
}

impl Named for CentralStepper {
    fn name(&self) -> &'static str {
        "central stepper"
    }
}

impl Stepper for CentralStepper {
    fn min_step_interval(&self) -> Duration {
        self.min_step_interval
    }

    fn target_step(&self, target: Point) -> eyre::Result<TargetStep> {
        let steps_per_rev = f64::from(self.steps_per_rev);
        let target_vec = target.location.coords;
        let angle = target_vec.angle(&vector_2::forward());
        let orientation = if target_vec.x >= 0.0 { angle } else { -angle };
        let target_step = orientation * steps_per_rev / TAU;
        let target_step = i16::try_from(target_step.round() as i32).wrap_err_with(|| {
            format!("failed to convert target step {target_step} to i16. target: {target:?}")
        })?;
        Ok(TargetStep(target_step))
    }

    fn delta_steps(
        &self,
        current: super::CurrentStep,
        target: super::TargetStep,
    ) -> eyre::Result<DeltaSteps> {
        Ok(DeltaSteps((target.0 - current.0) % self.steps_per_rev))
    }
}

impl Default for CentralStepper {
    fn default() -> Self {
        Builder::default().build()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Builder {
    steps_per_rev: i16,

    /// in radians
    max_velocity: f64,
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            steps_per_rev: 400,
            max_velocity: degrees_to_radians(90.0),
        }
    }
}

impl Builder {
    fn build(self) -> CentralStepper {
        CentralStepper {
            steps_per_rev: self.steps_per_rev,
            min_step_interval: self.min_step_interval(),
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Point2;

    #[test]
    fn nalgebra_angle_is_in_radians() {
        let actual = vector_2::forward().angle(&vector_2::right());
        assert_eq!(actual, TAU * 0.25);
    }

    #[test]
    fn default_bit_rate() {
        let message_size = 32.0;
        let bit_rate = Builder::default().motor().steps_per_sec() * message_size;
        assert!(bit_rate <= 9600.0, "actual = {bit_rate}");
        assert_eq!(bit_rate, 3200.0);
    }

    const TEST_STEPPER: CentralStepper = CentralStepper {
        steps_per_rev: 1200,
        min_step_interval: Duration::from_millis(0),
    };

    #[test]
    fn target_step_front_right() {
        let target = Point {
            location: Point2::new(10.0, -10.0),
            ..Default::default()
        };
        let actual = TEST_STEPPER.target_step(target).unwrap().0;
        assert_eq!(actual, 150);
    }

    #[test]
    fn target_step_back_right() {
        let target = Point {
            location: Point2::new(10.0, 10.0),
            ..Default::default()
        };
        let actual = TEST_STEPPER.target_step(target).unwrap().0;
        assert_eq!(actual, 450);
    }

    #[test]
    fn target_step_back_left() {
        let target = Point {
            location: Point2::new(-10.0, 10.0),
            ..Default::default()
        };
        let actual = TEST_STEPPER.target_step(target).unwrap().0;
        assert_eq!(actual, -450);
    }

    #[test]
    fn target_step_front_left() {
        let target = Point {
            location: Point2::new(-10.0, -10.0),
            ..Default::default()
        };
        let actual = TEST_STEPPER.target_step(target).unwrap().0;
        assert_eq!(actual, -150);
    }
}
