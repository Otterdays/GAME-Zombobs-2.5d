# Changelog

## [0.0.1] - 2026-01-XX - ALPHA
### Added
- **First-Person Camera System**: True first-person perspective with pointer lock mouse look
- **FPS Movement**: WASD movement relative to camera direction (like Minecraft)
- **Player Body Hiding**: Player model hidden in first-person (gun model remains visible)
- **Mouse Look**: Smooth camera rotation with configurable sensitivity
- **Gun Model**: Visible weapon in first-person view attached to player arm

### Changed
- Replaced isometric/3D toggle camera with dedicated first-person camera
- Movement now rotates with camera yaw for proper FPS feel
- Ground plane aligned to Z=0 to fix floating objects
- Camera smoothing set to stable (no jitter) for smooth gameplay

### Fixed
- Fixed inverted mouse look direction (left/right now correct)
- Fixed camera jitter during movement by disabling head bob and stabilizing smoothing
- Fixed floating trees by adjusting ground plane position

## [Unreleased]
### Added
- **[AMENDED 2026-04-06] Combat & loop polish**: Screen shake on camera (`shake_phase`/`shake_magnitude`, decay in tick); red `#damage-flash` on player hit via `window.playerDamageFlash`; hit markers triggered from projectile hits (`triggerHitMarker`); kill score tracked with `PlayerStats` in ECS.
- **Game over**: On player death HUD shows wave / kills / score; best score persisted in `localStorage` (`zombobs_highscore_v1`); **Try Again** calls `restartRun()` — clears ephemeral entities, wipes zombie trees, resets Jeff + wave 1, `resetRunUi()`.
- **Horde pacing**: Separation steering between zombies to reduce stacking; tweaked per-wave zombie health/speed/damage and spawn count curve (later waves slight count reduction multiplier).
- **Audio routing**: Dedicated SFX and Music gain stages (music bus reserved); settings sliders for SFX/Music volume; menu UI sounds stay on master path only.
- **HUD**: High score readout under wave indicator.
- Blob shadows for player and trees to ground actors.
- Tree scale variation for less repetitive silhouettes.
- Main menu overlay with Start/Resume/Settings/Quit buttons.
- Settings panel with fullscreen toggle.
- Menu state management with input gating (pauses game input when menu open).
- ESC key support to toggle menu during gameplay.
- MaterialId component system (Default, Ground, Shadow) for explicit material handling.
- Procedural ground shader with layered noise (grass/dirt patches), subtle grid overlay, and distance-based vignette.
- **Weapon System**: Complete `Weapon` component with fire rate, clip size, reload mechanics.
- **Taurus G2C Pistol**: Starter weapon (12+1 capacity, 25 damage, 4 shots/sec, 1.5s reload, 48 reserve ammo).
- **Gun Visual Model**: Dark grey gun entity attached to player's right arm.
- **In-Game HUD**: Complete HTML/CSS overlay system:
  - Health bar (top-left) with gradient fill and low-health pulse animation.
  - Ammo counter (bottom-right) with large numbers and empty-flash warning.
  - Kill counter (top-right) with scale-pop animation.
  - Wave indicator (top-center) with glow effect.
  - Crosshair (center, follows mouse, CSS-only design).
  - Reload indicator with progress bar (ready for future use).
  - Low health vignette overlay (red pulsing when HP < 30%).
- **Rust → JavaScript Bridge**: Real-time UI updates via `window.updateAmmo()`, `updateHealth()`, `incrementKills()`, `setWave()`.
- **Weapon System Integration**: `weapon_system()` updates all weapons, handles cooldowns and reload progress.
- **Smart Shooting Logic**: Proper fire rate control (4 shots/sec), auto-reload when empty, no spam clicking.
- **Tabbed Settings Panel**: Large centered panel (~70% x 70%) with Audio/Video/Controls tabs.
- **Audio System**: Web Audio API-based sound manager with synthesized sounds:
  - Menu hover/click sounds for all buttons
  - Gunshot sounds (noise + tones)
  - Zombie hit sounds (sawtooth + filtered noise)
  - Zombie death sounds (falling tone + noise)
  - Master volume slider in Audio tab
- **Combat Mechanics Improvements**:
  - Zombie attack system with damage, cooldown, and lunge mechanics
  - Knockback on bullet impact (zombies pushed back when hit)
  - HitFlash component for visual feedback on damage
  - Player damage system (zombies can attack and damage player)
- **CharacterMotor System**: Jump mechanics, ground detection, gravity physics for 3D movement.
- **Sprint Functionality**: Shift key for faster movement (5.0 → 9.0 speed).
- **Reload Keybind**: R key for manual weapon reload.
- **Keybinds System**: Refactored input handling with `Keybinds` struct for cleaner key mapping.
- **Wave Progression**: Dynamic zombie wave spawning with scaling counts and stats.
- **Zombie Variants**: Randomized speed/health/damage per wave spawn.
- **Player Hit Sound**: Added distinct hit sound for zombie attacks.
- **Settings Panel SFX**: Hover/click sounds now apply to settings tabs and inputs.

### Changed
- Stronger value separation via shader lighting and darker ground tones.
- Game loop now starts only after "Start Game" button click (no auto-start).
- Ground rendering now uses material-based system instead of color detection hack.
- Ground visuals improved with visible texture variation and better depth perception.
- Ground crispness pass: high-contrast palette (brighter grass, warmer dirt, packed soil), noise quantization for sharper regions, 2% transition blend range, edge ring definition at boundaries.
- **Shooting System**: Replaced random bullet throttle with proper weapon fire rate and ammo management.
- **Input System**: Integrated weapon logic (`weapon.can_fire()`, `weapon.fire()`) into `player_input_system()`. Refactored to use `Keybinds` struct for cleaner key mapping.
- **UI Visibility**: HUD shows on game start, hides in menu. Cursor hidden during gameplay (crosshair replaces it).
- **Settings Panel**: Upgraded from simple panel to large tabbed interface with Audio/Video/Controls sections. Added FOV slider (50-110°) and mouse sensitivity slider (0.0005-0.01) with real-time engine updates.
- **Movement System**: Added sprint (Shift) and jump mechanics with CharacterMotor for proper ground detection and gravity.
- **Camera Polish**: Added movement look-ahead and deadzone smoothing for stable first-person follow.

## [0.1.0] - 2026-01-14
### Added
- Initial Engine Setup (Rust + WASM + WebGPU).
- ECS Integration (`hecs`).
- Render Pipeline in `renderer.rs` using `wgpu` v23.0.
- Basic Shader (`shader.wgsl`) for rendering quads.
- Instanced Rendering support.
- Player Entity (Green Square).
- Input Handling (WASD/Arrow Keys).
- Camera System (Orthographic projection, Target following).
- `bootstrap.js` for engine initialization and loop.

### Fixed
- WebGPU `maxInterStageShaderComponents` limit error by upgrading `wgpu`.
