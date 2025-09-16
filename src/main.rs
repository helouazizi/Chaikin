mod  graphics;
mod point;
mod chaikin;

use graphics::*;
use point::*;
use chaikin::*;

fn main() {

    let mut canvas = Canvas::new("Chaikin Curve - ESC to exit");

    let base_points = vec![
        Point { x: 100.0, y: 300.0 },
        Point { x: 200.0, y: 100.0 },
        Point { x: 400.0, y: 150.0 },
        Point { x: 600.0, y: 300.0 },
        Point { x: 700.0, y: 500.0 },
    ];

    // Apply Chaikin's algorithm
    // let smooth_points = chaikin(&base_points, 3);

    // Main loop
    while canvas. {
        canvas.clear();
        canvas.draw_points(&base_points, 0x00FF00);      
        // canvas.draw_points(&smooth_points, 0xFFFFFF);    
        canvas.update();
    }
}



