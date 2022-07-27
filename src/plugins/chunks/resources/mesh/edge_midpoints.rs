/// Midpoints of each edge of a cube
pub const EDGE_MIDPOINTS: [[f32; 3]; 12] = [
    // bottom cube edges
    [0.5, 0.0, 1.0],
    [1.0, 0.0, 0.5],
    [0.5, 0.0, 0.0],
    [0.0, 0.0, 0.5],
    // top cube edges
    [0.5, 1.0, 1.0],
    [1.0, 1.0, 0.5],
    [0.5, 1.0, 0.0],
    [0.0, 1.0, 0.5],
    // middle cube edges
    [0.0, 0.5, 1.0],
    [1.0, 0.5, 1.0],
    [1.0, 0.5, 0.0],
    [0.0, 0.5, 0.0],
];

#[repr(usize)]
pub enum EdgeMidpointsIndices {
    BottomNorth,
    BottomEast,
    BottomSouth,
    BottomWest,
    TopNorth,
    TopEast,
    TopSouth,
    TopWest,
    NorthWest,
    NorthEast,
    SouthEast,
    SouthWest,
}

pub const CUBE_EDGES_COUNT: usize = 12;
