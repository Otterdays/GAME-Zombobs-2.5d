# Zombobs Image Generation Guide

## 🎨 Aesthetic Guidelines

This project has a very specific visual identity. All generated art, concepts, and assets must adhere to these pillars:

-   **Style**: Retro Arcade / Low-Poly 2.5D.
-   **Perspective**: Isometric (Tilted down) or Top-Down (Tactical/Mini-map).
-   **Color Palette**: High Contrast.
    -   **Primary**: "Matrix Digital" Green (#00ff00) for UI/overlays/tech.
    -   **Environment**: Vibrant Toxic Green (Grass) vs Dark Packed Brown (Dirt).
    -   **Lighting**: Dark, gritty atmosphere with punchy, unnatural highlights.
-   **Vibe**: Post-Apocalyptic, Tactical, " Code-Native" (procedural noise patterns).

## 🧠 Prompt Engineering Best Practices

When generating images for this project, **VERBOSITY IS KEY**. Simple prompts yield generic results. You must paint the picture with words before the AI paints it with pixels.

### The "Anatomy of a Perfect Prompt"

1.  **Subject**: What are we looking at? Be specific.
    *   *Bad*: "A gun."
    *   *Good*: "A low-poly compact 9mm pistol with a matte black slide and polymer grip."
2.  **Style & Medium**: How should it look?
    *   *Keywords*: "Concept art", "Isometric game screenshot", "Pixel art style", "Voxel geometry", "Low-poly 3D render".
3.  **Atmosphere & Lighting**: Set the mood.
    *   *Keywords*: "Dark gritty atmosphere", "Matrix green bioluminescent code overlay", "Sharp directional sunlight", "Contrast blob shadows".
4.  **Details & Texture**: Describe the surface.
    *   *Keywords*: "Noisy procedural texture", "Pixelated edges", "Blocky silhouette", "Grid pattern overlay".
5.  **Camera/View**: Where is the viewer?
    *   *Keywords*: "Isometric projection", "Top-down satellite view", "Close up weapon inspect".

### 📝 Example Prompts

**For World Concepts:**
> "Isometric 2.5D game screenshot concept art of a zombie survival game. Dark, gritty atmosphere with subtle Matrix-green lighting accents. A blocky, low-poly Survivor character in a grey t-shirt and cargo pants standing in a dense forest clearing. Tall, blocky trees with simple geometry. Ground is a noisy mix of vibrant green grass and dark packed dirt. Sharp blob shadows. Retro arcade aesthetic with modern lighting."

**For Map/Tactical Views:**
> "Top-down 2D game map concept art in the style of Dynmap or vibrant satellite imagery, showing a post-apocalyptic forest survival zone. Features: dense patches of dark green pixelated trees, distinct brown dirt paths cutting through the green, a central survival clearing, small blocky building footprints. Grid-like pixelated aesthetic, subtle Matrix green tactical grid overlay. High contrast between grass and dirt regions."

**For UI Mockups:**
> "A dark gaming UI mockup showing 'Coming Soon' feature placeholders in a retro arcade style. The design features: A greyed-out inventory section at the bottom center with 5 empty item slots (simple rectangles) and a floating 'COMING SOON' text badge in bright green (#00ff00) with a subtle pulse glow effect. Monospace/terminal font style."

**For Character Sheets:**
> "Low-poly 3D character model concept sheet. Left side: A 'Survivor' character with peach skin, tactical grey t-shirt, dark cargo pants, black combat boots, holding a small black pistol. Right side: A 'Zombie' character with green skin, tattered red shirt, dark pants. Blocky, voxel-inspired geometry, simple shapes, isometric angle. White background with tactical grid."

**For Combat FX:**
> "Action shot concept art of a Survivor character firing a pistol in a dark environment. Bright yellow/white muzzle flash illuminating the immediate area. A yellow tracer bullet flying through the air. A 'Hit Marker' X floating in the air near a struck zombie. Green blood particle effects (cubes) spraying. High contrast, dynamic lighting, retro arcade FX style."

**For Game Over Screens:**
> "Game Over screen UI concept for a retro zombie survival game. Dark background. Large, glitchy red text reading 'YOU DIED'. Below it, a score summary: 'WAVES SURVIVED: 12', 'ZOMBIES KILLED: 142'. 'RETRY' and 'QUIT' buttons in Matrix green style. Background is a blurred, desaturated screenshot of the game world with a red vignette overlay. Minimalist, hacker terminal aesthetic."

**For Weapon Sets:**
> "Concept art sheet of low-poly voxel weapons for a top-down survival game. Displayed on a dark grey tactical grid background. 1. A compact Taurus G2C pistol. 2. A tactical pump-action shotgun. 3. A compact SMG (UZI style). 4. A wooden baseball bat. All items are blocky, simple geometry, pixelated textures. Studio lighting, high contrast."

## 🧱 Texture Generation Strategy

To generate consistent, tiling ground textures for our procedural shaders, follow this specific formula:

**Formula:** `[Seamless + Perspective] + [Material + Condition] + [Specific Details] + [Style/Lighting]`

1.  **Seamless + Perspective**: Always start with "Seamless top-down ground texture concept" or "No perspective, perfect for tiling".
2.  **Material + Condition**: Define the base. e.g., "Dirty cracked concrete", "Rusted industrial metal", "Muddy swamp".
3.  **Specific Details**: Add the flavor. e.g., "Patches of neon green moss", "Oil stains", "Glowing ember cracks".
4.  **Style**: Enforce the aesthetic. "Pixelated noise texture", "Retro arcade style", "High contrast".

### Texture Prompt Examples (Verified)

**Dirty Urban Concrete:**
> "Seamless top-down ground texture concept. Dirty, cracked grey concrete surface. High-frequency pixelated noise detail. Stained with dark oil splotches and grime buildup in cracks. Retro arcade rendering style. Flat lighting, high contrast. No perspective, perfect for tiling."

**Overgrown Pavers:**
> "Seamless top-down ground texture concept. Hexagonal stone pavers overtaken by nature. Vibrant toxic green moss growing thick between the grey stones. Some stones are missing, revealing dark brown dirt underneath. Pixelated jagged edges, sharp separation of colors. Voxel-like flat texture."

**Muddy Swamp:**
> "Seamless top-down ground texture concept. Deep dark brown mud with puddles of stagnant green water. Small bright green vegetation patches floating on top. High gloss reflection detail in puddles. Sticky, wet appearance. Pixelated noise, coherent directional flow. Retro survival horror style."

**Rusted Industrial Floor:**
> "Seamless top-down ground texture concept. Industrial metal grating floor. Heavily rusted orange and brown metal plates bolted together. Diamond plate pattern worn down by age. Scratches and metallic highlights. Dark gaps between plates. Gritty, high-contrast industrial factory floor. Pixelated style."

**Scorched Earth:**
> "Seamless top-down ground texture concept. Scorched, blackened ash earth. Cracks revealing glowing orange embers underneath (magma-like but subtle). Patches of grey ash and black charcoal. Burnt debris scattered. omnisous, dark atmosphere texture. High contrast, sharp pixel edges."

## ⏳ Prompt Backlog (Future Ideas)

These are high-quality prompts ready to be generated when capacity allows:

**Melee & Tools:**
> "Concept art sheet of low-poly voxel melee weapons and tools. Tactical grid background. 1. A red Fire Axe. 2. A yellow industrial Chainsaw. 3. A rusty Pipe Wrench. 4. A lit Molotov Cocktail (bottle with rag). Blocky geometry, pixel art textures, high contrast lighting."

**Crash Site (World):**
> "Isometric 2.5D concept art of a military crash site in a forest. A low-poly crashed helicopter (Black Hawk style) smoking on the ground. Broken trees, scattered crates, and debris. Matrix green ambient lighting with orange fire highlights. Voxel-style geometry, dark gritty atmosphere."

**Urban Ruins Edge:**
> "Isometric 2.5D concept art of the transition between forest and city ruins. Crumbling brick walls, a broken street lamp flickering, cracked pavement with grass growing through cracks. A rusted burnt-out car shell. Matrix-green night atmosphere. Low-poly voxel style."

**Urban Ground Texture:**
> "Seamless texture concept art for a procedural urban ground shader. Top-down view. Cracked grey concrete pavement with patches of green moss/grass poking through fissures. Faded yellow road paint markings. Oil stains. Pixelated noise texture, high contrast, retro arcade style."

**Stormy Night Ambience:**
> "Atmospheric concept art of a survivor standing in a heavy thunderstorm. Isometric view. Rain streaks (Matrix code style rain). A lightning flash creating sharp white rim lighting on the trees and character. Wet ground reflections. Dark, moody, high contrast. Voxel art style."

**Toxic Fog Ambience:**
> "Atmospheric concept art of a 'Toxic Zone'. Thick, glowing green volumetric fog rolling through a dark forest. Silhouettes of zombies with glowing eyes visible in the mist. A survivor in a yellow Hazmat suit standing in the foreground. Voxel/blocky style. Hazardous warning atmosphere."

## 🚫 What to Avoid

-   **Hyper-realism**: We want *stylized* low-poly, not 4k photorealism.
-   **Generic "Mobile Game" Art**: Avoid soft, round, bubbly shapes. We want **Sharp, Blocky, Gritty**.
-   **Overly Complex Geometry**: Keep it simple. Think Minecraft meets Metal Gear Solid on the PS1.

---
*Reference this guide whenever you need to generate new assets or visualize features for Zombobs 2.5D.*
