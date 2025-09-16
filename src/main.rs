mod point;
mod chaikin;
mod graphics;

use minifb::{Window, WindowOptions, MouseButton, Key, KeyRepeat};
use crate::point::Point;
use crate::chaikin::chaikin_step;
use crate::graphics::{draw_circle, draw_line, WIDTH, HEIGHT};

fn main() {
    let mut window = Window::new(
        "Chaikin Curve Fully Connected",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap();

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut control_points: Vec<Point> = Vec::new();
    let mut curve_points: Vec<Point> = Vec::new();
    let mut animating = false;
    let mut step = 0;
    let max_steps = 7;

    while window.is_open() {
        buffer.fill(0);

        // Add points
        if window.get_mouse_down(MouseButton::Left) {
            if let Some((x, y)) = window.get_mouse_pos(minifb::MouseMode::Clamp) {
                let new_point = Point { x: x as f32, y: y as f32 };
                if control_points.last().map_or(true, |last| (last.x - new_point.x).abs() > 1.0 || (last.y - new_point.y).abs() > 1.0) {
                    control_points.push(new_point);
                    curve_points.clear();
                    animating = false;
                    step = 0;
                }
            }
        }

        // Start animation
        if window.is_key_pressed(Key::Enter, KeyRepeat::No) && control_points.len() > 1 {
            animating = true;
            step = 0;
            curve_points = control_points.clone();
        }

        // Step-by-step Chaikin animation
        if animating && step < max_steps {
            curve_points = chaikin_step(&curve_points); // compute next iteration
            step += 1;
        } else if control_points.len() == 2 {
            curve_points = control_points.clone(); // straight line for 2 points
        }

        // Draw fully connected curve
        for i in 0..curve_points.len().saturating_sub(1) {
            draw_line(&mut buffer, &curve_points[i], &curve_points[i + 1], 0x0000FF);
        }

        // Draw control points
        for p in &control_points {
            draw_circle(&mut buffer, p.x, p.y, 5.0, 0xFF0000);
        }

        // Quit
        if window.is_key_pressed(Key::Escape, KeyRepeat::No) {
            break;
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
