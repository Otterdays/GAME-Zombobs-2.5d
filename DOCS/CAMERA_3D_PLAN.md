# 3D Camera System Implementation Plan

## Overview
Add a new 3D perspective camera alongside the existing isometric orthographic camera, with toggle functionality and keybind display.

## Current State Analysis

### Existing Camera System
- **Location**: `src/engine/renderer.rs`
- **Type**: Orthographic isometric camera
- **Camera Struct**: `Camera` with `position: Vec2`, `zoom: f32`, `aspect: f32`
- **View Matrix**: Uses `Mat4::look_at_rh()` with fixed offsets (offset_y = -10.0, offset_z = 10.0)
- **Projection**: `Mat4::orthographic_rh()` with view_height = 10.0 / zoom
- **Screen-to-World**: Ray-plane intersection at Z=0

### Input System
- **Location**: `src/game.rs` (`on_key_down`, `on_key_up`)
- **Location**: `src/engine/input.rs` (`InputState` struct)
- **Keybinds**: WASD/Arrows (movement), Mouse (aiming), Mouse buttons (shoot)
- **JS Bridge**: `bootstrap.js` handles keyboard events and forwards to Rust

### UI System
- **HUD Container**: `#hud-container` in `index.html`
- **Styling**: `ui.css` with monospace font, green theme
- **Current Elements**: Health bar (top-left), Ammo (bottom-right), Kills (top-right), Wave (top-center), Crosshair (center)

## Implementation Plan

### Phase 1: Camera Architecture Refactor

#### 1.1 Create Camera Mode Enum
**File**: `src/engine/renderer.rs`

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CameraMode {
    Isometric,  // Current top-down isometric view
    Perspective3D,  // New 3D perspective view
}
```

#### 1.2 Refactor Camera Struct
**File**: `src/engine/renderer.rs`

Extend `Camera` struct to support both modes:
- Add `mode: CameraMode` field
- Add 3D-specific fields: `pitch: f32`, `yaw: f32`, `distance: f32` (for orbit camera)
- Keep existing `position`, `zoom`, `aspect` for compatibility

**New Camera Structure**:
```rust
pub struct Camera {
    pub position: glam::Vec2,  // X, Y world position (follows player)
    pub zoom: f32,             // Zoom level (isometric)
    pub aspect: f32,           // Aspect ratio
    pub mode: CameraMode,      // Current camera mode
    // 3D Camera parameters
    pub pitch: f32,            // Vertical angle (-90 to 90 degrees)
    pub yaw: f32,              // Horizontal rotation (0 to 360 degrees)
    pub distance: f32,        // Distance from target (3D mode)
    pub fov: f32,             // Field of view (3D mode, in radians)
}
```

#### 1.3 Implement Dual Camera Methods
**File**: `src/engine/renderer.rs`

Update `Camera::build_view_projection_matrix()` to switch based on mode:

```rust
pub fn build_view_projection_matrix(&self) -> Mat4 {
    match self.mode {
        CameraMode::Isometric => {
            // Existing isometric implementation
        }
        CameraMode::Perspective3D => {
            // New perspective implementation
            // Use perspective projection
            // Calculate eye position from pitch/yaw/distance
            // Look at player position
        }
    }
}
```

**3D Camera Implementation**:
- **Projection**: `Mat4::perspective_rh()` with `fov`, `aspect`, `near: 0.1`, `far: 100.0`
- **View**: Calculate eye position using spherical coordinates:
  - `eye_x = position.x + distance * cos(pitch) * sin(yaw)`
  - `eye_y = position.y + distance * cos(pitch) * cos(yaw)`
  - `eye_z = position.z + distance * sin(pitch)`
- **Target**: `Vec3::new(position.x, position.y, 0.0)` (player position at ground level)
- **Up Vector**: `Vec3::Z` (Z-up coordinate system)

#### 1.4 Update screen_to_world for 3D Mode
**File**: `src/engine/renderer.rs`

Extend `Camera::screen_to_world()` to handle perspective projection:
- For isometric: Keep existing ray-plane intersection
- For 3D: Use ray-casting with perspective projection (intersect with Z=0 plane)

### Phase 2: Input System Integration

#### 2.1 Add Camera Toggle to InputState
**File**: `src/engine/input.rs`

Add field to track toggle state (edge-triggered, not held):
```rust
pub struct InputState {
    // ... existing fields
    pub toggle_camera: bool,  // Set to true on key press, cleared after handling
}
```

#### 2.2 Add Keybind Handler
**File**: `src/game.rs`

Update `on_key_down()` to handle camera toggle:
```rust
pub fn on_key_down(&mut self, key_code: &str) {
    match key_code {
        // ... existing keybinds
        "KeyC" => self.input_state.toggle_camera = true,  // C key to toggle camera
        _ => {}
    }
}
```

#### 2.3 Add Camera Toggle System
**File**: `src/game.rs` (in `tick()` method)

Add camera toggle logic after input processing:
```rust
// Handle camera toggle (edge-triggered)
if self.input_state.toggle_camera {
    self.camera.mode = match self.camera.mode {
        CameraMode::Isometric => CameraMode::Perspective3D,
        CameraMode::Perspective3D => CameraMode::Isometric,
    };
    self.input_state.toggle_camera = false;  // Clear flag
}
```

#### 2.4 Update Camera Follow for 3D Mode
**File**: `src/game.rs`

Modify camera follow system to work with both modes:
- Isometric: Update `camera.position` (Vec2) from player position
- 3D: Update `camera.position` (Vec2) and maintain pitch/yaw/distance

### Phase 3: UI Keybind Display

#### 3.1 Add Keybind Display Element
**File**: `index.html`

Add new element in `#hud-container`:
```html
<!-- Keybind Display (Bottom-Center) -->
<div id="keybind-display">
    <div class="keybind-item">
        <span class="key">WASD</span>
        <span class="action">Move</span>
    </div>
    <div class="keybind-item">
        <span class="key">Mouse</span>
        <span class="action">Aim</span>
    </div>
    <div class="keybind-item">
        <span class="key">LMB</span>
        <span class="action">Shoot</span>
    </div>
    <div class="keybind-item">
        <span class="key">C</span>
        <span class="action">Toggle Camera</span>
    </div>
</div>
```

