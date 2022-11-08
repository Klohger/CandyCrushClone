//#![windows_subsystem = "windows"]
use glium::{
    glutin::{event_loop::EventLoop, window::WindowBuilder, ContextBuilder},
    Display,
};
use instance::Instance;


mod candy;
mod instance;
mod input;
mod stage;
mod tile;



use crate::candy::Candy;
/*
fn LoadImage(filePath : String) {
    let events_loop = EventLoop::new();
    let image = image::load(std::io::Cursor::new(&include_bytes!(filePath)), image::ImageFormat::Png).unwrap().to_rgba8();
    let image_dimensions = image.dimensions();

    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
}
*/

fn main() {

    let events_loop = EventLoop::new();
    let display = Display::new(WindowBuilder::new(), ContextBuilder::new().with_depth_buffer(24), &events_loop).unwrap();
    
    let mut instance = Instance::new(
        &display,
        &events_loop,
        vec![
            Candy {
                pos: [0.0, 0.0],
                t: candy::Type::Normal(0),
            },
            Candy {
                pos: [1.0, 0.0],
                t: candy::Type::Normal(1),
            },
            Candy {
                pos: [0.0, 1.0],
                t: candy::Type::Normal(2),
            },
        ],
    );
    
    loop {
        match instance.Run(Instance::REFRESH_RATES[1]) {
            Some(new_instance) => instance = new_instance,
            None => break,
        }   
    }
}
