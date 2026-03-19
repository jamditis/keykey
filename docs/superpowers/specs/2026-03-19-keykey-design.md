# KeyKey — keystroke visualizer design spec

## Overview

KeyKey is a lightweight keystroke visualization tool for screen recordings, tutorials, and live demos. It displays keyboard input as a floating overlay so viewers can see what keys are being pressed. Built with Tauri v2 (Rust backend, Svelte/TypeScript frontend), it targets Windows first with macOS support planned.

**Goals:**
- Fix multi-monitor pain points that plague existing tools (especially KeyViz)
- Full theme customization from day one
- Two display modes: text stream and visual keyboard
- Smart modifier handling that filters noise without losing useful information
- Lightweight — small memory footprint, no bundled Chromium (Tauri uses native WebView2)

**Approach:** Hybrid build — fresh Tauri v2 project with clean architecture, porting proven platform-specific solutions from KeyViz's codebase (rdev patches, window transparency tricks) while building our own UI, theme system, and settings from scratch.

**License:** MIT

---

## Architecture

```
+-------------------------------------+
|         Overlay window(s)           |  Transparent, always-on-top, click-through
|    HTML/CSS/JS rendered via Tauri    |  Renders text stream OR visual keyboard
|                                     |
+-------------------------------------+
|         Settings window             |  Standard Tauri window
|    Theme editor, positioning,       |  Opens from tray icon
|    modifier config, display mode    |
+-------------------------------------+
|         Rust backend (Tauri)        |
|  +----------+  +-----------------+  |
|  | rdev hook |  | Window manager  |  |  Global keyboard listener
|  | (patched) |  | (multi-monitor) |  |  Monitor tracking + overlay placement
|  +----------+  +-----------------+  |
|  +----------+  +-----------------+  |
|  | Event    |  | Config store    |  |  Smart mode logic (modifier timing)
|  | processor|  | (JSON on disk)  |  |  Persisted settings
|  +----------+  +-----------------+  |
+-------------------------------------+
```

### Data flow

1. rdev captures raw key events globally via OS-level hooks
2. Rust event processor applies smart mode logic (modifier hold timing, combo detection)
3. Processed events are emitted to the frontend via Tauri's event system
4. Frontend renders them in the overlay window using the active display mode and theme

### Windows

Two Tauri windows:

- **Overlay window** — one single window that is transparent, frameless, always-on-top, click-through, no taskbar entry. When the active monitor changes, the window repositions itself (instant move, no animation — animation would cause visible artifacts during the transition). Re-queries monitor geometry on each move to handle DPI differences.
- **Settings window** — created lazily on first open (not pre-created at startup). Opened from system tray, destroyed when closed to save memory. State is in the config file, so nothing is lost.

---

## Display modes

### Text stream (default)

- Keystrokes appear as styled labels in a configurable screen corner
- New keys stack vertically (newest at bottom or top, configurable)
- Each label fades out after a configurable duration (default ~2s)
- Modifier combos display as a single label: `Ctrl + Shift + S`
- Repeated keys compress: identical key pressed within 500ms of the previous press counts as a repeat. Counter updates live in the overlay (e.g., `↓ x2` becomes `↓ x3` as you keep pressing). Series ends when 500ms passes without another press.
- Max visible items configurable (default ~5) to prevent screen clutter

### Visual keyboard

- Rendered keyboard layout where pressed keys highlight
- Supports standard ANSI/ISO layouts
- Keys light up on press, dim on release
- Modifier keys stay highlighted while held
- Compact layout option — show only the relevant section or a mini keyboard

### Switching

- Toggle via settings window
- Optional global hotkey for quick switching (user-configured, no default)
- Both modes running simultaneously is a future feature, not v1

---

## Smart mode and modifier handling

### Smart mode (default)

- Modifier combos always display (`Ctrl+C`, `Alt+Tab`, `Win+L`)
- Standalone modifier presses shown only if held for >200ms
- This filters incidental Shift taps during typing but catches intentional holds
- The 200ms threshold is configurable in settings

### Per-modifier configuration

Users can override smart mode for any individual modifier:

| Mode | Behavior |
|------|----------|
| Always show | Display every press, even quick taps |
| Smart | Use the hold-duration threshold (default) |
| Never show | Suppress entirely |
| Combo only | Only show when combined with another key |

