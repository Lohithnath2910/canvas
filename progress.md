# Canvas — Progress Log

**Last updated:** 2026-08-25

---

## 🟢 GO — the exact next action

*If you have just been told "go", "continue", or anything equally short, start here.
This block is rewritten at the end of every unit of work, so it is always current.*

**Next code task: `S1-28` — first-launch empty state.** ([TASKS.md](TASKS.md), section 1h.)
Home with zero notes shows `NOTHING HERE YET` plus a line pointing at the create control,
centred, matching the note's own empty state (§8). Small. `home.slint` already has the
rows list and the `NEW NOTE` control to hang it off; delete `%APPDATA%/Canvas` to see the
empty case.

**Ask the user, in the same message you start work — do not wait for an answer:**

1. **The tray (S1-27) and the body scrollbar (S1-46)** — both written on 2026-08-25 and
   neither watched working. The `Tray` block and the scrollbar line in the VERIFICATION
   QUEUE at the top of [TASKS.md](TASKS.md) say exactly what to click.
2. **Confirm the resize grab zone (S1-06).** Rebuilt twice on 2026-08-25 and still never
   watched working. The user's second report was that the zone starts *outside* the note
   so the cursor has to sit in empty space; `Layout.resize-border` went 30px → 36px in
   response, which puts 12px of it inside the visible plate instead of 6px. Check: the
   visible plate edge and all four corners grab **without** having to move outside the
   note; live resizing is smooth rather than only updating on release; and the window
   still responds to clicks and drags afterwards. If the feel is still off, the number is
   the only thing to change — `Layout.resize-border` in `theme/layout.slint`, one line.
3. **The rest of the VERIFICATION QUEUE** — about 23 behaviours now, all written and
   compiling, none watched.

**Do not stall waiting on any of it.** Build S1-28 while they check.

**Two open questions block work that is coming up soon**, and both should be asked once
the current unit is reported rather than saved for later: what pin should do beyond the
desktop layer, and — before Stage W starts — whether the Spreadsheet supports formulas and
whether its rows and columns insert mid-sheet. That second one decides the CRDT shape and
cannot be retrofitted. All of them are in **Open questions** at the bottom of
[TASKS.md](TASKS.md).

**The rule that keeps this working:** finish a task → tick its box in `TASKS.md` → move
the **Current task** line → rewrite this GO block → append a dated entry below → commit
with the task ID in the subject. Every task, not every session.

---

**Build order changed 2026-08-25** — finish Stage 1, then build every widget type
single-player (Stage W), then the backend and sync. Rich text is dropped. See the
override block at the top of [TASKS.md](TASKS.md).

---

## START HERE — state of the project

Canvas is a native offline-first desktop widget platform in Rust. The full spec is
[CLAUDE.md](CLAUDE.md); the numbered backlog is [TASKS.md](TASKS.md); this file records
what is actually true. Read all three before writing code.

### What exists and runs

`cargo run -p canvas-app` opens a **home window** listing every note, plus a window for
each note flagged open. Notes are frameless, transparent-margined, rounded plates with a
dot-matrix glyph strip, an editable title, and four header controls. Text persists to
SQLite and survives a restart. 28 tests pass.

```
Canvas/
  CLAUDE.md          the spec (working copy; handoff/ is the pristine original)
  TASKS.md           numbered backlog — START AT THE FIRST UNTICKED BOX
  progress.md        this file
  docs/glyph-strip.md   what the seven strip states mean
  Cargo.toml         workspace; every dependency version pinned here only
  crates/
    canvas-app/      runtime: App owns the Store + live windows. app.rs is the core.
    canvas-ui/       Slint UI. ui/theme/* is the design system, ui/*.slint the windows.
    canvas-doc/      CRDT documents (yrs). 12 tests.
    canvas-storage/  SQLite. 16 tests.
    canvas-sync/     empty stub. Nothing here yet.
  handoff/           pristine spec + fonts + design screenshots. Do not edit.
```

### Commands

```bash
cargo run -p canvas-app              # run it
cargo run -p canvas-app -- --dump    # print every note's text/position/state from disk
cargo test --workspace               # 28 tests
cargo clippy --workspace --all-targets   # must be clean before ticking any task
cargo run -p canvas-app --features gpu -- --gpu   # GPU is opt-in twice, never default
```

Database lives at `%APPDATA%/Canvas/canvas_local.db`. Delete that folder for a clean slate.

