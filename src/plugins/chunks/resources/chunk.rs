use bevy::prelude::Mesh;

use super::{
    mesh::{append_vertices::append_vertices, mesh_rom_vertices, Vertex},
    pos::Position,
    voxel::Voxel,
};

pub const CHUNK_REAL_SIZE: usize = 32;
pub const CHUNK_VOXELS_SIZE: usize = CHUNK_REAL_SIZE + 1;
pub const CHUNK_VOLUME: usize = CHUNK_VOXELS_SIZE * CHUNK_VOXELS_SIZE * CHUNK_VOXELS_SIZE;

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
                    let pos = Self::get_pos_by_index(index) + pos * (CHUNK_REAL_SIZE as i64);

                    let scale = 5.;
                    let stretch = 10.;
                    let value = pos.y as f32
                        + ((pos.x as f32 / stretch).cos() + (pos.z as f32 / stretch).sin()) / 2.
                            * scale;
                    Voxel { value }
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
            (index % CHUNK_VOXELS_SIZE) as i64,
            ((index / CHUNK_VOXELS_SIZE) % CHUNK_VOXELS_SIZE) as i64,
            (index / CHUNK_VOXELS_SIZE / CHUNK_VOXELS_SIZE) as i64,
        )
    }

    fn get_index_by_pos(pos: Position) -> usize {
        pos.x as usize
            + (pos.y as usize) * CHUNK_VOXELS_SIZE
            + (pos.z as usize) * CHUNK_VOXELS_SIZE * CHUNK_VOXELS_SIZE
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

    pub fn generate_vertices(&self) -> Vec<Vertex> {
        let mut vertices: Vec<Vertex> = Vec::new();
        for x in 0..CHUNK_REAL_SIZE {
            for y in 0..CHUNK_REAL_SIZE {
                for z in 0..CHUNK_REAL_SIZE {
                    append_vertices(
                        Position::new(x as i64, y as i64, z as i64),
                        self,
                        &mut vertices,
                    );
                }
            }
        }

        for v in vertices.iter_mut() {
            v.pos = v.pos + (self.pos * (CHUNK_REAL_SIZE as i64)).to_vec();
        }

        vertices
    }

    pub fn generate_mesh(&self) -> Mesh {
        let vertices = self.generate_vertices();
        mesh_rom_vertices(vertices)
    }
}
