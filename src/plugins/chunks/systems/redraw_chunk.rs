use crate::plugins::chunks::resources::ChunksHolder;
use bevy::{
    prelude::*,
    render::mesh::{self, PrimitiveTopology},
};

pub fn redraw_chunk_sys(
    mut commands: Commands,
    mut chunks: ResMut<ChunksHolder>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // iterate through all chunks and redraw if necessary
    chunks.iter_chunks_mut().for_each(|chunk| match chunk {
        Some(chunk) if chunk.is_need_update() => {
            let vertices = chunk.generate_vertices();

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

            commands.spawn_bundle(PbrBundle {
                mesh: meshes.add(mesh),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb(1.0, 1.0, 1.0).into(),
                    perceptual_roughness: 1.,
                    metallic: 0.,
                    reflectance: 0.,
                    ..default()
                }),
                ..default()
            });

            // set state to updated after redraw completes
            chunk.set_updated();
        }
        _ => {}
    });
}
