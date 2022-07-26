use std::slice::IterMut;

use self::pos::ChunkPos;

pub mod pos;

pub struct ChunksHolder {
    pub chunks: Vec<Option<Chunk>>,
    size: usize,
}

impl ChunksHolder {
    pub fn new(size: usize) -> Self {
        let volume = size * size * size;
        let chunks = (0..volume)
            .map(|index| {
                let chunk_pos = Self::get_pos_by_cords(size, index);
                let chunk = Chunk::new(chunk_pos);
                Some(chunk)
            })
            .collect();

        Self { chunks, size }
    }

    fn get_pos_by_cords(size: usize, index: usize) -> ChunkPos {
        ChunkPos::new(
            (index % size) as i64,
            ((index / size) % size) as i64,
            (index / size / size) as i64,
        )
    }

    pub fn iter_chunks_mut(&mut self) -> IterMut<Option<Chunk>> {
        self.chunks.iter_mut()
    }
}

pub const CHUNK_SIZE: usize = 32;

pub struct Chunk {
    need_update: bool,
    pos: ChunkPos,
}

impl Chunk {
    pub fn new(pos: ChunkPos) -> Self {
        Self {
            need_update: true,
            pos,
        }
    }

    pub fn get_pos(&self) -> ChunkPos {
        self.pos
    }

    pub fn is_need_update(&self) -> bool {
        self.need_update
    }

    pub fn set_updated(&mut self) {
        self.need_update = false;
    }
}
