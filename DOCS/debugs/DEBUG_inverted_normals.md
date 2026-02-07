# Debug Log: Fix Inverted Normals (Character Transparency)

## Issue
User reported "seeing through the front of the character".
Investigation revealed that the Front Face (Y+), Bottom Face (Z-), and Left Face (X-) of the procedural cube in `shader.wgsl` were defined with Clockwise (CW) winding order relative to their outward normals.
Since the renderer uses Back-Face Culling (`cull_mode: Some(wgpu::Face::Back)`) and defines Front Faces as Counter-Clockwise (`front_face: wgpu::FrontFace::Ccw`), these faces were being identified as "Back Faces" when viewed from the outside, and thus culled.

## Fix
Modified `src/engine/shader.wgsl` to reverse the winding order of the vertices for:
- Front Face (Y+)
- Bottom Face (Z-)
- Left Face (X-)

This ensures the normals point outwards and the faces are correctly rendered.

## Implementation Details
The vertex definition logic was manually updated in the `vs_main` function in `shader.wgsl`.
Swapped 2nd & 3rd vertices and 5th & 6th vertices (effectively reversing the triangle winding) for the affected faces.
