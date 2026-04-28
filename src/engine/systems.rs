use hecs::World;
use super::components::{
    BodyPart, BodyPartType, CharacterMotor, Debris, GlobalRotation, Gravity, GunModel, Health,
    HitFlash, IsShooting, Jeff, LocalTransform, MuzzleFlash, Parent, Player, PlayerStats,
    Position, Projectile, Renderable, SimpleAI, Velocity, Weapon, Zombob, AABB,
};
use super::input::InputState;
use glam::{Vec3, Quat};
use rand::Rng; // Add rand import
use wasm_bindgen::prelude::*;

// External JS functions for UI updates
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = updateAmmo)]
    fn update_ammo_js(clip: u32, total: u32);
    
    #[wasm_bindgen(js_namespace = window, js_name = updateHealth)]
    fn update_health_js(current: f32, max: f32);
    
    #[wasm_bindgen(js_namespace = window, js_name = incrementKills)]
    fn increment_kills_js();

    #[wasm_bindgen(js_namespace = window, js_name = playGameSound)]
    fn play_game_sound_js(name: &str);

    #[wasm_bindgen(js_namespace = window, js_name = playerDamageFlash)]
    fn player_damage_flash_js();

    #[wasm_bindgen(js_namespace = window, js_name = triggerHitMarker)]
    fn trigger_hit_marker_js();
}

pub fn ai_system(world: &mut World, dt: f32, camera: &mut super::renderer::Camera) {
    // 1. Find the Player (Jeff) position and entity
    let mut jeff_target = None;
    for (entity, (pos, _jeff)) in world.query::<(&Position, &Jeff)>().iter() {
        jeff_target = Some((entity, Vec3::new(pos.x, pos.y, pos.z)));
        break; // Assume single player
    }

    let zpack: Vec<(hecs::Entity, f32, f32)> = world
        .query::<(&Position, &Zombob)>()
        .iter()
        .map(|(e, (p, _))| (e, p.x, p.y))
        .collect();

    // Collect damage events to apply after iteration
    let mut damage_events = Vec::new();

    if let Some((player_entity, target)) = jeff_target {
        // 2. Move AI entities towards Jeff
        for (z_id, (pos, vel, rotation, ai)) in world
            .query_mut::<(&Position, &mut Velocity, &mut GlobalRotation, &mut SimpleAI)>()
        {
            // Update cooldown
            if ai.attack_cooldown > 0.0 {
                ai.attack_cooldown -= dt;
            }

            let mut sep_x = 0.0f32;
            let mut sep_y = 0.0f32;
            const SEP_RADIUS: f32 = 0.48;
            for (other, ox, oy) in &zpack {
                if *other == z_id {
                    continue;
                }
                let dx = pos.x - ox;
                let dy = pos.y - oy;
                let d2 = dx * dx + dy * dy;
                if d2 >= SEP_RADIUS * SEP_RADIUS || d2 < 1e-6 {
                    continue;
                }
                let d = d2.sqrt();
                let push = (SEP_RADIUS - d) / SEP_RADIUS;
                sep_x += (dx / d) * push;
                sep_y += (dy / d) * push;
            }

            let dx = target.x - pos.x;
            let dy = target.y - pos.y;
            let dist_sq = dx * dx + dy * dy;
            let dist = dist_sq.sqrt().max(0.0001);

            // Attack Logic
            if dist < 1.0 {
                // Attack range
                if ai.attack_cooldown <= 0.0 {
                    damage_events.push((player_entity, ai.damage));
                    ai.attack_cooldown = 1.0; // 1 second cooldown

                    // Lunge forward slightly
                    vel.x += (dx / dist) * 5.0;
                    vel.y += (dy / dist) * 5.0;
                }
            }

            if dist_sq < ai.detection_radius * ai.detection_radius {
                // Move towards player
                if dist > 0.6 {
                    // maintain distance
                    let dir_x = dx / dist;
                    let dir_y = dy / dist;

                    vel.x = dir_x * ai.move_speed;
                    vel.y = dir_y * ai.move_speed;

                    // Rotate to face player
                    let angle = dy.atan2(dx) - std::f32::consts::FRAC_PI_2;
                    rotation.rotation = Quat::from_rotation_z(angle);
                } else {
                    // Too close, slow down (melee sizing) — separation pushes sideways
                    vel.x *= 0.65;
                    vel.y *= 0.65;
                    let angle = dy.atan2(dx) - std::f32::consts::FRAC_PI_2;
                    rotation.rotation = Quat::from_rotation_z(angle);
                }

                vel.x += sep_x * ai.move_speed * 2.2 * dt;
                vel.y += sep_y * ai.move_speed * 2.2 * dt;

                let sp = (vel.x * vel.x + vel.y * vel.y).sqrt();
                let cap = ai.move_speed * 1.35;
                if sp > cap {
                    let k = cap / sp;
                    vel.x *= k;
                    vel.y *= k;
                }
            } else {
                // Idle
                vel.x *= 0.9;
                vel.y *= 0.9;
                vel.x += sep_x * ai.move_speed * 1.5 * dt;
                vel.y += sep_y * ai.move_speed * 1.5 * dt;
            }
        }
    }

    // Apply damage to player
    for (entity, dmg) in damage_events {
        if let Ok(mut health) = world.get::<&mut Health>(entity) {
            health.current -= dmg;
            play_game_sound_js("player_hit");
            player_damage_flash_js();
            camera.add_shake(0.068);
        }
    }
}

