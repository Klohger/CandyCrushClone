use std::{collections::HashMap, f32::consts::PI, time::Duration, fs::File};

use crate::{
    asset,
    context::{self, Context},
    mesh::Mesh,
};
use cgmath::{num_traits::clamp, Deg, Matrix4, Vector3};
use glium::{
    uniforms::{AsUniformValue, UniformValue, Uniforms},
    DrawParameters, Frame, Rect, Surface,
};
use rodio::Source;

use crate::{
    input::KeyState,
    object::Object,
    scene::{NextScene, Scene},
};

use winit::event::VirtualKeyCode as Key;

pub trait Component {
    fn identifier(&self) -> &'static str;
    unsafe fn start_scene(
        &mut self,
        _object: *mut Object,
        _scene: *mut Scene,
        _context: &context::Context,
    ) {
    }
    unsafe fn update(
        &mut self,
        _object: *mut Object,
        _scene: *mut Scene,
        _context: &Context,
    ) -> Option<NextScene> {
        None
    }
    unsafe fn draw(&mut self, _scene: *mut Scene, _frame: &mut Frame, _context: &Context) {}
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
    unsafe fn start_scene(&mut self, object: *mut Object, _scene: *mut Scene, _context: &Context) {
        self.transform = Object::get_component(object, Transform::IDENTIFIER).unwrap();
    }
    unsafe fn update(
        &mut self,
        _object: *mut Object,
        scene: *mut Scene,
        _context: &Context,
    ) -> Option<NextScene> {
        (*scene).view = (*self.transform).model;
        None
    }
}

pub struct NoclipController {
    pub transform: *mut Transform,
    pub should_move: bool,
    pub view_x: f32,
}
impl NoclipController {
    pub const IDENTIFIER: &'static str = "NoclipController";
}
impl Component for NoclipController {
    fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
    unsafe fn start_scene(&mut self, object: *mut Object, _scene: *mut Scene, context: &Context) {
        self.transform = Object::get_component(object, Transform::IDENTIFIER).unwrap();
        let str = if self.should_move {
            "shmoovin"
        } else {
            "movement locked"
        };
        (*context.display).gl_window().window().set_title(str);
    }
    unsafe fn update(
        &mut self,
        _object: *mut Object,
        scene: *mut Scene,
        context: &Context,
    ) -> Option<NextScene> {
        let lock_key = (*scene).input.key_state(Key::L);
        if KeyState::held(lock_key) && KeyState::this_tick(lock_key) {
            self.should_move = !self.should_move;
            let str = if self.should_move {
                "shmoovin"
            } else {
                "movement locked"
            };
            (*context.display).gl_window().window().set_title(str);
        }
        if self.should_move {
            let (w, a, s, d) = (
                (*scene).input.key_state(Key::W),
                (*scene).input.key_state(Key::A),
                (*scene).input.key_state(Key::S),
                (*scene).input.key_state(Key::D),
            );
            if KeyState::held(a) {
                (*self.transform).model = (Matrix4::from((*self.transform).model)
                    * Matrix4::from_translation(Vector3 {
                        x: -(*scene).delta.as_secs_f32(),
                        y: 0.0,
                        z: 0.0,
                    }))
                .into();
            }
            if KeyState::held(d) {
                (*self.transform).model = (Matrix4::from((*self.transform).model)
                    * Matrix4::from_translation(Vector3 {
                        x: (*scene).delta.as_secs_f32(),
                        y: 0.0,
                        z: 0.0,
                    }))
                .into();
            }
            if KeyState::held(w) {
                (*self.transform).model = (Matrix4::from((*self.transform).model)
                    * Matrix4::from_translation(Vector3 {
                        x: 0.0,
                        y: 0.0,
                        z: -(*scene).delta.as_secs_f32(),
                    }))
                .into();
            }
            if KeyState::held(s) {
                (*self.transform).model = (Matrix4::from((*self.transform).model)
                    * Matrix4::from_translation(Vector3 {
                        x: 0.0,
                        y: 0.0,
                        z: (*scene).delta.as_secs_f32(),
                    }))
                .into();
            }

            (*self.transform).model = (Matrix4::from((*self.transform).model)
                * Matrix4::from_angle_x(Deg(self.view_x)))
            .into();
            (*self.transform).model = (Matrix4::from((*self.transform).model)
                * Matrix4::from_angle_y(Deg(-(*scene).input.mouse_delta.0 as f32)))
            .into();

            self.view_x += (*scene).input.mouse_delta.1 as f32;
            self.view_x = clamp(self.view_x, -90.0, 90.0);
            (*self.transform).model = (Matrix4::from((*self.transform).model)
                * Matrix4::from_angle_x(Deg(-self.view_x)))
            .into();
        }
        let esc = (*scene).input.key_state(Key::Escape);
        let should_exit = KeyState::held(esc);

        if should_exit {
            return Some(NextScene::Done);
        } else {
            return None;
        }
    }
}

