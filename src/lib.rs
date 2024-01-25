//! A crate providing data structures for square-tiled grids wrapped around the surface of certain objects.
//! This create was intended to be used for the creation of cellular automata on non-flat grids.
//! The crate provides a trait `SurfaceGrid` with an associated type `Point` which can be used to traverse the grid squares.
//! Additionally, for grids that wrap a sphere the `Point` type implements the `SpherePoint` trait providing conversions
//! between geographic and surface grid coordinates.
//! 
//! ## Available Surfaces
//! ### Spheres
//! - `RectangleSphereGrid` - Uses an equirectangular projection to wrap a rectangle around the sphere.
//! - `CubeSphereGrid` - Projects a cube over the sphere with each face being a square grid.

use std::ops::{IndexMut, Index};

use rayon::iter::ParallelIterator;

pub mod sphere;

/// A grid wrapped around a surface.
pub trait SurfaceGrid<T> : IndexMut<Self::Point> + Index<Self::Point, Output = T> + IntoIterator<Item = (Self::Point, T)> {
    /// The type of a point on this grid.
    type Point: GridPoint + Send;

    /// Creates a new surface grid by calling the specified function for each point in the grid.
    ///
    /// - `f` - The function to apply.
    fn from_fn<F: FnMut(&Self::Point) -> T>(f: F) -> Self;

    /// Creates a new surface grid by calling the specified function in parallel for each point in
    /// the grid.
    ///
    /// - `f` - The function to apply.
    fn from_fn_par<F: Fn(&Self::Point) -> T + Send + Sync>(f: F) -> Self where T: Send + Sync;

    /// Applies a function to each cell and its direct neighbours.
    ///
    /// The provided function is called with the arguments: current, up, down, left, right.
    ///
    /// `f` - The function to apply.
    fn map_neighbours<F: FnMut(&T, &T, &T, &T, &T) -> T>(&self, mut f: F) -> Self where Self: Sized {
        Self::from_fn(|current| {
            f(&self[current.clone()], &self[current.up()], &self[current.down()], &self[current.left()], &self[current.right()])
        })
    }
    
    /// Applies a function to each cell and its direct neighbours including diagonals.
    ///
    /// The provided function is called with the arguments: up_left, up, up_right,
    /// left, current, right, down_left, down, down_right.
    ///
    /// `f` - The function to apply.
    fn map_neighbours_diagonals<
                F: FnMut(&T, &T, &T, &T, &T, &T, &T, &T, &T) -> T
            >(&self, mut f: F) -> Self where Self: Sized {
        Self::from_fn(|current| {
            f(
                &self[current.up().left()], &self[current.up()], &self[current.up().right()],
                &self[current.left()], &self[current.clone()], &self[current.right()],
                &self[current.down().left()], &self[current.down()], &self[current.down().right()]
                )
        })
    }
    
    /// Applies a function in parallel to each cell and its direct neighbours.
    ///
    /// The provided function is called with the arguments: current, up, down, left, right.
    ///
    /// `f` - The function to apply.
    fn map_neighbours_par<
                F: Fn(&T, &T, &T, &T, &T) -> T + Send + Sync
            >(&self, f: F) -> Self where Self: Sized + Sync, T: Send + Sync {
        Self::from_fn_par(|current| {
            f(&self[current.clone()], &self[current.up()], &self[current.down()], &self[current.left()], &self[current.right()])
        })
    }
    
    /// Applies a function in parallel to each cell and its direct neighbours including diagonals.
    ///
    /// The provided function is called with the arguments: up_left, up, up_right,
    /// left, current, right, down_left, down, down_right.
    ///
    /// `f` - The function to apply.
    fn map_neighbours_diagonals_par<
                F: Fn(&T, &T, &T, &T, &T, &T, &T, &T, &T) -> T + Send + Sync
            >(&self, f: F) -> Self where Self: Sized + Sync, T: Send + Sync {
        Self::from_fn_par(|current| {
            f(
                &self[current.up().left()], &self[current.up()], &self[current.up().right()],
                &self[current.left()], &self[current.clone()], &self[current.right()],
                &self[current.down().left()], &self[current.down()], &self[current.down().right()]
                )
        })
    }
    
    /// Updates this surface grid by calling the specified function for each point in the grid.
    ///
    /// - `f` - The function to apply.
    fn set_from_fn<F: FnMut(&Self::Point) -> T>(&mut self, f: F);
    
    /// Updates this surface grid by calling the specified function for each point in the grid in
    /// parallel.
    ///
    /// - `f` - The function to apply.
    fn set_from_fn_par<F: Fn(&Self::Point) -> T + Send + Sync>(&mut self, f: F) where T: Send + Sync;

