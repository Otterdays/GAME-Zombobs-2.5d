# Architecture

## Overview
ZOMBS-ENGINE is a high-performance 2.5D game engine designed for the web, utilizing Rust for logic and WebGPU for rendering.

## Core Systems

### 1. The Cycle (Game Loop)
The game loop is driven by the browser's `requestAnimationFrame` within `bootstrap.js`. It calls the `tick()` function exposed by the WASM module.
- **Input**: JS Event Listeners -> `engine.on_key_*` -> Rust `InputState`.
- **Update**: `Game::update` processes physics, logic, and ECS systems.
- **Render**: `Game::render` builds the frame and submits work to the GPU.

### 2. Rendering Pipeline (WebGPU)
Located in `src/engine/renderer.rs`.
- **Backend**: `wgpu` (WebGPU).
- **Technique**: Instanced Rendering. All 2.5D sprites are quads rendered in a single draw call where possible.
- **Shaders**: `src/engine/shader.wgsl` handles vertex transformation (World -> Camera -> Clip) and fragment coloring/texturing.
- **Camera**: First-person cinematic camera system.
  - **Projection**: Perspective projection with configurable FOV (default 60°).
  - **View**: Eye positioned at player head height, looking direction controlled by yaw/pitch angles.
  - **Mouse Look**: Pointer lock API enables smooth mouse look rotation. ESC unlocks cursor.
  - **Cinematic Effects**: Head bob based on movement speed, camera smoothing for stable feel.
  - **Screen-to-World**: Ray casting from camera through mouse position, intersecting ground plane (Z=0) for aiming.

### 3. Entity Component System (ECS)
Powered by `hecs`.
- **Entities**: Player, Enemies, Scenery.
- **Components**:
    - `Position` (glam::Vec2)
    - `Velocity` (glam::Vec2)
    - `Player` (Tag)
    - `Sprite` (Visual data)
    - `Camera` (Tag for camera entity)
- **Systems** (`src/systems.rs`):
    - `movement_system`: Applies velocity to position.
    - `input_system`: Maps inputs to player velocity.
    - `camera_follow_system`: Updates camera position to target.

## Data Flow
1. **JS** captures Input -> Sends to **Rust**.
2. **Rust** updates ECS State (Systems).
3. **Rust** prepares instance buffer (Transforms/Colors).
4. **WGPU** draws frame.
5. **Canvas** displays result.
