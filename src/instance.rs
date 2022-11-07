use crate::{candy::{self, Candy}, input::Input};
use cgmath::{Matrix4, Vector3, SquareMatrix, Deg};
use glium::{
    self,
    index::{IndexBuffer, PrimitiveType},
    uniform, Display, Surface, VertexBuffer,
    glutin::event::VirtualKeyCode, texture::{RawImage2d, SrgbTexture2dArray},
};
use image::{io::Reader as ImageReader, image_dimensions};
use std::{time, fmt::format};

pub struct Instance {
    pub input : Input,
    pub view : Matrix4<f32>,
    pub camera : Matrix4<f32>,
    pub display: Display,
    delta: time::Duration,
    pub next_frame_instant: time::Instant,
    candies: Vec<Candy>,
    candy_vert_buf: VertexBuffer<candy::Vertex>,
    candy_ind_buf: IndexBuffer<u16>,
    candy_prog: glium::Program,
    candy_textures: SrgbTexture2dArray,
    sampler_behaviour : glium::uniforms::SamplerBehavior,
}

impl<'a> Instance {
    pub fn draw(&self) {
        let mut frame = self.display.draw();
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
    pub fn new(display: Display, candies: Vec<Candy>) -> Self {
        let candy_vert_buf = VertexBuffer::new(&display, &candy::MESH).unwrap();
        let candy_ind_buf =
            IndexBuffer::new(&display, PrimitiveType::TrianglesList, &candy::INDICES).unwrap();
        let candy_prog = glium::Program::from_source(
            &display,
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
        let textures = glium::texture::SrgbTexture2dArray::new(&display, raw_images).unwrap();

        return Self {
            input : Input::new(),
            display,
            delta: time::Duration::from_nanos(1),
            next_frame_instant: time::Instant::now(),
            candies,
            candy_vert_buf,
            candy_ind_buf,
            candy_prog,
            view: cgmath::perspective(Deg(90.0),(size.width as f32/ size.height as f32),0.05,100.0),
            camera : Matrix4::from_translation(Vector3 { x: 0.0, y: 0.0, z: 0.0 }),
            candy_textures: textures,
            sampler_behaviour: glium::uniforms::SamplerBehavior {
                minify_filter: glium::uniforms::MinifySamplerFilter::Nearest,
                magnify_filter: glium::uniforms::MagnifySamplerFilter::Nearest,
                ..Default::default()
            }
        };
        }
    pub fn update(&mut self, now: time::Instant, refresh_rate: time::Duration) { 
        self.display
            .gl_window()
            .window()
            .set_title(std::format!("{} fps", 1.0 / self.delta.as_secs_f64()).as_str());
        self.delta = refresh_rate + (self.next_frame_instant - now);
        self.next_frame_instant += self.delta;
        if self.input.is_key_held(VirtualKeyCode::A){
            self.camera = self.camera * Matrix4::from_translation(Vector3 { x: -self.delta.as_secs_f32(), y: 0.0, z: 0.0 });
        }
        if self.input.is_key_held(VirtualKeyCode::D){
            self.camera = self.camera * Matrix4::from_translation(Vector3 { x: self.delta.as_secs_f32(), y: 0.0, z: 0.0 });
        }
        if self.input.is_key_held(VirtualKeyCode::W){
            self.camera = self.camera * Matrix4::from_translation(Vector3 { x: 0.0, y: 0.0, z: -self.delta.as_secs_f32() });
        }
        if self.input.is_key_held(VirtualKeyCode::S){
            self.camera = self.camera * Matrix4::from_translation(Vector3 { x: 0.0, y: 0.0, z: self.delta.as_secs_f32() });
        }
    }
}
