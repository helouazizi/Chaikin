use crate::point::Point;

pub const WIDTH: usize = 800;
pub const HEIGHT: usize = 600;

pub fn draw_circle(buffer: &mut Vec<u32>, cx: f32, cy: f32, radius: f32, color: u32) {
    let radius_sq = radius * radius;
    let x0 = cx as isize;
    let y0 = cy as isize;
    for y in (cy - radius) as isize..=(cy + radius) as isize {
        for x in (cx - radius) as isize..=(cx + radius) as isize {
            let dx = x - x0;
            let dy = y - y0;
            if dx * dx + dy * dy <= radius_sq as isize {
                if x >= 0 && x < WIDTH as isize && y >= 0 && y < HEIGHT as isize {
                    buffer[y as usize * WIDTH + x as usize] = color;
                }
            }
        }
    }
}

pub fn draw_line(buffer: &mut Vec<u32>, p1: &Point, p2: &Point, color: u32) {
    let mut x0 = p1.x as isize;
    let mut y0 = p1.y as isize;
    let x1 = p2.x as isize;
    let y1 = p2.y as isize;

    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        if x0 >= 0 && x0 < WIDTH as isize && y0 >= 0 && y0 < HEIGHT as isize {
            buffer[y0 as usize * WIDTH + x0 as usize] = color;
        }
        if x0 == x1 && y0 == y1 { break; }
        let e2 = 2 * err;
        if e2 >= dy { err += dy; x0 += sx; }
        if e2 <= dx { err += dx; y0 += sy; }
    }
}
