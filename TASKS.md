# Canvas — Task Backlog

**Purpose:** a resumable, ordered list of small units of work. A new session reads
[progress.md](progress.md) first for the state of the project, then starts here at the
first unticked box.

**Last updated:** 2026-08-25
**Current task:** `S1-31` — the five palette icons
**Stage:** 1 — Sticky note + app shell
**Done:** 21 ticked, 44 open (5 dropped, 4 added — S1-46, S1-47 and the two Spreadsheet tasks)
**Left 🟡 for the user to look at:** `S1-28` (home empty state), `S1-29` (open/close animation), `S1-30` (off-screen clamping) — see the verification queue

**Told "go" and nothing else?** Do not ask what to work on and do not stop after one task.
Read the 🟢 GO block at the top of [progress.md](progress.md), then work **The loop**
below, task after task, until the backlog runs out or something on the stop list in that
section is true.

## ⚠ BUILD ORDER OVERRIDDEN — 2026-08-25

The user reordered the roadmap after reviewing what was left:

> *"all these widgets etc should be functioning correctly first and then only we can add
> the multiplayer stuff — so lets fix what's left with the current stage and then build
> out all of the widgets then stage 2 etc"*

**The order is now:**

1. **Finish Stage 1** — everything that does not need a second machine. Tray, palette,
   off-screen clamping, animation, reduced motion, and the whole verification queue.
2. **Stage W — every widget type**, single-player and fully working: Todo, Kanban,
   Calendar, Shared Document, Whiteboard, and a **Spreadsheet** the user added on
   2026-08-25. This is the old Stages 4 and 5, moved ahead of the backend, plus one type
   that is not in the spec at all.
3. **Stage 2 — backend skeleton**, then **Stage 3 — hardened sync**, then the two
   deferred Stage 1 tasks (S1-36, S1-37) that genuinely need a peer.

**Rich text is dropped** (S1-38 … S1-42). The user's call: Silkscreen ships Regular only,
and they do not want bold or italic. This closes open question 2. Body stays plain text in
one face. The tasks are struck through below rather than deleted, so the decision stays
visible instead of looking like an oversight.

---

## How to use this file

### The loop — this is what "go" means

**"go" means: work the backlog continuously until it runs out or the user stops you.**
It does **not** mean "do one task and come back". Repeat this cycle without pausing:

1. Read the 🟢 GO block at the top of [progress.md](progress.md), then the
   **Current task** line above.
2. Build **that one task only**. Do not chain two into one pass, and do not refactor
   next door.
3. `cargo clippy --workspace --all-targets` and `cargo test --workspace` must both be
   clean. `cargo fmt --all`.
4. Re-read **DO NOT REGRESS** in [progress.md](progress.md) and confirm you broke nothing
   on it. Anything on that list that a task genuinely has to change needs the user's word
   first.
5. Update the docs in this order: tick the box (or leave it open and add a 🟡 note —
   see rule 7), move **Current task**, rewrite the GO block in `progress.md`, append a
   dated entry to `progress.md`, `git commit` with the task ID in the subject.
6. **Go straight back to step 1 for the next task.** No summary, no check-in, no asking
   what to do next. The user said "go" once; that covers the whole backlog.
7. Stop only when one of these is true, and say which: the backlog has no task left that
   can be done without the user · the next task is blocked on an **Open question** below
   and no unblocked task remains · step 3 or step 4 has failed twice on the same task.

At the end of a run, report once: the task IDs you closed, anything left 🟡 for the user
to look at, and any question that blocked you.

### The rules the loop runs on

1. Work tasks **in numeric order**. A task's `Needs:` line lists its hard prerequisites.
   A task blocked on an open question is skipped, not guessed at — move to the next
   unblocked one and note the skip.
2. **Only tick a box for behaviour you actually saw work.** You cannot see the screen
   (§13.7). So anything that needs eyes lands as: code committed, `🟡` entry in
   `progress.md`, **box left unticked**, and a new line in the VERIFICATION QUEUE below
   saying exactly what to click. Then you carry on to the next task. This is the whole
   reason the loop can keep moving without the user in it.
