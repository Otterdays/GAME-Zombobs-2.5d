use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use hecs::World;
use crate::engine::renderer::{Renderer, Camera};
use crate::engine::components::{
    CharacterMotor, Debris, GunModel, GlobalRotation, Health, IsShooting, Jeff, LocalTransform,
    MaterialId, MuzzleFlash, Parent, Player, PlayerStats, Position, Projectile, Renderable,
    TextureAtlasRegion, Velocity, Weapon, Zombob, BodyPart, BodyPartType, AABB,
};
use crate::engine::systems::{movement_system, player_input_system, transform_propagation_system, animation_system, projectile_system, weapon_system, flash_system, update_gun_model};
use crate::engine::input::{InputState, Keybinds};
use rand::Rng;
use glam::{Vec3, Quat};

const SHADOW_COLOR: [f32; 4] = [0.04, 0.04, 0.04, 1.0];

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = setWave)]
    fn set_wave_js(wave: u32);

    #[wasm_bindgen(js_namespace = window, js_name = showGameOver)]
    fn show_game_over_js(wave: u32, kills: u32, score: u32);

    #[wasm_bindgen(js_namespace = window, js_name = resetRunUi)]
    fn reset_run_ui_js();
}

fn spawn_follow_shadow(world: &mut World, parent: hecs::Entity, width: f32, height: f32, z_offset: f32) {
    // [TRACE: ROADMAP.md]
    world.spawn((
        Position::new_3d(0.0, 0.0, 0.0),
        LocalTransform::new(Vec3::new(0.0, 0.0, z_offset), Quat::IDENTITY, Vec3::ONE),
        Parent { entity: parent },
        Renderable::new_3d(width, height, 0.02, SHADOW_COLOR),
        MaterialId::Shadow,
    ));
}

fn spawn_static_shadow(world: &mut World, pos: Vec3, width: f32, height: f32, z_offset: f32) {
    // [TRACE: ROADMAP.md]
    world.spawn((
        Position::new_3d(pos.x, pos.y, pos.z + z_offset),
        Renderable::new_3d(width, height, 0.02, SHADOW_COLOR),
        MaterialId::Shadow,
    ));
}

pub struct ZombobConfig {
    pub skin_color: [f32; 4],
    pub shirt_color: [f32; 4],
    pub pants_color: [f32; 4],
    pub shoe_color: [f32; 4],
    pub scale: f32,
    pub use_texture: bool,
}

impl ZombobConfig {
    pub fn default() -> Self {
        Self {
            skin_color: [0.96, 0.76, 0.62, 1.0], // Human Skin (Peach)
            shirt_color: [0.35, 0.4, 0.45, 1.0], // Tactical Grey-Blue
            pants_color: [0.25, 0.25, 0.23, 1.0], // Dark Grey/Brown
            shoe_color: [0.1, 0.1, 0.1, 1.0], // Black Combat Boots
            scale: 0.6, // Smaller "Survivor" scale
            use_texture: true,
        }
    }
}

// Spawn the player character "Jeff"
pub fn spawn_jeff(world: &mut World, start_pos: Vec3) -> hecs::Entity {
    // Jeff Config: Uses skin texture, Survivor scale
    let config = ZombobConfig::default();
    spawn_character_mesh(world, start_pos, config)
}

// Spawn the enemy "Zombob"
    pub fn spawn_zombob(world: &mut World, start_pos: Vec3) -> hecs::Entity {
        // Zombob Config: Green skin, Red shirt, Uses texture (zombob_skin.png)
        let mut config = ZombobConfig::default();
        config.skin_color = [0.2, 0.8, 0.2, 1.0]; // Green Skin (base color)
        config.shirt_color = [0.6, 0.2, 0.2, 1.0]; // Red Shirt (base color)
        config.pants_color = [0.1, 0.1, 0.1, 1.0]; // Black Pants (base color)
        config.use_texture = true; // Use texture for Zombob
        
        spawn_character_mesh(world, start_pos, config)
    }

