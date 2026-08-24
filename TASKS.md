# Canvas — Task Backlog

**Purpose:** a resumable, ordered list of small units of work. A new session reads
[progress.md](progress.md) first for the state of the project, then starts here at the
first unticked box.

**Last updated:** 2026-08-24
**Current task:** `S1-27` — tray icon and app lifetime
**Stage:** 1 — Sticky note + app shell
**Done:** 18 of 55

---

## How to use this file

1. Work tasks **in numeric order**. A task's `Needs:` line lists its hard prerequisites.
2. One task = one coherent, compiling, manually-verified unit. Do not chain two.
3. When a task is done: tick its box, update **Current task** above, append a dated entry
   to `progress.md`, and `git commit` with the task ID in the subject.
4. `cargo clippy --workspace --all-targets` must be clean before ticking anything.
5. **Only tick a box for behaviour you actually saw work.** Code that merely compiles is
   `🟡` in `progress.md` and the box stays unticked. This rule has been broken twice
   already — S1-06 was ticked because `resize-border-width` was set, not because resizing
   worked, and it did not work.
6. **Write no code comments.** User instruction. Reasoning goes here, in `progress.md`,
   and in commit messages.
7. Hit a spec ambiguity? Add it to **Open questions** at the bottom and ask — do not guess.

Size legend: `S` ≈ under 30 min · `M` ≈ 30–90 min · `L` ≈ half a session or more.

---

## ⚠ VERIFICATION QUEUE — do this first

Everything below is **written, compiling and clippy-clean, but has never been watched
working.** It was all built in one session without a click-through. Run `cargo run -p
canvas-app` and go through this list; tick the task where it passes, open a new numbered
task where it does not.

Doing this before S1-27 matters because later tasks build directly on top of it.

**Note window**
- [ ] Drag the header — window follows 1:1, no drift, no inertia → **S1-07**
- [ ] Hover anywhere in the header — four controls fade in over 100ms, nothing shifts → **S1-08**
- [ ] Drag each of the 4 edges and 4 corners — resizes like a normal window, clamps at
      240×180, corners do both axes → **S1-06**
- [ ] Tab through: body → colour → pin → delete → close, each showing a focus ring;
      Shift+Tab reverses; Esc from the body drops focus → **S1-11**, **S1-12**
- [ ] Click the colour icon repeatedly — plate steps through 5 grey tones → part of **S1-05**
- [ ] Click the trash icon — in-widget confirm appears, `CANCEL` restores, `DELETE NOTE`
      removes it. No modal, no desktop dimming → **S1-13**, **S1-14**
- [ ] Double-click the header — title becomes editable, pre-selected, Enter commits
- [ ] Click `x` — note closes but stays in the home list → **S1-21**
- [ ] Pin a note, open a browser over it — note stays behind. Click it, type, click away;
      it settles back to the desktop layer → **S1-22**
- [ ] Ctrl+G in the body — cycles all 7 strip states, title names each one → **S1-04**
- [ ] Move and resize a note, restart the app — it reopens exactly where it was → **S1-18**

**Home window**
- [ ] `NEW NOTE` — creates a note, opens its window, adds a row → **S1-25**
- [ ] Click a row — toggles that note open/closed; the row's dots reflect it → **S1-24**
- [ ] Row delete icon — confirm overlay, then gone from list and from disk → **S1-26**
- [ ] Empty state shows on a fresh database (delete `%APPDATA%/Canvas`) → **S1-28**
- [ ] Open three notes at once — all independently draggable and editable → confirms **S1-20**
- [ ] Home `x` closes home and leaves notes running

**Check state on disk at any point:** `cargo run -p canvas-app -- --dump`

---


## Stage 0 — Workspace bootstrap

Stage DoD (CLAUDE.md §14): `cargo run -p canvas-app` opens a window · idle RSS measured
and under budget · cold launch measured.

- [x] **S0-1 · Verify the pinned stack actually resolves** — `S`
  Confirm `slint`/`slint-build` 1.17.x, `yrs` 0.19.x, `rusqlite` (bundled), `tokio`,
  `uuid` 1.x, `mimalloc` all resolve on this toolchain (rustc 1.94.0 is installed).
  Use a throwaway probe crate in the scratchpad — do not pollute the real workspace.
  *DoD:* exact resolved versions written into `progress.md`. If anything differs from the
  pinned version, **stop and report** before writing code against the §11/§13 API details.
  *Refs:* §0.2, §3

