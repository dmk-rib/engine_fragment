use crate::schema::{RenderedFaces, Stroke};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Material {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
    pub rendered_faces: RenderedFaces,
    pub stroke: Stroke,
}

impl Material {
    pub fn new(r: u8, g: u8, b: u8, a: u8, rendered_faces: RenderedFaces, stroke: Stroke) -> Self {
        Self {
            r,
            g,
            b,
            a,
            rendered_faces,
            stroke,
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
            rendered_faces: RenderedFaces::default(),
            stroke: Stroke::default(),
        }
    }
}
