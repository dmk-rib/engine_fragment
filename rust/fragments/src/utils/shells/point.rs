use super::GeomsFbUtils;

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub hash: String,
    pub id: usize,
}

impl Point {
    pub fn new(vertices: &[f32], index: usize, id: usize, precision: f32) -> Self {
        let x = GeomsFbUtils::round(vertices[index * 3], precision);
        let y = GeomsFbUtils::round(vertices[index * 3 + 1], precision);
        let z = GeomsFbUtils::round(vertices[index * 3 + 2], precision);
        let hash = format!("{x}/{y}/{z}");
        Self { x, y, z, hash, id }
    }
}
