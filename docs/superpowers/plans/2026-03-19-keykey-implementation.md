# KeyKey implementation plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a Tauri v2 keystroke visualizer with text stream and visual keyboard modes, smart modifier handling, multi-monitor support, and a full theme system.

**Architecture:** Tauri v2 app with Rust backend (rdev for keyboard hooks, windows crate for monitor APIs) and Svelte/TypeScript frontend. Single overlay window repositions across monitors. Settings window created lazily from system tray. Config stored as JSON in `%APPDATA%/keykey/`.

**Tech stack:** Rust, Tauri v2, rdev, Svelte 5, TypeScript, Vite, CSS variables

**Spec:** `docs/superpowers/specs/2026-03-19-keykey-design.md`

---

## File structure

```
keykey/
├── package.json
├── tsconfig.json
├── vite.config.ts
├── svelte.config.js
├── index.html
├── src/
│   ├── main.ts                          # Vite entry point
│   ├── overlay.html                     # Overlay window entry
│   ├── overlay-main.ts                  # Overlay window bootstrap
│   ├── App.svelte                       # Settings window root
│   ├── lib/
│   │   ├── types.ts                     # Shared TypeScript types
│   │   ├── theme-engine.ts              # CSS variable theme application
│   │   ├── stores/
│   │   │   ├── keystream.ts             # Reactive keystroke store
│   │   │   └── config.ts                # Config store (syncs with backend)
│   │   ├── overlay/
│   │   │   ├── Overlay.svelte           # Overlay root (mode switcher)
│   │   │   ├── TextStream.svelte        # Text stream display mode
│   │   │   ├── KeyLabel.svelte          # Single keystroke label
│   │   │   ├── VisualKeyboard.svelte    # Visual keyboard display mode
│   │   │   └── KeyCap.svelte            # Single key cap in keyboard
│   │   ├── settings/
│   │   │   ├── Settings.svelte          # Settings window root
│   │   │   ├── DisplaySettings.svelte   # Display mode, positioning
│   │   │   ├── InputSettings.svelte     # Smart mode, modifier config
│   │   │   ├── AppearanceSettings.svelte # Theme selector, editor
│   │   │   ├── ShortcutSettings.svelte  # Global hotkey config
│   │   │   └── AboutSettings.svelte     # Version, links
│   │   └── keyboard-layouts/
│   │       └── ansi-104.ts              # ANSI 104-key layout data
│   └── styles/
│       ├── overlay.css                  # Overlay base styles
│       └── settings.css                 # Settings window styles
├── src-tauri/
│   ├── Cargo.toml
│   ├── build.rs
│   ├── tauri.conf.json
│   ├── capabilities/
│   │   └── default.json
│   ├── icons/
│   └── src/
│       ├── main.rs                      # Desktop entry point
│       ├── lib.rs                       # App builder, plugin registration
│       ├── keyboard/
│       │   ├── mod.rs                   # Keyboard module root
│       │   ├── listener.rs              # rdev hook, raw event capture
│       │   └── processor.rs             # Smart mode, combo detection, repeat compression
│       ├── monitor/
│       │   ├── mod.rs                   # Monitor module root
│       │   └── tracker.rs               # Monitor enumeration, focus tracking, positioning
│       ├── config/
│       │   ├── mod.rs                   # Config module root
│       │   ├── schema.rs                # Config structs (serde)
│       │   └── store.rs                 # Load/save/migrate config
│       ├── tray.rs                      # System tray setup
│       └── commands.rs                  # Tauri IPC commands
└── docs/
    └── superpowers/
        ├── specs/
        │   └── 2026-03-19-keykey-design.md
        └── plans/
            └── 2026-03-19-keykey-implementation.md
```

---

## Task 1: Environment setup and project scaffold

**Files:**
- Create: all scaffolded files from `npm create tauri-app@latest`
- Modify: `src-tauri/Cargo.toml`, `src-tauri/tauri.conf.json`

### Prerequisites

Rust must be installed. If `rustc --version` fails:

- [ ] **Step 1: Install Rust**

```bash
# Download and run rustup-init.exe from https://rustup.rs
# Or via winget:
winget install Rustlang.Rustup
```

After install, restart the terminal and verify:
```bash
rustc --version
cargo --version
```

- [ ] **Step 2: Scaffold the Tauri project**

```bash
cd "C:/Users/amdit/OneDrive/Desktop/Crimes/playground"
npm create tauri-app@latest keykey -- --template svelte-ts --manager npm
cd keykey
```

If the `--template` flag isn't supported by the version installed, run interactively and select: TypeScript/JavaScript (npm) > npm > Svelte > TypeScript.

- [ ] **Step 3: Install frontend dependencies**

```bash
npm install
```

- [ ] **Step 4: Add Rust dependencies**

Replace `src-tauri/Cargo.toml` dependencies section:

```toml
[dependencies]
tauri = { version = "2", features = ["tray-icon"] }
tauri-plugin-opener = "2"
tauri-plugin-global-shortcut = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
rdev = "0.5"
dirs = "6"

[target.'cfg(windows)'.dependencies]
windows = { version = "0.58", features = [
    "Win32_Foundation",
    "Win32_Graphics_Gdi",
    "Win32_UI_HiDpi",
    "Win32_UI_WindowsAndMessaging",
] }

[build-dependencies]
tauri-build = { version = "2", features = [] }
```

- [ ] **Step 5: Install the global-shortcut frontend package**

```bash
npm install @tauri-apps/plugin-global-shortcut
```

- [ ] **Step 6: Verify the project builds**

```bash
npm run tauri dev
```

Expected: a Tauri window opens with the default Svelte template. Close it.

- [ ] **Step 7: Commit**

```bash
git init
git add .
git commit -m "feat: scaffold Tauri v2 project with Svelte/TypeScript"
```

---

## Task 2: Overlay window

**Files:**
- Create: `src/overlay.html`, `src/overlay-main.ts`, `src/lib/overlay/Overlay.svelte`, `src/styles/overlay.css`
- Modify: `src-tauri/tauri.conf.json`, `src-tauri/src/lib.rs`, `src-tauri/capabilities/default.json`, `vite.config.ts`

- [ ] **Step 1: Create the overlay HTML entry point**

Create `src/overlay.html`:
```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title></title>
  <link rel="stylesheet" href="/src/styles/overlay.css" />
</head>
<body>
  <div id="overlay"></div>
  <script type="module" src="/src/overlay-main.ts"></script>
</body>
</html>
```

- [ ] **Step 2: Create the overlay bootstrap**

Create `src/overlay-main.ts`:
```typescript
import Overlay from './lib/overlay/Overlay.svelte';
import { mount } from 'svelte';

const app = mount(Overlay, {
  target: document.getElementById('overlay')!,
});

export default app;
```

- [ ] **Step 3: Create the overlay base styles**

Create `src/styles/overlay.css`:
```css
html, body {
  margin: 0;
  padding: 0;
  background: transparent;
  overflow: hidden;
  user-select: none;
  pointer-events: none;
}

#overlay {
  width: 100vw;
  height: 100vh;
  position: relative;
}
```

- [ ] **Step 4: Create the Overlay Svelte component**

Create `src/lib/overlay/Overlay.svelte`:
```svelte
<script lang="ts">
  // Placeholder — will listen for key events in Task 5
</script>

<div class="overlay-container">
  <div class="debug-label">KeyKey overlay active</div>
</div>

<style>
  .overlay-container {
    position: fixed;
    bottom: 32px;
    right: 32px;
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    gap: 4px;
  }

  .debug-label {
    background: rgba(0, 0, 0, 0.7);
    color: #fff;
    padding: 8px 16px;
    border-radius: 6px;
    font-family: system-ui, sans-serif;
    font-size: 14px;
  }
</style>
```

- [ ] **Step 5: Configure Vite for multi-page**

Update `vite.config.ts` to add the overlay as a second entry:
```typescript
import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import { resolve } from "path";

export default defineConfig({
  plugins: [svelte()],
  build: {
    rollupOptions: {
      input: {
        main: resolve(__dirname, "index.html"),
        overlay: resolve(__dirname, "src/overlay.html"),
      },
    },
  },
});
```

- [ ] **Step 6: Configure the overlay window in tauri.conf.json**

Add a second window to `app.windows` in `src-tauri/tauri.conf.json`:
```json
{
  "label": "overlay",
  "title": "",
  "url": "/src/overlay.html",
  "transparent": true,
  "decorations": false,
  "alwaysOnTop": true,
  "skipTaskbar": true,
  "resizable": false,
  "visible": true,
  "focus": false,
  "width": 400,
  "height": 300
}
```

Note: `width`/`height` are initial size — we'll resize dynamically based on content and monitor later. We set `visible: true` for now so we can see it during development.

- [ ] **Step 7: Set click-through in Rust setup**

Update `src-tauri/src/lib.rs`:
```rust
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Make overlay click-through
            if let Some(overlay) = app.get_webview_window("overlay") {
                overlay.set_ignore_cursor_events(true)?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

- [ ] **Step 8: Add permissions**

Update `src-tauri/capabilities/default.json` to include:
```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "default",
  "description": "Default capabilities",
  "windows": ["main", "overlay"],
  "permissions": [
    "core:default",
    "opener:default",
    "core:window:default",
    "core:window:allow-set-ignore-cursor-events",
    "core:window:allow-show",
    "core:window:allow-hide",
    "core:window:allow-set-position",
    "core:window:allow-set-size"
  ]
}
```

- [ ] **Step 9: Test overlay window**

```bash
npm run tauri dev
```

Expected: Two windows appear — the main Svelte window and a transparent overlay window showing "KeyKey overlay active" as a dark floating label. The overlay should not be clickable (mouse passes through to windows below). The overlay should have no title bar and no taskbar entry.

- [ ] **Step 10: Commit**

```bash
git add .
git commit -m "feat: add transparent click-through overlay window"
```

---

## Task 3: System tray

**Files:**
- Modify: `src-tauri/src/lib.rs`
- Modify: `src-tauri/tauri.conf.json`

- [ ] **Step 1: Remove the main window from tauri.conf.json**

Remove the `"main"` window entry from `app.windows` in `tauri.conf.json`. The settings window will be created lazily via `WebviewWindowBuilder` when the user clicks "Settings" in the tray menu. Only the overlay window should be in `tauri.conf.json`. Also update capabilities `windows` to `["overlay", "settings"]`.

- [ ] **Step 2: Add tray setup to lib.rs**

Update `src-tauri/src/lib.rs`:
```rust
use tauri::{
    Manager,
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
};

fn setup_tray(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let toggle_i = MenuItem::with_id(app, "toggle", "Pause capture", true, None::<&str>)?;
    let switch_mode_i = MenuItem::with_id(app, "switch_mode", "Switch to visual keyboard", true, None::<&str>)?;
    let settings_i = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&toggle_i, &switch_mode_i, &settings_i, &separator, &quit_i])?;

    let _tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .menu_on_left_click(false)
        .tooltip("KeyKey - active")
        .on_menu_event(|app, event| {
            match event.id().as_ref() {
                "toggle" => {
                    if let Some(overlay) = app.get_webview_window("overlay") {
                        if overlay.is_visible().unwrap_or(false) {
                            let _ = overlay.hide();
                            // Update menu item text and tooltip
                            // (Tray icon swap and menu text update will be
                            // refined once we have icon assets — for now,
                            // emitting an event so the frontend can track state)
                            let _ = app.emit("capture-toggled", false);
                        } else {
                            let _ = overlay.show();
                            let _ = app.emit("capture-toggled", true);
                        }
                    }
                }
                "switch_mode" => {
                    // Emit event — overlay listens and toggles mode
                    let _ = app.emit("switch-display-mode", ());
                }
                "settings" => {
                    // Lazy-create settings window (see issue #20 fix)
                    use tauri::WebviewUrl;
                    if let Some(existing) = app.get_webview_window("settings") {
                        let _ = existing.show();
                        let _ = existing.set_focus();
                    } else {
                        let _ = tauri::WebviewWindowBuilder::new(
                            app,
                            "settings",
                            WebviewUrl::App("index.html".into()),
                        )
                        .title("KeyKey settings")
                        .inner_size(700.0, 500.0)
                        .build();
                    }
                }
                "quit" => {
                    app.exit(0);
                }
                _ => {}
            }
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                // Left-click toggles settings window
                if let Some(win) = app.get_webview_window("settings") {
                    if win.is_visible().unwrap_or(false) {
                        let _ = win.hide();
                    } else {
                        let _ = win.show();
                        let _ = win.set_focus();
                    }
                } else {
                    use tauri::WebviewUrl;
                    let _ = tauri::WebviewWindowBuilder::new(
                        app,
                        "settings",
                        WebviewUrl::App("index.html".into()),
                    )
                    .title("KeyKey settings")
                    .inner_size(700.0, 500.0)
                    .build();
                }
            }
        })
        .build(app)?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            if let Some(overlay) = app.get_webview_window("overlay") {
                overlay.set_ignore_cursor_events(true)?;
            }

            setup_tray(app)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

