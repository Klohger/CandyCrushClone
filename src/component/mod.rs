

use crate::context::{self, Context};

use glium::Frame;

use crate::{
    object::Object,
    scene::{NextScene, Scene},
};



pub mod transform;
pub mod camera;

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