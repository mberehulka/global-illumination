use std::path::Path;
use image::{GenericImageView, Rgba};
use math::{Vec2, Vec3};

#[derive(Clone)]
pub struct Texture {
    pub size: Vec2,
    pub pixels: Vec<Vec<Vec3>>
}
impl Texture {
    pub fn load(path: impl AsRef<Path>) -> Self {
        let img = image::open(path).unwrap();
        let mut pixels = vec![vec![Vec3::default();img.width()as usize];img.height()as usize];
        for (x, y, Rgba::<u8>([r, g, b, _a])) in img.pixels() {
            pixels[y as usize][x as usize] = Vec3::new(r as f32 / 255., g as f32 / 255., b as f32 / 255.)
        }
        Self {
            size: Vec2::new(img.width()as f32, img.height()as f32),
            pixels
        }
    }
}