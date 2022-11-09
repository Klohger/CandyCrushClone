use std::time;

use glium::Display;

pub struct Scene<T>{
    pub data : T,
    pub update : fn(&mut T, &Display, time::Instant, time::Duration) -> bool,
    pub draw : fn(&T, &Display),
    pub get_next_scene : fn(&T) -> Option<Scene<T>>
}

impl<T> Scene<T> {
    pub fn new(data : T, update : fn(&mut T, &Display, time::Instant, time::Duration) -> bool, draw : fn(&T, &Display), get_next_scene : fn(&T) -> Option<Scene<T>>) -> Self {
        return Self {
            data,
            update,
            draw,
            get_next_scene,
        };
    }
}

