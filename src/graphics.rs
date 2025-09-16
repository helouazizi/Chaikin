

use minifb::{Key, Window, WindowOptions};



const WIDTH: usize = 800;
const HEIGHT: usize = 600;



pub struct Canvas {
    window :Window,
    content : Vec<u32>
}


impl Canvas {
    pub fn new(name:&str) -> Self{
        let window = Window::new(name,WIDTH , HEIGHT,WindowOptions::default()).expect("faild to create window");
        let content = vec![0;WIDTH*HEIGHT];
        Self{content : content, window : window}
    }
}