3. **Never regress working behaviour to make a new task easier.** The DO NOT REGRESS
   list in `progress.md` is the floor.
4. **Write no code comments.** User instruction. Reasoning goes here, in `progress.md`,
   and in commit messages.
5. **Ask before adding a dependency** (Hard Constraint 6). Enabling a feature on a crate
   already in the closed stack is not a new dependency; adding a crate is.
6. Hit a spec ambiguity? Add it to **Open questions** at the bottom, skip past it, and
   raise it in the end-of-run report. Do not guess on a CRDT shape, a wire protocol, a
   storage schema, or a design token.
7. Every number you need should already be in CLAUDE.md §5. If it isn't, take it off the
   existing 8px spacing scale and say so in `progress.md` — do not invent a token.

Size legend: `S` ≈ under 30 min · `M` ≈ 30–90 min · `L` ≈ half a session or more.

---

## ⚠ VERIFICATION QUEUE — do this first

Everything below is **written, compiling and clippy-clean, but has never been watched
working.** It was all built in one session without a click-through. Run `cargo run -p
canvas-app` and go through this list; tick the task where it passes, open a new numbered
task where it does not.

This queue is how the loop keeps moving without the user in it: anything a session cannot
see gets committed, logged 🟡, and listed here with exactly what to click. Ticked lines
are confirmed — leave them ticked. Do not re-open a ticked line to "make sure".

**Note window**
- [ ] Drag the header — window follows 1:1, no drift, no inertia → **S1-07**
- [ ] Hover anywhere in the header — four controls fade in over 100ms, nothing shifts → **S1-08**
- [x] Drag each of the 4 edges and 4 corners — resizes like a normal window, clamps at
      240×180, corners do both axes → **S1-06** ✅ confirmed 2026-08-25
- [ ] Tab through: body → colour → pin → delete → close, each showing a focus ring;
      Shift+Tab reverses; Esc from the body drops focus → **S1-11**, **S1-12**
- [ ] Click the colour icon repeatedly — plate steps through 5 grey tones → part of **S1-05**
- [ ] Click the trash icon — in-widget confirm appears, `CANCEL` restores, `DELETE NOTE`
      removes it. No modal, no desktop dimming → **S1-13**, **S1-14**
- [ ] Double-click the header — title becomes editable, pre-selected, Enter commits
- [ ] Click `x` — note closes but stays in the home list → **S1-21**
- [ ] Pin a note, open a browser over it — note stays behind. Click it, type, click away;
      it settles back to the desktop layer → **S1-22**
- [x] Type past the bottom of a note — a 4px thumb appears at the body's right edge,
      tracks the wheel, and drags to scroll. No thumb when the text fits → **S1-46**
      ✅ confirmed 2026-08-25
- [ ] Ctrl+G in the body — cycles all 7 strip states, title names each one → **S1-04**
- [ ] Move and resize a note, restart the app — it reopens exactly where it was → **S1-18**

**Home window**
- [ ] `NEW NOTE` — creates a note, opens its window, adds a row → **S1-25**
- [ ] Click a row — toggles that note open/closed; the row's dots reflect it → **S1-24**
- [ ] Row delete icon — confirm overlay, then gone from list and from disk → **S1-26**
- [ ] Empty state shows on a fresh database (delete `%APPDATA%/Canvas`), centred in the
      list area, and disappears the moment the first note exists → **S1-28**
- [ ] Open three notes at once — all independently draggable and editable → confirms **S1-20**
- [ ] Home `x` closes home and leaves notes running

**Open/close animation** → **S1-29**
- [ ] `NEW NOTE` — the new note's plate fades up over ~160ms and rises 6px into place.
      Nothing scales, tilts or bounces
- [ ] Click `x` on a note — the plate fades out over ~100ms and does **not** travel
      downward on the way out
- [ ] Same two checks on the home window: tray → `Open home` fades it up, `x` fades it out
- [ ] Delete a note from its own trash icon — same 100ms fade, not an instant vanish

**Off-screen clamping** → **S1-30**
- [ ] Drag a note off the left, right and bottom edges — it clips at the boundary, no
      push-back and no snap-in, and always leaves ~120px of plate plus the whole header
      grabbable
