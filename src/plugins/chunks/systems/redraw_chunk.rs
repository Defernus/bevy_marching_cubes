use crate::plugins::chunks::resources::{chunk::CHUNK_SIZE, ChunksHolder};
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
            let pos = chunk.get_pos();

            // Add a cube for now. Later we will implement the construction of the voxel mesh
            commands.spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: Transform::from_translation(pos.to_vec() * (CHUNK_SIZE as f32)),
                ..default()
            });

            // set state to updated after redraw completes
            chunk.set_updated();
        }
        _ => {}
    });
}