pub fn health_system(world: &mut World) {
    // Update UI for player health
    for (_id, (health, _player)) in world.query::<(&Health, &Player)>().iter() {
        update_health_js(health.current, health.max);
    }
}

pub fn weapon_system(world: &mut World, dt: f32) {
    // Update all weapons (cooldowns, reload progress)
    for (_id, weapon) in world.query_mut::<&mut Weapon>() {
        weapon.update(dt);
    }
    
    // Update UI for player weapon
    for (_id, (weapon, _player)) in world.query::<(&Weapon, &Player)>().iter() {
        update_ammo_js(weapon.current_clip, weapon.total_ammo);
    }
}

pub fn transform_propagation_system(world: &mut World) {
    // Iterate over children (Entities with Parent and LocalTransform)
    // Run multiple passes to resolve hierarchy depth (simple brute force for now)
    for _ in 0..3 {
        // Collect updates
        let mut frame_updates = Vec::new();
        
        for (id, (parent, local_transform)) in world.query::<(&Parent, &LocalTransform)>().iter() {
            // Check if parent has Position and optionally GlobalRotation
            // We need to read components from parent, but Hecs prevents random access during iteration if mutable?
            // Actually we are iterating immutably over (Parent, LocalTransform)
            // So we can use world.get()
            
            if let Ok(parent_pos) = world.get::<&Position>(parent.entity) {
                 let parent_translation = parent_pos.as_vec3();
                 let parent_rotation = if let Ok(gr) = world.get::<&GlobalRotation>(parent.entity) {
                     gr.rotation
                 } else {
                     Quat::IDENTITY
                 };
                 
                 // Apply parent rotation to local position
                 let rotated_local_pos = parent_rotation * local_transform.position;
                 let global_pos_vec = parent_translation + rotated_local_pos;
                 
                 // Calculate Global Rotation
                 let global_rot = parent_rotation * local_transform.rotation;
                 
                 frame_updates.push((id, 
                     Position::new_3d(global_pos_vec.x, global_pos_vec.y, global_pos_vec.z),
                     GlobalRotation::new(global_rot)
                 ));
            }
        }
        
        if frame_updates.is_empty() {
            break;
        }
        
        // Apply updates
        for (id, pos, rot) in frame_updates {
            let _ = world.insert_one(id, pos);
            let _ = world.insert_one(id, rot);
        }
    }
}