- [ ] Drag a note up past the top edge — the header never goes above the screen top
- [ ] Same four drags on the home window
- [ ] On a two-monitor setup, drag a note across the join — it must **not** stop at the
      internal edge. This is the one that would prove the virtual-screen clamp right
- [ ] 🟡 not testable here: move a note onto a second monitor, unplug it, relaunch —
      the note should come back at the nearest edge of the remaining screen

**Tray** → **S1-27** ✅ confirmed 2026-08-25
- [x] A Canvas icon (five dots) sits in the notification area at launch
- [x] Close home with two notes open — both stay on screen and editable
- [x] Left-click the tray icon — home comes back with the right list
- [x] Close home AND every note — Canvas stays running, tray icon still there
- [x] Right-click the tray — `New note` creates and opens one
- [x] `Quit Canvas` closes everything and the process actually exits

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

- [x] **S1-06 · Frameless window + native resize** — `M`
  Needs: S1-03. `no-frame: true`, `resize-border-width: 6px`, `preferred-width/height`
  320×280, `min-width/height` 240×180, **no max** (omit the properties; don't set a huge
  sentinel). `width` and `min-width` cannot coexist on one element — the compiler rejects
  it outright. Try `resize-border-width` **before** writing any manual pointer-tracking
  resize logic.
  *DoD:* the window resizes from every edge and corner, clamps at 240×180, never locks
  aspect ratio.
  *Refs:* §13.5, §7.3
  ✅ **Confirmed by the user 2026-08-25** — "resizing etc works", after the third attempt.
  The answer was `Layout.resize-border` = 36px: 24px of transparent `plate-margin` plus
  12px inside the visible plate, so the grab zone straddles the plate edge. Do not lower
  it without re-testing; at 6px and at 30px the zone sat almost entirely in empty space.
  The `WM_NCLBUTTONDOWN` mechanism it replaced must never come back — see trap 4.

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

- [x] **S1-27 · Tray icon and app lifetime** — `M`
  Needs: S1-23. Canvas keeps running when home is closed. The tray icon reopens home;
  quit is explicit and from the tray only. **Closing the home window must not close a
  single note.** Needs Slint's `system-tray` feature re-enabled — I disabled it for
  binary size in S0-2.
  *DoD:* close home with two notes open — both stay on screen and editable. Reopen home
  from the tray; the list is correct. Quit from the tray closes everything cleanly.
  ✅ **Confirmed by the user 2026-08-25.** `ui/tray.slint` (`CanvasTray inherits SystemTrayIcon`),
  `assets/icons/tray.png` (the five-cell glyph strip, 64×64 white on transparent), and
  `App::install_tray` / `sync_tray` / `show_home` / `quit` in `app.rs`. `maybe_quit` now
  returns early whenever a tray exists, so the tray is the only way out; without one
  (a Linux desktop with no StatusNotifier host) the old last-window-quits rule still
  applies. Tick it against the **Tray** block in the verification queue.

- [ ] **S1-28 · First-launch empty state** — `S`
  Needs: S1-24. Home with zero notes: `NOTHING HERE YET` plus a line pointing at the
  create control, centred, matching the note's own empty state.
  *DoD:* shows on a fresh database, disappears on the first note.
  *Refs:* §8
  🟡 **Built 2026-08-25, needs your eyes.** `home.slint` — the two lines now sit in a
  plain `Rectangle` spanning the whole list region and are centred inside it by absolute
  `y`, rather than a `VerticalLayout` parked 48px below the header. The rows `Flickable`
  became `if root.rows.length > 0:` so an empty list has nothing on top of the message.
  Verified only that a genuinely fresh `%APPDATA%/Canvas` boots clean into this branch.

- [ ] **S1-29 · Open/close animation** — `S` — 🟡 built, needs eyes
  Needs: S1-20. Open 160ms, opacity 0→1 plus a 6px rise. Close 100ms, fade only.
  **No scale, no bounce, no tilt** (rule 5).
  *DoD:* both visible and correct; nothing scales.
  *Refs:* §8, §5.5
  Built on both windows: the **plate** animates, not the OS window, so the 6px rise runs
  inside the 24px transparent margin and never clips. `opened`/`closing` are Rust-driven
  so a re-shown home window replays the open. Close is a real 100ms delay before `hide()`.

