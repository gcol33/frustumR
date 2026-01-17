//! R bindings for Frustum GPU rendering framework.
//!
//! Provides an R API for creating scientific 3D visualizations.

use extendr_api::prelude::*;

use frustum_core::{
    Camera as RustCamera,
    Light as RustLight,
    Material as RustMaterial,
    Mesh as RustMesh,
    PointCloud as RustPointCloud,
    Polyline as RustPolyline,
    ScalarMappedMaterial as RustScalarMappedMaterial,
    SolidMaterial as RustSolidMaterial,
    Volume as RustVolume,
    marching_cubes as rust_marching_cubes,
    marching_cubes_multi as rust_marching_cubes_multi,
};
use frustum_core::scene::{Bounds as RustBounds, Scene as RustScene};
use frustum_render::{render_to_png as rust_render_to_png, RenderConfig as RustRenderConfig};

// =============================================================================
// Camera
// =============================================================================

/// Create a perspective camera.
///
/// @param position Camera position as c(x, y, z)
/// @param target Look-at target as c(x, y, z)
/// @param fov_degrees Field of view in degrees
/// @return External pointer to Camera object
/// @export
#[extendr]
fn camera_perspective(position: &[f64], target: &[f64], fov_degrees: f64) -> ExternalPtr<RustCamera> {
    let pos: [f32; 3] = [position[0] as f32, position[1] as f32, position[2] as f32];
    let tgt: [f32; 3] = [target[0] as f32, target[1] as f32, target[2] as f32];
    ExternalPtr::new(RustCamera::perspective(pos, tgt, fov_degrees as f32))
}

/// Create an orthographic camera.
///
/// @param position Camera position as c(x, y, z)
/// @param target Look-at target as c(x, y, z)
/// @param scale Orthographic scale factor
/// @return External pointer to Camera object
/// @export
#[extendr]
fn camera_orthographic(position: &[f64], target: &[f64], scale: f64) -> ExternalPtr<RustCamera> {
    let pos: [f32; 3] = [position[0] as f32, position[1] as f32, position[2] as f32];
    let tgt: [f32; 3] = [target[0] as f32, target[1] as f32, target[2] as f32];
    ExternalPtr::new(RustCamera::orthographic(pos, tgt, scale as f32))
}

// =============================================================================
// Light
// =============================================================================

/// Create a directional light.
///
/// @param direction Light direction as c(x, y, z)
/// @param intensity Light intensity (0-1)
/// @return External pointer to Light object
/// @export
#[extendr]
fn light_new(direction: &[f64], intensity: f64) -> ExternalPtr<RustLight> {
    let dir: [f32; 3] = [direction[0] as f32, direction[1] as f32, direction[2] as f32];
    ExternalPtr::new(RustLight::new(dir, intensity as f32))
}

/// Scientific flat lighting preset.
/// @return External pointer to Light object
/// @export
#[extendr]
fn light_scientific_flat() -> ExternalPtr<RustLight> {
    ExternalPtr::new(RustLight::scientific_flat())
}

/// Studio soft lighting preset.
/// @return External pointer to Light object
/// @export
#[extendr]
fn light_studio_soft() -> ExternalPtr<RustLight> {
    ExternalPtr::new(RustLight::studio_soft())
}

/// Rim highlight lighting preset.
/// @return External pointer to Light object
/// @export
#[extendr]
fn light_rim_highlight() -> ExternalPtr<RustLight> {
    ExternalPtr::new(RustLight::rim_highlight())
}

/// Depth emphasis lighting preset.
/// @return External pointer to Light object
/// @export
#[extendr]
fn light_depth_emphasis() -> ExternalPtr<RustLight> {
    ExternalPtr::new(RustLight::depth_emphasis())
}

/// Side light lighting preset.
/// @return External pointer to Light object
/// @export
#[extendr]
fn light_side_light() -> ExternalPtr<RustLight> {
    ExternalPtr::new(RustLight::side_light())
}

/// Three-quarter lighting preset (classic).
/// @return External pointer to Light object
/// @export
#[extendr]
fn light_three_quarter() -> ExternalPtr<RustLight> {
    ExternalPtr::new(RustLight::three_quarter())
}

