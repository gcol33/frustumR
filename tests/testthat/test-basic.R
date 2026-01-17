# Basic tests for frustumR
# Note: These tests only check API structure since GPU rendering
# may not be available in all test environments

test_that("camera_perspective creates camera", {
  cam <- camera_perspective(c(5, 3, 5), c(0, 0, 0), 45)
  expect_true(inherits(cam, "externalptr"))
})

test_that("camera_orthographic creates camera", {
  cam <- camera_orthographic(c(0, 0, 10), c(0, 0, 0), 5)
  expect_true(inherits(cam, "externalptr"))
})

test_that("mesh_new creates mesh", {
  mesh <- mesh_new(
    positions = c(0, 0, 0, 1, 0, 0, 0.5, 1, 0),
    indices = c(0, 1, 2)
  )
  expect_true(inherits(mesh, "externalptr"))
})

test_that("mesh_vertex_count returns correct count", {
  mesh <- mesh_new(
    positions = c(0, 0, 0, 1, 0, 0, 0.5, 1, 0),
    indices = c(0, 1, 2)
  )
  expect_equal(mesh_vertex_count(mesh), 3)
})

test_that("mesh_triangle_count returns correct count", {
  mesh <- mesh_new(
    positions = c(0, 0, 0, 1, 0, 0, 0.5, 1, 0),
    indices = c(0, 1, 2)
  )
  expect_equal(mesh_triangle_count(mesh), 1)
})

test_that("mesh_with_material returns new mesh", {
  mesh <- mesh_new(
    positions = c(0, 0, 0, 1, 0, 0, 0.5, 1, 0),
    indices = c(0, 1, 2)
  )
  mesh2 <- mesh_with_material(mesh, "red")
  expect_true(inherits(mesh2, "externalptr"))
})

test_that("pointcloud_new creates point cloud", {
  pc <- pointcloud_new(c(0, 0, 0, 1, 1, 1), 5.0)
  expect_true(inherits(pc, "externalptr"))
})

test_that("polyline_new creates polyline", {
  line <- polyline_new(c(0, 0, 0, 1, 1, 1, 2, 0, 0), 2.0)
  expect_true(inherits(line, "externalptr"))
})

test_that("light_new creates light", {
  light <- light_new(c(1, 1, 1), 1.0)
  expect_true(inherits(light, "externalptr"))
})

test_that("light presets create lights", {
  expect_true(inherits(light_three_quarter(), "externalptr"))
  expect_true(inherits(light_scientific_flat(), "externalptr"))
  expect_true(inherits(light_studio_soft(), "externalptr"))
  expect_true(inherits(light_rim_highlight(), "externalptr"))
  expect_true(inherits(light_depth_emphasis(), "externalptr"))
  expect_true(inherits(light_side_light(), "externalptr"))
})

test_that("material_solid creates material", {
  mat <- material_solid("red", c(1, 0, 0))
  expect_true(inherits(mat, "externalptr"))
})

test_that("material_solid_alpha creates material", {
  mat <- material_solid_alpha("trans_red", c(1, 0, 0, 0.5))
  expect_true(inherits(mat, "externalptr"))
})

test_that("material_scalar_mapped creates material", {
  mat <- material_scalar_mapped("temp", "viridis", c(0, 1))
  expect_true(inherits(mat, "externalptr"))
})

test_that("scene_new creates scene", {
  cam <- camera_perspective(c(5, 3, 5), c(0, 0, 0), 45)
  scene <- scene_new(cam, c(-1, -1, -1), c(1, 1, 1))
  expect_true(inherits(scene, "externalptr"))
})

test_that("scene_add_mesh returns new scene", {
  cam <- camera_perspective(c(5, 3, 5), c(0, 0, 0), 45)
  scene <- scene_new(cam, c(-1, -1, -1), c(1, 1, 1))
  mesh <- mesh_new(c(0, 0, 0, 1, 0, 0, 0.5, 1, 0), c(0, 1, 2))
  scene2 <- scene_add_mesh(scene, mesh)
  expect_true(inherits(scene2, "externalptr"))
})

test_that("volume_new creates volume", {
  # 2x2x2 volume
  values <- c(0, 1, 1, 0, 1, 0, 0, 1)
  vol <- volume_new(values, c(2, 2, 2), c(1, 1, 1), c(0, 0, 0))
  expect_true(inherits(vol, "externalptr"))
})

test_that("volume_value_range returns correct range", {
  values <- c(0, 1, 2, 3, 4, 5, 6, 7)
  vol <- volume_new(values, c(2, 2, 2), c(1, 1, 1), c(0, 0, 0))
  range <- volume_value_range(vol)
  expect_equal(range[1], 0)
  expect_equal(range[2], 7)
})

test_that("sphere_volume creates volume", {
  vol <- sphere_volume(size = 10, radius = 0.5)
  expect_true(inherits(vol, "externalptr"))
})
