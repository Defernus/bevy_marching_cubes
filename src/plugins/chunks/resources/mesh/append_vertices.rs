use super::{
    edge_midpoints::{EdgeMidpointsIndices, CUBE_EDGES_COUNT},
    triangulation_table::get_triangles_by_voxels,
    BlockOfVoxels, Vertex,
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
    let midpoints = get_vertex_midpoints(voxels);

    let triangles = get_triangles_by_voxels(voxels);

    let mut triangle_offset = 0;

    // iterate through triangles
    while triangles[triangle_offset] != -1 {
        // get 3 triangle points and append them to the mesh
        let a = midpoints[triangles[triangle_offset] as usize];
        let b = midpoints[triangles[triangle_offset + 1] as usize];
        let c = midpoints[triangles[triangle_offset + 2] as usize];

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

    voxels
}

fn mix_positions(a: Vec3, b: Vec3, a_val: f32, b_val: f32) -> Vec3 {
    let transition = a_val / (a_val - b_val);

    a * (1.0 - transition) + b * transition
}

fn get_midpoint_data(voxels: &BlockOfVoxels, a: [usize; 3], b: [usize; 3]) -> Vec3 {
    let voxel_a = voxels[a[0]][a[1]][a[2]];
    let voxel_b = voxels[b[0]][b[1]][b[2]];

    let vec_a = Vec3::new(a[0] as f32, a[1] as f32, a[2] as f32);
    let vec_b = Vec3::new(b[0] as f32, b[1] as f32, b[2] as f32);

    if voxel_a.value > 0. && voxel_b.value <= 0. {
        mix_positions(vec_a, vec_b, voxel_a.value, voxel_b.value)
    } else if voxel_a.value <= 0. && voxel_b.value > 0. {
        mix_positions(vec_b, vec_a, voxel_b.value, voxel_a.value)
    } else {
        Vec3::default()
    }
}

/// get real vertex position for each midpoint
fn get_vertex_midpoints(voxels: BlockOfVoxels) -> [Vec3; CUBE_EDGES_COUNT] {
    let mut result = [Vec3::default(); CUBE_EDGES_COUNT];

    result[EdgeMidpointsIndices::BottomSouth as usize] =
        get_midpoint_data(&voxels, [0, 0, 0], [1, 0, 0]);
    result[EdgeMidpointsIndices::BottomEast as usize] =
        get_midpoint_data(&voxels, [1, 0, 0], [1, 0, 1]);
    result[EdgeMidpointsIndices::BottomNorth as usize] =
        get_midpoint_data(&voxels, [0, 0, 1], [1, 0, 1]);
    result[EdgeMidpointsIndices::BottomWest as usize] =
        get_midpoint_data(&voxels, [0, 0, 0], [0, 0, 1]);

    result[EdgeMidpointsIndices::NorthEast as usize] =
        get_midpoint_data(&voxels, [1, 0, 1], [1, 1, 1]);
    result[EdgeMidpointsIndices::NorthWest as usize] =
        get_midpoint_data(&voxels, [0, 0, 1], [0, 1, 1]);
    result[EdgeMidpointsIndices::SouthEast as usize] =
        get_midpoint_data(&voxels, [1, 0, 0], [1, 1, 0]);
    result[EdgeMidpointsIndices::SouthWest as usize] =
        get_midpoint_data(&voxels, [0, 0, 0], [0, 1, 0]);

    result[EdgeMidpointsIndices::TopSouth as usize] =
        get_midpoint_data(&voxels, [0, 1, 0], [1, 1, 0]);
    result[EdgeMidpointsIndices::TopEast as usize] =
        get_midpoint_data(&voxels, [1, 1, 0], [1, 1, 1]);
    result[EdgeMidpointsIndices::TopNorth as usize] =
        get_midpoint_data(&voxels, [0, 1, 1], [1, 1, 1]);
    result[EdgeMidpointsIndices::TopWest as usize] =
        get_midpoint_data(&voxels, [0, 1, 0], [0, 1, 1]);

    result
}
