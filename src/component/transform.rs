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
impl super::Component for Transform {
    fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}