pub struct ShitController {
    pub transform: *mut Transform,
    pub should_move: bool,
    pub position: Vector3<f32>,
    pub x_rotation: f32,
    pub y_rotation: f32,
}
impl ShitController {
    pub const IDENTIFIER: &'static str = "ShitController";
}
impl Component for ShitController {
    fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
    unsafe fn start_scene(&mut self, object: *mut Object, _scene: *mut Scene, context: &Context) {
        self.transform = Object::get_component(object, Transform::IDENTIFIER).unwrap();
        let str = if self.should_move {
            "shmoovin"
        } else {
            "movement locked"
        };
        (*context.display).gl_window().window().set_title(str);
    }
    unsafe fn update(
        &mut self,
        _object: *mut Object,
        scene: *mut Scene,
        context: &Context,
    ) -> Option<NextScene> {
        let lock_key = (*scene).input.key_state(Key::L);
        if KeyState::held(lock_key) && KeyState::this_tick(lock_key) {
            self.should_move = !self.should_move;
            let str = if self.should_move {
                "shmoovin"
            } else {
                "movement locked"
            };
            (*context.display).gl_window().window().set_title(str);
        }
        if self.should_move {
            let (w, a, s, d) = (
                (*scene).input.key_state(Key::W),
                (*scene).input.key_state(Key::A),
                (*scene).input.key_state(Key::S),
                (*scene).input.key_state(Key::D),
            );

            if KeyState::held(a) {
                self.position.x -= (*scene).delta.as_secs_f32();
            }
            if KeyState::held(d) {
                self.position.x += (*scene).delta.as_secs_f32();
            }
            if KeyState::held(w) {
                self.position.z -= (*scene).delta.as_secs_f32();
            }
            if KeyState::held(s) {
                self.position.z += (*scene).delta.as_secs_f32();
            }

            self.x_rotation -= (*scene).input.mouse_delta.1 as f32;
            self.x_rotation = clamp(self.x_rotation, -90.0, 90.0);
            self.y_rotation -= (*scene).input.mouse_delta.0 as f32;

            (*self.transform).model = (Matrix4::from_angle_y(Deg(self.y_rotation))
                * Matrix4::from_angle_x(Deg(self.x_rotation))
                * Matrix4::from_translation(self.position))
            .into();
        }
        let esc = (*scene).input.key_state(Key::Escape);
        let should_exit = KeyState::held(esc);

        if should_exit {
            return Some(NextScene::Done);
        } else {
            return None;
        }
    }
}
pub struct MeshRenderer {
    pub mesh: *const Mesh,
    pub prog: *const glium::Program,
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

impl MeshRenderer {
    pub const IDENTIFIER: &'static str = "MeshRenderer";
}
impl Component for MeshRenderer {
    fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
    unsafe fn start_scene(
        &mut self,
        object: *mut Object,
        _scene: *mut Scene,
        _context: &context::Context,
    ) {
        self.transform = Object::get_component(object, "Transform").unwrap();
    }
    unsafe fn draw(&mut self, scene: *mut Scene, frame: &mut Frame, _context: &Context) {
        self.uniforms
            .0
            .insert("proj", (*scene).proj.as_uniform_value());
        self.uniforms
            .0
            .insert("view", (*scene).inverted_view.as_uniform_value());
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

pub struct Title {
    pub mesh_renderer: *mut MeshRenderer,
}
impl Title {
    pub const IDENTIFIER: &'static str = "Title";
}
impl Component for Title {
    unsafe fn start_scene(
        &mut self,
        object: *mut Object,
        _scene: *mut Scene,
        _context: &context::Context,
    ) {
        self.mesh_renderer = Object::get_component(object, MeshRenderer::IDENTIFIER).unwrap();
    }
    unsafe fn update(
        &mut self,
        _object: *mut Object,
        scene: *mut Scene,
        _context: &Context,
    ) -> Option<NextScene> {
        if let UniformValue::Float(offset) =
            (*self.mesh_renderer).uniforms.0.get_mut("offset").unwrap()
        {
            *offset += (*scene).delta.as_secs_f32();
        }
        None
    }
    fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
pub struct Splash {
    pub mesh_renderer: *mut MeshRenderer,
}
impl Splash {
    pub const IDENTIFIER: &'static str = "Splash";
}
impl Component for Splash {
    unsafe fn start_scene(
        &mut self,
        object: *mut Object,
        _scene: *mut Scene,
        _context: &context::Context,
    ) {
        self.mesh_renderer = Object::get_component(object, MeshRenderer::IDENTIFIER).unwrap();
    }
    unsafe fn update(
        &mut self,
        _object: *mut Object,
        scene: *mut Scene,
        _context: &Context,
    ) -> Option<NextScene> {
        if let UniformValue::Float(scale) =
            (*self.mesh_renderer).uniforms.0.get_mut("scale").unwrap()
        {
            *scale += (*scene).delta.as_secs_f32() * PI;
        }
        None
    }
    fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}

pub struct Exit {
    pub scene: Option<unsafe fn(&Context) -> Scene>,
    pub transform: *const Transform,
    pub min: Vector3<f32>,
    pub max: Vector3<f32>,
}
impl Exit {
    pub const IDENTIFIER: &'static str = "Exit";
}
impl Component for Exit {
    fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
    unsafe fn update(
        &mut self,
        _object: *mut Object,
        _scene: *mut Scene,
        context: &Context,
    ) -> Option<NextScene> {
        let [x, y, z, _] = (*self.transform).model[3];

        if x > self.min.x
            && x < self.max.x
            && y > self.min.y
            && y < self.max.y
            && z > self.min.z
            && z < self.max.z
        {
            if let Some(func) = self.scene {
                Some(NextScene::Another(func(context)))
            } else {
                Some(NextScene::Done)
            }
        } else {
            None
        }
    }
}
pub enum WackyState {
    Waiting,
    Fading,
    DoingTheFunny,
    Crash,
}
pub struct TheWackyEntrance {
    pub timer: Duration,
    pub player: *const Transform,
    pub mesh_renderer: *mut MeshRenderer,
    pub min: Vector3<f32>,
    pub max: Vector3<f32>,
    pub state: WackyState,
    pub currentFrame: usize,
}
impl TheWackyEntrance {
    pub const IDENTIFIER: &'static str = "🤡";
    const FADE_TIME: Duration = Duration::from_millis(1_500);
    const VIDEO_FADE_TIME: Duration = Duration::from_secs(3);
    const fps: Duration = Duration::from_nanos(33_333_333);
    const swag: Rect = Rect {
        left: 0,
        bottom: 0,
        width: 256,
        height: 144,
    };
}
use image::{io::Reader as ImageReader, ImageBuffer, Rgba, AnimationDecoder};
impl Component for TheWackyEntrance {
    fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
    unsafe fn start_scene(
        &mut self,
        _object: *mut Object,
        _scene: *mut Scene,
        _context: &context::Context,
    ) {
        
        let frames = image::codecs::gif::GifDecoder::new(File::open("data/media/skyrim.gif").unwrap()).unwrap().into_frames();
        for frame in frames {
            let swag = frame.unwrap();
        }
    }
    unsafe fn update(
        &mut self,
        _object: *mut Object,
        scene: *mut Scene,
        context: &Context,
    ) -> Option<NextScene> {
        let [x, y, z, _] = (*self.player).model[3];
        match self.state {
            WackyState::Waiting => {
                if x > self.min.x
                    && x < self.max.x
                    && y > self.min.y
                    && y < self.max.y
                    && z > self.min.z
                    && z < self.max.z
                {
                    self.state = WackyState::Fading;
                    self.timer = Duration::ZERO;
                    context.sinks["music"].pause();
                    context.sinks["sfx0"].append(asset::load_audio("data/media/start.ogg"));
                    context.sinks["sfx0"].play();
                }

                None
            }
            WackyState::Fading => {
                self.timer += (*scene).delta;

                *(*self.mesh_renderer).uniforms.0.get_mut("opacity").unwrap() =
                    UniformValue::Float((self.timer.as_secs_f32() / Self::FADE_TIME.as_secs_f32()));
                if self.timer >= Self::FADE_TIME {
                    self.state = WackyState::DoingTheFunny;
                    self.timer = Duration::ZERO;
                }
                None
            }
            WackyState::DoingTheFunny => {
                self.timer += (*scene).delta;
                context.sinks["trolos"].append(asset::load_audio("data/media/skyrim.ogg"));
                context.sinks["trolos"].play();
                let swag = ((self.timer.as_nanos() / Self::fps.as_nanos()) + 1) as usize;
                if swag > self.currentFrame {
                    self.currentFrame = swag;
                    context.textures["pain"].write(
                        Self::swag,
                        glium::texture::RawImage2d::from_raw_rgba_reversed(
                            ImageReader::open(format!("data/media/skyrim/out{}.jpg", self.currentFrame))
                                .unwrap()
                                .decode()
                                .unwrap()
                                .to_rgba8()
                                .as_raw(),
                            (256, 144),
                        ),
                    );
                }
                *(*self.mesh_renderer)
                    .uniforms
                    .0
                    .get_mut("video_opacity")
                    .unwrap() = UniformValue::Float(clamp(
                    self.timer.as_secs_f32() / Self::VIDEO_FADE_TIME.as_secs_f32(),
                    0.0,
                    1.0,
                ));

                None
            }
            WackyState::Crash => Some(NextScene::Done),
        }
    }
}
