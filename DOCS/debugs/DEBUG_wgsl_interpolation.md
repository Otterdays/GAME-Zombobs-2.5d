# Debug Log: WGSL Material ID Interpolation Error

## Issue
WGSL shader compilation failed with error:
```
integral user-defined vertex outputs must have a '@interpolate(flat)' attribute
@location(2) material_id: u32
```

## Root Cause
WebGPU/WGSL requires that integer-type vertex outputs (like `u32`) must explicitly specify `@interpolate(flat)` because integers cannot be smoothly interpolated between vertices like floats can.

## Fix
Added `@interpolate(flat)` attribute to the `material_id` field in the `VertexOutput` struct in `shader.wgsl`:

```wgsl
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) world_pos: vec3<f32>,
    @location(2) @interpolate(flat) material_id: u32, // Fixed!
};
```

This ensures the material ID is passed to the fragment shader without interpolation (each fragment gets the exact integer value from the vertex).

## Build Result
Clean build successful after `cargo clean` and rebuild (37.64s).
