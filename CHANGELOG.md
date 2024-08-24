<a name="v0.4.0"></a>
# [Surface Grid Version 0.4.0 (v0.4.0)](https://github.com/WhyAreAllTheseTaken/surface-grid/releases/tag/v0.4.0) - 24 Aug 2024

## Features
- Added a `for_each` method that can be used to mutate each cell.

## Dependencies
* Bump winit from 0.30.3 to 0.30.5 by [@dependabot](https://github.com/dependabot) in https://github.com/WhyAreAllTheseTaken/surface-grid/pull/13

**Full Changelog**: https://github.com/WhyAreAllTheseTaken/surface-grid/compare/v0.3.2...v0.4.0

[Changes][v0.4.0]


<a name="v0.3.2"></a>
# [Surface Grid Version 0.3.2 (v0.3.2)](https://github.com/WhyAreAllTheseTaken/surface-grid/releases/tag/v0.3.2) - 07 Jul 2024

## Fixes
Fixed a bug where `RectangleSpherePoint` would fail for Y values in excess of `100`.

## Dependencies
* Bump itertools from 0.12.0 to 0.12.1 by [@dependabot](https://github.com/dependabot) in https://github.com/WhyAreAllTheseTaken/surface-grid/pull/1
* Bump winit from 0.29.10 to 0.29.11 by [@dependabot](https://github.com/dependabot) in https://github.com/WhyAreAllTheseTaken/surface-grid/pull/2
* Bump winit from 0.29.11 to 0.29.13 by [@dependabot](https://github.com/dependabot) in https://github.com/WhyAreAllTheseTaken/surface-grid/pull/3
* Bump rayon from 1.8.1 to 1.9.0 by [@dependabot](https://github.com/dependabot) in https://github.com/WhyAreAllTheseTaken/surface-grid/pull/4
* Bump winit from 0.29.13 to 0.29.15 by [@dependabot](https://github.com/dependabot) in https://github.com/WhyAreAllTheseTaken/surface-grid/pull/6
* Bump rayon from 1.9.0 to 1.10.0 by [@dependabot](https://github.com/dependabot) in https://github.com/WhyAreAllTheseTaken/surface-grid/pull/7
* Bump itertools from 0.12.1 to 0.13.0 by [@dependabot](https://github.com/dependabot) in https://github.com/WhyAreAllTheseTaken/surface-grid/pull/9
* Bump winit from 0.29.15 to 0.30.3 by [@dependabot](https://github.com/dependabot) in https://github.com/WhyAreAllTheseTaken/surface-grid/pull/11

**Full Changelog**: https://github.com/WhyAreAllTheseTaken/surface-grid/compare/v0.3.1...v0.3.2

[Changes][v0.3.2]


<a name="v0.3.1"></a>
# [Surface Grid Version 0.3.1 (v0.3.1)](https://github.com/WhyAreAllTheseTaken/surface-grid/releases/tag/v0.3.1) - 27 Jan 2024

## Fixes
- Fixed stretching on cube corners in `CubeSpherePoint`.
- Fixed certain positions not being mapped correctly on the top and bottom faces of `CubeSphereGrid`.
- Fixed latitude being inverted.

**Full Changelog**: https://github.com/Tomaso2468/surface-grid/compare/v0.3.0...v0.3.1

[Changes][v0.3.1]


<a name="v0.3.0"></a>
# [Surface Grid Version 0.3.0 (v0.3.0)](https://github.com/WhyAreAllTheseTaken/surface-grid/releases/tag/v0.3.0) - 27 Jan 2024

## Additions
- Added versions of `map_neighbours`, `map_neighbours_par`, `set_from_neighbours`, and `set_from_neighbours_par` that provide a position for each point calculated.

**Full Changelog**: https://github.com/Tomaso2468/surface-grid/compare/v0.2.0...v0.3.0


[Changes][v0.3.0]


<a name="v0.2.0"></a>
# [Surface Grid v0.2.0](https://github.com/WhyAreAllTheseTaken/surface-grid/releases/tag/v0.2.0) - 26 Jan 2024

## Additions
- Added methods to `SurfaceGrid` that perform operations in parallel.
  - `from_fn_par` - Initializes a grid in parallel. Can be used instead of `from_fn`.
  - `map_neighbours_par` - Performs `map_neighbours` in parallel.
  - `map_neighbours_diagonals_par` - Performs `map_neighbours_diagonals` in parallel.
  - `set_from_fn_par` - Sets the values in a grid in parallel. Can be used instead of `set_from_fn`.
  - `set_from_neighbours_par` - Performs `set_from_neighbours` in parallel.
  - `set_from_neighbours_diagonals_par` - Performs `set_from_neighbours_diagonals` in parallel.
  - `par_iter` - Returns a `ParallelIterator` over the points in the grid and their values.
  - `par_points` - Returns a `ParallelIterator` over the points in the grid.

## Fixes
- Fixed the Y position returned by the `position` method of `RectangleSpherePoint` returning 1.0 when it should return 0.0 and 0.0 when it should return 1.0.
- Fixed the incorrect geographic coordinates produced by `CubeSpherePoint`.
- Fixed incorrect position select when converting from geographic coordinates in `CubeSpherePoint`.

## Dependencies
- The crate now depends on `rayon` for parallel operations.
- The crate now uses `static-array` version 0.5.0 with the `rayon` feature.

**Full Changelog**: https://github.com/Tomaso2468/surface-grid/compare/v0.1.0...v0.2.0

[Changes][v0.2.0]


<a name="v0.1.0"></a>
# [Surface Grid v0.1.0](https://github.com/WhyAreAllTheseTaken/surface-grid/releases/tag/v0.1.0) - 25 Jan 2024

The initial release of Surface Grid.

[Changes][v0.1.0]


[v0.4.0]: https://github.com/WhyAreAllTheseTaken/surface-grid/compare/v0.3.2...v0.4.0
[v0.3.2]: https://github.com/WhyAreAllTheseTaken/surface-grid/compare/v0.3.1...v0.3.2
[v0.3.1]: https://github.com/WhyAreAllTheseTaken/surface-grid/compare/v0.3.0...v0.3.1
[v0.3.0]: https://github.com/WhyAreAllTheseTaken/surface-grid/compare/v0.2.0...v0.3.0
[v0.2.0]: https://github.com/WhyAreAllTheseTaken/surface-grid/compare/v0.1.0...v0.2.0
[v0.1.0]: https://github.com/WhyAreAllTheseTaken/surface-grid/tree/v0.1.0

<!-- Generated by https://github.com/rhysd/changelog-from-release v3.7.1 -->
