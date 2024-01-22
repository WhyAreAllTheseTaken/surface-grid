//! A module containing grids wrapped around spheres.

use std::{f64::consts::PI, ops::{Index, IndexMut}};

use static_array::HeapArray2D;

use crate::{GridPoint, SurfaceGrid};

/// A point on a spherical grid.
pub trait SpherePoint : GridPoint {
    /// Gets a sphere point for the specified geographic coordinates.
    ///
    /// - `latitude` - The latitude of the point in radians where 0 is the equator.
    /// - `longitude` - The longitude of the point in radians.
    fn from_geographic(latitude: f64, longitude: f64) -> Self;

    /// Gets the latitude of this point.
    fn latitude(&self) -> f64;

    /// Gets the longitude of this point.
    fn longitude(&self) -> f64;
    
    /// Returns a coordinate containing the latitude and longitude of this point.
    /// This returns a point with the X component being the longitude and the Y component being the
    /// latitude.
    fn sphere_coordinates(&self) -> (f64, f64) {
        (self.longitude(), self.latitude())
    }
}

/// A grid for a sphere based on the equirectangular projection.
///
/// # Type Parameters
/// - `T` - The type of data that the grid holds.
/// - `W` - The width of the grid.
/// - `H` - The height of the grid.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RectangleSphereGrid<T, const W: usize, const H: usize> {
    /// The data held in this grid.
    data: HeapArray2D<T, W, H>,
}

impl <T, const W: usize, const H: usize> SurfaceGrid<T> for RectangleSphereGrid<T, W, H> {
    type Point = RectangleSpherePoint<W, H>;

    fn from_fn<F: FnMut(&Self::Point) -> T>(mut f: F) -> Self {
        Self {
            data: HeapArray2D::from_fn(|y, x| {
                let point = RectangleSpherePoint::new(x, y);

                f(&point)
            })
        }
    }
}

impl <T, const W: usize, const H: usize> Index<RectangleSpherePoint<W, H>> for RectangleSphereGrid<T, W, H> {
    type Output = T;

    fn index(&self, index: RectangleSpherePoint<W, H>) -> &Self::Output {
        &self.data[index.y][index.x]
    }
}

impl <T, const W: usize, const H: usize> IndexMut<RectangleSpherePoint<W, H>> for RectangleSphereGrid<T, W, H> {
    fn index_mut(&mut self, index: RectangleSpherePoint<W, H>) -> &mut Self::Output {
        &mut self.data[index.y][index.x]
    }
}

/// A point on a `RectangleSphereGrid`.
///
/// # Type Parameters
/// - `W` - The width of the grid.
/// - `H` - The height of the grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RectangleSpherePoint<const W: usize, const H: usize> {
    /// The X position in the grid.
    x: usize,
    /// The Y position in the grid.
    y: usize,
}

impl <const W: usize, const H: usize> RectangleSpherePoint<W, H> {
    pub fn new(x: usize, y: usize) -> Self {
        let x = (x + y / H).rem_euclid(W);
        let y = y.rem_euclid(H);

        Self {
            x,
            y
        }
    }
}

impl <const W: usize, const H: usize> GridPoint for RectangleSpherePoint<W, H> {
    fn up(&self) -> Self {
        if self.x > W / 2 {
            if self.y == H - 1 {
                Self {
                    x: (self.x + W / 2).rem_euclid(W),
                    y: H - 1,
                }
            } else {
                Self {
                    x: self.x,
                    y: self.y + 1,
                }
            }
        } else {
            if self.y == 0 {
                Self {
                    x: (self.x + W / 2).rem_euclid(W),
                    y: 0,
                }
            } else {
                Self {
                    x: self.x,
                    y: self.y - 1,
                }
            }
        }
    }

    fn down(&self) -> Self {
        if self.x <= W / 2 {
            if self.y == H - 1 {
                Self {
                    x: (self.x + W / 2).rem_euclid(W),
                    y: H - 1,
                }
            } else {
                Self {
                    x: self.x,
                    y: self.y + 1,
                }
            }
        } else {
            if self.y == 0 {
                Self {
                    x: (self.x + W / 2).rem_euclid(W),
                    y: 0,
                }
            } else {
                Self {
                    x: self.x,
                    y: self.y - 1,
                }
            }
        }
    }

    fn left(&self) -> Self {
        Self {
            x: (self.x - 1).rem_euclid(W),
            y: self.y
        }
    }

    fn right(&self) -> Self {
        Self {
            x: (self.x + 1).rem_euclid(W),
            y: self.y
        }
    }

    fn position(&self, scale: f64) -> (f64, f64, f64) {
        let (long, lat) = self.sphere_coordinates();

        let y = scale * lat.cos();
        let radius = scale * lat.cos();

        let x = radius * long.sin();
        let z = radius * long.cos();

        (x, y, z)
    }
}

impl <const W: usize, const H: usize> SpherePoint for RectangleSpherePoint<W, H> {
    fn from_geographic(latitude: f64, longitude: f64) -> Self {
        let x = ((longitude / (PI * 2.0) * W as f64) as isize).rem_euclid(W as isize) as usize;
        let y = (latitude + PI / 2.0) / PI;

        let y = ((2 * (y.ceil() as isize).rem_euclid(2) - 1)
            * ((y * H as f64) as isize).rem_euclid(H as isize)
            + H as isize * (y.floor() as isize).rem_euclid(2)) as usize;

        Self {
            x, y
        }
    }
    
    fn latitude(&self) -> f64 {
        self.y as f64 / H as f64 * PI - PI / 2.0
    }

    fn longitude(&self) -> f64 {
        self.x as f64 / W as f64 * PI * 2.0
    }
}

