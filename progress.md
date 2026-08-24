# Canvas — Progress Log

**Last updated:** 2026-08-24
**Current position:** Stage 0 done. Sticky note shell built and compiling; awaiting a verification pass before S1-15 (CRDT doc).

## Standing update rule

After implementing and testing anything, append a dated entry below with the
correct status marker, and downgrade or remove anything it contradicts. Do
not batch updates — a stale log is worse than no log.

Template:

### YYYY-MM-DD — <short title>
- ✅ <what now works> — verified by <manual step or test name>
- 🟡 <what landed but is untested>

Markers: ✅ implemented and confirmed working (name the check) · 🟡 code exists and
compiles, behaviour unconfirmed · ❌ not started.

---

### 2026-08-24 — Project scaffolding and backlog

- ✅ `CLAUDE.md` copied to the project root — verified present; the five markdown image
  links repointed at `handoff/screenshots/`, spec text otherwise unchanged. `handoff/`
  kept pristine as the reference copy.
- ✅ `TASKS.md` written — 6 Stage 0 tasks, 38 Stage 1 tasks, Stages 2–5 as coarse
  placeholders, plus 5 open questions and a session-end checklist.
- ✅ Rust toolchain confirmed present — `cargo 1.94.0`, `rustc 1.94.0`, `git 2.51.0`,
  verified by `--version` on each.
- ❌ Crate versions from §3 (`slint` 1.17.x, `yrs` 0.19.x, …) not yet checked for
  resolution — that is task `S0-1` and it gates everything after it.
- ❌ No cargo workspace, no crates, no code.

**Not verified:** nothing has been run or rendered. Every §5–§13 spec detail is still
unproven against the actually-installed crate versions.

### 2026-08-24 — Stage 0 complete, sticky note shell standing

**Stack (S0-1)**
- ✅ Everything resolves, and to the exact versions §11/§13 were verified against:
  `slint`/`slint-build` **1.17.1**, `yrs` **0.19.2**, `rusqlite` 0.32.1, `tokio` 1.53.1,
  `uuid` 1.25.0, `mimalloc` 0.1.52. Every API pattern in the spec applies verbatim —
  no re-verification needed. Verified with a throwaway probe crate in the scratchpad.

**Workspace (S0-2, S0-4, S0-6)**
- ✅ Five crates, versions pinned once in `[workspace.dependencies]`.
- ✅ `slint` built with `default-features = false` — only `compat-1-2`, `std`,
  `backend-winit`, `renderer-software`. Drops femtovg, system-tray and accessibility
  from the default build. **Deviation to flag:** §3 describes `--gpu` as a pure runtime
  flag, but a runtime flag requires femtovg compiled in permanently, which contradicts
  "lightest". GPU is now opt-in twice: the `gpu` cargo feature *and* `--gpu` at runtime.
  Without the feature, `--gpu` prints a clear message instead of silently ignoring you.
- ✅ Release profile: `opt-level="z"`, fat LTO, 1 codegen unit, panic=abort, stripped.
- ✅ `cargo clippy --workspace --all-targets` clean.

**Budget baseline (S0-5)** — release build, software renderer, single note window
- ✅ Working set **20.6 MB** (app idle budget 20–40 MB).
- ✅ Private bytes **7.0 MB** (per-widget budget 5–15 MB).
- ✅ Binary **8.3 MB**.
- ✅ Launch to visible window: **636 ms genuinely cold**, **37–67 ms warm**. The <300 ms
  budget is met warm and missed on a first-ever cold start; disk cache is the difference.
  Recorded honestly rather than quoting only the warm number.

**Theme (S1-01, S1-02, S1-03)**
- ✅ `Palette` — every §5.1 token, both modes, all three light-mode corrections applied.
  No colour literal exists anywhere else in the codebase.
- ✅ Fonts embedded by compile-time `import` of the .ttf per §13.1. Confirmed by eye:
  the title renders in Silkscreen, body prose in the system sans.
- ✅ `Panel` renders with the 0.96 plate, radius 20, 1px edge.
- 🟡 The shadow *deepening on drag* has not been watched happen yet.

**Transparency — resolved, and it changes nothing downstream**
- ✅ Per-pixel window transparency works with the software renderer on Windows 11.
  Confirmed by eye. The 24px `plate-margin` stays, so the drop shadow, the outside
  focus ring and state 9 are all achievable. Had it failed, `Layout.plate-margin` was
  written as the single constant to set to 0.

