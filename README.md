# KeyKey

Lightweight keystroke visualizer for screen recordings, tutorials, and live demos. Displays keyboard input as a floating overlay so viewers can see what keys are being pressed.

Built with [Tauri v2](https://v2.tauri.app/) (Rust backend, SvelteKit frontend). Uses the native WebView2 runtime instead of bundling Chromium, so the installed size stays under 15MB and memory usage under 50MB.

<!-- TODO: Add screenshot here once the app is built -->
<!-- ![KeyKey overlay screenshot](docs/screenshots/overlay.png) -->

## Features

- **Two display modes** -- text stream (stacked key labels) or visual keyboard (highlighted keys)
- **Smart modifier handling** -- filters incidental modifier taps while showing intentional combos. Per-modifier overrides: always show, never show, combo only, or smart (configurable hold threshold).
- **Multi-monitor support** -- follows the active window, follows the mouse cursor, or pins to a specific monitor and corner. Repositions instantly on focus change. DPI-aware.
- **Theme system** -- 5 built-in presets (dark glass, light minimal, terminal, neon, subtle). All visual properties are CSS variables: fonts, colors, opacity, borders, shadows, animations. Hot-reloadable.
- **System tray** -- launches to tray, starts capturing immediately. Tray menu for pause, mode switch, settings, quit. Optional global hotkeys (user-configured, no hardcoded defaults).
- **Repeat compression** -- repeated keys collapse into a counter (`A x3`) instead of flooding the overlay

## Install

No installer yet. Build from source:

```bash
git clone https://github.com/jamditis/keykey.git
cd keykey
npm install
npm run tauri build
```

The installer will be in `src-tauri/target/release/bundle/`.

### Requirements

- Windows 10/11 (macOS planned)
- [Node.js](https://nodejs.org/) 18+
- [Rust](https://rustup.rs/) (stable)

## Development

```bash
npm run tauri dev
```

This starts the SvelteKit dev server and Rust backend together. Frontend hot-reloads; Rust recompiles on save.

```bash
# Type-check frontend
npm run check

# Run Rust tests
cd src-tauri && cargo test
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for more details.

## How it works

1. `rdev` captures global keyboard events via OS-level hooks (Win32 `SetWindowsHookEx`)
2. A Rust event processor applies smart mode logic -- modifier hold timing, combo detection, repeat compression
3. Processed events are sent to the frontend via Tauri's event system
4. The SvelteKit overlay renders them in the active display mode using the current theme

Two event channels run in parallel: raw press/release events for the visual keyboard, and processed display events for the text stream.

## Config

Settings are stored at `%APPDATA%/keykey/config.json`. Human-readable JSON, manually editable. The settings window (opened from the system tray) provides a visual editor.

## Why this exists

Existing keystroke visualizers have pain points:
- [KeyViz](https://github.com/nickvdyck/keyviz) caches monitor geometry at startup, breaking multi-monitor setups when focus moves between screens
- Most tools offer limited theme options or no customization
- Electron-based tools bundle Chromium and use 200MB+ of memory for a small overlay

KeyKey fixes these by re-querying monitor geometry on every focus change, providing a full CSS variable theme system, and using Tauri's native WebView2 instead of Chromium.

## License

[MIT](LICENSE)
