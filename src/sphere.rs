//! A module containing grids wrapped around spheres.

use std::{f64::consts::PI, ops::{Index, IndexMut}, vec, fmt::Debug};

use itertools::Itertools;
use rayon::prelude::*;
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
///
/// # Constant Parameters
/// - `W` - The width of the grid.
/// - `H` - The height of the grid.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct RectangleSphereGrid<T, const W: usize, const H: usize> {
    /// The data held in this grid.
    data: HeapArray2D<T, W, H>,
}

impl <T, const W: usize, const H: usize> SurfaceGrid<T> for RectangleSphereGrid<T, W, H> {
    type Point = RectangleSpherePoint<W, H>;

    fn from_fn<F: FnMut(&Self::Point) -> T>(mut f: F) -> Self {
        Self {
            data: HeapArray2D::from_fn(|y, x| {
                let point = RectangleSpherePoint::new(x as u32, y as u32);

                f(&point)
            })
        }
    }

    fn from_fn_par<F: Fn(&Self::Point) -> T + Send + Sync>(f: F) -> Self where T: Send + Sync {
        Self {
            data: HeapArray2D::from_fn_par(|y, x| {
                let point = RectangleSpherePoint::new(x as u32, y as u32);

                f(&point)
            })
        }
    }

    fn set_from_fn<F: FnMut(&Self::Point) -> T>(&mut self, mut f: F) {
        (0..H).cartesian_product(0..W)
            .map(|(y, x)| RectangleSpherePoint::new(x as u32, y as u32))
            .for_each(|point| self[point] = f(&point))
    }

    fn set_from_fn_par<F: Fn(&Self::Point) -> T + Send + Sync>(&mut self, f: F) where T: Send + Sync {
        self.data.iter_mut().enumerate().par_bridge().for_each(|(y, subarray)| {
            for x in 0..W {
                let point = RectangleSpherePoint::new(x as u32, y as u32);

                subarray[x] = f(&point);
            }
        })
    }

    fn iter<'a>(&'a self) -> impl Iterator<Item = (RectangleSpherePoint<W, H>, &'a T)> where T: 'a {
        (0..H).cartesian_product(0..W)
            .map(|(y, x)| (RectangleSpherePoint::new(x as u32, y as u32), &self.data[y][x]))
    }

    fn par_iter<'a>(&'a self) -> impl ParallelIterator<Item = (Self::Point, &'a T)> where T: 'a + Send + Sync {
        (0..H).cartesian_product(0..W)
            .par_bridge()
            .map(|(y, x)| (RectangleSpherePoint::new(x as u32, y as u32), &self.data[y][x]))
    }

    fn points(&self) -> impl Iterator<Item = Self::Point> {
        (0..H).cartesian_product(0..W)
            .map(|(y, x)| RectangleSpherePoint::new(x as u32, y as u32))
    }

    fn par_points(&self) -> impl ParallelIterator<Item = Self::Point> {
        (0..H).cartesian_product(0..W)
            .par_bridge()
            .map(|(y, x)| RectangleSpherePoint::new(x as u32, y as u32))
    }

    fn for_each(&mut self, mut f: impl FnMut(&mut T)) {
        (0..H).cartesian_product(0..W)
            .for_each(|(y, x)| f(&mut self.data[y][x]))
    }
}

impl <T, const W: usize, const H: usize> Index<RectangleSpherePoint<W, H>> for RectangleSphereGrid<T, W, H> {
    type Output = T;

    fn index(&self, index: RectangleSpherePoint<W, H>) -> &Self::Output {
        &self.data[index.y as usize][index.x as usize]
    }
}

impl <T, const W: usize, const H: usize> IndexMut<RectangleSpherePoint<W, H>> for RectangleSphereGrid<T, W, H> {
    fn index_mut(&mut self, index: RectangleSpherePoint<W, H>) -> &mut Self::Output {
        &mut self.data[index.y as usize][index.x as usize]
    }
}

impl <T, const W: usize, const H: usize> IntoIterator for RectangleSphereGrid<T, W, H> {
    type Item = (RectangleSpherePoint<W, H>, T);

    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let data: Vec<_> = self.data.into_iter()
            .enumerate()
            .flat_map(|(y, subarray)| subarray.into_iter()
                      .enumerate()
                      .map(move |(x, value)| (RectangleSpherePoint::new(x as u32, y as u32), value))
                      )
            .collect();

        data.into_iter()
    }
}

/// A point on a `RectangleSphereGrid`.
///
/// # Constant Parameters
/// - `W` - The width of the grid.
/// - `H` - The height of the grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RectangleSpherePoint<const W: usize, const H: usize> {
    /// The X position in the grid.
    x: u32,
    /// The Y position in the grid.
    y: u32,
}

impl <const W: usize, const H: usize> RectangleSpherePoint<W, H> {
    fn new(x: u32, y: u32) -> Self {
        let x = (x + y / H as u32).rem_euclid(W as u32);
        let y = y.rem_euclid(H as u32);

        Self {
            x,
            y
        }
    }
}

