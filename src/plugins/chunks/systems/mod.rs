use super::resources::ChunksHolder;
use bevy::prelude::*;

pub mod redraw_chunk;

const BASE_WORLD_SIZE: usize = 8;

pub fn chunks_startup_sys(mut commands: Commands) {
    let chunks = ChunksHolder::new(BASE_WORLD_SIZE);

    commands.insert_resource(chunks);
}
