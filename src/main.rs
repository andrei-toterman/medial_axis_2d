use medial_axis_2d::{point::Point, triangle::Triangle, triangulate};
use std::io::{BufRead, BufWriter, Write};

fn main() {
    let mut args = std::env::args().skip(1);
    let input = args.next().unwrap_or_else(|| String::from("points.txt"));
    let output = args.next().unwrap_or_else(|| String::from("delaunay.txt"));

    let mut points = Vec::new();

    let input = std::io::BufReader::new(std::fs::File::open(&input).unwrap());
    for (id, line) in input.lines().enumerate() {
        let line = line.unwrap();
        let mut tokens = line.split_whitespace().map(|token| token.parse().unwrap());
        let (x, y) = (tokens.next().unwrap(), tokens.next().unwrap());
        points.push(Point::new(id, x, y));
    }

    let start = std::time::Instant::now();
    let triangles = triangulate(&points);
    println!("{}", start.elapsed().as_micros());

    let mut output = BufWriter::new(std::fs::File::create(output).unwrap());

    for Point { x, y, .. } in points.iter() {
        writeln!(output, "p {} {}", x, y).unwrap();
    }

    for Triangle { p1, p2, p3, .. } in triangles.iter() {
        writeln!(output, "t {} {} {}", p1.id, p2.id, p3.id).unwrap();
    }
}
