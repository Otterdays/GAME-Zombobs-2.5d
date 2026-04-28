use web_sys::HtmlCanvasElement;
use wgpu::{Device, Queue, Surface, SurfaceConfiguration};
use hecs::World;
use super::components::{
    GunModel,
    Parent,
    Player,
    Position,
    Renderable,
    LocalTransform,
    GlobalRotation,
    MaterialId,
    TextureAtlasRegion,
};
use wgpu::util::DeviceExt;
use glam::{Mat4, Vec3};
use image::GenericImageView;
use std::collections::HashSet;

// [TRACE: ARCHITECTURE.md]
// First-person camera system - no modes needed

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Texture {
    pub fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8], 
        label: &str
    ) -> Result<Self, String> {
        let img = image::load_from_memory(bytes).map_err(|e| e.to_string())?;
        Self::from_image(device, queue, &img, label)
    }

    pub fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        img: &image::DynamicImage,
        label: &str
    ) -> Result<Self, String> {
        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Ok(Self { texture, view, sampler })
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    fn new() -> Self {
        Self {
            view_proj: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }
}

// [TRACE: ARCHITECTURE.md]
pub struct Camera {
    pub position: glam::Vec3,  // 3D position (follows player)
    pub aspect: f32,
    // First-person camera angles
    pub yaw: f32,    // Horizontal rotation (radians)
    pub pitch: f32,  // Vertical angle (radians, clamped)
    pub fov: f32,    // Field of view (radians)
    // Cinematic effects
    pub eye_height: f32,      // Height of eyes above ground
    pub smoothing: f32,       // Camera smoothing factor (0-1)
    pub head_bob_amount: f32, // Head bob intensity
    pub head_bob_time: f32,   // Head bob animation time
    /// Screen shake: `shake_phase` advances while `shake_magnitude` decays (see `decay_shake`).
    pub shake_phase: f32,
    pub shake_magnitude: f32,
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            position: Vec3::ZERO,
            aspect: width as f32 / height as f32,
            yaw: 0.0,
            pitch: 0.0,
            fov: 1.047198,    // 60 degrees in radians (π/3)
            eye_height: 0.7,  // Eye height to align with head
            smoothing: 1.0,    // Default to stable first-person
            head_bob_amount: 0.0,
            head_bob_time: 0.0,
            shake_phase: 0.0,
            shake_magnitude: 0.0,
        }
    }

    /// Add camera kick; stacks up to a cap (gunshot, hit, explosion).
    pub fn add_shake(&mut self, magnitude: f32) {
        let m = magnitude.abs().min(0.4);
        self.shake_magnitude = (self.shake_magnitude + m).min(0.22);
    }

    pub fn decay_shake(&mut self, dt: f32) {
        if self.shake_magnitude > 0.0005 {
            self.shake_phase += dt * 62.0;
            self.shake_magnitude *= 0.9_f32.powf(dt * 18.0);
            if self.shake_magnitude < 0.002 {
                self.shake_magnitude = 0.0;
            }
        }
    }

    pub fn build_view_projection_matrix(&self) -> Mat4 {
        // Perspective projection for first-person view
        let proj = Mat4::perspective_rh(self.fov, self.aspect, 0.1, 100.0);
        
        // Calculate eye position (player position + head bob)
        // Note: self.position should already be the target "Eye Position" set by the game loop.
        // We do NOT add self.eye_height here if the game loop already sets position to head level.
        // Previously we were adding eye_height on top of position.
        
        let head_bob_z = self.head_bob_amount * (self.head_bob_time * 10.0).sin();
        let shake_x = self.shake_phase.sin() * self.shake_magnitude;
        let shake_y = (self.shake_phase * 1.27).cos() * self.shake_magnitude * 0.88;
        let eye = Vec3::new(
            self.position.x + shake_x,
            self.position.y + shake_y,
            self.position.z + head_bob_z, // Removed + self.eye_height to prevent double offset
        );
        
        // Calculate forward direction from yaw and pitch
        // In our coordinate system: X=East, Y=North, Z=Up
        // Yaw rotates around Z axis, pitch tilts up/down
        let forward_x = self.yaw.sin() * self.pitch.cos();
        let forward_y = self.yaw.cos() * self.pitch.cos();
        let forward_z = self.pitch.sin();
        
        let forward = Vec3::new(forward_x, forward_y, forward_z).normalize();
        let target = eye + forward;
        
        // World up is Z axis
        let view = Mat4::look_at_rh(eye, target, Vec3::Z);
        
        proj * view
    }
    pub fn screen_to_world(&self, screen_pos: glam::Vec2, screen_size: glam::Vec2) -> glam::Vec2 {
        let view_proj = self.build_view_projection_matrix();
        let inv_view_proj = view_proj.inverse();
        
        // Convert screen pixel to NDC (-1 to +1)
        // WebGPU NDC: X [-1, 1], Y [-1, 1] (Up), Z [0, 1]
        // HTML Canvas: Y is Down. So we flip Y.
        let ndc_x = (2.0 * screen_pos.x / screen_size.x) - 1.0;
        let ndc_y = 1.0 - (2.0 * screen_pos.y / screen_size.y);

        // Cast ray from near plane (Z=0) to far plane (Z=1)
        let ndc_near = glam::Vec4::new(ndc_x, ndc_y, 0.0, 1.0);
        let ndc_far = glam::Vec4::new(ndc_x, ndc_y, 1.0, 1.0);
        
        // Unproject to World
        let world_near = inv_view_proj * ndc_near;
        let world_far = inv_view_proj * ndc_far;
        
        let p_near = world_near.truncate() / world_near.w;
        let p_far = world_far.truncate() / world_far.w;
        
        // Ray Direction
        let dir = (p_far - p_near).normalize();
        
        // Intersect with Plane Z = 0 (ground plane)
        if dir.z.abs() < 0.001 {
            // Ray is parallel to plane - return camera position projected to ground
            return glam::Vec2::new(self.position.x, self.position.y);
        }
        
        let t = -p_near.z / dir.z;
        let intersection = p_near + dir * t;
        
        glam::Vec2::new(intersection.x, intersection.y)
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct InstanceRaw {
    model_pos: [f32; 3],
    size: [f32; 3],
    rotation: [f32; 4], // Quaternion (x, y, z, w)
    color: [f32; 4],
    material_id: u32,
    uv_origin: [u32; 2], // x, y
    uv_dims: [u32; 3],   // w, h, d
}

impl InstanceRaw {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                // @location(1) model_pos: vec3<f32>
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // @location(2) size: vec3<f32>
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // @location(3) rotation: vec4<f32>
                wgpu::VertexAttribute {
                    offset: (mem::size_of::<[f32; 3]>() * 2) as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x4,
                },
                // @location(4) color: vec4<f32>
                wgpu::VertexAttribute {
                    offset: (mem::size_of::<[f32; 3]>() * 2 + mem::size_of::<[f32; 4]>()) as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x4,
                },
                // @location(5) material_id: u32
                wgpu::VertexAttribute {
                    offset: (mem::size_of::<[f32; 3]>() * 2 + mem::size_of::<[f32; 4]>() * 2) as wgpu::BufferAddress,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Uint32,
                },
                // @location(6) uv_origin: vec2<u32>
                wgpu::VertexAttribute {
                    offset: (mem::size_of::<[f32; 3]>() * 2 + mem::size_of::<[f32; 4]>() * 2 + mem::size_of::<u32>()) as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Uint32x2,
                },
                // @location(7) uv_dims: vec3<u32>
                wgpu::VertexAttribute {
                    offset: (mem::size_of::<[f32; 3]>() * 2 + mem::size_of::<[f32; 4]>() * 2 + mem::size_of::<u32>() + mem::size_of::<[u32; 2]>()) as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Uint32x3,
                },
            ],
        }
    }
}