- [ ] **Step 3: Test tray**

```bash
npm run tauri dev
```

Expected: App icon appears in system tray. Right-click shows menu with "Pause capture", "Switch to visual keyboard", "Settings", and "Quit". Clicking "Settings" creates and shows the settings window. Clicking "Quit" exits. Left-clicking the tray icon toggles the settings window.

- [ ] **Step 4: Commit**

```bash
git add .
git commit -m "feat: add system tray with context menu"
```

---

## Task 4: Keyboard listener

**Files:**
- Create: `src-tauri/src/keyboard/mod.rs`, `src-tauri/src/keyboard/listener.rs`
- Modify: `src-tauri/src/lib.rs`

- [ ] **Step 1: Create the keyboard module**

Create `src-tauri/src/keyboard/mod.rs`:
```rust
pub mod listener;
```

- [ ] **Step 2: Write the keyboard listener**

Create `src-tauri/src/keyboard/listener.rs`:
```rust
use rdev::{listen, Event, EventType, Key};
use serde::Serialize;
use std::sync::mpsc;
use std::thread;
use tauri::{AppHandle, Emitter};

#[derive(Debug, Clone, Serialize)]
pub struct KeyEvent {
    pub key: String,
    pub key_code: String,
    pub event_type: String, // "press" or "release"
    pub timestamp: u64,
}

fn key_to_display_name(key: &Key, name: &Option<String>) -> String {
    match key {
        Key::ControlLeft | Key::ControlRight => "Ctrl".to_string(),
        Key::ShiftLeft | Key::ShiftRight => "Shift".to_string(),
        Key::Alt => "Alt".to_string(),
        Key::AltGr => "AltGr".to_string(),
        Key::MetaLeft | Key::MetaRight => "Win".to_string(),
        Key::Return => "Enter".to_string(),
        Key::Space => "Space".to_string(),
        Key::Backspace => "Backspace".to_string(),
        Key::Tab => "Tab".to_string(),
        Key::Escape => "Esc".to_string(),
        Key::Delete => "Del".to_string(),
        Key::UpArrow => "Up".to_string(),
        Key::DownArrow => "Down".to_string(),
        Key::LeftArrow => "Left".to_string(),
        Key::RightArrow => "Right".to_string(),
        Key::Home => "Home".to_string(),
        Key::End => "End".to_string(),
        Key::PageUp => "PgUp".to_string(),
        Key::PageDown => "PgDn".to_string(),
        Key::CapsLock => "CapsLock".to_string(),
        Key::PrintScreen => "PrtSc".to_string(),
        Key::ScrollLock => "ScrLk".to_string(),
        Key::Pause => "Pause".to_string(),
        Key::NumLock => "NumLock".to_string(),
        Key::F1 => "F1".to_string(),
        Key::F2 => "F2".to_string(),
        Key::F3 => "F3".to_string(),
        Key::F4 => "F4".to_string(),
        Key::F5 => "F5".to_string(),
        Key::F6 => "F6".to_string(),
        Key::F7 => "F7".to_string(),
        Key::F8 => "F8".to_string(),
        Key::F9 => "F9".to_string(),
        Key::F10 => "F10".to_string(),
        Key::F11 => "F11".to_string(),
        Key::F12 => "F12".to_string(),
        _ => {
            // Use the layout-aware name if available
            if let Some(n) = name {
                if !n.is_empty() {
                    return n.to_uppercase();
                }
            }
            format!("{:?}", key)
        }
    }
}

fn key_to_code(key: &Key) -> String {
    format!("{:?}", key)
}

fn is_modifier(key: &Key) -> bool {
    matches!(
        key,
        Key::ControlLeft
            | Key::ControlRight
            | Key::ShiftLeft
            | Key::ShiftRight
            | Key::Alt
            | Key::AltGr
            | Key::MetaLeft
            | Key::MetaRight
    )
}

pub fn start_listener(app_handle: AppHandle) {
    let (tx, rx) = mpsc::channel::<Event>();

    // Spawn the rdev listener thread (blocking)
    thread::spawn(move || {
        if let Err(e) = listen(move |event| {
            match &event.event_type {
                EventType::KeyPress(_) | EventType::KeyRelease(_) => {
                    let _ = tx.send(event);
                }
                _ => {}
            }
        }) {
            eprintln!("rdev listener error: {:?}", e);
        }
    });

    // Spawn the event forwarding thread
    thread::spawn(move || {
        while let Ok(event) = rx.recv() {
            let (key, event_type_str) = match &event.event_type {
                EventType::KeyPress(k) => (k, "press"),
                EventType::KeyRelease(k) => (k, "release"),
                _ => continue,
            };

            let key_event = KeyEvent {
                key: key_to_display_name(key, &event.name),
                key_code: key_to_code(key),
                event_type: event_type_str.to_string(),
                timestamp: event
                    .time
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis() as u64,
            };

            let _ = app_handle.emit("key-event", &key_event);
        }
    });
}
```

- [ ] **Step 3: Register the listener on startup**

Add to `src-tauri/src/lib.rs`:
```rust
mod keyboard;

// Inside setup closure, after tray setup:
keyboard::listener::start_listener(app.handle().clone());
```

- [ ] **Step 4: Add a debug listener in the overlay**

Update `src/lib/overlay/Overlay.svelte`:
```svelte
<script lang="ts">
  import { listen } from '@tauri-apps/api/event';
  import { onMount, onDestroy } from 'svelte';

  interface KeyEvent {
    key: string;
    key_code: string;
    event_type: string;
    timestamp: number;
  }

  let lastKey = $state('waiting for input...');
  let unlisten: (() => void) | null = null;

  onMount(async () => {
    unlisten = await listen<KeyEvent>('key-event', (event) => {
      if (event.payload.event_type === 'press') {
        lastKey = event.payload.key;
      }
    });
  });

  onDestroy(() => {
    if (unlisten) unlisten();
  });
</script>

<div class="overlay-container">
  <div class="debug-label">{lastKey}</div>
</div>

<style>
  .overlay-container {
    position: fixed;
    bottom: 32px;
    right: 32px;
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    gap: 4px;
  }

  .debug-label {
    background: rgba(0, 0, 0, 0.7);
    color: #fff;
    padding: 8px 16px;
    border-radius: 6px;
    font-family: system-ui, sans-serif;
    font-size: 14px;
  }
</style>
```

- [ ] **Step 5: Add event listener permission**

Add `"core:event:default"` to `src-tauri/capabilities/default.json` permissions array.

- [ ] **Step 6: Test keyboard capture**

```bash
npm run tauri dev
```

Expected: The overlay label updates to show the last key pressed, in real-time, regardless of which application has focus. Press Ctrl, Shift, Alt — they should show as "Ctrl", "Shift", "Alt". Press letter keys — they should show the uppercase letter.

- [ ] **Step 7: Commit**

```bash
git add .
git commit -m "feat: add global keyboard listener with rdev"
```

---

## Task 5: Event processor (smart mode)

**Files:**
- Create: `src-tauri/src/keyboard/processor.rs`
- Modify: `src-tauri/src/keyboard/mod.rs`, `src-tauri/src/keyboard/listener.rs`

- [ ] **Step 1: Write processor tests**

