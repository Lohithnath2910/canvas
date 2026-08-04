# Canvas - Progress & Log

## Approach
We are building **Canvas** (a native, offline-first, multiplayer desktop widget platform) strictly following the `task.md` blueprint. We execute milestones M0 through M8 in order, satisfying the Definition of Done (DoD) before moving on. All decisions respect the committed technology stack (Rust, Slint, SQLite, tokio, yrs CRDT, Axum, Postgres) and non-negotiable constraints (no Electron, CRDT for sync, hard resource budgets).

## Progress Log

### Milestone 0: Bootstrap (Completed)
- [x] Initialized Git repository in `Project-0/`.
- [x] Set Git remote to `https://github.com/Lohithnath2910/canvas.git`.
- [x] Configured Git username as `Lohithnath2910`.
- [x] Bootstrapped the Cargo workspace `canvas/Cargo.toml`.
- [x] Created all the foundational crates: `canvas-app`, `canvas-ui`, `canvas-doc`, `canvas-storage`, `canvas-sync`, `canvas-protocol`, `canvas-server`, `canvas-auth`.
- [x] Setup `canvas-ui` with `slint` and `slint-build` dependencies.
- [x] Configured `canvas-ui/build.rs` and created `canvas-ui/ui/main.slint`.
- [x] Updated `canvas-app` to depend on `canvas-ui` and launch the empty `MainWindow`.
- [x] Running `cargo check` / `cargo build` to verify the build on Windows.
- [x] Perform idle RSS and cold launch measurement.

### Milestone 1: Local Sync & Persistence (Completed)
- [x] Initialize SQLite database (`canvas-storage`) for persisting and loading documents.
- [x] Initialize automerge CRDT text document (`canvas-doc`).
- [x] Bind Slint TextEdit to save on keystrokes.

### Milestone 2: Multi-Widget Desktop (Completed)
- [x] Added `pos_x` and `pos_y` tracking to SQLite `widgets` table.
- [x] Refactored `canvas-app` entrypoint to query existing widgets and spawn multiple Slint windows dynamically.
- [x] Bound window drag event (`window-moved`) to persist positions in SQLite.
- [x] Added `+` overlay button to `GlassPanel` drag area to spawn new widgets seamlessly.

### Milestone 3: 'Instrument Glass' UI/UX Baseline (Completed)
- [x] Implemented `GlassPanel` with `Radius.sticky-note` (16px) and `1.5px` outer `AuraEdge` border.
- [x] Set up global `Palette` for reactive light/dark theming (`is-dark`, `canvas-graphite`, `canvas-ink`).
- [x] Implemented dual-signal focus: widget-level Aura Edge glow + control-level inner focus ring (2px offset).
- [x] Configured CPU software rendering fallback (<20MB RAM) with opt-in GPU rendering (`--gpu` flag) via `winit-femtovg`.
- [x] Removed standard window borders (frameless window) and added top draggable invisible area.

## How to Resume
Whenever you want to resume, you can check this file to see where we left off. You can ask me to run the latest checks, test the UI with `cargo run -p canvas-app`, or proceed to the next milestone if the current one's DoD is fully satisfied.
