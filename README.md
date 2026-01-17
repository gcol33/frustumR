# frustumR

GPU-accelerated scientific 3D visualization for R.

## Requirements

- R 4.0+
- A GPU with Vulkan, Metal, or DX12 support
- Rust toolchain (cargo, rustc >= 1.70)

## Installation

```r
# Install from GitHub
remotes::install_github("gcol33/frustumR")
```

### Building from source

The package requires a Rust toolchain. Install it from https://rustup.rs/

```bash
# On Windows, ensure you have the GNU toolchain
rustup target add x86_64-pc-windows-gnu
```

## Quick Start

```r
library(frustumR)

# Create a camera
camera <- camera_perspective(
  position = c(5, 3, 5),
  target = c(0, 0, 0),
  fov_degrees = 45
)

# Create a scene with bounds
scene <- scene_new(camera, bounds_min = c(-1, -1, -1), bounds_max = c(1, 1, 1))

# Create a simple triangle mesh
mesh <- mesh_new(
  positions = c(0, 0, 0, 1, 0, 0, 0.5, 1, 0),
  indices = c(0, 1, 2)
)

# Add to scene and render
scene <- scene_add_mesh(scene, mesh)
png_data <- render_to_png(scene, width = 800, height = 600)
save_png(png_data, "output.png")
```

## Features

### Geometry Primitives

- **mesh_new()**: Triangle meshes with optional normals and scalars
- **pointcloud_new()**: Point clouds with configurable size
- **polyline_new()**: Connected line segments

### Materials

- **material_solid()**: Solid colors with optional alpha
- **material_scalar_mapped()**: Colormaps (viridis, plasma, magma, inferno, cividis)

### Isosurface Extraction

```r
# Create a 3D volume (sphere SDF)
nx <- ny <- nz <- 50
x <- seq(-1, 1, length.out = nx)
y <- seq(-1, 1, length.out = ny)
z <- seq(-1, 1, length.out = nz)
grid <- expand.grid(x = x, y = y, z = z)
values <- sqrt(grid$x^2 + grid$y^2 + grid$z^2) - 0.5

volume <- volume_new(
  values = values,
  dimensions = c(nx, ny, nz),
  spacing = c(2/nx, 2/ny, 2/nz),
  origin = c(-1, -1, -1)
)

mesh <- marching_cubes(volume, iso_value = 0)
```

### Camera Types

```r
# Perspective camera
cam <- camera_perspective(c(5, 3, 5), c(0, 0, 0), fov_degrees = 45)

# Orthographic camera
cam <- camera_orthographic(c(0, 0, 10), c(0, 0, 0), view_height = 5)
```

### Lighting

```r
# Custom directional light
light <- light_new(direction = c(1, 1, 1), intensity = 1.0)

# Presets
light <- light_three_quarter()
light <- light_scientific_flat()
light <- light_studio_soft()

scene <- scene_set_light(scene, light)
```

## API Reference

### Camera Functions

| Function | Description |
|----------|-------------|
| `camera_perspective()` | Create perspective camera |
| `camera_orthographic()` | Create orthographic camera |

### Geometry Functions

| Function | Description |
|----------|-------------|
| `mesh_new()` | Create triangle mesh |
| `mesh_with_material()` | Set mesh material |
| `mesh_with_normals()` | Set vertex normals |
| `mesh_with_scalars()` | Set vertex scalars |
| `pointcloud_new()` | Create point cloud |
| `polyline_new()` | Create polyline |

### Material Functions

| Function | Description |
|----------|-------------|
| `material_solid()` | Solid color material |
| `material_scalar_mapped()` | Colormap material |

### Scene Functions

| Function | Description |
|----------|-------------|
| `scene_new()` | Create empty scene |
| `scene_add_mesh()` | Add mesh to scene |
| `scene_add_points()` | Add point cloud |
| `scene_add_polyline()` | Add polyline |
| `scene_add_material()` | Add material |
| `scene_set_light()` | Set lighting |

### Rendering Functions

| Function | Description |
|----------|-------------|
| `render_to_png()` | Render scene to PNG bytes |
| `save_png()` | Save PNG to file |

### Isosurface Functions

| Function | Description |
|----------|-------------|
| `volume_new()` | Create 3D scalar volume |
| `marching_cubes()` | Extract isosurface |

## License

MIT
