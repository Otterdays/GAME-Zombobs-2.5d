# ZOMBS-ENGINE (v0.0.1 ALPHA)

A custom high-performance 2.5D game engine built in Rust and WebAssembly for **ZOMBOBS**.

## 🚀 Architecture

- **Language:** Rust (compiled to WebAssembly)
- **Rendering:** WebGPU (via `wgpu`)
- **ECS:** `hecs` (Entity Component System)
- **Math:** `glam` (SIMD-optimized vector math)
- **Concurrency:** `rayon` (Parallel iterators)

## 📁 Project Structure

```
Zombobs-2.5d-Custom-Engine/
├── src/
│   ├── lib.rs         # Entry point & JS bindings
│   ├── game.rs        # Main game loop & state
│   ├── renderer.rs    # WebGPU rendering pipeline
│   ├── ecs.rs         # Entity Component System setup
│   ├── systems.rs     # Game logic systems (movement, etc.)
│   ├── components.rs  # Data components (Position, Velocity, etc.)
│   ├── shader.wgsl    # WebGPU shaders
│   └── utils.rs       # Utility functions (logging, panic hooks)
├── Cargo.toml         # Dependencies
├── index.html         # Web entry point
├── bootstrap.js       # JS loader for WASM
└── README.md          # This file
```

## 🛠️ Prerequisites

1.  **Rust**: [Install Rust](https://rustup.rs/)
2.  **wasm-pack**: Install via `cargo install wasm-pack`

## 🏗️ Build & Run

1.  **Install Target**:
    ```bash
    rustup target add wasm32-unknown-unknown
    ```

2.  **Build**:
    ```bash
    wasm-pack build --target web
    ```
    *This will generate a `pkg/` directory containing the compiled `.wasm` binary and JS glue code.*

3.  **Run**:
    You need a local web server to serve the files (browsers block WASM from `file://` protocol).
    ```bash
    # Using python (if installed)
    python3 -m http.server
    
    # OR using node http-server
    npx http-server .
    ```

4.  **Play**:
    Open `http://localhost:8000` in a browser with WebGPU support (Chrome 113+, Edge, etc.).

## 🧩 Engine Modules

### 1. Renderer (`src/engine/renderer.rs`)
- Initializes WebGPU device and surface.
- Manages the render pipeline and shaders.
- Handles resizing and frame presentation.
- First-person camera with pointer lock mouse look and screen-to-world conversion.

### 2. ECS (`hecs`)
- **Entities**: Simple IDs.
- **Components**: Pure data structs (`Position`, `Velocity`, `Weapon`, `Player`, etc.).
- **Systems**: Functions that operate on queries (`movement_system`, `weapon_system`, `projectile_system`).

### 3. Game Loop (`src/game.rs`)
- Ticks at browser refresh rate (via `requestAnimationFrame`).
- Updates physics/logic (`dt` based).
- Calls renderer.
- Manages player spawn and world setup.

### 4. Weapon System (`src/engine/components.rs`, `src/engine/systems.rs`)
- `Weapon` component with fire rate, clip size, reload mechanics.
- `weapon_system()` updates weapon state and calls UI bridge.
- Integrated with `player_input_system()` for shooting logic.

### 5. UI System (`index.html`, `bootstrap.js`, `ui.css`)
- HTML/CSS overlay for zero performance impact.
- Rust → JavaScript bridge for real-time UI updates.
- Complete HUD with health, ammo, kills, wave, crosshair.

## 🎮 Current Features

- ✅ **ECS System**: Entity Component System with `hecs`
- ✅ **WebGPU Rendering**: High-performance instanced quad rendering
- ✅ **First-Person Camera**: Pointer lock mouse look with smooth rotation
- ✅ **FPS Movement**: WASD movement relative to camera direction (Minecraft-style)
- ✅ **Weapon System**: Taurus G2C pistol with ammo management
- ✅ **In-Game HUD**: Health bar, ammo counter, kill counter, wave indicator, crosshair, keybinds
- ✅ **Collision Detection**: AABB-based collision for trees and entities
- ✅ **Projectile System**: Bullets with physics and lifetime management
- ✅ **Menu System**: Start/Resume/Settings with fullscreen toggle

## 📝 Next Steps

1.  **Enemy System**: Add zombie AI that chases player
2.  **Health System**: Implement player health and damage from zombies
3.  **Bullet Collision**: Detect hits on enemies and call `incrementKills()`
4.  **Wave System**: Spawn increasing waves of zombies
5.  **Sprite Rendering**: Add texture support for more detailed visuals
