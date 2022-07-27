use crate::plugins::chunks::resources::{mesh::mesh_rom_vertices, ChunksHolder};
use bevy::prelude::*;

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
            let mesh = mesh_rom_vertices(vertices);

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
