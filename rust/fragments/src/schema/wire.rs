use crate::schema::FloatVector;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Wire {
    pub p1: FloatVector,
    pub p2: FloatVector,
}

impl Wire {
    pub fn new(p1: FloatVector, p2: FloatVector) -> Self {
        Self { p1, p2 }
    }
}

impl Default for Wire {
    fn default() -> Self {
        Self {
            p1: FloatVector::default(),
            p2: FloatVector::default(),
        }
    }
}