#### 3.2 Style Keybind Display
**File**: `ui.css`

Add styles for keybind display at bottom-center:
```css
/* Keybind Display (Bottom-Center) */
#keybind-display {
    position: absolute;
    bottom: 20px;
    left: 50%;
    transform: translateX(-50%);
    display: flex;
    gap: 20px;
    background: rgba(10, 12, 16, 0.8);
    border: 2px solid var(--color-primary);
    padding: 10px 20px;
    font-family: var(--font-family);
}

.keybind-item {
    display: flex;
    align-items: center;
    gap: 8px;
    color: var(--color-primary);
    font-size: var(--font-size-small);
}

.key {
    background: rgba(0, 255, 0, 0.2);
    border: 1px solid var(--color-primary);
    padding: 4px 8px;
    border-radius: 3px;
    font-weight: bold;
}

.action {
    color: rgba(255, 255, 255, 0.8);
}
```

### Phase 4: Testing & Polish

#### 4.1 Camera Transition
- Ensure smooth transition when toggling (no visual glitches)
- Verify player remains centered in both views
- Test screen-to-world conversion works in both modes

#### 4.2 Default 3D Camera Parameters
- **Pitch**: ~-30 degrees (slight downward angle)
- **Yaw**: 45 degrees (diagonal view)
- **Distance**: 15.0 units (adjustable)
- **FOV**: 60 degrees (π/3 radians)

#### 4.3 Edge Cases
- Handle window resize in both camera modes
- Ensure zoom works in isometric mode
- Verify mouse aiming works correctly in both modes

## File Changes Summary

### Modified Files
1. **`src/engine/renderer.rs`**
   - Add `CameraMode` enum
   - Extend `Camera` struct with 3D fields
   - Update `build_view_projection_matrix()` for dual mode
   - Update `screen_to_world()` for perspective projection

2. **`src/engine/input.rs`**
   - Add `toggle_camera: bool` to `InputState`

3. **`src/game.rs`**
   - Add camera toggle keybind handler (`KeyC`)
   - Add camera toggle system in `tick()`
   - Update camera initialization with default 3D params

4. **`index.html`**
   - Add `#keybind-display` element in HUD container

5. **`ui.css`**
   - Add styles for keybind display

### New Files
None (all changes are additions/modifications to existing files)

## Implementation Order

1. ✅ **Phase 1.1-1.2**: Create enum and extend Camera struct
2. ✅ **Phase 1.3**: Implement dual camera matrix building
3. ✅ **Phase 1.4**: Update screen_to_world for 3D
4. ✅ **Phase 2.1-2.2**: Add input handling
5. ✅ **Phase 2.3**: Add toggle system
6. ✅ **Phase 2.4**: Update camera follow
7. ✅ **Phase 3.1-3.2**: Add UI keybind display
8. ✅ **Phase 4**: Testing and parameter tuning

## Notes

- **Camera Toggle Key**: `C` key (can be changed if conflicts)
- **Default Mode**: Isometric (preserve existing behavior)
- **3D Camera Type**: Orbit camera (rotates around player, maintains distance)
- **Future Enhancements**: Mouse drag to rotate 3D camera, scroll to adjust distance

## Trace References
- [TRACE: ARCHITECTURE.md] - Camera system architecture
- [TRACE: SCRATCHPAD.md] - Active task tracking