- [ ] **S1-30 · Off-screen clamping** — `M` — 🟡 built, needs eyes
  Needs: S1-20. Clips at the boundary, radius and border simply cut — **no push-back, no
  bounce, no snap-in**. At least 120px of width plus the full header stays grabbable.
  Fully off-screen after a display change → nearest edge on next launch.
  *DoD:* dragged to each edge and verified. The display-change case may not be testable
  here — if not, mark it `🟡` and do not tick the box.
  *Refs:* §8
  Clamped against the **virtual screen rect** (`SM_*VIRTUALSCREEN`), not the nearest
  monitor's work area — a per-monitor clamp would push back at the internal edge between
  two monitors, which §8 forbids. The clamp keeps 120px of plate width and the full 48px
  header band on screen. Applied on the drag path and again after `show()`, which is what
  covers the display-change case. **The display-change half is untestable here.**

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

### 1j — Network-reflecting states — DEFERRED until after Stage 3

**Moved out of Stage 1 on 2026-08-25.** These two are the only Stage 1 tasks that
genuinely need a peer, and the user's order puts all multiplayer work last. They are not
cancelled — they come back immediately after Stage 3 lands. Until then the strip's
syncing / offline / peers / error states stay reachable only through the Ctrl+G dev
cycler, which is honest: nothing animates that would lie about a network that does not
exist yet (rule 3).

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

### 1k — Sign-off

- [ ] ~~**S1-38 … S1-42 · Rich text, 5 tasks**~~ — **DROPPED 2026-08-25.**
  Bold, italic, strikethrough, inline code and bulleted lists in the note body, on a flat
  delta model. Dropped by the user: the body face is Silkscreen, which ships Regular only,
  and they do not want the two formats it cannot render. The note body stays a single
  plain-text flow in one face. `StickyNoteDoc` already diffs properly (S1-15), so nothing
  else depended on this. **This closes open question 2.**
  Reinstating it means reverting `Typography.body-face` to the system sans first —
  see §7.2 and the override table in `progress.md`.

- [ ] **S1-43 · Undo/redo backed by `revision_history`** — `L`
  Needs: S1-19. **Re-based 2026-08-25** — used to need S1-42, which is dropped, so this now
  operates on the plain-text doc. Backed by the stored revision history, **not** a local
  edit stack — a local stack gives wrong results as soon as a remote edit interleaves, and
  building it as a stack now would only have to be thrown away at Stage 3.
  *DoD:* undo/redo correct across a restart, single-player. The named Stage 1 DoD bullet
  — *"undo works correctly after a remote edit arrives mid-session"* — **cannot be
  satisfied until Stage 3 exists.** Mark that half 🟡 with this reason; do not tick it and
  do not fake a remote edit.
  *Refs:* §14 Stage 1 item 12 and DoD

- [ ] **S1-44 · Reduced motion** — `M`
  Needs: S1-29. All travel, stepping, and shadow motion off; a 100ms crossfade replaces
  them; dots hold their final state. **Required for sign-off**, not a deferred nice-to-have.
  *DoD:* with the OS reduced-motion setting on, no widget travels, steps, or shifts shadow.
  *Refs:* §5.5

- [ ] **S1-45 · Stage 1 sign-off audit** — `M`
  Needs: S1-44. Walk all 11 states of §7.6 against the table **and** against
  `handoff/screenshots/sticky-note-states.png`, side by side. Re-measure per-widget RSS
  (must be 5–15 MB) and cold launch. Re-check the five rules line by line — especially that
  `signal` red appears in exactly two places in the entire product, and that no dot cell
  ever fades.
  *DoD:* every Stage 1 DoD bullet in §14 is **truthfully** satisfiable **except the three
  now known to be out of reach** — rich text (dropped), states 5/6/7/8 (need a peer,
  deferred to 1j), and undo-after-a-remote-edit (needs Stage 3). Those three are recorded
  as deferred, not ticked. Anything else that isn't satisfiable becomes a new numbered
  task here rather than a generous tick.
  *Refs:* §14 Stage 1 DoD, §1 (the five rules)