Create `src-tauri/src/keyboard/processor.rs` with tests at the bottom:
```rust
use std::collections::HashSet;
use std::time::{Duration, Instant};
use serde::Serialize;

/// A processed keystroke event ready for display
#[derive(Debug, Clone, Serialize)]
pub struct DisplayEvent {
    /// Full display text, e.g. "Ctrl + Shift + S"
    pub label: String,
    /// Whether this is a combo (modifier + key)
    pub is_combo: bool,
    /// Unique ID for deduplication/repeat counting
    pub id: u64,
    /// Timestamp
    pub timestamp: u64,
}

/// Per-modifier display mode
#[derive(Debug, Clone, PartialEq)]
pub enum ModifierMode {
    Smart,       // Show standalone only if held > threshold
    AlwaysShow,  // Show every press
    NeverShow,   // Suppress entirely
    ComboOnly,   // Only show when part of a combo
}

/// Tracks modifier state and processes raw key events into display events
pub struct EventProcessor {
    held_modifiers: HashSet<String>,
    modifier_press_times: std::collections::HashMap<String, Instant>,
    modifier_used_in_combo: HashSet<String>,
    smart_threshold_ms: u64,
    show_all_keystrokes: bool,
    modifier_modes: std::collections::HashMap<String, ModifierMode>,
    last_key: Option<String>,
    last_key_time: Option<Instant>,
    repeat_count: u32,
    repeat_window_ms: u64,
    next_id: u64,
}

impl EventProcessor {
    pub fn new() -> Self {
        let mut modifier_modes = std::collections::HashMap::new();
        for m in &["Ctrl", "Shift", "Alt", "AltGr", "Win"] {
            modifier_modes.insert(m.to_string(), ModifierMode::Smart);
        }

        Self {
            held_modifiers: HashSet::new(),
            modifier_press_times: std::collections::HashMap::new(),
            modifier_used_in_combo: HashSet::new(),
            smart_threshold_ms: 200,
            show_all_keystrokes: true,
            modifier_modes,
            last_key: None,
            last_key_time: None,
            repeat_count: 1,
            repeat_window_ms: 500,
            next_id: 0,
        }
    }

    pub fn set_smart_threshold_ms(&mut self, ms: u64) {
        self.smart_threshold_ms = ms;
    }

    pub fn set_repeat_window_ms(&mut self, ms: u64) {
        self.repeat_window_ms = ms;
    }

    pub fn set_show_all_keystrokes(&mut self, enabled: bool) {
        self.show_all_keystrokes = enabled;
    }

    pub fn set_modifier_mode(&mut self, modifier: &str, mode: ModifierMode) {
        self.modifier_modes.insert(modifier.to_string(), mode);
    }

    fn next_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    fn is_modifier(key: &str) -> bool {
        matches!(key, "Ctrl" | "Shift" | "Alt" | "AltGr" | "Win")
    }

    fn get_modifier_mode(&self, key: &str) -> &ModifierMode {
        self.modifier_modes.get(key).unwrap_or(&ModifierMode::Smart)
    }

    /// Process a key press. Returns Some(DisplayEvent) if this should be shown.
    pub fn on_key_press(&mut self, key: &str, timestamp: u64) -> Option<DisplayEvent> {
        let now = Instant::now();

        if Self::is_modifier(key) {
            self.held_modifiers.insert(key.to_string());
            self.modifier_press_times.insert(key.to_string(), now);

            // AlwaysShow modifiers produce a display event immediately on press
            if *self.get_modifier_mode(key) == ModifierMode::AlwaysShow {
                let id = self.next_id();
                return Some(DisplayEvent {
                    label: key.to_string(),
                    is_combo: false,
                    id,
                    timestamp,
                });
            }

            return None;
        }

        // Non-modifier key pressed
        let mut parts: Vec<String> = Vec::new();

        // Add held modifiers in canonical order (skip NeverShow modifiers)
        for m in &["Ctrl", "Alt", "Shift", "Win"] {
            if self.held_modifiers.contains(*m) {
                let mode = self.get_modifier_mode(m).clone();
                if mode != ModifierMode::NeverShow {
                    parts.push(m.to_string());
                    self.modifier_used_in_combo.insert(m.to_string());
                }
            }
        }

        let is_combo = !parts.is_empty();

        // If show_all_keystrokes is false, only show combos
        if !self.show_all_keystrokes && !is_combo {
            return None;
        }

        parts.push(key.to_string());
        let label = parts.join(" + ");

        // Check for repeat
        let is_repeat = if let (Some(ref last), Some(last_time)) =
            (&self.last_key, self.last_key_time)
        {
            last == &label
                && now.duration_since(last_time)
                    < Duration::from_millis(self.repeat_window_ms)
        } else {
            false
        };

        if is_repeat {
            self.repeat_count += 1;
            self.last_key_time = Some(now);
            Some(DisplayEvent {
                label: format!("{} x{}", label, self.repeat_count),
                is_combo,
                id: self.next_id.saturating_sub(1),
                timestamp,
            })
        } else {
            self.last_key = Some(label.clone());
            self.last_key_time = Some(now);
            self.repeat_count = 1;
            let id = self.next_id();
            Some(DisplayEvent {
                label,
                is_combo,
                id,
                timestamp,
            })
        }
    }

    /// Process a key release. Returns Some(DisplayEvent) for standalone modifier display.
    pub fn on_key_release(&mut self, key: &str, timestamp: u64) -> Option<DisplayEvent> {
        if Self::is_modifier(key) {
            self.held_modifiers.remove(key);
            let was_used_in_combo = self.modifier_used_in_combo.remove(key);
            let mode = self.get_modifier_mode(key).clone();

            match mode {
                ModifierMode::NeverShow => return None,
                ModifierMode::ComboOnly => return None, // already shown as part of combo
                ModifierMode::AlwaysShow => return None, // already shown on press
                ModifierMode::Smart => {
                    if was_used_in_combo {
                        return None; // already shown as part of combo
                    }
                    // Show standalone modifier only if held longer than threshold
                    if let Some(press_time) = self.modifier_press_times.remove(key) {
                        let held_duration = Instant::now().duration_since(press_time);
                        if held_duration >= Duration::from_millis(self.smart_threshold_ms) {
                            let id = self.next_id();
                            return Some(DisplayEvent {
                                label: key.to_string(),
                                is_combo: false,
                                id,
                                timestamp,
                            });
                        }
                    }
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regular_key_press() {
        let mut proc = EventProcessor::new();
        let result = proc.on_key_press("A", 1000);
        assert!(result.is_some());
        let event = result.unwrap();
        assert_eq!(event.label, "A");
        assert!(!event.is_combo);
    }

    #[test]
    fn test_modifier_press_alone_produces_nothing() {
        let mut proc = EventProcessor::new();
        let result = proc.on_key_press("Ctrl", 1000);
        assert!(result.is_none());
    }

    #[test]
    fn test_combo_ctrl_c() {
        let mut proc = EventProcessor::new();
        proc.on_key_press("Ctrl", 1000);
        let result = proc.on_key_press("C", 1001);
        assert!(result.is_some());
        let event = result.unwrap();
        assert_eq!(event.label, "Ctrl + C");
        assert!(event.is_combo);
    }

    #[test]
    fn test_combo_ctrl_shift_s() {
        let mut proc = EventProcessor::new();
        proc.on_key_press("Ctrl", 1000);
        proc.on_key_press("Shift", 1001);
        let result = proc.on_key_press("S", 1002);
        assert!(result.is_some());
        let event = result.unwrap();
        assert_eq!(event.label, "Ctrl + Shift + S");
        assert!(event.is_combo);
    }

    #[test]
    fn test_modifier_canonical_order() {
        let mut proc = EventProcessor::new();
        // Press in reverse order
        proc.on_key_press("Win", 1000);
        proc.on_key_press("Shift", 1001);
        proc.on_key_press("Alt", 1002);
        proc.on_key_press("Ctrl", 1003);
        let result = proc.on_key_press("X", 1004);
        assert!(result.is_some());
        // Should be in canonical order regardless of press order
        assert_eq!(result.unwrap().label, "Ctrl + Alt + Shift + Win + X");
    }

    #[test]
    fn test_repeat_detection() {
        let mut proc = EventProcessor::new();
        let first = proc.on_key_press("A", 1000).unwrap();
        assert_eq!(first.label, "A");

        // Second press within 500ms window
        let second = proc.on_key_press("A", 1100).unwrap();
        assert_eq!(second.label, "A x2");

        let third = proc.on_key_press("A", 1200).unwrap();
        assert_eq!(third.label, "A x3");
    }

    #[test]
    fn test_different_key_breaks_repeat() {
        let mut proc = EventProcessor::new();
        proc.on_key_press("A", 1000);
        let result = proc.on_key_press("B", 1100).unwrap();
        assert_eq!(result.label, "B");
    }

    #[test]
    fn test_modifier_release_after_combo_no_display() {
        let mut proc = EventProcessor::new();
        proc.on_key_press("Ctrl", 1000);
        proc.on_key_press("C", 1001);
        // Releasing Ctrl after combo should NOT show standalone "Ctrl"
        // because the modifier was used in a combo
        let result = proc.on_key_release("Ctrl", 1002);
        assert!(result.is_none());
    }

    #[test]
    fn test_modifier_never_show() {
        let mut proc = EventProcessor::new();
        proc.set_modifier_mode("Shift", ModifierMode::NeverShow);
        proc.on_key_press("Shift", 1000);
        let result = proc.on_key_press("A", 1001);
        // Shift should be excluded from the combo label
        let event = result.unwrap();
        assert_eq!(event.label, "A");
        assert!(!event.is_combo);
    }

    #[test]
    fn test_modifier_always_show() {
        let mut proc = EventProcessor::new();
        proc.set_modifier_mode("Ctrl", ModifierMode::AlwaysShow);
        // AlwaysShow should produce a display event on press
        let result = proc.on_key_press("Ctrl", 1000);
        assert!(result.is_some());
        assert_eq!(result.unwrap().label, "Ctrl");
    }

    #[test]
    fn test_modifier_combo_only() {
        let mut proc = EventProcessor::new();
        proc.set_modifier_mode("Ctrl", ModifierMode::ComboOnly);
        // Press alone produces nothing
        let result = proc.on_key_press("Ctrl", 1000);
        assert!(result.is_none());
        // Release after no combo produces nothing
        let result = proc.on_key_release("Ctrl", 1050);
        assert!(result.is_none());
        // But still appears in combos
        proc.on_key_press("Ctrl", 2000);
        let result = proc.on_key_press("C", 2001).unwrap();
        assert_eq!(result.label, "Ctrl + C");
    }

    #[test]
    fn test_show_all_keystrokes_false() {
        let mut proc = EventProcessor::new();
        proc.set_show_all_keystrokes(false);
        // Regular key press should be suppressed
        let result = proc.on_key_press("A", 1000);
        assert!(result.is_none());
        // But combos should still show
        proc.on_key_press("Ctrl", 2000);
        let result = proc.on_key_press("S", 2001);
        assert!(result.is_some());
        assert_eq!(result.unwrap().label, "Ctrl + S");
    }
}
```

- [ ] **Step 2: Run the tests**

```bash
cd src-tauri
cargo test
```

Expected: all tests pass.

- [ ] **Step 3: Update the listener to use the processor**

Update `src-tauri/src/keyboard/listener.rs` to use `EventProcessor`.

Replace the event forwarding thread. **Important:** emit BOTH `key-event` (raw, needed by visual keyboard mode) and `display-event` (processed, needed by text stream mode):
```rust
use super::processor::{EventProcessor, DisplayEvent};

// In start_listener, replace the forwarding thread:
thread::spawn(move || {
    let mut processor = EventProcessor::new();

    while let Ok(event) = rx.recv() {
        let (key, is_press) = match &event.event_type {
            EventType::KeyPress(k) => (k, true),
            EventType::KeyRelease(k) => (k, false),
            _ => continue,
        };

        let display_name = key_to_display_name(key, &event.name);
        let timestamp = event
            .time
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        // Always emit raw key event (visual keyboard needs press/release for every key)
        let raw_event = KeyEvent {
            key: display_name.clone(),
            key_code: key_to_code(key),
            event_type: if is_press { "press" } else { "release" }.to_string(),
            timestamp,
        };
        let _ = app_handle.emit("key-event", &raw_event);

        // Emit processed display event (text stream uses this)
        let display_event = if is_press {
            processor.on_key_press(&display_name, timestamp)
        } else {
            processor.on_key_release(&display_name, timestamp)
        };

        if let Some(evt) = display_event {
            let _ = app_handle.emit("display-event", &evt);
        }
    }
});
```

- [ ] **Step 4: Update keyboard/mod.rs**

```rust
pub mod listener;
pub mod processor;
```

- [ ] **Step 5: Update overlay to listen for display events**

Update `src/lib/overlay/Overlay.svelte` to listen for `display-event` instead of `key-event`:
```svelte
<script lang="ts">
  import { listen } from '@tauri-apps/api/event';
  import { onMount, onDestroy } from 'svelte';

  interface DisplayEvent {
    label: string;
    is_combo: boolean;
    id: number;
    timestamp: number;
  }

  let lastLabel = $state('waiting for input...');
  let unlisten: (() => void) | null = null;

  onMount(async () => {
    unlisten = await listen<DisplayEvent>('display-event', (event) => {
      lastLabel = event.payload.label;
    });
  });

  onDestroy(() => {
    if (unlisten) unlisten();
  });
</script>

<div class="overlay-container">
  <div class="debug-label">{lastLabel}</div>
</div>

<style>
  .overlay-container {
    position: fixed;
    bottom: 32px;
    right: 32px;
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    gap: 4px;
  }

  .debug-label {
    background: rgba(0, 0, 0, 0.7);
    color: #fff;
    padding: 8px 16px;
    border-radius: 6px;
    font-family: system-ui, sans-serif;
    font-size: 14px;
  }
</style>
```

- [ ] **Step 6: Test smart mode end-to-end**

```bash
cd src-tauri && cargo test && cd .. && npm run tauri dev
```

Expected: Tests pass. In the running app, pressing `Ctrl+C` shows "Ctrl + C". Pressing `A` shows "A". Quick Shift taps (during typing) don't show. Holding Shift for 200ms+ then releasing shows "Shift".

- [ ] **Step 7: Commit**

```bash
git add .
git commit -m "feat: add event processor with smart mode and repeat detection"
```

---

## Task 6: Text stream display

**Files:**
- Create: `src/lib/types.ts`, `src/lib/stores/keystream.ts`, `src/lib/overlay/KeyLabel.svelte`, `src/lib/overlay/TextStream.svelte`
- Modify: `src/lib/overlay/Overlay.svelte`

- [ ] **Step 1: Create shared types**

Create `src/lib/types.ts`:
```typescript
export interface DisplayEvent {
  label: string;
  is_combo: boolean;
  id: number;
  timestamp: number;
}

export interface StreamEntry {
  label: string;
  id: number;
  is_combo: boolean;
  created_at: number;
  repeat_count: number;
}
```

- [ ] **Step 2: Create the keystream store**

