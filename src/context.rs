use std::{collections::HashMap, time};

use glium::{texture::SrgbTexture2d, Display, glutin::{ContextBuilder, ContextCurrentState, self}};
use rodio::{OutputStream, OutputStreamHandle, Sink};
use winit::{event_loop::EventLoop, window::WindowBuilder, event::MouseScrollDelta};

use crate::{mesh::Mesh, scene::{Scene, self}};

pub struct Context {
    pub stream: OutputStream,
    pub stream_handle: OutputStreamHandle,
}
impl Default for Context {
    fn default() -> Self {
        let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        Self {
            stream,
            stream_handle,
        }
    }
}
impl Context {
    pub fn new<const NUM: usize>() -> Self {
        Self::default()
    }
    pub unsafe fn start<T : ContextCurrentState>(self, scene_func: unsafe fn(&Display, &Context) -> Scene, wb : WindowBuilder, cb : ContextBuilder<'_, T>) {
        let events_loop = EventLoop::new();
        let display = Display::new(
            wb,
            cb,
            &events_loop,
        )
        .unwrap();

        let mut scene = scene_func(&display, &self);
        Scene::init(&mut scene as *mut Scene, &self);

        let mut should_exit = false;
        let refresh_rate = time::Duration::from_nanos(16_666_667);

        events_loop.run(move |event, _target, control_flow| {
            let now = time::Instant::now();

            if now >= scene.next_frame_instant {
                Scene::draw(&mut scene as *mut Scene, &display, &self);

                match Scene::update(&mut scene as *mut Scene, &self, now, refresh_rate) {
                    Some(next_scene) => match next_scene {
                        scene::NextScene::Another(new_scene) => {
                            scene = new_scene;
                            Scene::init(&mut scene as *mut Scene, &self);
                        }
                        scene::NextScene::Done => should_exit = true,
                    },
                    None => (),
                }
            }

            if should_exit {
                *control_flow = glutin::event_loop::ControlFlow::Exit;
            } else {
                match event {
                    glutin::event::Event::WindowEvent {
                        window_id: _,
                        event,
                    } => match event {
                        glutin::event::WindowEvent::CloseRequested => {
                            *control_flow = glutin::event_loop::ControlFlow::Exit;
                            return;
                        }
                        glutin::event::WindowEvent::Resized(size) => {
                            scene.proj = cgmath::perspective(
                                cgmath::Deg(90.0),
                                size.width as f32 / size.height as f32,
                                0.05,
                                100.0,
                            )
                            .into();
                        }
                        _ => (),
                    },
                    glutin::event::Event::RedrawRequested(_) => {
                        Scene::draw(&mut scene as *mut Scene, &display,&self);
                    }
                    glutin::event::Event::DeviceEvent {
                        device_id: _,
                        event,
                    } => match event {
                        glutin::event::DeviceEvent::Key(key) => {
                            scene.input.poll_keys(key);
                        }
                        glutin::event::DeviceEvent::MouseMotion { delta } => {
                            scene.input.poll_mouse(delta);
                        }
                        glutin::event::DeviceEvent::MouseWheel { delta } => match delta {
                            MouseScrollDelta::LineDelta(x, y) => scene.input.poll_scroll((x, y)),
                            _ => (),
                        },
                        _ => (),
                    },
                    _ => (),
                }

                control_flow.set_wait_until(scene.next_frame_instant);
            }
        });
    }
}
