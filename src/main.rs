use medial_axis_2d::triangle::Triangle;
use medial_axis_2d::{edge::Edge, medial_axis, point::Point, point_inside_shape, triangulate};
use std::io::{BufRead, BufWriter, Write};

fn main() {
    let mut args = std::env::args().skip(1);
    let input = args.next().unwrap_or_else(|| String::from("points.txt"));

    let mut points = Vec::new();

    let input = std::io::BufReader::new(std::fs::File::open(&input).unwrap());
    for line in input.lines() {
        let line = line.unwrap();
        let mut tokens = line.split_whitespace().map(|token| token.parse().unwrap());
        let (x, y) = (tokens.next().unwrap(), tokens.next().unwrap());
        points.push(Point::new(x, y));
    }

    let start = std::time::Instant::now();
    let mut triangles = triangulate(&points);
    println!("{}", start.elapsed().as_micros());

    triangles.retain(|tri| point_inside_shape(&tri.centroid(), &points));
    let media_axis = medial_axis(&triangles);

    let mut output = BufWriter::new(std::fs::File::create("diagram_2d.txt").unwrap());

    for Point { x, y } in points {
        writeln!(output, "p {} {}", x, y).unwrap();
    }

    for Edge { p1, p2 } in media_axis {
        writeln!(output, "E {} {} {} {}", p1.x, p1.y, p2.x, p2.y).unwrap();
    }

    for Triangle { p1, p2, p3, .. } in triangles {
        writeln!(
            output,
            "T {} {} {} {} {} {}",
            p1.x, p1.y, p2.x, p2.y, p3.x, p3.y
        )
        .unwrap();
    }
}