// =============================================================================
// Materials
// =============================================================================

/// Create a solid color material.
///
/// @param id Material identifier string
/// @param color RGB color as c(r, g, b) with values in [0, 1]
/// @return External pointer to Material object
/// @export
#[extendr]
fn material_solid(id: &str, color: &[f64]) -> ExternalPtr<RustMaterial> {
    let rgb: [f32; 3] = [color[0] as f32, color[1] as f32, color[2] as f32];
    ExternalPtr::new(RustMaterial::Solid(RustSolidMaterial::new(id, rgb)))
}

/// Create a solid color material with alpha.
///
/// @param id Material identifier string
/// @param color RGBA color as c(r, g, b, a) with values in [0, 1]
/// @return External pointer to Material object
/// @export
#[extendr]
fn material_solid_alpha(id: &str, color: &[f64]) -> ExternalPtr<RustMaterial> {
    let rgba: [f32; 4] = [color[0] as f32, color[1] as f32, color[2] as f32, color[3] as f32];
    ExternalPtr::new(RustMaterial::Solid(RustSolidMaterial::with_alpha(id, rgba)))
}

/// Create a scalar-mapped colormap material.
///
/// @param id Material identifier string
/// @param colormap Colormap name ("viridis", "plasma", "magma", "inferno", "cividis")
/// @param range Scalar range as c(min, max)
/// @return External pointer to Material object
/// @export
#[extendr]
fn material_scalar_mapped(id: &str, colormap: &str, range: &[f64]) -> ExternalPtr<RustMaterial> {
    let r: [f32; 2] = [range[0] as f32, range[1] as f32];
    ExternalPtr::new(RustMaterial::ScalarMapped(RustScalarMappedMaterial::new(id, colormap, r)))
}

// =============================================================================
// Geometry
// =============================================================================

/// Create a mesh from positions and indices.
///
/// @param positions Flat vector of vertex positions (x0,y0,z0, x1,y1,z1, ...)
/// @param indices Flat vector of triangle indices (i0,i1,i2, ...)
/// @return External pointer to Mesh object
/// @export
#[extendr]
fn mesh_new(positions: &[f64], indices: &[i32]) -> ExternalPtr<RustMesh> {
    let pos: Vec<f32> = positions.iter().map(|&x| x as f32).collect();
    let idx: Vec<u32> = indices.iter().map(|&x| x as u32).collect();
    ExternalPtr::new(RustMesh::new(pos, idx))
}

/// Set material ID on a mesh.
///
/// @param mesh External pointer to Mesh
/// @param material_id Material identifier string
/// @return External pointer to new Mesh with material
/// @export
#[extendr]
fn mesh_with_material(mesh: ExternalPtr<RustMesh>, material_id: &str) -> ExternalPtr<RustMesh> {
    let m = mesh.as_ref().clone().with_material(material_id);
    ExternalPtr::new(m)
}

/// Set normals on a mesh.
///
/// @param mesh External pointer to Mesh
/// @param normals Flat vector of vertex normals
/// @return External pointer to new Mesh with normals
/// @export
#[extendr]
fn mesh_with_normals(mesh: ExternalPtr<RustMesh>, normals: &[f64]) -> ExternalPtr<RustMesh> {
    let n: Vec<f32> = normals.iter().map(|&x| x as f32).collect();
    let m = mesh.as_ref().clone().with_normals(n);
    ExternalPtr::new(m)
}

/// Set scalars on a mesh (for colormap materials).
///
/// @param mesh External pointer to Mesh
/// @param scalars Vector of per-vertex scalar values
/// @return External pointer to new Mesh with scalars
/// @export
#[extendr]
fn mesh_with_scalars(mesh: ExternalPtr<RustMesh>, scalars: &[f64]) -> ExternalPtr<RustMesh> {
    let s: Vec<f32> = scalars.iter().map(|&x| x as f32).collect();
    let m = mesh.as_ref().clone().with_scalars(s);
    ExternalPtr::new(m)
}

/// Get triangle count from mesh.
/// @param mesh External pointer to Mesh
/// @return Number of triangles
/// @export
#[extendr]
fn mesh_triangle_count(mesh: ExternalPtr<RustMesh>) -> i32 {
    (mesh.as_ref().indices.len() / 3) as i32
}

