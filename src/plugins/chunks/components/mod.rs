use super::resources::pos::ChunkPos;

pub struct ChunkComponent {
    pub pos: ChunkPos,
}

impl ChunkComponent {
    pub fn new(pos: ChunkPos) -> Self {
        Self { pos }
    }
}
