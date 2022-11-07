use crate::candy::{self, Candy};
use cgmath::{Matrix4, Vector3};
use glium::{
    self,
    index::{IndexBuffer, PrimitiveType},
    uniform, Display, Surface, VertexBuffer,
};
use std::time;

pub struct Instance {
    pub view : Matrix4<f32>,
    camera : Matrix4<f32>,
    display: Display,
    delta: time::Duration,
    pub next_frame_instant: time::Instant,
    candies: Vec<Candy>,
    candy_vert_buf: VertexBuffer<candy::Vertex>,
    candy_ind_buf: IndexBuffer<u16>,
    candy_prog: glium::Program,
}

impl<'a> Instance {
    pub fn draw(&self) {
        let mut frame = self.display.draw();
        frame.clear_color(0.0, 0.0, 0.0, 1.0);
        for candy in &self.candies {
            if let candy::Type::Normal(color) = candy.t {
                let mat : [[f32;4];4] = self.view.into();
                frame
                    .draw(
                        &self.candy_vert_buf,
                        &self.candy_ind_buf,
                        &self.candy_prog,
                        &uniform! {pos: candy.pos, candyColor: candy::DEBUG_COLORS[color], view: mat},
                        &Default::default(),
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
        return Self {
            display,
            delta: time::Duration::from_nanos(1),
            next_frame_instant: time::Instant::now(),
            candies,
            candy_vert_buf,
            candy_ind_buf,
            candy_prog,
            view: cgmath::perspective(cgmath::Deg(90.0), size.width as f32 / size.height as f32, 0.1, 10.0),
            camera : Matrix4::from_translation(Vector3 { x: 0.0, y: 0.0, z: 2.0 }),
        };
        }
    pub fn update(&mut self, now: time::Instant, refresh_rate: time::Duration) { 
        self.display
            .gl_window()
            .window()
            .set_title(std::format!("{} fps", 1.0 / self.delta.as_secs_f64()).as_str());
        self.delta = refresh_rate + (self.next_frame_instant - now);
        self.next_frame_instant += self.delta;
    }
}