**Open questions 1 and 2 — answered**
- ✅ **Q2 (always-on-top):** Slint 1.17 exposes `always-on-top` directly on `Window`.
  No dependency outside the closed stack. Hard Constraint 6 is not engaged. Set to true
  per §8.
- ✅ **Q1 (colour control):** decided — it cycles the plate through grey steps
  (`Palette.plate-tone` 0–4, reusing the peer greys). Rule 1 leaves no colour for it to
  set, so it sets grey level, the same mechanism peers are distinguished by.

**Sticky note shell (S1-05..S1-14)** — three bugs found by looking at the real window
- ✅ Header icons render as real vector `Path`s and hover-lighten.
- ✅ Empty state renders.
- ✅ Fixed: **all header content was 16px too high.** It was placed at `y: 0` when the
  32px band actually starts at `y: 16` — the hairline was right, the contents were not.
- ✅ Fixed: **the glyph strip was 23×32, not 23×3.** The §13.4 gotcha bit in a spot the
  spec's own workaround doesn't cover: wrapping it in a plain `Rectangle` is not enough,
  because a layout element still expands to fill a non-layout parent. With the strip at
  full header height, `(parent.height - self.height) / 2` evaluated to 0 and parked it
  against the plate's top corner. It now carries an explicit 23×3.
- ✅ Fixed: **the caret sat on top of the placeholder** instead of after it (§7.6 state 1
  says "caret after"). The input is now offset below the measured placeholder block.
- ✅ Changed: **the plate focus ring no longer keys off body focus.** State 3 says
  focused/editing has "no border/fill/shadow change from state 2", and the body
  auto-focuses on open, so a ring there was permanent and reported nothing. It is now
  driven by a `keyboard-focused` property for the desktop layer to set.
- 🟡 **Everything below is code that compiles and has not been watched work:** drag,
  resize, tab order, the three controls' behaviour, delete confirm, hover reveal,
  the glyph strip's other six states. All await a verification pass.
- ❌ Nothing is persisted yet. No CRDT, no SQLite. `canvas-doc`, `canvas-storage` and
  `canvas-sync` are still empty stubs.

### 2026-08-24 — User-directed design revisions

Three spec deviations, all decided by the user after looking at the running app.
They override the corresponding spec values from here on:

- ✅ **Glyph strip is 8×8 cells with 4px gaps (56×8 total)**, not §6.2's 3×3/2px/23×3.
  At spec size it was unreadable on a real screen. `Layout.dot-size`, `dot-gap`,
  `strip-w`, `strip-h` carry it; the title's left edge moves 51 → 84 to follow.
- ✅ **Header content is vertically centred in its band**: 8px above, 8px below,
  rather than §7.1's 16 above / 0 below. Band total stays 48px, so every body-box
  measurement in §7.3 still holds exactly.
- ✅ **Empty state is a true placeholder**, not §7.6 state 1's two-line block:
  one line of `NOTHING HERE YET` at 25% opacity sitting on the body's first line,
  caret at its start. "Start typing." is removed — it was what pushed the caret
  down a line.

- ✅ **All code comments removed, and no new ones will be written.** User instruction.
  The reasoning behind the non-obvious code (the §13.3 disjoint-TouchArea header
  partition, the §13.4 explicit-size requirement on the glyph strip, why the body uses
  bare `TextInput` rather than `TextEdit`) now lives only in TASKS.md, this log, and
  the git history. Anyone editing the .slint files should read those first — the code
  no longer warns you at the point of the mistake.
- ✅ `cargo clippy --workspace --all-targets` still clean after the strip.

### 2026-08-24 — Body face becomes the dot-matrix face

- ✅ **Body prose now renders in Silkscreen, not the system sans.** User's call: the
  system sans read as "too generic" next to the dot-matrix header. Placeholder and body
  text now share one face and one size (14px), so the placeholder no longer jumps when
  you type — it simply becomes your text. Body size stays 14/22 per §5.2.
  This contradicts §5.2 ("body prose is always the body face"). `Typography.body-face`
  is the single switch to revert it.
- ✅ **Body gained 16px top padding**, so text no longer touches the header hairline.
  Padding is now even on all four sides of the body box. Costs 16px of body height
  against §7.3's numbers.
- ✅ **S1-04 done — a dev control cycles all seven strip states.** Ctrl+G with the body
  focused steps through idle / focused / syncing / offline / peers / peers-idle / error
  and back to live behaviour; the title displays which state is showing. This is the
  §14 Stage 1 DoD for the glyph strip.
