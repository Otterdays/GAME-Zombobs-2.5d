# In-Game UI Implementation TODO

**Project**: Zombobs 2.5D Custom Engine  
**Created**: 2026-01-14  
**Status**: Planning Phase

---

## 🎯 Overview

This document outlines the complete UI implementation roadmap for Zombobs, divided into **Currently Working** features and **Coming Soon** features. All UI will be HTML/CSS overlays with minimal performance impact, following the existing menu system architecture.

---

## 🟢 PHASE 1: Core HUD (Currently Working)

### Priority: **URGENT** - Foundation for gameplay feedback

These are essential UI elements that should be implemented immediately to support current gameplay.

#### 1.1 Health Bar
- **Status**: ✅ Implemented
- **Priority**: HIGH
- **Location**: Top-left corner
- **Implementation**:
  - [x] Create `#health-bar` container in `index.html`
  - [x] Style with green-to-red gradient based on health percentage
  - [x] Add Rust function `update_health(current: f32, max: f32)` that calls JS
  - [x] JS function updates bar width and color
  - [x] Add pulsing animation when health < 30%
- **Design Notes**:
  - Monospace font for numbers
  - Green glow effect (#00ff00)
  - Format: `HP: 100/100` with visual bar underneath
  - Bar should "chunk" into segments (like classic arcade games)

#### 1.2 Ammo Counter
- **Status**: ✅ Implemented
- **Priority**: HIGH
- **Location**: Bottom-right corner
- **Implementation**:
  - [x] Create `#ammo-counter` in `index.html`
  - [x] Display current clip / total ammo (e.g., `12 / 48`)
  - [x] Add Rust function `update_ammo(clip: u32, total: u32)`
  - [x] Flash red when ammo = 0
  - [x] Add reload indicator (animated text "RELOADING...")
- **Design Notes**:
  - Large, bold numbers
  - Yellow/orange color scheme
  - Bullet icon using CSS shapes (no images)

#### 1.3 Kill Counter
- **Status**: ✅ Implemented (UI ready, needs enemy system for kills)
- **Priority**: MEDIUM
- **Location**: Top-right corner
- **Implementation**:
  - [x] Create `#kill-counter` in `index.html`
  - [x] Display total zombies killed
  - [x] Add Rust function `increment_kills()`
  - [x] Animate number increment with scale effect
  - [ ] Add milestone celebrations (every 10, 50, 100 kills)
- **Design Notes**:
  - Skull icon using CSS (⚠️ or custom shape)
  - Green monospace font
  - Format: `KILLS: 042`

#### 1.4 Wave Indicator
- **Status**: ✅ Implemented (UI ready, needs wave system)
- **Priority**: MEDIUM
- **Location**: Top-center
- **Implementation**:
  - [x] Create `#wave-indicator` in `index.html`
  - [x] Display current wave number
  - [x] Add Rust function `set_wave(wave_num: u32)`
  - [ ] Animate wave transitions with screen flash
  - [ ] Show "WAVE COMPLETE" message between waves
- **Design Notes**:
  - Large text with glow effect
  - Format: `WAVE 3`
  - Pulsing animation during active wave

#### 1.5 Crosshair
- **Status**: ✅ Implemented
- **Priority**: HIGH
- **Location**: Follows mouse cursor
- **Implementation**:
  - [x] Create `#crosshair` div in `index.html`
  - [x] Position absolutely, centered on cursor
  - [x] CSS-only design (no images)
  - [x] Hide default cursor with `cursor: none` on canvas
  - [x] Change color on enemy hover (red) vs empty space (green)
- **Design Notes**:
  - Simple cross design: `+` shape
  - 2px lines, 20px total size
  - Smooth color transitions

---

## 🟡 PHASE 2: Feedback Systems (Currently Working)

### Priority: **HIGH** - Enhances game feel

#### 2.1 Damage Indicators
- **Status**: Not Implemented
- **Priority**: HIGH
- **Location**: Floating on screen
- **Implementation**:
  - [ ] Create floating damage number system
  - [ ] Spawn `<div class="damage-number">` on hit
  - [ ] Animate upward with fade-out (CSS animation)
  - [ ] Color code: Red for damage taken, Yellow for damage dealt
  - [ ] Auto-remove after animation completes
- **Design Notes**:
  - Bold, large font (24px+)
  - 1-second lifetime
  - Slight random horizontal offset for stacking

#### 2.2 Hit Markers
- **Status**: Not Implemented
- **Priority**: MEDIUM
- **Location**: Center screen
- **Implementation**:
  - [ ] Create `#hit-marker` element
  - [ ] Show brief X shape when bullet hits enemy
  - [ ] Flash for 100ms then fade
  - [ ] Different marker for headshot (if implemented)
- **Design Notes**:
  - White X shape
  - 40px size
  - Opacity pulse animation

#### 2.3 Low Health Warning
- **Status**: Not Implemented
- **Priority**: MEDIUM
- **Location**: Full screen vignette
- **Implementation**:
  - [ ] Create `#low-health-vignette` overlay
  - [ ] Red vignette that pulses when HP < 30%
  - [ ] Increase pulse speed as health decreases
  - [ ] Pointer-events: none (doesn't block clicks)
- **Design Notes**:
  - Radial gradient from transparent to red
  - Subtle heartbeat pulse effect
  - Max opacity: 30%

#### 2.4 Reload Indicator
- **Status**: Not Implemented
- **Priority**: MEDIUM
- **Location**: Center-bottom
- **Implementation**:
  - [ ] Create `#reload-indicator` progress bar
  - [ ] Show during reload action
  - [ ] Animate from 0% to 100% over reload duration
  - [ ] Hide when complete
- **Design Notes**:
  - Horizontal bar, 200px wide
  - Yellow fill color
  - Text: "RELOADING" above bar

---

## 🔵 PHASE 3: Advanced HUD (Currently Working)

### Priority: **MEDIUM** - Quality of life improvements

#### 3.1 Mini-Map
- **Status**: Not Implemented
- **Priority**: LOW
- **Location**: Bottom-left corner
- **Implementation**:
  - [ ] Create `#minimap` canvas element (150x150px)
  - [ ] Draw simplified top-down view
  - [ ] Show player position (green dot)
  - [ ] Show enemies (red dots)
  - [ ] Show map boundaries
  - [ ] Update every frame via Rust → JS bridge
- **Design Notes**:
  - Semi-transparent background
  - Simple geometric shapes
  - Border with green glow

#### 3.2 Weapon Display
- **Status**: Not Implemented
- **Priority**: LOW
- **Location**: Bottom-right (near ammo)
- **Implementation**:
  - [ ] Create `#weapon-display` element
  - [ ] Show current weapon name
  - [ ] Show weapon icon (CSS-based)
  - [ ] Animate weapon switches
- **Design Notes**:
  - Format: `[PISTOL]` or `[SHOTGUN]`
  - Monospace font
  - Fade transition on weapon change

#### 3.3 Score Multiplier
- **Status**: Not Implemented
- **Priority**: LOW
- **Location**: Near kill counter
- **Implementation**:
  - [ ] Create `#multiplier` element
  - [ ] Show current kill streak multiplier
  - [ ] Animate on multiplier increase
  - [ ] Flash when multiplier resets
- **Design Notes**:
  - Format: `x2.5`
  - Scale animation on increase
  - Gold color for high multipliers

---

## 🔮 PHASE 4: "Coming Soon" Features (Future Implementation)

### Priority: **FUTURE** - Not yet implemented, show as "Coming Soon" placeholders

These features will be displayed in the UI with a **"Coming Soon"** floating text element to build anticipation.

#### 4.1 Inventory System
- **Status**: Coming Soon
- **Display**: Small inventory icon with "COMING SOON" badge
- **Location**: Bottom-center
- **Planned Features**:
  - Weapon inventory slots
  - Item pickups (health packs, ammo boxes)
  - Quick-swap system (number keys)
- **Placeholder Implementation**:
  - [ ] Create `#inventory-placeholder` div
  - [ ] Show greyed-out inventory slots (3-5 boxes)
  - [ ] Add floating "COMING SOON" text with pulse animation
  - [ ] Position in bottom-center of screen

#### 4.2 Skill Tree / Upgrades
- **Status**: Coming Soon
- **Display**: "UPGRADES" button with "COMING SOON" tag
- **Location**: Main menu
- **Planned Features**:
  - Permanent upgrades between runs
  - Skill points earned from kills
  - Upgrade tree visualization
- **Placeholder Implementation**:
  - [ ] Add `#upgrades-button` to main menu
  - [ ] Disable button (greyed out)
  - [ ] Add tooltip: "Unlock upgrades - Coming Soon!"
  - [ ] Small floating text animation

#### 4.3 Leaderboard
- **Status**: Coming Soon
- **Display**: "LEADERBOARD" menu option with badge
- **Location**: Main menu
- **Planned Features**:
  - Local high scores
  - Online leaderboard (future)
  - Stats tracking (accuracy, survival time, etc.)
- **Placeholder Implementation**:
  - [ ] Add `#leaderboard-button` to main menu
  - [ ] Show "COMING SOON" badge
  - [ ] Tooltip with planned features

#### 4.4 Achievements
- **Status**: Coming Soon
- **Display**: Trophy icon with "COMING SOON" overlay
- **Location**: Top-right (near kill counter)
- **Planned Features**:
  - Achievement popups
  - Progress tracking
  - Unlock conditions
- **Placeholder Implementation**:
  - [ ] Create `#achievements-icon` element
  - [ ] Show trophy icon (CSS-based)
  - [ ] Add "COMING SOON" floating text
  - [ ] Pulse animation to draw attention

#### 4.5 Co-op / Multiplayer
- **Status**: Coming Soon
- **Display**: "MULTIPLAYER" menu option (locked)
- **Location**: Main menu
- **Planned Features**:
  - Local co-op
  - Online multiplayer (ambitious)
  - Player 2 indicators
- **Placeholder Implementation**:
  - [ ] Add `#multiplayer-button` to main menu
  - [ ] Lock icon next to text
  - [ ] "COMING SOON" badge
  - [ ] Hover tooltip explaining future plans

#### 4.6 Boss Health Bar
- **Status**: Coming Soon
- **Display**: Placeholder at top of screen
- **Location**: Top-center (below wave indicator)
- **Planned Features**:
  - Large boss enemy health bar
  - Boss name display
  - Phase indicators
- **Placeholder Implementation**:
  - [ ] Create `#boss-bar-placeholder` element
  - [ ] Show greyed-out bar outline
  - [ ] Text: "BOSS BATTLES - COMING SOON"
  - [ ] Only visible during specific waves (e.g., Wave 5, 10)

---

## 🎨 Design System

### Color Palette
```css
/* Primary Colors */
--color-primary: #00ff00;      /* Matrix Green */
--color-danger: #ff0000;       /* Red */
--color-warning: #ffaa00;      /* Orange */
--color-info: #00aaff;         /* Blue */
--color-background: #0a0c10;   /* Dark Background */

/* UI States */
--color-health-high: #00ff00;
--color-health-medium: #ffaa00;
--color-health-low: #ff0000;
--color-ammo-normal: #ffaa00;
--color-ammo-empty: #ff0000;
```

### Typography
```css
/* Font Stack */
font-family: 'Courier New', 'Consolas', monospace;

/* Sizes */
--font-size-small: 12px;
--font-size-normal: 16px;
--font-size-large: 24px;
--font-size-xlarge: 32px;
```

### Animations
```css
/* Pulse Effect (for "Coming Soon" badges) */
@keyframes pulse {
    0%, 100% { opacity: 0.6; transform: scale(1); }
    50% { opacity: 1; transform: scale(1.05); }
}

/* Float Effect (for floating text) */
@keyframes float {
    0%, 100% { transform: translateY(0px); }
    50% { transform: translateY(-10px); }
}

/* Glow Effect */
@keyframes glow {
    0%, 100% { text-shadow: 0 0 5px currentColor; }
    50% { text-shadow: 0 0 20px currentColor; }
}
```

---

## 📋 Implementation Checklist

### Immediate (This Week)
- [x] Create `ui.css` file for all UI styles
- [x] Implement Health Bar (1.1)
- [x] Implement Ammo Counter (1.2)
- [x] Implement Kill Counter (1.3)
- [x] Implement Crosshair (1.5)
- [x] Add Rust → JS bridge functions for UI updates

### Short-Term (Next 2 Weeks)
- [ ] Implement Damage Indicators (2.1)
- [ ] Implement Hit Markers (2.2)
- [ ] Implement Low Health Warning (2.3)
- [ ] Implement Wave Indicator (1.4)
- [ ] Add "Coming Soon" placeholders to main menu (4.2, 4.3, 4.5)

### Medium-Term (Next Month)
- [ ] Implement Mini-Map (3.1)
- [ ] Implement Weapon Display (3.2)
- [ ] Implement Reload Indicator (2.4)
- [ ] Add in-game "Coming Soon" elements (4.1, 4.4, 4.6)

### Long-Term (Future Phases)
- [ ] Actually implement inventory system
- [ ] Actually implement skill tree
- [ ] Actually implement leaderboard
- [ ] Actually implement achievements

---

## 🔧 Technical Architecture

### File Structure
```
index.html
├── #ui-overlay (existing)
│   ├── #main-menu (existing)
│   ├── #settings-panel (existing)
│   └── #hud-container (NEW)
│       ├── #health-bar
│       ├── #ammo-counter
│       ├── #kill-counter
│       ├── #wave-indicator
│       ├── #crosshair
│       ├── #minimap
│       ├── #damage-numbers-container
│       ├── #hit-marker
│       ├── #low-health-vignette
│       └── #coming-soon-elements
│           ├── #inventory-placeholder
│           ├── #achievements-icon
│           └── #boss-bar-placeholder
```

### Rust → JS Bridge Functions
```rust
// In lib.rs or game.rs
#[wasm_bindgen]
pub fn update_health(current: f32, max: f32);

#[wasm_bindgen]
pub fn update_ammo(clip: u32, total: u32);

#[wasm_bindgen]
pub fn increment_kills();

#[wasm_bindgen]
pub fn set_wave(wave_num: u32);

#[wasm_bindgen]
pub fn show_damage_number(x: f32, y: f32, damage: i32);

#[wasm_bindgen]
pub fn trigger_hit_marker();
```

### JavaScript UI Controller
```javascript
// In bootstrap.js or new ui-controller.js
const UI = {
    updateHealth(current, max) {
        const bar = document.getElementById('health-bar-fill');
        const text = document.getElementById('health-text');
        const percent = (current / max) * 100;
        bar.style.width = percent + '%';
        text.textContent = `HP: ${current}/${max}`;
        // Color interpolation based on health
    },
    
    updateAmmo(clip, total) {
        const counter = document.getElementById('ammo-counter');
        counter.textContent = `${clip} / ${total}`;
        if (clip === 0) {
            counter.classList.add('empty');
        } else {
            counter.classList.remove('empty');
        }
    },
    
    // ... more UI functions
};
```

---

## 🎯 Success Metrics

### Phase 1 Complete When:
- [x] Player can see health, ammo, and kills at all times
- [x] Crosshair provides clear aiming feedback
- [x] Wave progression is clearly communicated

### Phase 2 Complete When:
- [x] Every hit provides visual feedback
- [x] Player knows when they're in danger (low health)
- [x] Reload timing is clear and visible

### Phase 3 Complete When:
- [x] Player has spatial awareness (minimap)
- [x] Weapon switching is smooth and clear
- [x] Score multiplier encourages aggressive play

### Phase 4 Complete When:
- [x] All "Coming Soon" features are actually implemented
- [x] Players are excited about upcoming features
- [x] UI feels complete and polished

---

## 📝 Notes

### Performance Considerations
- All UI elements use CSS transforms (GPU-accelerated)
- Damage numbers are pooled and reused (max 20 active)
- Minimap updates throttled to 30fps
- No DOM manipulation during critical game loops

### Accessibility
- High contrast colors for visibility
- Clear visual hierarchy
- No essential information conveyed by color alone
- Scalable UI for different screen sizes

### "Coming Soon" Philosophy
- Builds anticipation for future features
- Shows roadmap transparency
- Encourages player feedback
- Creates a sense of active development

---

**Last Updated**: 2026-01-14  
**Next Review**: After Phase 1 completion