### Ground rules that are NOT in the spec — carried over from this session

1. **Do not write code comments.** The user asked for none, anywhere. All reasoning goes
   in this file, TASKS.md, and git commit messages instead.
2. **Only tick a TASKS.md box for behaviour you actually watched work.** Code that
   compiles is `🟡` here and the box stays open. This has already been got wrong twice —
   S1-06 was ticked because a property was set, not because resizing worked.
3. **Ask before adding a dependency** (Hard Constraint 6). Current additions beyond the
   spec's closed stack, all signed off: `windows-sys`, `raw-window-handle`.
4. Verify against the pinned versions rather than memory. `slint 1.17.1` and `yrs 0.19.2`
   are exactly what §11/§13 were written against, so those code samples apply verbatim.

### Decisions that OVERRIDE the spec

The user made these after seeing the app run. They win over CLAUDE.md.

| Spec says | Actually built | Why |
|---|---|---|
| §6.2 glyph strip 3×3 cells, 23×3 total | **8×8 cells, 4px gaps, 56×8** | Unreadable at spec size on a real screen |
| §7.1 header 16px above content, 0 below | **8px above, 8px below** (band still 48px) | Spec version looked top-heavy; body box unchanged |
| §7.6 state 1 two-line empty state | **One 25%-opacity placeholder line, caret on it** | Second line pushed the caret down a row |
| §5.2 body prose in the system sans | **Body prose in Silkscreen** | User found the sans "too generic"; `Typography.body-face` reverts it |
| §7.1 three header controls | **Four: colour · pin · delete · close** | No way to put a note away without deleting it |
| §8 widgets always-on-top | **Never always-on-top.** Pin = glued to desktop layer | Floating over every app was intrusive |
| §8/§9 bare desktop + ⌘K palette only | **A home window too**, plus the palette | Notes need a place to be created and managed from |
| §12 `window_positions` columns | **Plus `pinned`, `tone`, `open`** | `open` is what makes closing ≠ deleting |
| §11.1 whole-string replace per keystroke | **Real prefix/suffix diffing** | Spec's own comment says to do this |
| §7.2/§7.7 rich text in the note body | **Dropped. Plain text, one face.** | Silkscreen has no bold or italic and the user does not want them |
| §14 stage order: sync (2–3) before widgets (4–5) | **All widget types first, backend and sync after** | User: get every widget working single-player before multiplayer |
| §10 lists five widget types | **Plus a Spreadsheet** — not in the spec at all | User asked for it 2026-08-25; no artboard, no §10 section, so it carries real open questions |
| §7.3 "no visible scrollbar track" | **A visible scrollbar in the note body** — thumb only, no track | User asked for one directly; wheel scrolling alone gave no sense of position |

### Traps already hit — do not repeat these

1. **yrs indexes text by UTF-8 BYTE offset, not character offset.** `OffsetKind::Bytes`
   is the 0.19 default (`doc.rs:795`). Character indices work perfectly for ASCII and
   silently corrupt on the first multi-byte character. Any code indexing a `yrs::Text`
   must use byte offsets walked to a `char_boundary`.
2. **A layout element fills a plain `Rectangle` parent too.** §13.4 says to wrap a
   `HorizontalLayout` in a `Rectangle` to stop a *layout* parent stretching it — that is
   not sufficient. The glyph strip rendered 23×32 instead of 23×3 and parked itself in
   the plate's corner. Give it an explicit width AND height.
3. **`resize-border-width` DOES work here — the earlier entry saying otherwise was
   wrong.** It puts the grab band at the *window* edge, and the window edge is 24px out in
   the transparent `plate-margin`, so at the 6px it was set to the band sat entirely in
   empty space. The fix is one number: `Layout.resize-border` is 30px = the 24px margin
   plus 6px, so the band spans the whole invisible margin and ends 6px inside the visible
   plate edge — exactly the grab area the eight grips were drawing by hand.
4. **NEVER call `SendMessageW(WM_NCLBUTTONDOWN)` from a Slint callback.** It is a
   *blocking* call: Windows runs its entire modal resize loop inside it, and that loop is
   nested inside the Slint event loop that dispatched the callback. Two symptoms, one
   cause — the UI cannot repaint for the whole drag (so the size only appears to change
   when you let go), and the modal loop swallows the mouse-up, leaving the `TouchArea`
   latched pressed forever, which is what made the window ignore every later click.
