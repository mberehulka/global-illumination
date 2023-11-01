use std::sync::{Arc, Mutex};

use noto_sans_mono_bitmap::{get_raster, FontWeight, RasterHeight, get_raster_width};

#[derive(Default, Clone)]
pub struct Log(Arc<Mutex<String>>);
impl Log {
    pub fn set(&self, text: String) {
        *self.0.lock().unwrap() = text
    }
    pub fn get(&self) -> String {
        self.0.lock().unwrap().clone()
    }
}

pub fn render_text(
    width: usize,
    pixels: &mut [u8],
    text: &str
) {
    let weight = FontWeight::Regular;
    let size = RasterHeight::Size16;
    let char_width = get_raster_width(weight, size);
    let mut offset_x = 0;
    let mut offset_y = 0;
    for c in text.chars() {
        if c == '\n' {
            offset_x = 0;
            offset_y += char_width * 2;
            continue
        }
        let char_raster = get_raster(c, weight, size).unwrap();
        for (y, row) in char_raster.raster().iter().enumerate() {
            for (x, pixel) in row.iter().enumerate() {
                let i = ((y + offset_y) * width + (x + offset_x)) * 4;
                pixels[i  ] = *pixel;
                pixels[i+1] = *pixel;
                pixels[i+2] = *pixel;
            }
        }
        offset_x += char_width
    }
}