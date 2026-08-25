# Canvas — Progress Log

**Last updated:** 2026-08-26

---

## 🟢 GO — read this first, then keep going

*If you have just been told "go", "continue", or anything equally short: this block is the
whole instruction. It is rewritten at the end of every task, so it is always current.*

### What "go" means

**Work the backlog continuously.** Not one task and a check-in — task after task, until
the backlog runs out or one of the stop conditions below is true. The user said "go"
once and that covers the whole run.

Each cycle, in order:

1. **Next task** — named below, and on the **Current task** line in [TASKS.md](TASKS.md).
2. **Build that one task only.** Read its entry in `TASKS.md` and the CLAUDE.md sections
   it points at. Don't chain two, don't tidy next door.
3. **`cargo clippy --workspace --all-targets` and `cargo test --workspace` clean**, then
   `cargo fmt --all`.
4. **Check DO NOT REGRESS below.** It is the floor. If a task genuinely has to change
   something on it, stop and ask — that is one of the few things worth interrupting for.
5. **Log it, in this order:** tick the box in `TASKS.md` (or leave it open with a 🟡 note
   — see the next paragraph), move **Current task**, rewrite this GO block so it names
   the *new* next action, append a dated entry below, `git commit` with the task ID in
   the subject.
6. **Go straight to the next task.** No summary between tasks, no asking what's next.

**You cannot see the screen** (CLAUDE.md §13.7), so most tasks cannot be honestly ticked
by you. That is not a reason to stop. Anything needing eyes lands as: code committed,
`🟡` entry here, **box left unticked**, and a new line in the VERIFICATION QUEUE in
`TASKS.md` saying exactly what to click. Then carry on. The user ticks it later.

**Stop at 90% of the Claude usage limit**, whatever the backlog says — user instruction,
2026-08-25, and it is also in CLAUDE.md's standing rules. Do not start another task past
that point. Finish the one in hand to a committed, logged state — tick or 🟡 the box, move
**Current task**, rewrite this block, append the entry below, commit — then stop and say
you stopped on the limit rather than on the backlog. A half-built task with nothing written
down is exactly what this protocol exists to prevent. Only an explicit "keep going"
overrides it.

**Stop only when** one of these is true, and say which: nothing unticked is left that can
be done without the user · the next task is blocked on an **Open question** and no
unblocked task remains · step 3 or 4 failed twice on the same task. Then report once:
task IDs closed, what is 🟡 waiting on the user, and any question that blocked you.

### The next task

**The backlog has no unblocked task left. The run stopped on that, not on a limit.**

**`SW-05` — the Todo widget window — is next and is BLOCKED on open question 8**: *do the
other widget types get the note's four header controls (colour · pin · delete · close)?*
Three of the four are window behaviours and obviously belong on any type. The open one is
**colour-as-grey-tone**. Question 8's own wording is *"decide once, apply to all types"*,
so guessing it costs one `IconButton` on six widget types, not one.

**Everything else in Stage W is behind the same question**: SW-06 (Shared Document), SW-08
(Calendar window), SW-10 (Kanban window), SW-12 (Whiteboard window — also behind question
4, which is that no artboard for it exists). SW-13 and SW-14 are behind questions 5, 6
and 7.

**Stage 1 ran out earlier and separately:** `S1-45` needs the VERIFICATION QUEUE walked by
the user, `S1-48` needs open question 9 (a schema change), `S1-36`/`S1-37` need a peer.

### The moment question 8 is answered

Go straight to `SW-05`. Everything it needs is already built and tested:
`TodoListDoc` with `counts() -> (open, done)` (SW-04), `Checkbox` and `Tag` in
`theme/controls.slint` (SW-03), and SW-01's `WidgetDoc`/`WidgetWindow` enums where a new
type is one arm each and the compiler names every `match` that needs it. Then `SW-02`
comes off the shelf — it was skipped because a create-control-per-type strip is pointless
while the note is the only type with a window — and SW-01's DoD line about opening a
non-note window can finally be ticked.

### Read this before assuming the widgets work

**Four CRDT documents were built this run — Todo, Calendar, Kanban, Whiteboard — and none
of them is wired to a window.** They are library code: 122 tests pass, and not one of them
is reachable from `cargo run -p canvas-app`. That was the right work while blocked, because
it is exactly the half that does not need eyes. Do not read the test count as a working
widget.

**The VERIFICATION QUEUE is now the only thing that moves the project forward**, and it has
a section per task. From this run: undo/redo (S1-43), reduced motion (S1-44), the home
scrollbar (S1-47), the SW-01 regression check, and the four §6 components via
`cargo run -p canvas-app -- --gallery` (SW-03). Underneath: the palette, the global hotkey,
the open/close animation, the screen clamp and the drag feel. All written, all clippy-clean,
none watched working.

### Questions worth raising in the end-of-run report — but never worth stalling for

- **Before Stage W starts:** does the Spreadsheet support formulas, and can rows and
  columns be inserted mid-sheet? The second decides the CRDT shape and cannot be
  retrofitted, so SW-13 is genuinely blocked on it. Everything before SW-13 is not.
- What pin should do beyond holding a note at the desktop layer.
- The Whiteboard has no artboard (open question 4) — SW-12 needs one.

All of them are in **Open questions** at the bottom of [TASKS.md](TASKS.md).

---

## 🔒 DO NOT REGRESS

Everything here has been watched working by the user. A new task may build on it; no task
may quietly break it. If you touch one of these files, re-check the matching line.

| Works | Where it lives | What breaks it |
|---|---|---|
| Notes resize from every edge and corner | `Layout.resize-border` = **36px**, `theme/layout.slint` | Lowering the number — at 6px and 30px the grab zone sat in empty space. And **never** re-introduce `SendMessageW(WM_NCLBUTTONDOWN)`; see trap 4 |
| Tray icon owns the app lifetime; quit is tray-only | `ui/tray.slint`, `App::install_tray`, `maybe_quit` in `main.rs` | Removing `maybe_quit`'s early return when a tray exists — that is the only thing keeping the app alive with no windows |
| Body scrollbar: thumb tracks the wheel and drags | `ScrollBar` in `theme/controls.slint`, wired in `sticky-note.slint` | Making the 8px gutter conditional on overflow (reflow oscillation), or putting a `TouchArea` over the `TextInput` (§13.3) |
| Text survives a restart; notes reopen where they were | `canvas-storage`, `App::flush_all`, the 500ms flush timer | Writing to disk on the keystroke path — updates buffer in `Note::pending_updates` (trap 7) |
| Typing never stutters | the same buffering | Any I/O, network or disk, in an `edited =>` handler (Hard Constraint 2) |
| 122 tests pass, clippy clean | `cargo test --workspace`, `cargo clippy --workspace --all-targets` | Landing anything without running both |
| Every user override in the table below | see **Decisions that OVERRIDE the spec** | Following CLAUDE.md literally where the table disagrees with it |

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
SQLite and survives a restart. There is a **command palette** over a full-screen scrim,
opened by an OS-global Ctrl+K (Ctrl+N creates a note) — **built but never watched
working**, with a note-body Ctrl+K left in as a fallback. 41 tests pass. Ctrl+Z/Ctrl+Y undo the note body, rebuilt from `revision_history` on open.

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
    canvas-doc/      CRDT documents (yrs). StickyNoteDoc, UndoHistory, TodoListDoc,
                     CalendarDoc, KanbanDoc, WhiteboardDoc + a hand-written fractional
                     index. 103 tests. The last four have NO window yet.
    canvas-storage/  SQLite. 16 tests.
    canvas-sync/     empty stub. Nothing here yet.
  handoff/           pristine spec + fonts + design screenshots. Do not edit.
