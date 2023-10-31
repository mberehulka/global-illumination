use winit::{event_loop::EventLoop, window::{WindowBuilder, Window}, dpi::PhysicalSize};
use pixels::{Pixels, SurfaceTexture};
use math::Quaternion;

use crate::{object::Object, camera::Camera, dir_light::DirectionalLight, render::{draw, clear}};

pub struct Engine<'s> {
    pub buff_w4: i32,

    pub width: u32,
    pub height: u32,
    pub pixels: Pixels,
    pub zbuffer: Vec<f32>,
    
    pub window: Window,
    pub objects: Vec<Object<'s>>,
    pub camera: Camera,
    pub dir_light: DirectionalLight
}

impl<'s> Engine<'s> {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let window = WindowBuilder::new()
            .with_resizable(false)
            .with_inner_size(PhysicalSize {
                width: 800,
                height: 600
            })
            .build(&event_loop).unwrap();
        let width = window.inner_size().width;
        let height = window.inner_size().height;
        let pixels = Pixels::new(width, height, SurfaceTexture::new(width, height, &window)).unwrap();
        Self {
            buff_w4: window.inner_size().width as i32 * 4,

            width: window.inner_size().width,
            height: window.inner_size().height,
            pixels,
            zbuffer: vec![f32::MAX;(width * height)as usize],
            
            window,
            objects: vec![],
            camera: Camera::new(),
            dir_light: DirectionalLight::default()
        }
    }
    pub fn rotate_objects(&mut self) {
        for object in self.objects.iter_mut() {
            object.transform.rotation = object.transform.rotation * Quaternion::from_angle_y(0.001);
        }
    }
    pub fn update(&mut self) {
        self.rotate_objects();
        
        let pixels = self.pixels.frame_mut();
        clear(pixels);
        self.camera.update(self.width, self.height);
        
        self.zbuffer.fill(f32::MAX);
        
        for object_id in 0..self.objects.len() {
            self.objects[object_id].update_shadow_map(&self.objects);
            draw(
                self.width as i32, self.height as i32,
                pixels,
                &mut self.zbuffer,
                &self.objects[object_id],
                &self.camera,
                &self.dir_light
            )
        }

        self.pixels.render().unwrap()
    }
}