fn spawn_zombob_wave(world: &mut World, rng: &mut impl Rng, count: u32, wave: u32) {
    let mut spawned = 0;
    let mut attempts = 0;
    let max_attempts = count * 10;

    while spawned < count && attempts < max_attempts {
        attempts += 1;
        let x: f32 = rng.gen_range(-18.0..18.0);
        let y: f32 = rng.gen_range(-18.0..18.0);
        if x.abs() < 5.0 && y.abs() < 5.0 {
            continue;
        }

        let zombie = spawn_zombob(world, Vec3::new(x, y, 0.0));
        world.insert_one(zombie, Zombob).unwrap();

        let health =
            (68.0 + rng.gen_range(0.0..36.0) + wave as f32 * 6.8).min(178.0);
        world.insert_one(zombie, Health::new(health)).unwrap();

        let speed = (1.05 + rng.gen_range(0.0..1.35) + wave as f32 * 0.09).min(3.85);
        let damage = (7.25 + wave as f32 * 0.92).min(23.5);
        world
            .insert_one(
                zombie,
                crate::engine::components::SimpleAI {
                    move_speed: speed,
                    detection_radius: 16.0,
                    damage,
                    attack_cooldown: 0.0,
                },
            )
            .unwrap();
        spawned += 1;
    }
}

/// Matches `spawn_character_mesh` root Z offset for default survivor scale (0.6).
const PLAYER_SPAWN_SCALE: f32 = 0.6;
const JEFF_REST_Z: f32 = 0.55 * PLAYER_SPAWN_SCALE;

fn despawn_entity_subtree(world: &mut World, root: hecs::Entity) {
    let mut children = Vec::new();
    for (entity, parent) in world.query::<&Parent>().iter() {
        if parent.entity == root {
            children.push(entity);
        }
    }
    for child in children {
        despawn_entity_subtree(world, child);
    }
    let _ = world.despawn(root);
}

fn despawn_all_ephemeral(world: &mut World) {
    let mut projectile_ids: Vec<hecs::Entity> =
        world.query::<&Projectile>().iter().map(|(e, _)| e).collect();
    let mut flashes: Vec<hecs::Entity> =
        world.query::<&MuzzleFlash>().iter().map(|(e, _)| e).collect();
    let mut debris: Vec<hecs::Entity> = world.query::<&Debris>().iter().map(|(e, _)| e).collect();

    projectile_ids.reverse();
    for id in projectile_ids {
        let _ = world.despawn(id);
    }
    flashes.reverse();
    for id in flashes {
        let _ = world.despawn(id);
    }
    debris.reverse();
    for id in debris {
        let _ = world.despawn(id);
    }
}