5. **Never `std-widgets` `TextEdit`.** The compiler defaults to the `fluent` style and
   Fluent's `TextEdit` bakes in a blue focus bar with no way to override it. Bare
   `TextInput` in a `Flickable` everywhere.
6. **Two `TouchArea`s must never share a screen region** (§13.3, an open Slint bug).
   The header is partitioned by absolute position. The title is editable via
   `double-clicked` on the *existing* drag area for exactly this reason.
7. **Disk writes must not happen on the keystroke path.** Updates buffer in
   `Note::pending_updates` and are flushed by a 500ms timer, and on close/delete/quit.

### Known gaps

- 🟡 **A tray icon now owns the app lifetime** (S1-27) — home can be closed without
  stranding the app, and quit is a tray menu entry. Written 2026-08-25, **not yet watched
  working**; if the icon turns out not to appear, `maybe_quit` in `main.rs` is the one
  place that decides whether the app can still be quit by closing its last window.
- `canvas-sync` is an empty stub. No networking exists, so the strip's syncing / offline
  / peers / error states are only reachable through the Ctrl+G dev cycler.
- **No rich text, and there will not be any** — dropped 2026-08-25. The body is plain text
  in one face. Silkscreen ships Regular only, and the user does not want bold or italic.
- Only sticky notes exist. Todo, kanban, calendar and document are types in the storage
  layer with correct default sizes and row glyphs, but **no doc types and no windows**,
  and `app.rs:50` will not even open a non-note row. That is Stage W, and it starts by
  generalising the runtime (SW-01) before any widget gets built.
- **The Whiteboard has no artboard.** §10.5 is one paragraph and `widget-types.png` does
  not cover it. Ask before SW-12 rather than inventing a tool dock.

---

### 2026-08-25 — Roadmap reordered; rich text dropped

Two user decisions, both taken after reviewing what was left in the backlog. No code
changed in this pass — this is a planning entry only.

**1. Rich text is dropped (S1-38 … S1-42).**
- The body face is Silkscreen, which ships Regular only. Rather than revert the body face
  to the system sans to get bold and italic, the user chose to drop the formats.
- The note body stays a single plain-text flow in one face. **This closes open question 2**,
  which had been blocking S1-39.
- Nothing else depended on it: `StickyNoteDoc` already does real prefix/suffix diffing
  (S1-15), which was the one thing rich text would have needed from the doc layer.
- **S1-43 (undo/redo) was re-based** off S1-42 onto S1-19, and now targets the plain-text
  doc. Its "correct after a remote edit" DoD bullet is explicitly unreachable until
  Stage 3 and is recorded as deferred rather than quietly dropped.
- Reinstating rich text later means reverting `Typography.body-face` first.

**2. Build order: all widgets before any multiplayer.**
- §14 puts the backend at Stage 2 and sync at Stage 3, with the other widget types not
  arriving until Stages 4 and 5. The user reversed that: *"all these widgets etc should be
  functioning correctly first and then only we can add the multiplayer stuff."*
- `TASKS.md` now carries a **Stage W** — Todo, Shared Document, Calendar, Kanban and
  Whiteboard, all single-player — broken into 13 numbered tasks (SW-01 … SW-13), sitting
  between the end of Stage 1 and the old Stage 2.
- **S1-36 (sync/offline/error strip states) and S1-37 (presence) are deferred out of
  Stage 1**, since they are the only two tasks in it that genuinely need a peer. Deferred,
  not cancelled — they are listed to return immediately after Stage 3, along with the
  Shared Document's collaborator cursors and the remote-edit half of S1-43.

**What Stage W actually costs, measured against the code rather than guessed:**
- ❌ `canvas-doc` has `StickyNoteDoc` only. No todo, calendar, kanban or whiteboard doc.
- ❌ No `todo.slint`, `kanban.slint`, `calendar.slint` or `document.slint` exists.
- ❌ `app.rs:50` filters boot to `KIND_STICKY_NOTE`, and `Note` is a concrete struct
  bound to `StickyNoteWindow`/`StickyNoteDoc`. **A second type cannot open at all until
  that is generalised** — SW-01, and the reason the stage starts there.
- ❌ Home has one create control, `NEW NOTE`, and one `create-note()` callback.
- ❌ `controls.slint` has `IconButton` and `Button` only. **Checkbox, Tag, TextField and
  Divider are all unbuilt**, and Todo needs two of them.
