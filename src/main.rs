#![windows_subsystem = "windows"]

use cgmath::Matrix4;
use glium::{
    glutin::{self, event_loop::EventLoop, window::WindowBuilder, ContextBuilder},
    Display,
};
use std::time;

use instance::Instance;
const REFRESH_RATES: [time::Duration; 1] = [time::Duration::from_nanos(16_666_667)];

mod candy;
mod instance;
use crate::candy::Candy;

fn main() {
    let events_loop = EventLoop::new();

    let mut instance = Instance::new(
        Display::new(WindowBuilder::new(), ContextBuilder::new(), &events_loop).unwrap(),
        vec![
            Candy {
                pos: [1.0, 0.0],
                t: candy::Type::Normal(0),
            },
        ],
    );

    let mut view: Matrix4<f32> = cgmath::perspective(cgmath::Deg(90.0), 16.0 / 9.0, 0.1, 10.0);
    view = view
        * Matrix4::from_translation(cgmath::Vector3 {
            x: 0.0,
            y: 0.0,
            z: -3.0,
        });

    let mut refresh_rate = REFRESH_RATES[0];

    events_loop.run(move |event, _target, control_flow| {
        let now = time::Instant::now();

        if now >= instance.next_frame_instant {
            instance.draw();
            instance.update(now, refresh_rate);
        }

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                glutin::event::WindowEvent::Resized(_) => {
                    instance.draw();
                }
                _ => (),
            },
            _ => (),
        }
        control_flow.set_wait_until(instance.next_frame_instant);
    });
}
