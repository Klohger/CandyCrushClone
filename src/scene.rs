use std::time::{self, Instant};

use cgmath::{Matrix4, SquareMatrix};
use glium::{Display, Surface};

use crate::{context, input::Input, object::Object};

pub enum NextScene {
    Another(Scene),
    Done,
}

pub struct Scene {
    pub view: [[f32; 4]; 4],
    pub proj: [[f32; 4]; 4],
    pub delta: time::Duration,
    pub next_frame_instant: time::Instant,
    pub objects: Vec<Object>,
    pub should_be_removed: Vec<bool>,
    pub input: Input,
    pub clear_color: (f32, f32, f32, f32),
}
impl Default for Scene {
    fn default() -> Self {
        Self {
            view: Matrix4::identity().into(),
            proj: cgmath::perspective(
                cgmath::Deg(90.0),
                1920.0 / 1080.0 as f32,
                0.05,
                100.0,
            )
            .into(),
            delta: time::Duration::from_nanos(1),
            next_frame_instant: Instant::now(),
            objects: vec![],
            should_be_removed: vec![],
            input: Default::default(),
            clear_color: (0.0, 0.0, 0.0, 1.0),
        }
    }
}

impl Scene {
    pub fn new(
        display: &Display,
        objects: Vec<Object>,
        view: Matrix4<f32>,
        clear_color: (f32, f32, f32, f32),
    ) -> Self {

        let size = display.gl_window().window().inner_size();
        let should_be_removed = vec![false; objects.len()];
        return Self {
            objects,
            should_be_removed,
            proj: cgmath::perspective(
                cgmath::Deg(90.0),
                size.width as f32 / size.height as f32,
                0.05,
                100.0,
            )
            .into(),
            clear_color,
            view: view.into(),
            ..Default::default()
        };
    }
    pub fn new_without_view_and_clear_color(display: &Display, objects: Vec<Object>) -> Self {
        let size = display.gl_window().window().inner_size();
        let should_be_removed = vec![false; objects.len()];
        return Self {
            objects,
            should_be_removed,
            proj: cgmath::perspective(
                cgmath::Deg(90.0),
                size.width as f32 / size.height as f32,
                0.05,
                100.0,
            )
            .into(),
            ..Default::default()
        };
    }
    pub fn new_without_clear_color(
        display: &Display,
        objects: Vec<Object>,
        view: Matrix4<f32>,
    ) -> Self {

        let size = display.gl_window().window().inner_size();
        let should_be_removed = vec![false; objects.len()];
        return Self {
            objects,
            should_be_removed,
            proj: cgmath::perspective(
                cgmath::Deg(90.0),
                size.width as f32 / size.height as f32,
                0.05,
                100.0,
            )
            .into(),
            view: view.into(),
            ..Default::default()
        };
    }
    pub fn new_without_view(
        display: &Display,
        objects: Vec<Object>,
        clear_color: (f32, f32, f32, f32),
    ) -> Self {

        let size = display.gl_window().window().inner_size();
        let should_be_removed = vec![false; objects.len()];
        return Self {
            objects,
            should_be_removed,
            proj: cgmath::perspective(
                cgmath::Deg(90.0),
                size.width as f32 / size.height as f32,
                0.05,
                100.0,
            )
            .into(),
            clear_color,
            ..Default::default()
        };
    }
    pub unsafe fn init(scene: *mut Scene, context: &context::Context) {
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
        frame.clear_color_and_depth((*scene).clear_color, 1.0);
        for object in &mut (*scene).objects {
            for component in &mut object.components {
                component.draw(&mut (*scene), &mut frame, context);
            }
        }
        frame.finish().unwrap();
    }
}