- ✅ Already done and reusable: all five type constants and their correct default sizes in
  `canvas-storage` (with a test proving `documents` holds two types side by side), and
  the per-type row glyphs in `home.slint`, which already pick correctly by kind.
- ✅ §11.2's array-of-maps code for the todo doc is verified-compiling against yrs 0.19.2
  and can be used as written.

**Gap worth flagging before SW-12:** there is **no artboard for the Whiteboard.**
`widget-types.png` covers Todo, Shared Document, Kanban and Calendar only; §10.5 is a
single paragraph. Tool dock contents, the shape set and the stroke style are all
unspecified. Logged as open question 4.

### 2026-08-25 — Resize rebuilt on the native path; easing warning gone

**The resize bug the user reported was two symptoms of one cause, and my own.**
Reported: resize "works, but not smooth — I drag and after some time I see it affected the
size", and then "once I try to resize I can't click or drag anything, some sort of very
weird bug."

- ❌ **Cause:** `begin_native_resize` called `SendMessageW(hwnd, WM_NCLBUTTONDOWN, ...)`
  from inside a Slint `pointer-event` callback. `SendMessageW` **blocks** — Windows runs
  its whole modal resize loop inside that call, nested inside the Slint event loop that
  dispatched the callback. So the UI could not repaint for the duration of the drag (hence
  "not smooth"), and the modal loop consumed the mouse-up, so the grip's `TouchArea` never
  saw `PointerEventKind.up` and stayed latched pressed — which is what made the window
  ignore every subsequent click and drag.
- ✅ **Fixed by deleting the mechanism, not patching it.** `resize-border-width` was
  already set on both windows — at **6px**, which is why the earlier log entry concluded it
  "cannot work here". It was not wrong about the symptom: the band sits at the *window*
  edge, and the window edge is 24px out in the transparent `plate-margin`, so a 6px band
  lands entirely in empty space. **The fix was the number, not the mechanism** —
  `Layout.resize-border` is now 30px (24px margin + 6px), so the band covers the whole
  invisible margin and ends 6px inside the visible plate edge, which is precisely the grab
  area the eight grips were reproducing by hand.
- ✅ **Deleted:** `theme/chrome.slint` entirely (`ResizeGrips`, `HitCode`, 8 `Grip`
  `TouchArea`s), the `resize-begin` callback on both windows, both `on_resize_begin`
  handlers in `app.rs`, and `begin_native_resize` plus its `ReleaseCapture` /
  `SendMessageW` / `WM_NCLBUTTONDOWN` imports from `desktop_layer.rs`. Resize is now
  winit's own hit-test path, with no Canvas code in it at all.
- 🟡 **NOT yet watched working.** It compiles, clippy is clean, and the app launches — none
  of which is evidence, per ground rule 2. **The user has to confirm three things:** that
  the visible plate edge and corners still grab, that live resizing is now smooth, and that
  the window still responds to clicks and drags afterwards. S1-06 stays unticked until then.
  If the transparent margin turns out not to be hit-testable, the fallback is a
  `WM_NCHITTEST` window subclass returning `HTLEFT`/`HTTOPLEFT`/etc — still native, still
  non-blocking, just more code.

**Compiler warning the user asked about — fixed.**
- ❌ `out property <easing> standard` on the exported `Motion` global made Slint warn
  *"Properties of type easing are not supported yet for public API"* on every build.
  Harmless (it only means Rust cannot read that property), but noisy.
- ✅ Making it private is not an option — Slint requires `in`/`out`/`in-out` on a global
  property for *any* other .slint file to read it, so private breaks all four call sites.
  Instead the curve moved to its own `Ease` global in the same file, which `main.slint`
  does **not** re-export. Slint only generates public API for globals exported from the
  entry file, so `Ease` is invisible to Rust and the warning is gone, while `Motion` stays
  exported because S1-44 will need to set `Motion.reduced` from Rust. Still one easing
  token, still one place to change it (§5.5).
- ✅ Also fixed a pre-existing clippy warning in `app.rs:118` (`unnecessary to_string`)
  that had crept in with the home window work. `cargo clippy --workspace --all-targets`
  is clean again.

