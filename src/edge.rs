use super::point::Point;
use std::hash::{Hash, Hasher};

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
        let (x1, y1) = (self.p1.x.to_bits(), self.p1.y.to_bits());
        let (x2, y2) = (self.p2.x.to_bits(), self.p2.y.to_bits());
        std::cmp::max((x1, y1), (x2, y2)).hash(state);
        std::cmp::min((x1, y1), (x2, y2)).hash(state);
    }
}