```

### Commands

```bash
cargo run -p canvas-app              # run it
cargo run -p canvas-app -- --dump    # print every note's text/position/state from disk
cargo run -p canvas-app -- --gallery # the section 6 component gallery (dev only)
cargo run -p canvas-app --example insert_todo   # a non-note row; --remove undoes it
cargo test --workspace               # 30 tests
cargo clippy --workspace --all-targets   # must be clean before ticking any task
cargo run -p canvas-app --features gpu -- --gpu   # GPU is opt-in twice, never default
```

Database lives at `%APPDATA%/Canvas/canvas_local.db`. Delete that folder for a clean slate.

### Ground rules that are NOT in the spec — carried over from this session

1. **Do not write code comments.** The user asked for none, anywhere. All reasoning goes
   in this file, TASKS.md, and git commit messages instead.
2. **Only tick a TASKS.md box for behaviour you actually watched work.** Code that
   compiles is `🟡` here and the box stays open. This has already been got wrong twice —
   S1-06 was ticked because a property was set, not because resizing worked. It is not a
   reason to stop working: log it 🟡, add the click-through to the VERIFICATION QUEUE,
   and move to the next task.
3. **Ask before adding a dependency** (Hard Constraint 6). Current additions beyond the
   spec's closed stack, all signed off: `windows-sys`, `raw-window-handle`. `windows-sys`
   features enabled so far: `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`,
   `Win32_UI_Input_KeyboardAndMouse`, `Win32_Graphics_Gdi` — features are not new
   dependencies (TASKS.md rule 5), but they are listed here so the surface stays visible.
4. Verify against the pinned versions rather than memory. `slint 1.17.1` and `yrs 0.19.2`
   are exactly what §11/§13 were written against, so those code samples apply verbatim.
5. **"go" means keep going.** One task, log it, commit, next task — no check-in in
   between. The full protocol is the 🟢 GO block at the top of this file.

### Decisions that OVERRIDE the spec

The user made these after seeing the app run. They win over CLAUDE.md.

| Spec says | Actually built | Why |
|---|---|---|
| §6.2 glyph strip 3×3 cells, 23×3 total | **8×8 cells, 4px gaps, 56×8** | Unreadable at spec size on a real screen |
| §7.1 header 16px above content, 0 below | **8px above, 8px below** (band still 48px) | Spec version looked top-heavy; body box unchanged |
| §7.6 state 1 two-line empty state | **One 25%-opacity placeholder line, caret on it** | Second line pushed the caret down a row |
| §5.2 body prose in the system sans | **Body prose in Silkscreen** | User found the sans "too generic"; `Typography.body-face` reverts it |
| §7.1 three header controls | **Four: colour · pin · delete · close**, then **three: pin · delete · close** from 2026-08-26 | Close was added because there was no way to put a note away without deleting it. Colour was then dropped by the user when open question 8 was answered — three controls on every widget type, the note included. `Palette.plate-tone` and the `tone` column stay, so an already-tinted note still renders; there is just no control. |
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

- `canvas-sync` is an empty stub. No networking exists, so the strip's syncing / offline
  / peers / error states are only reachable through the Ctrl+G dev cycler.
- **No rich text, and there will not be any** — dropped 2026-08-25. The body is plain text
  in one face. Silkscreen ships Regular only, and the user does not want bold or italic.
- **Only the sticky note is a usable widget. Four other CRDT documents exist with no window
  attached.** `TodoListDoc`, `CalendarDoc`, `KanbanDoc` and `WhiteboardDoc` are built and
  covered by 103 tests in `canvas-doc`, and **not one of them is reachable from the running
  app** — there is no window, no create control and no entry in `WidgetDoc`/`WidgetWindow`
  for them. Do not read the test count as a working widget. The runtime *can* now dispatch
  by type (SW-01) and says so on the console when a type has no window yet; what is missing
  is the windows, and **every one of them is blocked on open question 8**.
- **The four §6 components (`Checkbox`, `Tag`, `Divider`, `TextField`) exist but nothing
  uses them.** They render only in `cargo run -p canvas-app -- --gallery`, a dev window
  built so SW-03's by-eye DoD could be checked at all. The first real user is the Todo
  window.
- **The Whiteboard has no artboard.** §10.5 is one paragraph and `widget-types.png` does
  not cover it. Ask before SW-12 rather than inventing a tool dock.
- **Only the note body has a scrollbar.** The home list still scrolls bare; giving it one
  means narrowing the rows by the 8px gutter, which moves every row's delete icon. Logged
  as S1-47 rather than done on the way past.
- **The tray icon is white on transparent**, which reads on the default dark Windows 11
  taskbar and will read poorly on a light one. Worth revisiting if the user runs light.

---


---


---


---


---


---


---


---


---


---


---

### 2026-08-26 — SW-16, SW-05, SW-02: the second widget type, and colour dropped

**Open question 8 was answered by the user: three header controls on every type —
`pin · delete · close` — and colour is dropped from the sticky note as well.** They chose
that knowing it regresses a control they had previously confirmed working, which is the
explicit word the DO NOT REGRESS floor requires. The override table records it.

- 🟡 **SW-16 · the colour control is gone.** `Layout.cluster-w` is **64px**, not 88 — the
  drag region, the title width and the title editor all derive from that one number, so
  nothing else moved. `Palette.plate-tone` and the `tone` column in `window_positions`
  are **kept on purpose**: a note that already carries a tone still renders tinted, there
  is simply no control to change it, and deleting them would have needed a migration to
  undo. `IconPalette` stays in the icon library — it is one of §5.6's sixteen glyphs.
- ✅ **SW-05 · a Todo widget exists and persists** — confirmed 2026-08-26 headlessly:
  `cargo run -p canvas-app --example insert_todo` seeds a todo with three items (one
  done); the app opens it in its own window with nothing on stderr; `--dump` reads all
  three items back afterwards. `--dump` is now type-aware and reports any widget type
  through `WidgetDoc`.
- ✅ **SW-01's remaining DoD half closed.** "A non-note row opens its own window" is now
  true, which it could not be until a second window type existed.
- 🟡 **Adding, toggling, editing and deleting items has not been driven by hand.** Needs a
  pointer. In the VERIFICATION QUEUE.

**The enum dispatch did exactly what it was chosen for.** Adding Todo meant one arm per
`match` in `widget.rs`, and the compiler listed every site that needed one — including
`apply_placement`, `scale_resize_border` and `weak_open`, which are easy to forget. No
site was found by reading.

**Two decisions in the Todo window that §10.1 does not make:**

- **A compose row sits below the list** — a `plus` glyph and a `TextInput`, Enter adds and
  clears. §10.1 says nothing about how items get created. A modal or a floating button
  would both break the flat-plate language; an always-present input line is the smallest
  thing that works, and it puts the caret where a new note already puts it.
- **Double-click a row to rename it**, the same gesture as the note's title and for the
  same reason (trap 6): the row's *existing* `TouchArea` carries it rather than a second
  one overlapping. `editing-id` lives on the window so exactly one row can be in edit
  mode. Each row's `TouchArea` also stops 40px short of the right edge so it never shares
  a region with that row's delete button.

**Counts never touch a Slint binding.** `TodoListDoc::counts()` returns `(open, done)` in
Rust and both arrive as `in property<int>` — §13.6, where an imperative loop in a binding
is a parse error, not a runtime limit.

**SW-02 came off the shelf** now that there is a second type: `NEW TODO` beside `NEW NOTE`
in home's footer, the palette's `New todo` action live instead of printing "todo widgets
do not exist yet", the empty state reworded, and the footer counting `N WIDGETS` rather
than `N NOTES`. **Two labelled buttons will not scale to seven types** — that is the next
thing to fix, and the user said so.

### 2026-08-26 — SW-11: `WhiteboardDoc`, and the run stops on open question 8

- ✅ **`canvas-doc/src/whiteboard.rs`, 17 tests** — verified by `cargo test --workspace`,
  122 tests pass, clippy clean.

**Points are stored flat: `x, y, x, y, …` in one `yrs::Array` per stroke.** Not a map per
point, and not a nested array per point. This is the decision the DoD's whole traffic
requirement rests on — a sample is then two array appends and nothing else, while a map per
point would carry the key strings `"x"` and `"y"` for every sample of a freehand line, on
the wire and in the document. Shapes are a `yrs::Map` keyed by shape id (the same
by-construction reasoning as `KanbanDoc`'s cards), strokes a `yrs::Map` keyed by stroke id
with the point array inside each.

**`begin_stroke` then `extend_stroke(id, &[Point])` is the API a pointer-move handler
wants.** A stroke opens once and grows; nothing ever re-sends what is already drawn. A
whole-canvas resend is not merely discouraged by this shape — there is no method that
would do it.

**The DoD's claim was split into two tests, because it is two claims.** "Proportional to
the points added" and "not proportional to the canvas" can each pass while the other fails.
`appending_1000_points_costs_traffic_proportional_to_the_points_added` grows a stroke to
1000 points, then checks one more point costs under a twentieth of the whole snapshot and
that ten points cost under twelve times one.
`stroke_traffic_does_not_grow_with_the_canvas` draws the *same* ten points on an empty
board and on one carrying 200 shapes and 4000 other stroke points, and checks the cost
barely moves while the snapshot is more than twenty times either.

---

## The run stopped here, and why

**Every remaining task in Stage W is a widget *window*, and all of them are blocked on
open question 8** — *do the other widget types get the note's four header controls?* Its
own wording is *"Blocks: SW-05 onward — decide once, apply to all types, rather than per
widget."* SW-05, SW-06, SW-08, SW-10 and SW-12 are all behind it; SW-12 is behind question
4 as well (no Whiteboard artboard), and SW-13/SW-14 behind questions 5, 6 and 7.

**Stage 1 ran out first, for different reasons:** `S1-45` (sign-off audit) needs the
VERIFICATION QUEUE walked by the user, `S1-48` needs the new open question 9 answered (it
is a schema change), and `S1-36`/`S1-37` need a second machine.

**What this means for the shape of what was built.** The four CRDT documents from this run
— Todo, Calendar, Kanban, Whiteboard — are **library code with no window attached**. They
are fully tested and they are the half that cannot be checked by eye, so building them
while blocked was the right use of the time. But they are **not reachable from the running
app**, and they do not become so until question 8 is answered and the window tasks start.
A session picking this up cold should not mistake 122 passing tests for a working
whiteboard.

**What *is* wired into the running app from this run:** S1-43 (undo/redo on Ctrl+Z/Ctrl+Y),
S1-44 (reduced motion), S1-47 (the home scrollbar) and SW-01 (the type-dispatched runtime).
All four are 🟡 — none has been watched working.

### 2026-08-25 — SW-09: `KanbanDoc`, and a fractional index written by hand

- ✅ **`canvas-doc/src/fractional.rs`, 8 tests, and `canvas-doc/src/kanban.rs`, 22 tests**
  — verified by `cargo test --workspace`, 105 tests. The DoD's two named cases both pass,
  and five more concurrency cases with them.

**The nested-array failure mode is avoided by construction, not by care.** Cards live in a
**`yrs::Map` keyed by card id**. A card is therefore *one map entry* — there is no array
slot to reorder, so there is no operation that could duplicate or drop it, whatever two
peers do at once. Its column and its position are ordinary fields on that entry, and two
concurrent moves resolve the way any two concurrent field writes resolve: one column wins,
consistently on every peer, with the card still existing exactly once. The test asserts
precisely that, including that an untouched bystander card did not move. Columns stay a
`yrs::Array` of `{id, title}` — they are not reordered by dragging, so an array is honest
there.

**The fractional index is hand-written, because adding a crate needs the user's word**
(Hard Constraint 6) and the algorithm is about sixty lines. Digits are base-36, `0-9a-z`,
chosen because that set sorts lexicographically in the same order as its values in ASCII —
so plain string comparison *is* position comparison, which is what makes the `pos` field
sortable straight out of the document with no parsing.

Three things about it worth not re-deriving:

- **A key is the fraction after an implied decimal point**, always strictly inside
  `0 < k < 1`. The published versions of this algorithm carry an integer part to grow in
  both directions; that is unnecessary here, because you can always subdivide toward 0 or
  toward 1. `prepending_always_shrinks` and `appending_always_grows` each run 500 times.
- **No key may end in `0`.** That is the invariant the whole midpoint recursion rests on:
  a trailing zero makes "the next key down" unrepresentable. `key_between` returns `None`
  rather than a bad key if it is ever handed one, and there is a test for it.
- **`key_between` returns `Option`, not a key with a fallback.** An inverted range is a
  caller bug, and silently returning something plausible would put cards in the wrong order
  in a way no test could see. `move_card` computes its own neighbours so it cannot invert.

**`move_card(id, column, index)` is deliberately the API a drop gives you** — a column and
a slot — not a pair of keys. It resolves the neighbours in the destination itself,
excluding the card being moved, so SW-10 never touches an index key. `many_moves_into_the_
same_gap_stay_ordered` drops 200 cards into the single gap between the same two neighbours
and checks the order strictly holds; that is the case that makes a naive midpoint scheme
collapse.

**Two fields that are not speculative.** `assignee` and `due` are on the card because
§10.3's card meta line is literally `ASSIGNEE · DATE` (or `UNASSIGNED`). `due` is epoch
milliseconds as `Any::BigInt`, the same choice and for the same reason as `CalendarDoc`.

### 2026-08-25 — SW-05 blocked on open question 8; SW-07 `CalendarDoc` done instead

**SW-05, the Todo window, is the first task the loop could not take**, and the reason is
recorded rather than guessed around. **Open question 8** — *do the other widget types get
the note's four header controls?* — says in its own words *"Blocks: SW-05 onward — decide
once, apply to all types, rather than per widget."* Three of the four are obvious on any
type (pin, delete, close are window behaviours, not note behaviours). The fourth,
**colour-as-grey-tone**, is genuinely undecided, and a previous session deliberately
reserved it. Guessing costs one `IconButton` and would then be wrong on five more types.

So the loop moved to the **doc-layer tasks, which nothing blocks**: SW-07 (`CalendarDoc`),
SW-09 (`KanbanDoc`) and SW-11 (`WhiteboardDoc`) are pure CRDT work, fully specified in §11.3
and §10.3/§10.5, and testable without eyes. Every remaining *window* task in Stage W —
SW-05, SW-06, SW-08, SW-10, SW-12 — is behind the same question 8, and SW-12 is behind
question 4 as well.

- ✅ **`CalendarDoc` in `canvas-doc/src/calendar.rs`, 18 tests** — verified by
  `cargo test --workspace`, 75 tests. Convergence covered three ways, including a
  **concurrent move against a rename of the same event**, where both survive.

**Four choices §11.3 does not make, made here and worth not re-deriving:**

- **`start` and `end` are epoch milliseconds stored as `Any::BigInt`, not `Any::Number`.**
  `Any::Number` is an `f64`; a real millisecond timestamp is around 1.75×10¹², which f64
  holds exactly today but is the kind of thing that rots silently. `BigInt` is an `i64` and
  matches `canvas_storage::now_millis()`, so the two layers agree. `int_field` still accepts
  a `Number` if one ever appears in a document, so a foreign or hand-edited file degrades
  instead of reading back 0.
- **`events()` returns sorted by `(start, id)`.** SW-08 lays out seven day columns and
  would otherwise sort on every render; the id tie-break keeps two events starting at the
  same minute in the same order on every peer, which a sort by `start` alone would not.
- **`events_between` selects anything *overlapping* the window**, not merely contained by
  it. A meeting that starts before midnight and ends after it belongs to both days, and the
  contained-only version would drop it from the day you are looking at.
- **`add_event` and `set_span` clamp `end` up to `start`.** A negative span would make
  SW-08's duration-driven chip height negative, and a widget cannot render its way out of
  bad data. There is a test for it.

### 2026-08-25 — SW-04: `TodoListDoc`

- ✅ **`canvas-doc/src/todo.rs`, 16 tests, all passing** — verified by
  `cargo test --workspace` (57 tests now). Convergence is covered three ways rather than
  the one the DoD asks for: concurrent **adds** (nothing lost), a concurrent **toggle
  against a rename of the same item** (both survive, which is the case a
  last-write-wins merge would break), and a concurrent **delete against an edit of a
  sibling**. Snapshot round-trip, idempotent re-application, `diff_since` minimality and
  garbage-bytes-without-a-panic all mirror `StickyNoteDoc`'s set.

**§11.2's pattern was used as written** — `yrs::Array` of `yrs::Map`, `{id, text, done}`,
no `due` or `assignee` built on spec. Three things were added that its sketch does not
cover, each for a reason that already exists elsewhere in this codebase:

- **`set_text` and `set_done` return `None` when nothing changed.** `StickyNoteDoc::set_text`
  already does this, and it matters more here than it looks: a no-op that still returned an
  update would write a `revision_history` row, which S1-43's undo replay would then read as
  a real edit step. Undo would gain phantom stops.
- **`counts()` returns `(open, done)`.** §13.6 is explicit that a Slint binding cannot run
  an imperative loop — it is a parse error, not a runtime limit — so the footer's
  `N OPEN · N DONE` has to be computed in Rust and passed in as `in property<int>`.
- **`add_item` returns the new id with the update.** Otherwise the caller has to re-read
  the array to find the row it just created, which is a second transaction for information
  we already had.

**Trap 1 — yrs indexing by UTF-8 byte offset — does not bite yet, and it is worth being
clear why rather than assuming it is handled.** An item's text is replaced wholesale with
`map.insert(&mut txn, "text", …)`; nothing indexes into it, so there is no offset to get
wrong. The unicode test passes because there is no arithmetic to fail. **The trap goes
live the moment SW-05 does per-keystroke diffing inside an item**, the way
`StickyNoteDoc::set_text` does — at which point `common_affixes`' `char_boundary` walk is
the thing to reuse, not to re-derive.

**The whole-value replace is a real trade-off, not an oversight.** Two peers editing the
*same* item's text concurrently will have one of them win the field rather than merging
character by character — but that is a `Map` field write, which is still a CRDT
resolution, not last-write-wins across the document, and Hard Constraint 3 holds. Todo item
text is a short label, not prose. If that turns out to be wrong, the fix is a `yrs::Text`
per item rather than an `Any::String`, and it changes the data shape — so it is a question
for the user, not a later refactor.

### 2026-08-25 — SW-03: the four missing §6 components, and a gallery to look at them in

- 🟡 **`Checkbox`, `Tag`, `Divider` and `TextField` in `theme/controls.slint`.**
  Clippy-clean, 41 tests pass, and `--gallery` opens and stays up — but **nothing in it
  has been looked at**, and every one of these is a by-eye component. In the VERIFICATION
  QUEUE, item by item against §6's own tables.
- ✅ **`cargo run -p canvas-app -- --gallery` opens a window and survives** — verified
  2026-08-25 by launching the built binary and confirming the process was still alive
  after five seconds with nothing on stderr.

**A `TouchArea` forwards pointer events to its own children. Trap 6 does not apply to a
parent wrapping its child.** This is worth having straight, because it decides how every
remaining widget in Stage W builds a hoverable input. `TextField` needs a `dim` border on
hover (§6.6) and hover only exists on a `TouchArea` — but putting one over a `TextInput`
looked like exactly the overlap §13.3 warns about. Reading
`i-slint-core-1.17.1/items/input_items.rs`, `TouchArea::input_event_filter_before_children`
returns **`ForwardAndInterceptGrab`**: it records hover and the pointer position, then
forwards the event down, claiming the grab only if no child takes it. Trap 6 is about two
**sibling** areas covering the same region, where §13.3's ordering bug lives. So the
`TextInput` sits *inside* the hover `TouchArea` and keeps its own click-to-place-caret.
**That specific behaviour is the first thing to check in the queue** — if the caret only
ever lands at the end of the text, this reasoning was wrong and the fix is a ring of
disjoint areas around the frame instead.

**Note `enabled: false` on a `TouchArea` also clears `has_hover`** (same file, the
`!self.enabled()` branch sets it false and returns `ForwardAndIgnore`), so a disabled
field cannot show a hover border. That is correct behaviour here and it is why
`TextField`'s disabled state fades the whole control instead.

**The DoD said "rendered against its §6 table by eye" and there was nowhere to look**, so
`--gallery` is part of this task rather than scope creep. It is a dev-only window — the
same class of thing as `--dump` and the Ctrl+G state cycler, not product UI — and it shows
all four components in every state §6 lists, including states no real widget will reach for
a while: `Tag` active, `TextField` error, `TextField` disabled. Waiting for SW-05 to
exercise them would have verified about half the surface.

**Every number came from §6.** `checkbox-size` 16, `checkbox-gap` 12, `checkbox-row-h` 22,
`tag-h` 20, `tag-pad-x` 10, `field-h` 40, `field-pad-x` 12, `field-gap` 8 — all stated in
§6.4–§6.6, now in `theme/layout.slint` rather than inlined (rule 7). Two things that read
as mistakes and are not: the checkbox radius is **2px**, because 8px on a 16×16 box is a
disc; and there is **no tick glyph**, because the fill going white *is* the state, which is
the same language the dot cells speak.

**One deliberate departure from §6.4's sample code in CLAUDE.md.** Its strikethrough spans
`parent.width - 28px` — the whole rest of the row, well past the end of the text. Ours
spans `min(label-text.preferred-width, label-text.width)`, so it stops where the text
stops. §6.4 says the *label* is struck through, and a rule running into empty space reads
as a divider, not a strike.

**`SW-02` was skipped rather than blocked.** Its DoD is "each type creatable from home …
opens its own window", and the sticky note is still the only type with a window. Building
the control strip now would add one button that already exists plus buttons that print
"no window yet". Its runtime half already landed with SW-01 (`App::create_widget(kind)`).
It comes back after SW-05.

### 2026-08-25 — SW-01: the runtime stops being a sticky-note runtime

- ✅ **A non-note row no longer vanishes at boot** — verified 2026-08-25 by
  `cargo run -p canvas-app --example insert_todo` and reading the boot log:
  `canvas: 0a7f7e83… is a "todo" widget and that type has no document yet (Stage W)`.
  Before this, `App::boot` filtered the list to `KIND_STICKY_NOTE` and the row was dropped
  without a word.
- 🟡 **That sticky notes still behave identically has not been watched.** This is a rename
  and a re-shape of every note code path; clippy is clean and all 41 tests pass, but
  nothing about a note was *meant* to change, so the whole Note-window block of the
  VERIFICATION QUEUE is now also SW-01's regression check.

**Enums, not trait objects, and the reason matters for every type added after this.**
The obvious shape is `Box<dyn WidgetWindow>`. It does not work: `slint::ComponentHandle`
carries an associated `Inner` type, so it is not object-safe, and a hand-rolled object-safe
trait would still need one impl per window type — the same code, with the compiler no
longer able to tell you where the gaps are. `WidgetDoc` and `WidgetWindow` are therefore
plain enums with one arm each today. Adding Todo is one arm per `match`, and the compiler
names every site that has to learn about it. `WidgetWindow` gets a hand-written `Clone`
because Slint handles clone through `clone_strong()`, not `Clone`.

**The shared state is shared exactly once.** `LiveWidget` owns the placement, the pending
update buffer, both dirty flags, the undo history and the poll timer, and none of that is
inside the enums. So the 500ms flush timer, `flush_all`, the drag anchor, the screen clamp
and S1-43's checkpointing keep working for **any** type without being written again per
type — which was the specific thing the task said not to get wrong.

**Two things generalised on the way past, both of which were hardcoded to notes:**

- `App::boot` no longer filters `list_widgets()` by kind. Every open row is handed to
  `open_widget`, which dispatches on `widget_type` and says plainly when a type has no doc
  or no window yet instead of pretending nothing was there.
- The palette's result rows used to hardcode `icon: "note"`. They now pass the row's real
  `widget_type`, and `PaletteGlyph` in `palette.slint` maps all five kinds to their §5.6
  glyph at 24px — the same mapping `KindGlyph` already does at 16px in `home.slint`.
  **This renamed the icon key from `note` to `sticky_note`**, so if a palette result ever
  shows a blank square, that mapping is where to look.

**`create_note` is now a one-line wrapper over `create_widget(kind)`.** It is kept because
the tray, the home button and the palette's `new-note` action all mean *a note*
specifically — they are not generic create buttons yet. That is SW-02.

**The DoD is half-met on purpose.** "A non-note row opens its own window" cannot finish
until there is a second window type to open, which is SW-04. The dispatch that needs is in
place and the silent skip — the part that was actually a bug — is gone and confirmed.

### 2026-08-25 — S1-47: the home list gets a scrollbar, and does not get a gutter

- 🟡 **`ScrollBar` overlaid on the home list's right 8px.** Clippy-clean, 41 tests pass,
  the app launches with no runtime warning — but **three notes do not overflow a 520px
  list**, so nothing has been watched scrolling. In the VERIFICATION QUEUE.

**The task warned that the 8px gutter would move every row's delete icon inward. It
does — so the gutter was not reserved.** The note body has to reserve one: its
`TextInput` wraps, so a gutter that appeared only on overflow could re-wrap the text into
needing it and oscillate. That is the S1-46 reasoning and it still stands **there**. A
home row cannot oscillate: every text in it is `wrap: no-wrap; overflow: elide`, so there
is nothing to reflow and no reason to pay for a permanent gutter.

So the bar is **overlaid** on the rightmost 8px of the list area and the rows keep their
full width. The payoff is the alignment the task asked to be deliberate about: a row's
delete icon stays at `width - pad - icon-size`, which is exactly where the note window's
`close-btn` sits, so the two line up across windows.

**Overlaying does not break §13.3 here, and it is worth being precise about why** rather
than treating "two things in the same corner" as automatically wrong. The rule is that two
`TouchArea`s must not share a *screen region*. In a home row the region is already
carefully partitioned: the row's own `TouchArea` is `parent.width - 44px` wide, and the
delete `IconButton` spans `width-32 … width-16`. The scroll band is `width-8 … width`.
Nothing overlaps, and there is 8px of dead space between the icon and the bar. A
consequence worth knowing: hovering the thumb does **not** light the row behind it,
because that row's hover comes from a `TouchArea` that ends 44px short.

**The list and its bar now live in one conditional wrapper `Rectangle`.** The first
version made the `Flickable` unconditional with `visible: false` so the bar could reference
it by name across scopes — which would have left an invisible `Flickable` sitting exactly
on top of S1-28's empty state. Wrapping both in `if root.rows.length > 0:` keeps the
empty-state swap byte-for-byte what it was.

### 2026-08-25 — S1-44: reduced motion now reads the OS setting

- ✅ **The OS setting is read, and the polarity is right** — verified by running the built
  app twice and reading its own boot line. On this machine, with Windows animations at
  their default (on), it prints `canvas: reduced motion = false`; with
  `CANVAS_REDUCED_MOTION=1` it prints `true`.
- 🟡 **That the windows visibly stop travelling has not been watched.** In the
  VERIFICATION QUEUE, with the exact settings path to toggle.

**Most of this task was already done and the GO block was right about which half.**
`Motion.reduced` has existed in `theme/motion.slint` since the theme was built, and every
duration that needs to collapse already derives from it: `open` 160→100ms, `open-rise`
6→0px, `drag-shadow` 100→0ms, `presence` 80→0ms, and the glyph strip's sweep `Timer` is
gated on `!Motion.reduced` so the dots hold where they are. Nothing in the theme needed
changing. What did not exist was anything that ever set the flag to `true`.

**`SPI_GETCLIENTAREAANIMATION` answers the opposite question to the one you want.** It
returns TRUE when animations are **enabled**, so reduced motion is its inverse — get that
backwards and the app animates exactly when it should not, on a machine whose owner
specifically asked it not to. `reduced_motion::wanted()` returns
`ok != 0 && animations == 0`, and it is deliberately conservative: if the call fails it
reports `false` rather than silently killing every animation on a machine that never asked.

**A Slint global is per window, not per process.** `Motion` is instantiated once per
component tree, the same way `Typography` is — so setting `Motion.reduced` in one place
does nothing for the other windows. It is set at every one of the three creation sites
(home, note, palette), next to the existing `Typography` call.

**A mid-session toggle works without a relaunch**, which matters because that is how
anyone will actually test it: `refresh_reduced_motion` re-reads the setting inside the
500ms flush timer that already runs, and pushes it to home, the palette and every live
note only when the value has actually changed. `SystemParametersInfoW` for this parameter
is a cached read, so it costs nothing at that rate, and it is not on the keystroke path.

**`CANVAS_REDUCED_MOTION=1|0` overrides the OS**, which is what made the polarity check
above possible without touching the user's own accessibility settings. It is a testing
hook, not a preference — there is no settings screen yet to make it one.

**One thing reduced motion does not reach: `Flickable`'s own flick inertia.** Slint
animates a flick internally and does not expose a duration, so a hard flick in the note
body still glides. A wheel scroll is stepped and unaffected. Worth knowing before the
S1-45 audit calls it a miss.

### 2026-08-25 — S1-43: undo/redo, rebuilt from `revision_history`

- 🟡 **Ctrl+Z / Ctrl+Y / Ctrl+Shift+Z in the note body.** Clippy-clean, 41 tests pass
  (11 new). The model is unit-tested including the across-a-restart path; **the key
  bindings have not been watched working** and are in the VERIFICATION QUEUE.
- ✅ **`UndoHistory` replays stored revisions into undo states** — verified by
  `undo_survives_a_restart_through_stored_revisions` and nine sibling tests in
  `canvas-doc`.
- ✅ **A culled history refuses to offer wrong text** — verified by
  `a_trimmed_history_yields_no_restored_undo_rather_than_wrong_text`.

**`yrs::UndoManager` exists in 0.19.2 and is the wrong tool here. Do not reach for it.**
It builds its stack items from transactions it *observes*, through an `after_transaction`
subscription. There is no way to seed it. That kills it for this task, because the whole
point of "backed by `revision_history`" is undo that survives a restart:

- Apply the note's snapshot first, then replay its revisions — every replayed update is
  already in the doc, so each apply is a **no-op**, no transaction event fires, and the
  stack comes back empty.
- Replay the revisions first and apply the snapshot after — the tail updates depend on
  items the doc has not seen, so yrs parks them in its **pending** queue and integrates
  nothing until the snapshot lands. Same empty stack, plus a window where the text is
  wrong.

**What was built instead.** `UndoHistory` in `canvas-doc` holds a list of text states and
a cursor. On open, `from_revisions` replays the note's `revision_history` rows into a
scratch `StickyNoteDoc` and captures the text at each **flush batch boundary** — a
`created_at` gap of more than 100ms. That threshold is not arbitrary: revisions written by
one flush all get `now_millis()` within a millisecond or two of each other, and
consecutive flushes are 500ms apart, so the boundary the replay finds is exactly the
boundary the live session recorded. One flush = one undo step, on disk and in memory, by
construction. That is also why undo is not per-keystroke: a keystroke is not an undo unit,
half a second of typing is.

**Undo is still a real CRDT edit.** `step_history` walks the cursor, then hands the target
text to `StickyNoteDoc::set_text`, which prefix/suffix-diffs it (S1-15) and emits a proper
yrs update that joins `pending_updates` like any other edit. Nothing anywhere does
last-write-wins, and Hard Constraint 3 holds.

**The trimmed-history guard.** `trim_revisions` culls the oldest rows once a note passes
500 of them, so a replay from empty would silently reconstruct the *wrong* text. The
replay therefore checks its final state against the live document, and on a mismatch
throws the whole restored stack away and starts from the current text. Undo then covers
this session only. That is the honest failure and it is loud in the tests. Fixing it
properly means storing a compaction base alongside the cull — a schema change, so it needs
the user's word (rule 6) and is not worth it until a note actually reaches 500 revisions.

**Two behaviours worth knowing before judging it broken:**

- An undo is itself an edit, so it is appended to `revision_history` like anything else.
  After a **restart**, Ctrl+Y will not redo an undo made in a previous session — but
  Ctrl+Z will, because undoing the undo restores it. Within a session redo is normal.
- Checkpointing runs in the existing 500ms flush, never on the keystroke path, so trap 7
  and Hard Constraint 2 are untouched. `checkpoint` no-ops when the text already equals
  the state at the cursor, which is what stops an undo from being recorded as a new step.

**The Stage 1 DoD bullet "undo works correctly after a remote edit arrives mid-session"
is still unreachable** — there is no sync layer to deliver a remote edit. It stays
deferred, not ticked, exactly as the task said.

### 2026-08-25 — S1-49, S1-50, S1-51: three bugs the user hit while using the app

Off-backlog. The user reported drag stutter, RAM going from "4–5 MB" to "20 MB ish", and
the Ctrl+K palette landing to one side instead of centred. All three were real; two of
them turned out to be the same bug.

- ✅ **The palette window was built 1.5× too big — S1-49.** Verified by measurement, not
  by eye: read back with a **DPI-aware** `GetWindowRect`, the palette was
  **3840×2160 on a 2560×1440 screen**. Now `2560x1440 at (0,0)`.
- ✅ **The palette's framebuffer is given back on dismiss — S1-50.** Private bytes:
  open 41.4 → **25.0 MB**, dismissed 41.5 → **11.0 MB** against a boot figure of 10.9,
  and **9.95 MB** after a second open/close cycle. It used to keep every byte for the
  rest of the session.
- 🟡 **Drag anchors to the cursor instead of accumulating deltas — S1-51.** Clippy-clean
  and 30 tests pass, but **drag feel cannot be measured, only felt** — it is in the
  verification queue.

**The palette bug is a Slint DPI trap worth remembering.** `screen::primary_bounds()`
returns *physical* pixels. A Slint window that has not been `show()`n yet reports
`scale_factor() == 1.0` — the real 1.5 arrives later as an event-loop event. So
`set_size(PhysicalSize(2560,1440))` was passed to winit as "2560×1440 **logical**", winit
applied the 150% DPI, and the window came out 3840×2160. Everything inside then landed
1.5× too far out: the plate centres on `parent.width`, so it sat about 1440px across a
2560px screen — which is exactly the "off to the side" the user saw — and its top sat 1.5×
too low as well. `fill_screen()` now converts using `App::known_scale()`, the scale factor
of an **already open** window, and runs **before** `show()`.

**Do not "correct" the size again after `show()`.** That was tried and it reintroduced the
bug exactly: the post-show call reads the new window's own scale, which is still 1.0 at
that moment, and re-inflates the window. The debug line that proved it read
`scale=1 size=PhysicalSize { width: 2560, height: 1440 } known=1.5` — Slint's own idea of
the window was 2560 wide while Windows had made it 3840.

**The drag was running away, not stuttering in the render.** The old handler applied
`mouse-x - pressed-x` — a **window-relative** delta — to the window's own position on every
mouse-move, which feeds the window's movement back into the next delta. Measured with
synthesised input: the cursor stayed inside a 120px band and the window travelled ~970px to
the screen edge. `begin_drag()` now records `GetCursorPos()` minus the window origin once,
on `drag-started`, and `drag_move()` sets the position to `cursor − anchor` on every move.
Nothing accumulates, so nothing can drift. Two smaller things came off the drag path with
it: `set_position` is skipped when the result is unchanged, and `placement_dirty` is set on
`drag-ended` rather than on every mouse-move, so the 500ms flush cannot fire a SQLite write
mid-drag. `Note.placement` is still updated in memory every move, so `flush_all` on quit
still saves the right position even if the mouse-up is never seen.

**The RAM regression was smaller than it looked.** Release builds, measured:

| | private | working set |
|---|---|---|
| v1 milestone (`3627cd2`) — one note window, no DB, no home, no tray | 6.63 MB | 25.29 MB |
| current, at boot | 10.9 MB | 25.3 MB |

Boot is ~4 MB above the old build — that is SQLite, the home window (468×568 against the
note's 368×328) and the tray — and working set is unchanged. The 20 MB the user saw was
the palette. **`GetSystemMetrics` and `GetWindowRect` lie to a DPI-unaware process**, so
any future measurement of this kind has to call `SetProcessDpiAwarenessContext(-4)` first
or it will read every number back divided by 1.5.

**One thing found and not fixed: the app renders ~30–42 frames a second while completely
idle, every one of them with an empty dirty region.** Found via
`SLINT_DEBUG_PERFORMANCE=refresh_lazy,console`. Something is asking for a redraw ~35×/sec
with nothing to redraw. It is not the glyph strip's `Timer` (gated on `syncing`) and not
any `animate` in the tree — most likely the transparent/`no-frame` window defeating the
software renderer's partial rendering. Worth a task of its own; it is wasted CPU on every
widget and will be six times worse once Stage W has six window types open.

**Two Slint / Win32 debugging techniques that worked here, for next time:**
- `windows_subsystem = "windows"` hides the console, but **stderr still redirects**:
  `Start-Process -RedirectStandardError` captures `eprintln!` and `SLINT_DEBUG_PERFORMANCE`
  output from the release build without rebuilding it.
- `SendKeys("^k")` from PowerShell **does** fire a `RegisterHotKey` hotkey, so the S1-35
  global hotkey mechanism can be exercised headlessly. It fired both Ctrl+K and Ctrl+N from
  a non-Canvas foreground window — good evidence the hidden-message-window approach works,
  though the visual half is still the user's to confirm.

---

### 2026-08-25 — S1-35: a real global hotkey for the palette

- 🟡 **`RegisterHotKey` for Ctrl+K and Ctrl+N, OS-wide.** Written and clippy-clean.
  **Not watched working, and this one has a specific way it can fail** — see below.

**The mechanism, and where it might not hold.** `RegisterHotKey` posts `WM_HOTKEY` to the
registering thread's message queue. Slint's winit loop owns that queue, and winit has no
idea what `WM_HOTKEY` is — it would hand it to `DefWindowProc` and the key would vanish.
So `hotkey.rs` creates a **hidden `HWND_MESSAGE` window with its own wndproc** and
registers the hotkeys against that. winit's loop calls `DispatchMessageW` on everything in
the thread queue, and `DispatchMessageW` routes by `hwnd`, so our wndproc should be
called even though winit knows nothing about the window.

**"Should" is doing real work in that sentence.** If winit ever pumps with a *filtered*
`PeekMessage`, our window's messages never get dispatched and Ctrl+K silently does nothing
from outside the app. That is the single most likely failure, it is in the verification
queue as the first thing to check, and the fallback if it happens is to subclass one of
winit's own windows via `SetWindowLongPtrW(GWLP_WNDPROC)` instead. **The note-body Ctrl+K
from S1-32 was deliberately left in place** as the fallback path — when the global
registration succeeds Windows swallows Ctrl+K before the focused window sees it, so the
two cannot both fire.

**The palette takes the foreground explicitly.** A window that merely `show()`s while
another app is in front usually does not get keyboard focus on Windows, which would have
left the scrim up with no way to type in it or press Esc. `SetForegroundWindow` is
normally refused — but "the process owns the hotkey that was just pressed" is on Windows'
own list of cases where it is allowed, which is exactly the situation. New
`desktop_layer::take_foreground`.

The wndproc hands off through `slint::invoke_from_event_loop` rather than calling into
`App` directly, so nothing runs re-entrantly inside a message dispatch nested in Slint's
own loop.

**One feature was enabled, no crate added:** `WNDCLASSW` is gated behind `windows-sys`'s
`Win32_Graphics_Gdi` (its `hbrBackground` field is an `HBRUSH`). Rule 5 says enabling a
feature on a crate already in the closed stack is not a new dependency. Nothing from Gdi
is actually called.

If registration fails — most likely because another app already owns Ctrl+K — the app
prints a line saying so and saying the note-body fallback still works, rather than failing
silently.

---

### 2026-08-25 — S1-34: palette selection, footer, and note search

- 🟡 **The palette's rows are live.** ↑↓ move a highlight, ⏎ and click activate, the
  footer carries a result count, and a non-empty query searches note titles. Written and
  clippy-clean; **not watched working**, so the box stays open.
- ✅ **Two new unit tests** — `count_label` singular/plural/empty, and that the empty
  query offers exactly §9's three actions. 30 tests now, up from 28.

**Silkscreen cannot draw §9's footer legend, so it is not used.** Checked against the
font's own `cmap` table rather than assumed: `↑` (U+2191), `↓` (U+2193) and `⏎` (U+23CE)
are **all absent** from `Silkscreen-Regular.ttf`. `·` (U+00B7) is present. §9's
`↑↓ MOVE · ⏎ OPEN · ESC CLOSE` would therefore have rendered three tofu boxes. The footer
reads `UP/DOWN MOVE · ENTER OPEN · ESC CLOSE` instead — same meaning, same middle dots,
every glyph present. **This is a forced deviation from the spec, not a preference.**
Getting the symbols back means a second font for that one string, which is a call for the
user to make.

**Search is title-only, deliberately, and is logged as S1-48.** Matching note bodies the
obvious way is `get_document` + `StickyNoteDoc::load` for every note **on every
keystroke** — a disk read and a yrs decode per note per character. That is precisely the
shape of thing Hard Constraint 2 and trap 7 exist to keep off the typing path. It needs a
cheap searchable copy of the text (a `text` column on `widgets` maintained by the existing
500ms flush, or FTS5), and that is a **schema change, which rule 6 says to ask about**.
So: titles now, S1-48 for bodies.

Smaller decisions:
- **↑↓ wrap** at both ends. §9 does not say; wrapping is what every command palette does.
- **Hovering a row selects it**, so a click never activates a row other than the
  highlighted one. §9 only defines selection visually, so this is an addition — one
  `TouchArea` per row, which is safe because rows are disjoint from each other. The thing
  trap 6 forbids is one area spanning the plate, and there still isn't one.
- Selection is `elev-raised` fill plus the icon stepping to `text`, and nothing else —
  **no accent fill, no left bar** (§9, rule 2).
- `New todo` and `Settings` **print to stderr and do nothing**. There is no todo type
  until Stage W's SW-01 and no settings screen anywhere. Wiring them to something
  plausible would be worse than an obvious no-op.
- Keys live in `query-input`'s `key-pressed`, which is where focus is. `Key.UpArrow`,
  `Key.DownArrow` and `Key.Return` are intercepted before the `TextInput` sees them.

---

### 2026-08-25 — S1-33: palette input row and the three default actions

- 🟡 **The palette plate now has a search icon, a query line and three action rows.**
  Written and clippy-clean; **not watched working**, so the box stays open.

Straight from §9: 56px input row, `pad-doc` (24px) either side, a 24px `search` icon, a
12px gap to the query, the query in the interface face at 22px with a 2px caret, and
`TYPE A COMMAND` in the interface face at 11px `dim` when it is empty. The rows are 48px,
with a 24px icon at 24px in, a 16px gap, the label in the body face at 14px, and the
shortcut right-aligned in the interface face at 11px `dim`.

**The shortcuts read `CTRL+N` / `CTRL+T` / `CTRL+,`, not `⌘N` / `⌘T` / `⌘,`.** §9's
notation is macOS; this build is Windows and the keys S1-35 will actually register are
Ctrl. Showing a ⌘ that does nothing on this machine would be a lie in the UI.

**The rows are display-only.** Selection, ↑↓, ⏎, the footer and searching notes are all
S1-34, so a non-empty query currently yields an empty list rather than results. That is
the honest half-state, not an oversight — §9's own DoD for S1-33 is "opens showing the
three defaults; typing replaces them".

Two smaller decisions:
- The plate's height is computed from the row count
  (`palette-row-input + rows.length * palette-row-result`) rather than fixed, so it
  collapses to the bare 56px input row once a query empties the list. The footer will add
  its 32px in S1-34.
- The rows come from Rust (`default_actions()`), and so will the note search — §13.6 says
  to compute derived list data in Rust rather than reactively in Slint.

Three placement numbers went into `theme/layout.slint` rather than being inlined:
`query-x` (60px = 24 + 24 + 12), `result-label-x` (64px = 24 + 24 + 16), both read
straight off §9's own arithmetic, and `result-meta-w` (120px), which §9 does not give —
it is the right-hand column the meta is aligned in, and it is on the 8px scale.

Nothing on DO NOT REGRESS is touched. 28 tests, clippy clean.

---

### 2026-08-25 — S1-32: the command palette's overlay shell

- 🟡 **`ui/palette.slint` exists and can be opened.** A full-screen scrim with a 640px,
  28px-radius plate centred on it. Written and clippy-clean; **not watched working**, so
  the box stays open.

**What the palette is, as a window.** Every other window in the app is `no-frame` with a
24px transparent `plate-margin` and is sized to its own content. The palette cannot be:
§9 wants a `scrim` behind it covering what is on screen, so it has to be a window the size
of the screen with a transparent background, painting the scrim itself.

- `always-on-top: true`. This is **not** a contradiction of the user's "never
  always-on-top" override — that override is about *notes*, which were intrusive floating
  over every app. A command palette that another window can cover is simply broken.
- Sized and positioned to the **primary monitor** via a new
  `screen::primary_bounds()` (`SM_CXSCREEN`/`SM_CYSCREEN`), not the virtual desktop rect
  S1-30 uses. §9 says "centred horizontally, top at ~15% of screen height" — one screen.
  Spanning the scrim across every monitor would dim displays the user is not looking at.
  Opening on whichever monitor currently has focus would be better; that needs a
  per-monitor lookup and is not what this task asked for.
- The plate is the existing `Panel` with `corner-radius: Layout.radius-overlay` and
  `dragging: true`, which is how §9's drag-depth shadow is already expressed. Reusing
  `Panel` also gets the `plate` fill and 1px `edge` for free.

**No `TouchArea` anywhere in the palette.** Click-outside-to-dismiss is not in §9, and a
full-window `TouchArea` for it would sit underneath every result row S1-34 adds — exactly
trap 6, which Slint does not resolve by z-order. Dismissal is Esc, through a zero-size
`FocusScope` that takes focus in `init`, which is what §9's own footer legend says.

**The Ctrl+K that opens it is a stopgap and is S1-35's to replace.** It is bound in the
note body's `key-pressed`, next to the existing Ctrl+G, so it only fires while a note has
focus — not an OS-global hotkey. Without it the shell could not be put on screen at all,
which would have left S1-33 and S1-34 building blind on top of an unseen thing.

Five §9 numbers went into `theme/layout.slint` — `palette-w`, `palette-row-input`,
`palette-row-result`, `palette-footer-h`, `palette-top-fraction`. The last three are for
S1-33/S1-34; they are here because they come from the same paragraph of the spec.

`App::quit` now hides the palette along with everything else. Nothing on DO NOT REGRESS is
touched. 28 tests, clippy clean.

---

### 2026-08-25 — S1-31: icons can render at 24px

- 🟡 **`IconGlyph` takes a `size`, and everything inside it scales.** Written and
  clippy-clean. **Nothing renders at 24px yet**, because the palette does not exist — so
  this is verifiable only alongside S1-32, and the box stays open until then.

**All five palette glyphs already existed.** `search`, `plus`, `check`, `settings` and
`enter` were built in S1-05 along with the other eight, from §5.6's command table. So the
only thing S1-31 genuinely still owed was the **24px size** (§9 — a size that exists
nowhere else in the product).

**Slint does not scale `stroke-width` with the viewbox.** Worth recording, because it is
not what you would assume: `Path::fitted_path_events` (`i-slint-core-1.17.1/items/path.rs`)
fits the path *geometry* into the element bounds minus the stroke width, but passes
`stroke_width` through in logical pixels untouched. A `Path` at 24px with
`stroke-width: 1.5px` therefore draws a *thinner-looking* line than the same glyph at
16px, not a proportional one.

So `IconGlyph` now computes it: `size` in, `scale = size / Layout.icon-size`,
`stroke-w = Layout.icon-stroke * scale`. At 16px that is 1.5px — every existing call site
is byte-identical, since none of them pass a `size`. At 24px it is 2.25px.

**The assumption that needs the user's eye:** §5.6 says "1.5px stroke … drawn on a 16-unit
grid", which does not say outright whether the stroke scales at 24px. Scaling it is the
reading taken — the numbers in the command table are grid units and 1.5 is the same kind
of number, and an unscaled stroke would make the palette's icons visibly lighter than the
header's, which breaks a monoline set. If the user wants a flat 1.5px at both sizes, it is
one line in `IconGlyph`.

`IconSearch` was the only glyph that needed more than the stroke change — its circle is a
`Rectangle` with hardcoded px, not a `Path`, so its x/y/size/radius are now multiplied by
`scale` too. Two new tokens in `theme/layout.slint`, both straight out of §5.6/§9 rather
than invented: `icon-size-palette: 24px` and `icon-stroke: 1.5px`.

Nothing on DO NOT REGRESS is touched. 28 tests, clippy clean.

---

### 2026-08-25 — S1-30: off-screen clamping

- 🟡 **A note can no longer be dragged somewhere it cannot be grabbed back from.** 120px
  of plate width and the full 48px header band always stay on screen. Written and
  clippy-clean; **not watched working**, so the box stays open.

**Clamped against the virtual screen rect, not the nearest monitor's work area.** This is
the one real decision in the task. `MonitorFromPoint` + `GetMonitorInfoW` would give a
per-monitor clamp, and on a two-monitor desktop that clamp fires at the **internal** edge
between the two screens — the note would stop dead halfway across the join, which is
exactly the push-back §8 forbids. `SM_XVIRTUALSCREEN`/`SM_YVIRTUALSCREEN`/
`SM_CXVIRTUALSCREEN`/`SM_CYVIRTUALSCREEN` give the union bounding box of every monitor,
so adjacent screens are contiguous inside it and only the true outer boundary clamps.
The known cost: on monitors of different heights the union has dead corners a note could
still be parked in. That is a worse-looking edge case than a note that refuses to cross
between screens, so it is the trade taken.

New `crates/canvas-app/src/screen.rs` — `virtual_bounds() -> Option<Bounds>`, Win32 behind
`#[cfg(windows)]` and `None` everywhere else, so a non-Windows build simply does not
clamp. `GetSystemMetrics` is already in the `Win32_UI_WindowsAndMessaging` feature that
`desktop_layer.rs` uses, so **no dependency and no feature was added**.