### 1k — added after the fact

- [x] **S1-46 · Visible scrollbar in the note body** — `S`
  Needs: S1-09. **Overrides §7.3**, which says the body scrolls with no visible scrollbar.
  The user asked for one directly: the wheel worked but gave no sense of position or
  length. `ScrollBar` in `theme/controls.slint` — a 4px thumb, `dim` at rest stepping to
  `text` on hover/press, radius 2px, **no track drawn**, opacity 0 when the text fits.
  It sits in an 8px gutter the body reserves permanently, so the `TextInput` and the
  thumb's `TouchArea` never share a region (§13.3) and the gutter appearing cannot
  re-wrap the text into needing it. Dragging the thumb maps 1:1 onto `viewport-y`, which
  is two-way bound to the `Flickable`, so wheel and drag stay in step.
  *DoD:* the thumb appears only on overflow, tracks the wheel, drags to scroll, and
  reaches both ends. Reuse it for every scrolling widget in Stage W rather than
  rebuilding one per type.

- [ ] **S1-47 · The same scrollbar on the home list** — `S`
  Needs: S1-46. Home's rows `Flickable` still scrolls bare. Reuse `ScrollBar` — but the
  rows are full-bleed and carry a delete icon at their right edge, so the 8px gutter
  moves every row's icon inward. Decide that deliberately rather than on the way past.
  *DoD:* with more notes than fit, the thumb appears and drags; the delete icons still
  line up with the note window's own header cluster.

---

## Stage W — every widget type, single-player

**Created 2026-08-25 by the build-order override at the top of this file.** This is the
old Stages 4 and 5, pulled ahead of the backend. Every widget type gets built and made to
work correctly on one machine before any multiplayer work starts.

Broken down now rather than left coarse, because it is the next stage to be reached.

Stage DoD: all six widget types create, open, edit, close, delete and persist from home ·
each within the 5–15 MB per-widget budget · every type wears the same 32px header and
glyph strip (rule 3) · no second accent colour anywhere (rules 1 and 2).

### Wa — make the shell type-aware

Nothing else in this stage can start until these three land. Every one of them exists
because the app is currently hardcoded to a single type.

- [ ] **SW-01 · Generalise the runtime to any widget type** — `L`
  Needs: S1-20. `app.rs:50` filters boot to `KIND_STICKY_NOTE`, and `Note` is a concrete
  struct holding a `StickyNoteWindow` and a `StickyNoteDoc`. Both need to become
  type-dispatched: one live-widget registry that can hold any window/doc pair, spawned by
  `widget_type`. Keep the flush timer, the placement tracking and the pending-update
  buffer shared — they are type-agnostic already and must not be duplicated per type.
  *DoD:* a row of a non-note type in the database opens its own window at boot instead of
  being silently skipped. Verified with a hand-inserted row before any real second type
  exists.

- [ ] **SW-02 · Home: a create control per type** — `M`
  Needs: SW-01. `NEW NOTE` becomes a create control per built type, each using its own
  §5.6 glyph — the row glyphs in `home.slint` already exist and already pick correctly by
  kind, so the list side needs no work. New widgets get their own type's default size
  (already correct in `canvas-storage`) and the title `UNTITLED`.
  *DoD:* each type creatable from home, appears in the list with the right glyph, opens
  its own window, and survives a restart.

- [ ] **SW-03 · The four missing library components** — `M`
  Needs: S1-13. `controls.slint` currently has `IconButton` and `Button` only.
  Build `Checkbox` (§6.4 — radius **2px**, not 8px: 8px on a 16×16 box renders as a disc,
  and there is **no tick glyph**, the fill *is* the state), `Tag` (§6.5), `Divider` (§6.7),
  `TextField` (§6.6).
  *DoD:* each rendered against its §6 table by eye, in every state the table lists.

### Wb — Todo

