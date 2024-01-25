# Surface Grid
A crate providing data structures for square-tiled grids wrapped around the surface of certain objects.
This create was intended to be used for the creation of cellular automata on non-flat grids.
The crate provides a trait `SurfaceGrid` with an associated type `Point` which can be used to traverse the grid squares.
Additionally, for grids that wrap a sphere the `Point` type implements the `SpherePoint` trait providing conversions
between geographic and surface grid coordinates.

You can view examples in [examples](./examples).

## Available Surfaces
### Spheres
- `RectangleSphereGrid` - Uses an equirectangular projection to wrap a rectangle around the sphere.
- `CubeSphereGrid` - Projects a cube over the sphere with each face being a square grid.