- ✅ `docs/glyph-strip.md` written — what each of the seven states means, the four cell
  values, and why only idle and focused are currently reachable.

**Consequence to resolve before S1-32 (rich text):** Silkscreen ships Regular only — no
bold, no italic. §7.2 requires body bold at weight 600 and italic for rich text. With
Silkscreen as the body face, two of the five inline formats have no way to render.
Options when that lands: switch body back to the system sans, represent bold/italic
some other way, or drop them. Do not start S1-32 without deciding.

---

## v1 milestone — sticky note accepted (2026-08-24)

User signed off: *"it's a good first version of first step of whole project."*

**What works and has been seen working**
- ✅ A frameless, transparent, rounded sticky note renders on the desktop with the plate,
  header, hairline and body all at the agreed measurements.
- ✅ Silkscreen renders throughout — title at 22px, body and placeholder at 14px.
- ✅ Typing works; the placeholder becomes the text with no jump (S1-09, S1-10).
- ✅ Glyph strip reads clearly at 8×8 and reports idle vs focused correctly.
- ✅ All seven strip states reachable and legible via Ctrl+G (S1-04).
- ✅ Three header icons render as real vector paths and reveal on hover/focus.
- ✅ Release build inside every budget: 20.6 MB RSS, 7.0 MB private, 8.3 MB binary.

**Still 🟡 — compiles, not yet watched work**
- 🟡 Header drag, edge/corner resize, tab order, delete confirm, Button variants,
  plate tone cycling. None of these have been confirmed by use.

**Still ❌ — not started**
- ❌ No persistence of any kind. Close the app and the text is gone.
- ❌ `canvas-doc`, `canvas-storage`, `canvas-sync` are empty stubs. No CRDT, no SQLite.
- ❌ No way to create a second note. One window, hardcoded, that is the whole app.

**Direction change requested at this milestone**
The user wants the product reshaped before persistence work starts:
1. Pin must stop meaning "always float above every app" — the current
   `always-on-top: true` is intrusive.
2. There must be a way to create new notes.
3. There should be a **home page** that notes are created and launched from, which can
   be closed without taking the notes with it.
4. Closing a note must not mean deleting it. Today delete is the only way to make a note
   go away, which is wrong.

These reshape §8 and §9 of the spec (which assume a bare desktop of widgets plus a
command palette, with no home surface at all). Scoping questions are open — do not write
code for any of it until they are answered.

### 2026-08-24 — Replanned around a home page; pin rebuilt

**Backlog replanned.** §8 and §9 assume a bare desktop of widgets plus a ⌘K palette and
no home surface. The product now has a home window, a tray icon, and a close that is not
a delete, so Stage 1 section 1h was rewritten and everything after it renumbered.
55 tasks, 13 done. Decisions taken at this point:
- Pin = glued to the desktop layer.
- Home page **and** command palette, both built.
- Home shows every note, which are open, and creates/opens/closes/deletes them.
- A tray icon keeps Canvas alive when home is closed.

**Dependencies added — this is the Hard Constraint 6 record.** All signed off by the
user in the same exchange that chose the behaviours requiring them:
- `windows-sys` 0.59 (`Win32_Foundation`, `Win32_UI_WindowsAndMessaging`) — outside the
  spec's closed stack. Needed for desktop-layer pinning, and later for a true global
  hotkey (S1-35). Windows-only target dependency.
- `raw-window-handle` 0.6.2 — to get the HWND out of Slint. Slint already depends on
  this exact version, so it adds nothing new to the tree.
- Slint's own `raw-window-handle-06` feature enabled; `system-tray` still to enable at
  S1-27. Both are build flags inside the closed stack.

**S1-22 — pin is now desktop-layer glue**
- ✅ `always-on-top: true` removed from the note window. **Notes no longer float over
  other apps in either state** — that was the intrusive behaviour.
- ✅ Pinned: `SetWindowPos(HWND_BOTTOM, SWP_NOACTIVATE)` drops the note to the bottom of
  the z-order. Unpinned: `HWND_NOTOPMOST` restores an ordinary window.
- ✅ A 700ms poll re-applies the bottom position only while the note is **not** the
  foreground window. That is what makes a pinned note still editable: click it and it
  comes forward normally, click away and it settles back to the desktop on its own.
  Polling rather than a focus-change hook because Slint exposes no window-level
  focus-changed callback.