// Internal helper to build the mesh hierarchy
fn spawn_character_mesh(world: &mut World, start_pos: Vec3, config: ZombobConfig) -> hecs::Entity {
    let s = config.scale;
    let white = [1.0, 1.0, 1.0, 1.0];
    
    // Spawn Root (Torso is the center of mass)
    // Torso: 8x12x4 -> (16, 16)
    let torso_color = if config.use_texture { white } else { config.shirt_color };
    
    let mut root_builder = hecs::EntityBuilder::new();
    // Lift the root (Torso) up by half the leg height (approx) + half torso height to sit on top of legs?
    // Legs are 0.4 * s height.
    // Torso is 0.5 * s height.
    // Leg offset is -0.35 * s.
    // If root is at 0, legs are at -0.35*s. 
    // Leg height is 0.4*s, so they extend from -0.35*s - 0.2*s to -0.35*s + 0.2*s.
    // Bottom of leg is -0.55*s.
    // To put feet on ground (Z=0), Root needs to be at +0.55*s.
    let z_offset = 0.55 * s; 
    
    root_builder.add(Position::new_3d(start_pos.x, start_pos.y, start_pos.z + z_offset));
    root_builder.add(Velocity::new(0.0, 0.0));
    root_builder.add(GlobalRotation::new(Quat::IDENTITY));
    root_builder.add(Renderable::new_3d(0.4 * s, 0.25 * s, 0.5 * s, torso_color));
    root_builder.add(AABB::new(0.4, 0.4));
    root_builder.add(BodyPart { part_type: BodyPartType::Torso });
    if config.use_texture {
        root_builder.add(TextureAtlasRegion { x: 16, y: 16, w: 8, h: 12, d: 4 });
    }
    let root = world.spawn(root_builder.build());

    spawn_follow_shadow(world, root, 0.6 * s, 0.35 * s, -0.5 * s);

    // Head (Attached to Torso, Offset Z+ (Up))
    // Head: 8x8x8 -> (0, 0)
    let head_color = if config.use_texture { white } else { config.skin_color };
    let mut head_builder = hecs::EntityBuilder::new();
    head_builder.add(Position::new_3d(0.0, 0.0, 0.0));
    head_builder.add(LocalTransform::new(Vec3::new(0.0, 0.0, 0.4 * s), Quat::IDENTITY, Vec3::ONE * s));
    head_builder.add(Parent { entity: root });
    head_builder.add(Renderable::new_3d(0.3 * s, 0.3 * s, 0.3 * s, head_color));
    head_builder.add(BodyPart { part_type: BodyPartType::Head });
    if config.use_texture {
        head_builder.add(TextureAtlasRegion { x: 0, y: 0, w: 8, h: 8, d: 8 });
    }
    let head = world.spawn(head_builder.build());

    if !config.use_texture {
        // Eyes (Y+ is Forward face of Head)
        world.spawn((
            Position::new_3d(0.0, 0.0, 0.0),
            LocalTransform::new(Vec3::new(-0.08 * s, 0.16 * s, 0.05 * s), Quat::IDENTITY, Vec3::ONE * s),
            Parent { entity: head },
            Renderable::new_3d(0.06 * s, 0.02 * s, 0.06 * s, [1.0, 1.0, 1.0, 1.0]), // White Eye
        ));
        world.spawn((
            Position::new_3d(0.0, 0.0, 0.0),
            LocalTransform::new(Vec3::new(0.08 * s, 0.16 * s, 0.05 * s), Quat::IDENTITY, Vec3::ONE * s),
            Parent { entity: head },
            Renderable::new_3d(0.06 * s, 0.02 * s, 0.06 * s, [1.0, 1.0, 1.0, 1.0]), // White Eye
        ));

        // Pupils
        world.spawn((
            Position::new_3d(0.0, 0.0, 0.0),
            LocalTransform::new(Vec3::new(-0.08 * s, 0.171 * s, 0.05 * s), Quat::IDENTITY, Vec3::ONE * s),
            Parent { entity: head },
            Renderable::new_3d(0.02 * s, 0.01 * s, 0.02 * s, [0.0, 0.0, 0.0, 1.0]), // Black Pupil
        ));
        world.spawn((
            Position::new_3d(0.0, 0.0, 0.0),
            LocalTransform::new(Vec3::new(0.08 * s, 0.171 * s, 0.05 * s), Quat::IDENTITY, Vec3::ONE * s),
            Parent { entity: head },
            Renderable::new_3d(0.02 * s, 0.01 * s, 0.02 * s, [0.0, 0.0, 0.0, 1.0]), // Black Pupil
        ));
    }

    // Left Arm (Attached to Torso, Offset X-, Z+)
    // L Arm: 4x12x4 -> (32, 48)
    let arm_color = if config.use_texture { white } else { config.skin_color };
    let mut l_arm_builder = hecs::EntityBuilder::new();
    l_arm_builder.add(Position::new_3d(0.0, 0.0, 0.0));
    l_arm_builder.add(LocalTransform::new(Vec3::new(-0.35 * s, 0.0, 0.15 * s), Quat::IDENTITY, Vec3::ONE * s));
    l_arm_builder.add(Parent { entity: root });
    l_arm_builder.add(Renderable::new_3d(0.15 * s, 0.15 * s, 0.4 * s, arm_color));
    l_arm_builder.add(BodyPart { part_type: BodyPartType::LeftArm });
    if config.use_texture {
        l_arm_builder.add(TextureAtlasRegion { x: 32, y: 48, w: 4, h: 12, d: 4 });
    }
    let left_arm = world.spawn(l_arm_builder.build());

    if !config.use_texture {
        // Left Hand (Attached to Arm, Z-)
        world.spawn((
            Position::new_3d(0.0, 0.0, 0.0),
            LocalTransform::new(Vec3::new(0.0, 0.0, -0.22 * s), Quat::IDENTITY, Vec3::ONE * s),
            Parent { entity: left_arm },
            Renderable::new_3d(0.14 * s, 0.14 * s, 0.14 * s, [config.skin_color[0]*0.9, config.skin_color[1]*0.9, config.skin_color[2]*0.9, 1.0]),
        ));
    }

    // Right Arm (Attached to Torso, Offset X+, Z+)
    // R Arm: 4x12x4 -> (40, 16)
    let mut r_arm_builder = hecs::EntityBuilder::new();
    r_arm_builder.add(Position::new_3d(0.0, 0.0, 0.0));
    r_arm_builder.add(LocalTransform::new(Vec3::new(0.35 * s, 0.0, 0.15 * s), Quat::IDENTITY, Vec3::ONE * s));
    r_arm_builder.add(Parent { entity: root });
    r_arm_builder.add(Renderable::new_3d(0.15 * s, 0.15 * s, 0.4 * s, arm_color));
    r_arm_builder.add(BodyPart { part_type: BodyPartType::RightArm });
    if config.use_texture {
        r_arm_builder.add(TextureAtlasRegion { x: 40, y: 16, w: 4, h: 12, d: 4 });
    }
    let right_arm = world.spawn(r_arm_builder.build());

    if !config.use_texture {
        // Right Hand
        world.spawn((
            Position::new_3d(0.0, 0.0, 0.0),
            LocalTransform::new(Vec3::new(0.0, 0.0, -0.22 * s), Quat::IDENTITY, Vec3::ONE * s),
            Parent { entity: right_arm },
            Renderable::new_3d(0.14 * s, 0.14 * s, 0.14 * s, [config.skin_color[0]*0.9, config.skin_color[1]*0.9, config.skin_color[2]*0.9, 1.0]),
        ));
    }

    // Left Leg (Attached to Torso, Offset X-, Z- (Under torso))
    // L Leg: 4x12x4 -> (16, 48)
    // Leg Top is at -0.15s relative to root. Torso bottom is -0.25s. Overlap 0.1s.
    // Move legs down to fix clipping?
    // Torso: Center 0, Height 0.5s -> Top 0.25s, Bottom -0.25s.
    // Leg: Height 0.4s. Center should be at -0.25s - 0.2s = -0.45s to just touch.
    // Let's set it to -0.4s to have small overlap (0.05s) instead of 0.1s.
    let leg_y_offset = -0.4 * s; 
    
    let leg_color = if config.use_texture { white } else { config.pants_color };
    let mut l_leg_builder = hecs::EntityBuilder::new();
    l_leg_builder.add(Position::new_3d(0.0, 0.0, 0.0));
    l_leg_builder.add(LocalTransform::new(Vec3::new(-0.1 * s, 0.0, leg_y_offset), Quat::IDENTITY, Vec3::ONE * s));
    l_leg_builder.add(Parent { entity: root });
    l_leg_builder.add(Renderable::new_3d(0.15 * s, 0.15 * s, 0.4 * s, leg_color));
    l_leg_builder.add(BodyPart { part_type: BodyPartType::LeftLeg });
    if config.use_texture {
        l_leg_builder.add(TextureAtlasRegion { x: 16, y: 48, w: 4, h: 12, d: 4 });
    }
    let left_leg = world.spawn(l_leg_builder.build());
    
    if !config.use_texture {
        // Left Foot (Z-)
        world.spawn((
            Position::new_3d(0.0, 0.0, 0.0),
            LocalTransform::new(Vec3::new(0.0, 0.05 * s, -0.22 * s), Quat::IDENTITY, Vec3::ONE * s),
            Parent { entity: left_leg },
            Renderable::new_3d(0.16 * s, 0.2 * s, 0.1 * s, config.shoe_color), 
        ));
    }

    // Right Leg
    // R Leg: 4x12x4 -> (0, 16)
    let mut r_leg_builder = hecs::EntityBuilder::new();
    r_leg_builder.add(Position::new_3d(0.0, 0.0, 0.0));
    r_leg_builder.add(LocalTransform::new(Vec3::new(0.1 * s, 0.0, leg_y_offset), Quat::IDENTITY, Vec3::ONE * s));
    r_leg_builder.add(Parent { entity: root });
    r_leg_builder.add(Renderable::new_3d(0.15 * s, 0.15 * s, 0.4 * s, leg_color));
    r_leg_builder.add(BodyPart { part_type: BodyPartType::RightLeg });
    if config.use_texture {
        r_leg_builder.add(TextureAtlasRegion { x: 0, y: 16, w: 4, h: 12, d: 4 });
    }
    let right_leg = world.spawn(r_leg_builder.build());

    if !config.use_texture {
        // Right Foot
        world.spawn((
            Position::new_3d(0.0, 0.0, 0.0),
            LocalTransform::new(Vec3::new(0.0, 0.05 * s, -0.22 * s), Quat::IDENTITY, Vec3::ONE * s),
            Parent { entity: right_leg },
            Renderable::new_3d(0.16 * s, 0.2 * s, 0.1 * s, config.shoe_color),
        ));
    }

    root // Return root entity
}

