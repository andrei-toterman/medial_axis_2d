use std::hash::{Hash, Hasher};

#[derive(Copy, Clone, Default)]
pub struct Point {
    pub id: usize,
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(id: usize, x: f64, y: f64) -> Self {
        Self { id, x, y }
    }

    pub fn dist(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }

    pub fn norm(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }
}

fn almost_equal(a: f64, b: f64) -> bool {
    (a - b).abs() <= f64::EPSILON
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        almost_equal(self.x, other.x) && almost_equal(self.y, other.y)
    }
}

impl Eq for Point {}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
