mod point;
mod chaikin;
mod graphics;

use minifb::{Window, WindowOptions, MouseButton, Key, KeyRepeat};
use crate::point::Point;
use crate::chaikin::chaikin_step;
use crate::graphics::{draw_circle, draw_line, WIDTH, HEIGHT};
use std::time::{Duration, Instant};

fn main() {
    let mut window = Window::new(
        "Chaikin Curve Fully Connected",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap();

    window.limit_update_rate(Some(Duration::from_millis(16))); // ~60 FPS

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut control_points: Vec<Point> = Vec::new();
    let mut curve_points: Vec<Point> = Vec::new();
    
    let mut animating = false;
    let mut current_step_display = 0;
    let max_steps = 7;

    let mut last_step_time = Instant::now();
    let step_duration = Duration::from_millis(400); // milliseconds per step
    let mut all_curve_steps: Vec<Vec<Point>> = Vec::new();

    while window.is_open() {
        buffer.fill(0);

        // Add control points
        if window.get_mouse_down(MouseButton::Left) {
            if let Some((x, y)) = window.get_mouse_pos(minifb::MouseMode::Clamp) {
                let new_point = Point { x: x as f32, y: y as f32 };
                if control_points.last().map_or(true, |last| {
                    let dx = last.x - new_point.x;
                    let dy = last.y - new_point.y;
                    (dx*dx + dy*dy).sqrt() > 10.0
                }) {
                    control_points.push(new_point);
                    animating = false;
                    all_curve_steps.clear();
                    current_step_display = 0;
                }
            }
        }

        // Start animation
        if window.is_key_pressed(Key::Enter, KeyRepeat::No) && control_points.len() >= 3 {
            animating = true;
            current_step_display = 0;
            last_step_time = Instant::now();
            curve_points = control_points.clone();
            all_curve_steps.clear();
            all_curve_steps.push(curve_points.clone());
            
            // Precompute all steps
            for _ in 0..max_steps {
                curve_points = chaikin_step(&curve_points);
                all_curve_steps.push(curve_points.clone());
            }
        }

        // Clear points
        if window.is_key_pressed(Key::Space, KeyRepeat::No) {
            control_points.clear();
            all_curve_steps.clear();
            animating = false;
            current_step_display = 0;
        }

        // Animation timing
        if animating && current_step_display < all_curve_steps.len() - 1 {
            if last_step_time.elapsed() >= step_duration {
                current_step_display += 1;
                last_step_time = Instant::now();
                if current_step_display >= all_curve_steps.len() - 1 {
                    animating = false; // finished
                }
            }
        }

        // Determine points to display
        let display_points = if !all_curve_steps.is_empty() {
            &all_curve_steps[current_step_display]
        } else {
            &control_points
        };

        // Draw fully connected curve
        if display_points.len() >= 2 {
            for i in 0..display_points.len()-1 {
                draw_line(&mut buffer, &display_points[i], &display_points[i+1], 0x00AAFF);
            }
        }

        // Draw control points
        for (i, p) in control_points.iter().enumerate() {
            let color = if i == 0 { 0x00FF00 } else { 0xFF0000 };
            draw_circle(&mut buffer, p.x, p.y, 4.0, color);
        }

        // Draw points on the curve (optional, smaller)
        for p in display_points {
            draw_circle(&mut buffer, p.x, p.y, 2.0, 0xFFFFFF);
        }

        // Quit
        if window.is_key_pressed(Key::Escape, KeyRepeat::No) {
            break;
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