`clamp_to_screen` works in physical pixels, which is what the drag path already deals in:

- The plate is inset from the window by the 24px transparent `plate-margin`, so every
  bound is written against the *plate* rect, not the window rect.
- `KEEP_VISIBLE_W` = 120px (§8). `KEEP_VISIBLE_H` = 48px = `Layout.header-band`, this
  build's actual header — §8 says "the full 32px header", but the user's override made the
  band 48px, and the override wins.
- The top bound is the plate's top edge, not a 120px column: the header cannot go above
  the screen top at all, since it is the only draggable part.

Applied in three places: both windows' `on_window_moved` (the drag), and again right after
`show()` in `open_note` / `open_home`. **That second call is what covers the display-change
case** — a note whose stored position is off every current monitor gets pulled back to the
nearest edge of the virtual desktop on the next launch, and the corrected position is
written back through `placement_dirty`. It also covers a monitor being unplugged while a
note is merely hidden, since a re-show goes through the same path.

The clamp is deliberately *not* applied on resize. Resizing from a top or left edge is
handled natively by Windows through `resize-border-width`, and reaching into that would
mean re-entering the mechanism trap 4 is about.

**Untestable here:** the unplug-a-monitor-and-relaunch case. Logged in the verification
queue as such rather than ticked.

Nothing on DO NOT REGRESS is touched. 28 tests, clippy clean.