**Backlog: the Spreadsheet was added to Stage W (SW-13, SW-14).**
- The user asked for "an excel sheet kinda widget". It is **not in CLAUDE.md anywhere** —
  no §10 subsection, and `widget-types.png` does not cover it — so unlike the other five
  types there is no design to build against.
- What the existing language already settles without inventing a token: grid lines are 1px
  `edge`, headers are interface-face-11 `dim`, cells are body-face-14, and **the selected
  cell is a 1px `text` outline with no fill** — the same answer §10.3 already gives for the
  Kanban drop target, since rules 1 and 2 leave no accent colour to select with.
- What genuinely cannot be guessed, now logged as open questions 5, 6 and 7: whether
  formulas are in scope (a parser, a dependency graph, and a rule that formula *source* is
  synced while the computed value is derived locally — never synced, or two peers can
  disagree about a cell neither typed), whether rows and columns can be inserted mid-sheet
  (which decides the CRDT shape and cannot be retrofitted), and what its default size is.
- Stage W is now six types and 15 tasks. Nothing was started — this is backlog only.

### 2026-08-25 — Resize grab zone widened; one wrong diagnosis recorded

User's follow-up after the resize rebuild: *"the expanding thing is slightly starting from
outside the note and not properly from the edge, hence have to keep my cursor completely
out of the note and have to adjust it."*

**A wrong theory, checked and discarded before it shipped.** `resize-border-width` is a
logical `length`, but `i-slint-backend-winit-1.17.1/drag_resize_window.rs:48` compares it
against a **physical** cursor position with no scale factor applied. That predicts exactly
the reported symptom on a scaled display — at 150% a 30px band would be 20 logical px and
would end 4px *outside* the 24px margin. It was wrong: a temporary print of
`window().scale_factor()` reported **1**, so scaling was not the cause here.

**The actual cause, from reading `get_resize_direction`:** the band is measured inward
from the **window** edge, and the plate is inset by `Layout.plate-margin` (24px). A 30px
band therefore lands 24px in empty margin and only **6px inside the visible plate** —
the same thin 6px the deleted edge grips had, except the deleted *corner* grips were 16px
and the uniform band gives corners only 6px too. So the reliable part of the target is the
invisible margin, which is exactly what the user described.

- ✅ `Layout.resize-border` 30px → **36px**: 24px outside the plate plus **12px inside**,
  so the zone straddles the visible edge instead of sitting almost entirely beyond it.
  12px is the most that can be taken without reaching header content — the header band
  starts at plate `y=8`, and the icons sit at `y=16..32`, untouched. On the other three
  sides 12px stays inside the plate's own 16px padding, so it covers nothing.
- Worth knowing: **resize wins over UI content.** `event_loop.rs:435` returns early from
  `MouseInput` when a resize direction is live, before the press reaches Slint. So the top
  few pixels of the header resize rather than drag — which is how a real title bar behaves,
  not a regression.
- 🟡 **Kept, but unverified:** the border is now multiplied by `scale_factor()` after
  `show()` on both windows, via a new `resize-border` property on each. At scale 1 this
  is a no-op, which is why it could not be confirmed here — but the physical-vs-logical
  mismatch in the Slint source above is real, so a 125%/150% display would otherwise get a
  band that misses the plate entirely. Re-check this the first time Canvas runs on a scaled
  monitor. It is not re-applied when a window moves between monitors of different scale.
- 🟡 **Still not watched working.** S1-06 stays unticked. This is the second attempt at the
  same task and both were reported as compiling-and-plausible; the box does not get ticked
  until the user says the grab feels right.

## Standing update rule

After implementing and testing anything, append a dated entry below with the correct
status marker, and downgrade or remove anything it contradicts. Do not batch updates —
a stale log is worse than no log.

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

### 2026-08-24 — Home window (S1-23 built; S1-24/25/26/28 written, unverified)

- ✅ **`HomeWindow` exists and opens alongside the notes.** Same Panel, same 32px header,
  same tokens as a note — it reads as part of the same product, not a separate app.
  Title `CANVAS`, glyph strip lit when any note is open.
- ✅ **Boot order changed:** the app opens every note whose `open` flag is set, then
  always opens home. This closes the gap logged in the previous entry — the app can no
  longer start with no window and exit immediately.
- ✅ `maybe_quit` now also checks whether home is visible, so closing the last note
  leaves Canvas running with home on screen instead of quitting.