Create `src/lib/stores/keystream.ts`:
```typescript
import { writable, derived } from 'svelte/store';
import type { DisplayEvent, StreamEntry } from '../types';

const MAX_VISIBLE = 5;
const FADE_DURATION_MS = 2000;

function createKeystream() {
  const { subscribe, update } = writable<StreamEntry[]>([]);

  let cleanupTimer: ReturnType<typeof setInterval> | null = null;

  function startCleanup() {
    if (cleanupTimer) return;
    cleanupTimer = setInterval(() => {
      const now = Date.now();
      update((entries) =>
        entries.filter((e) => now - e.created_at < FADE_DURATION_MS)
      );
    }, 100);
  }

  function stopCleanup() {
    if (cleanupTimer) {
      clearInterval(cleanupTimer);
      cleanupTimer = null;
    }
  }

  return {
    subscribe,
    push(event: DisplayEvent) {
      startCleanup();
      update((entries) => {
        // Check if this is a repeat update (same id as last entry)
        if (entries.length > 0 && entries[entries.length - 1].id === event.id) {
          // Update the last entry's label (repeat count is in the label)
          const updated = [...entries];
          updated[updated.length - 1] = {
            ...updated[updated.length - 1],
            label: event.label,
            repeat_count: updated[updated.length - 1].repeat_count + 1,
          };
          return updated;
        }

        // New entry
        const newEntry: StreamEntry = {
          label: event.label,
          id: event.id,
          is_combo: event.is_combo,
          created_at: Date.now(),
          repeat_count: 1,
        };

        const updated = [...entries, newEntry];
        // Keep only the last MAX_VISIBLE
        return updated.slice(-MAX_VISIBLE);
      });
    },
    clear() {
      update(() => []);
    },
    destroy() {
      stopCleanup();
    },
  };
}

export const keystream = createKeystream();
```

- [ ] **Step 3: Create the KeyLabel component**

Create `src/lib/overlay/KeyLabel.svelte`:
```svelte
<script lang="ts">
  import type { StreamEntry } from '../types';

  interface Props {
    entry: StreamEntry;
    fadeMs: number;
  }

  let { entry, fadeMs }: Props = $props();

  let opacity = $state(1);
  let elapsed = $derived(Date.now() - entry.created_at);

  $effect(() => {
    const interval = setInterval(() => {
      const age = Date.now() - entry.created_at;
      const fadeStart = fadeMs * 0.6; // start fading at 60% of lifetime
      if (age > fadeStart) {
        opacity = Math.max(0, 1 - (age - fadeStart) / (fadeMs * 0.4));
      }
    }, 50);

    return () => clearInterval(interval);
  });
</script>

<div
  class="key-label"
  class:combo={entry.is_combo}
  style="opacity: {opacity}"
>
  {entry.label}
</div>

<style>
  .key-label {
    background: var(--kk-bg, rgba(0, 0, 0, 0.75));
    color: var(--kk-text, #ffffff);
    padding: var(--kk-padding-y, 6px) var(--kk-padding-x, 14px);
    border-radius: var(--kk-radius, 6px);
    font-family: var(--kk-font, system-ui, -apple-system, sans-serif);
    font-size: var(--kk-font-size, 15px);
    font-weight: var(--kk-font-weight, 500);
    border: var(--kk-border-width, 0px) solid var(--kk-border-color, transparent);
    box-shadow: var(--kk-shadow, none);
    white-space: nowrap;
    transition: opacity 0.15s ease-out;
    animation: kk-enter 0.15s ease-out;
  }

  .combo {
    font-weight: var(--kk-combo-font-weight, 600);
  }

  @keyframes kk-enter {
    from {
      opacity: 0;
      transform: translateY(8px) scale(0.95);
    }
    to {
      opacity: 1;
      transform: translateY(0) scale(1);
    }
  }
</style>
```

- [ ] **Step 4: Create the TextStream component**

Create `src/lib/overlay/TextStream.svelte`:
```svelte
<script lang="ts">
  import { keystream } from '../stores/keystream';
  import KeyLabel from './KeyLabel.svelte';

  const FADE_MS = 2000;
</script>

<div class="text-stream">
  {#each $keystream as entry (entry.id)}
    <KeyLabel {entry} fadeMs={FADE_MS} />
  {/each}
</div>

<style>
  .text-stream {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    gap: var(--kk-gap, 4px);
  }
</style>
```

- [ ] **Step 5: Wire up Overlay.svelte**

Replace `src/lib/overlay/Overlay.svelte`:
```svelte
<script lang="ts">
  import { listen } from '@tauri-apps/api/event';
  import { onMount, onDestroy } from 'svelte';
  import { keystream } from '../stores/keystream';
  import TextStream from './TextStream.svelte';
  import type { DisplayEvent } from '../types';

  let unlisten: (() => void) | null = null;

  onMount(async () => {
    unlisten = await listen<DisplayEvent>('display-event', (event) => {
      keystream.push(event.payload);
    });
  });

  onDestroy(() => {
    if (unlisten) unlisten();
    keystream.destroy();
  });
</script>

<div class="overlay-container">
  <TextStream />
</div>

<style>
  .overlay-container {
    position: fixed;
    bottom: 32px;
    right: 32px;
  }
</style>
```

- [ ] **Step 6: Test text stream display**

```bash
npm run tauri dev
```

Expected: Keystrokes appear as labels stacking in the bottom-right corner. They fade out after ~2s. Combos like `Ctrl+S` appear as a single "Ctrl + S" label. Rapid repeated keys show counts ("A x3"). Labels animate in with a subtle slide-up.

- [ ] **Step 7: Commit**

```bash
git add .
git commit -m "feat: add text stream display with fade animations and repeat compression"
```

---

## Task 7: Config system

**Files:**
- Create: `src-tauri/src/config/mod.rs`, `src-tauri/src/config/schema.rs`, `src-tauri/src/config/store.rs`
- Modify: `src-tauri/src/lib.rs`

- [ ] **Step 1: Define the config schema**

Create `src-tauri/src/config/schema.rs`:
```rust
use serde::{Deserialize, Serialize};

pub const CURRENT_CONFIG_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub version: u32,
    pub display: DisplayConfig,
    pub input: InputConfig,
    pub appearance: AppearanceConfig,
    pub shortcuts: ShortcutConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    pub mode: DisplayMode,
    pub position_strategy: PositionStrategy,
    pub corner: Corner,
    pub margin_x: u32,
    pub margin_y: u32,
    pub max_visible: u32,
    pub fade_duration_ms: u32,
    pub stack_direction: StackDirection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DisplayMode {
    TextStream,
    VisualKeyboard,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PositionStrategy {
    FollowActiveWindow,
    FollowMouse,
    Pinned,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Corner {
    BottomRight,
    BottomLeft,
    TopRight,
    TopLeft,
    BottomCenter,
    TopCenter,
    LeftCenter,
    RightCenter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StackDirection {
    Up,
    Down,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputConfig {
    pub smart_mode: bool,
    pub smart_threshold_ms: u64,
    pub repeat_window_ms: u64,
    pub show_all_keystrokes: bool,
    pub modifier_overrides: ModifierOverrides,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifierOverrides {
    pub ctrl: ModifierMode,
    pub alt: ModifierMode,
    pub shift: ModifierMode,
    pub win: ModifierMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModifierMode {
    Smart,
    AlwaysShow,
    NeverShow,
    ComboOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppearanceConfig {
    pub active_theme: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortcutConfig {
    pub toggle_capture: Option<String>,
    pub switch_mode: Option<String>,
    pub toggle_overlay: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            version: CURRENT_CONFIG_VERSION,
            display: DisplayConfig {
                mode: DisplayMode::TextStream,
                position_strategy: PositionStrategy::FollowActiveWindow,
                corner: Corner::BottomRight,
                margin_x: 32,
                margin_y: 32,
                max_visible: 5,
                fade_duration_ms: 2000,
                stack_direction: StackDirection::Up,
            },
            input: InputConfig {
                smart_mode: true,
                smart_threshold_ms: 200,
                repeat_window_ms: 500,
                show_all_keystrokes: true,
                modifier_overrides: ModifierOverrides {
                    ctrl: ModifierMode::Smart,
                    alt: ModifierMode::Smart,
                    shift: ModifierMode::Smart,
                    win: ModifierMode::Smart,
                },
            },
            appearance: AppearanceConfig {
                active_theme: "dark-glass".to_string(),
            },
            shortcuts: ShortcutConfig {
                toggle_capture: None,
                switch_mode: None,
                toggle_overlay: None,
            },
        }
    }
}
```

- [ ] **Step 2: Write the config store**

Create `src-tauri/src/config/store.rs`:
```rust
use super::schema::{AppConfig, CURRENT_CONFIG_VERSION};
use std::fs;
use std::path::PathBuf;

fn config_dir() -> PathBuf {
    let base = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    base.join("keykey")
}

fn config_path() -> PathBuf {
    config_dir().join("config.json")
}

pub fn load_config() -> AppConfig {
    let path = config_path();

    if !path.exists() {
        let config = AppConfig::default();
        let _ = save_config(&config);
        return config;
    }

    match fs::read_to_string(&path) {
        Ok(contents) => match serde_json::from_str::<AppConfig>(&contents) {
            Ok(mut config) => {
                if config.version < CURRENT_CONFIG_VERSION {
                    // Backup old config
                    let backup = config_dir().join("config.backup.json");
                    let _ = fs::copy(&path, &backup);
                    // Migrate: merge with defaults
                    config.version = CURRENT_CONFIG_VERSION;
                    let _ = save_config(&config);
                }
                config
            }
            Err(e) => {
                eprintln!("config parse error, resetting: {}", e);
                // Backup corrupt config
                let corrupt = config_dir().join("config.corrupt.json");
                let _ = fs::rename(&path, &corrupt);
                let config = AppConfig::default();
                let _ = save_config(&config);
                config
            }
        },
        Err(e) => {
            eprintln!("config read error: {}", e);
            AppConfig::default()
        }
    }
}

pub fn save_config(config: &AppConfig) -> Result<(), String> {
    let dir = config_dir();
    fs::create_dir_all(&dir).map_err(|e| format!("failed to create config dir: {}", e))?;

    let json =
        serde_json::to_string_pretty(config).map_err(|e| format!("serialize error: {}", e))?;

    fs::write(config_path(), json).map_err(|e| format!("write error: {}", e))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_default_config_serializes() {
        let config = AppConfig::default();
        let json = serde_json::to_string_pretty(&config).unwrap();
        let parsed: AppConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.version, CURRENT_CONFIG_VERSION);
    }
}
```

- [ ] **Step 3: Create config module**

Create `src-tauri/src/config/mod.rs`:
```rust
pub mod schema;
pub mod store;
```

- [ ] **Step 4: Add Tauri commands for config access**

Create `src-tauri/src/commands.rs`:
```rust
use crate::config::{schema::AppConfig, store};
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, State};

pub struct ConfigState(pub Mutex<AppConfig>);

#[tauri::command]
pub fn get_config(state: State<ConfigState>) -> AppConfig {
    state.0.lock().unwrap().clone()
}

#[tauri::command]
pub fn save_config(config: AppConfig, state: State<ConfigState>, app: AppHandle) -> Result<(), String> {
    store::save_config(&config)?;
    *state.0.lock().unwrap() = config.clone();
    // Notify all windows that config changed (theme hot-reload, mode switching)
    let _ = app.emit("config-changed", &config);
    Ok(())
}
```

- [ ] **Step 5: Register config state and commands in lib.rs**

Add to `src-tauri/src/lib.rs`:
```rust
mod config;
mod commands;

use commands::ConfigState;
use config::store;

// In run(), update Builder:
tauri::Builder::default()
    .manage(ConfigState(std::sync::Mutex::new(store::load_config())))
    .invoke_handler(tauri::generate_handler![
        commands::get_config,
        commands::save_config,
    ])
    // ... rest of setup
```

- [ ] **Step 6: Run tests**

```bash
cd src-tauri && cargo test
```

Expected: all tests pass including config serialization.

- [ ] **Step 7: Commit**

```bash
git add .
git commit -m "feat: add config system with JSON persistence and migration"
```

---

## Task 8: Theme system

**Files:**
- Create: `src/lib/theme-engine.ts`, `src/lib/stores/theme.ts`, `src/styles/themes/*.css`

- [ ] **Step 1: Define theme types**

Add to `src/lib/types.ts`:
```typescript
export interface Theme {
  name: string;
  label: string;
  vars: Record<string, string>;
}
```

- [ ] **Step 2: Create built-in theme presets**