---

### 2026-08-25 — S1-29: open and close animation

- 🟡 **The plate fades and rises on open, fades on close.** Open is 160ms, opacity 0→1
  plus a 6px rise; close is 100ms, opacity only. Written and clippy-clean; **not watched
  working**, so the box stays open and four lines went into the VERIFICATION QUEUE.

**The plate animates, not the OS window.** Both windows already sit inside a 24px
transparent `plate-margin`, so a 6px rise runs entirely inside that margin and never
clips at the window edge — and it costs no window-move calls, which matters because
`on_window_moved` is also the drag path. Two new properties on `StickyNoteWindow` and
`HomeWindow`:

- `opened` drives both `opacity` and `y` (`plate-margin + (opened ? 0px : open-rise)`).
- `closing` drives `opacity` only, so **the close is a pure fade with no travel** — the
  plate does not sink back down on the way out. `opened` deliberately stays true for the
  whole fade so `y` cannot move.
- The opacity animation picks its own duration: `closing ? Motion.close : Motion.open`.
  A binding on an `animate` duration is re-read when the animation starts, so one
  `animate opacity` block covers both directions at their two different speeds.

**Both flags are driven from Rust, not from a Slint `init` or an in-tree `Timer`.** The
home window is hidden and re-shown over and over by the tray, and an in-tree timer keyed
off `!opened` would have fired its open animation while the window was still hidden — the
user would then see it snap in fully lit. `play_open` therefore resets both flags, and a
16ms `Timer::single_shot` sets `opened` a frame later, so the first painted frame is
genuinely at opacity 0. `play_close` sets `closing`, then hides the window 100ms later
and resets the flags for the next show. A small `Fadeable` trait keeps one copy of each
for the two window types.