#[wasm_bindgen]
pub struct GameEngine {
    world: World,
    renderer: Renderer,
    camera: Camera,
    last_time: f64,
    input_state: InputState,
    keybinds: Keybinds,
    left_handed: bool,
    mouse_sensitivity: f32,
    fov: f32,
    wave: u32,
    wave_spawn_cooldown: f32,
    player_entity: hecs::Entity,
    player_dead: bool,
    game_over_announced: bool,
}

impl GameEngine {
    pub async fn new(canvas: HtmlCanvasElement) -> Result<Self, String> {
        let width = canvas.width();
        let height = canvas.height();
        let renderer = Renderer::new(canvas).await?;
        let mut world = World::new();
        let camera = Camera::new(width, height);

        // Spawn Player Character "Jeff"
        let player_entity = spawn_jeff(&mut world, Vec3::ZERO);
        world.insert_one(player_entity, Player { id: 1 }).unwrap();
        world.insert_one(player_entity, Jeff).unwrap(); // Identity
        world.insert_one(player_entity, Weapon::taurus_g2c()).unwrap();
        world.insert_one(player_entity, Health::new(100.0)).unwrap();
        world.insert_one(player_entity, IsShooting { active: false }).unwrap();
        world
            .insert_one(player_entity, PlayerStats::new())
            .unwrap();

        let ground_z = world
            .get::<&Position>(player_entity)
            .map(|pos| pos.z)
            .unwrap_or(0.0);
        world
            .insert_one(
                player_entity,
                CharacterMotor {
                    ground_z,
                    is_grounded: true,
                },
            )
            .unwrap();
        
        world.spawn((
            Position::new_3d(0.0, 0.0, 0.0),
            GlobalRotation::new(Quat::IDENTITY),
            Renderable::new_3d(0.08, 0.15, 0.08, [0.2, 0.2, 0.2, 1.0]),
            GunModel,
        ));

        // Spawn Ground Plane
        world.spawn((
            Position::new_3d(0.0, 0.0, 0.0), // Align ground with world zero
            Velocity::new(0.0, 0.0),
            GlobalRotation::new(Quat::IDENTITY),
            Renderable::new_3d(20.0, 20.0, 0.1, [0.5, 0.5, 0.5, 1.0]), // Base color (shader will override)
            MaterialId::Ground,
        ));

        // Create "Forest" Environment
        let mut rng = rand::thread_rng();
        for _ in 0..15 {
            let x = rng.gen_range(-9.0..9.0);
            let y = rng.gen_range(-9.0..9.0);
            let tree_scale = rng.gen_range(0.8..1.2);
            
            // Tree Trunk (Taller, thinner)
            let trunk = world.spawn((
                Position::new_3d(x, y, 0.5), 
                GlobalRotation::new(Quat::IDENTITY),
                Renderable::new_3d(0.2 * tree_scale, 0.2 * tree_scale, 1.0 * tree_scale, [0.4, 0.25, 0.1, 1.0]),
                AABB::new(0.3, 0.3), // Tree Collider (Static - NO Velocity)
            ));

            spawn_static_shadow(
                &mut world,
                Vec3::new(x, y, 0.0),
                0.7 * tree_scale,
                0.7 * tree_scale,
                -0.3,
            );
            
            // Tree Leaves (Small cap on top, not huge box)
            world.spawn((
                Position::new_3d(0.0, 0.0, 0.0),
                LocalTransform::new(Vec3::new(0.0, 0.0, 0.6 * tree_scale), Quat::IDENTITY, Vec3::ONE),
                Parent { entity: trunk },
                Renderable::new_3d(
                    0.6 * tree_scale,
                    0.6 * tree_scale,
                    0.6 * tree_scale,
                    [0.1, 0.5, 0.1, 1.0],
                ),
            ));
        }

        // Spawn initial Zombob wave
        spawn_zombob_wave(&mut world, &mut rng, 5, 1);
        
        Ok(Self {
            world,
            renderer,
            camera,
            last_time: 0.0,
            input_state: InputState::new(),
            keybinds: Keybinds::default(),
            left_handed: false, // Default Right Handed
            mouse_sensitivity: 0.002, // Default sensitivity
            fov: 1.047198, // Default 60 degrees in radians
            wave: 1,
            wave_spawn_cooldown: 0.0,
            player_entity,
            player_dead: false,
            game_over_announced: false,
        })
    }

