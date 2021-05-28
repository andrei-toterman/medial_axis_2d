use medial_axis_2d::{
    delaunay, edge::Edge, medial_axis, point::Point, point_inside_shape, triangle::Triangle,
};
use raylib::prelude::{
    rstr, Camera2D, Color, MouseButton, RaylibDraw, RaylibDrawGui, RaylibMode2DExt, Rectangle,
    Vector2,
};
use std::io::{BufRead, BufReader};

fn main() {
    let (mut rl_handle, rl_thread) = raylib::init().size(1000, 1000).title("skeleton 2d").build();

    let mut args = std::env::args().skip(1);
    let input = args.next().unwrap_or_else(|| String::from("points_2d.txt"));

    let mut points = Vec::new();
    let input = BufReader::new(std::fs::File::open(&input).unwrap());
    for line in input.lines() {
        let line = line.unwrap();
        let mut tokens = line.split_whitespace().map(|token| token.parse().unwrap());
        let (x, y) = (tokens.next().unwrap(), tokens.next().unwrap());
        points.push(Point::new(x, y));
    }

    let start = std::time::Instant::now();
    let mut triangles = delaunay(&points);
    println!("delaunay took {} microseconds", start.elapsed().as_micros());

    triangles.retain(|tri| point_inside_shape(&tri.centroid(), &points));
    let media_axis = medial_axis(&triangles);

    let mut points_draw = points
        .iter()
        .map(|p| Vector2::new(p.x as f32, p.y as f32))
        .collect::<Vec<_>>();
    points_draw.push(points_draw[0]);

    let triangles_draw = triangles
        .iter()
        .map(|Triangle { p1, p2, p3, .. }| {
            (
                Vector2::new(p1.x as f32, p1.y as f32),
                Vector2::new(p2.x as f32, p2.y as f32),
                Vector2::new(p3.x as f32, p3.y as f32),
            )
        })
        .collect::<Vec<_>>();

    let medial_axis_draw = media_axis
        .iter()
        .map(|Edge { p1, p2 }| {
            (
                Vector2::new(p1.x as f32, p1.y as f32),
                Vector2::new(p2.x as f32, p2.y as f32),
            )
        })
        .collect::<Vec<_>>();

    let mut camera = Camera2D {
        offset: Vector2::zero(),
        target: Vector2::zero(),
        rotation: 0.0,
        zoom: 1.0,
    };

    let mut prev_mouse = rl_handle.get_mouse_position();
    let mut show_triangulation = false;

    while !rl_handle.window_should_close() {
        let curr_mouse = rl_handle.get_mouse_position();
        let mouse_delta = prev_mouse - curr_mouse;
        prev_mouse = curr_mouse;
        if rl_handle.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
            camera.target = rl_handle.get_screen_to_world2D(camera.offset + mouse_delta, camera);
        }
        camera.zoom = (camera.zoom + rl_handle.get_mouse_wheel_move() * 0.1).max(0.1);

        let mut draw_handle = rl_handle.begin_drawing(&rl_thread);
        draw_handle.clear_background(Color::RAYWHITE);

        {
            let mut draw_handle = draw_handle.begin_mode2D(camera);

            for &(v1, v2, v3) in triangles_draw.iter() {
                draw_handle.draw_triangle(v1, v2, v3, Color::RED.fade(0.3));
                if show_triangulation {
                    draw_handle.draw_triangle_lines(v1, v2, v3, Color::RED);
                }
            }
            draw_handle.draw_line_strip(&points_draw, Color::RED);

            for &v in points_draw.iter() {
                draw_handle.draw_circle_v(v, 2.0, Color::MAROON);
            }

            for &(v1, v2) in medial_axis_draw.iter() {
                draw_handle.draw_line_ex(v1, v2, 1.5, Color::BLACK);
                draw_handle.draw_circle_v(v1, 2.0, Color::LIGHTGRAY);
                draw_handle.draw_circle_v(v2, 2.0, Color::LIGHTGRAY);
            }
        }

        show_triangulation = draw_handle.gui_check_box(
            Rectangle::new(10.0, 10.0, 30.0, 30.0),
            Some(rstr!("show triangulation")),
            show_triangulation,
        );
    }
}
