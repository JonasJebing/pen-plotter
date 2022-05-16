use std::f64::consts::TAU;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * TAU / 360.0
}