pub fn animation_system(world: &mut World, time: f64) {
    let t = time as f32 / 1000.0;
    
    // 1. Collect parent velocities to determine animation state
    let mut parent_velocities = std::collections::HashMap::new();
    for (id, vel) in world.query::<&Velocity>().iter() {
        parent_velocities.insert(id, (vel.x, vel.y));
    }
    
    // 2. Collect shooting state for players
    let mut player_shooting = std::collections::HashMap::new();
    for (id, shooting_state) in world.query::<&IsShooting>().iter() {
        player_shooting.insert(id, shooting_state.active);
    }
    
    for (_id, (local_transform, body_part, parent)) in world.query_mut::<(&mut LocalTransform, &BodyPart, &Parent)>() {
        let is_moving = if let Some(&(vx, vy)) = parent_velocities.get(&parent.entity) {
             vx.abs() > 0.01 || vy.abs() > 0.01
        } else {
             false
        };
        
        // Check if parent is a player who is shooting
        let is_shooting = player_shooting.get(&parent.entity).copied().unwrap_or(false);

        // Shooting animation takes priority over walking
        if is_shooting && matches!(body_part.part_type, BodyPartType::RightArm) {
            // Raise right arm to aim/shoot position (shoulder-mounted style)
            // Arm is on right side (X+), vertical (Z+)
            // To aim forward: rotate around X-axis to swing forward (Y+)
            // Positive X rotation swings the arm forward from the right side
            let aim_angle = 1.3; // Raised more for proper gun height (positive X rotation for right arm)
            local_transform.rotation = Quat::from_rotation_x(aim_angle);
            
            // Move arm forward (Y+) to simulate shoulder-mounted movement
            // Reduced offset so arm isn't past the body
            let forward_offset = 0.08; // Reduced forward movement - keep arm closer to body
            local_transform.position.y = forward_offset; // Original Y is 0.0, so just set to offset
        } else if is_moving {
            match body_part.part_type {
                BodyPartType::LeftArm => {
                    let angle = t.sin() * 0.5;
                    local_transform.rotation = Quat::from_rotation_x(angle);
                },
                BodyPartType::RightArm => {
                    let angle = (t + std::f32::consts::PI).sin() * 0.5;
                    local_transform.rotation = Quat::from_rotation_x(angle);
                    // Reset forward offset when walking
                    local_transform.position.y = 0.0;
                },
                BodyPartType::LeftLeg => {
                    let angle = (t + std::f32::consts::PI).sin() * 0.5;
                    local_transform.rotation = Quat::from_rotation_x(angle);
                },
                BodyPartType::RightLeg => {
                    let angle = t.sin() * 0.5;
                    local_transform.rotation = Quat::from_rotation_x(angle);
                },
                _ => {}
            }
        } else {
            // Idle: Reset rotation and position
            local_transform.rotation = Quat::IDENTITY;
            if matches!(body_part.part_type, BodyPartType::RightArm) {
                local_transform.position.y = 0.0; // Reset forward offset
            }
        }
    }
}

