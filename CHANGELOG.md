<a id="v0.5.0"></a>
# [Surface Grid 0.5.0 (v0.5.0)](https://github.com/WhyAreAllTheseTaken/surface-grid/releases/tag/v0.5.0) - 2025-06-04

## New Features
Added three new methods to `SurfaceGrid`:
- `for_each_with_position`
- `par_for_each`
- `par_for_each_with_position`

## Dependencies
* Bump pixels from 0.13.0 to 0.14.0 by [@dependabot](https://github.com/dependabot) in [#14](https://github.com/WhyAreAllTheseTaken/surface-grid/pull/14)
* Bump winit from 0.30.5 to 0.30.7 by [@dependabot](https://github.com/dependabot) in [#15](https://github.com/WhyAreAllTheseTaken/surface-grid/pull/15)
* Bump winit from 0.30.7 to 0.30.8 by [@dependabot](https://github.com/dependabot) in [#16](https://github.com/WhyAreAllTheseTaken/surface-grid/pull/16)
* Bump itertools from 0.13.0 to 0.14.0 by [@dependabot](https://github.com/dependabot) in [#17](https://github.com/WhyAreAllTheseTaken/surface-grid/pull/17)
* Bump rand from 0.8.5 to 0.9.0 by [@dependabot](https://github.com/dependabot) in [#19](https://github.com/WhyAreAllTheseTaken/surface-grid/pull/19)
* Bump winit from 0.30.8 to 0.30.9 by [@dependabot](https://github.com/dependabot) in [#20](https://github.com/WhyAreAllTheseTaken/surface-grid/pull/20)
* Bump rand from 0.9.0 to 0.9.1 by [@dependabot](https://github.com/dependabot) in [#21](https://github.com/WhyAreAllTheseTaken/surface-grid/pull/21)
* Bump winit from 0.30.9 to 0.30.10 by [@dependabot](https://github.com/dependabot) in [#22](https://github.com/WhyAreAllTheseTaken/surface-grid/pull/22)
* Bump winit from 0.30.10 to 0.30.11 by [@dependabot](https://github.com/dependabot) in [#23](https://github.com/WhyAreAllTheseTaken/surface-grid/pull/23)

**Full Changelog**: https://github.com/WhyAreAllTheseTaken/surface-grid/compare/v0.4.0...v0.5.0

[Changes][v0.5.0]


<a id="v0.4.0"></a>
# [Surface Grid Version 0.4.0 (v0.4.0)](https://github.com/WhyAreAllTheseTaken/surface-grid/releases/tag/v0.4.0) - 2024-08-24

## Features
- Added a `for_each` method that can be used to mutate each cell.

## Dependencies
* Bump winit from 0.30.3 to 0.30.5 by [@dependabot](https://github.com/dependabot) in [#13](https://github.com/WhyAreAllTheseTaken/surface-grid/pull/13)

**Full Changelog**: https://github.com/WhyAreAllTheseTaken/surface-grid/compare/v0.3.2...v0.4.0

[Changes][v0.4.0]


<a id="v0.3.2"></a>
# [Surface Grid Version 0.3.2 (v0.3.2)](https://github.com/WhyAreAllTheseTaken/surface-grid/releases/tag/v0.3.2) - 2024-07-07

## Fixes
Fixed a bug where `RectangleSpherePoint` would fail for Y values in excess of `100`.

## Dependencies
* Bump itertools from 0.12.0 to 0.12.1 by [@dependabot](https://github.com/dependabot) in [#1](https://github.com/WhyAreAllTheseTaken/surface-grid/pull/1)
* Bump winit from 0.29.10 to 0.29.11 by [@dependabot](https://github.com/dependabot) in [#2](https://github.com/WhyAreAllTheseTaken/surface-grid/pull/2)
* Bump winit from 0.29.11 to 0.29.13 by [@dependabot](https://github.com/dependabot) in [#3](https://github.com/WhyAreAllTheseTaken/surface-grid/pull/3)
* Bump rayon from 1.8.1 to 1.9.0 by [@dependabot](https://github.com/dependabot) in [#4](https://github.com/WhyAreAllTheseTaken/surface-grid/pull/4)
* Bump winit from 0.29.13 to 0.29.15 by [@dependabot](https://github.com/dependabot) in [#6](https://github.com/WhyAreAllTheseTaken/surface-grid/pull/6)
* Bump rayon from 1.9.0 to 1.10.0 by [@dependabot](https://github.com/dependabot) in [#7](https://github.com/WhyAreAllTheseTaken/surface-grid/pull/7)
* Bump itertools from 0.12.1 to 0.13.0 by [@dependabot](https://github.com/dependabot) in [#9](https://github.com/WhyAreAllTheseTaken/surface-grid/pull/9)
* Bump winit from 0.29.15 to 0.30.3 by [@dependabot](https://github.com/dependabot) in [#11](https://github.com/WhyAreAllTheseTaken/surface-grid/pull/11)

**Full Changelog**: https://github.com/WhyAreAllTheseTaken/surface-grid/compare/v0.3.1...v0.3.2

[Changes][v0.3.2]


<a id="v0.3.1"></a>
# [Surface Grid Version 0.3.1 (v0.3.1)](https://github.com/WhyAreAllTheseTaken/surface-grid/releases/tag/v0.3.1) - 2024-01-27

## Fixes
- Fixed stretching on cube corners in `CubeSpherePoint`.
- Fixed certain positions not being mapped correctly on the top and bottom faces of `CubeSphereGrid`.
- Fixed latitude being inverted.

**Full Changelog**: https://github.com/Tomaso2468/surface-grid/compare/v0.3.0...v0.3.1

[Changes][v0.3.1]


<a id="v0.3.0"></a>
# [Surface Grid Version 0.3.0 (v0.3.0)](https://github.com/WhyAreAllTheseTaken/surface-grid/releases/tag/v0.3.0) - 2024-01-27

## Additions
- Added versions of `map_neighbours`, `map_neighbours_par`, `set_from_neighbours`, and `set_from_neighbours_par` that provide a position for each point calculated.

**Full Changelog**: https://github.com/Tomaso2468/surface-grid/compare/v0.2.0...v0.3.0


[Changes][v0.3.0]


<a id="v0.2.0"></a>
# [Surface Grid v0.2.0](https://github.com/WhyAreAllTheseTaken/surface-grid/releases/tag/v0.2.0) - 2024-01-26

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


<a id="v0.1.0"></a>
# [Surface Grid v0.1.0](https://github.com/WhyAreAllTheseTaken/surface-grid/releases/tag/v0.1.0) - 2024-01-25

The initial release of Surface Grid.

[Changes][v0.1.0]


[v0.5.0]: https://github.com/WhyAreAllTheseTaken/surface-grid/compare/v0.4.0...v0.5.0
[v0.4.0]: https://github.com/WhyAreAllTheseTaken/surface-grid/compare/v0.3.2...v0.4.0
[v0.3.2]: https://github.com/WhyAreAllTheseTaken/surface-grid/compare/v0.3.1...v0.3.2
[v0.3.1]: https://github.com/WhyAreAllTheseTaken/surface-grid/compare/v0.3.0...v0.3.1
[v0.3.0]: https://github.com/WhyAreAllTheseTaken/surface-grid/compare/v0.2.0...v0.3.0
[v0.2.0]: https://github.com/WhyAreAllTheseTaken/surface-grid/compare/v0.1.0...v0.2.0
[v0.1.0]: https://github.com/WhyAreAllTheseTaken/surface-grid/tree/v0.1.0

<!-- Generated by https://github.com/rhysd/changelog-from-release v3.9.0 -->
