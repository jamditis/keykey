# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

KeyKey is a keystroke visualization overlay for screen recordings, tutorials, and live demos. Built with Tauri v2 (Rust backend, SvelteKit/TypeScript frontend). Captures global keyboard input via rdev and displays it as a floating, click-through overlay.

## Commands

```bash
# Development (starts both Rust backend + SvelteKit dev server)
npm run tauri dev

# Type-check frontend
npm run check

# Build for distribution
npm run tauri build

# Run Rust tests (13 tests in processor + config)
cd src-tauri && cargo test

# Rust-only check (faster than full build)
cd src-tauri && cargo check
```

Note: Rust must be on PATH. If `cargo` is not found: `export PATH="$HOME/.cargo/bin:$PATH"`

## Architecture

Two Tauri windows:
- **Overlay** — transparent, frameless, always-on-top, click-through. Defined in `tauri.conf.json`. Renders either text stream or visual keyboard mode.
- **Settings** — created lazily on demand from tray icon. Uses the root SvelteKit route (`/`).

Data flow: `rdev hook (OS-level) → Rust event processor (smart mode logic) → Tauri events → Svelte overlay`

Two event channels:
- `key-event` — raw press/release for visual keyboard mode
- `display-event` — processed events (combos, repeats, filtering) for text stream mode

### Backend (`src-tauri/src/`)

| File | Purpose |
|------|---------|
| `lib.rs` | App setup, tray menu, global shortcuts, overlay positioning thread |
| `commands.rs` | Tauri IPC commands (`get_config`, `save_config`, `get_monitors`) |
| `keyboard/listener.rs` | rdev hook, key name mapping, dual event emission, capture-enabled flag |
| `keyboard/processor.rs` | Smart mode, modifier handling, combo detection, repeat compression. **Has tests.** |
| `config/schema.rs` | All config types with serde. Enums use `snake_case` serialization. |
| `config/store.rs` | JSON persistence at `%APPDATA%/keykey/config.json` with corruption recovery |
| `monitor/tracker.rs` | Win32 monitor enumeration, DPI-aware positioning |

The Rust lib is named `keykey_lib` (not `keykey`) to avoid a Windows name collision with the binary.

### Frontend (`src/lib/`)

| File | Purpose |
|------|---------|
| `overlay/Overlay.svelte` | Root overlay — loads config, applies theme, listens for events |
| `overlay/TextStream.svelte` | Stacked key labels with fade-out |
| `overlay/VisualKeyboard.svelte` | ANSI keyboard with key highlighting |
| `stores/keystream.ts` | Reactive keystroke stream with configurable max entries + fade duration |
| `stores/config.ts` | Config store wrapping Tauri IPC |
| `theme-engine.ts` | 5 built-in themes, CSS variable application |
| `settings/` | Tabbed settings UI (display, input, appearance, shortcuts, about) |

Uses SvelteKit with `adapter-static` (not plain Svelte). Routes at `src/routes/`. Svelte 5 runes (`$state`, `$derived`, `$effect`, `$props`).

## Key conventions

- Frontend TypeScript types must exactly mirror Rust serde output. The `AppConfig` interface in `stores/config.ts` is the single TS definition — keep it in sync with `config/schema.rs`.
- All Rust enums use `#[serde(rename_all = "snake_case")]`. Frontend `<select>` values must use these snake_case strings.
- Config uses nested `margins: { x, y }` (not flat `margin_x`/`margin_y`).
- Modifier modes are: `smart`, `always_show`, `never_show`, `combo_only`.
- Position strategies are: `follow_active_window`, `follow_mouse`, `pinned`.
- Display modes are: `text_stream`, `visual_keyboard`.
- No hardcoded global hotkeys — all shortcuts are user-configured and optional.
- The `capture-toggled` Tauri event controls a static `AtomicBool` in listener.rs. The rdev hook keeps running but events are dropped when paused.

## Testing

Rust tests live alongside the code (`processor.rs` has 12 tests, `store.rs` has 1). Run with `cargo test` from `src-tauri/`. No frontend test framework is set up yet.