/// Get vertex count from mesh.
/// @param mesh External pointer to Mesh
/// @return Number of vertices
/// @export
#[extendr]
fn mesh_vertex_count(mesh: ExternalPtr<RustMesh>) -> i32 {
    (mesh.as_ref().positions.len() / 3) as i32
}

/// Create a point cloud.
///
/// @param positions Flat vector of point positions (x0,y0,z0, x1,y1,z1, ...)
/// @param size Point size
/// @return External pointer to PointCloud object
/// @export
#[extendr]
fn pointcloud_new(positions: &[f64], size: f64) -> ExternalPtr<RustPointCloud> {
    let pos: Vec<f32> = positions.iter().map(|&x| x as f32).collect();
    ExternalPtr::new(RustPointCloud::new(pos, size as f32))
}

/// Set material ID on a point cloud.
/// @export
#[extendr]
fn pointcloud_with_material(pc: ExternalPtr<RustPointCloud>, material_id: &str) -> ExternalPtr<RustPointCloud> {
    let p = pc.as_ref().clone().with_material(material_id);
    ExternalPtr::new(p)
}

/// Set scalars on a point cloud.
/// @export
#[extendr]
fn pointcloud_with_scalars(pc: ExternalPtr<RustPointCloud>, scalars: &[f64]) -> ExternalPtr<RustPointCloud> {
    let s: Vec<f32> = scalars.iter().map(|&x| x as f32).collect();
    let p = pc.as_ref().clone().with_scalars(s);
    ExternalPtr::new(p)
}

/// Create a polyline.
///
/// @param positions Flat vector of vertex positions
/// @param width Line width
/// @return External pointer to Polyline object
/// @export
#[extendr]
fn polyline_new(positions: &[f64], width: f64) -> ExternalPtr<RustPolyline> {
    let pos: Vec<f32> = positions.iter().map(|&x| x as f32).collect();
    ExternalPtr::new(RustPolyline::new(pos, width as f32))
}

/// Set material ID on a polyline.
/// @export
#[extendr]
fn polyline_with_material(line: ExternalPtr<RustPolyline>, material_id: &str) -> ExternalPtr<RustPolyline> {
    let l = line.as_ref().clone().with_material(material_id);
    ExternalPtr::new(l)
}

// =============================================================================
// Volume & Marching Cubes
// =============================================================================

/// Create a 3D volume from scalar values.
///
/// @param values Flat vector of scalar values (row-major order: x varies fastest)
/// @param dimensions Volume dimensions as c(nx, ny, nz)
/// @param spacing Voxel spacing as c(dx, dy, dz)
/// @param origin Volume origin as c(ox, oy, oz)
/// @return External pointer to Volume object
/// @export
#[extendr]
fn volume_new(values: &[f64], dimensions: &[i32], spacing: &[f64], origin: &[f64]) -> ExternalPtr<RustVolume> {
    let vals: Vec<f32> = values.iter().map(|&x| x as f32).collect();
    let dims: [usize; 3] = [dimensions[0] as usize, dimensions[1] as usize, dimensions[2] as usize];
    let sp: [f32; 3] = [spacing[0] as f32, spacing[1] as f32, spacing[2] as f32];
    let orig: [f32; 3] = [origin[0] as f32, origin[1] as f32, origin[2] as f32];
    ExternalPtr::new(RustVolume::new(vals, dims, sp, orig))
}

/// Get value range of a volume.
///
/// @param volume External pointer to Volume
/// @return c(min, max) value range
/// @export
#[extendr]
fn volume_value_range(volume: ExternalPtr<RustVolume>) -> Vec<f64> {
    let (min, max) = volume.as_ref().value_range();
    vec![min as f64, max as f64]
}

/// Compute gradient magnitude of a volume.
///
/// @param volume External pointer to Volume
/// @return External pointer to new Volume with gradient magnitude
/// @export
#[extendr]
fn volume_gradient_magnitude(volume: ExternalPtr<RustVolume>) -> ExternalPtr<RustVolume> {
    ExternalPtr::new(volume.as_ref().gradient_magnitude())
}

