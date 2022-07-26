use bevy::prelude::*;

use self::systems::{chunks_startup_sys, redraw_chunk::redraw_chunk_sys};

pub mod components;
pub mod resources;
mod systems;

pub struct ChunksPlugin;

impl Plugin for ChunksPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(chunks_startup_sys)
            .add_system(redraw_chunk_sys);
    }
}
