# frustumR

*3D rendered straight to a file*

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**Headless GPU rendering of scientific 3D figures, straight to PNG.**

No window opens. You build a scene in R, hand it to a Rust backend that draws on
the GPU (`wgpu`, so Vulkan / Metal / DX12), and get back the PNG bytes. The same
scene renders the same pixels on a laptop, a headless server, or CI, with no
display server and no OpenGL context to manage.

```r
library(frustumR)

# a sphere as a signed-distance volume, then its isosurface
vol  <- sphere_volume(size = 50, radius = 0.8)
mesh <- marching_cubes(vol, iso_value = 0)

cam   <- camera_perspective(c(2, 1.5, 2), c(0, 0, 0), fov_degrees = 45)
scene <- scene_new(cam, bounds_min = c(-1, -1, -1), bounds_max = c(1, 1, 1))
scene <- scene_set_light(scene, light_three_quarter())
scene <- scene_add_mesh(scene, mesh_with_material(mesh, "shell"))
scene <- scene_add_material(scene, material_solid("shell", c(0.3, 0.6, 0.9)))

fr_render(scene, "sphere.png", width = 800, height = 600)
```

## Where it runs

The renderer never opens a window. `render_to_png()` draws off-screen on the GPU
and returns raw PNG bytes; `fr_render()` wraps that and writes the file. That makes
it usable in the places interactive 3D normally can't reach: a render farm, a Docker
image, a GitHub Actions job, an `Rscript` on a remote box over SSH. The Rust/`wgpu`
backend targets Vulkan, Metal, and DX12, so the same code path covers Linux, macOS,
and Windows.

## Building scenes

A scene is a camera, some bounds, geometry, materials, and a light. Each `scene_*`
call returns a new scene, so a pipeline reads top to bottom:

```r
cam   <- camera_perspective(c(5, 3, 5), c(0, 0, 0), fov_degrees = 45)
scene <- scene_new(cam, c(-1, -1, -1), c(1, 1, 1)) |>
  scene_set_light(light_studio_soft())
```

Three geometry primitives cover most scientific figures:

- `mesh_new()` for triangle meshes, with optional `mesh_with_normals()` and
  `mesh_with_scalars()` for shading and colour mapping.
- `pointcloud_new()` for point clouds at a fixed point size.
- `polyline_new()` for connected line segments.

Cameras come in `camera_perspective()` and `camera_orthographic()`. Lighting is a
single directional light, either built with `light_new()` or picked from the presets:
`light_three_quarter()`, `light_studio_soft()`, `light_scientific_flat()`,
`light_rim_highlight()`, `light_depth_emphasis()`, `light_side_light()`.

## Materials and colour mapping

`material_solid()` (and `material_solid_alpha()`) paint a constant colour.
`material_scalar_mapped()` maps a per-vertex scalar through a colormap, so a field
on the surface becomes colour:

```r
mat  <- material_scalar_mapped("temp", "viridis", range = c(0, 1))
mesh <- mesh_with_scalars(mesh, scalar_values)
```

Available colormaps: `"viridis"`, `"plasma"`, `"magma"`, `"inferno"`, `"cividis"`.

## Volumes and isosurfaces

A `volume_new()` holds a 3D scalar field on a regular grid. `marching_cubes()`
extracts the isosurface at a given value as a mesh; `marching_cubes_multi()` extracts
several nested surfaces in one call. Two field operators run on the volume before
extraction:

- `volume_gradient_magnitude()` for edge / boundary surfaces.
- `volume_laplacian()` for ridge and zero-crossing surfaces.

```r
vol  <- volume_new(values, dimensions = c(nx, ny, nz),
                   spacing = c(dx, dy, dz), origin = c(-1, -1, -1))
grad <- volume_gradient_magnitude(vol)
mesh <- marching_cubes(grad, iso_value = 0.5)
```

## Installation

frustumR has a Rust backend, so building it needs a Rust toolchain (`cargo`,
`rustc >= 1.70`) and a GPU with Vulkan, Metal, or DX12 support. Install Rust from
[rustup.rs](https://rustup.rs/).

```r
# development version
remotes::install_github("gcol33/frustumR")
```

On Windows, add the GNU target so the package links against the R toolchain:

```bash
rustup target add x86_64-pc-windows-gnu
```

## Documentation

Function reference is available in the package help (`?marching_cubes`,
`?scene_new`, and so on) and in the [GitHub repository](https://github.com/gcol33/frustumR).

## License

MIT (see the LICENSE file)
