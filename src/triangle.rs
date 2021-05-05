use super::point::Point;

#[derive(Copy, Clone)]
pub struct Triangle {
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
    pub circumcenter: Point,
    pub circumradius: f64,
}

impl Triangle {
    pub fn new(p1: Point, p2: Point, p3: Point) -> Self {
        let p1n = p1.norm();
        let p2n = p2.norm();
        let p3n = p3.norm();
        let circum_x = (p1n * (p3.y - p2.y) + p2n * (p1.y - p3.y) + p3n * (p2.y - p1.y))
            / (p1.x * (p3.y - p2.y) + p2.x * (p1.y - p3.y) + p3.x * (p2.y - p1.y));
        let circum_y = (p1n * (p3.x - p2.x) + p2n * (p1.x - p3.x) + p3n * (p2.x - p1.x))
            / (p1.y * (p3.x - p2.x) + p2.y * (p1.x - p3.x) + p3.y * (p2.x - p1.x));

        let circumcenter = Point::new(0, circum_x / 2.0, circum_y / 2.0);
        let circumradius = p1.dist(&circumcenter);

        Self {
            p1,
            p2,
            p3,
            circumcenter,
            circumradius,
        }
    }

    pub fn has_point(&self, point: &Point) -> bool {
        (self.p1 == *point) || (self.p2 == *point) || (self.p3 == *point)
    }

    pub fn has_point_circumcircle(&self, point: &Point) -> bool {
        point.dist(&self.circumcenter) <= self.circumradius
    }
}
