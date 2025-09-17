use macroquad::prelude::*;

#[macroquad::main("Chaikin Curve")]
async fn main() {
    let mut points = Vec::new();
    let mut steps = Vec::new();
    let mut current_step = 0;
    let mut timer = 0.0;
    let step_duration = 0.5;

    loop {
        clear_background(BLACK);
        timer += get_frame_time();

        // Handle input
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_key_pressed(KeyCode::C) {
            points.clear();
            steps.clear();
            current_step = 0;
        }

        // Add points only if not animating
        if steps.is_empty() && is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();
            points.push(vec2(x, y));
        }

        // Start animation
        if is_key_pressed(KeyCode::Enter) && points.len() >= 2 && steps.is_empty() {
            steps = generate_steps(&points, 7);
            current_step = 0;
            timer = 0.0;
        }

        // Advance animation step
        if !steps.is_empty() && timer > step_duration {
            current_step = (current_step + 1) % steps.len();
            timer = 0.0;
        }

        // Draw user control points as hollow circles
        for p in &points {
            draw_circle_lines(p.x, p.y, 3.0, 1.0, WHITE);
        }

        // Draw Chaikin curve animation lines if animating
        if !steps.is_empty() {
            draw_lines(&steps[current_step], GREEN);
        }

        next_frame().await;
    }
}

fn draw_lines(points: &[Vec2], color: Color) {
    for i in 0..points.len() - 1 {
        draw_line(points[i].x, points[i].y, points[i + 1].x, points[i + 1].y, 3.0, color);
    }
}

fn chaikin_step(points: &[Vec2]) -> Vec<Vec2> {
    if points.len() < 2 {
        return points.to_vec();
    }

    let mut result = Vec::with_capacity(points.len() * 2);
    result.push(points[0]);

    for i in 0..points.len() - 1 {
        let p = points[i];
        let q = points[i + 1];
        result.push(p + (q - p) * 0.25);
        result.push(p + (q - p) * 0.75);
    }

    result.push(*points.last().unwrap());
    result
}

fn generate_steps(start: &[Vec2], count: usize) -> Vec<Vec<Vec2>> {
    let mut steps = vec![start.to_vec()];
    let mut current = start.to_vec();

    for _ in 0..count {
        current = chaikin_step(&current);
        steps.push(current.clone());
    }

    steps
}
