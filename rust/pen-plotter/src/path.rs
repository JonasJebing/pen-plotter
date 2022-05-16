use eyre::eyre;
use nalgebra::Point2;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Path {
    pub start: Point2<f64>,
    pub end: Point2<f64>,
    pub velocity: f64,
}

impl Path {
    pub fn interpolate(self, fraction: f64) -> Point {
        let fraction = fraction.clamp(0.0, 1.0);
        Point {
            location: self.start.coords.lerp(&self.end.coords, fraction).into(),
            private: (),
        }
    }

    pub fn duration(self) -> eyre::Result<Duration> {
        let velocity = self.velocity;
        if !velocity.is_normal() {
            return Err(eyre!("velocity {velocity} is not normal"));
        }
        let distance = self.distance();
        if !distance.is_normal() {
            return Err(eyre!("distance {distance} is not normal"));
        }
        let secs = 1.0 / (velocity / distance);
        Ok(Duration::from_secs_f64(secs))
    }

    fn distance(self) -> f64 {
        nalgebra::distance(&self.start, &self.end)
    }
}

impl Default for Path {
    fn default() -> Self {
        Self {
            start: Point2::new(0.0, -10.0),
            end: Point2::origin(),
            velocity: 1.2,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Point {
    pub location: Point2<f64>,
    #[doc(hidden)]
    pub private: (),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interpolate() {
        let path = Path {
            start: Point2::new(0.0, 0.0),
            end: Point2::new(12.0, 16.0),
            ..Default::default()
        };
        let actual = path.interpolate(0.75).location;
        let expected = Point2::new(9.0, 12.0);
        assert_eq!(actual, expected);
    }

    #[test]
    fn duration() {
        let path = Path {
            start: Point2::new(0.0, 0.0),
            end: Point2::new(3.0, 4.0),
            velocity: 50.0,
        };
        let actual = path.duration().unwrap();
        assert_eq!(actual, Duration::from_millis(100));
    }

    #[test]
    fn duration_zero_distance() {
        let path = Path {
            start: Point2::new(0.0, 0.0),
            end: Point2::new(0.0, 0.0),
            velocity: 50.0,
        };
        let _ = path.duration().unwrap_err();
    }

    #[test]
    fn duration_zero_velocity() {
        let path = Path {
            start: Point2::new(2.0, 1.0),
            end: Point2::new(3.0, 5.0),
            velocity: 0.0,
        };
        let _ = path.duration().unwrap_err();
    }

    #[test]
    fn distance() {
        let path = Path {
            start: Point2::new(0.0, 0.0),
            end: Point2::new(3.0, 4.0),
            ..Default::default()
        };
        let actual = path.distance();
        assert_eq!(actual, 5.0);
    }
}