pub fn movement_system(world: &mut World, dt: f32) {
    // 1. Collect all static colliders (Entities with AABB but WITHOUT Velocity)
    let mut static_colliders: Vec<(hecs::Entity, f32, f32, f32, f32)> = Vec::new();
    for (id, (pos, aabb)) in world.query::<(&Position, &AABB)>().iter() {
        // Skip if entity has Velocity (it's dynamic, not static)
        if world.get::<&Velocity>(id).is_ok() {
            continue;
        }
        static_colliders.push((id, pos.x, pos.y, aabb.width, aabb.height));
    }

    // 2. Move dynamic entities (Player, Zombies)
    for (_id, (pos, vel, aabb)) in world.query_mut::<(&mut Position, &mut Velocity, &AABB)>() {
        // Try X movement
        let next_x = pos.x + vel.x * dt;
        let mut collision_x = false;
        
        // World Boundary X
        if next_x < -20.0 || next_x > 20.0 { collision_x = true; }
        
        // Static Collision X
        if !collision_x {
            for (_, sx, sy, sw, sh) in &static_colliders {
                // AABB Overlap Test
                // Dynamic: next_x, pos.y, aabb.w, aabb.h
                // Static: sx, sy, sw, sh
                if (next_x - aabb.width/2.0 < *sx + sw/2.0) && (next_x + aabb.width/2.0 > *sx - sw/2.0) &&
                   (pos.y - aabb.height/2.0 < *sy + sh/2.0) && (pos.y + aabb.height/2.0 > *sy - sh/2.0) {
                    collision_x = true;
                    break;
                }
            }
        }

        if !collision_x {
            pos.x = next_x;
        } else {
            // Slide / Stop
            vel.x = 0.0;
        }

        // Try Y movement
        let next_y = pos.y + vel.y * dt;
        let mut collision_y = false;
        
        // World Boundary Y
        if next_y < -20.0 || next_y > 20.0 { collision_y = true; }
        
        // Static Collision Y
        if !collision_y {
            for (_, sx, sy, sw, sh) in &static_colliders {
                if (pos.x - aabb.width/2.0 < *sx + sw/2.0) && (pos.x + aabb.width/2.0 > *sx - sw/2.0) &&
                   (next_y - aabb.height/2.0 < *sy + sh/2.0) && (next_y + aabb.height/2.0 > *sy - sh/2.0) {
                    collision_y = true;
                    break;
                }
            }
        }

        if !collision_y {
            pos.y = next_y;
        } else {
            vel.y = 0.0;
        }
    }

    const CHARACTER_GRAVITY: f32 = 18.0;
    for (_id, (pos, vel, motor)) in world.query_mut::<(&mut Position, &mut Velocity, &mut CharacterMotor)>() {
        vel.z -= CHARACTER_GRAVITY * dt;
        pos.z += vel.z * dt;

        if pos.z <= motor.ground_z {
            pos.z = motor.ground_z;
            if vel.z < 0.0 {
                vel.z = 0.0;
            }
            motor.is_grounded = true;
        } else {
            motor.is_grounded = false;
        }
    }
    
    // 3. Debris Physics (Gravity + Rotation + Player Interaction)
    
    // Collect player positions for interaction
    let mut player_positions = Vec::new();
    for (_id, (pos, _player)) in world.query::<(&Position, &Player)>().iter() {
        player_positions.push(pos.as_vec3());
    }

    let mut to_despawn = Vec::new();
    for (id, (pos, vel, rot, debris, gravity, renderable)) in world.query_mut::<(&mut Position, &mut Velocity, &mut GlobalRotation, &mut Debris, &Gravity, &Renderable)>() {
        // Apply Gravity
        vel.z -= gravity.accel * dt;
        
        // Player Interaction (Kick debris)
        for p_pos in &player_positions {
            let dist_sq = (pos.x - p_pos.x).powi(2) + (pos.y - p_pos.y).powi(2);
            if dist_sq < 0.6 { // Interaction radius (Player 0.4 + Debris ~0.2)^2 approx
                let dx = pos.x - p_pos.x;
                let dy = pos.y - p_pos.y;
                let len = (dx*dx + dy*dy).sqrt();
                if len > 0.001 {
                    // Push away
                    let push_force = 5.0 * dt;
                    vel.x += (dx / len) * push_force;
                    vel.y += (dy / len) * push_force;
                }
            }
        }

        // Apply Velocity
        pos.x += vel.x * dt;
        pos.y += vel.y * dt;
        pos.z += vel.z * dt;
        
        // Floor Collision (Bounce)
        // Approximate radius from dimensions
        let radius = renderable.width.max(renderable.height).max(renderable.depth) * 0.5;
        
        if pos.z < radius {
            pos.z = radius;
            vel.z = -vel.z * 0.5; // Bounce with damping
            vel.x *= 0.8; // Friction
            vel.y *= 0.8;
            
            // Stop if slow
            if vel.z.abs() < 0.5 && vel.x.abs() < 0.1 && vel.y.abs() < 0.1 {
                vel.z = 0.0;
                vel.x = 0.0;
                vel.y = 0.0;
            }
        }
        
        // Apply Rotation
        let (axis, angle) = debris.rot_velocity.to_axis_angle();
        let delta_rot = Quat::from_axis_angle(axis, angle * dt);
        rot.rotation = delta_rot * rot.rotation;
        
        // Decay
        debris.time_to_live -= dt;
        if debris.time_to_live <= 0.0 {
            to_despawn.push(id);
        }
    }
    
    for id in to_despawn {
        let _ = world.despawn(id);
    }
}

