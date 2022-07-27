use bevy::prelude::Color;

use super::{pos::Position, voxel::Voxel};

pub const CHUNK_SIZE: usize = 32;
pub const CHUNK_VOLUME: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

pub struct Chunk {
    need_update: bool,
    pos: Position,
    voxels: Box<Vec<Voxel>>,
}

impl Chunk {
    pub fn new(pos: Position) -> Self {
        // generate chunk's voxels
        let voxels: Box<Vec<Voxel>> = Box::new(
            (0..CHUNK_VOLUME)
                .map(|index| {
                    // voxel world position - chunk offset plus voxel inchunk pos
                    let pos = Self::get_pos_by_index(index) + pos * (CHUNK_SIZE as i64);

                    let scale = 5.;
                    let stretch = 10.;
                    let value = pos.y as f32
                        + ((pos.x as f32 / stretch).cos() + (pos.y as f32 / stretch).sin()) / 2.
                            * scale;
                    Voxel {
                        value,
                        color: Color::RED,
                    }
                })
                .collect(),
        );

        Self {
            voxels,
            need_update: true,
            pos,
        }
    }

    fn get_pos_by_index(index: usize) -> Position {
        Position::new(
            (index % CHUNK_SIZE) as i64,
            ((index / CHUNK_SIZE) % CHUNK_SIZE) as i64,
            (index / CHUNK_SIZE / CHUNK_SIZE) as i64,
        )
    }

    fn get_index_by_pos(pos: Position) -> usize {
        pos.x as usize + (pos.y as usize) * CHUNK_SIZE + (pos.z as usize) * CHUNK_SIZE * CHUNK_SIZE
    }

    pub fn get_pos(&self) -> Position {
        self.pos
    }

    pub fn is_need_update(&self) -> bool {
        self.need_update
    }

    pub fn set_updated(&mut self) {
        self.need_update = false;
    }

    pub fn get_voxel(&self, pos: Position) -> Voxel {
        self.voxels[Self::get_index_by_pos(pos)]
    }
}
