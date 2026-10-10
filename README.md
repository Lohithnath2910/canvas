# Canvas

A native, offline-first desktop widget platform written in Rust. Small always-on-top
widgets live on your desktop, persist to local SQLite, and are built on CRDTs so
concurrent edits merge instead of overwrite.

No Electron, no webview, no browser engine. The UI is [Slint](https://slint.dev) on the
CPU software renderer, so a widget idles in single-digit megabytes.

## Design

Black, white, one red, dot-matrix type. State is shown by light, not icons: every widget
wears a five-cell **glyph strip** and has no other status indicator (see
[docs/glyph-strip.md](docs/glyph-strip.md)). Everything is flat and opaque: no blur,
gradients, glow or bounce.

## Widgets

- Sticky note (plain text)
- Todo
- Calendar
- Kanban
- Spreadsheet

Whiteboard and Shared Document are planned. Multiplayer sync is not wired up yet.

## Run

Requires a recent stable Rust toolchain. Windows is the primary target.

```bash
cargo run -p canvas-app
```

Opt in to GPU rendering (needs the `gpu` feature; software rendering is the default):

```bash
cargo run -p canvas-app --features gpu -- --gpu
```

`Ctrl+K` opens the command palette, `Ctrl+N` creates a note.

## Develop

```bash
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo fmt --all
```

## Layout

| Crate | Role |
|---|---|
| `canvas-app` | Binary: windows, desktop layer, hotkeys, widget wiring |
| `canvas-ui` | Slint components, theme tokens, bundled fonts and icons |
| `canvas-doc` | CRDT documents (`yrs`) per widget type |
| `canvas-storage` | SQLite persistence (`rusqlite`) |
| `canvas-sync` | Peer sync (stub for now) |

[brand/](brand/) holds launch-material drafts. Fonts are Silkscreen and Inter, both SIL OFL (in `crates/canvas-ui/assets/fonts/`).

## Status

Early development. Single-player widgets are built; the backend and sync stages come next.
