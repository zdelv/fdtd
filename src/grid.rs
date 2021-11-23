use std::iter::IntoIterator;

#[derive(Clone, Copy)]
pub struct Size {
    x: usize,
    y: usize,
    z: usize,
}

/// A cartesian grid based on the common x,y,z unit vectors. Can be 1D, 2D, or 3D.
pub struct CartesianGrid<const X: usize, const Y: usize, const Z: usize> {
    grid: Vec<f64>,
    size: Size,
}

type CartesianGrid3D<const X: usize, const Y: usize, const Z: usize> = CartesianGrid<X, Y, Z>;
type CartesianGrid2D<const X: usize, const Y: usize> = CartesianGrid<X, Y, 0>;
type CartesianGrid1D<const X: usize> = CartesianGrid<X, 0, 0>;

impl<const X: usize> CartesianGrid1D<X> {

}

impl CartesianGrid {
    // TODO: Use an option here to signify if failure occurs on the conversion
    // pub fn new(x: u32, y: u32, z: u32, dx: f64, dy: f64, dz: f64) -> Self {
    //     Self {
    //         grid: Vec::with_capacity((x * y).try_into().unwrap()),
    //         size: Size { x: x as usize, y: y as usize },
    //     }
    // }

    pub fn new_3d(x: usize, y: usize, z: usize) -> Self {
        Self {
            grid: Vec::with_capacity(x * y * z),
            size: Size { x, y, z },
        }
    }

    pub fn new_2d(x: usize, y: usize) -> Self {
        Self {
            grid: Vec::with_capacity(x * y),
            size: Size { x, y, z: 0 }
        }
    }

    pub fn new_1d(x: usize) -> Self {
        Self {
            grid: Vec::with_capacity(x),
            size: Size { x, y: 0, z: 0 }
        }
    }

    pub fn at(&self, x: usize, y: usize) -> Option<&f64> {
        self.grid.get((x * y) + x)
    }

    pub fn row_iter(&self) -> RowIter {
        RowIter {
            inner: &self.grid,
            pos: 0,
            size: self.size,
        }
    }
}

struct RowIter<'a> {
    inner: &'a Vec<f64>,
    pos: usize,
    size: Size,
}

impl<'a> Iterator for RowIter<'a> {
    type Item = &'a [f64];

    fn next(&mut self) -> Option<Self::Item> {
        let start: usize = self.pos * self.size.x;
        self.inner.get((start)..(start + self.size.x))
    }
}

#[test]
fn grid_iter_test() {
    let grid = CartesianGrid::new(10, 20, 1.0 / 10.0, 1.0 / 20.0);

    let out: usize = grid.row_iter().map(|x| x.len()).sum();
    assert_eq!(out, 10 * 20_usize)
}
