use std::{fs::File, io::BufReader};

use glium::{backend::Facade, texture::SrgbTexture2d, Display, IndexBuffer, VertexBuffer};
use image::io::Reader as ImageReader;
use obj::{load_obj, FromRawVertex, Obj};
use rodio::Decoder;

use crate::mesh::Mesh;

pub type AssetLoader<In, Out> = fn(In) -> Asset<Out>;

pub type Asset<Out> = Option<Out>;

pub fn load_texture<F: Facade>(image_name: &str, facade: &F) -> SrgbTexture2d {
    let image = ImageReader::open(image_name)
        .unwrap()
        .decode()
        .unwrap()
        .to_rgba8();
    let image_dimension = image.dimensions();
    let texture = glium::texture::SrgbTexture2d::new(
        facade,
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimension),
    )
    .unwrap();
    return texture;
}
pub fn load_model<
    V: FromRawVertex<I> + glium::vertex::Vertex,
    I: glium::index::Index,
    F: Facade,
>(
    path: &str,
    facade: &F,
) -> Mesh<V, I> {
    let input = BufReader::new(File::open(path).unwrap());
    let model: Obj<V, I> = load_obj(input).unwrap();
    let (verts, inds) = (model.vertices, model.indices);
    return Mesh {
        vertex_buffer: VertexBuffer::new(facade, &verts).unwrap(),
        index_buffer: IndexBuffer::new(facade, glium::index::PrimitiveType::TrianglesList, &inds)
            .unwrap(),
    };
}
pub fn load_program(path: &str, display: &Display) -> glium::Program {
    return glium::Program::from_source(
        display,
        std::fs::read_to_string(path.to_string() + ".vert")
            .unwrap()
            .as_str(),
        std::fs::read_to_string(path.to_string() + ".frag")
            .unwrap()
            .as_str(),
        None,
    )
    .unwrap();
}
pub fn load_audio(path: &str) -> Decoder<BufReader<File>> {
    rodio::decoder::Decoder::new(std::io::BufReader::new(std::fs::File::open(path).unwrap()))
        .unwrap()
}
pub fn load_video(path: &str) /*-> mp4::Mp4Reader<BufReader<File>>*/ {
    /*
    let f = File::open(path).unwrap();
    let size = f.metadata().unwrap().len();
    // Track info.
    mp4::Mp4Reader::read_header(BufReader::new(f), size).unwrap()
    */
}
/*
pub trait Asset {
    type Input;
    type Output;
    fn value(&self) -> &Option<Self::Output>;
    fn load(&mut self);
    fn new(input : Self::Input) -> Self;

}


pub struct TextureAsset {
    texture: Option<Result<SrgbTexture2d,TextureCreationError>>,
}
impl Asset for TextureAsset {
    type Input = (&str,&Display);

    type Output = Result<SrgbTexture2d,TextureCreationError>;

    fn value(&self) -> &Option<Self::Output> {
        &self.texture
    }

    fn load(&mut self) {
        let image = ImageReader::open(self.image_name)
            .unwrap()
            .decode()
            .unwrap()
            .to_rgba8();
        let image_dimension = image.dimensions();
        let texture = glium::texture::SrgbTexture2d::new(
            self.display,
            glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimension),
        );
        self.texture = Some(texture);
    }

    fn new(input : (&'a str,&'a Display)) -> Self {
        return Self { texture: None, image_name: input.0, display: input.1 }
    }
}
*/