/// Compute Laplacian of a volume.
///
/// @param volume External pointer to Volume
/// @return External pointer to new Volume with Laplacian
/// @export
#[extendr]
fn volume_laplacian(volume: ExternalPtr<RustVolume>) -> ExternalPtr<RustVolume> {
    ExternalPtr::new(volume.as_ref().laplacian())
}

/// Extract isosurface using marching cubes.
///
/// @param volume External pointer to Volume
/// @param iso_value Isovalue for surface extraction
/// @return External pointer to Mesh
/// @export
#[extendr]
fn marching_cubes(volume: ExternalPtr<RustVolume>, iso_value: f64) -> ExternalPtr<RustMesh> {
    ExternalPtr::new(rust_marching_cubes(volume.as_ref(), iso_value as f32))
}

/// Extract multiple isosurfaces.
///
/// @param volume External pointer to Volume
/// @param iso_values Vector of isovalues
/// @return List of Mesh external pointers
/// @export
#[extendr]
fn marching_cubes_multi(volume: ExternalPtr<RustVolume>, iso_values: &[f64]) -> List {
    let isos: Vec<f32> = iso_values.iter().map(|&x| x as f32).collect();
    let surfaces = rust_marching_cubes_multi(volume.as_ref(), &isos);

    let meshes: Vec<ExternalPtr<RustMesh>> = surfaces
        .into_iter()
        .map(|s| ExternalPtr::new(s.mesh))
        .collect();

    List::from_values(meshes)
}

// =============================================================================
// Scene
// =============================================================================

/// Internal scene storage
#[derive(Clone)]
struct SceneData {
    camera: RustCamera,
    bounds_min: [f32; 3],
    bounds_max: [f32; 3],
    meshes: Vec<RustMesh>,
    point_clouds: Vec<RustPointCloud>,
    polylines: Vec<RustPolyline>,
    materials: Vec<RustMaterial>,
    light: Option<RustLight>,
}

impl SceneData {
    fn to_rust_scene(&self) -> RustScene {
        let mut scene = RustScene::new(
            self.camera.clone(),
            RustBounds {
                min: self.bounds_min,
                max: self.bounds_max,
            },
        );

        for material in &self.materials {
            scene.materials.push(material.clone());
        }

        for mesh in &self.meshes {
            scene = scene.add_mesh(mesh.clone());
        }

        for pc in &self.point_clouds {
            scene = scene.add_point_cloud(pc.clone());
        }

        for line in &self.polylines {
            scene = scene.add_polyline(line.clone());
        }

        if let Some(light) = &self.light {
            scene.light = Some(light.clone());
        }

        scene
    }
}

/// Create a new scene.
///
/// @param camera External pointer to Camera
/// @param bounds_min Scene minimum bounds as c(x, y, z)
/// @param bounds_max Scene maximum bounds as c(x, y, z)
/// @return External pointer to Scene object
/// @export
#[extendr]
fn scene_new(camera: ExternalPtr<RustCamera>, bounds_min: &[f64], bounds_max: &[f64]) -> ExternalPtr<SceneData> {
    let bmin: [f32; 3] = [bounds_min[0] as f32, bounds_min[1] as f32, bounds_min[2] as f32];
    let bmax: [f32; 3] = [bounds_max[0] as f32, bounds_max[1] as f32, bounds_max[2] as f32];

    ExternalPtr::new(SceneData {
        camera: camera.as_ref().clone(),
        bounds_min: bmin,
        bounds_max: bmax,
        meshes: Vec::new(),
        point_clouds: Vec::new(),
        polylines: Vec::new(),
        materials: Vec::new(),
        light: None,
    })
}

/// Add a mesh to the scene (returns new scene).
/// @export
#[extendr]
fn scene_add_mesh(scene: ExternalPtr<SceneData>, mesh: ExternalPtr<RustMesh>) -> ExternalPtr<SceneData> {
    let mut new_scene = scene.as_ref().clone();
    new_scene.meshes.push(mesh.as_ref().clone());
    ExternalPtr::new(new_scene)
}

