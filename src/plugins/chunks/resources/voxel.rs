use bevy::prelude::Color;

#[derive(Debug, Default, Clone, Copy)]
pub struct Voxel {
    pub value: f32,
    pub color: Color,
}
