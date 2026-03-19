# Contributing to KeyKey

## Getting started

You'll need:
- [Node.js](https://nodejs.org/) 18+
- [Rust](https://rustup.rs/) (stable)
- Windows 10/11 (macOS support is planned but not yet implemented)

```bash
git clone https://github.com/jamditis/keykey.git
cd keykey
npm install
npm run tauri dev
```

This starts both the SvelteKit dev server and the Rust backend.

## Project structure

- `src/` — SvelteKit frontend (Svelte 5, TypeScript)
- `src-tauri/` — Rust backend (Tauri v2)
- `src-tauri/src/keyboard/` — Global keyboard capture and event processing
- `src-tauri/src/monitor/` — Multi-monitor detection and DPI handling
- `src-tauri/src/config/` — JSON config persistence

## Running tests

```bash
cd src-tauri
cargo test
```

Frontend type-checking:

```bash
npm run check
```

## Pull requests

- One focused change per PR
- Run `cargo test` and `npm run check` before submitting
- Describe what changed and why in the PR description

## Code style

- Rust: follow `rustfmt` defaults
- TypeScript/Svelte: follow the existing patterns in the codebase
- Keep frontend types in sync with Rust serde output (see `CLAUDE.md` for details)
- No emojis in source code, logs, or commit messages
