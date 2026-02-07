// Vertex shader

struct CameraUniform {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(1) @binding(1)
var s_diffuse: sampler;

struct InstanceInput {
    @location(1) model_pos: vec3<f32>,
    @location(2) size: vec3<f32>,
    @location(3) rotation: vec4<f32>,
    @location(4) color: vec4<f32>,
    @location(5) material_id: u32,
    @location(6) uv_origin: vec2<u32>,
    @location(7) uv_dims: vec3<u32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) world_pos: vec3<f32>,
    @location(2) @interpolate(flat) material_id: u32,
    @location(3) tex_coords: vec2<f32>,
    @location(4) @interpolate(flat) use_texture: u32,
};

fn q_transform(q: vec4<f32>, v: vec3<f32>) -> vec3<f32> {
    let t = 2.0 * cross(q.xyz, v);
    return v + q.w * t + cross(q.xyz, t);
}

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;
    
    // Cube vertices (Z-UP Coordinate System)
    // Centered at 0,0,0 with size 1x1x1
    var pos = array<vec3<f32>, 36>(
        // Top Face (Z+) -> Brightness 1.2
        vec3(-0.5, -0.5,  0.5), vec3( 0.5, -0.5,  0.5), vec3( 0.5,  0.5,  0.5),
        vec3( 0.5,  0.5,  0.5), vec3(-0.5,  0.5,  0.5), vec3(-0.5, -0.5,  0.5),
        
        // Bottom Face (Z-) -> Brightness 0.3
        vec3(-0.5, -0.5, -0.5), vec3( 0.5,  0.5, -0.5), vec3( 0.5, -0.5, -0.5),
        vec3( 0.5,  0.5, -0.5), vec3(-0.5, -0.5, -0.5), vec3(-0.5,  0.5, -0.5),
        
        // Front Face (Y+) -> Brightness 1.0
        vec3(-0.5,  0.5, -0.5), vec3( 0.5,  0.5,  0.5), vec3( 0.5,  0.5, -0.5),
        vec3( 0.5,  0.5,  0.5), vec3(-0.5,  0.5, -0.5), vec3(-0.5,  0.5,  0.5),
        
        // Back Face (Y-) -> Brightness 0.5
        vec3(-0.5, -0.5, -0.5), vec3( 0.5, -0.5, -0.5), vec3( 0.5, -0.5,  0.5),
        vec3( 0.5, -0.5,  0.5), vec3(-0.5, -0.5,  0.5), vec3(-0.5, -0.5, -0.5),
        
        // Right Face (X+) -> Brightness 0.8
        vec3( 0.5, -0.5, -0.5), vec3( 0.5,  0.5, -0.5), vec3( 0.5,  0.5,  0.5),
        vec3( 0.5,  0.5,  0.5), vec3( 0.5, -0.5,  0.5), vec3( 0.5, -0.5, -0.5),
        
        // Left Face (X-) -> Brightness 0.8
        vec3(-0.5, -0.5, -0.5), vec3(-0.5,  0.5,  0.5), vec3(-0.5,  0.5, -0.5),
        vec3(-0.5,  0.5,  0.5), vec3(-0.5, -0.5, -0.5), vec3(-0.5, -0.5,  0.5)
    );
    
    let base_pos = pos[in_vertex_index];
    
    // Scale
    let scaled_pos = base_pos * instance.size;
    
    // Rotate
    let rotated_pos = q_transform(instance.rotation, scaled_pos);
    
    // Translate
    let world_pos = rotated_pos + instance.model_pos;
    
    // Apply Camera View-Projection Matrix
    out.clip_position = camera.view_proj * vec4<f32>(world_pos, 1.0);
    out.world_pos = world_pos;
    out.material_id = instance.material_id;
    
    // UV Mapping Logic
    var uv = vec2<f32>(0.0, 0.0);
    var use_tex = 0u;
    
    if (instance.uv_dims.x > 0u) {
        use_tex = 1u;
        let u = f32(instance.uv_origin.x);
        let v = f32(instance.uv_origin.y);
        let w = f32(instance.uv_dims.x);
        let h = f32(instance.uv_dims.y);
        let d = f32(instance.uv_dims.z);
        
        // Vertex index in face (0-5)
        let idx = in_vertex_index % 6u;
        
        // UV offsets for the quad (0,0 is Top-Left of region)
        // 0: BL, 1: BR, 2: TR, 3: TR, 4: TL, 5: BL
        // Map to (0,1), (1,1), (1,0), (1,0), (0,0), (0,1) relative to region size
        
        var face_u = 0.0;
        var face_v = 0.0;
        
        if (idx == 0u || idx == 5u) { face_u = 0.0; face_v = 1.0; } // BL
        else if (idx == 1u) { face_u = 1.0; face_v = 1.0; } // BR
        else if (idx == 2u || idx == 3u) { face_u = 1.0; face_v = 0.0; } // TR
        else if (idx == 4u) { face_u = 0.0; face_v = 0.0; } // TL
        
        // Calculate Region Origin and Size based on Face
        var region_x = 0.0;
        var region_y = 0.0;
        var region_w = 0.0;
        var region_h = 0.0;
        
        if (in_vertex_index < 6u) { 
            // Top Face (Z+)
            region_x = u + d; region_y = v; 
            region_w = w; region_h = d;
        }
        else if (in_vertex_index < 12u) { 
            // Bottom Face (Z-)
            region_x = u + d + w; region_y = v; 
            region_w = w; region_h = d;
        }
        else if (in_vertex_index < 18u) { 
            // Front Face (Y+)
            region_x = u + d; region_y = v + d; 
            region_w = w; region_h = h;
        }
        else if (in_vertex_index < 24u) { 
            // Back Face (Y-)
            region_x = u + d + w + d; region_y = v + d; 
            region_w = w; region_h = h;
        }
        else if (in_vertex_index < 30u) { 
            // Right Face (X+)
            region_x = u; region_y = v + d; 
            region_w = d; region_h = h;
        }
        else { 
            // Left Face (X-)
            region_x = u + d + w; region_y = v + d; 
            region_w = d; region_h = h;
        }
        
        // Final UV
        let tex_x = region_x + face_u * region_w;
        let tex_y = region_y + face_v * region_h;
        
        uv = vec2<f32>(tex_x / 64.0, tex_y / 64.0);
    }
    
    out.tex_coords = uv;
    out.use_texture = use_tex;

    // Fake Directional Lighting (Top Down ish)
    var brightness = 1.0;
    if (in_vertex_index < 6u) { brightness = 1.35; } // Top
    else if (in_vertex_index < 12u) { brightness = 0.15; } // Bottom
    else if (in_vertex_index < 18u) { brightness = 1.05; } // Front
    else if (in_vertex_index < 24u) { brightness = 0.45; } // Back
    else { brightness = 0.7; } // Sides
    
    out.color = vec4<f32>(instance.color.rgb * brightness, instance.color.a);
    return out;
}

