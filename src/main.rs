#![windows_subsystem = "windows"]


use cgmath::{Matrix4, Deg};
use glium::{
    glutin::{self, event_loop::EventLoop, window::WindowBuilder, ContextBuilder, event::VirtualKeyCode},
    Display,
};
use std::time;

use instance::Instance;
const REFRESH_RATES: [time::Duration; 1] = [time::Duration::from_nanos(16_666_667)];

mod candy;
mod instance;
mod input;
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
    let mut instance = Instance::new(
        Display::new(WindowBuilder::new(), ContextBuilder::new().with_depth_buffer(24), &events_loop).unwrap(),
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
    
    
    

    let mut refresh_rate = REFRESH_RATES[0];

    events_loop.run(move |event, _target, control_flow| {
        
        let now = time::Instant::now();

        if now >= instance.next_frame_instant {
            instance.draw();
            instance.update(now, refresh_rate);
        }
        
        match event {
            glutin::event::Event::WindowEvent { window_id, event } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                glutin::event::WindowEvent::Resized(size) => {
                    instance.view = cgmath::perspective(cgmath::Deg(90.0),(size.width as f32/ size.height as f32),0.05,100.0);
                    
                }
                _ => (),
            },
            glutin::event::Event::RedrawRequested(_) => {
                instance.draw();
            }
            glutin::event::Event::DeviceEvent { device_id, event } => match event {
                glutin::event::DeviceEvent::Key(key) => {
                    instance.input.update(key);
                }
                glutin::event::DeviceEvent::MouseMotion { delta } => {
                    instance.camera = instance.camera * Matrix4::from_angle_y(Deg(delta.0 as f32 * -0.125)) * Matrix4::from_angle_x(Deg(delta.1 as f32 * -0.125));
                }
                _ => (),
            }
            _ => (),
        }
        
        control_flow.set_wait_until(instance.next_frame_instant);
        
    });
}