**Close now genuinely takes 100ms, so three call sites had to stop hiding instantly:**
`App::close_note` and `App::delete_note` hand their window to `play_close` instead of
`hide()`, and both windows' native `on_close_requested` returns
`CloseRequestResponse::KeepWindowShown` — `HideWindow` would have hidden the window
under the fade. The home path is now `App::hide_home`, which runs `sync_tray` and
`maybe_quit` **in the fade's completion callback**: during those 100ms the window is
still visible, so calling them immediately would have told the tray home was open.

`App::quit` still hides everything instantly and on purpose — the process is going away.

**Reduced motion (S1-44) mostly falls out of this**, as expected: `Motion.open` is
already 100ms and `Motion.open-rise` already 0px when `Motion.reduced` is set, so the
open collapses to the required 100ms crossfade with no travel. Nothing sets
`Motion.reduced` from the OS yet — that is still S1-44's job.

Nothing on DO NOT REGRESS is touched: resize border, tray lifetime, the body scrollbar
and the buffered-write path are all unchanged. 28 tests, clippy clean.

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

### 2026-08-25 — S1-06, S1-27, S1-46 confirmed · S1-28 built · the docs now run a loop

**Three tasks ticked on the user's word.** They reported *"resizing etc works"* covering
the three things the previous report asked them to click:

- ✅ **S1-06 — resize.** Third attempt, and the answer was one number:
  `Layout.resize-border` = 36px, being the 24px transparent `plate-margin` plus 12px
  inside the visible plate, so the grab zone straddles the plate edge instead of sitting
  beyond it. At 6px and at 30px it was almost entirely in empty space. This is now on the
  DO NOT REGRESS table; the number is not free to lower.
- ✅ **S1-27 — tray icon and app lifetime.**
- ✅ **S1-46 — the note body scrollbar.**

**S1-28 — first-launch empty state.** 🟡 built, needs eyes.

- The two lines existed already but were parked 48px below the header in a
  `VerticalLayout`, which is not what §8 asks for. They now sit in a plain `Rectangle`
  spanning the whole list region — hairline to footer rule — and are centred inside it by
  absolute `y`, with `empty-title.y` computed from its own height plus the 12px gap plus
  `empty-hint.height`. Absolute positioning rather than a layout, per §13.4.
- The rows `Flickable` became `if root.rows.length > 0:`. It is declared after the empty
  state, so with zero rows it was an invisible element sitting on top of the message and
  swallowing scroll events over it. Making it conditional is cheaper than reordering.
- ✅ Verified as far as is possible without eyes: `%APPDATA%/Canvas` was moved aside, the
  app launched genuinely cold into a fresh empty database, came up clean with no layout
  error and no panic, created the db, and stayed running — so the zero-rows branch is
  what rendered. The real database was then restored and `--dump` confirms the note is
  intact. **Whether it looks right is still the user's call.**