pub fn player_input_system(
    world: &mut World,
    input: &InputState,
    camera: &mut super::renderer::Camera,
    left_handed: bool,
    screen_width: f32,
    screen_height: f32,
) {
    const GUN_OFFSET_FORWARD: f32 = 0.22;
    const GUN_OFFSET_RIGHT: f32 = 0.18;
    const GUN_OFFSET_DOWN: f32 = 0.18;
    const MUZZLE_OFFSET_FORWARD: f32 = 0.12;

    const WALK_SPEED: f32 = 5.0;
    const SPRINT_SPEED: f32 = 9.0;
    const JUMP_VELOCITY: f32 = 6.0;
    
    let mut bullet_spawn: Option<(glam::Vec3, glam::Vec3, Quat)> = None;
    let mut muzzle_flash_spawn: Option<(glam::Vec3, Quat)> = None;
    let mut shooting_updates: Vec<(hecs::Entity, bool)> = Vec::new();

    for (player_id, (_pos, vel, rotation, weapon, motor, _player)) in world.query_mut::<(&Position, &mut Velocity, &mut GlobalRotation, &mut Weapon, &mut CharacterMotor, &Player)>() {
        // 1. Movement (WASD) relative to camera yaw (true FPS)
        let forward = Vec3::new(camera.yaw.sin(), camera.yaw.cos(), 0.0);
        let right = Vec3::new(forward.y, -forward.x, 0.0);
        
        let current_speed = if input.sprint { SPRINT_SPEED } else { WALK_SPEED };
        
        let mut move_dir = Vec3::ZERO;
        if input.up { move_dir += forward; }
        if input.down { move_dir -= forward; }
        if input.right { move_dir += right; }
        if input.left { move_dir -= right; }
        
        if move_dir.length_squared() > 0.0 {
            let dir = move_dir.normalize();
            vel.x = dir.x * current_speed;
            vel.y = dir.y * current_speed;
        } else {
            vel.x = 0.0;
            vel.y = 0.0;
        }

        if input.jump_pressed && motor.is_grounded {
            vel.z = JUMP_VELOCITY;
            motor.is_grounded = false;
        }

        // 2. Rotation (First-person: match camera yaw)
        // In first-person, player rotation matches camera yaw directly
        // Camera yaw: 0 = North (+Y), rotates clockwise
        // Our model faces +Y (North), so we use camera yaw directly
        let target_rotation = Quat::from_rotation_z(camera.yaw);
        rotation.rotation = target_rotation;
        
        // For aiming calculation, use camera forward direction
        let mouse_screen = glam::Vec2::new(input.mouse_x, input.mouse_y);
        let screen_size = glam::Vec2::new(screen_width, screen_height);
        let _mouse_world = camera.screen_to_world(mouse_screen, screen_size);

        // 3. Shooting (with proper weapon logic)
        let is_shooting = input.shoot && weapon.can_fire();
        if is_shooting {
            weapon.fire();
            play_game_sound_js("gun_shot");
            camera.add_shake(0.016);

            let forward_aim = Vec3::new(
                camera.yaw.sin() * camera.pitch.cos(),
                camera.yaw.cos() * camera.pitch.cos(),
                camera.pitch.sin(),
            )
            .normalize();
            let right_aim = Vec3::new(forward_aim.y, -forward_aim.x, 0.0).normalize();
            let side = if left_handed { -1.0 } else { 1.0 };
            let eye_pos = camera.position;
            let gun_pos = eye_pos
                + forward_aim * GUN_OFFSET_FORWARD
                + right_aim * (GUN_OFFSET_RIGHT * side)
                + Vec3::Z * -GUN_OFFSET_DOWN;
            let muzzle_pos = gun_pos + forward_aim * MUZZLE_OFFSET_FORWARD;

            // Simple raycast forward from eye to find target point (e.g. at 20 units)
            let aim_dist = 20.0;
            let target_point = eye_pos + forward_aim * aim_dist;

            // Bullet direction is Muzzle -> Target
            let dir = (target_point - muzzle_pos).normalize();

            // Bullet rotation: face travel direction
            let bullet_angle = dir.y.atan2(dir.x) - std::f32::consts::FRAC_PI_2;
            let bullet_rotation = Quat::from_rotation_z(bullet_angle);

            bullet_spawn = Some((muzzle_pos, dir, bullet_rotation));
            let muzzle_rot = Quat::from_rotation_z(camera.yaw)
                * Quat::from_rotation_x(camera.pitch);
            muzzle_flash_spawn = Some((muzzle_pos, muzzle_rot));
        }
        
        // Update shooting state for animation system
        // Arm should be raised when holding shoot button (aiming) or when recently fired
        let shooting_state = input.shoot || weapon.fire_cooldown > 0.0;
        shooting_updates.push((player_id, shooting_state));
        
        // 4. Reload (R key)
        if input.reload {
            weapon.start_reload();
        }
        if weapon.current_clip == 0 && !weapon.is_reloading {
            weapon.start_reload();
        }
    }
    
    // Apply shooting state updates after query loop
    for (player_id, shooting_state) in shooting_updates {
        let _ = world.insert_one(player_id, IsShooting { active: shooting_state });
    }

    // Spawn Bullet (weapon system handles fire rate now)
    if let Some((pos, dir, bullet_rot)) = bullet_spawn {
        world.spawn((
            Position::new_3d(pos.x, pos.y, pos.z),
            Velocity::new(dir.x * 20.0, dir.y * 20.0), // Fast bullet
            GlobalRotation::new(bullet_rot),
            Renderable::new_3d(0.15, 0.15, 0.15, [1.0, 1.0, 0.0, 1.0]), // Small yellow cube
            Projectile { time_to_live: 1.0 }, // 1 sec life
            AABB::new(0.15, 0.15),
        ));
    }

    // Spawn Muzzle Flash
    if let Some((pos, rot)) = muzzle_flash_spawn {
        // Spawn muzzle flash every shot (no random throttle)
        world.spawn((
            Position::new_3d(pos.x, pos.y, pos.z),
            GlobalRotation::new(rot),
            Renderable::new_3d(0.3, 0.3, 0.3, [1.0, 1.0, 0.5, 1.0]), // Bright yellow-white flash
            MuzzleFlash { time_to_live: 0.05 }, // Very short lifetime (3 frames at 60fps)
        ));
    }
}