    fn update_wave(&mut self, dt: f32) {
        let alive = self.world.query::<&Zombob>().iter().count() as u32;
        if alive > 0 {
            self.wave_spawn_cooldown = 1.0;
            return;
        }

        if self.wave_spawn_cooldown > 0.0 {
            self.wave_spawn_cooldown -= dt;
            return;
        }

        self.wave += 1;
        let spawn_count = ((3 + self.wave * 2) as f32 * (if self.wave > 8 { 0.92 } else { 1.0 })) as u32;
        let spawn_count = spawn_count.max(4);
        let mut rng = rand::thread_rng();
        spawn_zombob_wave(&mut self.world, &mut rng, spawn_count, self.wave);
        set_wave_js(self.wave);
        self.wave_spawn_cooldown = 2.25;
    }

    fn finalize_player_death_ui(&mut self) {
        if self.game_over_announced || !self.player_dead {
            return;
        }
        self.game_over_announced = true;
        let kills = self
            .world
            .get::<&PlayerStats>(self.player_entity)
            .map(|s| s.kills)
            .unwrap_or(0);
        let score = kills.saturating_mul(12).saturating_add(self.wave.saturating_mul(80));
        show_game_over_js(self.wave, kills, score);
    }
}

#[wasm_bindgen]
impl GameEngine {
    #[wasm_bindgen(js_name = restartRun)]
    /// Clears run state and respawns a fresh horde (used after "Try Again" in UI).
    pub fn restart_run(&mut self) {
        despawn_all_ephemeral(&mut self.world);

        let z_roots: Vec<hecs::Entity> = self.world.query::<&Zombob>().iter().map(|(e, _)| e).collect();
        for root in z_roots {
            despawn_entity_subtree(&mut self.world, root);
        }

        self.wave = 1;
        self.wave_spawn_cooldown = 0.0;
        self.player_dead = false;
        self.game_over_announced = false;
        self.camera.shake_phase = 0.0;
        self.camera.shake_magnitude = 0.0;

        if let Ok(mut pos) = self.world.get::<&mut Position>(self.player_entity) {
            pos.x = 0.0;
            pos.y = 0.0;
            pos.z = JEFF_REST_Z;
        }
        if let Ok(mut vel) = self.world.get::<&mut Velocity>(self.player_entity) {
            vel.x = 0.0;
            vel.y = 0.0;
            vel.z = 0.0;
        }
        if let Ok(mut h) = self.world.get::<&mut Health>(self.player_entity) {
            h.current = h.max;
        }
        if let Ok(mut stats) = self.world.get::<&mut PlayerStats>(self.player_entity) {
            stats.kills = 0;
        }
        let _ = self
            .world
            .insert_one(self.player_entity, Weapon::taurus_g2c());
        let _ = self
            .world
            .insert_one(self.player_entity, IsShooting { active: false });

        let mut rng = rand::thread_rng();
        spawn_zombob_wave(&mut self.world, &mut rng, 5, 1);
        set_wave_js(1);
        reset_run_ui_js();
    }