- [x] **S0-2 · Cargo workspace + five crate skeletons** — `M`
  Needs: S0-1. Root `Cargo.toml` with `[workspace]` + `[workspace.dependencies]` pinning
  every version from §3 in one place. Crates: `canvas-app` (bin), `canvas-ui`,
  `canvas-doc`, `canvas-storage`, `canvas-sync` (stub).
  *DoD:* `cargo check --workspace` clean; `cargo tree` shows no browser/webview transitive
  dependency (Hard Constraint 1).
  *Refs:* §14 Stage 0, §2.1, §3

- [x] **S0-3 · Fonts into the UI crate + blank window** — `M`
  Needs: S0-2. Copy `handoff/fonts/*` (both `.ttf` files **and** both `-OFL.txt` licences)
  into `crates/canvas-ui/assets/fonts/`. `build.rs` calling
  `slint_build::compile("ui/main.slint")`. Minimal `main.slint` window.
  *DoD:* `cargo run -p canvas-app` opens a visible window. **Human-eye check**, not a
  screenshot pipeline (§13.7).
  *Refs:* §14 Stage 0, §15.2

- [x] **S0-4 · Software renderer default + `--gpu` opt-in + mimalloc** — `S`
  Needs: S0-3. Force `winit-software` as the default backend; `--gpu` switches to
  `winit-femtovg`. `mimalloc` as `#[global_allocator]`. **Never default to GPU** — the
  memory budget depends on software rendering.
  *DoD:* both `cargo run -p canvas-app` and `-- --gpu` launch; log the chosen backend at
  startup to confirm the default path is software.
  *Refs:* §3 (UI row), §2.5

- [x] **S0-5 · Measure the budget baseline** — `S`
  Needs: S0-4. Measure idle RSS and cold-launch time on Windows (`Get-Process`, Task
  Manager). Budgets: app idle 20–40 MB, cold launch < 300 ms.
  *DoD:* both numbers recorded in `progress.md` as the baseline every later stage is
  compared against. Over budget at a blank window ⇒ **stop and report**.
  *Refs:* §2.5, §14 Stage 0 DoD

- [x] **S0-6 · Lint/format baseline** — `S`
  Needs: S0-2. `cargo fmt --all`, then `cargo clippy --all-targets -- -D warnings`.
  *DoD:* both exit 0 on a clean tree.
  *Refs:* §4.2

---

## Stage 1 — Sticky note, complete

Stage DoD (§14): all 11 states of §7.6 implemented and matching the table · rich text
formatting survives a restart · a note is fully creatable/editable/closable with **no
mouse** · undo correct after a remote edit arrives mid-session · reduced motion respected ·
widget RSS still 5–15 MB.

### 1a — Theme foundation

The spec's own Stage 1 list starts at the header, but the header needs tokens to exist
first. These three come before it.

- [x] **S1-01 · `Palette` global — every colour token** — `M`
  Needs: S0-3. `theme/palette.slint`. All §5.1 tokens, both modes, plus `is-dark`.
  Apply the three documented light-mode corrections: `dot-unlit` at ~22% (not 12%), `dim`
  at ~57–58% black (55% fails WCAG AA at 4.27:1), light shadows `0 1px 3px rgba(0,0,0,0.10)`
  idle / `0 6px 20px rgba(0,0,0,0.20)` drag.
  *DoD:* every token in §5.1 present, named exactly as the table names it. From here on,
  no colour literal appears anywhere else in the codebase.
  *Refs:* §5.1

- [x] **S1-02 · `Typography` global + font embedding** — `M`
  Needs: S1-01. Embed via **compile-time `import "…/Silkscreen-Regular.ttf";`** in the
  `.slint` file. `slint::register_font_from_memory` **does not exist** in 1.17 — a web
  search will suggest it anyway; do not reach for it. `system-sans` is `in-out`, set from
  Rust per platform (Windows → `Segoe UI Variable`).
  *DoD:* a 22px Silkscreen string and a 14px body string both render correctly, confirmed
  by eye. Interface face is used at **only** 11px and 22px, nowhere else, ever.
  *Refs:* §13.1, §5.2

- [x] **S1-03 · `Panel` component** — `S`
  Needs: S1-01. Opaque `plate` @ 0.96, radius 20, 1px `edge` border, single-layer shadow
  that deepens on `dragging` over 100ms. No blur, no gradient, no glass (rule 5).
  *DoD:* panel renders; toggling a test `dragging` property visibly deepens the shadow.
  *Refs:* §6.1, §5.5

### 1b — The signature elements