Create `src/lib/theme-engine.ts`:
```typescript
import type { Theme } from './types';

export const BUILT_IN_THEMES: Theme[] = [
  {
    name: 'dark-glass',
    label: 'Dark glass',
    vars: {
      '--kk-bg': 'rgba(15, 15, 20, 0.8)',
      '--kk-text': '#e8e8e8',
      '--kk-font': "'Segoe UI', system-ui, sans-serif",
      '--kk-font-size': '15px',
      '--kk-font-weight': '500',
      '--kk-combo-font-weight': '600',
      '--kk-padding-x': '14px',
      '--kk-padding-y': '7px',
      '--kk-radius': '8px',
      '--kk-border-width': '1px',
      '--kk-border-color': 'rgba(255, 255, 255, 0.1)',
      '--kk-shadow': '0 4px 16px rgba(0, 0, 0, 0.4)',
      '--kk-gap': '4px',
    },
  },
  {
    name: 'light-minimal',
    label: 'Light minimal',
    vars: {
      '--kk-bg': 'rgba(255, 255, 255, 0.9)',
      '--kk-text': '#1a1a1a',
      '--kk-font': "'Segoe UI', system-ui, sans-serif",
      '--kk-font-size': '13px',
      '--kk-font-weight': '400',
      '--kk-combo-font-weight': '600',
      '--kk-padding-x': '10px',
      '--kk-padding-y': '5px',
      '--kk-radius': '4px',
      '--kk-border-width': '0px',
      '--kk-border-color': 'transparent',
      '--kk-shadow': '0 1px 4px rgba(0, 0, 0, 0.1)',
      '--kk-gap': '3px',
    },
  },
  {
    name: 'terminal',
    label: 'Terminal',
    vars: {
      '--kk-bg': 'rgba(0, 0, 0, 0.9)',
      '--kk-text': '#33ff33',
      '--kk-font': "'Cascadia Code', 'Consolas', monospace",
      '--kk-font-size': '14px',
      '--kk-font-weight': '400',
      '--kk-combo-font-weight': '700',
      '--kk-padding-x': '12px',
      '--kk-padding-y': '6px',
      '--kk-radius': '0px',
      '--kk-border-width': '1px',
      '--kk-border-color': '#33ff33',
      '--kk-shadow': '0 0 8px rgba(51, 255, 51, 0.2)',
      '--kk-gap': '2px',
    },
  },
  {
    name: 'neon',
    label: 'Neon',
    vars: {
      '--kk-bg': 'rgba(10, 5, 20, 0.85)',
      '--kk-text': '#ff44ff',
      '--kk-font': "'Segoe UI', system-ui, sans-serif",
      '--kk-font-size': '16px',
      '--kk-font-weight': '600',
      '--kk-combo-font-weight': '700',
      '--kk-padding-x': '16px',
      '--kk-padding-y': '8px',
      '--kk-radius': '10px',
      '--kk-border-width': '1px',
      '--kk-border-color': 'rgba(255, 68, 255, 0.4)',
      '--kk-shadow': '0 0 20px rgba(255, 68, 255, 0.3), 0 0 40px rgba(255, 68, 255, 0.1)',
      '--kk-gap': '6px',
    },
  },
  {
    name: 'subtle',
    label: 'Subtle',
    vars: {
      '--kk-bg': 'rgba(0, 0, 0, 0.0)',
      '--kk-text': 'rgba(255, 255, 255, 0.7)',
      '--kk-font': "'Segoe UI', system-ui, sans-serif",
      '--kk-font-size': '14px',
      '--kk-font-weight': '400',
      '--kk-combo-font-weight': '500',
      '--kk-padding-x': '8px',
      '--kk-padding-y': '4px',
      '--kk-radius': '4px',
      '--kk-border-width': '0px',
      '--kk-border-color': 'transparent',
      '--kk-shadow': '0 1px 8px rgba(0, 0, 0, 0.5)',
      '--kk-gap': '3px',
    },
  },
];

export function applyTheme(theme: Theme) {
  const root = document.documentElement;
  for (const [key, value] of Object.entries(theme.vars)) {
    root.style.setProperty(key, value);
  }
}

export function getThemeByName(name: string): Theme | undefined {
  return BUILT_IN_THEMES.find((t) => t.name === name);
}
```

- [ ] **Step 3: Apply theme on overlay load**

Update `src/lib/overlay/Overlay.svelte` to apply the default theme on mount:
```svelte
<script lang="ts">
  import { listen } from '@tauri-apps/api/event';
  import { invoke } from '@tauri-apps/api/core';
  import { onMount, onDestroy } from 'svelte';
  import { keystream } from '../stores/keystream';
  import TextStream from './TextStream.svelte';
  import { applyTheme, getThemeByName } from '../theme-engine';
  import type { DisplayEvent } from '../types';

  let unlisten: (() => void) | null = null;

  onMount(async () => {
    // Load config and apply theme
    try {
      const config: any = await invoke('get_config');
      const theme = getThemeByName(config.appearance.active_theme);
      if (theme) applyTheme(theme);
    } catch (e) {
      console.error('failed to load config:', e);
    }

    unlisten = await listen<DisplayEvent>('display-event', (event) => {
      keystream.push(event.payload);
    });
  });

  onDestroy(() => {
    if (unlisten) unlisten();
    keystream.destroy();
  });
</script>

<div class="overlay-container">
  <TextStream />
</div>

<style>
  .overlay-container {
    position: fixed;
    bottom: 32px;
    right: 32px;
  }
</style>
```

- [ ] **Step 4: Test themes**

```bash
npm run tauri dev
```

Expected: Overlay labels render with the dark glass theme (dark semi-transparent background, white text, rounded corners, subtle border). Change `active_theme` in the config JSON to "terminal" and restart — should show green-on-black monospace labels.

- [ ] **Step 5: Commit**

```bash
git add .
git commit -m "feat: add theme system with 5 built-in presets"
```

---

## Task 9: Multi-monitor support

**Files:**
- Create: `src-tauri/src/monitor/mod.rs`, `src-tauri/src/monitor/tracker.rs`
- Modify: `src-tauri/src/lib.rs`, `src-tauri/src/commands.rs`

- [ ] **Step 1: Write the monitor tracker (Windows)**

Create `src-tauri/src/monitor/mod.rs`:
```rust
pub mod tracker;
```

Create `src-tauri/src/monitor/tracker.rs`:
```rust
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct MonitorInfo {
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub work_x: i32,
    pub work_y: i32,
    pub work_width: i32,
    pub work_height: i32,
    pub dpi: u32,
    pub scale_factor: f64,
    pub is_primary: bool,
}

#[cfg(target_os = "windows")]
pub mod platform {
    use super::MonitorInfo;
    use std::mem;
    use windows::Win32::Foundation::{BOOL, HWND, LPARAM, RECT, TRUE};
    use windows::Win32::Graphics::Gdi::{
        EnumDisplayMonitors, GetMonitorInfoW, MonitorFromWindow, HDC, HMONITOR,
        MONITORINFOEXW, MONITOR_DEFAULTTONEAREST,
    };
    use windows::Win32::UI::HiDpi::{GetDpiForMonitor, MDT_EFFECTIVE_DPI};
    use windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow;

    pub fn enumerate_monitors() -> Vec<MonitorInfo> {
        let mut monitors: Vec<MonitorInfo> = Vec::new();
        unsafe {
            let _ = EnumDisplayMonitors(
                HDC::default(),
                None,
                Some(enum_callback),
                LPARAM(&mut monitors as *mut Vec<MonitorInfo> as isize),
            );
        }
        monitors
    }

    unsafe extern "system" fn enum_callback(
        hmonitor: HMONITOR,
        _hdc: HDC,
        _lprect: *mut RECT,
        lparam: LPARAM,
    ) -> BOOL {
        let monitors = &mut *(lparam.0 as *mut Vec<MonitorInfo>);
        if let Some(info) = get_monitor_info(hmonitor) {
            monitors.push(info);
        }
        TRUE
    }

    unsafe fn get_monitor_info(hmonitor: HMONITOR) -> Option<MonitorInfo> {
        let mut info: MONITORINFOEXW = mem::zeroed();
        info.monitorInfo.cbSize = mem::size_of::<MONITORINFOEXW>() as u32;

        if !GetMonitorInfoW(hmonitor, &mut info as *mut _ as *mut _).as_bool() {
            return None;
        }

        let rc = info.monitorInfo.rcMonitor;
        let rc_work = info.monitorInfo.rcWork;
        let is_primary = (info.monitorInfo.dwFlags & 1) != 0;

        let name = String::from_utf16_lossy(
            &info.szDevice[..info
                .szDevice
                .iter()
                .position(|&c| c == 0)
                .unwrap_or(info.szDevice.len())],
        );

        let mut dpi_x: u32 = 96;
        let mut dpi_y: u32 = 96;
        let _ = GetDpiForMonitor(hmonitor, MDT_EFFECTIVE_DPI, &mut dpi_x, &mut dpi_y);

        Some(MonitorInfo {
            name,
            x: rc.left,
            y: rc.top,
            width: rc.right - rc.left,
            height: rc.bottom - rc.top,
            work_x: rc_work.left,
            work_y: rc_work.top,
            work_width: rc_work.right - rc_work.left,
            work_height: rc_work.bottom - rc_work.top,
            dpi: dpi_x,
            scale_factor: dpi_x as f64 / 96.0,
            is_primary,
        })
    }

    /// Get the monitor that the currently focused window is on
    pub fn get_active_monitor() -> Option<MonitorInfo> {
        unsafe {
            let hwnd = GetForegroundWindow();
            if hwnd == HWND::default() {
                return None;
            }
            let hmonitor = MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST);
            if hmonitor == HMONITOR::default() {
                return None;
            }
            get_monitor_info(hmonitor)
        }
    }

    /// Get the monitor that the mouse cursor is currently on
    pub fn get_cursor_monitor() -> Option<MonitorInfo> {
        use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;
        use windows::Win32::Graphics::Gdi::MonitorFromPoint;
        use windows::Win32::Foundation::POINT;
        unsafe {
            let mut point = POINT::default();
            if !GetCursorPos(&mut point).as_bool() {
                return None;
            }
            let hmonitor = MonitorFromPoint(point, MONITOR_DEFAULTTONEAREST);
            if hmonitor == HMONITOR::default() {
                return None;
            }
            get_monitor_info(hmonitor)
        }
    }
}

#[cfg(not(target_os = "windows"))]
pub mod platform {
    use super::MonitorInfo;

    pub fn enumerate_monitors() -> Vec<MonitorInfo> {
        vec![MonitorInfo {
            name: "default".to_string(),
            x: 0, y: 0,
            width: 1920, height: 1080,
            work_x: 0, work_y: 0,
            work_width: 1920, work_height: 1040,
            dpi: 96, scale_factor: 1.0,
            is_primary: true,
        }]
    }

    pub fn get_active_monitor() -> Option<MonitorInfo> {
        Some(enumerate_monitors().remove(0))
    }
}
```

- [ ] **Step 2: Add monitor commands**

Add to `src-tauri/src/commands.rs`:
```rust
use crate::monitor::tracker::{self, MonitorInfo};

#[tauri::command]
pub fn get_monitors() -> Vec<MonitorInfo> {
    tracker::platform::enumerate_monitors()
}

#[tauri::command]
pub fn get_active_monitor() -> Option<MonitorInfo> {
    tracker::platform::get_active_monitor()
}
```

Register in `lib.rs`:
```rust
.invoke_handler(tauri::generate_handler![
    commands::get_config,
    commands::save_config,
    commands::get_monitors,
    commands::get_active_monitor,
])
```

- [ ] **Step 3: Add overlay repositioning logic**

Add a Tauri command and background thread that periodically checks the active monitor and repositions the overlay. Add to `src-tauri/src/lib.rs` setup:

