# Project Summary

## Project: Zombobs 2.5D Custom Engine
**Path**: `c:/Users/home/Desktop/AI/GAME-Zombobs-2.5d-Custom-Engine`
**Version**: 0.0.1 ALPHA

## Status
- **Phase**: Alpha - First-Person Prototype
- **Core Engine**: Functional (Render, Loop, ECS, WebGPU).
- **Game Logic**: First-person movement (walk/sprint/jump), camera with look-ahead, aiming, shooting, weapon system, zombie AI with attacks and mild separation (anti-stacking).
- **Combat**: Weapon system with reload, zombie attacks with damage/cooldown, knockback on hit, visual hit flash feedback, wave progression spawner, screen shake, wired hit markers, game-over flow with restart.
- **Audio**: Web Audio API sound system (menu sounds, gunshots, zombie hit/death/player hit) with master + SFX + music gain stages (music bus ready for a future loop).
- **Visuals**: First-person view, procedural ground, blob shadows, Minecraft-style character models, hit flash effects, damage flash overlay.
- **UI**: Main menu + tabbed settings panel (Audio/Video/Controls) + in-game HUD (Health, Ammo, Kills, Wave, high score, Crosshair, Keybinds) + death overlay with local best score.
- **Weapon System**: Taurus G2C pistol with ammo management, fire rate control, manual reload (R key).

[AMENDED 2026-04-06]: Reflects game-over/restart, high score, SFX/music sliders, camera shake, separation tuning.

## Key Components
- **Render Engine**: WebGPU (Instanced Quads).
- **Logic**: Rust ECS (`hecs`).
- **Platform**: Web (WASM).
- **Structure**: Engine modules under `src/engine/`.
- **UI Layer**: HTML/CSS overlay with menu state management.

## Documentation Index
- [README](../README.md): Quick start and overview.
- [ARCHITECTURE](ARCHITECTURE.md): Deep dive into code structure.
- [ROADMAP](ROADMAP.md): Development phases and milestones.
- [UI_IMPLEMENTATION_TODO](UI_IMPLEMENTATION_TODO.md): In-game UI planning and roadmap.
- [CHANGELOG](CHANGELOG.md): Version history.
- [SBOM](SBOM.md): Dependency graph.
- [SCRATCHPAD](SCRATCHPAD.md): Active task list and reasoning.