Example config: "Always show Ctrl and Alt, never show standalone Shift, smart mode for Win key."

### Regular key filtering

- Option to show all keystrokes vs. only shortcuts/combos (privacy during live demos)
- Allowlist/blocklist for specific keys
- App-specific profiles (suppress in password managers) — stretch goal, not v1

---

## Multi-monitor support

### Positioning strategies (user picks in settings)

1. **Follow active window** (default) — overlay appears on the monitor with the focused window. When focus moves to another monitor, overlay transitions to that screen. Uses `GetForegroundWindow()` + `MonitorFromWindow()`.

2. **Follow mouse** — overlay stays on whichever monitor the cursor is on. Uses `WH_MOUSE_LL` hook (event-driven, no polling) to detect when the cursor crosses monitor boundaries. Only repositions on boundary crossings, not every mouse move.

3. **Pinned** — user picks a specific monitor and corner. Overlay stays fixed.

### Corner positioning

- 8 positions: four corners + four edge midpoints
- Configurable margin/offset from the edge
- Drag-to-reposition as an alternative to preset positions. Drag saves a custom offset that persists across sessions. In "follow active window" or "follow mouse" mode, drag sets the relative offset within whatever monitor is currently active.

### Monitor handling

- Enumerate monitors via Windows display API
- Handle hot-plugging (monitor connected/disconnected while running)
- Respect per-monitor DPI scaling
- Re-query monitor geometry on every focus change rather than caching at startup (this is the specific bug that causes KeyViz's multi-monitor issues)

---

## Theme system

### Architecture

CSS variables drive the overlay appearance. Themes stored as JSON. Settings window provides a visual editor. Themes are hot-reloadable (changes apply instantly).

### Configurable properties

- Font family, size, weight
- Text color, background color, border color
- Background opacity (fully transparent to solid)
- Border radius, border width
- Padding and spacing between key labels
- Animation style for enter/exit (fade, slide, scale, or none)
- Animation duration
- Shadow/glow effects
- Key label format: symbol vs. text vs. abbreviated for modifiers

### Built-in presets (~5)

| Name | Description |
|------|-------------|
| Dark glass | Dark semi-transparent background, white text. Uses `backdrop-filter: blur()` where supported; falls back to higher opacity when the compositor doesn't support it. |
| Light minimal | Light background, dark text, no border, small font |
| Terminal | Monospace font, green-on-black, no rounded corners |
| Neon | Dark background, colored glow on text, high contrast |
| Subtle | Nearly invisible background, floating text with slight shadow |

### Custom themes

- Create/edit via settings UI (color pickers, sliders, font selector, live preview)
- Saved as JSON files in `themes/` inside the app config directory
- Import/export as single JSON files for sharing

### Visual keyboard mode theme extensions

- Key cap color, pressed key color, text color on keys
- Key gap spacing, key border radius
- Keyboard background (transparent or solid)

---

## Settings and system tray

### System tray

- App launches to tray (no main window on startup, overlay starts immediately)
- Tray icon indicates state: active vs. paused
- Right-click menu: toggle capture, switch display mode, open settings, quit

### Settings window sections

| Section | Contents |
|---------|----------|
| Display | Mode selection, positioning strategy, monitor, corner, margins |
| Input | Smart mode toggle, threshold slider, per-modifier overrides, key filtering |
| Appearance | Theme selector, theme editor, animation settings |
| Shortcuts | Global hotkey configuration |
| About | Version, links, license |

### Global hotkeys

- No hardcoded defaults that auto-register (common combos conflict with other apps)
- On first launch, settings window prompts user to configure hotkeys (or skip)
- Conflict detection via `RegisterHotKey` failure — warns if a combo is taken by another app using the same API. Note: apps that capture shortcuts via low-level hooks (not RegisterHotKey) won't be detected.
- All functions are accessible from the tray menu, so the app works with zero hotkeys
- Configurable actions: toggle capture, switch display mode, show/hide overlay

### Config storage

- Single JSON config file in OS-standard app data directory (`%APPDATA%/keykey/` on Windows, `~/Library/Application Support/keykey/` on macOS)
- Themes stored as separate JSON files in a `themes/` subdirectory
- Config file is the source of truth — settings UI reads and writes it
- Human-readable and manually editable
- Config includes a `version` field (integer, starting at 1). On load, if the version is older than current, the app merges saved values into the new default config, preserving user settings while adding new fields. Old config is backed up as `config.backup.json` before migration.

---

## Tech stack

### Backend (Rust)

| Dependency | Purpose |
|------------|---------|
| Tauri v2 | App framework, window management, tray, IPC |
| rdev (forked when needed) | Global keyboard hook |
| serde + serde_json | Config serialization |
| tauri-plugin-global-shortcut | Global hotkey registration |
| windows crate | Win32 APIs for monitor enumeration, foreground window tracking, DPI |

### Frontend

| Dependency | Purpose |
|------------|---------|
| Svelte + TypeScript | UI framework — compiled, no virtual DOM, minimal runtime |
| Vite | Build tooling |
| CSS variables | Theme system backbone |

Svelte over React: smaller runtime, compiled reactivity, less JS overhead for a high-frequency overlay. KeyViz uses React; Svelte is a deliberate differentiation.

### Build and packaging

- Tauri bundler for `.msi` (Windows) and `.dmg` (macOS)
- GitHub Actions for CI/CD when open sourced
- NSIS installer option for Windows

---

## Error handling

| Failure | Response |
|---------|----------|
| rdev hook fails to install (antivirus, permissions) | Show a tray notification explaining the issue. Offer to retry. Log the error. App stays running but in "paused" state. |
| WebView2 not installed | Tauri v2 bundles a WebView2 bootstrapper. If it's missing and can't be installed, show a native error dialog (no webview needed) with download instructions. |
| Config file corrupted or missing fields | Load defaults for any missing/invalid fields. If the file can't be parsed at all, rename it to `config.corrupt.json` and create a fresh default config. Log a warning. |
| Monitor disconnected while overlay is on it | Detect via display change event, reposition overlay to the primary monitor. |
| Theme file invalid | Fall back to the first built-in preset. Log a warning. |

---

## First launch

1. App installs and starts — overlay begins capturing immediately with default settings (text stream mode, follow-active-window, dark glass theme)
2. Settings window opens automatically on first launch only
3. Settings window shows a brief welcome: "KeyKey is running. Configure your preferences below, or close this window to use the defaults."
4. Hotkey configuration section is highlighted but not mandatory — user can skip it
5. On subsequent launches, app starts silently to tray

---

## Performance targets

| Metric | Target |
|--------|--------|
| Idle CPU (capturing, no keys pressed) | <1% |
| Active CPU (continuous typing) | <3% |
| Memory | <50MB |
| Keypress-to-render latency | <16ms (one frame at 60Hz) |
| Startup time | <2s to tray with overlay active |
| Binary size | <15MB installed |

---

## Elevation and security

**v1 runs as a regular user (no admin).** This means:
- Keystrokes in elevated (admin) windows will not be captured
- This is an acceptable trade-off — most tutorial/demo scenarios don't involve admin windows
- No UAC prompt on startup, which is better UX for the common case

**Antivirus considerations (for future open source release):**
- Global keyboard hooks are commonly flagged by security software
- Code signing certificate will be needed before public distribution
- Windows SmartScreen will warn on unsigned installers
- These are distribution concerns, not v1 development concerns

---

## Known rdev limitations

- IME input (CJK languages) may not be captured correctly — known upstream issue
- Some media keys and laptop-specific Fn combos may not register
- Key repeat events vary by OS — Windows sends repeated WM_KEYDOWN without WM_KEYUP
- KeyViz's rdev fork (mulaRahul/rdev) patches several of these; we'll port patches as needed rather than forking preemptively

---

## Scope boundaries

### v1 (this spec)

- Text stream display mode
- Visual keyboard display mode — **scope risk: this is the largest single feature.** If it threatens the timeline, ship v1 with text stream only and add visual keyboard in v1.1. Text stream is the higher-priority mode for tutorials.
- Smart mode + per-modifier configuration
- Multi-monitor support (all three strategies)
- Full theme system with presets and custom themes
- System tray with settings window
- User-configured global hotkeys
- Windows as primary platform

### Future (not v1)

- Mouse click visualization
- Simultaneous dual display modes
- App-specific profiles
- macOS build
- Linux/Wayland support
- Theme marketplace / community sharing
- Recording/replay of keystroke sequences
