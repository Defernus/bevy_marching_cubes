use super::voxel::Voxel;
use bevy::{
    math::Vec3,
    prelude::{Color, Mesh},
    render::mesh::{self, PrimitiveTopology},
};
pub mod append_vertices;
pub mod edge_midpoints;
pub mod triangulation_table;

pub struct Vertex {
    pub pos: Vec3,
    pub normal: Vec3,
    pub color: Color,
}

pub type BlockOfVoxels = [[[Voxel; 2]; 2]; 2];

pub fn mesh_rom_vertices(vertices: Vec<Vertex>) -> Mesh {
    let mut indices = Vec::new();

    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut colors: Vec<u32> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    for vertex in vertices.iter() {
        indices.push(positions.len() as u32);

        positions.push(vertex.pos.into());
        normals.push(vertex.normal.into());
        colors.push(vertex.color.as_rgba().as_rgba_u32());
        uvs.push([0., 0.]);
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(mesh::Indices::U32(indices)));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);

    mesh
}
