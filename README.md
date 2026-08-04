<div align="center">

# Canvas

**A blazing-fast, offline-first, multiplayer desktop widget platform.**

Built with Rust, Slint UI, SQLite, and Automerge CRDTs.

</div>

---

## Overview

**Canvas** brings powerful, native desktop widgets back to your screen. No Electron, no bloated web views—just ultra-lean, hardware-accelerated (or CPU-fallback) widgets that idle at under 20MB of RAM. 

Every widget in Canvas is built on a local-first architecture. Your data is stored locally in SQLite and synchronized automatically using Conflict-Free Replicated Data Types (CRDTs), ensuring that you can work offline and sync effortlessly when online.

## Features

- 🚀 **Native & Lightweight:** Written in Rust and Slint. Idles at <20MB RAM using the CPU software renderer.
- 🎨 **Premium 'Instrument Glass' UI:** A custom design system featuring translucent frosted glass, Aura Edge focus rings, and meticulously crafted typography.
- 📶 **Offline-First:** Local SQLite storage ensures instantaneous loads and true offline capabilities.
- 🔄 **CRDT Syncing:** Powered by `automerge`, text and state conflicts are resolved automatically across multiple clients.
- 🎮 **GPU Acceleration:** Opt-in GPU rendering for high framerates and advanced visual effects.

## Current Milestone

**Milestone 3: Instrument Glass Baseline Completed**
We have successfully implemented the core multi-crate architecture, local storage backend, CRDT text documents, and the beautiful 'Instrument Glass' sticky note widget with adaptive dark mode and dual-focus signals.

## Architecture

The repository is structured as a Cargo workspace:

- `canvas-app`: The main executable handling window initialization and the Slint event loop.
- `canvas-ui`: The Slint UI components, themes, and layouts.
- `canvas-storage`: SQLite persistence layer using `rusqlite`.
- `canvas-doc`: The CRDT data structures powered by `automerge`.
- `canvas-sync` / `canvas-protocol` / `canvas-server`: The networking and synchronization layers (Upcoming).
- `canvas-auth`: Authentication and user management (Upcoming).

## Getting Started

Ensure you have Rust installed, then clone the repository:

```bash
git clone https://github.com/Lohithnath2910/canvas.git
cd canvas
```

To run the app with the ultra-low RAM CPU software renderer:
```bash
cargo run --release -p canvas-app
```

To run the app with GPU hardware acceleration:
```bash
cargo run --release -p canvas-app -- --gpu
```

## Documentation

Check out the root documentation files for deep dives into our architecture and plans:
- [`task.md`](task.md) - The Master Blueprint and Milestones.
- [`design.md`](design.md) - The "Instrument Glass" UI/UX Design System.
- [`progress.md`](progress.md) - The project progress and completion log.

---

*Built for speed, aesthetics, and resilience.*
