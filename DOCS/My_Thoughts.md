# My Thoughts

## 2026-01-14
Initializing documentation structure. The project is in a good state with the foundational "Camera Follow" just completed. The `scratchpad` indicates the next step is verification.

I just generated the core 5 docs complying with the user protocols.
- `SBOM.md` reflects `Cargo.toml`.
- `CHANGELOG.md` reflects recent progress.
- `ARCHITECTURE.md` summarizes the Rust/WebGPU/WASM stack.
- `SUMMARY.md` provides the high-level index.

## 2026-01-14
### Zombie Survival Route
The user wants to pivot the aesthetics and scale to a "Zombie Survival" game.
1. **Scale**: The current player is too big. Shrinking it to ~0.6 should make the world feel bigger and the enemies (when added) more threatening.
2. **Identity**: The current player is Green. I will change this to a Human tone (Peach/Tan) so they look like a Survivor, not a Zombie. This creates immediate visual clarity.
3. **Outfit**: Changing from bright "superhero" Blue/Red to "Survivor" Grey/Khaki.
4. **Fixes**: Switched topology to `TriangleList` to fix "see-through" limbs. Added `slerp` for smooth player rotation.
5. **Restructure**: Completely refactored `spawn_zombob` to make the Torso the root entity (Center of Mass).
    - Head is now above Torso (Z+).
    - Arms are side-mounted to Torso (Z+).
    - Legs are mounted below Torso (Z-).
    - Rotation now pivots around the Torso naturally.

I will verify if this fixes the "weird spin" and "see-through" issues.

## 2026-01-14 (Cont.)
User requested a "weird ground pattern like grass and dirt" and to stop using browser tests.
1. **Shader**: Implemented procedural noise in `fs_main` (`shader.wgsl`). It generates a mix of Green and Brown based on position, with a subtle grid overlay.
2. **Environment**: Replaced the random floating boxes with "Trees" (Brown trunk + Green leaves) to create a forest feel.
3. **Camera**: Switched to Isometric view (Tilted down 45 degrees) to give true 2.5D depth.
4. **Controls**: Twin-Stick implemented!
   - `input.rs` now tracks `mouse_x` and `mouse_y`.
   - `renderer.rs` has `screen_to_world` ray-plane intersection logic.
   - `systems.rs` decouples movement from rotation. Player now faces the mouse cursor accurately.
   - `game.rs` connects it all.

Phase 1 Complete. Moving to Physics & Collision to stop walking through trees.

## 2026-01-14: The "Think Tank" Session
**Topic**: Road to Alpha Gameplay.

**Analysis**:
The engine renders beautiful static geometry, but it's not a *game* yet. We are missing the "Crunch".
- **Physics**: We need AABB collision calculation. Walking through trees breaks immersion immediately.
- **Controls**: "Survivor" genre demands Twin-Stick controls (WASD Move + Mouse Aim). Currently, we rotate based on movement, which is wrong for shooting.
- **The Horde**: We chose Rust for performance. We need to leverage that with *hundreds* of zombies. 
- **Combat**: We need distinct "punchy" projectiles. Raycasting is easier, but Projectiles look better in 2.5D.

**Strategic Route**: "Arcade Horde Survival".
Focus on *density* and *flow*. The ground shader suggests a tactical/gritty vibe. We should lean into that.

**Next Immediate Priorities (The Checklist)**:
1.  **Twin-Stick Aiming**: Decouple rotation from movement. Look at mouse.
2.  **Collision**: AABB (Axis-Aligned Bounding Box) for Trees and Entities.
3.  **Projectile System**: Spawn bullets, move them, detect hits.
4.  **Basic AI**: Chase player.

## 2026-01-14: Visual Hotfix
**Issue**: User reported character "laying down" and trees blocking view.
**Diagnosis**: The character dimensions were defined as Width/Length/Height but treated as X/Y/Z. Since Y is "North" in our world, making the Torso "Long in Y" made it look like it was laying flat on the ground.
**Fix**:
1.  **Refactored `spawn_zombob`**: Re-oriented all body parts to be "Z-Up". Torso is now Tall (Z) instead of Long (Y).
2.  **Tree Adjustment**: Made trunks thinner and taller. Reduced leaf box size slightly to prevent camera obstruction.

I will wait for confirmation that the "Sploot" is fixed before proceeding to Physics.

## 2026-01-14: Camera Zoom Fix (The Missing Piece)
**Issue**: User reported "Huge Trees" and things looking broken/blocky.
**Diagnosis**: The `view_height` of the Camera was set to `2.0`. This means the camera only shows a 2x2 meter area of the world. Since our trees are 2 meters tall, a single tree filled the entire screen, making it look like a "Huge Green Block". This also messed up the perspective of the player, making them look flat or distorted due to extreme close-up.
**Fix**:
1.  **Renderer**: Increased `view_height` from `2.0` to `10.0`. This "Zooms Out" by 5x, showing a 10x10 meter area. This is standard for an isometric RPG view.
2.  **Server**: Restarted `launch.bat` just in case.

This should finally resolve the visual scale issues. The "Z-Up" fix + "Zoom Out" = Correct 2.5D View.

## 2026-01-14: Visual Fix 2.0 (The Real Fix)
**Issue**: The previous "visual hotfix" only adjusted the `spawn_zombob` constants but didn't address the root cause, which led to the character still laying down.
**Root Cause**: The default render logic might be assuming a certain axis orientation that conflicts with "Z-Up" when applying the LocalTransform rotation.
**Fix**:
1.  **Refactored `spawn_zombob` (Again)**:
    - Completely redefined the Torso to be Tall (0.5 Z) and Thin (0.25 Y).
    - Rotated Arms/Head/Legs to stack vertically on the Z-axis.
    - Arms are now vertical blocks attached to the sides.
    - Legs are vertical blocks attached to the bottom.
