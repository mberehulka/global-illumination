use std::{path::Path, cell::RefCell};
use math::{Vec3, Vec2, Transform};

use crate::texture::Texture;

#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub uv: Vec2
}

pub struct Object<'s> {
    pub vertices: Vec<[Vertex;3]>,
    pub transform: Transform,
    pub texture: &'s Texture,
    pub shadow_map: RefCell<Texture>
}
impl<'s> Object<'s> {
    pub fn load(
        path: impl AsRef<Path>,
        texture: &'s Texture,
        transform: Transform
    ) -> Self {
        let (gltf, buffers, _) = gltf::import(path).unwrap();
        let meshes = gltf.meshes().collect::<Vec<_>>();
        let primitives = meshes.iter().map(|mesh| mesh.primitives() ).flatten().collect::<Vec<_>>();
        let readers = primitives.iter()
            .map(|primitive| primitive.reader(|buffer| Some(&buffers[buffer.index()])) )
            .collect::<Vec<_>>();
        let mut readers_sizes = Vec::new();

        let positions = readers.iter()
            .map(|reader| {
                let pos = reader.read_positions().unwrap().collect::<Vec<_>>();
                readers_sizes.push(pos.len()as u32);
                pos
            })
            .flatten()
            .collect::<Vec<_>>();

        let normals = readers.iter()
            .map(|reader| reader.read_normals().unwrap().collect::<Vec<_>>())
            .flatten()
            .collect::<Vec<_>>();

        let uvs = readers.iter()
            .map(|reader| reader.read_tex_coords(0).unwrap().into_f32() )
            .flatten()
            .collect::<Vec<_>>();

        let mut index_reader_offset = 0;
        let vertices = readers.iter()
            .zip(readers_sizes)
            .map(|(reader, reader_size)| {
                let res = reader.read_indices().unwrap().into_u32()
                    .map(|i| i + index_reader_offset)
                    .collect::<Vec<_>>();
                index_reader_offset += reader_size;
                res
            })
            .flatten()
            .collect::<Vec<_>>()
            .chunks_exact(3)
            .map(|v| [
                Vertex {
                    position: positions[v[0]as usize].into(),
                    normal: normals[v[0]as usize].into(),
                    uv: uvs[v[0]as usize].into()
                },
                Vertex {
                    position: positions[v[1]as usize].into(),
                    normal: normals[v[1]as usize].into(),
                    uv: uvs[v[1]as usize].into()
                },
                Vertex {
                    position: positions[v[2]as usize].into(),
                    normal: normals[v[2]as usize].into(),
                    uv: uvs[v[2]as usize].into()
                }
            ])
            .collect();
            
        let shadow_map = Texture::new(texture.size.x as usize / 5, texture.size.y as usize / 5, 0.5.into()).into();
            
        Self {
            vertices,
            texture,
            shadow_map,
            transform
        }
    }
    pub fn update_shadow_map(&self, objects: &[Self]) {
        let mut shadow_map = self.shadow_map.borrow_mut();
        let width = shadow_map.size.y as usize;
        let height = shadow_map.size.x as usize;
        let mut x;
        let mut y;
        for object in objects.iter() {
            y = 0;
            while y < height {
                x = 0;
                while x < width {
                    x += 1
                }
                y += 1
            }
        }
    }
}