- [ ] **SW-04 · `TodoListDoc` in `canvas-doc`** — `M`
  Needs: S0-2. §11.2's array-of-maps pattern, which is already verified-compiling against
  yrs 0.19.2 — use it as written. `{id, text, done}` only; **do not** build `due` or
  `assignee` speculatively. **Remember the trap:** yrs indexes by UTF-8 byte offset, so any
  text indexing inside an item must walk to a `char_boundary`.
  *DoD:* unit tests matching `StickyNoteDoc`'s — two docs adding and toggling concurrently
  converge, snapshot round-trips, garbage bytes rejected without a panic.

- [ ] **SW-05 · Todo widget window** — `L`
  Needs: SW-02, SW-03, SW-04. `340×360`. Row gap 12, `Checkbox` + 12px gap. Completed rows:
  `dim` label + 1px strike. Due date as a `Tag` on the right, gap 8. Footer: 1px `edge`
  rule + `N OPEN · N DONE` in interface face 11 `dim`.
  **Counts are computed in Rust and passed in as `in property<int>`** — a Slint binding
  cannot run an imperative loop, that is a parse error not a runtime limit (§13.6).
  *DoD:* add, toggle, edit and delete items; restart and they are all still there.

### Wc — Shared Document

- [ ] **SW-06 · Shared Document widget** — `M`
  Needs: SW-02. `520×420`, **24px padding** (the only type that is not 16), paragraph gap
  16. Single-player this is a sticky note at a larger size with wider padding, so it reuses
  `StickyNoteDoc` and most of `sticky-note.slint`.
  **The collaborator cursors (§10.2) are NOT in this task** — they need a peer and land
  after Stage 3, via relative positions, never absolute offsets.
  *DoD:* creates, edits, persists. Cursor work explicitly recorded as deferred.

### Wd — Calendar

- [ ] **SW-07 · `CalendarDoc`** — `M`
  Needs: SW-04. Array of maps `{id, title, start, end, attendees}` (§11.3).
  *DoD:* same convergence and round-trip tests as `TodoListDoc`.

- [ ] **SW-08 · Calendar widget window** — `L`
  Needs: SW-07. `480×360`. Seven equal day columns, gap 8. Day label interface-face-11
  `dim`; **today's label steps to `text` with a 1px `text` rule above it, and every other
  day gets a 1px `edge` rule in the same spot — it is a rule, not a wash.**
  Event chips: radius 8, height by duration (24/32/40/56px), fill from the peer-grey steps,
  vertical position = offset from column top.
  **Chips carry the 24-hour start time only, never the event name** — at a ~52px column an
  11px Silkscreen label holds five characters and a truncated name reads worse than none.
  Grey level carries duration. A day view is what is needed if names are ever required.
  *DoD:* a week renders with correctly positioned and sized chips; today is marked; events
  survive a restart.

### We — Kanban

- [ ] **SW-09 · `KanbanDoc` with fractional indexing** — `L`
  Needs: SW-04. Columns as an array of maps; cards keyed by id with a **fractional-index**
  position field. **Never nested-array reordering** — it is a known CRDT failure mode that
  duplicates and loses cards under concurrent moves, and it must not ship even as a
  single-player shortcut, because it would have to be rewritten at Stage 3 anyway.
  *DoD:* unit tests — a card moved between columns keeps its identity; two docs moving the
  same card concurrently converge with no duplication and no loss.

- [ ] **SW-10 · Kanban widget window + card drag** — `L`
  Needs: SW-09. `640×400`. Three equal columns, gap 12. Column header interface-face-11
  `text` + count `dim`, gap 8, then a 1px `edge` rule. Cards: `elev-raised`, 1px `edge`,
  radius 8, padding 12, gap 8, title body-face-14, meta interface-face-11 `dim`
  (`ASSIGNEE · DATE` or `UNASSIGNED`). Dragged card gets `shadow-drag` **only** — no tilt,
  no scale (rule 5).
  **The drop target is a 1px `text` outline around the whole column. No fill, no tint.**
  Card drag needs disjoint `TouchArea`s per §13.3 — do not layer one over the column.
  *DoD:* cards drag between columns, land in the right order, and survive a restart.

### Wf — Whiteboard

