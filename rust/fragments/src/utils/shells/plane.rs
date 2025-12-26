use super::{GeomsFbUtils, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub struct Plane {
    pub normal: Vector3,
    pub constant: f32,
    pub id: String,
    pub faces: Vec<usize>,
}

impl Plane {
    pub fn new(normal: Vector3, constant: f32, precision: f32, normal_precision: f32) -> Self {
        let nx = GeomsFbUtils::round(normal.x, normal_precision);
        let ny = GeomsFbUtils::round(normal.y, normal_precision);
        let nz = GeomsFbUtils::round(normal.z, normal_precision);
        let c = GeomsFbUtils::round(constant, precision);

        let plane_separator = "||";
        let id = format!("{nx}{plane_separator}{ny}{plane_separator}{nz}{plane_separator}{c}");

        Self {
            normal: Vector3::new(nx, ny, nz),
            constant: c,
            id,
            faces: Vec::new(),
        }
    }
}
