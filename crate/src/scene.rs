use std::time::{self, Instant};

use cgmath::{Matrix4, SquareMatrix};
use glium::{Display, Surface};

use crate::{input::Input, object::Object, context};

pub enum NextScene {
    Another(Scene),
    Done,
}

pub struct Scene {
    pub view: [[f32;4];4],
    pub inverted_view: [[f32;4];4],
    pub proj: [[f32;4];4],
    pub delta: time::Duration,
    pub next_frame_instant: time::Instant,
    pub objects: Vec<Object>,
    pub should_be_removed: Vec<bool>,
    pub input: Input,
}
impl Default for Scene {
    fn default() -> Self {
        Self { view: Default::default(), inverted_view: Default::default(), proj: Default::default(), delta: Default::default(), next_frame_instant: Instant::now(), objects: Default::default(), should_be_removed: Default::default(), input: Default::default() }
    }
}

impl Scene {
    pub fn new(
        objects: Vec<Object>,
        view: Matrix4<f32>,
        display: &Display,
        
    ) -> Self {
        let should_be_removed = vec![false; objects.len()];

        let size = display.gl_window().window().inner_size();
        return Self {
            inverted_view : view.invert().unwrap().into(),
            objects,
            should_be_removed,
            view: view.into(),
            delta: time::Duration::from_nanos(1),
            next_frame_instant: time::Instant::now(),
            proj: cgmath::perspective(
                cgmath::Deg(90.0),
                size.width as f32 / size.height as f32,
                0.05,
                100.0,
            ).into(),
            input: Input::new(),
        };
    }
    pub unsafe fn init(scene: *mut Scene, context : &context::Context) {
        for object in &mut (*scene).objects {
            let object = object as *mut Object;
            for component in &mut (*object).components {
                component.start_scene(object, scene, context);
            }
        }
    }
    pub unsafe fn update(
        scene: *mut Scene,
        context: &context::Context,
        now: time::Instant,
        refresh_rate: time::Duration,
    ) -> Option<NextScene> {
        (*scene).inverted_view = Matrix4::from((*scene).view).invert().unwrap().into();
        (*scene).delta = refresh_rate + ((*scene).next_frame_instant - now);
        (*scene).next_frame_instant += (*scene).delta;
        for object in &mut (*scene).objects {
            let object = object as *mut Object;
            for component in &mut (*object).components {
                let possitble = component.update(object, scene, context);
                if let Some(_) = possitble {
                    return possitble;
                }
            }
        }
        (*scene).input.update_has_ran();
        return None;
    }
    pub unsafe fn draw(scene: *mut Scene, context: &context::Context) {
        let mut frame = (*context.display).draw();
        frame.clear_color_and_depth((0.0, 0.0, 0.01, 1.0), 1.0);
        for object in  &mut (*scene).objects {
            for component in &mut object.components {
                component.draw( &mut (*scene), &mut frame, context);
            }
        }
        frame.finish().unwrap();
    }
}
