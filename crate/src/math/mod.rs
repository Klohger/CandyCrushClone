use std::ops::{Deref, DerefMut};

use cgmath::{num_traits::Float, Vector4};

pub struct Matrix4<'a, F: Float>{
    underlying : cgmath::Matrix4<F>,
    cur : usize,
}


impl<F : Float> Deref for Matrix4<'_, F> {
    type Target = cgmath::Matrix4<F>;

    
    fn deref(&self) -> &Self::Target {
        &self.underlying
    }
}
impl<F : Float> DerefMut for Matrix4<'_, F> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.underlying
    }
}
impl<'a, F:Float> Iterator for Matrix4<'a, F> {
    type Item = Vector4<F>;
    fn next(&mut self) -> Option<Self::Item> {
        let swag =  [&mut self.underlying.x, &mut self.underlying.y, &mut self.underlying.z, &mut self.underlying.w];
        if self.cur < swag.len() {
            let current = swag[self.cur];
            self.cur += 1;

            return Some(current);
        } else {
            return None;
        }
    }
    fn count(self) -> usize {   
        return 4 * 4;
    }
}