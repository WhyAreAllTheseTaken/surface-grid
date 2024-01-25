# Surface Grid
A crate providing data structures for square-tiled grids wrapped around the surface of certain objects.
You can view examples in [examples].

## Available Surfaces
### Spheres
- `RectangleSphereGrid` - Uses an equirectangular projection to wrap a rectangle around the sphere.
- `CubeSphereGrid` - Projects a cube over the sphere with each face being a square grid.
