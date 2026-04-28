// bootstrap.js

const ZOMBOBS_HIGH_SCORE_KEY = 'zombobs_highscore_v1';

class AudioManager {
    constructor() {
        this.context = null;
        this.masterGain = null;
        this.sfxGain = null;
        this.musicGain = null;
        this.unlocked = false;
        this.pendingVolume = 0.7;
        this.lastPlay = {};
    }

    ensureContext() {
        if (this.context) {
            return;
        }

        const AudioCtx = window.AudioContext || window.webkitAudioContext;
        if (!AudioCtx) {
            return;
        }

        this.context = new AudioCtx();
        this.masterGain = this.context.createGain();
        this.masterGain.gain.value = this.pendingVolume;

        this.sfxGain = this.context.createGain();
        this.sfxGain.gain.value = 0.85;
        this.sfxGain.connect(this.masterGain);

        this.musicGain = this.context.createGain();
        this.musicGain.gain.value = 0.45;
        this.musicGain.connect(this.masterGain);

        this.masterGain.connect(this.context.destination);
    }

    unlock() {
        this.ensureContext();
        if (!this.context) {
            return;
        }

        if (this.context.state === 'suspended') {
            this.context.resume();
        }

        this.unlocked = true;
    }

    setMasterVolume(value) {
        this.pendingVolume = value;
        this.ensureContext();
        if (this.masterGain) {
            this.masterGain.gain.value = value;
        }
    }

    setSfxVolume(value) {
        this.ensureContext();
        if (this.sfxGain) {
            this.sfxGain.gain.value = value;
        }
    }

    setMusicVolume(value) {
        this.ensureContext();
        if (this.musicGain) {
            this.musicGain.gain.value = value;
        }
    }

    canPlay(name, minIntervalMs) {
        const now = performance.now();
        const last = this.lastPlay[name] || 0;
        if (now - last < minIntervalMs) {
            return false;
        }
        this.lastPlay[name] = now;
        return true;
    }

    play(name) {
        this.ensureContext();
        if (!this.context || !this.unlocked) {
            return;
        }

        switch (name) {
            case 'menu_hover':
                if (!this.canPlay(name, 80)) {
                    return;
                }
                this.playMenuHover();
                break;
            case 'menu_click':
                if (!this.canPlay(name, 120)) {
                    return;
                }
                this.playMenuClick();
                break;
            case 'gun_shot':
                if (!this.canPlay(name, 80)) {
                    return;
                }
                this.playGunShot();
                break;
            case 'zombie_hit':
                if (!this.canPlay(name, 100)) {
                    return;
                }
                this.playZombieHit();
                break;
            case 'zombie_death':
                if (!this.canPlay(name, 250)) {
                    return;
                }
                this.playZombieDeath();
                break;
            case 'player_hit':
                if (!this.canPlay(name, 150)) {
                    return;
                }
                this.playPlayerHit();
                break;
            default:
                break;
        }
    }

    /**
     * @param {'master'|'sfx'} routing — menu sounds use `master` (not affected by SFX slider).
     */
    playTone(freq, type, duration, volume, endFreq, routing = 'master') {
        const ctx = this.context;
        const osc = ctx.createOscillator();
        const gain = ctx.createGain();

        osc.type = type;
        osc.frequency.setValueAtTime(freq, ctx.currentTime);
        if (endFreq) {
            osc.frequency.exponentialRampToValueAtTime(
                endFreq,
                ctx.currentTime + duration
            );
        }

        gain.gain.setValueAtTime(0.0001, ctx.currentTime);
        gain.gain.exponentialRampToValueAtTime(
            volume,
            ctx.currentTime + 0.01
        );
        gain.gain.exponentialRampToValueAtTime(
            0.0001,
            ctx.currentTime + duration
        );

        osc.connect(gain);
        const dest =
            routing === 'sfx' && this.sfxGain ? this.sfxGain : this.masterGain;
        gain.connect(dest);

        osc.start();
        osc.stop(ctx.currentTime + duration + 0.02);
    }

