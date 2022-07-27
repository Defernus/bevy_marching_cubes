use super::{
    edge_midpoints::EDGE_MIDPOINTS, triangulation_table::get_triangles_by_voxels, BlockOfVoxels,
    Vertex,
};
use crate::plugins::chunks::resources::{chunk::Chunk, pos::Position, voxel::Voxel};
use bevy::{math::Vec3, prelude::Color};

/// Append vertices for 8 voxels at position "pos" based on triangulation table
///
/// # Example
/// one of 256 possible cases:  
/// 3 voxels at the bottom are filled(with value > 0), the rest are empty (value <= 0)  
/// ```
///        -1                             -2
///         +-   -   -   -   -   -   -   - +
///        /                              /
///                                         
///      /                              /   
///  -3 +-   -   -   -   -   -   -   - + -7  
///         .------------------------------.
///        /|                         ___./
///       /         pos->+       ____/   | |
///      /  |               ____/        |
///     .-----------------./             | |
///         |              \            |
///     |                    \          |  |
///       1 +-     -     -     \     -  |  + 4
///     |  /                     \     |  /
///                                \   |
///     |/                           \ |/
///   3 +-       -       -       -     . -1
/// ```
///
pub fn append_vertices(pos: Position, chunk: &Chunk, vertices: &mut Vec<Vertex>) {
    let voxels = get_voxels_for_vertex(chunk, pos);

    let triangles = get_triangles_by_voxels(voxels);

    let mut triangle_offset = 0;

    // iterate through triangles
    while triangles[triangle_offset] != -1 {
        // get 3 triangle points and append them to the mesh
        let a: Vec3 = EDGE_MIDPOINTS[triangles[triangle_offset] as usize].into();
        let b: Vec3 = EDGE_MIDPOINTS[triangles[triangle_offset + 1] as usize].into();
        let c: Vec3 = EDGE_MIDPOINTS[triangles[triangle_offset + 2] as usize].into();

        append_triangle(pos, vertices, a, b, c, Color::rgb(0.5, 0.45, 0.4));

        triangle_offset += 3;
    }
}

fn append_triangle(
    pos: Position,
    vertices: &mut Vec<Vertex>,
    a: Vec3,
    b: Vec3,
    c: Vec3,
    color: Color,
) {
    let pos_vec = Vec3::new(pos.x as f32, pos.y as f32, pos.z as f32);

    let a = a + pos_vec;
    let b = b + pos_vec;
    let c = c + pos_vec;

    let normal = (c - a).cross(b - a).normalize();

    vertices.push(Vertex {
        color,
        normal,
        pos: a,
    });
    vertices.push(Vertex {
        color,
        normal,
        pos: b,
    });
    vertices.push(Vertex {
        color,
        normal,
        pos: c,
    });
}

/// Returns a 3D array with 8 voxels as cube vertices.
fn get_voxels_for_vertex(chunk: &Chunk, base_pos: Position) -> BlockOfVoxels {
    let voxels: [[[Voxel; 2]; 2]; 2] = [
        [
            [
                chunk.get_voxel(base_pos + Position::new(0, 0, 0)),
                chunk.get_voxel(base_pos + Position::new(0, 0, 1)),
            ],
            [
                chunk.get_voxel(base_pos + Position::new(0, 1, 0)),
                chunk.get_voxel(base_pos + Position::new(0, 1, 1)),
            ],
        ],
        [
            [
                chunk.get_voxel(base_pos + Position::new(1, 0, 0)),
                chunk.get_voxel(base_pos + Position::new(1, 0, 1)),
            ],
            [
                chunk.get_voxel(base_pos + Position::new(1, 1, 0)),
                chunk.get_voxel(base_pos + Position::new(1, 1, 1)),
            ],
        ],
    ];
    return voxels;
}
