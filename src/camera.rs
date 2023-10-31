use std::f32::consts::PI;

use math::{Vec3, Mat4x4, Vec2};

pub struct Camera {
    pub translation: Vec3,
    pub rotation: Vec2,
    pub distance: f32,
    pub position: Vec3,
    pub mat: Mat4x4
}
impl Camera {
    pub fn new() -> Self {
        Self {
            translation: Vec3::new(0., 0., 0.),
            rotation: Vec2::new(0., 0.),
            distance: 4.,
            position: Default::default(),
            mat: Default::default()
        }
    }
    pub fn update(&mut self, width: u32, height: u32) {
        self.rotation.x = self.rotation.x.min(PI / 3.).max(-PI / 3.);
        self.position = Vec3::new(0., 0., self.distance)
            .rotate_x(self.rotation.x)
            .rotate_y(self.rotation.y)
            + self.translation;
        let aspect = width as f32 / height as f32;
        let proj = Mat4x4::perspective(aspect, aspect, 0.01, 100.);
        let view = Mat4x4::look_at(self.position, self.translation);
        self.mat = proj * view;
    }
}