/// Add a point cloud to the scene (returns new scene).
/// @export
#[extendr]
fn scene_add_points(scene: ExternalPtr<SceneData>, points: ExternalPtr<RustPointCloud>) -> ExternalPtr<SceneData> {
    let mut new_scene = scene.as_ref().clone();
    new_scene.point_clouds.push(points.as_ref().clone());
    ExternalPtr::new(new_scene)
}

/// Add a polyline to the scene (returns new scene).
/// @export
#[extendr]
fn scene_add_polyline(scene: ExternalPtr<SceneData>, polyline: ExternalPtr<RustPolyline>) -> ExternalPtr<SceneData> {
    let mut new_scene = scene.as_ref().clone();
    new_scene.polylines.push(polyline.as_ref().clone());
    ExternalPtr::new(new_scene)
}

/// Add a material to the scene (returns new scene).
/// @export
#[extendr]
fn scene_add_material(scene: ExternalPtr<SceneData>, material: ExternalPtr<RustMaterial>) -> ExternalPtr<SceneData> {
    let mut new_scene = scene.as_ref().clone();
    new_scene.materials.push(material.as_ref().clone());
    ExternalPtr::new(new_scene)
}

/// Set the scene light (returns new scene).
/// @export
#[extendr]
fn scene_set_light(scene: ExternalPtr<SceneData>, light: ExternalPtr<RustLight>) -> ExternalPtr<SceneData> {
    let mut new_scene = scene.as_ref().clone();
    new_scene.light = Some(light.as_ref().clone());
    ExternalPtr::new(new_scene)
}

// =============================================================================
// Rendering
// =============================================================================

/// Render scene to PNG bytes.
///
/// @param scene External pointer to Scene
/// @param width Image width in pixels
/// @param height Image height in pixels
/// @param background Background color as c(r, g, b, a)
/// @return Raw vector of PNG data
/// @export
#[extendr]
fn render_to_png(scene: ExternalPtr<SceneData>, width: i32, height: i32, background: &[f64]) -> Raw {
    let config = RustRenderConfig {
        width: width as u32,
        height: height as u32,
        background: [
            background[0] as f32,
            background[1] as f32,
            background[2] as f32,
            background[3] as f32,
        ],
    };

    let rust_scene = scene.as_ref().to_rust_scene();

    match rust_render_to_png(&rust_scene, &config) {
        Ok(png_data) => Raw::from_bytes(&png_data),
        Err(e) => {
            eprintln!("Render error: {:?}", e);
            Raw::new(0)
        }
    }
}

/// Save PNG data to file.
///
/// @param png_data Raw vector of PNG data
/// @param filename Output filename
/// @return Invisible TRUE on success, or error
/// @export
#[extendr]
fn save_png(png_data: Raw, filename: &str) -> extendr_api::Result<bool> {
    std::fs::write(filename, png_data.as_slice())
        .map_err(|e| extendr_api::Error::Other(format!("Failed to write PNG file: {}", e)))?;
    Ok(true)
}

// Macro to generate exports
extendr_module! {
    mod frustumR;

    // Camera
    fn camera_perspective;
    fn camera_orthographic;

    // Light
    fn light_new;
    fn light_scientific_flat;
    fn light_studio_soft;
    fn light_rim_highlight;
    fn light_depth_emphasis;
    fn light_side_light;
    fn light_three_quarter;

    // Materials
    fn material_solid;
    fn material_solid_alpha;
    fn material_scalar_mapped;

    // Mesh
    fn mesh_new;
    fn mesh_with_material;
    fn mesh_with_normals;
    fn mesh_with_scalars;
    fn mesh_triangle_count;
    fn mesh_vertex_count;

    // PointCloud
    fn pointcloud_new;
    fn pointcloud_with_material;
    fn pointcloud_with_scalars;

    // Polyline
    fn polyline_new;
    fn polyline_with_material;

    // Volume
    fn volume_new;
    fn volume_value_range;
    fn volume_gradient_magnitude;
    fn volume_laplacian;

    // Marching Cubes
    fn marching_cubes;
    fn marching_cubes_multi;

    // Scene
    fn scene_new;
    fn scene_add_mesh;
    fn scene_add_points;
    fn scene_add_polyline;
    fn scene_add_material;
    fn scene_set_light;

    // Rendering
    fn render_to_png;
    fn save_png;
}
