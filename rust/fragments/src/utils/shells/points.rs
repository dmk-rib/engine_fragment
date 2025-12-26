use std::collections::HashMap;

use super::{Point, Vector3};

#[derive(Debug)]
pub struct Points {
    pub list: HashMap<String, Point>,
    temp_v1: Vector3,
    temp_v2: Vector3,
    temp_v3: Vector3,
    precision: f32,
}

impl Points {
    pub fn new(precision: f32) -> Self {
        Self {
            list: HashMap::new(),
            temp_v1: Vector3::new(0.0, 0.0, 0.0),
            temp_v2: Vector3::new(0.0, 0.0, 0.0),
            temp_v3: Vector3::new(0.0, 0.0, 0.0),
            precision,
        }
    }

    pub fn create(&mut self, vertices: &[f32], index: usize) -> Point {
        let point = Point::new(vertices, index, self.list.len(), self.precision);
        self.list
            .entry(point.hash.clone())
            .or_insert_with(|| point.clone());
        self.list.get(&point.hash).cloned().expect("point present")
    }

    pub fn values(&self) -> Vec<[f32; 3]> {
        self.list
            .values()
            .map(|point| [point.x, point.y, point.z])
            .collect()
    }

    pub fn is_valid_triangle(
        &mut self,
        position: &[f32],
        index1: usize,
        index2: usize,
        index3: usize,
    ) -> bool {
        self.temp_v1.set(
            position[index1 * 3],
            position[index1 * 3 + 1],
            position[index1 * 3 + 2],
        );
        self.temp_v2.set(
            position[index2 * 3],
            position[index2 * 3 + 1],
            position[index2 * 3 + 2],
        );
        self.temp_v3.set(
            position[index3 * 3],
            position[index3 * 3 + 1],
            position[index3 * 3 + 2],
        );

        let point_precision = (1.0 / self.precision) * 10.0;
        let d1_valid = self.temp_v1.distance_to(&self.temp_v2) > point_precision;
        let d2_valid = self.temp_v1.distance_to(&self.temp_v3) > point_precision;
        let d3_valid = self.temp_v2.distance_to(&self.temp_v3) > point_precision;

        d1_valid && d2_valid && d3_valid
    }
}
