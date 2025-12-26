use crate::schema::Axis;

#[derive(Debug, Clone, PartialEq)]
pub struct CircleExtrusion {
    pub radius: Vec<f64>,
    pub axes: Vec<Axis>,
}

impl CircleExtrusion {
    pub fn new(radius: Vec<f64>, axes: Vec<Axis>) -> Self {
        Self { radius, axes }
    }
}

impl Default for CircleExtrusion {
    fn default() -> Self {
        Self {
            radius: Vec::new(),
            axes: Vec::new(),
        }
    }
}
