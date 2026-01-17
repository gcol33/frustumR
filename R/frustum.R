#' Render a scene to PNG file (convenience function)
#'
#' High-level rendering function that combines common operations.
#'
#' @param scene Scene object created with scene_new()
#' @param filename Output PNG filename
#' @param width Image width in pixels (default: 512)
#' @param height Image height in pixels (default: 512)
#' @param background Background color as c(r, g, b, a) (default: dark gray)
#' @return Invisible NULL
#' @export
#' @examples
#' \dontrun{
#' # Create camera and scene
#' cam <- camera_perspective(c(2, 1.5, 2), c(0, 0, 0), 45)
#' scene <- scene_new(cam, c(-1, -1, -1), c(1, 1, 1))
#'
#' # Add lighting
#' scene <- scene_set_light(scene, light_three_quarter())
#'
#' # Create and add a sphere
#' size <- 30
#' values <- numeric(size^3)
#' idx <- 1
#' for (z in seq_len(size)) {
#'   for (y in seq_len(size)) {
#'     for (x in seq_len(size)) {
#'       fx <- (x - 1) / (size - 1) * 2 - 1
#'       fy <- (y - 1) / (size - 1) * 2 - 1
#'       fz <- (z - 1) / (size - 1) * 2 - 1
#'       values[idx] <- sqrt(fx^2 + fy^2 + fz^2) - 0.8
#'       idx <- idx + 1
#'     }
#'   }
#' }
#'
#' vol <- volume_new(values, c(size, size, size),
#'                   c(2/(size-1), 2/(size-1), 2/(size-1)),
#'                   c(-1, -1, -1))
#' mesh <- marching_cubes(vol, 0.0)
#' mesh <- mesh_with_material(mesh, "sphere")
#'
#' mat <- material_solid("sphere", c(0.3, 0.6, 0.9))
#' scene <- scene_add_material(scene, mat)
#' scene <- scene_add_mesh(scene, mesh)
#'
#' # Render
#' fr_render(scene, "sphere.png")
#' }
fr_render <- function(scene, filename, width = 512, height = 512,
                      background = c(0.1, 0.1, 0.15, 1.0)) {
  png_data <- render_to_png(scene, width, height, background)
  save_png(png_data, filename)
  invisible(NULL)
}

#' Create a sphere volume for testing
#'
#' @param size Grid size in each dimension (default: 30)
#' @param radius Sphere radius (default: 0.8)
#' @return External pointer to Volume
#' @export
sphere_volume <- function(size = 30, radius = 0.8) {
  values <- numeric(size^3)
  idx <- 1
  for (z in seq_len(size)) {
    for (y in seq_len(size)) {
      for (x in seq_len(size)) {
        fx <- (x - 1) / (size - 1) * 2 - 1
        fy <- (y - 1) / (size - 1) * 2 - 1
        fz <- (z - 1) / (size - 1) * 2 - 1
        values[idx] <- sqrt(fx^2 + fy^2 + fz^2) - radius
        idx <- idx + 1
      }
    }
  }

  spacing <- rep(2 / (size - 1), 3)
  volume_new(values, c(size, size, size), spacing, c(-1, -1, -1))
}