- [ ] **SW-11 · `WhiteboardDoc`** — `L`
  Needs: SW-04. Map of shapes `{type, x, y, w, h, rotation, style}` + arrays of point lists
  for strokes. **Strokes must be appended incrementally as points arrive**, never
  re-encoded as a full canvas — that shape is what makes Stage 3 traffic scale with stroke
  length instead of canvas size, so it has to be right from the start.
  *DoD:* unit tests — appending 1000 points produces updates proportional to the points
  added, not to the canvas contents.

- [ ] **SW-12 · Whiteboard widget window** — `L`
  Needs: SW-11. **No artboard exists for this one** — §10.5 is the only reference, so
  expect real design questions and ask rather than guess (§4.1). Infinite canvas, shapes,
  freehand strokes. Floating pill-shaped tool dock near the working area, **not** a docked
  toolbar. An 8px dot grid at `edge` opacity — the one place in the product where the
  underlying grid is deliberately made visible.
  *DoD:* draw, pan, zoom, erase; strokes survive a restart. Anything §10.5 does not answer
  becomes an open question below, not a guess.

### Wg — Spreadsheet

**Requested by the user 2026-08-25**, after the widget list was reviewed. It is **not in
CLAUDE.md at all** — there is no §10 subsection and no artboard for it, so it carries more
unanswered design questions than any other type here. Open questions 5 and 6 below must be
answered before SW-13 starts.

- [ ] **SW-13 · `SheetDoc`** — `L`
  Needs: SW-04. **The data shape is an open question — do not guess it** (§4.1: ask on
  anything touching a CRDT data shape). The likely answer is a **sparse map keyed by cell
  address**, not an array of row arrays: a nested-array grid has the same concurrent-move
  failure mode as Kanban's columns, and inserting a row or column mid-sheet would rewrite
  every key below it. If row and column insertion is in scope, both axes need
  **fractional indices**, exactly as SW-09 does for cards.
  *DoD:* unit tests — two docs editing different cells converge; two docs editing the
  **same** cell converge without either edit being silently dropped.

