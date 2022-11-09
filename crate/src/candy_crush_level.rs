use std::time;

use cgmath::{Matrix4, Vector3, Deg, SquareMatrix};
use glium::{VertexBuffer, IndexBuffer, texture::SrgbTexture2dArray, index::PrimitiveType, Surface, uniform, Display};

use crate::{input::{Input, KeyState}, candy::{self, Candy}, scene::Scene};
use image::{io::Reader as ImageReader};
use winit::event::VirtualKeyCode as Key;

pub struct CandyCrushLevel {
    pub input : Input,
    pub view : Matrix4<f32>,
    pub camera : Matrix4<f32>,
    delta: time::Duration,
    pub next_frame_instant: time::Instant,
    candies: Vec<Candy>,
    candy_vert_buf: VertexBuffer<candy::Vertex>,
    candy_ind_buf: IndexBuffer<u16>,
    candy_prog: glium::Program,
    candy_textures: SrgbTexture2dArray,
    sampler_behaviour : glium::uniforms::SamplerBehavior,
}

impl CandyCrushLevel {
    pub fn new(candies: Vec<Candy>, display : &Display) -> Self {
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
        }
    }
    pub fn draw(&self, display : &Display) {
            let mut frame = display.draw();
        frame.clear_color(0.0, 0.0, 0.0, 1.0);
        for candy in &self.candies {
            if let candy::Type::Normal(color) = candy.t {
                let view : [[f32;4];4] = self.view.into();
                let camera : [[f32;4];4] = self.camera.invert().unwrap().into();
                frame
                    .draw(
                        &self.candy_vert_buf,
                        &self.candy_ind_buf,
                        &self.candy_prog,
                        &uniform! {
                            pos: candy.pos, 
                            view: view,
                            camera: camera,
                            tex: glium::uniforms::Sampler(&self.candy_textures, self.sampler_behaviour),
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
        pub fn update(&mut self, display : &Display, now: time::Instant, refresh_rate: time::Duration) -> bool { 

            // change window text to fps
            display
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
            
            // clears the self_TICK bit from all the input
            self.input.update_has_ran();
            return true;
        }
        pub fn get_next_scene(&self) -> Option<Scene<CandyCrushLevel>> {
            return None;
        }
    }