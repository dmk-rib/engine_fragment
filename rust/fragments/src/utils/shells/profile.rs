use std::cmp::Ordering;

use super::{Edge, Plane, Point, Vector2};

#[derive(Debug, Clone)]
pub struct Profile {
    pub closed: bool,
    pub open_start_point: Option<String>,
    pub open_end_point: Option<String>,
    pub plane: Plane,
    pub ordered_points: Vec<Point>,
}

impl Profile {
    pub fn new(plane: Plane) -> Self {
        Self {
            closed: false,
            open_start_point: None,
            open_end_point: None,
            plane,
            ordered_points: Vec::new(),
        }
    }

    pub fn edges(&self, reverse: bool) -> Vec<Edge> {
        let mut edges = Vec::new();
        if reverse {
            for i in (1..self.ordered_points.len()).rev() {
                edges.push(Edge::new(
                    self.ordered_points[i].clone(),
                    self.ordered_points[i - 1].clone(),
                ));
            }
        } else {
            for i in 0..self.ordered_points.len().saturating_sub(1) {
                edges.push(Edge::new(
                    self.ordered_points[i].clone(),
                    self.ordered_points[i + 1].clone(),
                ));
            }
        }
        edges
    }

    pub fn indices(&self) -> Vec<usize> {
        self.ordered_points.iter().map(|point| point.id).collect()
    }

    pub fn add(&mut self, edge: Edge) {
        if self.ordered_points.is_empty() {
            self.open_start_point = Some(edge.p1.hash.clone());
            self.open_end_point = Some(edge.p2.hash.clone());
            self.ordered_points.push(edge.p1);
            self.ordered_points.push(edge.p2);
            return;
        }

        let matches = self.match_edge(&edge);
        if matches == 0 {
            panic!("Fragments: Edge doesn't match with any open point");
        }
        if matches > 2 {
            panic!("Fragments: Edge matches with more than 2 open points");
        }
        if matches == 2 {
            self.closed = true;
            self.open_end_point = None;
            self.open_start_point = None;
            return;
        }

        let start = self.open_start_point.clone();
        let end = self.open_end_point.clone();

        if start.as_deref() == Some(&edge.p1.hash) {
            self.ordered_points.insert(0, edge.p2.clone());
            self.open_start_point = Some(edge.p2.hash.clone());
        } else if end.as_deref() == Some(&edge.p1.hash) {
            self.ordered_points.push(edge.p2.clone());
            self.open_end_point = Some(edge.p2.hash.clone());
        } else if start.as_deref() == Some(&edge.p2.hash) {
            self.ordered_points.insert(0, edge.p1.clone());
            self.open_start_point = Some(edge.p1.hash.clone());
        } else if end.as_deref() == Some(&edge.p2.hash) {
            self.ordered_points.push(edge.p1.clone());
            self.open_end_point = Some(edge.p1.hash.clone());
        }
    }

    pub fn match_edge(&self, edge: &Edge) -> usize {
        if self.closed {
            return 0;
        }
        let mut matches = 0;
        if self.open_start_point.as_deref() == Some(&edge.p1.hash) {
            matches += 1;
        }
        if self.open_start_point.as_deref() == Some(&edge.p2.hash) {
            matches += 1;
        }
        if self.open_end_point.as_deref() == Some(&edge.p1.hash) {
            matches += 1;
        }
        if self.open_end_point.as_deref() == Some(&edge.p2.hash) {
            matches += 1;
        }
        matches
    }

    pub fn merge(&mut self, new_profile: Profile) {
        if new_profile.closed || self.closed {
            panic!("Fragments: Cannot merge closed profiles");
        }

        if new_profile.open_start_point == self.open_end_point
            && new_profile.open_end_point == self.open_start_point
        {
            panic!("Fragments: Cannot merge profiles that close each other");
        }

        if new_profile.open_end_point == self.open_end_point
            && new_profile.open_start_point == self.open_start_point
        {
            panic!("Fragments: Cannot merge profiles that close each other");
        }

        let mut reverse = false;
        if new_profile.open_end_point == self.open_start_point
            || new_profile.open_end_point == self.open_end_point
        {
            reverse = true;
        }

        let new_edges = new_profile.edges(reverse);
        for edge in new_edges {
            self.add(edge);
        }
    }

    pub fn area(&self) -> f32 {
        if self.ordered_points.is_empty() {
            return 0.0;
        }

        let abs_x = self.plane.normal.x.abs();
        let abs_y = self.plane.normal.y.abs();
        let abs_z = self.plane.normal.z.abs();

        let (dim1, dim2) = match (abs_x.partial_cmp(&abs_y), abs_x.partial_cmp(&abs_z)) {
            (
                Some(Ordering::Greater | Ordering::Equal),
                Some(Ordering::Greater | Ordering::Equal),
            ) => (1, 2),
            _ => match (abs_y.partial_cmp(&abs_x), abs_y.partial_cmp(&abs_z)) {
                (
                    Some(Ordering::Greater | Ordering::Equal),
                    Some(Ordering::Greater | Ordering::Equal),
                ) => (0, 2),
                _ => (0, 1),
            },
        };

        let mut projected = Vec::with_capacity(self.ordered_points.len());
        for point in &self.ordered_points {
            let coords = [point.x, point.y, point.z];
            projected.push(Vector2::new(coords[dim1], coords[dim2]));
        }

        let mut total = 0.0;
        let len = projected.len();
        for i in 0..len {
            let next = if i == len - 1 { 0 } else { i + 1 };
            total += projected[i].x * projected[next].y * 0.5;
            total -= projected[next].x * projected[i].y * 0.5;
        }
        total.abs()
    }
}
