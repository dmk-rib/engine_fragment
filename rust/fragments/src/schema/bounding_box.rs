use crate::schema::FloatVector;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoundingBox {
    pub min: FloatVector,
    pub max: FloatVector,
}

impl BoundingBox {
    pub fn new(min: FloatVector, max: FloatVector) -> Self {
        Self { min, max }
    }
}

impl Default for BoundingBox {
    fn default() -> Self {
        Self {
            min: FloatVector::default(),
            max: FloatVector::default(),
        }
    }
}
