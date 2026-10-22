# Story: frustumR

## Claim
Headless GPU rendering of scientific 3D figures from R, written straight to PNG.

## Audience
R users who need 3D figures (isosurfaces, meshes, point clouds) in scripts that run without a display: servers, containers, CI.

## Relation
- to: none

## Arcs

### A1 Rendering without a display
- question: can I make a 3D figure on a server with no screen?
- answer: off-screen wgpu rendering in Rust, PNG bytes back to R
- home: README (no article yet)

### A2 Isosurfaces from volumes
- question: how do I show a 3D scalar field?
- answer: marching cubes on a regular-grid volume, with gradient and Laplacian operators
- home: README; ?marching_cubes

### A3 Colour mapping
- question: how do I colour a surface by a value?
- answer: per-vertex scalars through a named colour map
- home: README; ?material_scalar_mapped

## Non-goals
Interactive viewing (implied by the design; not stated in the docs).

## Documents
- DESCRIPTION: claim, primitives, Rust and wgpu
- README: claim, list of arcs, installation (Rust toolchain), first render, routing to help pages
- articles: none yet; no pkgdown site

## Open
- No pkgdown site, so the README has no article links and no BibTeX/RIS download line.
- DESCRIPTION maintainer email and missing ORCID to be checked by the author.