- 🟡 Not yet watched work — needs a browser opened over a pinned note to confirm.
- ❌ Pin state is not persisted yet; that lands with S1-21.

### 2026-08-24 — S1-15, S1-16, S1-19: CRDT and storage

**S1-15 — `StickyNoteDoc` (12 tests, all passing)**
- ✅ Convergence proven: two docs and three docs editing concurrently converge after
  exchanging updates. Updates are idempotent (applying one three times is safe).
  Garbage bytes are rejected without panicking. No last-write-wins anywhere.
- ✅ **Went past §11.1's sample, deliberately.** The spec's `update_text` replaces the
  whole string on every keystroke and its own comment says to diff instead once you
  build past plain text. It diffs now: common prefix/suffix are skipped and only the
  changed span is sent. An append to a 43-char note produces a <40 byte update rather
  than re-encoding the document.
- ✅ `diff_since(state_vector)` sends only what a peer is missing — needed for S1-36.

**⚠️ Real bug found, worth remembering: yrs indices are UTF-8 BYTE offsets, not
character offsets.** `yrs 0.19.2` defaults to `OffsetKind::Bytes` (confirmed by reading
`doc.rs:795`). The first diff implementation used character indices, which is correct
for ASCII and silently corrupts the moment a multi-byte character appears — the emoji
test produced `"héllo wörld🎉 🎉"` from an append. Prefix/suffix are now computed in
bytes and walked back to a `char_boundary` on both strings. §11.1's whole-string-replace
sample never hits this, so the spec gives no warning about it. **Any future code that
indexes a `yrs::Text` must use byte offsets.**

**S1-16 — `canvas-storage` (16 tests, all passing)**
- ✅ All seven §12 tables, `user_version` migration, idempotent.
- ✅ `documents` is widget-type-agnostic — one table proven to hold a note and a todo
  side by side, never one table per type.
- ✅ New widgets get `UNTITLED` and their own type's default size (note 320×280,
  todo 340×360, kanban 640×400, calendar 480×360, document 520×420).
- ✅ WAL journal mode, `synchronous=NORMAL` — keeps writes off the UI thread's critical
  path for Hard Constraint 2.
- ✅ Database lives in `%APPDATA%/Canvas/canvas_local.db`, not the working directory.
- ✅ **`window_positions` gained `pinned`, `tone` and `open` columns** beyond §12's
  schema. `open` is what makes close-≠-delete possible (S1-21): closing clears the flag,
  the widget row and its document stay. Proven by test.

**S1-19 — revision history bounded**
- ✅ Trims to the most recent 200 once a widget passes 500, so the row count oscillates
  between 200 and 500 rather than growing without limit. Verified at 600 and at 5000
  updates. Trimming one widget provably leaves other widgets' history alone.
- Note: the first version of this test asserted `count <= 200`, which the spec's own
  design does not hold — it bounces. The test now asserts the real contract.

### 2026-08-24 — S1-17, S1-18, S1-20: the app runtime. Typing survives a restart.

- ✅ **`canvas-app` is now a real multi-window runtime.** An `App` owns the `Store` and a
  map of live notes keyed by widget id; each note owns its window, its CRDT doc and its
  placement. Notes are spawned from the database at boot.
- ✅ **Verified end-to-end, not assumed:** typed `persistence works` into the running
  window via SendKeys, hard-killed the process, and read it back off disk with a new
  `--dump` flag. Text intact, 17 revisions recorded. This is §14 Stage 1's
  "text survives a restart" bullet, actually demonstrated.
- ✅ Writes are debounced on a 500ms flush timer and never happen on the keystroke path
  — a keystroke updates the doc and paints, and the snapshot is written later
  (Hard Constraint 2). Every edit appends to `revision_history` immediately, which is
  cheap, and the full snapshot is what gets debounced.
- ✅ Position, size, tone and pin are all tracked into `WindowPlacement` and persisted.
- ✅ `--dump` prints every widget with its text, revision count, position, tone and pin
  state. Kept in — it is how persistence gets verified without needing eyes on a window.
- ✅ Closing a note window calls `set_open(false)` and preserves the widget and document.

**Known gap this creates, fixed by the home window (S1-23):** if you close the only note
and relaunch, `to_open` is empty, no window opens, and the event loop exits immediately.
The app currently has no surface that is not a note. This is exactly why the home window
is the next task and not a later one.
