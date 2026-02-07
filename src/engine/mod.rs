// Engine Module
// Re-exports all engine components for easy use

#[cfg(target_arch = "wasm32")]
pub mod renderer;

#[cfg(not(target_arch = "wasm32"))]
pub mod renderer {
    use glam::{Vec2, Vec3};

    #[derive(Debug, Clone, Copy)]
    pub struct Camera {
        pub position: Vec3,
        pub smoothing: f32,
        pub yaw: f32,
        pub pitch: f32,
    }

    impl Camera {
        pub fn new(_width: u32, _height: u32) -> Self {
            Self {
                position: Vec3::ZERO,
                smoothing: 0.15,
                yaw: 0.0,
                pitch: 0.0,
            }
        }

        pub fn screen_to_world(&self, _screen_pos: Vec2, _screen_size: Vec2) -> Vec3 {
            Vec3::ZERO
        }
    }
}

pub mod components;
pub mod systems;
pub mod input;
pub mod ecs;