pub fn update_gun_model(world: &mut World, camera: &super::renderer::Camera, left_handed: bool) {
    const GUN_OFFSET_FORWARD: f32 = 0.22;
    const GUN_OFFSET_RIGHT: f32 = 0.18;
    const GUN_OFFSET_DOWN: f32 = 0.18;

    let gun = world.query::<&GunModel>().iter().next().map(|(id, _)| id);
    let Some(gun_id) = gun else {
        return;
    };

    let forward = Vec3::new(
        camera.yaw.sin() * camera.pitch.cos(),
        camera.yaw.cos() * camera.pitch.cos(),
        camera.pitch.sin(),
    )
    .normalize();
    let right = Vec3::new(forward.y, -forward.x, 0.0).normalize();
    let side = if left_handed { -1.0 } else { 1.0 };

    let gun_pos = camera.position
        + forward * GUN_OFFSET_FORWARD
        + right * (GUN_OFFSET_RIGHT * side)
        + Vec3::Z * -GUN_OFFSET_DOWN;
    let gun_rot = Quat::from_rotation_z(camera.yaw) * Quat::from_rotation_x(camera.pitch);

    let _ = world.insert_one(gun_id, Position::new_3d(gun_pos.x, gun_pos.y, gun_pos.z));
    let _ = world.insert_one(gun_id, GlobalRotation::new(gun_rot));
}

