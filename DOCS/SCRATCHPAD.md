# SCRATCHPAD

## Active Tasks
- [x] Fix WebGPU "maxInterStageShaderComponents" limit error
- [x] Initialize Game Engine (ECS, WebGPU, Loop)
- [x] Add Player Entity (Green, Centered)
- [x] Add Input Handling (WASD/Arrows)
- [x] Implement Camera Follow
- [x] Add blob shadows for visual grounding
- [x] Improve value separation (shader lighting + ground tones)
- [x] Build main menu + settings panel UI
- [x] Upgrade ground visuals with procedural material system
- [x] Plan comprehensive UI implementation roadmap
- [x] Ground crispness pass (improved color contrast, sharp transitions, edge definition)
- [x] Implement Weapon component system (Taurus G2C pistol)
- [x] Add weapon_system for ammo/reload management
- [x] Create in-game HUD (Health, Ammo, Kills, Wave, Crosshair)
- [x] Add Rust → JS bridge for UI updates
- [x] Spawn gun visual model attached to player
- [x] Implement Minecraft skin support (texture loading, UV mapping, components)
- [x] Implement 3D perspective camera system with toggle (C key) and keybind display UI
- [x] Replace camera system with first-person cinematic camera
- [x] Design and implement tabbed settings panel (Audio/Video/Controls) with large layout
- [x] Build audio system (menu, gunshots, zombie hit/death)
- [x] Improve Zombob Mechanics (Juice & Combat)
  - [x] Add HitFlash component to components.rs
  - [x] Implement Zombie Attack logic (damage player) in systems.rs
  - [x] Add Knockback & Flash to projectile_system
- [x] Add CharacterMotor system (jump, ground detection, gravity)
- [x] Add sprint functionality (Shift key)
- [x] Add reload keybind (R key)
- [x] Refactor input system with Keybinds struct

## Blockers
None

## Last 5 Actions
1. Prepped for git: root `.gitignore` (target/, pkg/, .env, logs, OS/IDE).
2. Updated SCRATCHPAD with way forward; fixed SUMMARY path.
3. Realigned gun model to camera using shared offsets (no floating).
4. Added wave progression system with dynamic zombie spawns and UI updates.
5. Added player hit sound effect and settings panel audio interactions.

## Next Steps
- [ ] Balance wave scaling (spawn counts, speed, damage, health)
- [ ] Add SFX and Music volume sliders (currently placeholders)
- [ ] Add player damage UI feedback (screen flash or vignette)

## Git Prep (done)
- [x] Root `.gitignore` added (target/, pkg/, .env, *.log, OS/IDE junk)
- [ ] Run `git init` when ready; then `git add .` and first commit
- **Way forward**: Alpha is playable; next priorities: balance pass, SFX/Music sliders, damage feedback; then ROADMAP Phase 1.5 (character details) or Phase 3 (horde/flow field).

