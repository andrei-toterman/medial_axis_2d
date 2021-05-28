pub mod edge;
pub mod point;
pub mod triangle;

use edge::Edge;
use point::Point;
use std::collections::{HashMap, HashSet};
use triangle::Triangle;

pub fn almost_equal(a: f64, b: f64) -> bool {
    (a - b).abs() <= f64::EPSILON
}

pub fn delaunay(points: &[Point]) -> Vec<Triangle> {
    let mut triangles = Vec::new();

    let Point {
        x: mut min_x,
        y: mut min_y,
        ..
    } = points[0];
    let (mut max_x, mut max_y) = (min_x, min_y);

    for &Point { x, y } in points.iter() {
        if x < min_x {
            min_x = x;
        }
        if y < min_y {
            min_y = y;
        }
        if x > max_x {
            max_x = x;
        }
        if y > max_y {
            max_y = y;
        }
    }

    let (dx, dy) = (max_x - min_x, max_y - min_y);
    let d_max = if dx > dy { dx } else { dy };
    let (mid_x, mid_y) = ((min_x + max_x) / 2.0, (min_y + max_y) / 2.0);

    let super_p1 = Point::new(mid_x - 20.0 * d_max, mid_y - d_max);
    let super_p2 = Point::new(mid_x, mid_y + 20.0 * d_max);
    let super_p3 = Point::new(mid_x + 20.0 * d_max, mid_y - d_max);

    triangles.push((Triangle::new(super_p1, super_p2, super_p3), false));

    let mut hole = HashSet::new();
    for point in points.iter() {
        for (tri, bad) in triangles.iter_mut() {
            if tri.has_point_circumcircle(point) {
                *bad = true;
                let edges = [
                    Edge::new(tri.p1, tri.p2),
                    Edge::new(tri.p2, tri.p3),
                    Edge::new(tri.p3, tri.p1),
                ];
                for &edge in edges.iter() {
                    if !hole.insert(edge) {
                        hole.remove(&edge);
                    }
                }
            }
        }

        triangles.retain(|&(_, bad)| !bad);

        for edge in hole.iter() {
            triangles.push((Triangle::new(edge.p1, edge.p2, *point), false));
        }
        hole.clear();
    }

    triangles
        .into_iter()
        .filter_map(|(tri, _)| {
            if [super_p1, super_p2, super_p3]
                .iter()
                .all(|p| !tri.has_point(p))
            {
                Some(tri)
            } else {
                None
            }
        })
        .collect()
}

pub fn edge_adjacency(triangles: &[Triangle]) -> HashMap<Edge, (Triangle, Option<Triangle>)> {
    let mut edges = HashMap::new();

    for &tri in triangles {
        edges
            .entry(Edge::new(tri.p1, tri.p2))
            .and_modify(|(_, t)| *t = Some(tri))
            .or_insert((tri, None));
        edges
            .entry(Edge::new(tri.p2, tri.p3))
            .and_modify(|(_, t)| *t = Some(tri))
            .or_insert((tri, None));
        edges
            .entry(Edge::new(tri.p3, tri.p1))
            .and_modify(|(_, t)| *t = Some(tri))
            .or_insert((tri, None));
    }

    edges
}

pub fn medial_axis(triangles: &[Triangle]) -> Vec<Edge> {
    edge_adjacency(triangles)
        .into_iter()
        .filter_map(|(_, (t1, t2))| t2.map(|t2| Edge::new(t1.circumcenter, t2.circumcenter)))
        .collect()
}

pub fn point_inside_shape(point: &Point, shape: &[Point]) -> bool {
    let mut inside = false;

    let mut j = shape.len() - 1;
    for i in 0..shape.len() {
        if almost_equal(point.x, shape[i].x) && almost_equal(point.y, shape[i].y) {
            return true;
        }
        if (shape[i].y > point.y) != (shape[j].y > point.y) {
            let slope = (point.x - shape[i].x) * (shape[j].y - shape[i].y)
                - (shape[j].x - shape[i].x) * (point.y - shape[i].y);
            if almost_equal(slope, 0.0) {
                return true;
            }
            if (slope < 0.0) != (shape[j].y < shape[i].y) {
                inside = !inside;
            }
        }
        j = i;
    }

    inside
}
