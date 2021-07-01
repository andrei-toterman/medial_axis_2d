use medial_axis_2d::{delaunay, medial_axis, point::Point, point_inside_shape};
use raylib::prelude::{
    get_random_value, rstr, Camera2D, Color, KeyboardKey, MouseButton, RaylibDraw, RaylibDrawGui,
    RaylibMode2DExt, Rectangle, Vector2,
};
use std::{
    io::{BufRead, BufReader},
    time::UNIX_EPOCH,
};

fn main() {
    let mut args = std::env::args().skip(1);
    let file_name = args.next().expect("no file given");
    let input = BufReader::new(std::fs::File::open(&file_name).unwrap());

    let mut points = Vec::new();

    for line in input.lines().map(Result::unwrap) {
        let mut tokens = line.split_whitespace().map(|token| token.parse().unwrap());
        let (x, y) = (tokens.next().unwrap(), tokens.next().unwrap());
        points.push(Point::new(x, y));
    }

    let start = std::time::Instant::now();
    let mut triangles = delaunay(&points);
    println!("{}", start.elapsed().as_micros());

    triangles.retain(|tri| point_inside_shape(&tri.centroid(), &points));
    let media_axis = medial_axis(&triangles);

    let mut outline_draw = points.iter().map(|&p| Vector2::from(p)).collect::<Vec<_>>();
    outline_draw.push(outline_draw[0]);

    let triangles_draw = triangles
        .iter()
        .map(|tri| {
            (
                Vector2::from(tri.p1),
                Vector2::from(tri.p2),
                Vector2::from(tri.p3),
                Color::color_from_hsv(get_random_value::<i32>(0, 360) as f32, 0.5, 0.8),
            )
        })
        .collect::<Vec<_>>();

    let medial_axis_draw = media_axis
        .iter()
        .map(|edge| (Vector2::from(edge.p1), Vector2::from(edge.p2)))
        .collect::<Vec<_>>();

    let (mut rl_handle, rl_thread) = raylib::init().size(1000, 1000).title("skeleton 2d").build();
    rl_handle.set_target_fps(60);

    let mut camera = Camera2D {
        offset: Vector2::zero(),
        target: Vector2::zero(),
        rotation: 0.0,
        zoom: 1.0,
    };
    let mut prev_mouse = rl_handle.get_mouse_position();

    let mut show_ui = true;

    let mut tri_iter = (0..triangles.len()).cycle();
    let mut tri_index = tri_iter.next().unwrap();

    let mut show_delaunay = false;
    let mut show_skeleton = false;
    let mut show_skeleton_balls = false;
    let mut show_outline = true;
    let mut show_vertices = true;
    let mut show_circles = false;

    while !rl_handle.window_should_close() {
        if rl_handle.is_key_pressed(KeyboardKey::KEY_U) {
            show_ui = !show_ui;
        }

        if rl_handle.is_key_pressed(KeyboardKey::KEY_S) {
            rl_handle.take_screenshot(
                &rl_thread,
                format!(
                    "{}.png",
                    std::time::SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs()
                )
                .as_str(),
            );
        }

        let curr_mouse = rl_handle.get_mouse_position();
        let mouse_delta = prev_mouse - curr_mouse;
        prev_mouse = curr_mouse;
        if rl_handle.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
            camera.target = rl_handle.get_screen_to_world2D(camera.offset + mouse_delta, camera);
        }
        camera.zoom = (camera.zoom + rl_handle.get_mouse_wheel_move() * 0.1).max(0.1);

        let mut draw_handle = rl_handle.begin_drawing(&rl_thread);
        draw_handle.clear_background(Color::WHITE);

        {
            let mut draw_handle = draw_handle.begin_mode2D(camera);

            if show_delaunay {
                for &(v1, v2, v3, color) in triangles_draw.iter() {
                    draw_handle.draw_triangle(v1, v2, v3, color.fade(0.3));
                    draw_handle.draw_line_ex(v1, v2, 1.0, color);
                    draw_handle.draw_line_ex(v2, v3, 1.0, color);
                    draw_handle.draw_line_ex(v3, v1, 1.0, color);
                }
            }

            if show_skeleton {
                for &(v1, v2) in medial_axis_draw.iter() {
                    draw_handle.draw_line_ex(v1, v2, 3.0, Color::PURPLE);
                    if show_skeleton_balls {
                        draw_handle.draw_circle_v(v1, 5.0, Color::PURPLE);
                        draw_handle.draw_circle_v(v2, 5.0, Color::PURPLE);
                    }
                }
            }

            if show_outline {
                for v in outline_draw.windows(2) {
                    draw_handle.draw_line_ex(v[0], v[1], 3.0, Color::BLACK);
                }
            }

            if show_vertices {
                for point in points.iter() {
                    draw_handle.draw_circle_v(
                        Vector2::from(*point),
                        5.0,
                        if !show_circles {
                            Color::BLACK
                        } else {
                            let tetra = &triangles[tri_index];
                            if tetra.has_point(point) {
                                Color::BLUE
                            } else if tetra.has_point_circumcircle(point) {
                                Color::RED
                            } else {
                                Color::GREEN
                            }
                        },
                    );
                }
            }

            if show_circles {
                let tri = &triangles[tri_index];
                draw_handle.draw_triangle(
                    Vector2::from(tri.p1),
                    Vector2::from(tri.p2),
                    Vector2::from(tri.p3),
                    Color::BLUE.fade(0.3),
                );
                draw_handle.draw_circle_v(
                    Vector2::from(tri.circumcenter),
                    tri.circumradius.sqrt() as f32,
                    Color::BLUE.fade(0.3),
                );
            }
        }

        if show_ui {
            let mut gui_y = (10..).step_by(35).map(|n| n as f32);

            show_outline = draw_handle.gui_check_box(
                Rectangle::new(10.0, gui_y.next().unwrap(), 30.0, 30.0),
                Some(rstr!("show outline")),
                show_outline,
            );
            show_vertices = draw_handle.gui_check_box(
                Rectangle::new(10.0, gui_y.next().unwrap(), 30.0, 30.0),
                Some(rstr!("show vertices")),
                show_vertices,
            );
            show_delaunay = draw_handle.gui_check_box(
                Rectangle::new(10.0, gui_y.next().unwrap(), 30.0, 30.0),
                Some(rstr!("show delaunay")),
                show_delaunay,
            );
            show_skeleton = draw_handle.gui_check_box(
                Rectangle::new(10.0, gui_y.next().unwrap(), 30.0, 30.0),
                Some(rstr!("show skeleton")),
                show_skeleton,
            );
            show_skeleton_balls = draw_handle.gui_check_box(
                Rectangle::new(10.0, gui_y.next().unwrap(), 30.0, 30.0),
                Some(rstr!("show skeleton balls")),
                show_skeleton_balls,
            );
            show_circles = draw_handle.gui_check_box(
                Rectangle::new(10.0, gui_y.next().unwrap(), 30.0, 30.0),
                Some(rstr!("show circles")),
                show_circles,
            );
            if draw_handle.gui_button(
                Rectangle::new(10.0, gui_y.next().unwrap(), 30.0, 30.0),
                Some(rstr!("{}", tri_index).as_c_str()),
            ) {
                tri_index = tri_iter.next().unwrap();
            }
        }
    }
}
