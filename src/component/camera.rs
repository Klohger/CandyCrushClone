pub struct Camera {
    pub transform: *mut super::transform::Transform,
}
impl Camera {
    pub const IDENTIFIER: &'static str = "Camera";
}
impl super::Component for Camera {
    fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
    unsafe fn start_scene(&mut self, object: *mut crate::object::Object, _scene: *mut crate::scene::Scene, _context: &crate::context::Context) {
        self.transform = crate::object::Object::get_component(object, super::transform::Transform::IDENTIFIER).unwrap();
    }
    unsafe fn update(
        &mut self,
        _object: *mut crate::object::Object,
        scene: *mut crate::scene::Scene,
        _context: &crate::context::Context,
    ) -> Option<crate::scene::NextScene> {
        (*scene).view = (*self.transform).model;
        None
    }
}