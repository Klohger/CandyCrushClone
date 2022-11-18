#![windows_subsystem = "windows"]

use std::{any::Any, ptr, time::{self, Duration}};

//use asset::{TextureAsset, Asset};
//use ball::Ball;
//use ball::Ball;
use cgmath::{vec3, Deg, Matrix4, SquareMatrix, Vector3, Zero};
use component::{DynamicUniforms, MeshRenderer};
use context::Context;
use glium::{
    glutin::{self, event_loop::EventLoop, window::WindowBuilder, ContextBuilder},
    uniforms::{AsUniformValue, UniformValue},
    Blend, BlendingFunction, Display,
};

use object::Object;
use rodio::Source;
use scene::Scene;
use winit::event::MouseScrollDelta;

#[macro_export]
macro_rules! collection {
    // map-like
    ($($k:expr => $v:expr),* $(,)?) => {{
        core::convert::From::from([$(($k, $v),)*])
    }};
    // set-like
    ($($v:expr),* $(,)?) => {{
        core::convert::From::from([$($v,)*])
    }};
}

//mod candy;
//mod test_scene;
//mod stage;
//mod tile;

mod asset;
mod component;
mod context;
mod input;
mod mesh;
mod object;
mod scene;
//mod ball;
//mod camera_controller;

//mod serialization;
/*
fn LoadImage(filePath : String) {
    let events_loop = EventLoop::new();
    let image = image::load(std::io::Cursor::new(&include_bytes!(filePath)), image::ImageFormat::Png)?.to_rgba8();
    let image_dimensions = image.dimensions();

    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
}
*/

const REFRESH_RATES: [time::Duration; 2] = [
    time::Duration::from_nanos(16_666_667),
    time::Duration::from_nanos(15_625_000),
];

