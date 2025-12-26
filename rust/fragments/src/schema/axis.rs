use crate::schema::{AxisPartClass, CircleCurve, Wire, WireSet};

#[derive(Debug, Clone, PartialEq)]
pub struct Axis {
    pub wires: Vec<Wire>,
    pub order: Vec<u32>,
    pub parts: Vec<AxisPartClass>,
    pub wire_sets: Vec<WireSet>,
    pub circle_curves: Vec<CircleCurve>,
}

impl Axis {
    pub fn new(
        wires: Vec<Wire>,
        order: Vec<u32>,
        parts: Vec<AxisPartClass>,
        wire_sets: Vec<WireSet>,
        circle_curves: Vec<CircleCurve>,
    ) -> Self {
        Self {
            wires,
            order,
            parts,
            wire_sets,
            circle_curves,
        }
    }
}

impl Default for Axis {
    fn default() -> Self {
        Self {
            wires: Vec::new(),
            order: Vec::new(),
            parts: Vec::new(),
            wire_sets: Vec::new(),
            circle_curves: Vec::new(),
        }
    }
}
