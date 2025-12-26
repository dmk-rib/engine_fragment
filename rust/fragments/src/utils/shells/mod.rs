mod edge;
mod face;
mod faces;
mod math;
mod plane;
mod point;
mod points;
mod profile;
mod profiles;

pub use edge::Edge;
pub use face::Face;
pub use faces::Faces;
pub use math::{Vector2, Vector3};
pub use plane::Plane;
pub use point::Point;
pub use points::Points;
pub use profile::Profile;
pub use profiles::{ProfileData, Profiles};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aabb {
    pub min: Vector3,
    pub max: Vector3,
}

pub struct GeomsFbUtils;

impl GeomsFbUtils {
    pub const USHORT_MAX_VALUE: usize = 65_000;

    pub fn round(value: f32, precision: f32) -> f32 {
        (value * precision).round() / precision
    }

    pub fn get_aabb(vertices: &[f32]) -> Aabb {
        let mut min_x = f32::INFINITY;
        let mut min_y = f32::INFINITY;
        let mut min_z = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut max_y = f32::NEG_INFINITY;
        let mut max_z = f32::NEG_INFINITY;

        for chunk in vertices.chunks_exact(3) {
            let x = chunk[0];
            let y = chunk[1];
            let z = chunk[2];

            if x < min_x {
                min_x = x;
            }
            if y < min_y {
                min_y = y;
            }
            if z < min_z {
                min_z = z;
            }
            if x > max_x {
                max_x = x;
            }
            if y > max_y {
                max_y = y;
            }
            if z > max_z {
                max_z = z;
            }
        }

        Aabb {
            min: Vector3::new(min_x, min_y, min_z),
            max: Vector3::new(max_x, max_y, max_z),
        }
    }
}
