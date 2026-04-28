# SCRATCHPAD

## Active Tasks
- [x] Combat UX pass (damage flash, hit markers wired from WASM, camera screen shake)
- [x] Game over overlay + score + localStorage high score + Try Again (`restartRun` / `resetRunUi`)
- [x] Zombie separation steering (anti-stack) + softer wave/stat scaling tweaks
- [x] SFX + Music volume sliders wired to Web Audio gains (music bus ready for future loop)
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
1. `PlayerStats`, `showGameOver` / `resetRunUi`, `restartRun()` + JS game-over HUD and high-score persistence.
2. Camera shake pipeline (`add_shake` / `decay_shake`), damage red flash `#damage-flash`, hit marker calls from projectile/hit flows.
3. AI separation forces + wave spawn pacing adjustment (later waves slightly eased on count).
4. Audio: SFX sub-bus + Music sub-bus sliders; menu sounds remain on master-only path.
5. Prepped `pkg` wasm-pack output after API additions (`restartRun`).

[AMENDED 2026-04-06] — Earlier actions retained for trace:
- Prepped for git: root `.gitignore` (target/, pkg/, .env, logs, OS/IDE).
- Updated SCRATCHPAD with way forward; fixed SUMMARY path.
- Realigned gun model to camera using shared offsets (no floating).
- Added wave progression system with dynamic zombie spawns and UI updates.
- Added player hit sound effect and settings panel audio interactions.

## Next Steps
- [ ] Further balance pass from live playtests (waves 5+ tuning)
- [ ] Hook music track into `musicGain` bus when asset exists

## Git Prep (done)
- [x] Root `.gitignore` added (target/, pkg/, .env, *.log, OS/IDE junk)
- [x] `git init`; first commit; remote `origin` = https://github.com/Otterdays/GAME-Zombobs-2.5d
- **Push**: `git push -u origin master` (create repo on GitHub first if 404)
- **Way forward**: Alpha is playable; next priorities: balance pass, SFX/Music sliders, damage feedback; then ROADMAP Phase 1.5 (character details) or Phase 3 (horde/flow field).

