use std::time;

use candy_crush_level::CandyCrushLevel;
//#![windows_subsystem = "windows"]
use glium::{
    glutin::{event_loop::EventLoop, window::WindowBuilder, ContextBuilder},
    Display,
};
use instance::Instance;
use scene::Scene;
use winit::{event_loop::EventLoopBuilder, platform::run_return::EventLoopExtRunReturn};

mod candy;
mod candy_crush_level;
mod input;
mod instance;
mod scene;
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
    let mut events_loop = EventLoop::new();
    let display = Display::new(
        WindowBuilder::new(),
        ContextBuilder::new().with_depth_buffer(24),
        &events_loop,
    )
    .unwrap();

    let mut scene = Scene::new(
        CandyCrushLevel::new(
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
            &display,
        ),
        CandyCrushLevel::update,
        CandyCrushLevel::draw,
        CandyCrushLevel::get_next_scene,
    );
    let refresh_rate = Instance::REFRESH_RATES[1];
    events_loop.run(move |event, _target, control_flow| {
        let now = time::Instant::now();

        if now >= scene.data.next_frame_instant {
            (scene.draw)(&scene.data, &display);
            (scene.update)(&mut scene.data, &display, now, refresh_rate);
        }
    });
    /*
    self.events_loop.run_return(|event, _target, control_flow| {

        let now = time::Instant::now();

        if now >= self.next_frame_instant {
            self.draw();
            self.update(now, refresh_rate);
        }

        match event {
            glutin::event::Event::WindowEvent { window_id: _, event } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                glutin::event::WindowEvent::Resized(size) => {
                    self.view = cgmath::perspective(cgmath::Deg(90.0),size.width as f32/ size.height as f32,0.05,100.0);

                }
                _ => (),
            },
            glutin::event::Event::RedrawRequested(_) => {
                self.draw();
            }
            glutin::event::Event::DeviceEvent { device_id: _, event } => match event {
                glutin::event::DeviceEvent::Key(key) => {
                    self.input.poll_keys(key);
                }
                glutin::event::DeviceEvent::MouseMotion { delta } => {
                    self.input.poll_mouse(delta);
                }
                glutin::event::DeviceEvent::MouseWheel { delta } => {

                    match delta {
                        MouseScrollDelta::LineDelta(x, y) => self.input.poll_scroll((x,y)),
                        _ => (),
                    }
                }
                _ => (),
            }
            _ => (),
        }

        control_flow.set_wait_until(self.next_frame_instant);

    });
    */
}