```rust
use crate::config::schema::PositionStrategy;

// In setup closure, after keyboard listener:
let app_handle = app.handle().clone();
std::thread::spawn(move || {
    let mut last_monitor_name = String::new();
    loop {
        // Read position strategy from config state
        let strategy = {
            let state = app_handle.state::<ConfigState>();
            let config = state.0.lock().unwrap();
            config.display.position_strategy.clone()
        };

        let target_monitor = match strategy {
            PositionStrategy::FollowActiveWindow => {
                monitor::tracker::platform::get_active_monitor()
            }
            PositionStrategy::FollowMouse => {
                monitor::tracker::platform::get_cursor_monitor()
            }
            PositionStrategy::Pinned => {
                // For pinned mode, don't reposition — skip this cycle
                std::thread::sleep(std::time::Duration::from_millis(500));
                continue;
            }
        };

        if let Some(monitor) = target_monitor {
            if monitor.name != last_monitor_name {
                last_monitor_name = monitor.name.clone();
                if let Some(overlay) = app_handle.get_webview_window("overlay") {
                    // Position overlay in bottom-right of the target monitor's work area
                    let x = monitor.work_x + monitor.work_width - 420;
                    let y = monitor.work_y + monitor.work_height - 320;
                    let _ = overlay.set_position(tauri::Position::Physical(
                        tauri::PhysicalPosition::new(x, y),
                    ));
                }
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(250));
    }
});
```

Note: 250ms polling for focus/cursor changes is a reasonable trade-off for v1. Later we can optimize to event-driven tracking via `SetWinEventHook` (focus) or `WH_MOUSE_LL` (cursor). For pinned mode, the overlay position is set once on startup from config and the thread just sleeps.

- [ ] **Step 4: Add monitor module to lib.rs**

```rust
mod monitor;
```

- [ ] **Step 5: Test multi-monitor**

```bash
npm run tauri dev
```

Expected: On a multi-monitor setup, the overlay follows the active window's monitor. Switch focus to a window on another screen — the overlay repositions to that monitor's bottom-right corner within ~250ms. On a single monitor, overlay stays in the bottom-right.

- [ ] **Step 6: Commit**

```bash
git add .
git commit -m "feat: add multi-monitor support with active window tracking"
```

---

## Task 10: Settings window UI

**Files:**
- Create: `src/lib/settings/Settings.svelte`, `src/lib/settings/DisplaySettings.svelte`, `src/lib/settings/InputSettings.svelte`, `src/lib/settings/AppearanceSettings.svelte`, `src/lib/settings/ShortcutSettings.svelte`, `src/lib/settings/AboutSettings.svelte`, `src/lib/stores/config.ts`
- Modify: `src/App.svelte`, `src/styles/settings.css`

This is a large task. The settings window ties together config, themes, and display options. Each settings panel reads from and writes to the config store.

- [ ] **Step 1: Create the config store**

Create `src/lib/stores/config.ts`:
```typescript
import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export interface AppConfig {
  version: number;
  display: {
    mode: string;
    position_strategy: string;
    corner: string;
    margin_x: number;
    margin_y: number;
    max_visible: number;
    fade_duration_ms: number;
    stack_direction: string;
  };
  input: {
    smart_mode: boolean;
    smart_threshold_ms: number;
    repeat_window_ms: number;
    show_all_keystrokes: boolean;
    modifier_overrides: {
      ctrl: string;
      alt: string;
      shift: string;
      win: string;
    };
  };
  appearance: {
    active_theme: string;
  };
  shortcuts: {
    toggle_capture: string | null;
    switch_mode: string | null;
    toggle_overlay: string | null;
  };
}

function createConfigStore() {
  const { subscribe, set, update } = writable<AppConfig | null>(null);

  return {
    subscribe,
    async load() {
      const config = await invoke<AppConfig>('get_config');
      set(config);
      return config;
    },
    async save(config: AppConfig) {
      await invoke('save_config', { config });
      set(config);
    },
    update,
  };
}

export const configStore = createConfigStore();
```

- [ ] **Step 2: Create the Settings root component**

Create `src/lib/settings/Settings.svelte`:
```svelte
<script lang="ts">
  import { onMount } from 'svelte';
  import { configStore } from '../stores/config';
  import DisplaySettings from './DisplaySettings.svelte';
  import InputSettings from './InputSettings.svelte';
  import AppearanceSettings from './AppearanceSettings.svelte';
  import ShortcutSettings from './ShortcutSettings.svelte';
  import AboutSettings from './AboutSettings.svelte';

  let activeTab = $state('display');
  let config = $state<any>(null);

  const tabs = [
    { id: 'display', label: 'Display' },
    { id: 'input', label: 'Input' },
    { id: 'appearance', label: 'Appearance' },
    { id: 'shortcuts', label: 'Shortcuts' },
    { id: 'about', label: 'About' },
  ];

  onMount(async () => {
    config = await configStore.load();
  });

  configStore.subscribe((value) => {
    config = value;
  });

  async function handleSave() {
    if (config) {
      await configStore.save(config);
    }
  }
</script>

<div class="settings-container">
  <nav class="settings-nav">
    {#each tabs as tab}
      <button
        class="nav-tab"
        class:active={activeTab === tab.id}
        onclick={() => (activeTab = tab.id)}
      >
        {tab.label}
      </button>
    {/each}
  </nav>

  <main class="settings-content">
    {#if config}
      {#if activeTab === 'display'}
        <DisplaySettings bind:config onSave={handleSave} />
      {:else if activeTab === 'input'}
        <InputSettings bind:config onSave={handleSave} />
      {:else if activeTab === 'appearance'}
        <AppearanceSettings bind:config onSave={handleSave} />
      {:else if activeTab === 'shortcuts'}
        <ShortcutSettings bind:config onSave={handleSave} />
      {:else if activeTab === 'about'}
        <AboutSettings />
      {/if}
    {:else}
      <p>Loading configuration...</p>
    {/if}
  </main>
</div>

<style>
  .settings-container {
    display: flex;
    height: 100vh;
    font-family: 'Segoe UI', system-ui, sans-serif;
    color: #e0e0e0;
    background: #1a1a2e;
  }

  .settings-nav {
    width: 180px;
    background: #16162a;
    padding: 16px 0;
    display: flex;
    flex-direction: column;
    gap: 2px;
    border-right: 1px solid #2a2a4a;
  }

  .nav-tab {
    background: none;
    border: none;
    color: #888;
    padding: 10px 20px;
    text-align: left;
    cursor: pointer;
    font-size: 14px;
    transition: all 0.15s;
  }

  .nav-tab:hover {
    color: #ccc;
    background: rgba(255, 255, 255, 0.04);
  }

  .nav-tab.active {
    color: #fff;
    background: rgba(255, 255, 255, 0.08);
    border-left: 2px solid #6366f1;
  }

  .settings-content {
    flex: 1;
    padding: 24px 32px;
    overflow-y: auto;
  }
</style>
```

- [ ] **Step 3: Create stub settings panels**

Create each settings panel as a minimal working component. These will be fleshed out as each feature lands. For brevity, here's the pattern — create each file:

`src/lib/settings/DisplaySettings.svelte`:
```svelte
<script lang="ts">
  interface Props {
    config: any;
    onSave: () => void;
  }

  let { config = $bindable(), onSave }: Props = $props();
</script>

<h2>Display</h2>

<label>
  Mode
  <select bind:value={config.display.mode} onchange={onSave}>
    <option value="text_stream">Text stream</option>
    <option value="visual_keyboard">Visual keyboard</option>
  </select>
</label>

<label>
  Position strategy
  <select bind:value={config.display.position_strategy} onchange={onSave}>
    <option value="follow_active_window">Follow active window</option>
    <option value="follow_mouse">Follow mouse</option>
    <option value="pinned">Pinned</option>
  </select>
</label>

<label>
  Corner
  <select bind:value={config.display.corner} onchange={onSave}>
    <option value="bottom_right">Bottom right</option>
    <option value="bottom_left">Bottom left</option>
    <option value="top_right">Top right</option>
    <option value="top_left">Top left</option>
    <option value="bottom_center">Bottom center</option>
    <option value="top_center">Top center</option>
  </select>
</label>

<label>
  Max visible labels
  <input type="number" bind:value={config.display.max_visible} onchange={onSave} min="1" max="20" />
</label>

<label>
  Fade duration (ms)
  <input type="number" bind:value={config.display.fade_duration_ms} onchange={onSave} min="500" max="10000" step="100" />
</label>

<style>
  h2 { margin: 0 0 20px; font-weight: 500; }
  label { display: block; margin-bottom: 16px; font-size: 14px; color: #aaa; }
  select, input { display: block; margin-top: 4px; padding: 6px 10px; background: #2a2a4a; border: 1px solid #3a3a5a; color: #e0e0e0; border-radius: 4px; font-size: 14px; }
</style>
```

`src/lib/settings/InputSettings.svelte`:
```svelte
<script lang="ts">
  interface Props {
    config: any;
    onSave: () => void;
  }

  let { config = $bindable(), onSave }: Props = $props();

  const modifierModes = [
    { value: 'smart', label: 'Smart' },
    { value: 'always_show', label: 'Always show' },
    { value: 'never_show', label: 'Never show' },
    { value: 'combo_only', label: 'Combo only' },
  ];
</script>

<h2>Input</h2>

<label class="checkbox">
  <input type="checkbox" bind:checked={config.input.smart_mode} onchange={onSave} />
  Smart mode
</label>

{#if config.input.smart_mode}
  <label>
    Smart threshold (ms)
    <input type="range" bind:value={config.input.smart_threshold_ms} onchange={onSave} min="50" max="1000" step="50" />
    <span>{config.input.smart_threshold_ms}ms</span>
  </label>
{/if}

<h3>Modifier overrides</h3>

{#each ['ctrl', 'alt', 'shift', 'win'] as mod}
  <label>
    {mod.charAt(0).toUpperCase() + mod.slice(1)}
    <select bind:value={config.input.modifier_overrides[mod]} onchange={onSave}>
      {#each modifierModes as mode}
        <option value={mode.value}>{mode.label}</option>
      {/each}
    </select>
  </label>
{/each}

<label class="checkbox">
  <input type="checkbox" bind:checked={config.input.show_all_keystrokes} onchange={onSave} />
  Show all keystrokes (uncheck for shortcuts only)
</label>

<style>
  h2 { margin: 0 0 20px; font-weight: 500; }
  h3 { font-size: 14px; font-weight: 500; margin: 20px 0 12px; color: #ccc; }
  label { display: block; margin-bottom: 16px; font-size: 14px; color: #aaa; }
  label.checkbox { display: flex; align-items: center; gap: 8px; }
  select, input[type="range"] { display: block; margin-top: 4px; }
  select { padding: 6px 10px; background: #2a2a4a; border: 1px solid #3a3a5a; color: #e0e0e0; border-radius: 4px; font-size: 14px; }
  span { font-size: 13px; color: #888; }
</style>
```

`src/lib/settings/AppearanceSettings.svelte`:
```svelte
<script lang="ts">
  import { BUILT_IN_THEMES } from '../theme-engine';

  interface Props {
    config: any;
    onSave: () => void;
  }

  let { config = $bindable(), onSave }: Props = $props();
</script>

<h2>Appearance</h2>

<label>
  Theme
  <select bind:value={config.appearance.active_theme} onchange={onSave}>
    {#each BUILT_IN_THEMES as theme}
      <option value={theme.name}>{theme.label}</option>
    {/each}
  </select>
</label>

<p class="hint">Custom theme editor coming soon.</p>

<style>
  h2 { margin: 0 0 20px; font-weight: 500; }
  label { display: block; margin-bottom: 16px; font-size: 14px; color: #aaa; }
  select { display: block; margin-top: 4px; padding: 6px 10px; background: #2a2a4a; border: 1px solid #3a3a5a; color: #e0e0e0; border-radius: 4px; font-size: 14px; }
  .hint { font-size: 13px; color: #666; }
</style>
```

