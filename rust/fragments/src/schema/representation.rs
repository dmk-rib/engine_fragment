use crate::schema::{BoundingBox, RepresentationClass};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Representation {
    pub id: u32,
    pub bbox: BoundingBox,
    pub representation_class: RepresentationClass,
}

impl Representation {
    pub fn new(id: u32, bbox: BoundingBox, representation_class: RepresentationClass) -> Self {
        Self {
            id,
            bbox,
            representation_class,
        }
    }
}

impl Default for Representation {
    fn default() -> Self {
        Self {
            id: 0,
            bbox: BoundingBox::default(),
            representation_class: RepresentationClass::default(),
        }
    }
}