    createNoiseBuffer(duration) {
        const ctx = this.context;
        const length = Math.floor(ctx.sampleRate * duration);
        const buffer = ctx.createBuffer(1, length, ctx.sampleRate);
        const data = buffer.getChannelData(0);

        for (let i = 0; i < length; i += 1) {
            data[i] = Math.random() * 2 - 1;
        }

        return buffer;
    }

    playNoise(duration, volume, filterType, filterFreq, routing = 'master') {
        const ctx = this.context;
        const buffer = this.createNoiseBuffer(duration);
        const source = ctx.createBufferSource();
        source.buffer = buffer;

        const gain = ctx.createGain();
        gain.gain.setValueAtTime(0.0001, ctx.currentTime);
        gain.gain.exponentialRampToValueAtTime(
            volume,
            ctx.currentTime + 0.01
        );
        gain.gain.exponentialRampToValueAtTime(
            0.0001,
            ctx.currentTime + duration
        );

        let node = source;
        if (filterType && filterFreq) {
            const filter = ctx.createBiquadFilter();
            filter.type = filterType;
            filter.frequency.setValueAtTime(filterFreq, ctx.currentTime);
            node.connect(filter);
            node = filter;
        }

        node.connect(gain);
        const dest =
            routing === 'sfx' && this.sfxGain ? this.sfxGain : this.masterGain;
        gain.connect(dest);

        source.start();
        source.stop(ctx.currentTime + duration + 0.02);
    }

    playMenuHover() {
        this.playTone(720, 'triangle', 0.05, 0.12, 920, 'master');
    }

    playMenuClick() {
        this.playTone(420, 'square', 0.08, 0.18, 220, 'master');
    }

    playGunShot() {
        this.playNoise(0.06, 0.5, 'highpass', 600, 'sfx');
        this.playTone(120, 'square', 0.05, 0.25, 80, 'sfx');
        this.playTone(1400, 'triangle', 0.02, 0.12, 900, 'sfx');
    }

    playZombieHit() {
        this.playTone(180, 'sawtooth', 0.18, 0.22, 90, 'sfx');
        this.playNoise(0.08, 0.18, 'lowpass', 500, 'sfx');
    }

    playZombieDeath() {
        this.playTone(240, 'sawtooth', 0.6, 0.25, 60, 'sfx');
        this.playNoise(0.25, 0.2, 'lowpass', 700, 'sfx');
    }

    playPlayerHit() {
        this.playTone(110, 'square', 0.12, 0.28, 70, 'sfx');
        this.playNoise(0.06, 0.12, 'bandpass', 500, 'sfx');
    }
}