- 🟡 **Written and compiling, but NOT yet watched work:** the note list rows, clicking a
  row to open/close, NEW NOTE, per-row delete with its confirm, the row's open/closed
  dot indicator, and the empty state. Boxes stay unticked until they are clicked.
- ✅ Six new icons added as real vector paths: `plus`, `note`, `check`, `columns`,
  `calendar`, `doc`, plus `search`, `settings` and `enter` for the palette (S1-31).
  Widget type is shown by its own glyph on each row, which is what will make todo,
  kanban and calendar rows legible later without further work.
- Memory with home plus one note open: 35.4 MB working set (debug build).

### 2026-08-24 — Rename, close, and the saving path (user review)

Three issues raised after using the app. All three were real.

**1. Notes could not be renamed — every note was `UNTITLED` forever.**
- ✅ **Double-click the header to rename.** The title becomes a `TextInput` in a recessed
  field, text pre-selected; Enter or Esc returns to the body. Every keystroke commits, so
  clicking away never loses the edit. An empty title falls back to `UNTITLED`.
- Double-click rather than single-click deliberately: the header is also the drag region,
  and giving the title its own `TouchArea` would put two of them in the same screen
  region, which is the §13.3 failure mode. `double-clicked` on the existing drag
  `TouchArea` avoids that entirely — one touch region, two gestures.

**2. Notes could only be deleted, never closed.**
- ✅ **A close control (`x`) is now the rightmost header icon.** Cluster is
  colour · pin · delete · close, 88px wide. Close hides the window and clears the `open`
  flag; the note keeps its text, position, tone and pin, and reopens from home unchanged.
- The data layer already supported this (S1-21) — there was simply no way to reach it
  from the UI, which is exactly the gap reported.

**3. "The saving part is not so clean" — correct, and it was my bug.**
- ❌ **Was:** every single keystroke performed a synchronous SQLite INSERT into
  `revision_history`. The previous entry in this log claims "nothing touches disk on the
  keystroke path" — **that claim was wrong**, and only the snapshot was actually
  debounced.
- ✅ **Now:** updates accumulate in an in-memory `pending_updates` buffer and are written
  in a batch by the 500ms flush timer, alongside the snapshot. A keystroke now touches no
  disk at all. Buffered updates are also drained on close, on delete and on quit, so
  nothing is dropped.
- This is what Hard Constraint 2 actually asks for. The earlier claim should not have
  been written without checking the write path.

### 2026-08-24 — Native resize, home close (user review)

**Resize was broken, and the cause was structural.**
- ❌ `resize-border-width: 6px` puts the grab zone at the **window** edge — which sits
  24px out in the transparent `plate-margin`, over empty space. Dragging the visible
  plate edge did nothing, because the plate edge is not the window edge. S1-06 was
  ticked on the strength of the property being set, which was not evidence.
- ✅ **Replaced with eight grips on the plate's own perimeter** — four 16px corners and
  four 6px edges, in `theme/chrome.slint`. Each is a disjoint `TouchArea` (§13.3) and
  each carries the correct resize cursor.
- ✅ **The grips hand the drag to Windows** rather than tracking the pointer:
  `ReleaseCapture()` then `SendMessage(hwnd, WM_NCLBUTTONDOWN, HT*)`. Windows runs its
  own resize loop from there, so it behaves exactly like a normal window — live sizing,
  correct cursors, Aero Snap, and `min-width`/`min-height` still enforced by Slint. No
  manual pointer maths, and nothing to get wrong at DPI scaling.
- The header drag region is now inset to x=16, y=6 so it cannot overlap the top and
  corner grips. Two `TouchArea`s must never share a region.

**Home could not be closed.**
- ✅ Home now has its own `x` control, always visible (it is not a note; there is no
  hover-reveal to hide it behind). It hides the window and leaves notes running.
- ✅ Home also gets the same eight resize grips.

**Note titles were clipped.**
- The title elides because the header is a fixed 32px with an 88px control cluster; at
  the 320px default width the title box is ~120px, and Silkscreen at 22px is wide.
  Resizing now works, so the window can be widened — which is what the user asked for
  rather than a type change. Double-clicking the header shows the full title in an
  editable field regardless of window width.

### 2026-08-25 — Body scrollbar (S1-46) and the tray icon (S1-27)

