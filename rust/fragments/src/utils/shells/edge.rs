use super::Point;

#[derive(Debug, Clone, PartialEq)]
pub struct Edge {
    pub p1: Point,
    pub p2: Point,
    pub hash: String,
}

impl Edge {
    pub fn new(p1: Point, p2: Point) -> Self {
        let (first, second) = order_points(&p1, &p2);
        let hash = format!("{}_{}", first.hash, second.hash);
        Self { p1, p2, hash }
    }
}

fn order_points<'a>(a: &'a Point, b: &'a Point) -> (&'a Point, &'a Point) {
    if a.x < b.x {
        return (a, b);
    }
    if a.x > b.x {
        return (b, a);
    }

    if a.y < b.y {
        return (a, b);
    }
    if a.y > b.y {
        return (b, a);
    }

    if a.z <= b.z {
        (a, b)
    } else {
        (b, a)
    }
}