async function run() {
    try {
        // Cache busting
        const timestamp = new Date().getTime();

        // Dynamic import to force browser to fetch fresh WASM/JS
        const module = await import(`./pkg/zombs_engine.js?t=${timestamp}`);
        const init = module.default;
        const { create_engine, init_panic_hook } = module;

        // Initialize WASM module with explicit path to bust cache
        // Passing as object to avoid "deprecated parameters" warning
        await init({ module_or_path: `./pkg/zombs_engine_bg.wasm?t=${timestamp}` });

        // Setup panic hook
        init_panic_hook();

        console.log("WASM loaded. Starting engine...");

        // Get canvas
        const canvas = document.getElementById('game-canvas');

        // Initial resize
        canvas.width = window.innerWidth;
        canvas.height = window.innerHeight;

        // Create engine instance
        const engine = await create_engine("game-canvas");

        // Menu state machine
        let menuState = {
            menuOpen: true,
            settingsOpen: false,
            gameStarted: false,
            loopRunning: false,
            playerDead: false
        };

        const audioManager = new AudioManager();
        window.playGameSound = function(name) {
            audioManager.play(name);
        };

        document.addEventListener(
            'click',
            () => {
                audioManager.unlock();
            },
            { once: true }
        );

        // UI elements
        const uiOverlay = document.getElementById('ui-overlay');
        const mainMenu = document.getElementById('main-menu');
        const settingsPanel = document.getElementById('settings-panel');
        const startButton = document.getElementById('start-button');
        const resumeButton = document.getElementById('resume-button');
        const settingsButton = document.getElementById('settings-button');
        const quitButton = document.getElementById('quit-button');
        const backButton = document.getElementById('back-button');
        const fullscreenToggle = document.getElementById('fullscreen-toggle');
        const menuButtons = document.querySelectorAll('.menu-button');
        
        // Settings tab elements
        const settingsTabs = document.querySelectorAll('.settings-tab');
        const tabPanels = document.querySelectorAll('.tab-panel');
        const settingsInputs = settingsPanel
            ? settingsPanel.querySelectorAll('input')
            : [];
        const masterVolumeSlider = document.getElementById('master-volume-slider');
        const masterVolumeValue = document.getElementById('master-volume-value');
        const fovSlider = document.getElementById('fov-slider');
        const fovValue = document.getElementById('fov-value');
        const sensitivitySlider = document.getElementById('sensitivity-slider');
        const sensitivityValue = document.getElementById('sensitivity-value');
        
        // HUD elements
        const hudContainer = document.getElementById('hud-container');
        const healthText = document.getElementById('health-text');
        const healthBarFill = document.getElementById('health-bar-fill');
        const ammoText = document.getElementById('ammo-text');
        const ammoCounter = document.getElementById('ammo-counter');
        const killCount = document.getElementById('kill-count');
        const killCounter = document.getElementById('kill-counter');
        const waveNumber = document.getElementById('wave-number');
        const crosshair = document.getElementById('crosshair');
        const reloadIndicator = document.getElementById('reload-indicator');
        const reloadBarFill = document.getElementById('reload-bar-fill');
        const lowHealthVignette = document.getElementById('low-health-vignette');
        const sfxVolumeSlider = document.getElementById('sfx-volume-slider');
        const sfxVolumeValue = document.getElementById('sfx-volume-value');
        const musicVolumeSlider = document.getElementById('music-volume-slider');
        const musicVolumeValue = document.getElementById('music-volume-value');
        const gameOverPanel = document.getElementById('game-over-panel');
        const restartButton = document.getElementById('restart-button');

        /** @type {HTMLElement | null} */
        let highScoreValueEl = document.getElementById('high-score-value');

        function readHighScore() {
            try {
                const raw = parseInt(localStorage.getItem(ZOMBOBS_HIGH_SCORE_KEY) || '0', 10);
                return Number.isFinite(raw) && raw >= 0 ? raw : 0;
            } catch (_) {
                return 0;
            }
        }

        function writeHighScore(score) {
            try {
                localStorage.setItem(ZOMBOBS_HIGH_SCORE_KEY, String(score));
            } catch (_) {
                /* ignore quota / privacy mode */
            }
        }

        function updateHighScoreHud() {
            if (highScoreValueEl) {
                highScoreValueEl.textContent = String(readHighScore());
            }
        }
        updateHighScoreHud();

        function exitPointerLockIfAny() {
            if (document.exitPointerLock) {
                document.exitPointerLock();
            } else if (document.mozExitPointerLock) {
                document.mozExitPointerLock();
            } else if (document.webkitExitPointerLock) {
                document.webkitExitPointerLock();
            }
        }

        function openGameOver(waveVal, killsVal, scoreVal) {
            const prev = readHighScore();
            const best = Math.max(prev, scoreVal | 0);
            if (best > prev) {
                writeHighScore(best);
            }
            updateHighScoreHud();

            const wEl = document.getElementById('go-wave');
            const kEl = document.getElementById('go-kills');
            const sEl = document.getElementById('go-score');
            const hEl = document.getElementById('go-high-score');
            if (wEl) {
                wEl.textContent = String(waveVal);
            }
            if (kEl) {
                kEl.textContent = String(killsVal);
            }
            if (sEl) {
                sEl.textContent = String(scoreVal);
            }
            if (hEl) {
                hEl.textContent = String(best);
            }

            menuState.playerDead = true;
            exitPointerLockIfAny();

            if (gameOverPanel) {
                gameOverPanel.classList.add('active');
            }
            document.body.style.cursor = 'default';
        }
        
        // Game state
        let gameState = {
            health: 100,
            maxHealth: 100,
            kills: 0,
            wave: 1
        };
        
        // UI Controller - Exposed to Rust via window
        window.updateHealth = function(current, max) {
            gameState.health = current;
            gameState.maxHealth = max;
            const percent = (current / max) * 100;
            healthText.textContent = `HP: ${Math.floor(current)}/${Math.floor(max)}`;
            healthBarFill.style.width = percent + '%';
            
            // Color based on health
            if (percent < 30) {
                healthBarFill.classList.add('low');
                lowHealthVignette.classList.add('active');
            } else {
                healthBarFill.classList.remove('low');
                lowHealthVignette.classList.remove('active');
            }
        };
        
        window.updateAmmo = function(clip, total) {
            ammoText.textContent = `${clip} / ${total}`;
            
            // Flash red when empty
            if (clip === 0) {
                ammoCounter.classList.add('empty');
            } else {
                ammoCounter.classList.remove('empty');
            }
        };
        
        window.incrementKills = function() {
            gameState.kills++;
            killCount.textContent = String(gameState.kills).padStart(3, '0');
            killCounter.classList.add('increment');
            setTimeout(() => killCounter.classList.remove('increment'), 300);
        };
        
        window.setWave = function(wave) {
            gameState.wave = wave;
            waveNumber.textContent = wave;
        };
        
        window.showReloadIndicator = function(progress) {
            if (progress > 0 && progress < 1) {
                reloadIndicator.classList.add('active');
                reloadBarFill.style.width = (progress * 100) + '%';
            } else {
                reloadIndicator.classList.remove('active');
            }
        };
        
        window.triggerHitMarker = function() {
            const hitMarker = document.getElementById('hit-marker');
            hitMarker.classList.add('active');
            setTimeout(() => hitMarker.classList.remove('active'), 200);
        };

        window.playerDamageFlash = function() {
            const el = document.getElementById('damage-flash');
            if (!el) {
                return;
            }
            el.classList.add('active');
            clearTimeout(window.__zombobsDamageFlashT);
            window.__zombobsDamageFlashT = setTimeout(() => {
                el.classList.remove('active');
            }, 95);
        };

        window.showGameOver = function(waveVal, killsVal, scoreVal) {
            openGameOver(waveVal, killsVal, scoreVal);
        };

        window.resetRunUi = function() {
            gameState.kills = 0;
            gameState.wave = 1;
            gameState.health = 100;
            gameState.maxHealth = 100;
            menuState.playerDead = false;

            if (killCount) {
                killCount.textContent = '000';
            }
            if (waveNumber) {
                waveNumber.textContent = '1';
            }
            window.updateHealth(100, 100);
            window.updateAmmo(12, 48);
            if (gameOverPanel) {
                gameOverPanel.classList.remove('active');
            }
        };
        
        // Crosshair follows mouse
        window.addEventListener('mousemove', (e) => {
            crosshair.style.left = e.clientX - 10 + 'px';
            crosshair.style.top = e.clientY - 10 + 'px';
        });

        // Menu control functions
        function showMainMenu() {
            menuState.menuOpen = true;
            menuState.settingsOpen = false;
            uiOverlay.classList.add('active');
            mainMenu.classList.add('active');
            settingsPanel.classList.remove('active');
            hudContainer.style.display = 'none';
            document.body.style.cursor = 'default'; // Show cursor in menu
            if (menuState.gameStarted) {
                startButton.style.display = 'none';
                resumeButton.style.display = 'block';
            } else {
                startButton.style.display = 'block';
                resumeButton.style.display = 'none';
            }
        }

        function hideMainMenu() {
            menuState.menuOpen = false;
            menuState.settingsOpen = false;
            uiOverlay.classList.remove('active');
            mainMenu.classList.remove('active');
            settingsPanel.classList.remove('active');
        }

        function showSettings() {
            menuState.settingsOpen = true;
            mainMenu.classList.remove('active');
            settingsPanel.classList.add('active');
        }

        function hideSettings() {
            menuState.settingsOpen = false;
            settingsPanel.classList.remove('active');
            if (menuState.menuOpen) {
                mainMenu.classList.add('active');
            }
        }

        function startGame() {
            audioManager.unlock();
            menuState.gameStarted = true;
            hideMainMenu();
            hudContainer.style.display = 'block';
            document.body.style.cursor = 'none'; // Hide cursor for crosshair
            
            // Initialize UI
            window.updateHealth(100, 100);
            window.updateAmmo(12, 48);
            window.setWave(1);
            updateHighScoreHud();
            if (gameOverPanel) {
                gameOverPanel.classList.remove('active');
            }
            menuState.playerDead = false;
            
            if (!menuState.loopRunning) {
                menuState.loopRunning = true;
                requestAnimationFrame(loop);
            }
        }

        function quitGame() {
            location.reload();
        }

        // Fullscreen handling
        function updateFullscreenState() {
            const isFullscreen = !!(document.fullscreenElement || document.webkitFullscreenElement || document.mozFullScreenElement || document.msFullscreenElement);
            fullscreenToggle.checked = isFullscreen;
        }

        async function toggleFullscreen() {
            try {
                if (!document.fullscreenElement && !document.webkitFullscreenElement && !document.mozFullScreenElement && !document.msFullscreenElement) {
                    // Use document.documentElement to make the whole page fullscreen, preserving UI overlays
                    const element = document.documentElement;
                    if (element.requestFullscreen) {
                        await element.requestFullscreen();
                    } else if (element.webkitRequestFullscreen) {
                        await element.webkitRequestFullscreen();
                    } else if (element.mozRequestFullScreen) {
                        await element.mozRequestFullScreen();
                    } else if (element.msRequestFullscreen) {
                        await element.msRequestFullscreen();
                    }
                } else {
                    if (document.exitFullscreen) {
                        await document.exitFullscreen();
                    } else if (document.webkitExitFullscreen) {
                        await document.webkitExitFullscreen();
                    } else if (document.mozCancelFullScreen) {
                        await document.mozCancelFullScreen();
                    } else if (document.msExitFullscreen) {
                        await document.msExitFullscreen();
                    }
                }
            } catch (err) {
                console.error("Fullscreen error:", err);
            }
        }

        // Fullscreen change listeners
        document.addEventListener('fullscreenchange', updateFullscreenState);
        document.addEventListener('webkitfullscreenchange', updateFullscreenState);
        document.addEventListener('mozfullscreenchange', updateFullscreenState);
        document.addEventListener('MSFullscreenChange', updateFullscreenState);

        // UI event handlers
        startButton.addEventListener('click', startGame);
        resumeButton.addEventListener('click', startGame);
        settingsButton.addEventListener('click', showSettings);
        backButton.addEventListener('click', () => {
            hideSettings();
            if (menuState.menuOpen) {
                showMainMenu();
            }
        });
        quitButton.addEventListener('click', quitGame);
        fullscreenToggle.addEventListener('change', toggleFullscreen);

        // Audio settings
        if (masterVolumeSlider && masterVolumeValue) {
            const initialVolume = parseFloat(masterVolumeSlider.value);
            masterVolumeValue.textContent = initialVolume.toFixed(2);
            audioManager.setMasterVolume(initialVolume);
            masterVolumeSlider.addEventListener('input', (e) => {
                const volume = parseFloat(e.target.value);
                masterVolumeValue.textContent = volume.toFixed(2);
                audioManager.setMasterVolume(volume);
            });
        }

        if (sfxVolumeSlider && sfxVolumeValue) {
            const v = parseFloat(sfxVolumeSlider.value);
            sfxVolumeValue.textContent = v.toFixed(2);
            audioManager.setSfxVolume(v);
            sfxVolumeSlider.addEventListener('input', (e) => {
                const next = parseFloat(e.target.value);
                sfxVolumeValue.textContent = next.toFixed(2);
                audioManager.setSfxVolume(next);
            });
        }

        if (musicVolumeSlider && musicVolumeValue) {
            const mv = parseFloat(musicVolumeSlider.value);
            musicVolumeValue.textContent = mv.toFixed(2);
            audioManager.setMusicVolume(mv);
            musicVolumeSlider.addEventListener('input', (e) => {
                const next = parseFloat(e.target.value);
                musicVolumeValue.textContent = next.toFixed(2);
                audioManager.setMusicVolume(next);
            });
        }

        if (restartButton) {
            restartButton.addEventListener('click', () => {
                audioManager.unlock();
                if (typeof engine.restartRun === 'function') {
                    engine.restartRun();
                }
                document.body.style.cursor = 'none';
            });
        }

        // Menu button sounds
        menuButtons.forEach((button) => {
            button.addEventListener('mouseenter', () => {
                audioManager.play('menu_hover');
            });
            button.addEventListener('click', () => {
                audioManager.unlock();
                audioManager.play('menu_click');
            });
        });

        // Settings panel sounds
        settingsTabs.forEach((tab) => {
            tab.addEventListener('mouseenter', () => {
                audioManager.play('menu_hover');
            });
            tab.addEventListener('click', () => {
                audioManager.unlock();
                audioManager.play('menu_click');
            });
        });

        settingsInputs.forEach((input) => {
            input.addEventListener('mousedown', () => {
                audioManager.unlock();
                audioManager.play('menu_click');
            });
        });

        // Tab switching
        function switchTab(tabName) {
            // Remove active from all tabs and panels
            settingsTabs.forEach(tab => tab.classList.remove('active'));
            tabPanels.forEach(panel => panel.classList.remove('active'));
            
            // Activate selected tab and panel
            const selectedTab = document.querySelector(`.settings-tab[data-tab="${tabName}"]`);
            const selectedPanel = document.getElementById(`tab-${tabName}`);
            if (selectedTab) selectedTab.classList.add('active');
            if (selectedPanel) selectedPanel.classList.add('active');
        }

        // Tab click handlers
        settingsTabs.forEach(tab => {
            tab.addEventListener('click', () => {
                const tabName = tab.getAttribute('data-tab');
                switchTab(tabName);
            });
        });

        // Initialize with Video tab active
        switchTab('video');

        // FOV slider handler
        if (fovSlider && fovValue) {
            fovSlider.addEventListener('input', (e) => {
                const fovDegrees = parseFloat(e.target.value);
                fovValue.textContent = fovDegrees;
                if (engine && typeof engine.set_fov_degrees === 'function') {
                    engine.set_fov_degrees(fovDegrees);
                }
            });
        }

        // Mouse sensitivity slider handler
        if (sensitivitySlider && sensitivityValue) {
            sensitivitySlider.addEventListener('input', (e) => {
                const sensitivity = parseFloat(e.target.value);
                sensitivityValue.textContent = sensitivity.toFixed(4);
                if (engine && typeof engine.set_mouse_sensitivity === 'function') {
                    engine.set_mouse_sensitivity(sensitivity);
                }
            });
        }

        // ESC key to toggle menu
        window.addEventListener('keydown', (e) => {
            if (e.code === 'Escape') {
                if (menuState.settingsOpen) {
                    hideSettings();
                    if (menuState.menuOpen) {
                        showMainMenu();
                    }
                } else if (menuState.menuOpen && menuState.gameStarted) {
                    hideMainMenu();
                } else if (!menuState.menuOpen && menuState.gameStarted) {
                    showMainMenu();
                }
            }
        });

        // Handle resizing
        function resize() {
            canvas.width = window.innerWidth;
            canvas.height = window.innerHeight;
            engine.resize(window.innerWidth, window.innerHeight);
        }
        window.addEventListener('resize', resize);

        // Input handling with gating
        window.addEventListener('keydown', (e) => {
            if (!menuState.menuOpen && !menuState.settingsOpen) {
                engine.on_key_down(e.code);
            }
        });

        window.addEventListener('keyup', (e) => {
            if (!menuState.menuOpen && !menuState.settingsOpen) {
                engine.on_key_up(e.code);
            }
        });

        // Pointer lock for first-person mouse look
        let pointerLocked = false;
        
        function requestPointerLock() {
            if (canvas.requestPointerLock) {
                canvas.requestPointerLock();
            } else if (canvas.mozRequestPointerLock) {
                canvas.mozRequestPointerLock();
            } else if (canvas.webkitRequestPointerLock) {
                canvas.webkitRequestPointerLock();
            }
        }
        
        function exitPointerLock() {
            if (document.exitPointerLock) {
                document.exitPointerLock();
            } else if (document.mozExitPointerLock) {
                document.mozExitPointerLock();
            } else if (document.webkitExitPointerLock) {
                document.webkitExitPointerLock();
            }
        }
        
        // Handle pointer lock change events
        function handlePointerLockChange() {
            pointerLocked = !!(document.pointerLockElement || document.mozPointerLockElement || document.webkitPointerLockElement);
        }
        
        document.addEventListener('pointerlockchange', handlePointerLockChange);
        document.addEventListener('mozpointerlockchange', handlePointerLockChange);
        document.addEventListener('webkitpointerlockchange', handlePointerLockChange);
        
        // Request pointer lock on canvas click (when game is running)
        canvas.addEventListener('click', () => {
            if (!menuState.menuOpen && !menuState.settingsOpen && menuState.gameStarted) {
                requestPointerLock();
            }
        });
        
        // ESC key exits pointer lock
        window.addEventListener('keydown', (e) => {
            if (e.code === 'Escape' && pointerLocked) {
                exitPointerLock();
            }
        });

        // Mouse handling with gating
        let lastMouseX = 0;
        let lastMouseY = 0;
        
        window.addEventListener('mousemove', (e) => {
            if (!menuState.menuOpen && !menuState.settingsOpen) {
                if (pointerLocked) {
                    // Use movementX/Y for relative movement when pointer locked
                    const dx = e.movementX || 0;
                    const dy = e.movementY || 0;
                    engine.on_mouse_delta(dx, dy);
                } else {
                    // Fallback to absolute position when not locked
                    engine.on_mouse_move(e.clientX, e.clientY);
                    lastMouseX = e.clientX;
                    lastMouseY = e.clientY;
                }
            }
        });

        window.addEventListener('mousedown', (e) => {
            if (!menuState.menuOpen && !menuState.settingsOpen) {
                engine.on_mouse_down(e.button);
                // Request pointer lock on first click
                if (!pointerLocked && menuState.gameStarted) {
                    requestPointerLock();
                }
            }
        });

        window.addEventListener('mouseup', (e) => {
            if (!menuState.menuOpen && !menuState.settingsOpen) {
                engine.on_mouse_up(e.button);
            }
        });

        // Hide loading screen
        const loading = document.getElementById('loading');
        if (loading) loading.style.display = 'none';

        // Show main menu initially
        showMainMenu();

        // Game loop
        function loop(timestamp) {
            if (menuState.loopRunning && !menuState.menuOpen && !menuState.settingsOpen) {
                engine.tick(timestamp);
            }
            if (menuState.loopRunning) {
                requestAnimationFrame(loop);
            }
        }

        // Don't start loop automatically - wait for "Start Game" button

    } catch (e) {
        console.error("Failed to start engine:", e);
        const loading = document.getElementById('loading');
        if (loading) {
            loading.textContent = "ERROR: " + e;
            loading.style.color = "red";
        }
    }
}

run();
