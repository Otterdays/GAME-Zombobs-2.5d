# ZOMBOBS DEVELOPMENT ROADMAP

## 🗺️ Strategic Overview
**Goal**: Build a high-performance, high-density 2.5D Zombie Survival game that leverages Rust/WASM to render hundreds of entities without lag.
**Vibe**: Arcade functionality with Tactical/Gritty aesthetics (Isometric view, Procedural noise textures).

---

## 📅 Phases & Milestones

### Phase 1: The "Game Feel" (Current Focus)
**Objective**: Make moving and looking feel responsive and grounded.
- [x] **Twin-Stick Aiming** (Priority: **COMPLETE**)
    - [x] Decouple player rotation from velocity.
    - [x] Raycast from Camera to Ground Plane to find Mouse World Position.
    - [x] Player looks at Mouse Cursor.
- [x] **Physics & Collision** (Priority: **COMPLETE**)
    - [x] Implement AABB (Axis-Aligned Bounding Box) component.
    - [x] Prevent Player from walking through Trees (Sliding Collision).
    - [x] Prevent Player from walking off the map.
- [ ] **Camera Polish** (Priority: **MEDIUM**)
    - [ ] Implement "Deadzone" or "Look-Ahead" camera (camera shifts towards where you aim).

### Phase 1.5: Visual Definition (The "Looking Good" Update)
**Objective**: Ensure the 2.5D character reads well from all angles (Front/Back/Side).
- [ ] **Character Details** (Priority: **HIGH**)
    - [ ] **Backpack/Gear**: Add a backpack entity to the player's back. This assumes "Back View" is visually distinct from "Front View".
    - [ ] **Hair/Hat**: Add geometry to the top of the head to break the perfect cube silhouette.
    - [ ] **Limb Animation**: Improve the "Swing" animation to look more natural (bending knees/elbows if possible, or just better pivots).
- [x] **Dynamic Shadows** (Priority: **MEDIUM**)
    - [x] Simple "Blob Shadow" under the player (Quad on the floor).
    - [x] Helps ground the character in the 2.5D space.

### Phase 2: Violence (The "Crunch" Update)
**Objective**: Enable the player to fight back with satisfying feedback.
- [x] **Projectile System** (Priority: **COMPLETE**)
    - [x] Spawn "Bullet" entities (small yellow quads or tracers).
    - [x] **Linear Velocity**: Bullets move straight at high speed.
    - [x] **Despawn**: Bullets vanish after 1s or hitting a tree.
- [x] **Hit Detection** (Priority: **COMPLETE**)
    - [x] `check_collisions` system extension: Bullet vs Zombie AABB.
    - [x] On Hit: Delete Bullet, Damage/Kill Zombie (4-hit kill).
- [x] **Visual Juice** (The "Boiling Point")
    - [x] **Muzzle Flash**: Spawn a 1-frame bright yellow/white quad at the gun tip.
    - [x] **Debris System**: When a zombie dies, its body parts detach and bounce with physics (Ragdoll-lite).
    - [ ] **Screen Shake**: When shooting, offset the camera slightly for 2-3 frames.

### Phase 3: The Horde (Stress Test)
**Objective**: Leverage WASM for high-density enemy counts (MVP Goal: 500 Zombies).
- [x] **Zombie AI** (Priority: **COMPLETE**)
    - [x] **Simple Chase**: Zombies track and follow Jeff.
    - [ ] **Flow Field or Boid Logic**: Zombies shouldn't just stack. They need "Separation" force to create a "Wall of Meat" effect.
    - [ ] **Spawner**: Every 5 seconds, spawn 10 zombies at the map edge.
- [ ] **Performance Tuning**
    - [ ] Ensure Instanced Rendering handles 500+ draw calls (It should, we are using one draw call per mesh type).

### Phase 4: The Loop (Game Structure)
**Objective**: Create a cohesive 30-second loop.
- [ ] **The "Last Stand" Metric**:
    - [x] Score Counter (Zombies Killed).
    - [ ] High Score LocalStorage save.
    - [ ] "You Died" Overlay when an enemy touches you.
- [ ] **Audio** (Priority: **LOW but High Impact**)
    - [ ] Procedural synthesizer for gunshots (White noise burst envelope) - keeps assets zero.

### Phase 5: First-Person Polish (New)
**Objective**: Refine the FPS experience.
- [x] **True FPS Camera**: Camera positioned at eye-level (inside Jeff's head).
- [x] **Handedness**: Toggle between Right and Left-handed gun models.
- [x] **Custom Cursor**: Bullet-shaped crosshair with bold visibility.
- [x] **Debris Interaction**: Player can kick/push zombie debris.

---

## 🧠 "Think Tank" Notes
*   **Rendering**: The current "Instance" rendering is perfect for the Horde. We can render 10,000 sprites easily. Keep using this.
*   **Art Style**: procedural `wgsl` shaders are working great. Continue using noise/math for textures instead of loading PNGs to keep the build size tiny and "code-native".
*   **ECS**: We are using `hecs`. Ensure we don't borrow `World` mutably in two places at once. This will be the main source of bugs as we add Bullet/Zombie interaction.