**The three markdown files now describe a loop, not a single unit of work.**
The user asked that "go" in a cold chat pick up the next task and *keep going* without
breaking what already works. Concretely:

- `progress.md`'s GO block was rewritten from "here is the next action" into **the
  operating instruction**: the cycle (build one task → clippy + tests → check the
  regression floor → tick or 🟡 → move Current task → rewrite GO → log → commit → next
  task), an explicit "no check-in between tasks", and three named stop conditions.
- **New: a 🔒 DO NOT REGRESS table** directly under it. Seven rows, each naming the
  behaviour, the file it lives in, and the specific thing that breaks it. This is the
  floor a task may build on but not quietly undo.
- `TASKS.md` gained a **The loop** section at the top of *How to use this file*, with the
  same cycle and the same stop conditions, plus the rule that makes the loop possible:
  a session cannot see the screen, so eyes-needed work is committed, logged 🟡, left
  unticked, and pushed onto the VERIFICATION QUEUE — and then the session **carries on**
  rather than waiting.
- `CLAUDE.md`'s "If the user just said go" block was rewritten to match, and its standing
  rules gained two lines: never regress the floor, and **§4.1's "stop and report after
  each unit" is explicitly retired.** Small units and one-task-per-commit still stand;
  the stopping does not. That is override 14 in spirit, though it is a process decision
  rather than a design one, so it is not in the override table.

**Also logged rather than done on the way past:** `S1-47`, the same scrollbar on the home
list. It is not a copy-paste — home's rows are full-bleed and carry a delete icon at the
right edge, so the 8px gutter moves every icon inward, and that is a look decision.

- ✅ `cargo clippy --workspace --all-targets` clean, `cargo fmt --all`, 28 tests pass.
