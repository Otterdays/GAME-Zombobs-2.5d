# ZOMBOBS 2.5D CUSTOM ENGINE

## Project Overview

**ZOMBOBS 2.5D** is a custom high-performance game engine built for the web using **Rust** and **WebAssembly (WASM)**, rendering with **WebGPU**. It features a 2.5D perspective (3D world with 2D sprite billboards), a first-person camera system, and a robust Entity Component System (ECS).

### Key Technologies
- **Core Logic:** Rust (compiled to WASM via `wasm-pack`)
- **Rendering:** WebGPU (via `wgpu` crate)
- **ECS:** `hecs`
- **Math:** `glam` (SIMD-optimized)
- **Web Interface:** HTML5, CSS3, JavaScript (ES Modules)
- **Build System:** `cargo`, `wasm-pack`

## Architecture

The project is split into a Rust core (game logic, physics, rendering) and a JavaScript shell (input handling, DOM manipulation, UI overlay).

### Directory Structure
- **`src/`**: Rust source code.
    - `lib.rs`: WASM entry point and JS bindings.
    - `game.rs`: Main game loop and state management.
    - `engine/`: Core engine sub-modules (renderer, input, components, systems).
    - `engine/renderer.rs`: WebGPU pipeline setup and frame rendering.
    - `engine/shader.wgsl`: WebGPU Shader Language files.
- **`pkg/`**: Generated WASM binary and JS glue code (created by build).
- **`DOCS/`**: Comprehensive project documentation.
- **`concept-art/`**: Visual assets and reference images.
- **`bootstrap.js`**: JavaScript loader that initializes WASM and manages the main loop.
- **`index.html`**: The main entry point for the browser.

## Getting Started

### Prerequisites
- **Rust**: Latest stable release.
- **wasm-pack**: `cargo install wasm-pack`
- **Python**: For the local development server (or Node.js).
- **WebGPU Browser**: Chrome 113+, Edge, or Firefox Nightly.

### Build and Run
The project uses a convenience script `launch.bat` for Windows.

**Command:**
```cmd
.\launch.bat
```

**Manual Steps:**
1.  **Build WASM:**
    ```bash
    wasm-pack build --target web
    ```
2.  **Serve:**
    Start a local HTTP server (required for WASM MIME types).
    ```bash
    python -m http.server 8675
    ```
3.  **Access:**
    Open `http://localhost:8675` in your WebGPU-compatible browser.

## Development Conventions

### Coding Style
- **Rust**: Follow standard Rust formatting (`rustfmt`) and idioms. Use `snake_case` for functions/variables.
- **JavaScript**: Use `camelCase`.
- **CSS**: Use `kebab-case` for classes/IDs. Define variables for colors/fonts.

### UI Guidelines
- UI is implemented as an HTML/CSS overlay on top of the WebGPU canvas.
- **Colors**:
    - Primary: `#00ff00` (Matrix Green)
    - Danger: `#ff0000` (Red)
    - Background: `#0a0c10` (Dark)
- **Fonts**: Monospace (`Courier New`, `Consolas`).

### Documentation
- Maintain `DOCS/CHANGELOG.md` for major updates.
- Use `[TRACE: filename.md]` comments in code to link to documentation logic.

### Commit Messages
Follow **Conventional Commits**:
- `feat:` New features
- `fix:` Bug fixes
- `refactor:` Code restructuring
- `docs:` Documentation updates