`src/lib/settings/ShortcutSettings.svelte`:
```svelte
<script lang="ts">
  interface Props {
    config: any;
    onSave: () => void;
  }

  let { config = $bindable(), onSave }: Props = $props();
</script>

<h2>Shortcuts</h2>

<p class="hint">Global hotkeys are optional. All actions are accessible from the tray menu.</p>

<label>
  Toggle capture
  <input type="text" bind:value={config.shortcuts.toggle_capture} onchange={onSave} placeholder="Not set" />
</label>

<label>
  Switch display mode
  <input type="text" bind:value={config.shortcuts.switch_mode} onchange={onSave} placeholder="Not set" />
</label>

<label>
  Toggle overlay visibility
  <input type="text" bind:value={config.shortcuts.toggle_overlay} onchange={onSave} placeholder="Not set" />
</label>

<p class="hint">Hotkey recording will be added in a future update. For now, enter combos like "Ctrl+Shift+F9".</p>

<style>
  h2 { margin: 0 0 20px; font-weight: 500; }
  label { display: block; margin-bottom: 16px; font-size: 14px; color: #aaa; }
  input { display: block; margin-top: 4px; padding: 6px 10px; background: #2a2a4a; border: 1px solid #3a3a5a; color: #e0e0e0; border-radius: 4px; font-size: 14px; width: 200px; }
  .hint { font-size: 13px; color: #666; margin-bottom: 16px; }
</style>
```

`src/lib/settings/AboutSettings.svelte`:
```svelte
<h2>About</h2>

<p><strong>KeyKey</strong> v0.1.0</p>
<p>A keystroke visualizer for tutorials and screen sharing.</p>
<p>License: MIT</p>

<style>
  h2 { margin: 0 0 20px; font-weight: 500; }
  p { font-size: 14px; color: #aaa; margin: 8px 0; }
</style>
```

- [ ] **Step 4: Update App.svelte to use Settings**

Replace `src/App.svelte`:
```svelte
<script lang="ts">
  import Settings from './lib/settings/Settings.svelte';
</script>

<Settings />
```

- [ ] **Step 5: Test settings window**

```bash
npm run tauri dev
```

Expected: Right-click tray icon > "Settings" opens a window with a tabbed settings UI. Left sidebar shows Display, Input, Appearance, Shortcuts, About tabs. Changing theme in Appearance saves to config.json.

- [ ] **Step 6: Commit**

```bash
git add .
git commit -m "feat: add settings window with tabbed UI"
```

---

## Task 11: Visual keyboard mode

**Scope risk: this is the largest feature. If it threatens progress, skip it and come back later. Text stream mode is fully functional without it.**

**Files:**
- Create: `src/lib/keyboard-layouts/ansi-104.ts`, `src/lib/overlay/VisualKeyboard.svelte`, `src/lib/overlay/KeyCap.svelte`
- Modify: `src/lib/overlay/Overlay.svelte`

- [ ] **Step 1: Define the ANSI keyboard layout data**

Create `src/lib/keyboard-layouts/ansi-104.ts`:
```typescript
export interface KeyDef {
  code: string;      // matches rdev Key debug name
  label: string;     // display text
  width: number;     // relative width (1 = standard key)
  row: number;       // 0 = function row, 1-5 = main rows
}

// Compact layout — just the main typing area, no numpad
export const ANSI_COMPACT: KeyDef[] = [
  // Row 0: Esc + F-keys
  { code: 'Escape', label: 'Esc', width: 1, row: 0 },
  { code: 'F1', label: 'F1', width: 1, row: 0 },
  { code: 'F2', label: 'F2', width: 1, row: 0 },
  { code: 'F3', label: 'F3', width: 1, row: 0 },
  { code: 'F4', label: 'F4', width: 1, row: 0 },
  { code: 'F5', label: 'F5', width: 1, row: 0 },
  { code: 'F6', label: 'F6', width: 1, row: 0 },
  { code: 'F7', label: 'F7', width: 1, row: 0 },
  { code: 'F8', label: 'F8', width: 1, row: 0 },
  { code: 'F9', label: 'F9', width: 1, row: 0 },
  { code: 'F10', label: 'F10', width: 1, row: 0 },
  { code: 'F11', label: 'F11', width: 1, row: 0 },
  { code: 'F12', label: 'F12', width: 1, row: 0 },

  // Row 1: number row
  { code: 'BackQuote', label: '`', width: 1, row: 1 },
  { code: 'Num1', label: '1', width: 1, row: 1 },
  { code: 'Num2', label: '2', width: 1, row: 1 },
  { code: 'Num3', label: '3', width: 1, row: 1 },
  { code: 'Num4', label: '4', width: 1, row: 1 },
  { code: 'Num5', label: '5', width: 1, row: 1 },
  { code: 'Num6', label: '6', width: 1, row: 1 },
  { code: 'Num7', label: '7', width: 1, row: 1 },
  { code: 'Num8', label: '8', width: 1, row: 1 },
  { code: 'Num9', label: '9', width: 1, row: 1 },
  { code: 'Num0', label: '0', width: 1, row: 1 },
  { code: 'Minus', label: '-', width: 1, row: 1 },
  { code: 'Equal', label: '=', width: 1, row: 1 },
  { code: 'Backspace', label: 'Bksp', width: 2, row: 1 },

  // Row 2: QWERTY
  { code: 'Tab', label: 'Tab', width: 1.5, row: 2 },
  { code: 'KeyQ', label: 'Q', width: 1, row: 2 },
  { code: 'KeyW', label: 'W', width: 1, row: 2 },
  { code: 'KeyE', label: 'E', width: 1, row: 2 },
  { code: 'KeyR', label: 'R', width: 1, row: 2 },
  { code: 'KeyT', label: 'T', width: 1, row: 2 },
  { code: 'KeyY', label: 'Y', width: 1, row: 2 },
  { code: 'KeyU', label: 'U', width: 1, row: 2 },
  { code: 'KeyI', label: 'I', width: 1, row: 2 },
  { code: 'KeyO', label: 'O', width: 1, row: 2 },
  { code: 'KeyP', label: 'P', width: 1, row: 2 },
  { code: 'LeftBracket', label: '[', width: 1, row: 2 },
  { code: 'RightBracket', label: ']', width: 1, row: 2 },
  { code: 'BackSlash', label: '\\', width: 1.5, row: 2 },

  // Row 3: home row
  { code: 'CapsLock', label: 'Caps', width: 1.75, row: 3 },
  { code: 'KeyA', label: 'A', width: 1, row: 3 },
  { code: 'KeyS', label: 'S', width: 1, row: 3 },
  { code: 'KeyD', label: 'D', width: 1, row: 3 },
  { code: 'KeyF', label: 'F', width: 1, row: 3 },
  { code: 'KeyG', label: 'G', width: 1, row: 3 },
  { code: 'KeyH', label: 'H', width: 1, row: 3 },
  { code: 'KeyJ', label: 'J', width: 1, row: 3 },
  { code: 'KeyK', label: 'K', width: 1, row: 3 },
  { code: 'KeyL', label: 'L', width: 1, row: 3 },
  { code: 'SemiColon', label: ';', width: 1, row: 3 },
  { code: 'Quote', label: "'", width: 1, row: 3 },
  { code: 'Return', label: 'Enter', width: 2.25, row: 3 },

  // Row 4: shift row
  { code: 'ShiftLeft', label: 'Shift', width: 2.25, row: 4 },
  { code: 'KeyZ', label: 'Z', width: 1, row: 4 },
  { code: 'KeyX', label: 'X', width: 1, row: 4 },
  { code: 'KeyC', label: 'C', width: 1, row: 4 },
  { code: 'KeyV', label: 'V', width: 1, row: 4 },
  { code: 'KeyB', label: 'B', width: 1, row: 4 },
  { code: 'KeyN', label: 'N', width: 1, row: 4 },
  { code: 'KeyM', label: 'M', width: 1, row: 4 },
  { code: 'Comma', label: ',', width: 1, row: 4 },
  { code: 'Dot', label: '.', width: 1, row: 4 },
  { code: 'Slash', label: '/', width: 1, row: 4 },
  { code: 'ShiftRight', label: 'Shift', width: 2.75, row: 4 },

  // Row 5: bottom row
  { code: 'ControlLeft', label: 'Ctrl', width: 1.25, row: 5 },
  { code: 'MetaLeft', label: 'Win', width: 1.25, row: 5 },
  { code: 'Alt', label: 'Alt', width: 1.25, row: 5 },
  { code: 'Space', label: '', width: 6.25, row: 5 },
  { code: 'AltGr', label: 'Alt', width: 1.25, row: 5 },
  { code: 'MetaRight', label: 'Win', width: 1.25, row: 5 },
  { code: 'ControlRight', label: 'Ctrl', width: 1.25, row: 5 },
];
```

- [ ] **Step 2: Create the KeyCap component**

Create `src/lib/overlay/KeyCap.svelte`:
```svelte
<script lang="ts">
  import type { KeyDef } from '../keyboard-layouts/ansi-104';

  interface Props {
    keyDef: KeyDef;
    pressed: boolean;
  }

  let { keyDef, pressed }: Props = $props();

  const BASE_SIZE = 40; // px per 1u key
  const GAP = 2;
</script>

<div
  class="keycap"
  class:pressed
  style="width: {keyDef.width * BASE_SIZE + (keyDef.width - 1) * GAP}px"
>
  {keyDef.label}
</div>