pub fn projectile_system(world: &mut World, dt: f32, camera: &mut super::renderer::Camera) {
    let mut to_despawn = Vec::new();
    let mut damage_events = Vec::new(); // (Entity, Damage)
    let mut hit_events: Vec<(hecs::Entity, Velocity)> = Vec::new();
    
    // Collect zombies for collision
    // (id, x, y, radius)
    let mut zombies = Vec::new();
    for (id, (pos, _zombob)) in world.query::<(&Position, &super::components::Zombob)>().iter() {
        zombies.push((id, pos.x, pos.y, 0.4)); // 0.4 radius
    }

    // Update projectiles
    for (id, (projectile, pos, vel)) in world.query_mut::<(&mut Projectile, &Position, &Velocity)>() {
        projectile.time_to_live -= dt;
        let projectile_velocity = *vel; // Copy for use in knockback calculation
        
        // Despawn if old or out of bounds
        if projectile.time_to_live <= 0.0 || pos.x.abs() > 30.0 || pos.y.abs() > 30.0 {
            to_despawn.push(id);
            continue;
        }

        // Check Collision with Zombies
        for (z_id, zx, zy, zr) in &zombies {
            let dist_sq = (pos.x - zx).powi(2) + (pos.y - zy).powi(2);
            let collision_radius = zr + 0.1; // 0.1 projectile radius
            if dist_sq < collision_radius * collision_radius {
                // Hit!
                to_despawn.push(id);
                damage_events.push((*z_id, 25.0)); // 25 Damage per shot
                hit_events.push((*z_id, projectile_velocity));
                trigger_hit_marker_js();
                camera.add_shake(0.014);
                break; // One bullet hits one zombie
            }
        }
    }

    for (z_id, projectile_velocity) in hit_events {
        if let Ok(mut z_vel) = world.get::<&mut Velocity>(z_id) {
            let speed = (projectile_velocity.x * projectile_velocity.x
                + projectile_velocity.y * projectile_velocity.y)
                .sqrt();
            if speed > 0.0 {
                let dir_x = projectile_velocity.x / speed;
                let dir_y = projectile_velocity.y / speed;
                let knockback = 8.0;
                z_vel.x += dir_x * knockback;
                z_vel.y += dir_y * knockback;
            }
        }

        let _ = world.insert_one(z_id, HitFlash { time_to_live: 0.1 });
    }
    
    // Apply Damage
    let mut dead_zombies = Vec::new();
    for (entity, damage) in damage_events {
        if let Ok(mut health) = world.get::<&mut Health>(entity) {
            health.current -= damage;
            play_game_sound_js("zombie_hit");
            if health.current <= 0.0 {
                dead_zombies.push(entity);
            }
        }
    }
    
    // Process deaths separately to avoid double borrow on despawn
    for entity in dead_zombies {
        play_game_sound_js("zombie_death");
        // Collect children parts to turn into debris
        let mut parts = Vec::new();
        for (child_id, (parent, local_transform, renderable, _body_part)) in world.query::<(&Parent, &LocalTransform, &Renderable, &BodyPart)>().iter() {
            if parent.entity == entity {
                parts.push((child_id, *local_transform, *renderable));
            }
        }
        
        // Get parent position/rotation
        let (parent_pos, parent_rot) = if let Ok(pos) = world.get::<&Position>(entity) {
            let rot = if let Ok(r) = world.get::<&GlobalRotation>(entity) {
                r.rotation
            } else {
                Quat::IDENTITY
            };
            (pos.as_vec3(), rot)
        } else {
            (Vec3::ZERO, Quat::IDENTITY)
        };

        // Despawn the Zombob (including children by default in ECS? No, hecs doesn't cascade despawn automatically unless we handle it, but here we want to detach them anyway)
        // Actually hecs doesn't auto-despawn children. So we should despawn the root and children, but we are respawning children as debris.
        // So we despawn the children too?
        // Wait, if we despawn the root, the children keep existing but their Parent component points to a dead entity.
        // We should despawn the children and spawn new Debris entities to replace them.
        
        for (child_id, local, renderable) in parts {
            let _ = world.despawn(child_id);
            
            // Calculate world spawn position for debris
            let part_pos = parent_pos + parent_rot * local.position;
            let part_rot = parent_rot * local.rotation;
            
            // Random velocity for "explosion" / fall
            let mut rng = rand::thread_rng();
            let vx = rng.gen_range(-2.0..2.0);
            let vy = rng.gen_range(-2.0..2.0);
            let vz = rng.gen_range(2.0..5.0); // Pop up
            
            let rot_vel = Quat::from_axis_angle(
                Vec3::new(rng.gen(), rng.gen(), rng.gen()).normalize(), 
                rng.gen_range(1.0..5.0)
            );

            world.spawn((
                Position::new_3d(part_pos.x, part_pos.y, part_pos.z),
                Velocity::new_3d(vx, vy, vz), // 3D velocity needed
                GlobalRotation::new(part_rot),
                renderable, // Keep appearance
                Debris { time_to_live: 10.0, rot_velocity: rot_vel },
                Gravity { accel: 9.8 },
            ));
        }

        let _ = world.despawn(entity);
        for (_, stats) in world.query_mut::<&mut PlayerStats>() {
            stats.kills += 1;
            break;
        }
        increment_kills_js();
        camera.add_shake(0.038);
    }

    // Update muzzle flashes
    for (id, (flash, _)) in world.query_mut::<(&mut MuzzleFlash, &Position)>() {
        flash.time_to_live -= dt;
        
        // Despawn when expired
        if flash.time_to_live <= 0.0 {
            to_despawn.push(id);
        }
    }
    
    for id in to_despawn {
        let _ = world.despawn(id);
    }
}

pub fn flash_system(world: &mut World, dt: f32) {
    let mut to_remove = Vec::new();
    for (id, (flash, renderable)) in world.query_mut::<(&mut HitFlash, &mut Renderable)>() {
        flash.time_to_live -= dt;
        // Flash White-Hot
        renderable.color = [5.0, 5.0, 5.0, 1.0]; 
        
        if flash.time_to_live <= 0.0 {
            to_remove.push(id);
            // Reset to default white (for textured entities)
            renderable.color = [1.0, 1.0, 1.0, 1.0];
        }
    }
    
    for id in to_remove {
        let _ = world.remove_one::<HitFlash>(id);
    }
}
