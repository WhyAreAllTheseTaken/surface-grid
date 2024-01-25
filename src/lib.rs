//! A crate containing square tiled grids wrapped around various surfaces.

use std::ops::{IndexMut, Index};

pub mod sphere;

/// A grid wrapped around a surface.
pub trait SurfaceGrid<T> : IndexMut<Self::Point> + Index<Self::Point, Output = T> + IntoIterator<Item = (Self::Point, T)> {
    /// The type of a point on this grid.
    type Point: GridPoint;

    /// Creates a new surface grid by calling the specified function for each point in the grid.
    ///
    /// - `f` - The function to apply.
    fn from_fn<F: FnMut(&Self::Point) -> T>(f: F) -> Self;

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
    fn map_neighbours_diagonals<F: FnMut(&T, &T, &T, &T, &T, &T, &T, &T, &T) -> T>(&self, mut f: F) -> Self where Self: Sized {
        Self::from_fn(|current| {
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

    /// Applies a function to each cell and its direct neighbours.
    ///
    /// The provided function is called with the arguments: current, up, down, left, right.
    ///
    /// `source` - The source grid from which to read data.
    /// `f` - The function to apply.
    fn set_from_neighbours<U, G: SurfaceGrid<U, Point = Self::Point>, F: FnMut(&U, &U, &U, &U, &U) -> T>(&mut self, source: &G, mut f: F) {
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
    fn set_from_neighbours_diagonals<U, G: SurfaceGrid<U, Point = Self::Point>, F: FnMut(&U, &U, &U, &U, &U, &U, &U, &U, &U) -> T>(&mut self, source: &G, mut f: F) {
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

    /// Iterates over the points in this grid.
    fn points(&self) -> impl Iterator<Item = Self::Point>;
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