- [ ] **SW-14 · Spreadsheet widget window** — `L`
  Needs: SW-03, SW-13. Grid of cells with row and column headers.
  What the design language already settles, without needing a new token:
  - Grid lines are 1px `edge` hairlines (§6.7's divider rule — the gap positions them).
  - Row/column headers are interface face 11 UPPERCASE `dim`, like every other metadata
    label in the product.
  - Cell contents are body face 14/22.
  - **The selected cell is a 1px `text` outline — no fill, no tint**, the same decision
    §10.3 already makes for the Kanban drop target. There is no accent colour to select
    with (rules 1 and 2), and `signal` red is spoken for.
  - The editing cell reuses `TextField`'s focus treatment from SW-03.
  - Scrolling uses a `Flickable` with **no visible scrollbar track**, per §7.3.
  **Cell hit-testing must use one `TouchArea` over the grid that resolves the cell from
  `mouse-x`/`mouse-y`, not one `TouchArea` per cell** — per-cell areas would be
  thousands of overlapping regions, which is the §13.3 failure mode at scale, and a
  per-cell allocation the 5–15 MB budget will not absorb.
  *DoD:* type into cells, navigate with arrows/Tab/Enter, resize the window, restart —
  contents and grid geometry intact. Formula support is **out of scope** unless open
  question 5 says otherwise.

### Wh — sign-off

- [ ] **SW-15 · Stage W sign-off audit** — `M`
  Needs: SW-14. Walk all six types against `handoff/screenshots/widget-types.png` side by
  side — noting the artboard covers four of them, and that the Whiteboard and Spreadsheet
  have no reference to check against beyond the five rules.
  Re-measure per-widget RSS for each type (5–15 MB each) and app total (< 50 MB).
  Re-check the five rules across every new surface — especially that `signal` red still
  appears in exactly **two** places in the whole product, and that no dot cell fades.
  *DoD:* every Stage W DoD bullet truthfully satisfiable; anything that is not becomes a
  new numbered task rather than a generous tick.

---

## Stages 2, 3 — after Stage W

Deliberately coarse. Expand into numbered tasks when reached.

- [ ] **Stage 2 · Backend skeleton** — Axum, Postgres, Redis, object storage, Docker Compose
  for local dev, OAuth, workspaces, permissions. *Refs:* §14 Stage 2
- [ ] **Stage 3 · Sync, hardened** — a binary wire protocol replacing the placeholder JSON
  `SyncMessage`, offline-then-reconnect replay, snapshot compaction (**required before this
  stage ships**, not optional), convergence tests with N ≥ 4 simulated peers.
  *Refs:* §14 Stage 3
- [ ] **Then the deferred multiplayer tasks**, in this order: **S1-36** (sync / offline /
  error strip states + conflict UI), **S1-37** (presence pills), the Shared Document's
  collaborator cursors from SW-06 via **relative positions**, and the remote-edit half of
  **S1-43**'s undo DoD. All four were deferred, not cancelled.

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
- ~~Rich text vs the dot-matrix body face?~~ **Dropped, 2026-08-25.** The user does not
  want bold or italic, so Silkscreen shipping Regular only stops being a problem. The
  note body is plain text in one face. S1-38 … S1-42 are struck through above.
- ~~Which widget type comes after the sticky note?~~ **All of them, before multiplayer.**
  The user reordered the roadmap on 2026-08-25 — see the override at the top of this file.
  Todo is still first within Stage W, for the reason §10.1 gives.

**Still open**

1. **What should pin do beyond the desktop layer?** Every note is now non-floating, so
   pin's only current effect is the bottom z-order. Should a pinned note also be exempt
   from close, or always reopen at launch? *Blocks:* nothing yet; decide by S1-27.
2. **Is light mode in scope, or dark only?** Both token sets are built and the three
   documented light corrections are applied. The open part is only whether a switcher
   ships, and where its control lives now that a home window exists.
3. **`handoff/screenshots/light-mode.png` contains "the written verdict on what breaks"**
   (§15) and has still not been read. Read it before shipping light mode.
4. **Does the Whiteboard have a design?** §10.5 describes it in one paragraph and there is
   **no artboard for it** — `widget-types.png` covers Todo, Shared Document, Kanban and
   Calendar only. Tool dock contents, shape set, and the stroke style are all unspecified.
   *Blocks:* SW-12. Ask before starting it, do not invent a tool palette.
5. **Does the Spreadsheet support formulas?** A spreadsheet that only holds literal values
   is a table; one that evaluates `=SUM(A1:A9)` needs a parser, a dependency graph and a
   recalculation order, and that is a larger piece of engineering than any other widget in
   Stage W. It also has a CRDT consequence: a formula cell stores its *source*, and the
   computed value must be derived locally, never synced as a value — otherwise two peers
   can disagree about a cell neither of them typed. *Blocks:* SW-13. **Ask before starting.**
6. **Can Spreadsheet rows and columns be inserted and deleted mid-sheet, or is the grid a
   fixed size?** Fixed size makes the CRDT a flat map keyed by cell address, which is
   simple. Insertion needs fractional indices on **both** axes and is substantially more
   work. *Blocks:* SW-13, and it decides the data shape, so it cannot be deferred and
   retrofitted later.
7. **Is the Spreadsheet's default size 640×400 like the Kanban, or wider?** No spec value
   exists — `canvas-storage`'s `default_size` needs a real number for a new `KIND_SHEET`,
   and §12 says size by type at creation. *Blocks:* SW-14.
8. **Do the other widget types need their own header controls?** The note has four
   (colour · pin · delete · close). Colour-as-grey-tone makes sense on a note; whether a
   Kanban or Calendar wants it is undecided. *Blocks:* SW-05 onward — decide once, apply
   to all types, rather than per widget.

---

## Session-end checklist

Before a session ends — especially one running low on context:

1. `TASKS.md`: boxes ticked, **Current task** line updated.
2. `progress.md`: the 🟢 GO block rewritten to name the exact next action, and a dated
   entry appended with honest `✅` / `🟡` / `❌` markers.
3. `git commit` — the working tree should never be left dirty at a session boundary.
4. Anything half-finished: leave a `// TODO(<task-id>)` in the code **and** a note under
   that task here.
