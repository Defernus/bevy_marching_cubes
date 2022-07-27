use super::resources::pos::Position;

pub struct ChunkComponent {
    pub pos: Position,
}

impl ChunkComponent {
    pub fn new(pos: Position) -> Self {
        Self { pos }
    }
}