unsafe fn title(context: &Context) -> Scene {
    let mut scene = Scene::new(
        vec![
            Object {
                components: vec![
                    Box::new(component::Transform {
                        model: (Matrix4::from_angle_x(Deg(22.5))
                            * Matrix4::from_translation(Vector3::new(0.0, 0.0, -3.0)))
                        .into(),
                        ..Default::default()
                    }),
                    Box::new(component::MeshRenderer {
                        mesh: &context.meshes["title"],
                        prog: &context.shader_programs["title"],

                        uniforms: DynamicUniforms(
                            collection! {"offset" => 0.0_f32.as_uniform_value()},
                        ),
                        draw_parameters: glium::DrawParameters {
                            depth: glium::Depth {
                                test: glium::draw_parameters::DepthTest::IfLess,
                                write: true,
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        transform: ptr::null_mut(),
                    }),
                    Box::new(component::Title {
                        mesh_renderer: ptr::null_mut(),
                    }),
                ],
            },
            Object {
                components: vec![
                    Box::new(component::Transform {
                        model: Into::<[[f32; 4]; 4]>::into(
                            Matrix4::from_angle_x(Deg(22.5))
                                * Matrix4::from_translation(Vector3::new(1.25, 0.5, -2.9)),
                        ),
                        ..Default::default()
                    }),
                    Box::new(component::MeshRenderer {
                        mesh: &context.meshes["splash"],
                        prog: &context.shader_programs["splash"],

                        uniforms: DynamicUniforms(collection! {
                            "scale" => 1.0_f32.as_uniform_value(),
                            "model2" => UniformValue::Mat4(Into::<[[f32;4];4]>::into(Matrix4::from_angle_x(Deg(22.5)) * Matrix4::from_translation(Vector3::new(1.25, 0.5, -2.9)) * Matrix4::from_scale(0.5))),
                        }),
                        draw_parameters: glium::DrawParameters {
                            depth: glium::Depth {
                                test: glium::draw_parameters::DepthTest::IfLess,
                                write: true,
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        transform: ptr::null_mut(),
                    }),
                    Box::new(component::Splash {
                        mesh_renderer: ptr::null_mut(),
                    }),
                ],
            },
            Object {
                components: vec![
                    Box::new(component::Transform {
                        model: Matrix4::identity().into(),
                        ..Default::default()
                    }),
                    Box::new(component::NoclipController {
                        should_move: true,
                        transform: ptr::null_mut(),
                        view_x: 0.0,
                    }),
                    Box::new(component::Camera {
                        transform: ptr::null_mut(),
                    }),
                ],
            },
        ],
        Matrix4::identity().into(),
        &*context.display,
    );
    scene.objects.push(Object {
        components: vec![
            Box::new(component::Transform {
                model: Matrix4::from_translation(vec3(1.0, -1.0, -4.0)).into(),
                ..Default::default()
            }),
            
            Box::new(component::MeshRenderer {
                mesh: &context.meshes["exit"],
                prog: &context.shader_programs["door"],
                uniforms: DynamicUniforms(collection! {
                }),
                draw_parameters: glium::DrawParameters {
                    depth: glium::Depth {
                        test: glium::draw_parameters::DepthTest::IfLess,
                        write: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                transform: ptr::null_mut(),
            }),
            Box::new(component::Exit{ scene: None, transform: Object::get_component(&scene.objects[2] as *const Object, "Transform").unwrap(), min: Vector3 { x: 0.5, y: -2.0, z: -4.5 }, max: Vector3 { x: 1.5, y: 2.0, z: -3.5 } })
        ],
    });
    let swag = context as *const Context;
    scene.objects.push(Object {
        components: vec![
            Box::new(component::Transform {
                model: Matrix4::from_translation(vec3(-1.0, -1.0, -4.0)).into(),
                ..Default::default()
            }),
            
            Box::new(component::MeshRenderer {
                mesh: &context.meshes["start"],
                prog: &context.shader_programs["door"],
                uniforms: DynamicUniforms(collection! {
                }),
                draw_parameters: glium::DrawParameters {
                    depth: glium::Depth {
                        test: glium::draw_parameters::DepthTest::IfLess,
                        write: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                transform: ptr::null_mut(),
            }),
            Box::new(component::MeshRenderer {
                mesh: &context.meshes["plane"],
                prog: &context.shader_programs["trolos"],
                uniforms: DynamicUniforms(collection! {
                    "opacity" => UniformValue::Float(0.0),
                    "video" => UniformValue::SrgbTexture2d(&(*swag).textures["pain"], None),
                    "video_opacity" => UniformValue::Float(0.0),
                }),
                draw_parameters: glium::DrawParameters {
                    depth: glium::Depth {
                        test: glium::draw_parameters::DepthTest::IfLess,
                        write: true,
                        ..Default::default()
                    },
                    blend: Blend {
                        color: BlendingFunction::Addition {
                            source: glium::LinearBlendingFactor::SourceAlpha,
                            destination: glium::LinearBlendingFactor::OneMinusSourceAlpha,
                        },
                        alpha: BlendingFunction::Addition {
                            source: glium::LinearBlendingFactor::SourceAlpha,
                            destination: glium::LinearBlendingFactor::OneMinusSourceAlpha,
                        },
                        constant_value: (0.0, 0.0, 0.0, 0.0),
                    },
                    ..Default::default()
                },
                transform: ptr::null_mut(),
            }),
            
        ],
    });
    
    
    let bruh = Box::new(component::TheWackyEntrance {
        timer: Duration::ZERO,
        player: Object::get_component(&scene.objects[2] as *const Object, "Transform").unwrap(),
        mesh_renderer: &*(scene.objects[4].components[2]) as *const dyn component::Component
            as *mut dyn component::Component as *mut MeshRenderer,
        min: vec3(-1.5, -2.0, -4.5),
        max: vec3(-0.5, 2.0, -3.5),
        state: component::WackyState::Waiting,
        currentFrame: 0,
    });
    scene.objects[4].components.push(bruh);
    
    return scene;
}
use glium::texture::SrgbTexture2d;
fn main() {
    unsafe {
        let events_loop = EventLoop::new();
        let display = Display::new(
            WindowBuilder::new().with_maximized(true),
            ContextBuilder::new()
                .with_depth_buffer(24)
                .with_double_buffer(Some(true))
                .with_multisampling(1),
            &events_loop,
        )
        .unwrap();
        let context = Context::new(
            &display,
            collection! {
                //"moon-monkey" => todo!(),
                "pain" => SrgbTexture2d::empty(&display, 256,144).unwrap(),
            },
            collection! {
                "monkey" => asset::load_model("data/meshes/monkey.obj", &display),
                "title" => asset::load_model("data/meshes/title.obj", &display),
                "splash" => asset::load_model("data/meshes/splash.obj", &display),
                "start" => asset::load_model("data/meshes/start.obj", &display),
                "exit" => asset::load_model("data/meshes/exit.obj", &display),
                "plane" => asset::load_model("data/meshes/plane.obj", &display),
            },
            collection!(
                "monkey" => asset::load_program("data/shaders/monkey", &display),
                "title" => asset::load_program("data/shaders/title", &display),
                "splash" => asset::load_program("data/shaders/splash", &display),
                "door" => asset::load_program("data/shaders/start", &display),
                "trolos" => asset::load_program("data/shaders/trolos", &display),
            ),
            ["music", "sfx0", "trolos"],
        );
        context.sinks["music"].append(asset::load_audio("data/media/title.ogg").repeat_infinite());
        context.sinks["music"].play();

        let mut scene = title(&context);
        Scene::init(&mut scene as *mut Scene, &context);

        let mut should_exit = false;
        let refresh_rate = REFRESH_RATES[1];
        events_loop.run(move |event, _target, control_flow| {
            let now = time::Instant::now();

            if now >= scene.next_frame_instant {
                Scene::draw(&mut scene as *mut Scene, &context);

                match Scene::update(&mut scene as *mut Scene, &context, now, refresh_rate) {
                    Some(next_scene) => match next_scene {
                        scene::NextScene::Another(new_scene) => {
                            scene = new_scene;
                            Scene::init(&mut scene as *mut Scene, &context);
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
                        Scene::draw(&mut scene as *mut Scene, &context);
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
