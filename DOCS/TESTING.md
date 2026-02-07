# Testing Strategy

## Overview
Because `zombs-engine` relies heavily on `wasm-bindgen` and `web-sys` (DOM/Canvas access), standard `cargo test` runs are tricky for the entire engine. However, we can and should test logic-heavy systems (ECS, Physics, Math) in isolation.

## Test Structure
*   **`tests/` Integration Tests**: These test public modules of the library.
    *   `collision_tests.rs`: Verifies AABB logic and Movement System sliding.
    *   `math_tests.rs`: Verifies Grid/Iso conversion logic (Todo).

## Running Tests
To run the headless logic tests (No WASM/Browser required):
```bash
cargo test --test collision_tests
```

To run WASM-specific tests (requires headless browser):
```bash
wasm-pack test --headless --chrome
```

## Critical Test Cases (To Implement)
1.  **AABB Static Collision**: Player moving +X should stop when hitting a Wall.
2.  **Projectile Despawn**: `projectile_system` should remove entities after `time_to_live` expires.
3.  **Coordinate Conversion**: `screen_to_world` logic should be mathematically verified against known inputs.