- [ ] **S1-04 · `GlyphStrip`, all 7 states** — `M`
  Needs: S1-01. Five cells, 3×3 each, 2px gap, 23×3 total. Enum: idle / focused / syncing /
  offline / peers / peers-idle / error. Stepped 120ms, **never interpolated** (rule 4 — the
  first opacity tween on a dot cell destroys the product's identity).
  **Gotcha:** `GlyphStrip` is itself a `HorizontalLayout`; placed directly inside another
  layout it gets stretched and an explicit `height`/`y` is silently ignored. Wrap it in a
  plain `Rectangle { width: 23px; height: 32px; }`.
  *DoD:* a temporary dev control cycles all 7 states and each matches the §6.2 table by eye.
  `syncing` renders as `idle` until real sync exists — never a decorative loop that lies
  about the network.
  *Refs:* §6.2, §13.4, rule 4

- [x] **S1-05 · The three header icons as real vector `Path`s** — `M`
  Needs: S1-01. `palette`, `pin`, `trash-2` — one component per icon, geometry copied
  exactly from the §5.6 table. 1.5px stroke, square caps and joins, 16-unit grid, rendered
  at 16px (not 20px — a 20px icon does not fit a 32px header beside a 22px title).
  `palette` needs two `Path` children: one stroke-only square, one fill-only triangle.
  Rest `dim` → hover `text`. **No placeholder text glyphs.**
  *DoD:* all three render as the correct shapes and change colour on their own hover.
  *Refs:* §5.6, §14 Stage 1 item 3

### 1c — Window shell, header, drag

- [ ] **S1-06 · Frameless window + native resize** — `M`
  Needs: S1-03. `no-frame: true`, `resize-border-width: 6px`, `preferred-width/height`
  320×280, `min-width/height` 240×180, **no max** (omit the properties; don't set a huge
  sentinel). `width` and `min-width` cannot coexist on one element — the compiler rejects
  it outright. Try `resize-border-width` **before** writing any manual pointer-tracking
  resize logic.
  *DoD:* the window resizes from every edge and corner, clamps at 240×180, never locks
  aspect ratio.
  *Refs:* §13.5, §7.3

- [ ] **S1-07 · Header: disjoint TouchArea regions, glyph strip, title, drag** — `L`
  Needs: S1-04, S1-05, S1-06. Header exactly 32px. Glyph strip at `x=16`, title at `x=51`
  (16+23+12), interface face 22, elided, **never wraps**. Icon cluster right-aligned with
  its right edge at `width−16`, gap 8.
  **Two `TouchArea`s must never occupy the same region** — overlapping ones are a documented
  open Slint limitation, not your mistake. Partition by absolute position: icon-cluster
  region first, then a drag region ending exactly where the icons begin. The glyph strip and
  title render on top of the drag region and carry no `TouchArea` of their own, so there is
  no event ambiguity — only visual layering.
  *DoD:* the title is **visible** (it went missing in the prior attempt — check it
  explicitly), dragging the header moves the window 1:1 with no drift or inertia, and
  clicking an icon does not drag the window.
  *Refs:* §13.3, §7.1, §5.3, §5.5 (drag row)

- [ ] **S1-08 · Hover-revealed control cluster** — `S`
  Needs: S1-07. Opacity 0→1 over 100ms on `cubic-bezier(0.2,0,0,1)`. **Position never
  moves.** Track hover as the OR of each disjoint region's own `has-hover` plus text focus
  — not a background overlay layer (that reintroduces the overlap bug).
  *DoD:* controls fade in on hover and out on leave; nothing shifts position; icons still
  hover-lighten individually.
  *Refs:* §7.4, §13.3, §7.6 state 4

### 1d — Body and text

- [x] **S1-09 · Note body: bare `TextInput` in a `Flickable`** — `M`
  Needs: S1-02, S1-07. **Never `std-widgets`' `TextEdit`.** Slint's compiler defaults to
  the `fluent` style whenever none is configured, and Fluent's `TextEdit` draws a 2px blue
  accent bar under the field on focus that is not exposed as an overridable property — that
  stray blue bar was a prior-attempt bug. Use the bare `TextInput` primitive:
  `wrap: word-wrap`, `single-line: false`, `text-cursor-width: 2px`, selection colours from
  `Palette`. Wrap it in a `Flickable` yourself (there is no built-in `ScrollView`). Call
  `text-input.focus()` in the window's `init`.
  *DoD:* typing works, wraps at the body edge, scrolls past the bottom with **no visible
  scrollbar track**, and **there is no blue anywhere on screen** — check it focused.
  *Refs:* §13.2, §7.1, §7.3

- [x] **S1-10 · Empty state (state 1)** — `S`
  Needs: S1-09. `NOTHING HERE YET` (interface face 22) + "Start typing." (body face,
  `dim`), gap 8, caret after. Strip `focused`. Controls hidden.
  *DoD:* a freshly created note shows it, and it disappears on the first keystroke.
  *Refs:* §7.6 state 1

### 1e — Keyboard

- [ ] **S1-11 · Focus ring, everywhere** — `S`
  Needs: S1-09. 2px `text` ring at 2px offset, **outside** the border. Radius = the
  element's own radius + 2 (plate → 22, control → 10). **Never `signal`** — a red focus
  ring reads as a warning and is a bug.
  *DoD:* the ring appears on the plate and on each control when focused.
  *Refs:* §6.9, §7.6 state 11

- [ ] **S1-12 · Tab order + Esc** — `M`
  Needs: S1-11. Body → colour → pin → delete. `Tab` from delete leaves the widget;
  `Shift+Tab` reverses. `Esc` from the body drops focus and hides the cluster.
  *DoD:* walked with the keyboard only, in both directions, matching §7.5 exactly.
  *Refs:* §7.5

### 1f — Controls and the destructive flow

- [ ] **S1-13 · `Button` component, three variants** — `M`
  Needs: S1-01, S1-02. Radius 8, padding 8/16, computed height 34px, label interface face
  11 UPPERCASE +0.08em, never wraps, min gap 8 between two buttons. Variants default /
  primary / destructive with the exact fills from §6.3. Disabled = whole control at 0.40
  opacity.
  *DoD:* all three variants × rest / hover / active / disabled / focused checked by eye
  against the §6.3 table. `signal` appears **only** on the destructive variant.
  *Refs:* §6.3, rule 2

- [ ] **S1-14 · Delete confirm, in-widget (state 10)** — `M`
  Needs: S1-13. Body text steps to `dim`. 1px `edge` hairline (**not** `signal`),
  `DELETE — CANNOT BE UNDONE` label, then `DELETE NOTE` (destructive) + `CANCEL` (default),
  gap 8. **In-widget — no modal, no desktop dimming.** `Enter` on delete opens it, `Esc`
  cancels.
  *DoD:* the full flow works by keyboard and by mouse; cancelling restores the body exactly.
  *Refs:* §7.6 state 10, §7.5

### 1g — Data, persistence, and the no-save-button rule

- [x] **S1-15 · `StickyNoteDoc` in `canvas-doc`** — `M`
  Needs: S0-2. Implement §11.1 against the version S0-1 actually resolved.
  Note the spec's own caveat: `update_text` replaces the whole string on every keystroke —
  still a valid CRDT (no LWW), but it bloats update size and will defeat cursor preservation
  once rich text lands. Acceptable **now**; S1-31 replaces it with a real diff. Leave a
  `// TODO(S1-31)` at that spot.
  *DoD:* unit tests — two docs edited concurrently converge after exchanging updates, and
  `snapshot()` round-trips through `load()`. **No last-write-wins anywhere**, not even as a
  temporary shortcut (Hard Constraint 3).
  *Refs:* §11.1, §2.3

- [x] **S1-16 · SQLite schema in `canvas-storage`** — `M`
  Needs: S0-2. All seven tables from §12 verbatim, in `canvas_local.db`. Open / create /
  migrate with a `schema_version`. `documents` is widget-type-agnostic — **one table for
  every widget type**, never one per type. Default size by `widget_type` at creation
  (sticky note 320×280, todo 340×360) rather than one hardcoded size. Default title
  `"UNTITLED"`, not a type-specific placeholder.
  *DoD:* the db file is created on first run; all tables present (verify with a query);
  reopening an existing db does not clobber it.
  *Refs:* §12

- [x] **S1-17 · Wire doc ↔ UI ↔ storage (no save button)** — `L`
  Needs: S1-09, S1-15, S1-16. Keystroke → local doc → paint, **then** persist. Persist on a
  debounce/idle, never synchronously on the UI thread's critical path. If a code path can
  make typing wait on I/O, it is wrong.
  *DoD:* type text, hard-kill the app, relaunch → the text is there. **No save button
  exists** (Hard Constraint 4). Typing stays responsive with a large note open (Hard
  Constraint 2).
  *Refs:* §2.2, §2.4, §12

- [ ] **S1-18 · Window position + size + z-order persistence** — `M`
  Needs: S1-17, S1-06. Persist to `window_positions` at the end of a move or resize.
  *DoD:* move and resize a note, restart → it reopens exactly where and how it was.
  *Refs:* §12, §8

- [x] **S1-19 · Bound `revision_history` growth** — `S`
  Needs: S1-17. Trim to the most recent ~200 rows once a widget passes ~500, or growth is
  unbounded and the budget fails.
  *DoD:* a test that writes 600 updates leaves ≤ 200 rows, and the db stops growing.
  *Refs:* §12, §2.5

### 1h — App shell: home page, tray, note lifecycle

**Replanned 2026-08-24 after the v1 review.** This section supersedes the original
"Multiple widgets on the desktop". §8 and §9 assume a bare desktop of free-floating
widgets with a ⌘K palette and no home surface; the product now has a home window that
notes are created and launched from, a tray icon that keeps Canvas alive, and a close
that is not a delete.

Two dependency changes were signed off at this point, both required by these tasks:
`windows-sys` (Win32, outside the spec's closed stack) and Slint's own `system-tray`
and `raw-window-handle-06` features (inside it — build flags only).

- [x] **S1-20 · Multi-window runtime** — `L`
  Needs: S1-17. Rust owns a registry of live note windows keyed by widget id: spawn,
  look up, close, drop. One `StickyNoteWindow` per open note, each with its own
  position, size, tone and document. Z-order = most recently interacted on top;
  focusing one reorders **no** other. **No grid, no snapping, no auto-arrange**, overlap
  allowed, and **the app draws no wallpaper**.
  *DoD:* three notes open at once, each independently draggable and editable, each
  persisting its own text and position. Closing one leaves the others untouched.
  *Refs:* §8

- [ ] **S1-21 · Close ≠ delete** — `M`
  Needs: S1-20. Closing a note window hides it and leaves the row in `widgets`; the note
  reopens from home with its text and position intact. **Delete is the only thing that
  removes data**, and it keeps the in-widget confirm from S1-14.
  *DoD:* close a note, restart the app, reopen it from home — text and position are
  exactly as they were. Deleting genuinely removes it from the database.

- [ ] **S1-22 · Pin = glued to the desktop layer** — `M`
  Needs: S1-20. Pinned notes sit at the bottom of the z-order so they are visible on the
  desktop but never over other apps; unpinned notes are ordinary windows.
  `always-on-top` is off in both states. Implemented with `SetWindowPos(HWND_BOTTOM)`
  re-applied on a poll while the note is not the foreground window, so clicking a pinned
  note still lets you type in it and it settles back afterwards.
  *DoD:* pin a note, open a browser over it — the note stays behind. Click the note,
  type, click away; it returns to the desktop layer on its own. Pin state survives a
  restart.

- [x] **S1-23 · Home window shell** — `M`
  Needs: S1-02, S1-03. A Panel like any widget: 32px header, glyph strip, title `CANVAS`,
  same tokens and measurements. Sized for a list rather than a note.
  *DoD:* opens, is draggable and resizable, matches the note's visual language exactly.

- [ ] **S1-24 · Home: the note list** — `L`
  Needs: S1-23, S1-16, S1-20. Every note in the database as a row: title plus a preview
  line of its text. **A row shows whether that note is currently open on screen.** Click
  a row to open its window; click again to close it.
  *DoD:* the list matches the database, open/closed state is visibly correct and updates
  live as notes are opened and closed from anywhere.

- [ ] **S1-25 · Home: create** — `M`
  Needs: S1-24. A create control per widget type — sticky note now, the other types as
  they are built. New notes get their type's default size (§7.1, §10.1), the title
  `UNTITLED`, and open immediately.
  *DoD:* creating a note adds a row, opens a window, and survives a restart.

- [ ] **S1-26 · Home: delete** — `M`
  Needs: S1-24. Delete a note from its row, with the same confirm language as §7.6
  state 10. Removes the widget, its document, its position and its revision history.
  *DoD:* deleted notes are gone from the list, from disk, and from screen if open.

- [ ] **S1-27 · Tray icon and app lifetime** — `M`
  Needs: S1-23. Canvas keeps running when home is closed. The tray icon reopens home;
  quit is explicit and from the tray only. **Closing the home window must not close a
  single note.** Needs Slint's `system-tray` feature re-enabled — I disabled it for
  binary size in S0-2.
  *DoD:* close home with two notes open — both stay on screen and editable. Reopen home
  from the tray; the list is correct. Quit from the tray closes everything cleanly.

- [ ] **S1-28 · First-launch empty state** — `S`
  Needs: S1-24. Home with zero notes: `NOTHING HERE YET` plus a line pointing at the
  create control, centred, matching the note's own empty state.
  *DoD:* shows on a fresh database, disappears on the first note.
  *Refs:* §8

- [ ] **S1-29 · Open/close animation** — `S`
  Needs: S1-20. Open 160ms, opacity 0→1 plus a 6px rise. Close 100ms, fade only.
  **No scale, no bounce, no tilt** (rule 5).
  *DoD:* both visible and correct; nothing scales.
  *Refs:* §8, §5.5

- [ ] **S1-30 · Off-screen clamping** — `M`
  Needs: S1-20. Clips at the boundary, radius and border simply cut — **no push-back, no
  bounce, no snap-in**. At least 120px of width plus the full header stays grabbable.
  Fully off-screen after a display change → nearest edge on next launch.
  *DoD:* dragged to each edge and verified. The display-change case may not be testable
  here — if not, mark it `🟡` and do not tick the box.
  *Refs:* §8

### 1i — Command palette

Still built, per your "both together" — home for browsing and managing, ⌘K for fast
keyboard creation from anywhere. This is also what keeps Stage 1's "creatable with no
mouse" bar reachable.

- [ ] **S1-31 · The five palette icons** — `S`
  Needs: S1-05. `search`, `plus`, `check`, `settings`, `enter`, same construction rules,
  rendered at **24px — a size that exists nowhere else in the product**.
  *DoD:* all five render correctly at 24px.
  *Refs:* §5.6, §9

- [ ] **S1-32 · Palette overlay shell** — `M`
  Needs: S1-31. 640px wide, radius 28, `plate` fill, 1px `edge`, drag-depth shadow.
  Centred horizontally, top at ~15% of screen height. `scrim` behind it — **flat, no
  blur**; the one deliberate translucency exception, and only while the palette is open.
  *DoD:* appears centred with the scrim; nothing behind it is blurred.
  *Refs:* §9, §5.1

- [ ] **S1-33 · Palette input row + empty-query actions** — `M`
  Needs: S1-32. Input row 56px, padding 0/24, 24px `search` icon + 12px gap + query in
  the interface face at 22px + a 2×22 caret. Empty placeholder `TYPE A COMMAND`.
  Default actions: **New note** (⌘N, `plus`), **New todo** (⌘T, `check`), **Settings**
  (⌘, `settings`).
  *DoD:* opens showing the three defaults; typing replaces them with results.
  *Refs:* §9

- [ ] **S1-34 · Palette results, selection, footer** — `M`
  Needs: S1-33. Rows 48px, 24px icon + 16px gap + body-14 label + right-aligned
  interface-11 `dim` meta. Selection = row fill `elev-raised`, icon steps to `text` —
  **no accent fill, no left bar**. Footer 32px, count left,
  `↑↓ MOVE · ⏎ OPEN · ESC CLOSE` right. Searching existing notes belongs here too.
  *DoD:* ↑↓ moves, ⏎ opens, Esc closes, count correct, no accent anywhere.
  *Refs:* §9, rule 2

- [ ] **S1-35 · Palette hotkey** — `M`
  Needs: S1-34. Ctrl+K opens it; Ctrl+N creates a note.
  A **true OS-global** hotkey needs Win32 `RegisterHotKey` — `windows-sys` is already a
  dependency by then, so this no longer needs a separate sign-off. The stopgap bound in a
  focused window's `key-pressed` is acceptable first, provided it is **stated plainly**
  as only firing while a Canvas window has focus.
  *DoD:* a note is creatable with **no mouse at all**. The hotkey's real scope stated
  honestly in `progress.md`.
  *Refs:* §14 Stage 1 item 10

### 1j — Network-reflecting states

These need a peer. Expect several to sit at `🟡` until Stage 3 — that is correct and
expected, not a failure. Do not fake them, and do not animate anything that lies about the
network.

- [ ] **S1-36 · Sync / offline / error strip states + conflict UI** — `L`
  Needs: S1-04, S1-17. States 5, 6, and 8. `syncing` sweeps **only while sync is genuinely
  in flight**. Offline: five cells `dot-half`, steady, plus an optional
  `OFFLINE — N CHANGES QUEUED` line in interface face 11 `dim` — **no banner, no icon, no
  toast, anywhere** (rule 3). Conflict: the strip steps to `signal` once and cell 3 holds;
  the body gets a 1px `signal` hairline, a `CONFLICT — 2 VERSIONS` label in `signal`, and
  `KEEP MINE` / `KEEP THEIRS` buttons in the **`default` variant, not red**.
  *DoD:* every state drivable from a local test harness (a `canvas-sync` stub is fine).
  Anything you cannot genuinely trigger → `🟡` with the reason.
  *Refs:* §7.6 states 5/6/8, §6.2, rules 2 and 3

- [ ] **S1-37 · Presence: `PresencePill` + state 7** — `M`
  Needs: S1-36. Pill height 20, min-width 28, padding 0/8, radius 999, 1px `edge`, 2-char
  initials in interface face 11. Fill = that peer's grey step — peers are distinguished by
  **grey level and initials, never colour**. The strip fills from the right, one cell per
  peer. Pills occupy the **same box** as the controls: peers present and pointer outside →
  pills; hover → controls. **They never stack.** Idle over 30s → initials drop to `dim` and
  the strip's lit cells become `dot-half`.
  *DoD:* driven from a fake-peer harness; the swap between pills and controls is clean in
  both directions.
  *Refs:* §7.6 state 7, §6.8, §5.1 (peer-1..4)

### 1k — The big one, and sign-off

- [ ] **S1-38 · Rich text 1/5: delta model in `canvas-doc`** — `L`
  Needs: S1-15. One `yrs::Text` carrying inline attributes (`bold` / `italic` / `code` /
  `strike`) plus a block-level attribute marking list-item membership — the flat delta model
  Yjs uses with Quill. **Not** an `XmlFragment` tree: no headings, tables, or nesting are
  needed, so the tree is unnecessary extra engineering. Replace S1-15's whole-string replace
  with a real diff here.
  *DoD:* unit tests — attributes survive a snapshot round-trip, and two clients applying
  overlapping formatting converge.
  *Refs:* §7.7, §11.1

- [ ] **S1-39 · Rich text 2/5: styled-run layout renderer** — `L`
  Needs: S1-38. Slint has **no built-in support** for mixed formatting within one flow —
  neither `Text` nor `TextInput`/`TextEdit` can do it. This is a custom wrapped flow of
  styled runs, and it is the single largest piece of engineering in the sticky note.
  Bold = weight 600 at the **same colour** — never a colour change. Strikethrough steps the
  colour to `dim` plus a 1px line. Inline code: body face (**no monospace face exists in
  this product**), `elev-recessed` fill, 1px `edge` border, radius 8, padding 1/5 — the
  border is what makes it read; the fill alone is invisible against `plate`. Bullet markers
  are 3×3 `dim` squares (the dot-cell shape, not a typographic bullet), 8px gap to text,
  offset 9px from the item top. Block and paragraph gap 12.
  *DoD:* a document containing every format renders correctly and rewraps on resize.
  *Refs:* §7.7, §7.2

- [ ] **S1-40 · Rich text 3/5: caret and selection** — `L`
  Needs: S1-40. Caret 2×14 inline (2×18 on an empty line), colour `text`, 1000ms **2-step
  hard switch — no fade**. Selection uses the `edge` background.
  *DoD:* click-to-place, drag-to-select, shift-arrow selection, and Home/End all land on the
  correct character across mixed-format runs.
  *Refs:* §7.2, §5.5 (caret row)

- [ ] **S1-41 · Rich text 4/5: input and editing** — `L`
  Needs: S1-33. Insert, delete, backspace, paste-as-plain-text, Enter/paragraph splitting,
  and list continuation — all routed through the delta model, never through a plain string.
  *DoD:* editing mid-document preserves the surrounding formatting; the 12px paragraph gap
  holds.
  *Refs:* §7.7, §7.2

- [ ] **S1-42 · Rich text 5/5: formatting shortcuts + persistence** — `M`
  Needs: S1-41, S1-17. Ctrl+B, Ctrl+I, strikethrough, inline code, bullet toggle.
  *DoD:* **format a note, restart the app, the formatting is still there** — this is a named
  Stage 1 DoD item, not an implied one.
  *Refs:* §14 Stage 1 DoD

- [ ] **S1-43 · Undo/redo backed by `revision_history`** — `L`
  Needs: S1-42, S1-19. Backed by the stored revision history, **not** a local edit stack — a
  local stack gives wrong results as soon as a remote edit interleaves.
  *DoD:* undo is correct **after a remote edit arrives mid-session** (named Stage 1 DoD).
  Drive the remote edit from the S1-36 harness.
  *Refs:* §14 Stage 1 item 12 and DoD

- [ ] **S1-44 · Reduced motion** — `M`
  Needs: S1-43. All travel, stepping, and shadow motion off; a 100ms crossfade replaces
  them; dots hold their final state. **Required for sign-off**, not a deferred nice-to-have.
  *DoD:* with the OS reduced-motion setting on, no widget travels, steps, or shifts shadow.
  *Refs:* §5.5

- [ ] **S1-45 · Stage 1 sign-off audit** — `M`
  Needs: S1-44. Walk all 11 states of §7.6 against the table **and** against
  `handoff/screenshots/sticky-note-states.png`, side by side. Re-measure per-widget RSS
  (must be 5–15 MB) and cold launch. Re-check the five rules line by line — especially that
  `signal` red appears in exactly two places in the entire product, and that no dot cell
  ever fades.
  *DoD:* every Stage 1 DoD bullet in §14 is **truthfully** satisfiable. Anything that isn't
  becomes a new numbered task here rather than a generous tick.
  *Refs:* §14 Stage 1 DoD, §1 (the five rules)

---

## Stages 2–5 — not yet broken down

Deliberately coarse. Expand each into numbered tasks **when it is reached** — breaking them
down now would only mean rewriting them, since the earlier stages will change what's true.

- [ ] **Stage 2 · Backend skeleton** — Axum, Postgres, Redis, object storage, Docker Compose
  for local dev, OAuth, workspaces, permissions. *Refs:* §14 Stage 2
- [ ] **Stage 3 · Sync, hardened** — a binary wire protocol replacing the placeholder JSON
  `SyncMessage`, offline-then-reconnect replay, snapshot compaction (**required before this
  stage ships**, not optional), convergence tests with N ≥ 4 simulated peers.
  *Refs:* §14 Stage 3
- [ ] **Stage 4 · Todo, Shared Document, Calendar** — Todo first (§10.1: simplest, and the
  proof that a second widget type works; the array-of-maps code in §11.2 is already
  verified). Shared Document cursors via **relative positions** — absolute offsets break
  under concurrent inserts and must not ship even as a placeholder. This stage also lands
  the rest of the component library: Checkbox (§6.4 — radius **2px**, since 8px on a 16×16
  box renders as a disc), Tag (§6.5), TextField (§6.6), Divider (§6.7).
  *Refs:* §14 Stage 4, §10.1–10.4, §11.3
- [ ] **Stage 5 · Whiteboard and Kanban** — Kanban cards positioned by **fractional index**,
  never nested-array reordering (a known CRDT failure mode: duplication and loss under
  concurrent moves). Whiteboard strokes sync **incrementally** as points are added, never as
  full-canvas resends. *Refs:* §14 Stage 5, §10.3, §10.5

---

## Open questions — ask before the named task starts

Per §4.1: ask rather than guess on anything touching the CRDT data shape, the wire
protocol, the storage schema, or a design token not already specified.

**Answered — kept for the record**

- ~~What does the colour control do?~~ **Cycles the plate through 5 grey tones**
  (`Palette.plate-tone`). Rule 1 leaves no colour for it, so it sets grey level.
- ~~Always-on-top support?~~ **Moot.** Slint 1.17 exposes `always-on-top` on `Window`,
  but the user rejected the behaviour outright. Pin now means glued to the desktop layer.
- ~~Is a stopgap global hotkey acceptable?~~ **No longer needed as a stopgap.**
  `windows-sys` is already a dependency, so `RegisterHotKey` gives a real OS-global
  hotkey at S1-35.

**Still open**

1. **What should pin do beyond the desktop layer?** Every note is now non-floating, so
   pin's only current effect is the bottom z-order. Should a pinned note also be exempt
   from close, or always reopen at launch? *Blocks:* nothing yet; decide by S1-27.
2. **Rich text vs the dot-matrix body face.** Silkscreen ships Regular only — no bold, no
   italic — but §7.2 requires both. Either revert `Typography.body-face` to the system
   sans, express emphasis some other way, or drop those two formats. **Must be answered
   before S1-39.**
3. **Is light mode in scope, or dark only?** Both token sets are built and the three
   documented light corrections are applied. The open part is only whether a switcher
   ships, and where its control lives now that a home window exists.
4. **`handoff/screenshots/light-mode.png` contains "the written verdict on what breaks"**
   (§15) and has still not been read. Read it before shipping light mode.
5. **Which widget type comes after the sticky note?** §14 Stage 4 says Todo first, and
   §11.2 already carries verified array-of-maps code for it. The home window's rows
   already render per-type glyphs, so a second type needs no home work. Confirm Todo is
   the right next type before Stage 4 starts.


---

## Session-end checklist

Before a session ends — especially one running low on context:

1. `TASKS.md`: boxes ticked, **Current task** line updated.
2. `progress.md`: dated entry appended with honest `✅` / `🟡` / `❌` markers.
3. `git commit` — the working tree should never be left dirty at a session boundary.
4. Anything half-finished: leave a `// TODO(<task-id>)` in the code **and** a note under
   that task here.