impl <const W: usize, const H: usize> GridPoint for RectangleSpherePoint<W, H> {
    fn up(&self) -> Self {
        if self.x >= W as u32 / 2 {
            if self.y == H as u32 - 1 {
                Self {
                    x: (self.x + W as u32 / 2).rem_euclid(W as u32),
                    y: H as u32 - 1,
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
                    x: (self.x + W as u32 / 2).rem_euclid(W as u32),
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
        if self.x < W as u32 / 2 {
            if self.y == H as u32 - 1 {
                Self {
                    x: (self.x + W as u32 / 2).rem_euclid(W as u32),
                    y: H as u32 - 1,
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
                    x: (self.x + W as u32 / 2).rem_euclid(W as u32),
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
            x: (self.x as i32 - 1).rem_euclid(W as i32) as u32,
            y: self.y
        }
    }

    fn right(&self) -> Self {
        Self {
            x: (self.x + 1).rem_euclid(W as u32),
            y: self.y
        }
    }

    fn position(&self, scale: f64) -> (f64, f64, f64) {
        let (long, lat) = self.sphere_coordinates();

        let y = scale * lat.sin();
        let radius = scale * lat.cos();

        let x = radius * long.sin();
        let z = radius * long.cos();

        (x, y, z)
    }
}

impl <const W: usize, const H: usize> SpherePoint for RectangleSpherePoint<W, H> {
    fn from_geographic(latitude: f64, longitude: f64) -> Self {
        let latitude = -latitude;

        let x = ((longitude / (PI * 2.0) * W as f64) as i32).rem_euclid(W as i32) as u32;
        let y = (latitude + PI / 2.0) / PI;

        let y = ((2 * (y.ceil() as i32).rem_euclid(2) - 1)
            * ((y * H as f64) as i32).rem_euclid(H as i32)
            + H as i32 * (y.floor() as i32).rem_euclid(2)) as u32;

        let y = if y == H as u32 {
            H as u32 - 1
        } else {
            y
        };

        Self {
            x, y
        }
    }
    
    fn latitude(&self) -> f64 {
        -(self.y as f64 / H as f64 * PI - PI / 2.0)
    }

    fn longitude(&self) -> f64 {
        self.x as f64 / W as f64 * PI * 2.0
    }
}

/// A grid that wraps a cube around a sphere in order to determine grid positions.
///
/// # Type Parameters.
/// - `T` - The type of element stored in each grid cell.
///
/// # Constant Parameters
/// - `S` - The size of each side of each face.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct CubeSphereGrid<T, const S: usize> {
    top: HeapArray2D<T, S, S>,
    left: HeapArray2D<T, S, S>,
    front: HeapArray2D<T, S, S>,
    right: HeapArray2D<T, S, S>,
    back: HeapArray2D<T, S, S>,
    bottom: HeapArray2D<T, S, S>,
}

impl <T: Debug, const S: usize> SurfaceGrid<T> for CubeSphereGrid<T, S> {
    type Point = CubeSpherePoint<S>;

    fn from_fn<F: FnMut(&Self::Point) -> T>(mut f: F) -> Self {
        Self {
            top: HeapArray2D::from_fn(|y, x| f(&CubeSpherePoint::new(CubeFace::Top, x as u16, y as u16))),
            left: HeapArray2D::from_fn(|y, x| f(&CubeSpherePoint::new(CubeFace::Left, x as u16, y as u16))),
            front: HeapArray2D::from_fn(|y, x| f(&CubeSpherePoint::new(CubeFace::Front, x as u16, y as u16))),
            right: HeapArray2D::from_fn(|y, x| f(&CubeSpherePoint::new(CubeFace::Right, x as u16, y as u16))),
            back: HeapArray2D::from_fn(|y, x| f(&CubeSpherePoint::new(CubeFace::Back, x as u16, y as u16))),
            bottom: HeapArray2D::from_fn(|y, x| f(&CubeSpherePoint::new(CubeFace::Bottom, x as u16, y as u16))),
        }
    }

    fn from_fn_par<F: Fn(&Self::Point) -> T + Send + Sync>(f: F) -> Self where T: Send + Sync {
        Self {
            top: HeapArray2D::from_fn_par(|y, x| f(&CubeSpherePoint::new(CubeFace::Top, x as u16, y as u16))),
            left: HeapArray2D::from_fn_par(|y, x| f(&CubeSpherePoint::new(CubeFace::Left, x as u16, y as u16))),
            front: HeapArray2D::from_fn_par(|y, x| f(&CubeSpherePoint::new(CubeFace::Front, x as u16, y as u16))),
            right: HeapArray2D::from_fn_par(|y, x| f(&CubeSpherePoint::new(CubeFace::Right, x as u16, y as u16))),
            back: HeapArray2D::from_fn_par(|y, x| f(&CubeSpherePoint::new(CubeFace::Back, x as u16, y as u16))),
            bottom: HeapArray2D::from_fn_par(|y, x| f(&CubeSpherePoint::new(CubeFace::Bottom, x as u16, y as u16))),
        }
    }

    fn set_from_fn<F: FnMut(&Self::Point) -> T>(&mut self, mut f: F) {
        [
            CubeFace::Top,
            CubeFace::Left,
            CubeFace::Front,
            CubeFace::Right,
            CubeFace::Back,
            CubeFace::Bottom,
        ].into_iter()
            .cartesian_product(0..S)
            .cartesian_product(0..S)
            .map(|((face, x), y)| CubeSpherePoint::new(face, x as u16, y as u16))
            .map(|point| (point, f(&point)))
            .for_each(|(point, value)| self[point] = value)
    }

    fn set_from_fn_par<F: Fn(&Self::Point) -> T + Send + Sync>(&mut self, f: F) where T: Send + Sync {
        for face in [
            CubeFace::Top,
            CubeFace::Left,
            CubeFace::Front,
            CubeFace::Right,
            CubeFace::Back,
            CubeFace::Bottom,
        ] {
            match face {
                CubeFace::Front => &mut self.front,
                CubeFace::Back => &mut self.back,
                CubeFace::Left => &mut self.left,
                CubeFace::Right => &mut self.right,
                CubeFace::Top => &mut self.top,
                CubeFace::Bottom => &mut self.bottom,
            }.iter_mut().enumerate().par_bridge().for_each(|(y, subarray)| for x in 0..S {
                let point = CubeSpherePoint::new(face, x as u16, y as u16);

                subarray[x] = f(&point);
            });
        }
    }

    fn iter<'a>(&'a self) -> impl Iterator<Item = (Self::Point, &'a T)> where T: 'a {
        self.points()
            .map(|point| (point, &self[point]))
    }

    fn par_iter<'a>(&'a self) -> impl ParallelIterator<Item = (Self::Point, &'a T)> where T: 'a + Send + Sync {
        self.par_points()
            .map(|point| (point, &self[point]))
    }

    fn points(&self) -> impl Iterator<Item = Self::Point> {
        [
            CubeFace::Top,
            CubeFace::Left,
            CubeFace::Front,
            CubeFace::Right,
            CubeFace::Back,
            CubeFace::Bottom,
        ].into_iter()
            .cartesian_product(0..S)
            .cartesian_product(0..S)
            .map(|((face, x), y)| CubeSpherePoint::new(face, x as u16, y as u16))
    }

    fn par_points(&self) -> impl ParallelIterator<Item = Self::Point> {
        [
            CubeFace::Top,
            CubeFace::Left,
            CubeFace::Front,
            CubeFace::Right,
            CubeFace::Back,
            CubeFace::Bottom,
        ].into_iter()
            .cartesian_product(0..S)
            .cartesian_product(0..S)
            .par_bridge()
            .map(|((face, x), y)| CubeSpherePoint::new(face, x as u16, y as u16))
    }

    fn for_each(&mut self, mut f: impl FnMut(&mut T)) {
        (0..S).cartesian_product(0..S)
            .for_each(|(x, y)| {
                f(&mut self.front[y][x]);
                f(&mut self.back[y][x]);
                f(&mut self.left[y][x]);
                f(&mut self.right[y][x]);
                f(&mut self.top[y][x]);
                f(&mut self.bottom[y][x]);
            })
    }
}

impl <T, const S: usize> Index<CubeSpherePoint<S>> for CubeSphereGrid<T, S> {
    type Output = T;

    fn index(&self, index: CubeSpherePoint<S>) -> &Self::Output {
        match index.face {
            CubeFace::Front => &self.front[index.y as usize][index.x as usize],
            CubeFace::Back => &self.back[index.y as usize][index.x as usize],
            CubeFace::Left => &self.left[index.y as usize][index.x as usize],
            CubeFace::Right => &self.right[index.y as usize][index.x as usize],
            CubeFace::Top => &self.top[index.y as usize][index.x as usize],
            CubeFace::Bottom => &self.bottom[index.y as usize][index.x as usize],
        }
    }
}

impl <T, const S: usize> IndexMut<CubeSpherePoint<S>> for CubeSphereGrid<T, S> {
    fn index_mut(&mut self, index: CubeSpherePoint<S>) -> &mut Self::Output {
        match index.face {
            CubeFace::Front => &mut self.front[index.y as usize][index.x as usize],
            CubeFace::Back => &mut self.back[index.y as usize][index.x as usize],
            CubeFace::Left => &mut self.left[index.y as usize][index.x as usize],
            CubeFace::Right => &mut self.right[index.y as usize][index.x as usize],
            CubeFace::Top => &mut self.top[index.y as usize][index.x as usize],
            CubeFace::Bottom => &mut self.bottom[index.y as usize][index.x as usize],
        }
    }
}

impl <T, const S: usize> IntoIterator for CubeSphereGrid<T, S> {
    type Item = (CubeSpherePoint<S>, T);

    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut data: Vec<_> = self.top.into_iter()
            .enumerate()
            .flat_map(|(y, subarray)| subarray.into_iter()
                        .enumerate()
                        .map(move |(x, value)| (CubeSpherePoint::new(CubeFace::Top, x as u16, y as u16), value))
                      )
            .collect();

        data.extend(self.left.into_iter()
                    .enumerate()
                    .flat_map(|(y, subarray)| subarray.into_iter()
                              .enumerate()
                              .map(move |(x, value)| (CubeSpherePoint::new(CubeFace::Left, x as u16, y as u16), value))
                              ));
        data.extend(self.front.into_iter()
                    .enumerate()
                    .flat_map(|(y, subarray)| subarray.into_iter()
                              .enumerate()
                              .map(move |(x, value)| (CubeSpherePoint::new(CubeFace::Front, x as u16, y as u16), value))
                              ));
        data.extend(self.right.into_iter()
                    .enumerate()
                    .flat_map(|(y, subarray)| subarray.into_iter()
                              .enumerate()
                              .map(move |(x, value)| (CubeSpherePoint::new(CubeFace::Right, x as u16, y as u16), value))
                              ));
        data.extend(self.back.into_iter()
                    .enumerate()
                    .flat_map(|(y, subarray)| subarray.into_iter()
                              .enumerate()
                              .map(move |(x, value)| (CubeSpherePoint::new(CubeFace::Back, x as u16, y as u16), value))
                              ));
        data.extend(self.bottom.into_iter()
                    .enumerate()
                    .flat_map(|(y, subarray)| subarray.into_iter()
                              .enumerate()
                              .map(move |(x, value)| (CubeSpherePoint::new(CubeFace::Bottom, x as u16, y as u16), value))
                              ));

        data.into_iter()
    }
}

/// A point on a `CubeSphereGrid`.
///
/// # Constant Parameters
/// - `S` - The size of each side of each face.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CubeSpherePoint<const S: usize> {
    face: CubeFace,
    x: u16,
    y: u16,
}

impl <const S: usize> CubeSpherePoint<S> {
    /// Creates a new `CubeSpherePoint`.
    ///
    /// - `face` - The face on which the point lies.
    /// - `x` - The X position on the face.
    /// - `y` - The Y position on the face.
    fn new(face: CubeFace, x: u16, y: u16) -> Self {
        Self {
            face,
            // Clamp to account for floating point rounding error.
            x: x.clamp(0, S as u16 - 1),
            y: y.clamp(0, S as u16 - 1)
        }
    }
}

impl <const S: usize> GridPoint for CubeSpherePoint<S> {
    fn up(&self) -> Self {
        match self.face {
            CubeFace::Front => if self.y == 0 {
                Self {
                    face: CubeFace::Top,
                    x: self.x,
                    y: S as u16 - 1,
                }
            } else {
                Self {
                    face: CubeFace::Front,
                    x: self.x,
                    y: self.y - 1,
                }
            },
            CubeFace::Back => if self.y == 0 {
                Self {
                    face: CubeFace::Bottom,
                    x: self.x,
                    y: S as u16 - 1,
                }
            } else {
                Self {
                    face: CubeFace::Back,
                    x: self.x,
                    y: self.y - 1,
                }
            },
            CubeFace::Left => if self.y == 0 {
                Self {
                    face: CubeFace::Top,
                    x: 0,
                    y: self.x
                }
            } else {
                Self {
                    face: CubeFace::Left,
                    x: self.x,
                    y: self.y - 1,
                }
            },
            CubeFace::Right => if self.y == 0 {
                Self {
                    face: CubeFace::Top,
                    x: S as u16 - 1,
                    y: self.x
                }
            } else {
                Self {
                    face: CubeFace::Right,
                    x: self.x,
                    y: self.y - 1,
                }
            },
            CubeFace::Top => if self.y == 0 {
                Self {
                    face: CubeFace::Back,
                    x: self.x,
                    y: S as u16 - 1,
                }
            } else {
                Self {
                    face: CubeFace::Top,
                    x: self.x,
                    y: self.y - 1,
                }
            },
            CubeFace::Bottom => if self.y == 0 {
                Self {
                    face: CubeFace::Front,
                    x: self.x,
                    y: S as u16 - 1,
                }
            } else {
                Self {
                    face: CubeFace::Bottom,
                    x: self.x,
                    y: self.y - 1,
                }
            },
        }
    }

    fn down(&self) -> Self {
        match self.face {
            CubeFace::Front => if self.y == S as u16 - 1 {
                Self {
                    face: CubeFace::Bottom,
                    x: self.x,
                    y: 0,
                }
            } else {
                Self {
                    face: CubeFace::Front,
                    x: self.x,
                    y: self.y + 1,
                }
            },
            CubeFace::Back => if self.y == S as u16 - 1 {
                Self {
                    face: CubeFace::Top,
                    x: self.x,
                    y: 0,
                }
            } else {
                Self {
                    face: CubeFace::Back,
                    x: self.x,
                    y: self.y + 1,
                }
            },
            CubeFace::Left => if self.y == S as u16 - 1 {
                Self {
                    face: CubeFace::Bottom,
                    x: 0,
                    y: self.x
                }
            } else {
                Self {
                    face: CubeFace::Left,
                    x: self.x,
                    y: self.y + 1,
                }
            },
            CubeFace::Right => if self.y == S as u16 - 1 {
                Self {
                    face: CubeFace::Bottom,
                    x: 0,
                    y: self.x
                }
            } else {
                Self {
                    face: CubeFace::Right,
                    x: self.x,
                    y: self.y + 1,
                }
            },
            CubeFace::Top => if self.y == S as u16 - 1 {
                Self {
                    face: CubeFace::Front,
                    x: self.x,
                    y: 0,
                }
            } else {
                Self {
                    face: CubeFace::Top,
                    x: self.x,
                    y: self.y + 1,
                }
            },
            CubeFace::Bottom => if self.y == S as u16 - 1 {
                Self {
                    face: CubeFace::Back,
                    x: self.x,
                    y: 0,
                }
            } else {
                Self {
                    face: CubeFace::Bottom,
                    x: self.x,
                    y: self.y + 1,
                }
            },
        }
    }

    fn left(&self) -> Self {
        match self.face {
            CubeFace::Front => if self.x == 0 {
                Self {
                    face: CubeFace::Left,
                    x: S as u16 - 1,
                    y: self.y
                }
            } else {
                Self {
                    face: CubeFace::Front,
                    x: self.x - 1,
                    y: self.y
                }
            },
            CubeFace::Back => if self.x == S as u16 - 1 {
                Self {
                    face: CubeFace::Right,
                    x: S as u16 - 1,
                    y: self.y
                }
            } else {
                Self {
                    face: CubeFace::Back,
                    x: self.x + 1,
                    y: self.y
                }
            },
            CubeFace::Left => if self.x == 0 {
                Self {
                    face: CubeFace::Back,
                    x: 0,
                    y: self.y,
                }
            } else {
                Self {
                    face: CubeFace::Left,
                    x: self.x - 1,
                    y: self.y
                }
            },
            CubeFace::Right => if self.x == 0 {
                Self {
                    face: CubeFace::Front,
                    x: S as u16 - 1,
                    y: self.y
                }
            } else {
                Self {
                    face: CubeFace::Right,
                    x: self.x - 1,
                    y: self.y,
                }
            },
            CubeFace::Top => if self.x == 0 {
                Self {
                    face: CubeFace::Left,
                    x: self.y,
                    y: 0,
                }
            } else {
                Self {
                    face: CubeFace::Top,
                    x: self.x - 1,
                    y: self.y
                }
            },
            CubeFace::Bottom => if self.x == 0 {
                Self {
                    face: CubeFace::Left,
                    x: self.y,
                    y: S as u16 - 1,
                }
            } else {
                Self {
                    face: CubeFace::Bottom,
                    x: self.x - 1,
                    y: self.y
                }
            },
        }
    }

    fn right(&self) -> Self {
        match self.face {
            CubeFace::Front => if self.x == S as u16 - 1 {
                Self {
                    face: CubeFace::Right,
                    x: 0,
                    y: self.y
                }
            } else {
                Self {
                    face: CubeFace::Front,
                    x: self.x + 1,
                    y: self.y
                }
            },
            CubeFace::Back => if self.x == 0 {
                Self {
                    face: CubeFace::Left,
                    x: 0,
                    y: self.y
                }
            } else {
                Self {
                    face: CubeFace::Back,
                    x: self.x - 1,
                    y: self.y
                }
            },
            CubeFace::Left => if self.x == S as u16 - 1 {
                Self {
                    face: CubeFace::Front,
                    x: 0,
                    y: self.y,
                }
            } else {
                Self {
                    face: CubeFace::Left,
                    x: self.x + 1,
                    y: self.y
                }
            },
            CubeFace::Right => if self.x == S as u16 - 1 {
                Self {
                    face: CubeFace::Back,
                    x: S as u16 - 1,
                    y: self.y
                }
            } else {
                Self {
                    face: CubeFace::Right,
                    x: self.x + 1,
                    y: self.y,
                }
            },
            CubeFace::Top => if self.x == S as u16 - 1{
                Self {
                    face: CubeFace::Right,
                    x: self.y,
                    y: 0,
                }
            } else {
                Self {
                    face: CubeFace::Top,
                    x: self.x + 1,
                    y: self.y
                }
            },
            CubeFace::Bottom => if self.x == S as u16 - 1 {
                Self {
                    face: CubeFace::Right,
                    x: self.y,
                    y: S as u16 - 1,
                }
            } else {
                Self {
                    face: CubeFace::Bottom,
                    x: self.x + 1,
                    y: self.y
                }
            },
        }
    }

    fn position(&self, scale: f64) -> (f64, f64, f64) {
        let x = self.x as f64;
        let y = self.y as f64;

        let (x, y, z) = match self.face {
            CubeFace::Front => (x * 2.0 - S as f64, y * 2.0 - S as f64, S as f64),
            CubeFace::Back => (x * 2.0 - S as f64, -y * 2.0 + S as f64, -(S as f64)),
            CubeFace::Left => (-(S as f64), y * 2.0 - S as f64, x * 2.0 -(S as f64)),
            CubeFace::Right => (S as f64, y * 2.0 - S as f64, S as f64 - x * 2.0),
            CubeFace::Top => (x * 2.0 - S as f64, S as f64, y * 2.0 - S as f64),
            CubeFace::Bottom => (x * 2.0 - S as f64, -(S as f64), S as f64 - y * 2.0),
        };

        let length = (x * x + y * y + z * z).sqrt();

        (x / length * scale, y / length * scale, z / length * scale)
    }
}

impl <const S: usize> SpherePoint for CubeSpherePoint<S> {
    fn from_geographic(latitude: f64, longitude: f64) -> Self {
        let y = latitude.sin();

        let radius = latitude.cos();

        let x = radius * longitude.sin();
        let z = radius * longitude.cos();

        let longitude = longitude.rem_euclid(2.0 * PI);

        let face = if y > 0.0 {
            let scale = S as f64 / y;

            let z = z * scale;
            let x = x * scale;
            
            let x2 = (x + S as f64) / 2.0;
            let y2 = (z + S as f64) / 2.0;

            if (x2 as i32) >= 0 && (x2 as i32) < (S as i32) && (y2 as i32) > 0 && (y2 as i32) < (S as i32) {
                return CubeSpherePoint::new(CubeFace::Top, x2 as u16, y2 as u16);
            }
                
            if longitude > PI / 4.0 + 3.0 * PI / 2.0 {
                CubeFace::Front
            } else if longitude > PI / 4.0 + 2.0 * PI / 2.0 {
                CubeFace::Left
            } else if longitude > PI / 4.0 + PI / 2.0 {
                CubeFace::Back
            } else if longitude > PI / 4.0 {
                CubeFace::Right
            } else {
                CubeFace::Front
            }
        } else {
            let scale = -(S as f64) / y;

            let z = z * scale;
            let x = x * scale;
            
            let x2 = (x + S as f64) / 2.0;
            let y2 = (S as f64 - z) / 2.0;

            if (x2 as i32) >= 0 && (x2 as i32) < (S as i32) && (y2 as i32) > 0 && (y2 as i32) < (S as i32) {
                return CubeSpherePoint::new(CubeFace::Bottom, x2 as u16, y2 as u16);
            }
            if longitude > PI / 4.0 + 3.0 * PI / 2.0 {
                CubeFace::Front
            } else if longitude > PI / 4.0 + 2.0 * PI / 2.0 {
                CubeFace::Left
            } else if longitude > PI / 4.0 + PI / 2.0 {
                CubeFace::Back
            } else if longitude > PI / 4.0 {
                CubeFace::Right
            } else {
                CubeFace::Front
            }
        };

        match face {
            CubeFace::Front => {
                let scale = S as f64 / z;

                let x2 = (x * scale + S as f64) / 2.0;
                let y2 = (y * scale + S as f64) / 2.0;

                CubeSpherePoint::new(CubeFace::Front, x2 as u16, y2 as u16)
            },
            CubeFace::Back => {
                let scale = -(S as f64) / z;

                let x = x * scale;
                let y = y * scale;
                
                let x2 = (x + S as f64) / 2.0;
                let y2 = (y - S as f64) / -2.0;
                
                CubeSpherePoint::new(CubeFace::Back, x2 as u16, y2 as u16)
            },
            CubeFace::Left => {
                let scale = -(S as f64) / x;

                let z = z * scale;
                let y = y * scale;

                let x2 = (z + S as f64) / 2.0;
                let y2 = (y + S as f64) / 2.0;
                
                CubeSpherePoint::new(CubeFace::Left, x2 as u16, y2 as u16)
            },
            CubeFace::Right => {
                let scale = S as f64 / x;

                let z = z * scale;
                let y = y * scale;
                
                let x2 = (S as f64 - z) / 2.0;
                let y2 = (y + S as f64) / 2.0;
                
                CubeSpherePoint::new(CubeFace::Right, x2 as u16, y2 as u16)
            },
            CubeFace::Top => {
                let scale = S as f64 / y;

                let z = z * scale;
                let x = x * scale;
                
                let x2 = (x + S as f64) / 2.0;
                let y2 = (z + S as f64) / 2.0;
                
                CubeSpherePoint::new(CubeFace::Top, x2 as u16, y2 as u16)
            },
            CubeFace::Bottom => {
                let scale = -(S as f64) / y;

                let z = z * scale;
                let x = x * scale;
                
                let x2 = (x + S as f64) / 2.0;
                let y2 = (S as f64 - z) / 2.0;
                
                CubeSpherePoint::new(CubeFace::Bottom, x2 as u16, y2 as u16)
            },
        }
    }

    fn latitude(&self) -> f64 {
        let (x, y, z) = self.position(1.0);

        let distance = (x * x + z * z).sqrt();

        (y / distance).atan()
    }

    fn longitude(&self) -> f64 {
        let (x, _, z) = self.position(1.0);

        x.atan2(z).rem_euclid(2.0 * PI)
    }
}

/// A face of a cube.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)] // For better alignment.
enum CubeFace {
    Front,
    Back,
    Left,
    Right,
    Top,
    Bottom,
}

#[cfg(test)]
mod test {
    use std::{f64::consts::PI, hint::black_box};

    use approx::assert_relative_eq;

    use crate::{GridPoint, SurfaceGrid, sphere::{CubeSpherePoint, CubeFace, CubeSphereGrid}};

    use super::{RectangleSpherePoint, SpherePoint, RectangleSphereGrid};

    #[test]
    fn test_rect_point_up_middle() {
        let point: RectangleSpherePoint<10, 10> = RectangleSpherePoint::new(3, 4);

        assert_eq!(RectangleSpherePoint::new(3, 3), point.up());
    }
    
    #[test]
    fn test_rect_point_up_top_left() {
        let point: RectangleSpherePoint<10, 10> = RectangleSpherePoint::new(0, 0);

        assert_eq!(RectangleSpherePoint::new(5, 0), point.up());
    }
    
    #[test]
    fn test_rect_point_up_top_right() {
        let point: RectangleSpherePoint<10, 10> = RectangleSpherePoint::new(9, 0);

        assert_eq!(RectangleSpherePoint::new(9, 1), point.up());
    }
    
    #[test]
    fn test_rect_point_up_bottom_left() {
        let point: RectangleSpherePoint<10, 10> = RectangleSpherePoint::new(0, 9);

        assert_eq!(RectangleSpherePoint::new(0, 8), point.up());
    }
    
    #[test]
    fn test_rect_point_up_bottom_right() {
        let point: RectangleSpherePoint<10, 10> = RectangleSpherePoint::new(9, 9);

        assert_eq!(RectangleSpherePoint::new(4, 9), point.up());
    }
    
    #[test]
    fn test_rect_point_down_middle() {
        let point: RectangleSpherePoint<10, 10> = RectangleSpherePoint::new(3, 4);

        assert_eq!(RectangleSpherePoint::new(3, 5), point.down());
    }
    
    #[test]
    fn test_rect_point_down_top_left() {
        let point: RectangleSpherePoint<10, 10> = RectangleSpherePoint::new(0, 0);

        assert_eq!(RectangleSpherePoint::new(0, 1), point.down());
    }
    
    #[test]
    fn test_rect_point_down_top_right() {
        let point: RectangleSpherePoint<10, 10> = RectangleSpherePoint::new(9, 0);

        assert_eq!(RectangleSpherePoint::new(4, 0), point.down());
    }
    
    #[test]
    fn test_rect_point_down_bottom_left() {
        let point: RectangleSpherePoint<10, 10> = RectangleSpherePoint::new(0, 9);

        assert_eq!(RectangleSpherePoint::new(5, 9), point.down());
    }
    
    #[test]
    fn test_rect_point_down_bottom_right() {
        let point: RectangleSpherePoint<10, 10> = RectangleSpherePoint::new(9, 9);

        assert_eq!(RectangleSpherePoint::new(9, 8), point.down());
    }

    #[test]
    fn test_rect_point_left_middle() {
        let point: RectangleSpherePoint<10, 10> = RectangleSpherePoint::new(5, 5);

        assert_eq!(RectangleSpherePoint::new(4, 5), point.left());
    }
    
    #[test]
    fn test_rect_point_left_left() {
        let point: RectangleSpherePoint<10, 10> = RectangleSpherePoint::new(0, 5);

        assert_eq!(RectangleSpherePoint::new(9, 5), point.left());
    }
   
    #[test]
    fn test_rect_point_left_right() {
        let point: RectangleSpherePoint<10, 10> = RectangleSpherePoint::new(9, 5);

        assert_eq!(RectangleSpherePoint::new(8, 5), point.left());
    }

    #[test]
    fn test_rect_point_right_middle() {
        let point: RectangleSpherePoint<10, 10> = RectangleSpherePoint::new(5, 5);

        assert_eq!(RectangleSpherePoint::new(6, 5), point.right());
    }
    
    #[test]
    fn test_rect_point_right_left() {
        let point: RectangleSpherePoint<10, 10> = RectangleSpherePoint::new(0, 5);

        assert_eq!(RectangleSpherePoint::new(1, 5), point.right());
    }
   
    #[test]
    fn test_rect_point_right_right() {
        let point: RectangleSpherePoint<10, 10> = RectangleSpherePoint::new(9, 5);

        assert_eq!(RectangleSpherePoint::new(0, 5), point.right());
    }

    #[test]
    fn test_rect_point_from_geographic_equator() {
        let point: RectangleSpherePoint<100, 100> = RectangleSpherePoint::from_geographic(0.0, PI);

        assert_eq!(RectangleSpherePoint::new(50, 50), point);
    }
    
    #[test]
    fn test_rect_point_from_geographic_north_pole() {
        let point: RectangleSpherePoint<100, 100> = RectangleSpherePoint::from_geographic(PI / 2.0, PI);

        assert_eq!(RectangleSpherePoint::new(50, 0), point);
    }
    
    #[test]
    fn test_rect_point_from_geographic_south_pole() {
        let point: RectangleSpherePoint<100, 100> = RectangleSpherePoint::from_geographic(-PI / 2.0, PI);

        assert_eq!(RectangleSpherePoint::new(50, 99), point);
    }
    
    #[test]
    fn test_rect_point_from_geographic_equator_wrap_north() {
        let point: RectangleSpherePoint<100, 100> = RectangleSpherePoint::from_geographic(-PI, PI);

        assert_eq!(RectangleSpherePoint::new(50, 50), point);
    }
    
    #[test]
    fn test_rect_point_from_geographic_equator_wrap_south() {
        let point: RectangleSpherePoint<100, 100> = RectangleSpherePoint::from_geographic(PI, PI);

        assert_eq!(RectangleSpherePoint::new(50, 50), point);
    }
    
    #[test]
    fn test_rect_point_from_geographic_east() {
        let point: RectangleSpherePoint<100, 100> = RectangleSpherePoint::from_geographic(0.0, PI * 2.0);

        assert_eq!(RectangleSpherePoint::new(0, 50), point);
    }
    
    #[test]
    fn test_rect_point_from_geographic_west() {
        let point: RectangleSpherePoint<100, 100> = RectangleSpherePoint::from_geographic(0.0, 0.0);

        assert_eq!(RectangleSpherePoint::new(0, 50), point);
    }

    #[test]
    fn test_rect_point_up_loop() {
        let start: RectangleSpherePoint<10, 5> = RectangleSpherePoint::new(0, 3);

        assert_eq!(start, start.up().up().up().up().up().up().up().up().up().up());
    }
    
    #[test]
    fn test_rect_point_down_loop() {
        let start: RectangleSpherePoint<10, 5> = RectangleSpherePoint::new(5, 3);

        assert_eq!(start, start.down().down().down().down().down().down().down().down().down().down());
    }
    
    #[test]
    fn test_rect_point_left_loop() {
        let start: RectangleSpherePoint<10, 5> = RectangleSpherePoint::new(0, 3);

        assert_eq!(start, start.left().left().left().left().left().left().left().left().left().left());
    }
    
    #[test]
    fn test_rect_point_right_loop() {
        let start: RectangleSpherePoint<10, 5> = RectangleSpherePoint::new(0, 3);

        assert_eq!(start, start.right().right().right().right().right().right().right().right().right().right());
    }

    #[test]
    fn test_rect_point_up_inverse_middle() {
        let start: RectangleSpherePoint<10, 5> = RectangleSpherePoint::new(5, 3);

        assert_eq!(start, start.up().down());
    }
    
    #[test]
    fn test_rect_point_down_inverse_middle() {
        let start: RectangleSpherePoint<10, 5> = RectangleSpherePoint::new(5, 3);

        assert_eq!(start, start.down().up());
    }
    
    #[test]
    fn test_rect_point_left_inverse_middle() {
        let start: RectangleSpherePoint<10, 5> = RectangleSpherePoint::new(5, 3);

        assert_eq!(start, start.left().right());
    }
    
    #[test]
    fn test_rect_point_right_inverse_middle() {
        let start: RectangleSpherePoint<10, 5> = RectangleSpherePoint::new(5, 3);

        assert_eq!(start, start.right().left());
    }
    
    #[test]
    fn test_rect_point_up_inverse_edge() {
        let start: RectangleSpherePoint<10, 5> = RectangleSpherePoint::new(0, 0);

        assert_eq!(start, start.up().down());
    }
    
    #[test]
    fn test_rect_point_down_inverse_edge() {
        let start: RectangleSpherePoint<10, 5> = RectangleSpherePoint::new(0, 4);

        assert_eq!(start, start.down().up());
    }
    
    #[test]
    fn test_rect_point_left_inverse_edge() {
        let start: RectangleSpherePoint<10, 5> = RectangleSpherePoint::new(0, 0);

        assert_eq!(start, start.left().right());
    }
    
    #[test]
    fn test_rect_point_right_inverse_edge() {
        let start: RectangleSpherePoint<10, 5> = RectangleSpherePoint::new(9, 0);

        assert_eq!(start, start.right().left());
    }

    #[test]
    fn test_rect_from_fn() {
        let grid: RectangleSphereGrid<u32, 200, 100> = RectangleSphereGrid::from_fn(|point| point.x + point.y);

        assert_eq!(15, grid[RectangleSpherePoint::new(5, 10)]);
    }

    #[test]
    fn test_rect_from_neighbours() {
        let grid: RectangleSphereGrid<u32, 20, 10> = RectangleSphereGrid::from_fn(|point| point.x);

        let grid2 = grid.map_neighbours(|current, up, down, left, right| current + up + down + left + right);

        assert_eq!(25, grid2[RectangleSpherePoint::new(5, 3)])
    }
    
    #[test]
    fn test_rect_from_neighbours_diagonals() {
        let grid: RectangleSphereGrid<u32, 20, 10> = RectangleSphereGrid::from_fn(|point| point.x);

        let grid2 = grid.map_neighbours_diagonals(|up_left, up, up_right, left, current, right, down_left, down, down_right| up_left + up + up_right + left + current + right + down_left + down + down_right);

        assert_eq!(4 * 3 + 5 * 3 + 6 * 3, grid2[RectangleSpherePoint::new(5, 3)])
    }

    #[test]
    fn test_rect_point_latitude_0() {
        let point: RectangleSpherePoint<1000000000, 500000000> = RectangleSpherePoint::from_geographic(0.0, 0.0);

        assert_relative_eq!(0.0, point.latitude());
    }
    
    #[test]
    fn test_rect_point_latitude_1() {
        let point: RectangleSpherePoint<1000000000, 500000000> = RectangleSpherePoint::from_geographic(1.0, 0.0);

        assert_relative_eq!(1.0, point.latitude(), epsilon = 0.001);
    }
    
    #[test]
    fn test_rect_point_latitude_minus_1() {
        let point: RectangleSpherePoint<1000000000, 500000000> = RectangleSpherePoint::from_geographic(-1.0, 0.0);

        assert_relative_eq!(-1.0, point.latitude(), epsilon = 0.001);
    }
    
    #[test]
    fn test_rect_point_latitude_0_far() {
        let point: RectangleSpherePoint<1000000000, 500000000> = RectangleSpherePoint::from_geographic(0.0, 4.0);

        assert_relative_eq!(0.0, point.latitude());
    }
    
    #[test]
    fn test_rect_point_latitude_1_far() {
        let point: RectangleSpherePoint<1000000000, 500000000> = RectangleSpherePoint::from_geographic(1.0, 2.0);

        assert_relative_eq!(1.0, point.latitude(), epsilon = 0.001);
    }
    
    #[test]
    fn test_rect_point_latitude_minus_1_far() {
        let point: RectangleSpherePoint<1000000000, 500000000> = RectangleSpherePoint::from_geographic(-1.0, 3.0);

        assert_relative_eq!(-1.0, point.latitude(), epsilon = 0.001);
    }
    
    #[test]
    fn test_rect_point_longitude_0() {
        let point: RectangleSpherePoint<1000000000, 500000000> = RectangleSpherePoint::from_geographic(0.0, 0.0);

        assert_relative_eq!(0.0, point.longitude(), epsilon = 0.001);
    }
    
    #[test]
    fn test_rect_point_longitude_1() {
        let point: RectangleSpherePoint<1000000000, 500000000> = RectangleSpherePoint::from_geographic(0.0, 1.0);

        assert_relative_eq!(1.0, point.longitude(), epsilon = 0.001);
    }
    
    #[test]
    fn test_rect_point_longitude_2() {
        let point: RectangleSpherePoint<1000000000, 500000000> = RectangleSpherePoint::from_geographic(0.0, 2.0);

        assert_relative_eq!(2.0, point.longitude(), epsilon = 0.001);
    }
    
    #[test]
    fn test_rect_point_longitude_4() {
        let point: RectangleSpherePoint<1000000000, 500000000> = RectangleSpherePoint::from_geographic(0.0, 4.0);

        assert_relative_eq!(4.0, point.longitude(), epsilon = 0.001);
    }
    
    #[test]
    fn test_rect_point_longitude_6() {
        let point: RectangleSpherePoint<1000000000, 500000000> = RectangleSpherePoint::from_geographic(0.0, 6.0);

        assert_relative_eq!(6.0, point.longitude(), epsilon = 0.001);
    }
    
    #[test]
    fn test_rect_point_longitude_0_north() {
        let point: RectangleSpherePoint<1000000000, 500000000> = RectangleSpherePoint::from_geographic(-1.5, 0.0);

        assert_relative_eq!(0.0, point.longitude(), epsilon = 0.001);
    }
    
    #[test]
    fn test_rect_point_longitude_1_north() {
        let point: RectangleSpherePoint<1000000000, 500000000> = RectangleSpherePoint::from_geographic(-1.5, 1.0);

        assert_relative_eq!(1.0, point.longitude(), epsilon = 0.001);
    }
    
    #[test]
    fn test_rect_point_longitude_2_north() {
        let point: RectangleSpherePoint<1000000000, 500000000> = RectangleSpherePoint::from_geographic(-1.5, 2.0);

        assert_relative_eq!(2.0, point.longitude(), epsilon = 0.001);
    }
    
    #[test]
    fn test_rect_point_longitude_4_north() {
        let point: RectangleSpherePoint<1000000000, 500000000> = RectangleSpherePoint::from_geographic(-1.5, 4.0);

        assert_relative_eq!(4.0, point.longitude(), epsilon = 0.001);
    }
    
    #[test]
    fn test_rect_point_longitude_6_north() {
        let point: RectangleSpherePoint<1000000000, 500000000> = RectangleSpherePoint::from_geographic(-1.5, 6.0);

        assert_relative_eq!(6.0, point.longitude(), epsilon = 0.001);
    }
    
    #[test]
    fn test_rect_point_longitude_0_south() {
        let point: RectangleSpherePoint<1000000000, 500000000> = RectangleSpherePoint::from_geographic(1.5, 0.0);

        assert_relative_eq!(0.0, point.longitude(), epsilon = 0.001);
    }
    
    #[test]
    fn test_rect_point_longitude_1_south() {
        let point: RectangleSpherePoint<1000000000, 500000000> = RectangleSpherePoint::from_geographic(1.5, 1.0);

        assert_relative_eq!(1.0, point.longitude(), epsilon = 0.001);
    }
    
    #[test]
    fn test_rect_point_longitude_2_south() {
        let point: RectangleSpherePoint<1000000000, 500000000> = RectangleSpherePoint::from_geographic(1.5, 2.0);

        assert_relative_eq!(2.0, point.longitude(), epsilon = 0.001);
    }
    
    #[test]
    fn test_rect_point_longitude_4_south() {
        let point: RectangleSpherePoint<1000000000, 500000000> = RectangleSpherePoint::from_geographic(1.5, 4.0);

        assert_relative_eq!(4.0, point.longitude(), epsilon = 0.001);
    }
    
    #[test]
    fn test_rect_point_longitude_6_south() {
        let point: RectangleSpherePoint<1000000000, 500000000> = RectangleSpherePoint::from_geographic(1.5, 6.0);

        assert_relative_eq!(6.0, point.longitude(), epsilon = 0.001);
    }

    #[test]
    fn test_cube_point_up_middle() {
        let point: CubeSpherePoint<5> = CubeSpherePoint::new(CubeFace::Front, 3, 4);

        assert_eq!(CubeSpherePoint::new(CubeFace::Front, 3, 3), point.up());
    }
    
    #[test]
    fn test_cube_point_up_top() {
        let point: CubeSpherePoint<5> = CubeSpherePoint::new(CubeFace::Front, 0, 0);

        assert_eq!(CubeSpherePoint::new(CubeFace::Top, 0, 4), point.up());
    }
    
    #[test]
    fn test_cube_point_up_bottom() {
        let point: CubeSpherePoint<5> = CubeSpherePoint::new(CubeFace::Front, 0, 4);

        assert_eq!(CubeSpherePoint::new(CubeFace::Front, 0, 3), point.up());
    }
    
    #[test]
    fn test_cube_point_down_middle() {
        let point: CubeSpherePoint<10> = CubeSpherePoint::new(CubeFace::Front, 3, 4);

        assert_eq!(CubeSpherePoint::new(CubeFace::Front, 3, 5), point.down());
    }
    
    #[test]
    fn test_cube_point_down_top() {
        let point: CubeSpherePoint<10> = CubeSpherePoint::new(CubeFace::Front, 0, 0);

        assert_eq!(CubeSpherePoint::new(CubeFace::Front, 0, 1), point.down());
    }
    
    #[test]
    fn test_cube_point_down_bottom() {
        let point: CubeSpherePoint<10> = CubeSpherePoint::new(CubeFace::Front, 0, 9);

        assert_eq!(CubeSpherePoint::new(CubeFace::Bottom, 0, 0), point.down());
    }

    #[test]
    fn test_cube_point_left_middle() {
        let point: CubeSpherePoint<10> = CubeSpherePoint::new(CubeFace::Left, 5, 5);

        assert_eq!(CubeSpherePoint::new(CubeFace::Left, 4, 5), point.left());
    }
    
    #[test]
    fn test_cube_point_left_left() {
        let point: CubeSpherePoint<10> = CubeSpherePoint::new(CubeFace::Left, 0, 5);

        assert_eq!(CubeSpherePoint::new(CubeFace::Back, 0, 5), point.left());
    }
   
    #[test]
    fn test_cube_point_left_right() {
        let point: CubeSpherePoint<10> = CubeSpherePoint::new(CubeFace::Left, 9, 5);

        assert_eq!(CubeSpherePoint::new(CubeFace::Left, 8, 5), point.left());
    }

    #[test]
    fn test_cube_point_right_middle() {
        let point: CubeSpherePoint<10> = CubeSpherePoint::new(CubeFace::Right, 5, 5);

        assert_eq!(CubeSpherePoint::new(CubeFace::Right, 6, 5), point.right());
    }
    
    #[test]
    fn test_cube_point_right_left() {
        let point: CubeSpherePoint<10> = CubeSpherePoint::new(CubeFace::Right, 0, 5);

        assert_eq!(CubeSpherePoint::new(CubeFace::Right, 1, 5), point.right());
    }
   
    #[test]
    fn test_cube_point_right_right() {
        let point: CubeSpherePoint<10> = CubeSpherePoint::new(CubeFace::Right, 9, 5);

        assert_eq!(CubeSpherePoint::new(CubeFace::Back, 9, 5), point.right());
    }

    #[test]
    fn test_cube_point_from_geographic_equator() {
        let point: CubeSpherePoint<100> = CubeSpherePoint::from_geographic(0.0, PI);

        assert_eq!(CubeSpherePoint::new(CubeFace::Back, 50, 50), point);
    }
    
    #[test]
    fn test_cube_point_from_geographic_north_pole() {
        let point: CubeSpherePoint<100> = CubeSpherePoint::from_geographic(PI / 2.0, PI);

        assert_eq!(CubeSpherePoint::new(CubeFace::Top, 50, 50), point);
    }
    
    #[test]
    fn test_cube_point_from_geographic_south_pole() {
        let point: CubeSpherePoint<100> = CubeSpherePoint::from_geographic(-PI / 2.0, PI);

        assert_eq!(CubeSpherePoint::new(CubeFace::Bottom, 50, 50), point);
    }
    
    #[test]
    fn test_cube_point_from_geographic_east() {
        let point: CubeSpherePoint<100> = CubeSpherePoint::from_geographic(0.0, -PI / 2.0);

        assert_eq!(CubeSpherePoint::new(CubeFace::Left, 50, 50), point);
    }
    
    #[test]
    fn test_cube_point_from_geographic_west() {
        let point: CubeSpherePoint<100> = CubeSpherePoint::from_geographic(0.0, PI / 2.0);

        assert_eq!(CubeSpherePoint::new(CubeFace::Right, 50, 50), point);
    }
    
    #[test]
    fn test_cube_point_from_geographic_less_west() {
        let point: CubeSpherePoint<100> = CubeSpherePoint::from_geographic(0.0, PI / 2.0 - 0.2);

        assert_eq!(CubeSpherePoint::new(CubeFace::Right, 39, 50), point);
    }


    #[test]
    fn test_cube_point_up_loop() {
        let start: CubeSpherePoint<3> = CubeSpherePoint::new(CubeFace::Bottom, 1, 2);

        assert_eq!(start, start.up().up().up()
                   .up().up().up()
                   .up().up().up()
                   .up().up().up());
    }
    
    #[test]
    fn test_cube_point_down_loop() {
        let start: CubeSpherePoint<3> = CubeSpherePoint::new(CubeFace::Top, 0, 0);

        assert_eq!(start, start.down().down().down()
                   .down().down().down()
                   .down().down().down()
                   .down().down().down());
    }
    
    #[test]
    fn test_cube_point_left_loop() {
        let start: CubeSpherePoint<3> = CubeSpherePoint::new(CubeFace::Back, 1, 0);

        assert_eq!(start, start.left().left().left()
                   .left().left().left()
                   .left().left().left()
                   .left().left().left());
    }
    
    #[test]
    fn test_cube_point_right_loop() {
        let start: CubeSpherePoint<3> = CubeSpherePoint::new(CubeFace::Front, 0, 2);

        assert_eq!(start, start.right().right().right()
                   .right().right().right()
                   .right().right().right()
                   .right().right().right());
    }

    #[test]
    fn test_cube_point_up_inverse_middle() {
        let start: CubeSpherePoint<10> = CubeSpherePoint::new(CubeFace::Front, 5, 3);

        assert_eq!(start, start.up().down());
    }
    
    #[test]
    fn test_cube_point_down_inverse_middle() {
        let start: CubeSpherePoint<10> = CubeSpherePoint::new(CubeFace::Front, 5, 3);

        assert_eq!(start, start.down().up());
    }
    
    #[test]
    fn test_cube_point_left_inverse_middle() {
        let start: CubeSpherePoint<10> = CubeSpherePoint::new(CubeFace::Front, 5, 3);

        assert_eq!(start, start.left().right());
    }
    
    #[test]
    fn test_cube_point_right_inverse_middle() {
        let start: CubeSpherePoint<10> = CubeSpherePoint::new(CubeFace::Front, 5, 3);

        assert_eq!(start, start.right().left());
    }
    
    #[test]
    fn test_cube_point_up_inverse_edge() {
        let start: CubeSpherePoint<10> = CubeSpherePoint::new(CubeFace::Front, 0, 0);

        assert_eq!(start, start.up().down());
    }
    
    #[test]
    fn test_cube_point_down_inverse_edge() {
        let start: CubeSpherePoint<10> = CubeSpherePoint::new(CubeFace::Front, 0, 9);

        assert_eq!(start, start.down().up());
    }
    
    #[test]
    fn test_cube_point_left_inverse_edge() {
        let start: CubeSpherePoint<10> = CubeSpherePoint::new(CubeFace::Front, 0, 0);

        assert_eq!(start, start.left().right());
    }
    
    #[test]
    fn test_cube_point_right_inverse_edge() {
        let start: CubeSpherePoint<10> = CubeSpherePoint::new(CubeFace::Front, 9, 0);

        assert_eq!(start, start.right().left());
    }

    #[test]
    fn test_cube_from_fn() {
        let grid: CubeSphereGrid<u16, 100> = CubeSphereGrid::from_fn(|point| point.x + point.y);

        assert_eq!(15, grid[CubeSpherePoint::new(CubeFace::Front, 5, 10)]);
    }
    
    #[test]
    fn test_cube_from_neighbours() {
        let grid: CubeSphereGrid<u16, 10> = CubeSphereGrid::from_fn(|point| point.x);

        let grid2 = grid.map_neighbours(|current, up, down, left, right| current + up + down + left + right);

        assert_eq!(25, grid2[CubeSpherePoint::new(CubeFace::Front, 5, 3)])
    }
    
    #[test]
    fn test_cube_from_neighbours_diagonals() {
        let grid: CubeSphereGrid<u16, 10> = CubeSphereGrid::from_fn(|point| point.x);

        let grid2 = grid.map_neighbours_diagonals(|up_left, up, up_right, left, current, right, down_left, down, down_right| up_left + up + up_right + left + current + right + down_left + down + down_right);

        assert_eq!(4 * 3 + 5 * 3 + 6 * 3, grid2[CubeSpherePoint::new(CubeFace::Front, 5, 3)])
    }
    
    #[test]
    fn test_cube_point_latitude_0() {
        let point: CubeSpherePoint<32000> = CubeSpherePoint::from_geographic(0.0, 0.0);
        
        println!("{:?}", point);

        assert_relative_eq!(0.0, point.latitude(), epsilon = 0.01);
    }
    
    #[test]
    fn test_cube_point_latitude_1() {
        let point: CubeSpherePoint<32000> = CubeSpherePoint::from_geographic(1.0, 0.0);

        println!("{:?}", point);
        println!("{:?}", point.position(1.0));

        assert_relative_eq!(1.0, point.latitude(), epsilon = 0.01);
    }
    
    #[test]
    fn test_cube_point_latitude_minus_1() {
        let point: CubeSpherePoint<32000> = CubeSpherePoint::from_geographic(-1.0, 0.0);
        
        println!("{:?}", point);
        println!("{:?}", point.position(1.0));

        assert_relative_eq!(-1.0, point.latitude(), epsilon = 0.01);
    }
    
    #[test]
    fn test_cube_point_latitude_half() {
        let point: CubeSpherePoint<32000> = CubeSpherePoint::from_geographic(0.5, 0.0);

        println!("{:?}", point);
        println!("{:?}", point.position(1.0));

        assert_relative_eq!(0.5, point.latitude(), epsilon = 0.01);
    }
    
    #[test]
    fn test_cube_point_latitude_minus_half() {
        let point: CubeSpherePoint<32000> = CubeSpherePoint::from_geographic(-0.5, 0.0);
        
        println!("{:?}", point);
        println!("{:?}", point.position(1.0));

        assert_relative_eq!(-0.5, point.latitude(), epsilon = 0.01);
    }
    
    #[test]
    fn test_cube_point_latitude_0_far() {
        let point: CubeSpherePoint<32000> = CubeSpherePoint::from_geographic(0.0, 3.0);
        
        println!("{:?}", point);
        println!("{:?}", point.position(1.0));

        assert_relative_eq!(0.0, point.latitude(), epsilon = 0.01);
    }
    
    #[test]
    fn test_cube_point_latitude_1_far() {
        let point: CubeSpherePoint<32000> = CubeSpherePoint::from_geographic(1.0, 2.0);
        
        println!("{:?}", point);
        println!("{:?}", point.position(1.0));

        assert_relative_eq!(1.0, point.latitude(), epsilon = 0.1);
    }
    
    #[test]
    fn test_cube_point_latitude_minus_1_far() {
        let point: CubeSpherePoint<32000> = CubeSpherePoint::from_geographic(-1.0, 3.0);
        
        println!("{:?}", point);
        println!("{:?}", point.position(1.0));

        assert_relative_eq!(-1.0, point.latitude(), epsilon = 0.01);
    }
    
    #[test]
    fn test_cube_point_latitude_half_far() {
        let point: CubeSpherePoint<32000> = CubeSpherePoint::from_geographic(0.5, 5.0);

        println!("{:?}", point);
        println!("{:?}", point.position(1.0));

        assert_relative_eq!(0.5, point.latitude(), epsilon = 0.01);
    }
    
    #[test]
    fn test_cube_point_latitude_minus_half_far() {
        let point: CubeSpherePoint<32000> = CubeSpherePoint::from_geographic(-0.5, 3.0);
        
        println!("{:?}", point);
        println!("{:?}", point.position(1.0));

        assert_relative_eq!(-0.5, point.latitude(), epsilon = 0.01);
    }
   
    #[test]
    fn test_cube_point_longitude_0() {
        let point: CubeSpherePoint<32000> = CubeSpherePoint::from_geographic(0.0, 0.0);
        
        println!("{:?}", point);
        println!("{:?}", point.position(1.0));

        assert_relative_eq!(0.0, point.longitude(), epsilon = 0.01);
    }
    
    #[test]
    fn test_cube_point_longitude_1() {
        let point: CubeSpherePoint<32000> = CubeSpherePoint::from_geographic(0.0, 1.0);
        
        println!("{:?}", point);
        println!("{:?}", point.position(1.0));

        assert_relative_eq!(1.0, point.longitude(), epsilon = 0.01);
    }
    
    #[test]
    fn test_cube_point_longitude_2() {
        let point: CubeSpherePoint<32000> = CubeSpherePoint::from_geographic(0.0, 2.0);
        
        println!("{:?}", point);
        println!("{:?}", point.position(1.0));

        assert_relative_eq!(2.0, point.longitude(), epsilon = 0.01);
    }
    
    #[test]
    fn test_cube_point_longitude_3() {
        let point: CubeSpherePoint<32000> = CubeSpherePoint::from_geographic(0.0, 3.0);
        
        println!("{:?}", point);
        println!("{:?}", point.position(1.0));

        assert_relative_eq!(3.0, point.longitude(), epsilon = 0.01);
    }
    
    #[test]
    fn test_cube_point_longitude_4() {
        let point: CubeSpherePoint<32000> = CubeSpherePoint::from_geographic(0.0, 4.0);
        
        println!("{:?}", point);
        println!("{:?}", point.position(1.0));

        assert_relative_eq!(4.0, point.longitude(), epsilon = 0.01);
    }
    
    #[test]
    fn test_cube_point_longitude_5() {
        let point: CubeSpherePoint<32000> = CubeSpherePoint::from_geographic(0.0, 5.0);
        
        println!("{:?}", point);
        println!("{:?}", point.position(1.0));

        assert_relative_eq!(5.0, point.longitude(), epsilon = 0.01);
    }
    
    #[test]
    fn test_cube_point_longitude_6() {
        let point: CubeSpherePoint<32000> = CubeSpherePoint::from_geographic(0.0, 6.0);
        
        println!("{:?}", point);
        println!("{:?}", point.position(1.0));

        assert_relative_eq!(6.0, point.longitude(), epsilon = 0.01);
    }
    
    #[test]
    fn test_cube_point_longitude_0_north() {
        let point: CubeSpherePoint<32000> = CubeSpherePoint::from_geographic(-1.5, 0.0);
        
        println!("{:?}", point);
        println!("{:?}", point.position(1.0));

        assert_relative_eq!(0.0, point.longitude(), epsilon = 0.01);
    }
    
    #[test]
    fn test_cube_point_longitude_1_north() {
        let point: CubeSpherePoint<32000> = CubeSpherePoint::from_geographic(-1.5, 1.0);
        
        println!("{:?}", point);
        println!("{:?}", point.position(1.0));

        assert_relative_eq!(1.0, point.longitude(), epsilon = 0.01);
    }
    
    #[test]
    fn test_cube_point_longitude_2_north() {
        let point: CubeSpherePoint<32000> = CubeSpherePoint::from_geographic(-1.5, 2.0);
        
        println!("{:?}", point);
        println!("{:?}", point.position(1.0));

        assert_relative_eq!(2.0, point.longitude(), epsilon = 0.01);
    }
    
    #[test]
    fn test_cube_point_longitude_4_north() {
        let point: CubeSpherePoint<32000> = CubeSpherePoint::from_geographic(-1.5, 4.0);
        
        println!("{:?}", point);
        println!("{:?}", point.position(1.0));

        assert_relative_eq!(4.0, point.longitude(), epsilon = 0.001);
    }
    
    #[test]
    fn test_cube_point_longitude_6_north() {
        let point: CubeSpherePoint<32000> = CubeSpherePoint::from_geographic(-1.5, 6.0);
        
        println!("{:?}", point);
        println!("{:?}", point.position(1.0));

        assert_relative_eq!(6.0, point.longitude(), epsilon = 0.01);
    }
    
    #[test]
    fn test_cube_point_longitude_0_south() {
        let point: CubeSpherePoint<32000> = CubeSpherePoint::from_geographic(1.5, 0.0);
        
        println!("{:?}", point);
        println!("{:?}", point.position(1.0));

        assert_relative_eq!(0.0, point.longitude(), epsilon = 0.01);
    }
    
    #[test]
    fn test_cube_point_longitude_1_south() {
        let point: CubeSpherePoint<32000> = CubeSpherePoint::from_geographic(1.5, 1.0);
        
        println!("{:?}", point);
        println!("{:?}", point.position(1.0));

        assert_relative_eq!(1.0, point.longitude(), epsilon = 0.01);
    }
    
    #[test]
    fn test_cube_point_longitude_2_south() {
        let point: CubeSpherePoint<32000> = CubeSpherePoint::from_geographic(1.5, 2.0);
        
        println!("{:?}", point);
        println!("{:?}", point.position(1.0));

        assert_relative_eq!(2.0, point.longitude(), epsilon = 0.01);
    }
    
    #[test]
    fn test_cube_point_longitude_4_south() {
        let point: CubeSpherePoint<32000> = CubeSpherePoint::from_geographic(1.5, 4.0);
        
        println!("{:?}", point);
        println!("{:?}", point.position(1.0));

        assert_relative_eq!(4.0, point.longitude(), epsilon = 0.01);
    }
    
    #[test]
    fn test_cube_point_longitude_6_south() {
        let point: CubeSpherePoint<32000> = CubeSpherePoint::from_geographic(1.5, 6.0);
        
        println!("{:?}", point);
        println!("{:?}", point.position(1.0));

        assert_relative_eq!(6.0, point.longitude(), epsilon = 0.01);
    }

    #[test]
    fn test_cube_clone_128() {
        let grid: CubeSphereGrid<u64, 128> = CubeSphereGrid::default();

        assert_eq!(grid, black_box(grid.clone()));
    }
    
    #[test]
    fn test_cube_clone_128_large_struct() {
        let grid: CubeSphereGrid<[u8; 1024], 128> = CubeSphereGrid::from_fn(|_| [0b11101101; 1024]);

        assert_eq!(grid, black_box(grid.clone()));
    }
    
    #[test]
    fn test_cube_clone_4096() {
        let grid: CubeSphereGrid<u64, 4096> = CubeSphereGrid::default();

        assert_eq!(grid, black_box(grid.clone()));
    }
}