pub struct Renderer {
    surface: Surface<'static>,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    pub size: (u32, u32),
    pipeline: wgpu::RenderPipeline,
    instance_buffer: wgpu::Buffer,
    instance_capacity: usize,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    depth_texture: wgpu::Texture,
    depth_view: wgpu::TextureView,
    diffuse_bind_group: wgpu::BindGroup,
    zombob_bind_group: wgpu::BindGroup,
}

impl Renderer {
    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    pub fn create_depth_texture(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration, label: &str) -> (wgpu::Texture, wgpu::TextureView) {
        let size = wgpu::Extent3d {
            width: config.width,
            height: config.height,
            depth_or_array_layers: 1,
        };
        let desc = wgpu::TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        };
        let texture = device.create_texture(&desc);
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        (texture, view)
    }

    pub async fn new(canvas: HtmlCanvasElement) -> Result<Self, String> {
        let width = canvas.width();
        let height = canvas.height();

        let instance = wgpu::Instance::default();
        
        let surface = instance.create_surface(wgpu::SurfaceTarget::Canvas(canvas)).map_err(|e| e.to_string())?;
        
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }).await.ok_or("Failed to find an appropriate adapter")?;

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::default(),
            },
            None,
        ).await.map_err(|e| e.to_string())?;

        let surface_caps = surface.get_capabilities(&adapter);
        let texture_format = surface_caps.formats.iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: texture_format,
            width,
            height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        // Camera Uniform
        let camera_uniform = CameraUniform::new();
        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("camera_bind_group_layout"),
        });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        // Load Skin Texture (Minecraft Skin for Jeff)
        let diffuse_bytes = include_bytes!("../../minecraft_skin.png");
        let diffuse_texture = Texture::from_bytes(&device, &queue, diffuse_bytes, "minecraft_skin.png").unwrap();

        // Load Zombob Skin (Enemy Texture)
        let zombob_skin_bytes = include_bytes!("../../zombob_skin.png");
        let zombob_texture = Texture::from_bytes(&device, &queue, zombob_skin_bytes, "zombob_skin.png").unwrap();

        let texture_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    // This should match the filterable: true above
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("texture_bind_group_layout"),
        });

        let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                },
            ],
            label: Some("diffuse_bind_group"),
        });

        let zombob_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&zombob_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&zombob_texture.sampler),
                },
            ],
            label: Some("zombob_bind_group"),
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&camera_bind_group_layout, &texture_bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[InstanceRaw::desc()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: Self::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        // Initial capacity for instances
        let instance_capacity = 1000;
        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Instance Buffer"),
            size: (instance_capacity * std::mem::size_of::<InstanceRaw>()) as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let (depth_texture, depth_view) = Self::create_depth_texture(&device, &config, "depth_texture");

        Ok(Self {
            surface,
            device,
            queue,
            config,
            size: (width, height),
            pipeline,
            instance_buffer,
            instance_capacity,
            camera_buffer,
            camera_bind_group,
            depth_texture,
            depth_view,
            diffuse_bind_group,
            zombob_bind_group,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.size = (width, height);
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
            let (depth_texture, depth_view) = Self::create_depth_texture(&self.device, &self.config, "depth_texture");
            self.depth_texture = depth_texture;
            self.depth_view = depth_view;
        }
    }

    pub fn render(&mut self, world: &World, camera: &Camera) -> Result<(), String> {
        let output = self.surface.get_current_texture().map_err(|e| e.to_string())?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Update Camera Buffer
        let mut camera_uniform = CameraUniform::new();
        camera_uniform.view_proj = camera.build_view_projection_matrix().to_cols_array_2d();
        self.queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[camera_uniform]));

        // 1. Collect instances from World
        // Hide player body for true first-person (keep gun model visible)
        let mut hidden_entities: HashSet<hecs::Entity> = HashSet::new();
        for (id, _player) in world.query::<&Player>().iter() {
            hidden_entities.insert(id);
        }
        let mut added = true;
        while added {
            added = false;
            for (id, parent) in world.query::<&Parent>().iter() {
                if hidden_entities.contains(&parent.entity) && !hidden_entities.contains(&id) {
                    hidden_entities.insert(id);
                    added = true;
                }
            }
        }

        let mut instances = Vec::new();
        let mut zombob_instances = Vec::new();
        
        // Query for everything renderable
        for (id, (pos, renderable, global_rot, local_transform, material_id, texture_region, gun_model)) in world.query::<(&Position, &Renderable, Option<&GlobalRotation>, Option<&LocalTransform>, Option<&MaterialId>, Option<&TextureAtlasRegion>, Option<&GunModel>)>().iter() {
            if hidden_entities.contains(&id) && gun_model.is_none() {
                continue;
            }
            let rotation = if let Some(gr) = global_rot {
                gr.rotation.to_array()
            } else if let Some(lt) = local_transform {
                lt.rotation.to_array()
            } else {
                [0.0, 0.0, 0.0, 1.0]
            };

            let mat_id = material_id.map(|m| *m as u32).unwrap_or(MaterialId::Default as u32);
            
            let (uv_origin, uv_dims) = if let Some(tr) = texture_region {
                ([tr.x, tr.y], [tr.w, tr.h, tr.d])
            } else {
                ([0, 0], [0, 0, 0])
            };

            let instance = InstanceRaw {
                model_pos: [pos.x, pos.y, pos.z],
                size: [renderable.width, renderable.height, renderable.depth],
                rotation,
                color: renderable.color,
                material_id: mat_id,
                uv_origin,
                uv_dims,
            };

            // Separate instances by texture type
            // Check if this entity or its root parent is a Zombob
            // Optimization: For now, we assume if it has texture regions and IS NOT hidden (player), it might be a Zombob?
            // Better: We need to know if this entity belongs to a Zombob.
            // Complex hierarchy check is slow here. 
            // Quick hack: Zombobs have Green skin color [0.2, 0.8, 0.2, 1.0] or Red shirt [0.6, 0.2, 0.2, 1.0]
            // This is brittle but fast for now.
            // TODO: Add explicit Component or Material ID for ZombobSkin
            
            let is_zombob_part = (renderable.color[1] > 0.7 && renderable.color[0] < 0.3) || // Green skin
                                 (renderable.color[0] > 0.5 && renderable.color[1] < 0.3);   // Red shirt
            
            if is_zombob_part && texture_region.is_some() {
                zombob_instances.push(instance);
            } else {
                instances.push(instance);
            }
        }
        
        // Merge lists for buffer update (we'll use draw ranges later if we want strict separation, 
        // but for now we just want to bind different textures.
        // Wait, WebGPU Instancing requires same bind group for the draw call.
        // We need TWO draw calls.
        // 1. Draw Default/Player (Diffuse Texture)
        // 2. Draw Zombobs (Zombob Texture)
        
        // We need to write ALL instances to the buffer, but know the offset/count for each group.
        let default_count = instances.len();
        let zombob_count = zombob_instances.len();
        
        instances.extend(zombob_instances);


        // 2. Resize buffer if needed
        if instances.len() > self.instance_capacity {
            self.instance_capacity = instances.len().max(self.instance_capacity * 2);
            self.instance_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Instance Buffer"),
                size: (self.instance_capacity * std::mem::size_of::<InstanceRaw>()) as wgpu::BufferAddress,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }

        // 3. Update buffer
        if !instances.is_empty() {
            self.queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&instances));
        }

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.1,
                            b: 0.1,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            
            // 1. Draw Default Instances (Player, Environment, Props) using Diffuse Texture
            if default_count > 0 {
                render_pass.set_bind_group(1, &self.diffuse_bind_group, &[]);
                // Slice: Start 0, Length default_count * stride
                let start_offset = 0;
                let end_offset = (default_count * std::mem::size_of::<InstanceRaw>()) as u64;
                render_pass.set_vertex_buffer(0, self.instance_buffer.slice(start_offset..end_offset));
                render_pass.draw(0..36, 0..default_count as u32);
            }

            // 2. Draw Zombob Instances using Zombob Texture
            if zombob_count > 0 {
                render_pass.set_bind_group(1, &self.zombob_bind_group, &[]);
                // Slice: Start default_count * stride, Length zombob_count * stride
                let start_offset = (default_count * std::mem::size_of::<InstanceRaw>()) as u64;
                let end_offset = ((default_count + zombob_count) * std::mem::size_of::<InstanceRaw>()) as u64;
                render_pass.set_vertex_buffer(0, self.instance_buffer.slice(start_offset..end_offset));
                render_pass.draw(0..36, 0..zombob_count as u32);
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
