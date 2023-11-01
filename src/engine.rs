use winit::{event_loop::EventLoop, window::{WindowBuilder, Window}, dpi::PhysicalSize};
use pixels::{Pixels, SurfaceTexture};
use math::Quaternion;

use crate::{object::Object, camera::Camera, dir_light::DirectionalLight, render::{draw, clear}, text::{render_text, Log}};

pub struct Engine {
    pub buff_w4: i32,

    pub width: u32,
    pub height: u32,
    pub pixels: Pixels,
    pub zbuffer: Vec<f32>,
    
    pub window: Window,
    pub objects: &'static [Object],
    pub camera: Camera,
    pub dir_light: &'static DirectionalLight,

    pub logs: Vec<Log>
}

impl Engine {
    pub fn new(
        event_loop: &EventLoop<()>,
        objects: &'static [Object],
        logs: Vec<Log>
    ) -> Self {
        let window = WindowBuilder::new()
            .with_title("Global illumination")
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
            objects,
            camera: Camera::new(),
            dir_light: Box::leak(Box::new(DirectionalLight::default())),

            logs
        }
    }
    pub fn rotate_object(&mut self) {
        let object = &self.objects.first().unwrap();
        let mut transform = object.transform.lock().unwrap();
        transform.rotation = transform.rotation * Quaternion::from_angle_y(0.001);
    }
    pub fn update(&mut self) {
        self.rotate_object();
        
        let pixels = self.pixels.frame_mut();
        clear(pixels);
        self.camera.update(self.width, self.height);
        
        self.zbuffer.fill(f32::MAX);
        
        for object in self.objects.iter() {
            draw(
                self.width as i32, self.height as i32,
                pixels,
                &mut self.zbuffer,
                object,
                &self.camera
            )
        }

        render_text(
            self.width as usize,
            pixels,
            &self.logs.iter().map(|log|log.get()).collect::<Vec<_>>().join("\n")
        );

        self.pixels.render().unwrap()
    }
}