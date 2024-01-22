//! A crate containing square tiled grids wrapped around various surfaces.

use std::ops::{IndexMut, Index};

pub mod sphere;

/// A grid wrapped around a surface.
pub trait SurfaceGrid<T> : IndexMut<Self::Point> + Index<Self::Point, Output = T> {
    /// The type of a point on this grid.
    type Point: GridPoint;

    /// Creates a new surface grid by calling the specified function for each point in the grid.
    ///
    /// - `f` - The function to apply.
    fn from_fn<F: FnMut(&Self::Point) -> T>(f: F) -> Self;
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

