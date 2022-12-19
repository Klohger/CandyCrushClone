

use std::collections::HashMap;

use crate::{context::{self, Context}, mesh::Mesh};

use cgmath::{Matrix4, SquareMatrix};
use glium::{Frame, DrawParameters, uniforms::{UniformValue, Uniforms, AsUniformValue}, Surface, Display};

use crate::{
    object::Object,
    scene::{NextScene, Scene},
};

pub trait Component {
    fn identifier(&self) -> &'static str;
    unsafe fn start_scene(
        &mut self,
        object: *mut Object,
        scene: *mut Scene,
        display : &Display,
        context: &context::Context,
    ) {
    }
    unsafe fn update(
        &mut self,
        object: *mut Object,
        scene: *mut Scene,
        display : &Display,
        context: &Context,
    ) -> Option<NextScene> {
        None
    }
    unsafe fn draw(&mut self, scene: *mut Scene, frame: &mut Frame, display : &Display, context: &Context) {}
}
pub struct Transform {
    pub model: [[f32; 4]; 4],
    pub translation: [[f32; 4]; 4],
    pub rotation: [[f32; 4]; 4],
    pub scale: [[f32; 4]; 4],
}
impl Transform {
    pub fn new(
        model: [[f32; 4]; 4],
        translation: [[f32; 4]; 4],
        rotation: [[f32; 4]; 4],
        scale: [[f32; 4]; 4],
    ) -> Self {
        Self {
            model,
            translation,
            rotation,
            scale,
        }
    }

    pub fn set_model(_matrix: [[f32; 4]; 4]) {
        todo!()
    }
    pub fn set_translation(_matrix: [[f32; 4]; 4]) {
        todo!()
    }
    pub fn set_local_translation(_matrix: [[f32; 4]; 4]) {
        todo!()
    }
    pub fn set_rotation(_matrix: [[f32; 4]; 4]) {
        todo!()
    }
    pub fn set_local_rotation(_matrix: [[f32; 4]; 4]) {
        todo!()
    }
    pub fn set_scale(_matrix: [[f32; 4]; 4]) {
        todo!()
    }
    pub const IDENTIFIER: &'static str = "Transform";
}
impl Default for Transform {
    fn default() -> Self {
        Self {
            model: Default::default(),
            translation: Default::default(),
            rotation: Default::default(),
            scale: Default::default(),
        }
    }
}
impl Component for Transform {
    fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}

pub struct Camera {
    pub transform: *mut Transform,
}
impl Camera {
    pub const IDENTIFIER: &'static str = "Camera";
}
impl Component for Camera {
    fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
    unsafe fn start_scene(&mut self, object: *mut crate::object::Object, _scene: *mut crate::scene::Scene, _display : &Display, _context: &crate::context::Context) {
        self.transform = crate::object::Object::get_component(object, Transform::IDENTIFIER).unwrap();
    }
    unsafe fn update(
        &mut self,
        _object: *mut crate::object::Object,
        scene: *mut crate::scene::Scene,
        _display : &Display,
        _context: &crate::context::Context,
    ) -> Option<crate::scene::NextScene> {
        (*scene).view = (*self.transform).model;
        None
    }
}

pub struct MeshRenderer<'a> {
    pub mesh: &'a Mesh,
    pub prog: &'a glium::Program,
    pub uniforms: DynamicUniforms,
    pub draw_parameters: DrawParameters<'static>,
    pub transform: *const Transform,
}


pub struct DynamicUniforms(pub HashMap<&'static str, UniformValue<'static>>);
impl Uniforms for DynamicUniforms {
    fn visit_values<'a, F>(&'a self, mut next_uniform: F)
    where
        F: FnMut(&str, UniformValue<'a>),
    {
        for (name, value) in &self.0 {
            // 'static outlives 'a so it's fine to just pass values in here.
            next_uniform(name, *value);
        }
    }
}
#[macro_export]
macro_rules! dynamic_uniform {
    () => {
        $crate::uniforms::EmptyUniforms
    };

    ($field:ident: $value:expr) => {
        $crate::component::DynamicUniforms($crate::collection!{stringify!($field).to_string() => $value.as_uniform_value()})
    };

    ($field1:ident: $value1:expr, $($field:ident: $value:expr),+) => {
        {
            let uniforms = $crate::uniforms::UniformsStorage::new(stringify!($field1), $value1);
            $(
                let uniforms = uniforms.add(stringify!($field), $value);
            )+
            uniforms
        }
    };

    ($($field:ident: $value:expr),*,) => {
        $crate::dynamic_uniform!($($field: $value),*)
    };
}

impl MeshRenderer<'_> {
    pub const IDENTIFIER: &'static str = "MeshRenderer";
}
impl Component for MeshRenderer<'_> {
    fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
    unsafe fn start_scene(
        &mut self,
        object: *mut Object,
        _scene: *mut Scene,
        _display : &Display,
        _context: &context::Context,
    ) {
        self.transform = Object::get_component(object, "Transform").unwrap();
    }
    unsafe fn draw(&mut self, scene: *mut Scene, frame: &mut Frame, _display : &Display, _context: &Context) {
        self.uniforms
            .0
            .insert("proj", (*scene).proj.as_uniform_value());
        self.uniforms
            .0
            .insert("view", UniformValue::Mat4(Matrix4::from((*scene).view).invert().unwrap().into()));
        self.uniforms
            .0
            .insert("model", (*self.transform).model.as_uniform_value());

        frame
            .draw(
                &(*(self.mesh)).vertex_buffer,
                &(*(self.mesh)).index_buffer,
                &*self.prog,
                &self.uniforms,
                &self.draw_parameters,
            )
            .unwrap();
    }
}