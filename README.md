<div align="center">

# 🔮 Canvas

**A blazing-fast, offline-first, multiplayer desktop widget platform.**

Built with Rust, Slint UI, SQLite, and Automerge CRDTs.

</div>

---

## Overview

**Canvas** brings powerful, native desktop widgets back to your screen. No Electron, no bloated web views—just ultra-lean, hardware-accelerated (or CPU-fallback) widgets that idle at under 20MB of RAM. 

Every widget in Canvas is built on a local-first architecture. Your data is stored locally in SQLite and synchronized automatically using Conflict-Free Replicated Data Types (CRDTs), ensuring that you can work offline and sync effortlessly when online with peers.

## 🌟 Features

- 🚀 **Native & Lightweight:** Written in Rust and Slint. Idles at <20MB RAM using the CPU software renderer.
- 🎨 **Premium 'Instrument Glass' UI:** A custom design system featuring translucent frosted glass, Aura Edge focus rings, and meticulously crafted typography.
- 📶 **Offline-First:** Local SQLite storage ensures instantaneous loads and true offline capabilities.
- 🔄 **Multiplayer CRDT Syncing:** Powered by `automerge`, text and state conflicts are resolved automatically across multiple clients via WebSockets (Host/Client peer-to-peer).
- 🎮 **GPU Acceleration:** Opt-in GPU rendering for high framerates and advanced visual effects.

## 🚧 Current Progress

**Milestone 4: Phase 1 - Multiplayer Sync Completed**
We have successfully implemented the core multi-crate architecture, local storage backend, CRDT text documents, the beautiful 'Instrument Glass' sticky note widget, and the peer-to-peer CRDT WebSocket synchronization.

Next up: UI integrations for multiplayer (Host/Join buttons) and Rich Text styling!

## 🏗️ Architecture

The repository is structured as a Cargo workspace:

- `canvas-app`: The main executable handling window initialization and the Slint event loop.
- `canvas-ui`: The Slint UI components, themes, and layouts.
- `canvas-storage`: SQLite persistence layer using `rusqlite`.
- `canvas-doc`: The CRDT data structures powered by `automerge`.
- `canvas-sync`: WebSocket networking for multi-peer synchronization using `tokio`.
- `canvas-auth`: Authentication and user management (Upcoming).

## 🚀 Getting Started

Ensure you have Rust installed, then clone the repository:

```bash
git clone https://github.com/Lohithnath2910/canvas.git
cd canvas
```

To run the app as a **Standalone/Host**:
```bash
cargo run --release -p canvas-app -- --host 127.0.0.1:9000
```

To run the app as a **Client** (joining a host):
```bash
cargo run --release -p canvas-app -- --join 127.0.0.1:9000
```

*(GPU hardware acceleration can be enabled by passing `--gpu`)*

## 📚 Documentation

Check out the root documentation files for deep dives into our architecture and plans:
- [`task.md`](task.md) - The Master Blueprint and Milestones.
- [`design.md`](design.md) - The "Instrument Glass" UI/UX Design System.
- [`progress.md`](progress.md) - The project progress and completion log.

---

*Built for speed, aesthetics, and resilience.*
