use std::path::Path;

use math::{Vec3, Vec2, Transform};

#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub uv: Vec2
}

pub struct Object {
    pub vertices: Vec<[Vertex;3]>,
    pub transform: Transform
}
impl Object {
    pub fn load(path: impl AsRef<Path>, transform: Transform) -> Self {
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
            
        Self {
            vertices,
            transform
        }
    }
}