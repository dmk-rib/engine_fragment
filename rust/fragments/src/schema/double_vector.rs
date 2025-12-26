#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DoubleVector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl DoubleVector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl Default for DoubleVector {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}