<style>
  .keycap {
    height: 36px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--kk-kb-key-bg, rgba(40, 40, 60, 0.9));
    color: var(--kk-kb-key-text, #ccc);
    border-radius: var(--kk-kb-key-radius, 4px);
    font-family: var(--kk-font, system-ui, sans-serif);
    font-size: 11px;
    font-weight: 500;
    transition: background 0.08s, color 0.08s, transform 0.08s;
    flex-shrink: 0;
  }

  .keycap.pressed {
    background: var(--kk-kb-key-pressed-bg, rgba(99, 102, 241, 0.9));
    color: var(--kk-kb-key-pressed-text, #fff);
    transform: scale(0.95);
  }
</style>
```

- [ ] **Step 3: Create the VisualKeyboard component**

Create `src/lib/overlay/VisualKeyboard.svelte`:
```svelte
<script lang="ts">
  import { listen } from '@tauri-apps/api/event';
  import { onMount, onDestroy } from 'svelte';
  import { ANSI_COMPACT } from '../keyboard-layouts/ansi-104';
  import KeyCap from './KeyCap.svelte';

  interface RawKeyEvent {
    key: string;
    key_code: string;
    event_type: string;
    timestamp: number;
  }

  let pressedKeys = $state<Set<string>>(new Set());
  let unlisten: (() => void) | null = null;

  // Group keys by row
  const rows = $derived(
    Array.from(new Set(ANSI_COMPACT.map((k) => k.row)))
      .sort()
      .map((row) => ANSI_COMPACT.filter((k) => k.row === row))
  );

  onMount(async () => {
    unlisten = await listen<RawKeyEvent>('key-event', (event) => {
      const code = event.payload.key_code;
      if (event.payload.event_type === 'press') {
        pressedKeys = new Set([...pressedKeys, code]);
      } else {
        const next = new Set(pressedKeys);
        next.delete(code);
        pressedKeys = next;
      }
    });
  });

  onDestroy(() => {
    if (unlisten) unlisten();
  });
</script>

<div class="visual-keyboard">
  {#each rows as row}
    <div class="keyboard-row">
      {#each row as keyDef}
        <KeyCap {keyDef} pressed={pressedKeys.has(keyDef.code)} />
      {/each}
    </div>
  {/each}
</div>

<style>
  .visual-keyboard {
    background: var(--kk-kb-bg, rgba(10, 10, 20, 0.85));
    padding: 6px;
    border-radius: 8px;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .keyboard-row {
    display: flex;
    gap: 2px;
  }
</style>
```

- [ ] **Step 4: Update Overlay to support mode switching**

Update `src/lib/overlay/Overlay.svelte` to render either TextStream or VisualKeyboard based on config:
```svelte
<script lang="ts">
  import { listen } from '@tauri-apps/api/event';
  import { invoke } from '@tauri-apps/api/core';
  import { onMount, onDestroy } from 'svelte';
  import { keystream } from '../stores/keystream';
  import TextStream from './TextStream.svelte';
  import VisualKeyboard from './VisualKeyboard.svelte';
  import { applyTheme, getThemeByName } from '../theme-engine';
  import type { DisplayEvent } from '../types';

  let displayMode = $state('text_stream');
  let unlistenDisplay: (() => void) | null = null;
  let unlistenConfig: (() => void) | null = null;
  let unlistenSwitchMode: (() => void) | null = null;

  function loadConfig(config: any) {
    displayMode = config.display.mode;
    const theme = getThemeByName(config.appearance.active_theme);
    if (theme) applyTheme(theme);
  }

  onMount(async () => {
    try {
      const config: any = await invoke('get_config');
      loadConfig(config);
    } catch (e) {
      console.error('failed to load config:', e);
    }

    unlistenDisplay = await listen<DisplayEvent>('display-event', (event) => {
      keystream.push(event.payload);
    });

    // Hot-reload when config changes from settings window
    unlistenConfig = await listen<any>('config-changed', (event) => {
      loadConfig(event.payload);
    });

    // Toggle mode from tray menu
    unlistenSwitchMode = await listen('switch-display-mode', () => {
      displayMode = displayMode === 'text_stream' ? 'visual_keyboard' : 'text_stream';
    });
  });

  onDestroy(() => {
    if (unlistenDisplay) unlistenDisplay();
    if (unlistenConfig) unlistenConfig();
    if (unlistenSwitchMode) unlistenSwitchMode();
    keystream.destroy();
  });
</script>

<div class="overlay-container">
  {#if displayMode === 'visual_keyboard'}
    <VisualKeyboard />
  {:else}
    <TextStream />
  {/if}
</div>

<style>
  .overlay-container {
    position: fixed;
    bottom: 32px;
    right: 32px;
  }
</style>
```

- [ ] **Step 5: Re-emit raw key events for visual keyboard mode**

The visual keyboard needs raw key press/release events (not processed display events) to know which keys are currently held. The listener already emits these as `key-event` — we just need to make sure both `key-event` (raw) and `display-event` (processed) are emitted.

Check that `src-tauri/src/keyboard/listener.rs` still emits `key-event` for the raw events in addition to `display-event` from the processor. If not, add the raw emit back:

```rust
// In the forwarding thread, before processing:
let raw_event = KeyEvent {
    key: display_name.clone(),
    key_code: key_to_code(key),
    event_type: if is_press { "press" } else { "release" }.to_string(),
    timestamp,
};
let _ = app_handle.emit("key-event", &raw_event);
```

- [ ] **Step 6: Test visual keyboard mode**

Change `display.mode` to `"visual_keyboard"` in the config JSON and restart:

```bash
npm run tauri dev
```

Expected: A compact rendered keyboard appears in the bottom-right. Pressing keys highlights them on the keyboard. Holding modifiers keeps them highlighted. Releasing dims them.

- [ ] **Step 7: Commit**

```bash
git add .
git commit -m "feat: add visual keyboard display mode with ANSI layout"
```

---

## Task 12: Global hotkeys

**Files:**
- Modify: `src-tauri/src/lib.rs`

- [ ] **Step 1: Register global shortcuts from config**

Add hotkey registration to the setup in `src-tauri/src/lib.rs`. This reads the config and registers any configured hotkeys:

```rust
use tauri_plugin_global_shortcut::{
    Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState,
};

fn parse_shortcut(s: &str) -> Option<Shortcut> {
    let parts: Vec<&str> = s.split('+').map(|p| p.trim()).collect();
    let mut mods = Modifiers::empty();
    let mut code = None;

    for part in &parts {
        match part.to_lowercase().as_str() {
            "ctrl" | "control" => mods |= Modifiers::CONTROL,
            "shift" => mods |= Modifiers::SHIFT,
            "alt" => mods |= Modifiers::ALT,
            "win" | "super" | "meta" | "cmd" => mods |= Modifiers::SUPER,
            key => {
                code = match key {
                    // F-keys
                    "f1" => Some(Code::F1), "f2" => Some(Code::F2),
                    "f3" => Some(Code::F3), "f4" => Some(Code::F4),
                    "f5" => Some(Code::F5), "f6" => Some(Code::F6),
                    "f7" => Some(Code::F7), "f8" => Some(Code::F8),
                    "f9" => Some(Code::F9), "f10" => Some(Code::F10),
                    "f11" => Some(Code::F11), "f12" => Some(Code::F12),
                    // All letters
                    "a" => Some(Code::KeyA), "b" => Some(Code::KeyB),
                    "c" => Some(Code::KeyC), "d" => Some(Code::KeyD),
                    "e" => Some(Code::KeyE), "f" => Some(Code::KeyF),
                    "g" => Some(Code::KeyG), "h" => Some(Code::KeyH),
                    "i" => Some(Code::KeyI), "j" => Some(Code::KeyJ),
                    "k" => Some(Code::KeyK), "l" => Some(Code::KeyL),
                    "m" => Some(Code::KeyM), "n" => Some(Code::KeyN),
                    "o" => Some(Code::KeyO), "p" => Some(Code::KeyP),
                    "q" => Some(Code::KeyQ), "r" => Some(Code::KeyR),
                    "s" => Some(Code::KeyS), "t" => Some(Code::KeyT),
                    "u" => Some(Code::KeyU), "v" => Some(Code::KeyV),
                    "w" => Some(Code::KeyW), "x" => Some(Code::KeyX),
                    "y" => Some(Code::KeyY), "z" => Some(Code::KeyZ),
                    // Digits
                    "0" => Some(Code::Digit0), "1" => Some(Code::Digit1),
                    "2" => Some(Code::Digit2), "3" => Some(Code::Digit3),
                    "4" => Some(Code::Digit4), "5" => Some(Code::Digit5),
                    "6" => Some(Code::Digit6), "7" => Some(Code::Digit7),
                    "8" => Some(Code::Digit8), "9" => Some(Code::Digit9),
                    // Common non-letter keys
                    "space" => Some(Code::Space),
                    "enter" | "return" => Some(Code::Enter),
                    "escape" | "esc" => Some(Code::Escape),
                    "backspace" => Some(Code::Backspace),
                    "tab" => Some(Code::Tab),
                    "delete" | "del" => Some(Code::Delete),
                    _ => None,
                };
            }
        }
    }

    code.map(|c| Shortcut::new(if mods.is_empty() { None } else { Some(mods) }, c))
}

#[cfg(test)]
mod shortcut_tests {
    use super::*;

    #[test]
    fn test_parse_ctrl_shift_f9() {
        let s = parse_shortcut("Ctrl+Shift+F9");
        assert!(s.is_some());
    }

    #[test]
    fn test_parse_single_key_no_mods() {
        let s = parse_shortcut("F12");
        assert!(s.is_some());
    }

    #[test]
    fn test_parse_invalid() {
        let s = parse_shortcut("Ctrl+???");
        assert!(s.is_none());
    }

    #[test]
    fn test_parse_all_letters() {
        for c in 'a'..='z' {
            let input = format!("Ctrl+{}", c);
            assert!(parse_shortcut(&input).is_some(), "failed for {}", c);
        }
    }
}

// In setup, after loading config:
// let config = store::load_config();
// Store shortcut-to-action mapping for the handler
let config = store::load_config();
let mut shortcut_actions: std::collections::HashMap<String, String> = std::collections::HashMap::new();

if let Some(ref s) = config.shortcuts.toggle_capture {
    shortcut_actions.insert(s.clone(), "toggle_capture".to_string());
}
if let Some(ref s) = config.shortcuts.switch_mode {
    shortcut_actions.insert(s.clone(), "switch_mode".to_string());
}
if let Some(ref s) = config.shortcuts.toggle_overlay {
    shortcut_actions.insert(s.clone(), "toggle_overlay".to_string());
}

let actions = shortcut_actions.clone();

#[cfg(desktop)]
{
    app.handle().plugin(
        tauri_plugin_global_shortcut::Builder::new()
            .with_handler(move |app, shortcut, event| {
                if event.state() == ShortcutState::Pressed {
                    // Find which action this shortcut maps to
                    let shortcut_str = format!("{:?}", shortcut);
                    for (key_str, action) in &actions {
                        if let Some(parsed) = parse_shortcut(key_str) {
                            if &parsed == shortcut {
                                // Dispatch the action
                                match action.as_str() {
                                    "toggle_capture" => {
                                        if let Some(overlay) = app.get_webview_window("overlay") {
                                            let visible = overlay.is_visible().unwrap_or(false);
                                            if visible {
                                                let _ = overlay.hide();
                                            } else {
                                                let _ = overlay.show();
                                            }
                                            let _ = app.emit("capture-toggled", !visible);
                                        }
                                    }
                                    "switch_mode" => {
                                        let _ = app.emit("switch-display-mode", ());
                                    }
                                    "toggle_overlay" => {
                                        if let Some(overlay) = app.get_webview_window("overlay") {
                                            if overlay.is_visible().unwrap_or(false) {
                                                let _ = overlay.hide();
                                            } else {
                                                let _ = overlay.show();
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                                break;
                            }
                        }
                    }
                }
            })
            .build(),
    )?;

    // Register all configured shortcuts
    for shortcut_str in shortcut_actions.keys() {
        if let Some(shortcut) = parse_shortcut(shortcut_str) {
            if let Err(e) = app.global_shortcut().register(shortcut) {
                eprintln!("failed to register shortcut '{}': {}", shortcut_str, e);
            }
        }
    }
}
```

Also add `"global-shortcut:default"` to `src-tauri/capabilities/default.json` permissions array.

- [ ] **Step 2: Test with a configured shortcut**

Edit config.json to set `"toggle_overlay": "Ctrl+Shift+F9"`. Restart the app. Press Ctrl+Shift+F9 — overlay should toggle visibility.

- [ ] **Step 3: Commit**

```bash
git add .
git commit -m "feat: add configurable global hotkeys"
```

---

## Task 13: First launch experience

**Files:**
- Modify: `src-tauri/src/config/store.rs`, `src-tauri/src/lib.rs`

- [ ] **Step 1: Add first_launch flag to config**

Add to `AppConfig`:
```rust
pub first_launch: bool,
```

Default to `true`. After first launch setup, set to `false`.

- [ ] **Step 2: Show settings window on first launch**

In `lib.rs` setup, after loading config:
```rust
let config = store::load_config();
if config.first_launch {
    // Lazy-create the settings window on first launch
    use tauri::WebviewUrl;
    let _ = tauri::WebviewWindowBuilder::new(
        app,
        "settings",
        WebviewUrl::App("index.html".into()),
    )
    .title("KeyKey settings")
    .inner_size(700.0, 500.0)
    .build();

    // Mark as no longer first launch
    let mut config = config;
    config.first_launch = false;
    let _ = store::save_config(&config);
}
```

- [ ] **Step 3: Test first launch**

Delete config.json from `%APPDATA%/keykey/`. Run the app:

```bash
npm run tauri dev
```

Expected: Settings window opens automatically. Close it and restart — settings window should NOT open automatically this time.

- [ ] **Step 4: Commit**

```bash
git add .
git commit -m "feat: add first launch detection with auto-open settings"
```

---

## Summary

| Task | What it delivers |
|------|-----------------|
| 1 | Working Tauri v2 project with all dependencies |
| 2 | Transparent, click-through overlay window |
| 3 | System tray with context menu |
| 4 | Global keyboard capture via rdev |
| 5 | Smart mode processing with repeat detection |
| 6 | Text stream display with fade animations |
| 7 | JSON config with migration and persistence |
| 8 | Theme system with 5 presets |
| 9 | Multi-monitor support |
| 10 | Settings window with tabbed UI |
| 11 | Visual keyboard mode (scope risk — skippable) |
| 12 | Configurable global hotkeys |
| 13 | First launch experience |

Each task produces a working, testable increment. Tasks 1-6 give you a functional keystroke visualizer. Tasks 7-10 add the configuration and polish. Tasks 11-13 add the remaining v1 features.

## Explicitly deferred from this plan

These features are in the spec's v1 scope but deferred to a follow-up plan to keep this plan focused:

- **Key allowlist/blocklist** — config fields and filtering logic for specific key suppression
- **Drag-to-reposition** — requires toggling click-through state, non-trivial UX design
- **Monitor hot-plugging** — handling `WM_DISPLAYCHANGE` events for monitor connect/disconnect
- **Custom theme editor UI** — settings panel for creating/editing themes via color pickers and sliders
- **Theme import/export** — loading/saving theme JSON files
