use crate::point::Point;

pub fn chaikin_step(points: &[Point]) -> Vec<Point> {
    if points.len() < 2 {
        return points.to_vec();
    }

    let mut new_points = Vec::new();
    for i in 0..points.len() - 1 {
        let p0 = &points[i];
        let p1 = &points[i + 1];

        // Q = 3/4*p0 + 1/4*p1
        new_points.push(Point {
            x: 0.75 * p0.x + 0.25 * p1.x,
            y: 0.75 * p0.y + 0.25 * p1.y,
        });

        // R = 1/4*p0 + 3/4*p1
        new_points.push(Point {
            x: 0.25 * p0.x + 0.75 * p1.x,
            y: 0.25 * p0.y + 0.75 * p1.y,
        });
    }
    new_points
}
