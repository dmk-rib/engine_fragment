use crate::schema::FloatVector;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CircleCurve {
    pub aperture: f32,
    pub position: FloatVector,
    pub radius: f32,
    pub x_direction: FloatVector,
    pub y_direction: FloatVector,
}

impl CircleCurve {
    pub fn new(
        aperture: f32,
        position: FloatVector,
        radius: f32,
        x_direction: FloatVector,
        y_direction: FloatVector,
    ) -> Self {
        Self {
            aperture,
            position,
            radius,
            x_direction,
            y_direction,
        }
    }
}

impl Default for CircleCurve {
    fn default() -> Self {
        Self {
            aperture: 0.0,
            position: FloatVector::default(),
            radius: 0.0,
            x_direction: FloatVector::default(),
            y_direction: FloatVector::default(),
        }
    }
}
