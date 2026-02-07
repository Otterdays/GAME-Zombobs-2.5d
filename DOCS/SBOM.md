# Security Bill of Materials (SBOM)

## Rust Crates (Cargo.toml)
| Package | Version | Purpose |
| :--- | :--- | :--- |
| `wasm-bindgen` | 0.2 | WASM JavaScript bindings |
| `wasm-bindgen-futures` | 0.4 | Async/Future support for WASM |
| `console_error_panic_hook` | 0.1.6 | Better panic logging in browser console |
| `wgpu` | 23.0 | WebGPU Implementation |
| `hecs` | 0.10 | Entity Component System |
| `glam` | 0.25 | Linear Algebra (Vectors/Matrices) |
| `rayon` | 1.8 | Parallelism |
| `serde` | 1.0 | Serialization |
| `serde-wasm-bindgen` | 0.4 | WASM serialization glue |
| `js-sys` | 0.3 | JS standard library bindings |
| `rand` | 0.8 | Random number generation |
| `getrandom` | 0.2 | Random source (with js feature) |
| `bytemuck` | 1.24.0 | Casting between plain data types |
| `image` | 0.24 | PNG decoding for textures |
| `web-sys` | 0.3 | Web API bindings (Window, Document, WebGPU, etc.) |

## Dev Dependencies
| Package | Version | Purpose |
| :--- | :--- | :--- |
| `wasm-bindgen-test` | 0.3 | Testing in WASM |

Last Updated: 2026-01-14
