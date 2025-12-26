use std::collections::HashMap;

use super::{Edge, Face, Plane};

#[derive(Debug, Default)]
pub struct Faces {
    pub list: HashMap<usize, Face>,
    pub next_face_id: usize,
}

impl Faces {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, triangle: Vec<Edge>, plane: Plane) {
        let matches = self.match_face_ids(&triangle, &plane);

        if matches.is_empty() {
            let face_id = self.next_face_id;
            self.next_face_id += 1;
            let mut face = Face::new(face_id, plane);
            face.add(triangle);
            self.list.insert(face.id, face);
            return;
        }

        if matches.len() == 1 {
            if let Some(face) = self.list.get_mut(&matches[0]) {
                face.add(triangle);
            }
            return;
        }

        if matches.len() > 1 {
            let base_face_id = matches[0];
            if let Some(base_face) = self.list.get_mut(&base_face_id) {
                base_face.add(triangle);
                for face_id in matches.iter().skip(1) {
                    if let Some(face) = self.list.remove(face_id) {
                        base_face.merge(face);
                    }
                }
            }
        }
    }

    fn match_face_ids(&self, triangle: &Vec<Edge>, plane: &Plane) -> Vec<usize> {
        let mut matched = Vec::new();
        for face in self.list.values() {
            if face.matches(triangle.clone(), plane) {
                matched.push(face.id);
            }
        }
        matched
    }
}
