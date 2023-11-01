use std::{path::Path, sync::{Mutex, atomic::AtomicU32}};
use math::{Vec3, Vec2, Transform};

use crate::{texture::Texture, gimap::GIMap};

#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub uv: Vec2
}

pub static ID: AtomicU32 = AtomicU32::new(0);

pub struct Object {
    pub id: u32,
    pub triangles: Vec<[Vertex;3]>,
    pub transform: Mutex<Transform>,
    pub texture: &'static Texture,
    pub gimap: GIMap
}
impl Object {
    pub fn load(
        path: impl AsRef<Path>,
        texture: &'static Texture,
        transform: Transform
    ) -> Self {
        let id = ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        
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
        let triangles = readers.iter()
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
            .collect::<Vec<_>>();
            
        let gimap = GIMap::new(id, &texture, &triangles, 1. / 10.).into();
            
        Self {
            id,
            triangles,
            texture,
            gimap,
            transform: transform.into()
        }
    }
}