**A visible scrollbar in the note body — this overrides §7.3.**
The user asked for one directly: wheel scrolling already worked, but with nothing drawn
there was no sense of how long the note was or where in it you were. §7.3's "no visible
scrollbar track" is now row 13 of the override table.

- 🟡 `ScrollBar` in `theme/controls.slint`, built as a reusable component rather than
  inline in the note, because every Stage W widget that scrolls will want the same one.
  A 4px thumb, radius 2px (`Layout.radius-checkbox` — the same answer §5.4 already gives
  for the checkbox, since 8px on a 4px bar is a capsule), `dim` at rest stepping to `text`
  on hover or press over `Motion.hover`. **No track is drawn** — that half of §7.3 stands.
  Opacity goes to 0 when the content fits, so a short note looks exactly as it did.
- 🟡 The thumb is draggable, not just an indicator. `viewport-y` is two-way bound
  (`<=>`) to the `Flickable`'s own, so the wheel moves the thumb and the thumb moves the
  wheel position with no second source of truth. The drag maps travel to overflow through
  `overflow / travel`, guarded so a zero-length track cannot divide by zero.
- **Why the gutter is permanent, not conditional.** The bar lives in an 8px gutter that
  the body reserves whether or not it is scrolling. Two reasons, both real: a `TouchArea`
  over the `TextInput` would be the exact overlap §13.3 says is unreliable, and a gutter
  that appears only on overflow narrows the text, which can *create* the overflow that
  justifies the gutter — the classic reflow oscillation. Reserving it always costs 8px of
  body width and settles both.
- New numbers, all off the existing 8px scale rather than invented: `scroll-gutter: 8px`,
  `scroll-thumb-w: 4px`, `scroll-thumb-min: 24px` in `theme/layout.slint`.

**Tray icon and app lifetime (S1-27).**

- ✅ Slint 1.17.1 has a real `SystemTrayIcon` element behind its `system-tray` feature —
  confirmed by reading `i-slint-compiler-1.17.1/builtins.slint` and
  `i-slint-core-1.17.1/items/system_tray/windows.rs`, which is a `Shell_NotifyIconW`
  wrapper. The feature is re-enabled in the workspace `Cargo.toml`. It pulls in `ksni`
  (the Linux D-Bus tray) transitively; that is cfg-gated to unix and never builds here.
- 🟡 `ui/tray.slint` — `CanvasTray inherits SystemTrayIcon`. **A `SystemTrayIcon` is a
  top-level component like `Window`, not something you put inside one**, and it must have
  exactly one `Menu` child, not inside an `if` or a `for`. Menu: `Open Canvas` (disabled
  and relabelled `Canvas is open` while home is up), `New note`, separator, `Quit Canvas`.
- 🟡 **The built-in `clicked` callback is not exposed to Rust unless you bind it in the
  `.slint` file.** `tray.on_clicked(...)` does not compile — the generated component has
  no such method. Bind `clicked => { root.open-home(); }` inside `CanvasTray` and go
  through your own callback instead. Cost one compile error to find.
- 🟡 `assets/icons/tray.png` — 64×64, five 10px white squares with 3px gaps on a
  transparent ground: the glyph strip, which is the one mark this product actually has.
  White reads on the default dark Windows 11 taskbar and will read poorly on a light one;
  worth a second look if the user runs light mode.
- 🟡 `App::install_tray` / `sync_tray` / `show_home` / `quit` in `app.rs`, plus a
  `tray: Option<CanvasTray>` on `App`. **`maybe_quit` now returns early whenever a tray
  exists** — that single check is what makes quit tray-only. If the tray fails to
  construct (a Linux desktop with no StatusNotifier host), it logs and falls back to the
  old rule where closing the last window quits, so the app can never become unquittable.
- ✅ Launched and left running for 25s: `cargo run -p canvas-app` starts clean, prints
  only `canvas: renderer = winit-software`, and logs **neither** a tray failure nor an
  image-decode error — so `CanvasTray::new()` and `tray.show()` both succeeded and the
  embedded PNG decoded. That is evidence the icon was *registered*, not that it is
  *visible* in the notification area (Windows can put a new icon in the overflow tray).
- ✅ `cargo clippy --workspace --all-targets` clean, `cargo fmt --all` applied, 28 tests
  still pass.

**Neither task is ticked.** Both are behaviour I have not watched, per ground rule 2.
`TASKS.md`'s verification queue has a new **Tray** block and a scrollbar line with exactly
what to click.
