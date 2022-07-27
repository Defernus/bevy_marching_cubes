use self::{chunk::Chunk, pos::Position};
use std::slice::IterMut;

pub mod chunk;
pub mod mesh;
pub mod pos;
pub mod voxel;

pub struct ChunksHolder {
    pub chunks: Vec<Option<Chunk>>,
}

impl ChunksHolder {
    pub fn new(size: usize) -> Self {
        let volume = size * size * size;
        let chunks = (0..volume)
            .map(|index| {
                let offset = size as i64 / 2;
                let chunk_pos =
                    Self::get_pos_by_index(size, index) - Position::new(offset, offset, offset);
                let chunk = Chunk::new(chunk_pos);
                Some(chunk)
            })
            .collect();

        Self { chunks }
    }

    fn get_pos_by_index(size: usize, index: usize) -> Position {
        Position::new(
            (index % size) as i64,
            ((index / size) % size) as i64,
            (index / size / size) as i64,
        )
    }

    pub fn iter_chunks_mut(&mut self) -> IterMut<Option<Chunk>> {
        self.chunks.iter_mut()
    }
}
