#[inline(always)]
pub fn from_f32(v: f32) -> [u8;3] {
    [(v * 255.)as u8;3]
}