// Pseudo-random function
fn hash(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2<f32>(12.9898, 78.233))) * 43758.5453);
}

fn noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    
    // Four corners
    let a = hash(i);
    let b = hash(i + vec2<f32>(1.0, 0.0));
    let c = hash(i + vec2<f32>(0.0, 1.0));
    let d = hash(i + vec2<f32>(1.0, 1.0));
    
    // Smooth interpolation
    let u = f * f * (3.0 - 2.0 * f);
    return mix(a, b, u.x) + (c - a) * u.y * (1.0 - u.x) + (d - b) * u.x * u.y;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let mat_id = in.material_id;
    
    // Material ID 2 = Shadow (preserve dark color)
    if (mat_id == 2u) {
        return vec4<f32>(0.03, 0.03, 0.03, 1.0);
    }
    
    // Material ID 1 = Ground (procedural ground shading)
    if (mat_id == 1u) {
        let uv = in.world_pos.xy;
        
        // Layer 1: Low frequency noise (large macro regions)
        let n1 = noise(uv * 0.25);
        
        // Layer 2: Mid frequency noise (texture detail)
        let n2 = noise(uv * 1.8);
        
        // Quantize noise into bands for crisper regions
        let n1_banded = floor(n1 * 4.0) / 4.0;
        let n2_banded = floor(n2 * 3.0) / 3.0;
        
        // Combine noise layers with refined weights
        let combined_noise = n1_banded * 0.65 + n2_banded * 0.35;
        
        // High contrast palette with packed soil tone
        let grass = vec3<f32>(0.24, 0.55, 0.26);  // Brighter, more saturated green
        let dirt = vec3<f32>(0.40, 0.30, 0.22);   // Warmer, richer brown
        let packed_soil = vec3<f32>(0.22, 0.16, 0.12); // Dark packed earth tone
        
        // Crisp transitions with very tight thresholds (2% blend range)
        let grass_mask = smoothstep(0.49, 0.51, combined_noise);
        let dirt_mask = smoothstep(0.33, 0.49, combined_noise);
        
        // Mix colors based on masks
        var base_color = mix(packed_soil, dirt, dirt_mask);
        base_color = mix(base_color, grass, grass_mask);
        
        // Edge ring definition at boundaries (stronger for crispness)
        let edge_threshold_low = 0.47;
        let edge_threshold_high = 0.53;
        let edge_mask = smoothstep(edge_threshold_low, edge_threshold_low + 0.015, combined_noise) * 
                        (1.0 - smoothstep(edge_threshold_high - 0.015, edge_threshold_high, combined_noise));
        // Darken edge for crisp definition
        base_color = mix(base_color, base_color * 0.80, edge_mask * 0.5);
        
        // Add subtle grid for scale reference (faint, only in dirt areas)
        let grid_pos = uv;
        let grid_x = step(0.99, fract(grid_pos.x));
        let grid_y = step(0.99, fract(grid_pos.y));
        let grid = grid_x + grid_y;
        if (grid > 0.0) {
            // Grid is more visible in dirt, less in grass
            let grid_strength = mix(0.7, 0.85, dirt_mask);
            base_color = base_color * grid_strength;
        }
        
        // Distance-based vignette (darker at edges)
        let dist_from_center = length(in.world_pos.xy);
        let vignette = 1.0 - smoothstep(8.0, 15.0, dist_from_center) * 0.15;
        base_color = base_color * vignette;
        
        return vec4<f32>(base_color, 1.0);
    }
    
    // Material ID 0 = Default (actors, props)
    
    var base_color = in.color;
    if (in.use_texture == 1u) {
        // Use textureSampleLevel to avoid non-uniform control flow error
        // Mip level 0 is perfect for pixel art minecraft skins
        let tex = textureSampleLevel(t_diffuse, s_diffuse, in.tex_coords, 0.0);
        base_color = tex * base_color;
    }
    
    // Restore original color (brightness was applied in vertex)
    var final_color = vec3<f32>(base_color.rgb);
    
    // Slight actor lift to separate from ground
    final_color = min(final_color * 1.05, vec3<f32>(1.0));
    return vec4<f32>(final_color, 1.0);
}
