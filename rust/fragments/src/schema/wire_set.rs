use crate::schema::FloatVector;

#[derive(Debug, Clone, PartialEq)]
pub struct WireSet {
    pub ps: Vec<FloatVector>,
}

impl WireSet {
    pub fn new(ps: Vec<FloatVector>) -> Self {
        Self { ps }
    }
}

impl Default for WireSet {
    fn default() -> Self {
        Self { ps: Vec::new() }
    }
}