    pub fn toggle_handedness(&mut self) {
        self.left_handed = !self.left_handed;
    }

    pub fn set_mouse_sensitivity(&mut self, value: f32) {
        // Clamp between 0.0005 and 0.01
        self.mouse_sensitivity = value.clamp(0.0005, 0.01);
    }

    pub fn set_fov_degrees(&mut self, degrees: f32) {
        // Clamp between 50 and 110 degrees, convert to radians
        let clamped_degrees = degrees.clamp(50.0, 110.0);
        self.fov = clamped_degrees.to_radians();
        self.camera.fov = self.fov;
    }

    pub fn tick(&mut self, time: f64) {
        let dt = (time - self.last_time) as f32 / 1000.0; // convert ms to s
        self.last_time = time;

        self.camera.decay_shake(dt);

        if self.player_dead {
            self.input_state.end_frame();
            if let Err(e) = self.renderer.render(&self.world, &self.camera) {
                web_sys::console::error_1(&JsValue::from_str(&format!("Render error: {}", e)));
            }
            return;
        }

        self.camera.yaw += self.input_state.mouse_dx * self.mouse_sensitivity;
        self.camera.pitch -= self.input_state.mouse_dy * self.mouse_sensitivity;
        let max_pitch = std::f32::consts::FRAC_PI_2 - 0.1;
        self.camera.pitch = self.camera.pitch.clamp(-max_pitch, max_pitch);

        // Run systems
        weapon_system(&mut self.world, dt);
        crate::engine::systems::ai_system(&mut self.world, dt, &mut self.camera);
        crate::engine::systems::health_system(&mut self.world);
        player_input_system(
            &mut self.world,
            &self.input_state,
            &mut self.camera,
            self.left_handed,
            self.renderer.size.0 as f32,
            self.renderer.size.1 as f32,
        );
        projectile_system(&mut self.world, dt, &mut self.camera);
        flash_system(&mut self.world, dt);
        movement_system(&mut self.world, dt);
        animation_system(&mut self.world, time);
        transform_propagation_system(&mut self.world);
        self.update_wave(dt);

        if let Ok(health) = self.world.get::<&Health>(self.player_entity) {
            if health.current <= 0.0 {
                self.player_dead = true;
            }
        }
        self.finalize_player_death_ui();

        // Update camera to follow player and apply mouse look
        let mut player_velocity = glam::Vec2::ZERO;
        const CAMERA_DEADZONE: f32 = 0.02;
        const LOOK_AHEAD_FACTOR: f32 = 0.12;
        for (_id, (pos, vel, _player)) in self.world.query::<(&Position, &Velocity, &Player)>().iter() {
            // Target is eye height (Head is at +0.24 relative to root)
            // Root is at Z=0.33. Head is at Z=0.57.
            // Let's set camera to 0.25 above root.
            let base_pos = glam::Vec3::new(pos.x, pos.y, pos.z + 0.25);
            let velocity = glam::Vec2::new(vel.x, vel.y);
            player_velocity = velocity;

            let look_ahead = if velocity.length_squared() > 0.01 {
                let dir = velocity.normalize();
                glam::Vec3::new(dir.x, dir.y, 0.0) * LOOK_AHEAD_FACTOR
            } else {
                glam::Vec3::ZERO
            };
            let target_pos = base_pos + look_ahead;
            let delta = target_pos - self.camera.position;
            if delta.length() > CAMERA_DEADZONE {
                self.camera.position = self.camera.position.lerp(
                    target_pos,
                    self.camera.smoothing
                );
            }
        }

        // Update head bob based on movement speed
        let speed = (player_velocity.x * player_velocity.x + player_velocity.y * player_velocity.y).sqrt();
        if speed > 0.01 {
            self.camera.head_bob_time += dt * speed * 2.0;
        } else {
            // Smoothly return head bob to zero when not moving
            self.camera.head_bob_time *= 0.9;
        }

        update_gun_model(&mut self.world, &self.camera, self.left_handed);
        
        self.input_state.end_frame();

        // Render
        if let Err(e) = self.renderer.render(&self.world, &self.camera) {
            web_sys::console::error_1(&JsValue::from_str(&format!("Render error: {}", e)));
        }
    }
    
    pub fn resize(&mut self, width: u32, height: u32) {
        self.renderer.resize(width, height);
        self.camera.aspect = width as f32 / height as f32;
    }
    
    pub fn on_mouse_move(&mut self, x: f32, y: f32) {
        self.input_state.mouse_x = x;
        self.input_state.mouse_y = y;
    }

    pub fn on_mouse_delta(&mut self, dx: f32, dy: f32) {
        self.input_state.mouse_dx += dx;
        self.input_state.mouse_dy += dy;
    }

    pub fn on_mouse_down(&mut self, _button: i32) {
        self.input_state.shoot = true;
    }

    pub fn on_mouse_up(&mut self, _button: i32) {
        self.input_state.shoot = false;
    }

    pub fn on_key_down(&mut self, key_code: &str) {
        self.keybinds.on_key_down(&mut self.input_state, key_code);
    }

    pub fn on_key_up(&mut self, key_code: &str) {
        self.keybinds.on_key_up(&mut self.input_state, key_code);
    }
}
