use crate::schema::{DoubleVector, FloatVector};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    pub position: DoubleVector,
    pub x_direction: FloatVector,
    pub y_direction: FloatVector,
}

impl Transform {
    pub fn new(position: DoubleVector, x_direction: FloatVector, y_direction: FloatVector) -> Self {
        Self {
            position,
            x_direction,
            y_direction,
        }
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: DoubleVector::default(),
            x_direction: FloatVector::default(),
            y_direction: FloatVector::default(),
        }
    }
}
