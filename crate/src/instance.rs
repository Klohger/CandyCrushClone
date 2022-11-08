use crate::{candy::{self, Candy}, input::{Input, self, KeyState}};
use cgmath::{Matrix4, Vector3, SquareMatrix, Deg};
use glium::{
    self,
    index::{IndexBuffer, PrimitiveType},
    uniform, Display, Surface, VertexBuffer,
    texture::{SrgbTexture2dArray}, glutin,
};
use glium::glutin::event::VirtualKeyCode as Key;
use glutin::platform::run_return::EventLoopExtRunReturn;
use image::{io::Reader as ImageReader};
use winit::{event_loop::EventLoop, event::MouseScrollDelta};
use std::{time};

pub struct Instance<'a> {
    pub input : Input,
    pub view : Matrix4<f32>,
    pub camera : Matrix4<f32>,
    pub display: &'a Display,
    pub events_loop : &'a mut EventLoop<()>,
    delta: time::Duration,
    pub next_frame_instant: time::Instant,
    candies: Vec<Candy>,
    candy_vert_buf: VertexBuffer<candy::Vertex>,
    candy_ind_buf: IndexBuffer<u16>,
    candy_prog: glium::Program,
    candy_textures: SrgbTexture2dArray,
    sampler_behaviour : glium::uniforms::SamplerBehavior,
}

impl<'a> Instance<'a> {

    pub const REFRESH_RATES: [time::Duration; 2] = [time::Duration::from_nanos(16_666_667), time::Duration::from_nanos(15_625_000)];

    pub fn draw(this : &mut Instance) {
        let mut frame = this.display.draw();
        frame.clear_color(0.0, 0.0, 0.0, 1.0);
        for candy in &this.candies {
            if let candy::Type::Normal(color) = candy.t {
                let view : [[f32;4];4] = this.view.into();
                let camera : [[f32;4];4] = this.camera.invert().unwrap().into();
                frame
                    .draw(
                        &this.candy_vert_buf,
                        &this.candy_ind_buf,
                        &this.candy_prog,
                        &uniform! {
                            pos: candy.pos, 
                            view: view,
                            camera: camera,
                            tex: glium::uniforms::Sampler(&this.candy_textures, this.sampler_behaviour),
                            colorId: color as f32,
                        },
                        &glium::DrawParameters { 
                            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
                            ..Default::default()
                        },
                    )
                    .unwrap();
            }
        }

        frame.finish().unwrap();
    }
    pub fn new(display: &'a Display, events_loop : &'a mut EventLoop<()>, candies: Vec<Candy>) -> Self {
        let candy_vert_buf = VertexBuffer::new(display, &candy::MESH).unwrap();
        let candy_ind_buf =
            IndexBuffer::new(display, PrimitiveType::TrianglesList, &candy::INDICES).unwrap();
        let candy_prog = glium::Program::from_source(
            display,
            candy::VERTEX_SHADER_SRC,
            candy::FRAGMENT_SHADER_SRC,
            None,
        )
        .unwrap();
        let size = display.gl_window().window().inner_size();
        let mut raw_images = Vec::new();
        
        for i in 0..5 {
            let image = ImageReader::open(format!("./data/textures/candy{}.png",i)).unwrap().decode().unwrap().to_rgba8();
            let image_dimension = image.dimensions();
            raw_images.push(glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimension));
        }
        let textures = glium::texture::SrgbTexture2dArray::new(display, raw_images).unwrap();

        return Self {
            input : Input::new(),
            display,
            delta: time::Duration::from_nanos(1),
            next_frame_instant: time::Instant::now(),
            candies,
            candy_vert_buf,
            candy_ind_buf,
            candy_prog,
            view: cgmath::perspective(Deg(90.0),size.width as f32/ size.height as f32,0.05,100.0),
            camera : Matrix4::from_translation(Vector3 { x: 0.0, y: 0.0, z: 0.0 }),
            candy_textures: textures,
            sampler_behaviour: glium::uniforms::SamplerBehavior {
                minify_filter: glium::uniforms::MinifySamplerFilter::Nearest,
                magnify_filter: glium::uniforms::MagnifySamplerFilter::Nearest,
                ..Default::default()
            },
            events_loop,
        };
        }
    pub fn update(&mut self, now: time::Instant, refresh_rate: time::Duration) { 


        // change window text to fps
        self.display
            .gl_window()
            .window()
            .set_title(std::format!("{} fps", 1.0 / self.delta.as_secs_f64()).as_str());
        
        
        // calculate delta
        self.delta = refresh_rate + (self.next_frame_instant - now);
        self.next_frame_instant += self.delta;

        let (w, a, s, d, t) = (self.input.key_state(Key::W), self.input.key_state(Key::A), self.input.key_state(Key::S), self.input.key_state(Key::D), self.input.key_state(Key::T));

        if KeyState::held(a) {
            self.camera = self.camera * Matrix4::from_translation(Vector3 { x: -self.delta.as_secs_f32(), y: 0.0, z: 0.0 });
        }
        if KeyState::held(d){
            self.camera = self.camera * Matrix4::from_translation(Vector3 { x: self.delta.as_secs_f32(), y: 0.0, z: 0.0 });
        }
        if KeyState::held(w){
            self.camera = self.camera * Matrix4::from_translation(Vector3 { x: 0.0, y: 0.0, z: -self.delta.as_secs_f32() });
        }
        if KeyState::held(s) {
            self.camera = self.camera * Matrix4::from_translation(Vector3 { x: 0.0, y: 0.0, z: self.delta.as_secs_f32() });
        }


        if KeyState::held(t) && KeyState::this_tick(t) {
            self.camera = self.camera * Matrix4::from_translation(Vector3 { x: 0.0, y: 0.5, z: 0.0 });
        }
        
        // clears the THIS_TICK bit from all the input
        self.input.update_has_ran()
    }
    pub fn Run(&mut self, refresh_rate: time::Duration) -> Option<Instance<'a>> {
        
        self.events_loop.run_return(|event, _target, control_flow|{
            
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
        return None;
    }
    pub fn event_handler(event : glutin::event::Event<'_, ()>, target : &glutin::event_loop::EventLoopWindowTarget<()>, control_flow : &mut glutin::event_loop::ControlFlow) {

    }
}
