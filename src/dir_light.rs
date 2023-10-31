use math::Vec3;

pub struct DirectionalLight {
    pub direction: Vec3
}
impl Default for DirectionalLight {
    fn default() -> Self {
        Self {
            direction: Vec3::new(0.1, 0.5, -1.)
        }
    }
}