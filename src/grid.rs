use std::ops::{Index, IndexMut};

use crate::position::{Position, Position1D, Position2D, Position3D};

#[derive(Clone, Copy)]
pub struct Size {
    x: usize,
    y: usize,
    z: usize,
}

impl Size {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }
}

pub trait Grid {}

/// A cartesian grid based on the common x, y, and z unit vectors. Can be N-dimensional, based on 
/// an associated constant `DIMENSION`. `GridData` represents the inner data type of the grid.
/// 
/// TODO: Adding a macro for implementing this might be helpful
pub trait CartesianGrid 
    : IndexMut<usize> + Index<usize, Output = Self::GridData> + Grid
where
    <Self as Index<usize>>::Output: Copy
{
    /// Inner datatype of the grid
    type GridData;

    /// Type used to designate a position in space
    type PositionType;

    /// Number of dimensions
    const DIMENSION: u8;

    /// Unit Vector in the X-direction
    const UNIT_X: [f64; 3] = [1.0, 0.0, 0.0];
    /// Unit Vector in the Y-direction
    const UNIT_Y: [f64; 3] = [0.0, 1.0, 0.0];
    /// Unit Vector in the Z-direction
    const UNIT_Z: [f64; 3] = [0.0, 0.0, 1.0];

    /// Returns an empty Cartesian grid
    /// TODO: Probably can just use [`Default`] here instead
    fn empty() -> Self;

    /// TODO: There must be a *much* better solution to this. I'd like to avoid using a slice and
    /// forcing the user to check at runtime. This Size struct works but it seems annoying to create
    /// a struct each time the grid is made. Providing just N parameters for N-dimensional
    /// implementations would be nice. Const generics maybe.
    fn new(size: Size) -> Self;

    /// Returns a mutable reference to a grid point. The grid is accessed differently based on the
    /// dimension. This function normalizes the interface.
    fn at(&mut self, idx: Self::PositionType) -> &mut Self::GridData;
}

/// A one-dimensional Cartesian grid.
/// See [`CartesianGrid`] for the trait implementation
pub struct CartesianGrid1D {
    data: Vec<f64>,
    size: Size
}

impl CartesianGrid for CartesianGrid1D {
    type GridData = f64;
    type PositionType = Position1D;

    const DIMENSION: u8 = 1;

    fn empty() -> Self {
        Self {
            data: Vec::new(),
            size: Size { x: 0, y: 0, z: 0 }
        }
    }

    fn new(size: Size) -> Self {
        Self {
            data: Vec::with_capacity(size.x),
            size
        }
    }

    fn at(&mut self, idx: Self::PositionType) -> &mut Self::GridData {
        &mut self[idx.x]
    }
}

impl Index<usize> for CartesianGrid1D {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for CartesianGrid1D {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

/// A two-dimensional Cartesian grid.
/// See [`CartesianGrid`] for the trait implementation
pub struct CartesianGrid2D {
    data: Vec<f64>,
    size: Size
}

impl CartesianGrid for CartesianGrid2D {
    type GridData = f64;
    type PositionType = Position2D;

    const DIMENSION: u8 = 1;

    fn empty() -> Self {
        Self {
            data: Vec::new(),
            size: Size { x: 0, y: 0, z: 0 }
        }
    }

    fn new(size: Size) -> Self {
        Self {
            data: Vec::with_capacity(size.x * size.y),
            size
        }
    }

    fn at(&mut self, idx: Self::PositionType) -> &mut Self::GridData {
        let pos = (self.size.x * idx.y) + idx.x;
        &mut self[ pos ]
    }
}

impl Index<usize> for CartesianGrid2D {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for CartesianGrid2D {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

/// A three-dimensional Cartesian grid.
/// See [`CartesianGrid`] for the trait implementation
pub struct CartesianGrid3D {
    data: Vec<f64>,
    size: Size
}

impl CartesianGrid for CartesianGrid3D {
    type GridData = f64;
    type PositionType = Position3D;

    const DIMENSION: u8 = 1;

    fn empty() -> Self {
        Self {
            data: Vec::new(),
            size: Size { x: 0, y: 0, z: 0 }
        }
    }

    fn new(size: Size) -> Self {
        Self {
            data: Vec::with_capacity(size.x),
            size
        }
    }

    fn at(&mut self, idx: Self::PositionType) -> &mut Self::GridData {
        let pos = ((self.size.x * self.size.y) * idx.z) + (self.size.x * idx.y) + idx.x;
        &mut self[ pos ]
    }
}

impl Index<usize> for CartesianGrid3D {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for CartesianGrid3D {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