    /// Applies a function to each cell and its direct neighbours.
    ///
    /// The provided function is called with the arguments: current, up, down, left, right.
    ///
    /// `source` - The source grid from which to read data.
    /// `f` - The function to apply.
    fn set_from_neighbours<
                U,
                G: SurfaceGrid<U, Point = Self::Point>,
                F: FnMut(&U, &U, &U, &U, &U) -> T
            >(&mut self, source: &G, mut f: F) {
        self.set_from_fn(|current| {
            f(&source[current.clone()], &source[current.up()], &source[current.down()], &source[current.left()], &source[current.right()])
        })
    }
    
    /// Applies a function to each cell and its direct neighbours including diagonals.
    ///
    /// The provided function is called with the arguments: up_left, up, up_right,
    /// left, current, right, down_left, down, down_right.
    ///
    /// `source` - The source grid from which to read data.
    /// `f` - The function to apply.
    fn set_from_neighbours_diagonals<
                U,
                G: SurfaceGrid<U, Point = Self::Point>,
                F: FnMut(&U, &U, &U, &U, &U, &U, &U, &U, &U) -> T
            >(&mut self, source: &G, mut f: F) {
        self.set_from_fn(|current| {
            f(
                &source[current.up().left()], &source[current.up()], &source[current.up().right()],
                &source[current.left()], &source[current.clone()], &source[current.right()],
                &source[current.down().left()], &source[current.down()], &source[current.down().right()]
                )
        })
    }
    
    /// Applies a function to each cell and its direct neighbours in parallel.
    ///
    /// The provided function is called with the arguments: current, up, down, left, right.
    ///
    /// `source` - The source grid from which to read data.
    /// `f` - The function to apply.
    fn set_from_neighbours_par<
                U,
                G: SurfaceGrid<U, Point = Self::Point> + Sync,
                F: Fn(&U, &U, &U, &U, &U) -> T + Send + Sync
            >(&mut self, source: &G, f: F) where T: Send + Sync {
        self.set_from_fn(|current| {
            f(&source[current.clone()], &source[current.up()], &source[current.down()], &source[current.left()], &source[current.right()])
        })
    }
    
    /// Applies a function to each cell and its direct neighbours including diagonals in parallel.
    ///
    /// The provided function is called with the arguments: up_left, up, up_right,
    /// left, current, right, down_left, down, down_right.
    ///
    /// `source` - The source grid from which to read data.
    /// `f` - The function to apply.
    fn set_from_neighbours_diagonals_par<
                U,
                G: SurfaceGrid<U, Point = Self::Point> + Sync,
                F: Fn(&U, &U, &U, &U, &U, &U, &U, &U, &U) -> T + Send + Sync
            >(&mut self, source: &G, f: F) where T: Send + Sync {
        self.set_from_fn(|current| {
            f(
                &source[current.up().left()], &source[current.up()], &source[current.up().right()],
                &source[current.left()], &source[current.clone()], &source[current.right()],
                &source[current.down().left()], &source[current.down()], &source[current.down().right()]
                )
        })
    }

    /// Iterates over the points in this grid and their values.
    fn iter<'a>(&'a self) -> impl Iterator<Item = (Self::Point, &'a T)> where T: 'a;

    /// Iterates over the points in this grid and their values in parallel.
    fn par_iter<'a>(&'a self) -> impl ParallelIterator<Item = (Self::Point, &'a T)> where T: 'a + Send + Sync;

    /// Iterates over the points in this grid.
    fn points(&self) -> impl Iterator<Item = Self::Point>;

    /// Iterates over the points in this grid in parallel.
    fn par_points(&self) -> impl ParallelIterator<Item = Self::Point>;
}

/// A point on a surface grid.
/// 
/// A type implementing this trait should ensure that the following conditions are met:
/// - Two points are equal if they point to the same physical location on the grid.
///
/// A point on the grid must also be associated with some direction.
/// For surfaces that loop this should work such that moving in the same direction will eventually
/// result in reaching the point at which you started moving.
pub trait GridPoint : Eq + PartialEq + Clone {
    /// Gets the point that is immediately above this grid point.
    fn up(&self) -> Self;
    
    /// Gets the point that is immediately below this grid point.
    fn down(&self) -> Self;

    /// Gets the point that is immediately to the left of this grid point.
    fn left(&self) -> Self;

    /// Gets the point that is immediately to the right of this grid point.
    fn right(&self) -> Self;

    /// Gets the position of the point in 3D space.
    ///
    /// - `scale` - The scale of the 3D object.
    fn position(&self, scale: f64) -> (f64, f64, f64);
}

