use glam::{Vec2, Vec3, Quat};
use hecs::Entity;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y, z: 0.0 }
    }

    pub fn new_3d(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    
    pub fn as_vec2(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
    
    pub fn as_vec3(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub z: f32, // Added Z component for 3D physics (debris)
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y, z: 0.0 }
    }
    
    pub fn new_3d(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LocalTransform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl LocalTransform {
    pub fn new(position: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self { position, rotation, scale }
    }
    
    pub fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Parent {
    pub entity: Entity,
}

#[derive(Debug, Clone, Copy)]
pub struct Renderable {
    pub width: f32,
    pub height: f32,
    pub depth: f32,
    pub color: [f32; 4],
}

impl Renderable {
    pub fn new(width: f32, height: f32, color: [f32; 4]) -> Self {
        Self { width, height, depth: 0.1, color } // Default depth
    }
    
    pub fn new_3d(width: f32, height: f32, depth: f32, color: [f32; 4]) -> Self {
        Self { width, height, depth, color }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BodyPartType {
    Head,
    Torso,
    LeftArm,
    RightArm,
    LeftLeg,
    RightLeg,
}

#[derive(Debug, Clone, Copy)]
pub struct BodyPart {
    pub part_type: BodyPartType,
}

#[derive(Debug, Clone, Copy)]
pub struct TextureAtlasRegion {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    pub d: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct GlobalRotation {
    pub rotation: Quat,
}

impl GlobalRotation {
    pub fn new(rotation: Quat) -> Self {
        Self { rotation }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Player {
    pub id: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct CharacterMotor {
    pub ground_z: f32,
    pub is_grounded: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct AABB {
    pub width: f32,
    pub height: f32,
    // We treat AABB as centered on Position
}

impl AABB {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Projectile {
    pub time_to_live: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct MuzzleFlash {
    pub time_to_live: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct HitFlash {
    pub time_to_live: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum MaterialId {
    Default = 0,
    Ground = 1,
    Shadow = 2,
}

// ============================================
// WEAPON SYSTEM
// ============================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponType {
    TaurusG2C,      // Starter pistol - cheap, reliable, 12+1 capacity
    // Future weapons:
    // Glock17,
    // Shotgun,
    // SMG,
}

#[derive(Debug, Clone, Copy)]
pub struct Weapon {
    pub weapon_type: WeaponType,
    pub damage: f32,
    pub fire_rate: f32,        // Shots per second
    pub clip_size: u32,
    pub current_clip: u32,
    pub total_ammo: u32,
    pub reload_time: f32,      // Seconds to reload
    pub is_reloading: bool,
    pub reload_progress: f32,  // 0.0 to 1.0
    pub fire_cooldown: f32,    // Time until next shot allowed
}

impl Weapon {
    pub fn taurus_g2c() -> Self {
        // Taurus G2C: Budget 9mm pistol
        // Real specs: 12+1 capacity, ~3.2" barrel
        Self {
            weapon_type: WeaponType::TaurusG2C,
            damage: 25.0,          // One-shot kills basic zombies (requires 4 hits if HP is 100)
            fire_rate: 4.0,        // 4 shots/sec (semi-auto, player limited)
            clip_size: 12,
            current_clip: 12,
            total_ammo: 48,        // 4 extra mags
            reload_time: 1.5,      // 1.5 sec reload
            is_reloading: false,
            reload_progress: 0.0,
            fire_cooldown: 0.0,
        }
    }
    
    pub fn can_fire(&self) -> bool {
        self.current_clip > 0 && !self.is_reloading && self.fire_cooldown <= 0.0
    }
    
    pub fn fire(&mut self) {
        if self.can_fire() {
            self.current_clip -= 1;
            self.fire_cooldown = 1.0 / self.fire_rate;
        }
    }
    
    pub fn start_reload(&mut self) {
        if !self.is_reloading && self.current_clip < self.clip_size && self.total_ammo > 0 {
            self.is_reloading = true;
            self.reload_progress = 0.0;
        }
    }
    
    pub fn update(&mut self, dt: f32) {
        // Update fire cooldown
        if self.fire_cooldown > 0.0 {
            self.fire_cooldown -= dt;
        }
        
        // Update reload
        if self.is_reloading {
            self.reload_progress += dt / self.reload_time;
            if self.reload_progress >= 1.0 {
                // Reload complete
                let ammo_needed = self.clip_size - self.current_clip;
                let ammo_to_load = ammo_needed.min(self.total_ammo);
                self.current_clip += ammo_to_load;
                self.total_ammo -= ammo_to_load;
                self.is_reloading = false;
                self.reload_progress = 0.0;
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }
}

// Gun visual entity marker
#[derive(Debug, Clone, Copy)]
pub struct GunModel;

// Shooting state for animation system
#[derive(Debug, Clone, Copy)]
pub struct IsShooting {
    pub active: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct Zombob; // The enemy (formerly Zombie)

#[derive(Debug, Clone, Copy)]
pub struct Jeff; // The player character identity

/// Run stats tracked for scoring and game-over screen.
#[derive(Debug, Clone, Copy)]
pub struct PlayerStats {
    pub kills: u32,
}

impl PlayerStats {
    pub fn new() -> Self {
        Self { kills: 0 }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SimpleAI {
    pub move_speed: f32,
    pub detection_radius: f32,
    pub damage: f32,
    pub attack_cooldown: f32, // Time until next attack
}

#[derive(Debug, Clone, Copy)]
pub struct Debris {
    pub time_to_live: f32,
    pub rot_velocity: Quat, // Random rotation spin
}

#[derive(Debug, Clone, Copy)]
pub struct Gravity {
    pub accel: f32,
}
