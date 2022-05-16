use nalgebra::Vector2;

#[allow(dead_code)]
pub fn right() -> Vector2<f64> {
    Vector2::new(1.0, 0.0)
}

#[allow(dead_code)]
pub fn left() -> Vector2<f64> {
    -right()
}

pub fn back() -> Vector2<f64> {
    Vector2::new(0.0, 1.0)
}

pub fn forward() -> Vector2<f64> {
    -back()
}
