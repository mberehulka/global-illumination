use std::{sync::{Mutex, atomic::AtomicU32}, mem::transmute};
use math::Vec3;

use crate::{object::{Object, Vertex}, dir_light::DirectionalLight, texture::Texture};

#[derive(Default, Clone, Copy)]
pub struct GITextureVertex {
    pub position: Vec3,
    pub triangle_id: u32
}

pub struct GIMap {
    pub obj_id: u32,
    pub width: usize,
    pub height: usize,
    pub values: Vec<Vec<AtomicU32>>,
    pub vertices: Mutex<Vec<Vec<GITextureVertex>>>
}
impl GIMap {
    pub fn new(
        obj_id: u32,
        texture: &Texture,
        triangles: &[[Vertex;3]],
        gi_texture_scale: f32
    ) -> Self {
        let width = texture.size.x * gi_texture_scale;
        let height = texture.size.y * gi_texture_scale;
        
        let mut values = Vec::with_capacity(height as usize);
        for _ in 0..height as usize {
            let mut row = Vec::with_capacity(width as usize);
            for _ in 0..width as usize {
                row.push(AtomicU32::new(unsafe{ transmute(1f32) }))
            }
            values.push(row)
        }

        let mut vertices = vec![vec![Default::default(); width as usize]; height as usize];
        for (triangle_id, [a, b, c]) in triangles.iter().enumerate() {
            raster_triangle(
                width as i32, height as i32,
                &mut vertices,
                triangle_id as u32,
                (a.uv.x * width)as i32, (a.uv.y * height)as i32,
                (b.uv.x * width)as i32, (b.uv.y * height)as i32,
                (c.uv.x * width)as i32, (c.uv.y * height)as i32,
                Vec3::new(a.position.x, b.position.x, c.position.x),
                Vec3::new(a.position.y, b.position.y, c.position.y),
                Vec3::new(a.position.z, b.position.z, c.position.z)
            )
        }
        
        Self {
            obj_id,
            width: width as usize,
            height: height as usize,
            values,
            vertices: vertices.into()
        }
    }
    #[inline(always)]
    pub fn get_value(&self, x: f32, y: f32) -> f32 {
        unsafe { transmute(self.values[y as usize][x as usize].load(std::sync::atomic::Ordering::Relaxed)) }
    }
    #[inline(always)]
    pub fn set_value(&self, x: usize, y: usize, v: f32) {
        self.values[y][x].store(unsafe { transmute(v) }, std::sync::atomic::Ordering::Relaxed)
    }
    pub fn update(
        &self,
        objects: &[Object],
        dir_light: &DirectionalLight
    ) {
        let s_object = &objects[self.obj_id as usize];
        let transform = s_object.transform.lock().unwrap().clone();
        let vertices = self.vertices.lock().unwrap();
        for y in 0..self.height {
            for x in 0..self.width {
                let vertex = vertices[y][x];
                let triangle = &s_object.triangles[vertex.triangle_id as usize][0];
                let object_normal = (transform.rotation * triangle.normal).normalized();
                let intensity = object_normal.dot(dir_light.direction);
                self.set_value(x, y, intensity)
            }
        }
    }
}
#[inline(always)]
fn raster_triangle(
    width: i32, height: i32,
    vertices: &mut Vec<Vec<GITextureVertex>>,
    triangle_id: u32,
    ax: i32, ay: i32,
    bx: i32, by: i32,
    cx: i32, cy: i32,
    px: Vec3, py: Vec3, pz: Vec3
) {
    let max_width = width - 1;
    let max_height = height - 1;
    
    let minx = max_width.min(ax).max(0).min(bx).max(0).min(cx).max(0);
    let mut miny = max_height.min(ay).max(0).min(by).max(0).min(cy).max(0);
    let maxx = 0.max(ax).min(max_width).max(bx).min(max_width).max(cx).min(max_width);
    let maxy = 0.max(ay).min(max_height).max(by).min(max_height).max(cy).min(max_height);

    let l1x = cx as f32 - ax as f32;
    let l1y = bx as f32 - ax as f32;
    let mut l1z;
    let l2x = cy as f32 - ay as f32;
    let l2y = by as f32 - ay as f32;
    let mut l2z;
    
    let mut ux; let mut uy;
    let uz = (l1x * l2y) - (l1y * l2x);
    if uz.abs() < 1. { return }

    let mut x;
    let mut baryc = Vec3::default();
    
    while miny <= maxy {
        l2z = ay as f32 - miny as f32;
        x = minx;
        while x <= maxx {
            l1z = ax as f32 - x as f32;
            ux = (l1y * l2z) - (l1z * l2y);
            uy = (l1z * l2x) - (l1x * l2z);

            baryc.x = 1.-(ux+uy)/uz;
            if baryc.x < 0. { x += 1; continue }

            baryc.y = uy/uz;
            if baryc.y < 0. { x += 1; continue }

            baryc.z = ux/uz;
            if baryc.z < 0. { x += 1; continue }

            vertices[miny as usize][x as usize] = GITextureVertex {
                position: Vec3::new(px.dot(baryc), py.dot(baryc), pz.dot(baryc)),
                triangle_id
            };
            
            x += 1;
        }
        miny += 1;
    }
}