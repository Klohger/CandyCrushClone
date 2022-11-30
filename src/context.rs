use std::collections::HashMap;

use glium::{texture::SrgbTexture2d, Display};
use rodio::{ OutputStream, OutputStreamHandle, Sink};

use crate::mesh::Mesh;

pub struct Context {
    pub display: *const Display,
    pub textures: HashMap<&'static str, SrgbTexture2d>,
    pub meshes: HashMap<&'static str, Mesh>,
    pub shader_programs: HashMap<&'static str, glium::program::Program>,
    stream: OutputStream,
    stream_handle: OutputStreamHandle,
    pub sinks : HashMap<&'static str, Sink>,
}

impl Context {
    pub fn new<const NUM : usize>(
        display: *const Display,
        textures: HashMap<&'static str, SrgbTexture2d>,
        meshes: HashMap<&'static str, Mesh>,
        shader_programs: HashMap<&'static str, glium::program::Program>,
        sinks : [&'static str; NUM],
    ) -> Self {
        let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        let mut map : HashMap<&'static str, Sink> = HashMap::with_capacity(NUM);
        for i in 0..NUM {
            map.insert(sinks[i], Sink::try_new(&stream_handle).unwrap());
        }
        Self {
            display,
            textures,
            meshes,
            shader_programs,
            stream,
            stream_handle,
            sinks : map,
        }
    }

}
