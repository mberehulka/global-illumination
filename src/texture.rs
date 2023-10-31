use std::path::Path;
use image::{GenericImageView, Rgba};
use math::Vec2;

pub struct Texture {
    pub size: Vec2,
    pub pixels: Vec<Vec<(u8, u8, u8)>>
}
impl Texture {
    pub fn load(path: impl AsRef<Path>) -> Self {
        let img = image::open(path).unwrap();
        let mut pixels = vec![vec![(0, 0, 0);img.width()as usize];img.height()as usize];
        for (x, y, Rgba::<u8>([r, g, b, _a])) in img.pixels() {
            pixels[y as usize][x as usize] = (r, g, b)
        }
        Self {
            size: Vec2::new((img.width()-1)as f32, (img.height()-1)as f32),
            pixels
        }
    }
}