use std::collections::{HashMap, HashSet};

use super::{Edge, Plane};

#[derive(Debug, Clone)]
pub struct Face {
    pub edges: HashMap<String, Edge>,
    pub open_edges: HashSet<String>,
    pub id: usize,
    pub plane: Plane,
}

impl Face {
    pub fn new(id: usize, plane: Plane) -> Self {
        Self {
            edges: HashMap::new(),
            open_edges: HashSet::new(),
            id,
            plane,
        }
    }

    pub fn add(&mut self, triangle: impl IntoIterator<Item = Edge>) {
        if self.edges.is_empty() {
            for edge in triangle {
                self.open_edges.insert(edge.hash.clone());
                self.edges.insert(edge.hash.clone(), edge);
            }
            return;
        }

        for edge in triangle {
            if self.open_edges.contains(&edge.hash) {
                self.open_edges.remove(&edge.hash);
            } else {
                self.open_edges.insert(edge.hash.clone());
            }
            self.edges.insert(edge.hash.clone(), edge);
        }
    }

    pub fn matches(&self, triangle: impl IntoIterator<Item = Edge>, plane: &Plane) -> bool {
        if plane.id != self.plane.id {
            return false;
        }

        for edge in triangle {
            if self.open_edges.contains(&edge.hash) {
                return true;
            }
        }
        false
    }

    pub fn open_edges(&self) -> Vec<Edge> {
        self.open_edges
            .iter()
            .filter_map(|edge_id| self.edges.get(edge_id).cloned())
            .collect()
    }

    pub fn merge(&mut self, face: Face) {
        for (edge_id, edge) in face.edges {
            self.edges.insert(edge_id, edge);
        }

        for edge_id in face.open_edges {
            if self.open_edges.contains(&edge_id) {
                self.open_edges.remove(&edge_id);
            } else {
                self.open_edges.insert(edge_id);
            }
        }
    }
}
