use std::time;

use cgmath::{Deg, Matrix4, SquareMatrix, Vector3};
use glium::{
    index::PrimitiveType, texture::SrgbTexture2dArray, uniform, Display, IndexBuffer, Surface,
    VertexBuffer,
};

use crate::{
    candy::{self, Candy},
    input::{Input, KeyState},
    scene::{NextScene, Scene},
};
use image::io::Reader as ImageReader;
use winit::event::VirtualKeyCode as Key;

pub struct TestScene {
    counter: u8,
    pub input: Input,
    pub projection: Matrix4<f32>,
    pub camera: Matrix4<f32>,
    camera_x: f32,
    pub delta: time::Duration,
    pub next_frame_instant: time::Instant,
    candies: Vec<Candy>,
    candy_vert_buf: VertexBuffer<candy::Vertex>,
    candy_ind_buf: IndexBuffer<u16>,
    candy_prog: glium::Program,
    candy_textures: SrgbTexture2dArray,
    sampler_behaviour: glium::uniforms::SamplerBehavior,
}

impl TestScene {
    pub fn new(candies: Vec<Candy>, display: &Display, counter: u8) -> Self {
        let candy_vert_buf = VertexBuffer::new(display, &candy::MESH)?;
        let candy_ind_buf =
            IndexBuffer::new(display, PrimitiveType::TrianglesList, &candy::INDICES)?;
        let candy_prog = glium::Program::from_source(
            display,
            candy::VERTEX_SHADER_SRC,
            candy::FRAGMENT_SHADER_SRC,
            None,
        )
        ?;
        let size = display.gl_window().window().inner_size();
        let mut raw_images = Vec::new();

        for i in 0..5 {
            let image = ImageReader::open(format!("data/textures/candy{}.png", i))
                ?
                .decode()
                ?
                .to_rgba8();
            let image_dimension = image.dimensions();
            raw_images.push(glium::texture::RawImage2d::from_raw_rgba_reversed(
                &image.into_raw(),
                image_dimension,
            ));
        }
        let textures = glium::texture::SrgbTexture2dArray::new(display, raw_images)?;
        return Self {
            counter,
            input: Input::new(),
            camera_x: 0.0,
            delta: time::Duration::from_nanos(1),
            next_frame_instant: time::Instant::now(),
            candies,
            candy_vert_buf,
            candy_ind_buf,
            candy_prog,
            projection: cgmath::perspective(
                Deg(90.0),
                size.width as f32 / size.height as f32,
                0.05,
                100.0,
            ),
            camera: Matrix4::from_translation(Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }),
            candy_textures: textures,
            sampler_behaviour: glium::uniforms::SamplerBehavior {
                minify_filter: glium::uniforms::MinifySamplerFilter::Nearest,
                magnify_filter: glium::uniforms::MagnifySamplerFilter::Nearest,
                ..Default::default()
            },
        };
    }
}