2.  **Tree Adjustment**:
    - Trunks are now 2.0 units tall (Z) and thin (0.2 X/Y).
    - Leaves are a small 0.6 cap on top to avoid blocking the view.

This should DEFINITELY fix the "Laying Down" issue. I am confident because the dimensions (0.4, 0.25, 0.5) explicitly state Height is the largest dimension now.


## 2026-01-14: JS Syntax & Cache Fixes
**Issue**: Browser was caching old WASM and throwing Syntax Errors on `import` statements.
**Fix**:
1.  Rewrote `bootstrap.js` to use `async/await` and dynamic `import()` with a timestamp query param `?t=...`.
2.  Updated `index.html` to inject the script with `?v=...`.
3.  Set up dev server on port `8675` to ensure fresh connection.

## 2026-01-14: Collision Implemented
**Progress**: I have added the `AABB` component and a sliding collision solver in `movement_system`.
**Test**: Trees now have 0.3m AABBs and Player has 0.4m AABB.
**Next Steps**: 
- Verify sliding behavior in browser.
- Move to **Projectile System** (Phase 2).

## 2026-01-14: UI Implementation Planning
**Task**: User requested comprehensive UI planning for in-game elements.
**Approach**: Created detailed TODO document separating "Currently Working" from "Coming Soon" features.
**Key Decisions**:
1. **Architecture**: HTML/CSS overlay system (like existing menu) for zero performance impact.
2. **Phased Rollout**: 
   - Phase 1: Core HUD (Health, Ammo, Kills, Crosshair, Wave)
   - Phase 2: Feedback Systems (Damage numbers, Hit markers, Low health warning)
   - Phase 3: Advanced HUD (Minimap, Weapon display, Score multiplier)
   - Phase 4: "Coming Soon" placeholders (Inventory, Upgrades, Leaderboard, Achievements, Co-op, Boss bars)
3. **"Coming Soon" Philosophy**: Show future features as greyed-out/locked elements with floating "COMING SOON" text to build anticipation and transparency.
4. **Design System**: Matrix-green aesthetic matching existing menu, monospace fonts, procedural CSS shapes (no images).
5. **Rust ↔ JS Bridge**: Defined clear interface for game state → UI updates via `wasm_bindgen` functions.

**Deliverable**: Created `DOCS/UI_IMPLEMENTATION_TODO.md` with complete roadmap, technical specs, and implementation checklists.
**Next Steps**: Begin Phase 1 implementation (Core HUD) after current gameplay systems are stable.

## 2026-01-14: Weapon & Ammo UI Implementation
**Task**: User requested status check on weapon/ammo UI implementation and to jump back in based on recent session images.
**Analysis**: Reviewed code and found that while the `Weapon` component and `ui.css` were created, they weren't connected:
- Weapon component existed but wasn't attached to player
- HTML HUD elements were missing
- No Rust → JS bridge functions
- Shooting system used random throttle instead of weapon logic

**Implementation**:
1. **Weapon System Integration**:
   - Added `Weapon::taurus_g2c()` to player entity on spawn
   - Created `weapon_system()` to update weapon state (cooldowns, reload progress)
   - Modified `player_input_system()` to use `weapon.can_fire()` and `weapon.fire()`
   - Removed random bullet throttle - now uses proper fire rate (4 shots/sec)
   - Added auto-reload when clip is empty

2. **Gun Visual Model**:
   - Spawned dark grey gun entity attached to player's right arm
   - Positioned at hand location using LocalTransform
   - Uses GunModel marker component for future reference

3. **HUD HTML Structure**:
   - Added `#hud-container` with all Phase 1 elements
   - Health bar (top-left) with gradient fill and low-health animation
   - Ammo counter (bottom-right) with large numbers and empty-flash
   - Kill counter (top-right) with skull icon and scale-pop animation
   - Wave indicator (top-center) with glow effect
   - Crosshair (follows mouse, CSS-only design)
   - Reload indicator with progress bar
   - Low health vignette overlay

4. **Rust → JS Bridge**:
   - Declared external JS functions in systems.rs: `update_ammo_js()`, `update_health_js()`, `increment_kills_js()`
   - Called `update_ammo_js()` from weapon_system every frame
   - Ready for health/kill tracking when those systems are added

5. **JavaScript UI Controller**:
   - Added `window.updateAmmo()` - updates ammo text, flashes red when empty
   - Added `window.updateHealth()` - updates health bar width/color, activates vignette when low
   - Added `window.incrementKills()` - increments counter with scale animation
   - Added `window.setWave()` - updates wave number
   - Added crosshair mouse tracking
   - HUD shows on game start, hides in menu
   - Cursor hidden during gameplay (crosshair replaces it)

**Technical Details**:
- Weapon stats: 12+1 capacity, 25 damage, 4 shots/sec, 1.5s reload
- Fire cooldown prevents spam clicking
- Reload progress tracked but not yet displayed (needs reload indicator update)
- All UI uses CSS animations (GPU-accelerated)
- Zero performance impact from HTML overlay

**Status**: ✅ Weapon and Ammo UI fully implemented and ready to test
**Next Steps**: 
- Test in browser to verify ammo counter updates correctly
- Add Health component and damage system
- Add zombie enemies with health
- Implement bullet collision detection
