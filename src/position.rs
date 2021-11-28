pub struct Position1D {
    pub x: usize
}

pub struct Position2D {
    pub x: usize,
    pub y: usize
}

pub struct Position3D {
    pub x: usize,
    pub y: usize,
    pub z: usize
}

pub trait Position {}

impl Position for Position1D {}
impl Position for Position2D {}
impl Position for Position3D {}