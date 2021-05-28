use super::point::Point;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy)]
pub struct Edge {
    pub p1: Point,
    pub p2: Point,
}

impl Edge {
    pub fn new(p1: Point, p2: Point) -> Self {
        Self { p1, p2 }
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        (self.p1 == other.p1 && self.p2 == other.p2) || (self.p1 == other.p2 && self.p2 == other.p1)
    }
}

impl Eq for Edge {}

impl Hash for Edge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut points = [
            (self.p1.x.to_bits(), self.p1.y.to_bits()),
            (self.p2.x.to_bits(), self.p2.y.to_bits()),
        ];
        points.sort_unstable();
        points.hash(state);
    }
}
