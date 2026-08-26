# Canvas — Task Backlog

**Purpose:** a resumable, ordered list of small units of work. A new session reads
[progress.md](progress.md) first for the state of the project, then starts here at the
first unticked box.

**Last updated:** 2026-08-26
**Current task:** `SW-24` — rip out the `document` widget type, the user's call on 2026-08-26. Then **Stage W2** in order: SW-19, SW-20, SW-21, SW-13, SW-14.
**Stage:** W2 — the rework the user asked for on 2026-08-26 after walking the app. Stage W is built; Stage W2 fixes two live bugs, adds the editing layer neither §10.3 nor §10.4 specifies, and unblocks the Spreadsheet. The Whiteboard is deferred.
**Done:** 29 ticked. 160 tests pass. Stage W2 added 6 tasks on 2026-08-26 (SW-18 to SW-23); SW-12, SW-13 and SW-14 were rewritten into it. `SW-24` was added mid-run on 2026-08-26 when the user asked for the `document` type to be ripped out.
**Left 🟡 for the user to look at:** `S1-43` (undo/redo), `S1-44` (reduced motion), `S1-47` (home scrollbar), `SW-01` (the type-dispatched runtime), `SW-03` (the four components, via `--gallery`), `SW-05`+`SW-02` (the Todo widget), `SW-16` (three header controls), `SW-08` (the Calendar widget, seeded and ready to look at), `SW-17` (the footer glyph strip — and the Shared Document and Calendar are creatable for the first time), `SW-10` (the Kanban board and its card drag), `SW-22` (typing into a card or row no longer dies after one character), `S1-28` (home empty state), `S1-29` (open/close animation), `S1-30` (off-screen clamping), `S1-31`–`S1-35` (the palette and its hotkey), `S1-51` (drag feel) — see the verification queue

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
   see rule 2 in the next section), move **Current task**, rewrite the GO block in `progress.md`, append a
   dated entry to `progress.md`, `git commit` with the task ID in the subject.
6. **Go straight back to step 1 for the next task.** No summary, no check-in, no asking
   what to do next. The user said "go" once; that covers the whole backlog.
7. **Stop at 90% of the Claude usage limit**, whatever the backlog says — user instruction,
   2026-08-25, also in CLAUDE.md's standing rules and the GO block in [progress.md](progress.md).
   Do not start another task past it; finish the one in hand to a committed, logged state,
   then stop and say you stopped on the limit rather than on the backlog. Only an explicit
   "keep going" overrides it.
8. Stop also when one of these is true, and say which:
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

**⚠ The user asked on 2026-08-26 for this queue to be CLEARED** — walked end to end and
ticked or turned into numbered bugs — **once the widgets are wired and working.** They then
walked the app themselves and found that the Kanban is not working (SW-22, SW-21), so the
clearing is now gated on **Stage W2** rather than Stage W. It is still not an indefinite
parking lot: when Stage W2 is done, the walk-through is the next unit of work, not another
feature. Say so in the end-of-run report every time until it happens.

**A session can now take its own screenshots** — the recipe is in `progress.md`'s START HERE.
It is not a substitute for the user's eyes on *"is this beautiful"*, but it does settle
alignment, spacing and "did it render at all", so **check the queue lines you can check
yourself before handing the list over.**

**Note window**
- [ ] **Drag the header — the plate must stay glued to the grab point for the whole
      drag, no drift, no inertia, no stutter.** Rewritten in **S1-51** after the user
      reported stutter: it now anchors to `GetCursorPos()` at press instead of
      accumulating window-relative deltas. Do the same drag on the home window's header
      → **S1-07**, **S1-51**
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
- [ ] The **note glyph** in the footer strip — creates a note, opens its window, adds a row
      → **S1-25**. It was a `NEW NOTE` button until **SW-17**
- [ ] Click a row — toggles that note open/closed; the row's dots reflect it → **S1-24**
- [ ] Row delete icon — confirm overlay, then gone from list and from disk → **S1-26**
- [ ] Empty state shows on a fresh database (delete `%APPDATA%/Canvas`), centred in the
      list area, and disappears the moment the first note exists → **S1-28**
- [ ] Open three notes at once — all independently draggable and editable → confirms **S1-20**
- [ ] Home `x` closes home and leaves notes running
- [ ] Make **ten or more** notes (hold the note glyph) so the list overflows: a 4px thumb
      appears at the home list's right edge, tracks the wheel, and drags to scroll →
      **S1-47**
- [ ] With the list overflowing, hover a row — the row highlight and its delete icon must
      still work right up against the thumb, and hovering the thumb must **not** highlight
      the row behind it → **S1-47**
- [ ] Put a note window next to home and compare right edges: the row's delete icon and
      the note's `x` must sit at the same distance from their plate's right edge. The
      scrollbar is overlaid, not guttered, precisely so this stays true → **S1-47**
- [ ] Delete notes back down to a few — the thumb disappears and no 8px strip is left
      behind → **S1-47**

**Open/close animation** → **S1-29**
- [ ] Create a note from the footer strip — its plate fades up over ~160ms and rises 6px
      into place.
      Nothing scales, tilts or bounces
- [ ] Click `x` on a note — the plate fades out over ~100ms and does **not** travel
      downward on the way out
- [ ] Same two checks on the home window: tray → `Open home` fades it up, `x` fades it out
- [ ] Delete a note from its own trash icon — same 100ms fade, not an instant vanish

**Command palette** → **S1-31**, **S1-32**
- [ ] **The big one for S1-35:** click on something that is *not* Canvas — a browser, the
      desktop — and press **Ctrl+K**. The palette should come up over it, focused, ready
      to type. If nothing happens, say so: it means winit is not pumping the hidden
      message window, and the fix is to subclass a real window instead → **S1-35**
- [ ] From anywhere, press **Ctrl+N** — a new note appears without touching the mouse
      → **S1-35**
- [ ] Watch the console at launch for `Ctrl+K / Ctrl+N are not registered globally` —
      that line means another app already owns the combination → **S1-35**
- [ ] Click into a note's body and press **Ctrl+K** — the screen dims to a flat scrim
      (dimmed, **not** blurred — text behind it must stay sharp) and a 640px-wide plate
      with a 28px radius sits centred, its top about 15% down the screen
- [ ] Press **Esc** — the palette goes away and the note is still there
- [ ] The scrim covers the whole primary monitor, including over other apps
- [ ] **The plate is centred left-to-right with its top ~15% down the screen.** It used to
      land off to the right because the window was built 1.5× too big; the window geometry
      is now confirmed correct by measurement, so this line is only about the plate inside
      it → **S1-49**
- [ ] The plate shows a 24px `search` icon, `TYPE A COMMAND` in small dim caps, and three
      rows — `New note` / `New todo` / `Settings` with `CTRL+N` / `CTRL+T` / `CTRL+,`
      right-aligned, each with a 24px icon → **S1-33**
- [ ] The four 24px icons (`search`, `plus`, `check`, `settings`) look like the header's
      16px ones, just bigger — same stroke weight relative to the glyph → **S1-31**
- [ ] Type anything — the placeholder goes, the query shows in big caps with a tall caret,
      and the three rows disappear (results arrive in S1-34) → **S1-33**
- [ ] **Look hard at the footer text for tofu boxes.** It should read
      `UP/DOWN MOVE · ENTER OPEN · ESC CLOSE`. Silkscreen has no arrow or return glyphs,
      so §9's `↑↓ MOVE · ⏎ OPEN · ESC CLOSE` was deliberately not used — if you would
      rather have the symbols, they need a second font and that is a decision to make
      → **S1-34**
- [ ] ↑ and ↓ move the highlight, which is a plain `elev-raised` row fill with the icon
      stepping to white — **no accent colour and no left bar anywhere** → **S1-34**
- [ ] The footer's left side reads `3 RESULTS` / `1 RESULT` / `NO RESULTS` and matches
      what is actually listed → **S1-34**
- [ ] With rows showing, press ⏎ on `New note` — the palette closes and a new note opens
      → **S1-34**
- [ ] Type part of an existing note's **title** — it appears as a row with the note glyph,
      and `OPEN` on the right if that note is currently on screen. ⏎ opens it → **S1-34**
- [ ] 🟡 typing a word that is only in a note's **body** finds nothing. That is deliberate
      and logged as **S1-48**, not a bug
- [ ] 🟡 `New todo` and `Settings` do nothing — there is no todo type until Stage W and no
      settings screen at all. Both print a line to the console instead of pretending


**Undo / redo** → **S1-43**
- [ ] Type a sentence into a note, wait a second, type another, then press **Ctrl+Z** —
      the second sentence goes and the first stays. Ctrl+Z again clears the first.
      **Ctrl+Y** (or **Ctrl+Shift+Z**) puts each one back
- [ ] Undo far enough that the note is empty — `NOTHING HERE YET` comes back and further
      Ctrl+Z does nothing (no crash, no flicker)
- [ ] Undo once, then type a new word — Ctrl+Y must now do **nothing**: the redo tail is
      gone, which is correct
- [ ] **The restart one.** Type three separate sentences, pausing ~1s between each. Close
      the note's window with `x`, reopen it from the home list, and press Ctrl+Z — it must
      still walk back through those three sentences. Now quit from the tray, relaunch, and
      do the same
- [ ] Undo, then check the home row's preview line updates to match
- [ ] 🟡 An undo is itself recorded as a forward step on disk. So after a **restart**,
      Ctrl+Y will not redo an undo made in the previous session — Ctrl+Z will, because it
      undoes the undo. Deliberate, not a bug


**Reduced motion** → **S1-44**
- [ ] Launch and read the console line `canvas: reduced motion = <bool>`. On a default
      Windows box it says `false`
- [ ] Turn **Settings → Accessibility → Visual effects → Animation effects** OFF and watch
      the *already running* app: within half a second the next new note must appear with
      a **flat 100ms fade and no 6px rise**, and dragging a note must not deepen its shadow
      over time — the shadow changes instantly or not at all
- [ ] With it still off, Ctrl+G a note to the `syncing` state — the lit dot must **stop
      sweeping and hold**, not fade or crawl
- [ ] Turn the setting back on — the rise and the shadow ramp come back without a relaunch
- [ ] Faster path if you would rather not touch Windows settings:
      `$env:CANVAS_REDUCED_MOTION="1"; cargo run -p canvas-app` forces it on, `"0"` forces
      it off. This overrides the OS and is for testing only


**The runtime after SW-01** → **SW-01**
- [ ] Everything in the **Note window** and **Home window** blocks above still behaves the
      same. SW-01 renamed `App.notes` to `App.widgets` and moved the window and document
      behind enums — **nothing about a sticky note was meant to change**, so any difference
      you notice here is a regression, not a feature
- [ ] `cargo run -p canvas-app --example insert_todo`, then launch: the console says
      `is a "todo" widget and that type has no document yet (Stage W)` instead of the row
      being silently dropped, and the row shows in home with the `check` glyph. Clicking it
      prints the same line rather than doing nothing. `--example insert_todo --remove`
      deletes it again
- [ ] Ctrl+K and type a note's title — the result row's 24px glyph is the **note** glyph.
      SW-01 made the palette pick its glyph by widget type instead of hardcoding `note`,
      so this is the line that proves the mapping still resolves → **S1-34**


**The four library components** → **SW-03**

`cargo run -p canvas-app -- --gallery` opens a dev-only window with all four in every
state §6 lists. Check each against §6's own table.

- [ ] **Checkbox box is a 16×16 SQUARE with barely-rounded corners**, not a disc. That is
      the whole reason §5.4 carves out 2px for it
- [ ] Checked shows **no tick** — the fill going white *is* the state
- [ ] Hover a checkbox: the box border steps `edge` → `dim`, the label does not move
- [ ] The completed row's label is `dim` with a 1px strike **that stops where the text
      stops**, not one running to the edge of the row
- [ ] Tab to a checkbox and press Space — it toggles, and the focus ring is a 2px **white**
      square ring around the box. Never red (§6.9)
- [ ] Tag: rest is dark with a dim label and a hairline border; active is a solid white
      pill with black text and **no** border. Both fully round
- [ ] TextField: hover the first one — the border steps `edge` → `dim`. **Click into it and
      check the caret lands where you clicked**, not just at the end. That is the specific
      thing the nested-TouchArea decision could have broken
- [ ] The error field's border is red and **nothing else is** — no red text, no icon, no
      message inside the field (§6.6)
- [ ] Focused field: 2px white ring sitting 2px outside the border, corners following it
- [ ] The disabled field is uniformly faded, text included, and will not take a caret
- [ ] Divider is a 1px hairline running the full width with no inset at either end


**Three header controls, colour dropped** → **SW-16**
- [ ] A note's header now shows **three** icons — pin · delete · close — not four. The
      rightmost is still 16px from the plate's right edge and they are evenly spaced
- [ ] The title has 24px more room before it elides
- [ ] A note that already had a grey tone set **still opens tinted**. The tone is kept in
      the database and still renders; there is just no control to change it now
- [ ] Hovering anywhere in the header still fades all three in together


**The Todo widget** → **SW-05**, **SW-02**

This is the first widget type other than the sticky note. Everything in the **Note window**
block above should behave the same way here — it is the same header, the same drag region,
the same three controls.

- [ ] The **check glyph** in the footer strip creates a todo, opens its window, and adds a
      row to the list with the **check** glyph rather than the note glyph → **SW-02**.
      This was a pair of labelled `NEW NOTE` / `NEW TODO` buttons until **SW-17**
- [ ] Ctrl+K → `New todo` (⌘T) does the same thing. It used to print "todo widgets do not
      exist yet" to the console → **SW-02**
- [ ] The todo window is `340×360` with the same 32px header, glyph strip, editable title
      and three controls as a note
- [ ] Type in the bottom row and press **Enter** — the item appears in the list and the
      input clears, ready for the next one
- [ ] Click an item's checkbox — the box fills white with **no tick**, the label steps to
      `dim`, and a 1px strike appears **over the text only**, stopping where the text stops
- [ ] The footer reads `N OPEN · N DONE` and both numbers change as you tick things
- [ ] **Double-click an item** — it becomes an editable field, pre-selected. Enter or Esc
      commits. Only one row can be editing at a time
- [ ] Hover an item — a delete icon appears at its right edge; clicking it removes the item
- [ ] Add more items than fit — the same 4px thumb as the note body appears and drags
- [ ] The trash control says `DELETE LIST`, not `DELETE NOTE`
- [ ] **Add items, close the window with `x`, reopen it from home — they are all there.**
      Then quit from the tray, relaunch, and check again
- [ ] Drag the todo window by its header — same feel as a note, no drift → also
      **S1-51**
- [ ] 🟡 A due-date `Tag` is built into the row but nothing sets a due date yet —
      `TodoListDoc` stores `{id, text, done}` only, deliberately (SW-04). An empty tag
      column is expected

**The Calendar widget** → **SW-08**

A calendar row has already been seeded into the database — it is called **THIS WEEK**, it
holds 8 events, and it is flagged open, so it should appear the moment you launch. If it
is not there, run `cargo run -p canvas-app --example insert_calendar`.
`--example insert_calendar -- --remove` deletes every calendar row when you are done.

- [ ] The window is `480×360` with the same 32px header, glyph strip, editable title and
      three controls as a note, and the trash control says `DELETE CALENDAR`
- [ ] Seven equal day columns, `MON`–`SUN`, gap 8, each with a 1px rule above its label
- [ ] **Today's column only**: the rule and the label step to `text`; the other six stay
      `edge` and `dim`. It is a rule, not a wash — nothing behind the column changes
- [ ] Eight chips, at these positions — this is the artboard's own layout, so it should
      look like `handoff/screenshots/widget-types.png`:
      MON 09:30 · TUE 11:00 · WED 09:30 and 14:00 · THU 09:30 and 16:00 · FRI 13:00 ·
      SUN 18:00, and **SAT empty**
- [ ] Chips carry the **24-hour time only** — no event names anywhere
- [ ] Chip height and grey level both track duration: the 30-minute stand-ups are the
      shortest and darkest, WED 14:00 (3 hours) is the tallest and lightest
- [ ] Chips sit lower down the column the later they start — 09:30 near the top, 18:00
      near the bottom
- [ ] **Resize the window taller** — the chips spread out to keep the same relative times.
      Their heights must not change; only their spacing does
- [ ] Close it with `x`, reopen from home, quit from the tray and relaunch — all 8 chips
      come back in the same places
- [ ] 🟡 Nothing in the window can add, move or delete an event yet — §10.4 specifies the
      readout and no editing UI at all, so that is **open question 10**, not an oversight


**The create control** → **SW-17**

Home's two labelled buttons are gone. In their place is a strip of 16px glyphs, one per
built type, with a single label to their right.

- [ ] The footer shows **four glyphs** — note, check, doc, calendar — at 16px, gap 8,
      `dim` at rest, in the same place the two buttons used to be
- [ ] The label beside them reads `NEW` at rest and changes to `NEW NOTE`, `NEW TODO`,
      `NEW DOCUMENT`, `NEW CALENDAR` as you hover each glyph. **The label must not move,
      only its text changes**, and the glyph itself steps `dim` → `text`
- [ ] Tab into the strip — the same label follows keyboard focus, and each glyph shows a
      focus ring. Enter or Space creates that type
- [ ] Click each of the four — each opens a window of the right type and adds a row to the
      list with the matching glyph. **The Shared Document and the Calendar were previously
      unreachable; this is the first time either can be created from the UI**
- [ ] Ctrl+K with an empty query now lists **five** rows: New note, New todo, New document,
      New calendar, Settings — each with the same glyph home uses for that type
- [ ] `New note` still says `CTRL+N`; `New todo` still says `CTRL+T`; document and calendar
      show no shortcut. 🟡 **`CTRL+T` is not actually registered** — only Ctrl+K and Ctrl+N
      are. That is §9's own text and predates SW-17
- [ ] Ctrl+K, type a note's title, Enter — it still **opens that note** rather than creating
      anything. This is the one that would catch an action id colliding with a widget id
- [ ] With nothing in the list, the empty state now reads
      `Press Ctrl+K, or pick a type below.`
- [ ] 🟡 The hover label is driven by Slint's `changed hovered` handler, which is the one
      piece of this that has never been run. If the label sticks on a type after the pointer
      leaves, or never changes at all, that is the handler not firing — open a task

**The Kanban board** → **SW-10**

Create one from the **board glyph** in home's footer strip, or `New board` in the palette,
or launch `cargo run -p canvas-app -- --open-kind kanban` (**SW-23**).
It should open already carrying three columns — you do not have to set anything up.

- [ ] ⚠ **Do this one first — a card may not be surviving a restart.** On 2026-08-26 a
      screenshot caught this board carrying one `New card` in `TODO`; every later launch,
      and `--dump`, showed the board empty with `revs=15` in `revision_history`. The frame
      was taken with a DPI-unaware capture whose geometry did not match the window list, so
      it may simply be an unreliable frame — nothing in the code writes an empty document
      over a full one, and a probe run added no revisions. **Add a card, wait two seconds,
      quit from the tray, relaunch.** If the card is gone, that is a real persistence bug
      and it outranks everything else in Stage W2

- [ ] The window is `640×400` with the same 32px header, glyph strip, editable title and
      three controls as a note, and the trash control says `DELETE BOARD`
- [ ] Three equal columns, gap 12, titled `TODO` `DOING` `DONE`, each with its name in
      `text`, a `0` count in `dim` at the right, and a 1px `edge` rule under both
- [ ] A `+` sits under each column. Click one — a card called `New card` appears at the
      bottom of that column and the column's count goes up
- [ ] Cards are `elev-raised` with a 1px `edge`, radius 8, and read
      `New card` over `UNASSIGNED` in interface-face-11 `dim`
- [ ] **Drag a card into another column.** While you drag: the card follows the pointer 1:1,
      its shadow deepens, and **nothing tilts, scales or rotates** (rule 5)
- [ ] **The column you are over gets a 1px `text` outline around the whole column — no
      fill, no tint.** If it fills or tints instead, that is a bug
- [ ] Let go — the card lands in that column, both counts update, and the order is the
      order you dropped it in. Drop it between two existing cards and check it stays there
- [ ] Drag a card **within** its own column to reorder it
- [ ] Double-click a card — its title becomes an editable field, pre-selected. Enter or Esc
      commits. Only one card can be editing at a time
- [ ] Hover a card — a trash icon appears at its top right; clicking it removes the card.
      **The right edge of a card is not draggable** — that strip belongs to the icon, the
      same arrangement the Todo rows use
- [ ] **Add cards, move them, close the window with `x`, reopen from home — everything is
      where you left it. Then quit from the tray, relaunch and check again.** This is the
      one that proves the fractional index is being written, not just computed
- [ ] Drag the board window by its header — same feel as a note, no drift → also **S1-51**
- [ ] 🟡 Fill a column past the bottom of the window — the extra cards are **clipped and
      there is no scroll.** §10.3 never specified one. Expected, not a bug; open a task if
      you want it
- [ ] 🟡 Nothing sets an assignee or a due date, so every card reads `UNASSIGNED`. The
      fields exist in `KanbanDoc` and the meta line renders `MIRA · 25 AUG` correctly when
      they are set — there is just no UI for setting them, same shape as the Todo widget's
      unset due-date `Tag`
**Typing into a repeater row** → **SW-22**
- [ ] **The one this task exists for:** double-click a Kanban card and type a whole
      sentence. Every character must land — the caret must not vanish after the first one
- [ ] Press Enter — the card shows the full sentence, and the count and layout are right
- [ ] Do the same on a **Todo** row: double-click it, type a sentence, press Enter
- [ ] Escape mid-sentence instead of Enter — the row still shows what you typed (the
      keystrokes went into the document as they were made; Esc only ends the edit)
- [ ] Close the window with `x`, reopen from home — the text is still there. Then quit from
      the tray, relaunch, and check again
- [ ] While a card is being edited, click the `+` on another column — the edit ends and
      **both** the typed card and the new one are painted. Same for a Todo row: type into
      one, then tick another row's checkbox. Structural changes end the edit on purpose,
      because the rebuild they need is the one thing the fix suppresses

**The `Sheet` overlay** → **SW-18**

`cargo run -p canvas-app -- --gallery` — the right-hand plate is a mock Kanban at the real
640×400 with the sheet already open over it.

- [ ] The sheet **starts open**. Press `Esc` — it fades out over ~100ms, opacity only, no
      travel, and the three columns and the `Behind the sheet` card are underneath
- [ ] Click `OPEN SHEET` — it fades up over ~160ms **and rises 6px** into place. Nothing
      scales or bounces
- [ ] While it is open, the glyph strip and the `SHEET DEMO` title are still visible and the
      board is **completely** hidden — no veil, no dimming, nothing showing through
- [ ] Drag the 4px thumb at the right of the field column, and wheel-scroll the fields —
      the fifth field `NOTES` comes into view. The footer does not move
- [ ] Click `SAVE` — the sheet closes and the line at the bottom left of the board reads
      `SAVED — Ship the strip spec`. `CANCEL` gives `CANCELLED`, `DELETE CARD` gives
      `DELETE REQUESTED`
- [ ] Press Enter inside any field — same as `SAVE`
- [ ] `SAVE` is a solid white pill with black text, `CANCEL` is dark with a hairline, and
      `DELETE CARD` is dark with a **red** border and red label — and red appears nowhere
      else on the sheet (rule 2)

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

- [ ] **S1-31 · The five palette icons** — `S` — 🟡 built, verify with S1-32
  Needs: S1-05. `search`, `plus`, `check`, `settings`, `enter`, same construction rules,
  rendered at **24px — a size that exists nowhere else in the product**.
  *DoD:* all five render correctly at 24px.
  *Refs:* §5.6, §9
  All five glyphs already existed from S1-05. What this task actually added is the
  **24px size**: `IconGlyph` now takes a `size`, and the stroke scales with it
  (`Layout.icon-stroke * size / icon-size` = 2.25px at 24px), because §5.6 draws the
  glyphs on a 16-unit grid and Slint does **not** scale `stroke-width` with the viewbox.
  **Nothing renders at 24px until the palette exists**, so this box can only honestly be
  ticked alongside S1-32.

- [ ] **S1-32 · Palette overlay shell** — `M` — 🟡 built, needs eyes
  Needs: S1-31. 640px wide, radius 28, `plate` fill, 1px `edge`, drag-depth shadow.
  Centred horizontally, top at ~15% of screen height. `scrim` behind it — **flat, no
  blur**; the one deliberate translucency exception, and only while the palette is open.
  *DoD:* appears centred with the scrim; nothing behind it is blurred.
  *Refs:* §9, §5.1
  `ui/palette.slint` — a frameless, transparent, always-on-top window sized to the
  **primary monitor**, drawing the scrim plus a `Panel` at `dragging: true` for the
  drag-depth shadow. Esc dismisses. **No `TouchArea` anywhere in it** — click-outside is
  not in §9, and a full-window one would sit under every row S1-34 adds (trap 6).
  Opened by a **temporary** Ctrl+K in the note body so the shell can be seen at all;
  **S1-35 owns the real hotkey** and should replace it.

- [ ] **S1-33 · Palette input row + empty-query actions** — `M` — 🟡 built, needs eyes
  Needs: S1-32. Input row 56px, padding 0/24, 24px `search` icon + 12px gap + query in
  the interface face at 22px + a 2×22 caret. Empty placeholder `TYPE A COMMAND`.
  Default actions: **New note** (⌘N, `plus`), **New todo** (⌘T, `check`), **Settings**
  (⌘, `settings`).
  *DoD:* opens showing the three defaults; typing replaces them with results.
  *Refs:* §9
  Shortcut meta reads `CTRL+N` / `CTRL+T` / `CTRL+,` — §9's ⌘ is macOS notation and this
  build is Windows. The rows are **display-only here**: selection, ⏎, the footer and
  searching notes are all S1-34, so typing currently empties the list rather than showing
  results. The plate's height follows the row count, so it collapses to the 56px input
  row alone.

- [ ] **S1-34 · Palette results, selection, footer** — `M` — 🟡 built, needs eyes
  Needs: S1-33. Rows 48px, 24px icon + 16px gap + body-14 label + right-aligned
  interface-11 `dim` meta. Selection = row fill `elev-raised`, icon steps to `text` —
  **no accent fill, no left bar**. Footer 32px, count left,
  `↑↓ MOVE · ⏎ OPEN · ESC CLOSE` right. Searching existing notes belongs here too.
  *DoD:* ↑↓ moves, ⏎ opens, Esc closes, count correct, no accent anywhere.
  *Refs:* §9, rule 2
  **The footer legend reads `UP/DOWN MOVE · ENTER OPEN · ESC CLOSE`, not §9's string.**
  Silkscreen has no `↑` (U+2191), `↓` (U+2193) or `⏎` (U+23CE) — checked against the
  font's own cmap, not guessed — so §9's literal string would draw three tofu boxes. `·`
  is present and kept. **Search is title-only**: matching bodies would mean decoding every
  note's yrs blob from disk on every keystroke. Logged as **S1-48**.

- [ ] **S1-35 · Palette hotkey** — `M` — 🟡 built, needs eyes
  Needs: S1-34. Ctrl+K opens it; Ctrl+N creates a note.
  A **true OS-global** hotkey needs Win32 `RegisterHotKey` — `windows-sys` is already a
  dependency by then, so this no longer needs a separate sign-off. The stopgap bound in a
  focused window's `key-pressed` is acceptable first, provided it is **stated plainly**
  as only firing while a Canvas window has focus.
  *DoD:* a note is creatable with **no mouse at all**. The hotkey's real scope stated
  honestly in `progress.md`.
  *Refs:* §14 Stage 1 item 10
  Real `RegisterHotKey`, delivered through a hidden `HWND_MESSAGE` window with its own
  wndproc, since Slint's winit loop would hand `WM_HOTKEY` to `DefWindowProc` otherwise.
  **The unverified part is whether winit's loop pumps that window's messages at all** — if
  Ctrl+K does nothing from another app, that is the reason, and the note-body Ctrl+K
  fallback is deliberately still there. Needs the `Win32_Graphics_Gdi` feature on
  `windows-sys` (a feature, not a new crate — rule 5).

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
  **Built 2026-08-25 as `UndoHistory` in `canvas-doc`.** `yrs::UndoManager` exists in
  0.19.2 and was rejected for a specific reason: its stack items are built from observed
  transactions, so it **cannot be seeded from stored updates** — replaying a note's
  revisions onto a doc that already holds the snapshot is a no-op and produces no stack at
  all, which kills the across-a-restart half of the DoD. `UndoHistory` instead replays
  `revision_history` into a scratch doc and captures the text at each flush batch
  boundary (`created_at` gap > 100ms; batches land ~500ms apart because the flush timer
  does). Undo/redo then walk those states and re-enter the live doc through
  `StickyNoteDoc::set_text`, so every step is still a real CRDT diff and no code path
  does last-write-wins. If the replay's final text does not match the live doc — which is
  what a `trim_revisions` cull looks like — the restored stack is discarded rather than
  offering wrong text. Ctrl+Z / Ctrl+Y / Ctrl+Shift+Z in the note body.
  *DoD:* undo/redo correct across a restart, single-player. 🟡 — the model is unit-tested
  (10 tests, including the restart path), but the key bindings have not been watched
  working. In the verification queue. The named Stage 1 DoD bullet
  — *"undo works correctly after a remote edit arrives mid-session"* — **cannot be
  satisfied until Stage 3 exists.** Mark that half 🟡 with this reason; do not tick it and
  do not fake a remote edit.
  *Refs:* §14 Stage 1 item 12 and DoD

- [ ] **S1-44 · Reduced motion** — `M`
  Needs: S1-29. All travel, stepping, and shadow motion off; a 100ms crossfade replaces
  them; dots hold their final state. **Required for sign-off**, not a deferred nice-to-have.
  **Done 2026-08-25.** The theme half was already right — `Motion.reduced` collapses
  `open` 160→100ms, `open-rise` 6→0px, `drag-shadow` 100→0ms and `presence` 80→0ms, and
  the glyph strip's sweep `Timer` is gated on `!Motion.reduced` so the dots hold. What was
  missing was **reading the OS setting**: `reduced_motion.rs` calls
  `SystemParametersInfoW(SPI_GETCLIENTAREAANIMATION)` — TRUE means animations are
  *enabled*, so reduced motion is its inverse. `App` caches it, sets
  `Motion.reduced` on every window at creation, and re-reads it inside the existing 500ms
  flush timer so a mid-session toggle of the Windows setting reaches live windows without
  a relaunch. `CANVAS_REDUCED_MOTION=1|0` overrides it for testing, and the boot line
  `canvas: reduced motion = <bool>` says which way it went.
  *DoD:* with the OS reduced-motion setting on, no widget travels, steps, or shifts shadow.
  ✅ the setting is read correctly — confirmed 2026-08-25 by running the app on a default
  machine (`false`, animations on) and under the override (`true`). 🟡 **that the windows
  visibly stop travelling still needs eyes** — in the verification queue.
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
  **Done 2026-08-25, and the gutter was NOT reserved.** The note body has to reserve one
  because its `TextInput` wraps and a gutter that came and went would re-wrap the text.
  A home row cannot: every text in it is `wrap: no-wrap; overflow: elide`, so there is
  nothing to reflow. The `ScrollBar` is therefore **overlaid** on the rightmost 8px of the
  list area and the rows keep their full width — which keeps each row's delete icon at
  `width - pad - icon-size`, exactly where the note window's `close-btn` sits, so the two
  still line up. §13.3 holds without a gutter as well: the row's own `TouchArea` stops
  44px short of the right edge and the delete `IconButton` spans `width-32 … width-16`, so
  the 8px band overlaps neither, with 8px of dead space between them. The list and its bar
  now share one conditional wrapper `Rectangle` so the empty state (S1-28) still swaps in
  and out exactly as before, rather than sitting behind a hidden `Flickable`.
  *DoD:* with more notes than fit, the thumb appears and drags; the delete icons still
  line up with the note window's own header cluster. 🟡 **needs eyes** — the app launches
  clean but three notes do not overflow a 520px list. In the verification queue.

- [ ] **S1-48 · Palette search should match note bodies, not just titles** — `M`
  Needs: S1-34. S1-34 searches titles only. Matching bodies the obvious way means
  `get_document` + `StickyNoteDoc::load` for **every** note **on every keystroke** — disk
  I/O and a yrs decode on the typing path, which is what Hard Constraint 2 and trap 7
  exist to prevent. It needs a cheap searchable copy of the text instead: either a plain
  `text` column on `widgets` kept up to date by the same 500ms flush that already writes
  the snapshot, or SQLite FTS5. **Ask before changing the schema** (rule 6).
  *DoD:* typing a word that appears only in a note's body finds that note, and typing in
  the palette with 200 notes present does not stutter.
  **Blocked 2026-08-25 on open question 9** — a `text` column on `widgets` or FTS5 is a
  schema change, and rule 6 says do not make one unsupervised. Skipped; not abandoned.
  *Refs:* §9, §12, Hard Constraint 2

- [x] **S1-49 · The palette opened 1.5× too big and off to the side** — `S`
  Needs: S1-32. Reported by the user: the Ctrl+K plate lands to the right of centre
  instead of in the middle. **Cause, measured with a DPI-aware `GetWindowRect`:** the
  window was created **3840×2160 on a 2560×1440 screen**. `screen::primary_bounds()`
  returns *physical* pixels, but a Slint window that has not been `show()`n yet reports
  `scale_factor() == 1.0` — the real 1.5 only arrives later as an event-loop event. So
  Slint passed "2560×1440 logical" to winit, winit applied the 150% DPI, and every
  coordinate inside the window came out 1.5× too far: the plate centres on `parent.width`,
  so it sat ~1440px across a 2560px screen, and its top sat 1.5× too low too.
  *Fix:* `fill_screen()` converts physical bounds to logical using `App::known_scale()` —
  the scale factor of an **already open** window (home reports the correct 1.5) — and is
  applied **before** `show()`. Do not "correct" it again after `show()`: that call reads
  the new window's own scale, still 1.0, and puts the bug straight back.
  *DoD:* the palette window measures exactly the screen. ✅ confirmed 2026-08-25 by
  measurement — `2560x1440 at (0,0)`. The plate's own centring still wants eyes.

- [x] **S1-50 · The palette's framebuffer was never given back** — `S`
  Needs: S1-49. A full-screen software-rendered window is 15 MB of framebuffer at
  2560×1440, and was 33 MB at the wrong 3840×2160. `hide_palette` only called `hide()`,
  so the window — and its buffer — stayed alive for the rest of the session. It now
  `take()`s the palette out of `App` and drops it from a 16ms `Timer::single_shot`, so the
  component is never destroyed inside its own `dismissed` callback. The per-note pin-poll
  timer was `Box::leak`ed and outlived its note; it is now owned by `Note`.
  *DoD:* memory returns to the boot figure after the palette closes. ✅ confirmed
  2026-08-25 by measurement: open 41.4 → 25.0 MB private, dismissed 41.5 → 11.0 MB
  (boot is 10.9), 9.95 MB after a second open/close cycle.

- [ ] **S1-51 · Drag tracks the cursor absolutely instead of accumulating deltas** — `S`
  Needs: S1-07. Reported by the user: dragging a note stutters and does not feel 1:1 on a
  165Hz screen. The old handler applied `mouse-x - pressed-x` — a **window-relative** delta
  — to the window's own position on every move, which feeds the window's movement back
  into the next delta. Measured with synthesised input: the cursor stayed inside a 120px
  band and the window ran ~970px to the screen edge.
  *Fix:* `begin_drag()` records `GetCursorPos()` minus the window origin once, on
  `drag-started`; `drag_move()` then sets the position to `cursor − anchor` every move.
  Nothing accumulates, so nothing can drift. It also skips `set_position` entirely when
  the result is unchanged, and `placement_dirty` is now set on `drag-ended` rather than on
  every mouse-move, keeping SQLite off the drag path. Falls back to the old delta maths if
  `GetCursorPos` is unavailable (non-Windows).
  *DoD:* **needs eyes** — the plate must sit still under the grab point for a whole drag,
  with no drift and no stutter. In the verification queue.
  *Refs:* §13.3, §5.5

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

- [x] **SW-01 · Generalise the runtime to any widget type** — `L`
  Needs: S1-20. `app.rs:50` filters boot to `KIND_STICKY_NOTE`, and `Note` is a concrete
  struct holding a `StickyNoteWindow` and a `StickyNoteDoc`. Both need to become
  type-dispatched: one live-widget registry that can hold any window/doc pair, spawned by
  `widget_type`. Keep the flush timer, the placement tracking and the pending-update
  buffer shared — they are type-agnostic already and must not be duplicated per type.
  **Done 2026-08-25.** New `widget.rs`: `WidgetDoc` and `WidgetWindow` are enums with one
  arm each today, and `LiveWidget` holds the shared state — placement, pending updates,
  the two dirty flags, the undo history and the poll timer — **once**, not per type.
  Enums rather than trait objects because `slint::ComponentHandle` has an associated type
  and is not object-safe; adding a type is one arm per `match`, and the compiler then
  names every site that needs it. `App.notes` became `App.widgets`, `open_note` became
  `open_widget`, and boot no longer filters by kind.
  *DoD:* a row of a non-note type in the database opens its own window at boot instead of
  being silently skipped. Verified with a hand-inserted row before any real second type
  exists. ✅ **the silent skip is gone** — confirmed 2026-08-25 by
  `cargo run -p canvas-app --example insert_todo` and reading the boot log:
  `canvas: <id> is a "todo" widget and that type has no document yet (Stage W)`.
  ✅ **"opens its own window" closed 2026-08-26 by SW-05**, as this entry predicted: with a
  real Todo window in the enums, `cargo run -p canvas-app --example insert_todo` followed by
  a launch now opens that row in its own window with nothing on stderr, instead of the
  "no window yet" line. `--example insert_todo --remove` cleans the row up again.

- [ ] **SW-16 · Three header controls on every type; the colour control is dropped** — `S`
  Needs: nothing. **Added and done 2026-08-26**, answering open question 8. The user chose
  `pin · delete · close` on every widget type, and **removed colour from the sticky note
  too** — knowing it regresses a control they had previously confirmed working. That
  explicit word is what the DO NOT REGRESS floor requires; the override table in
  `progress.md` records it.
  `colour-btn` and the `tone-changed` callback are gone from `sticky-note.slint`, along
  with `App::on_tone_changed`'s wiring. **`Layout.cluster-w` is now 64px, not 88** — the
  drag region, the title width and the title editor all derive from it, so that one number
  is the whole layout change. `IconPalette` stays in `theme/icons.slint`: it is one of
  §5.6's sixteen named glyphs and the library keeps all of them.
  **`Palette.plate-tone` and the `tone` column in `window_positions` are deliberately
  kept.** A note already carrying a tone still renders it; there is simply no longer a
  control to change it. Deleting them would have made this irreversible and would have
  needed a migration.
  *DoD:* the note header shows three controls, evenly spaced, with the rightmost still
  16px from the plate edge; the title has 24px more room; a note that already had a tone
  still opens tinted. 🟡 **needs eyes** — the app builds, is clippy-clean and launches, but
  the header has not been looked at. In the verification queue.

- [ ] **SW-02 · Home: a create control per type** — `M`
  Needs: SW-01. `NEW NOTE` becomes a create control per built type, each using its own
  §5.6 glyph — the row glyphs in `home.slint` already exist and already pick correctly by
  kind, so the list side needs no work. New widgets get their own type's default size
  (already correct in `canvas-storage`) and the title `UNTITLED`.
  **Done 2026-08-26 alongside SW-05**, which is what gave it a second type to offer.
  `NEW TODO` sits beside `NEW NOTE` in home's footer — `primary` for the note, `default`
  for the todo, so there is still exactly one primary action. The empty state now reads
  *"Press NEW NOTE or NEW TODO to create your first widget"* and the footer count says
  `N WIDGETS · N OPEN` rather than `N NOTES`. The palette's `New todo` action (⌘T) is
  live too — it was printing "todo widgets do not exist yet" until now.
  **The buttons carry labels, not glyphs.** §5.6's glyphs are already doing the work in
  the row list, where they distinguish types at a glance; a `Button` in this design system
  is a labelled control (§6.3) and there is no icon-button-with-label variant. Adding one
  would be a new component, not a create control.
  ⚠ **Superseded by SW-17 on 2026-08-26.** Two labelled buttons do not scale to seven types,
  and the user said so. The footer is now a strip of 16px glyphs with a hover readout, and
  the paragraph above — that a `Button` is a labelled control and glyphs belong in the row
  list — is the argument the user overruled. Everything else in this entry still holds.
  *DoD:* each type creatable from home, appears in the list with the right glyph, opens
  its own window, and survives a restart.
  ✅ **creation, opening and persistence** — confirmed 2026-08-26 headlessly: a seeded
  todo row loads, opens its own window with no error, and its three items read back
  intact through `--dump` after an app run. 🟡 **the two buttons and the row glyph have
  not been looked at.** In the verification queue.

- [ ] **SW-05 · Todo widget window** — `L`
  Needs: SW-02, SW-03, SW-04. `340×360`. Row gap 12, `Checkbox` + 12px gap. Completed rows:
  `dim` label + 1px strike. Due date as a `Tag` on the right, gap 8. Footer: 1px `edge`
  rule + `N OPEN · N DONE` in interface face 11 `dim`.
  **Counts are computed in Rust and passed in as `in property<int>`** — a Slint binding
  cannot run an imperative loop, that is a parse error not a runtime limit (§13.6).
  **Built 2026-08-26** as `ui/todo.slint` + `wire_todo` in `app.rs`. Same 32px header,
  glyph strip, drag region and three controls as the note (SW-16), because rule 3 says
  every widget wears the strip and nothing else reports state. Rows are a `TodoItemData`
  `struct` + `ModelRc` per §13.6; `open-count`/`done-count` come from
  `TodoListDoc::counts()` in Rust, never computed in a binding.
  **A compose row sits below the list** — a `plus` glyph and a `TextInput`, Enter adds and
  clears. Nothing in §10.1 says how items get created, and a modal or a floating button
  would both break the flat-plate language; an always-present input line is the smallest
  thing that works and it matches where the caret already is on a new note.
  **Double-click a row to rename it**, the same gesture and the same reason as the note's
  title (trap 6 — the row's existing `TouchArea` carries it, rather than a second one).
  Editing is driven by `editing-id` on the window so exactly one row can be in edit mode.
  Each row's `TouchArea` stops `todo-row-tail` (40px) short of the right edge so it never
  shares a region with the row's delete `IconButton`.
  *DoD:* add, toggle, edit and delete items; restart and they are all still there.
  ✅ **persistence across a restart** — confirmed 2026-08-26 headlessly: a todo seeded with
  three items (one done) opens in the app, and `--dump` reads all three back afterwards.
  `--dump` is now type-aware, so it reports any widget type through `WidgetDoc`.
  🟡 **add, toggle, edit and delete have not been driven by hand** — they need eyes and a
  pointer. In the verification queue.

- [ ] **SW-17 · A create control that scales past two types** — `M`
  Needs: SW-02. **Raised by the user 2026-08-26**, looking at home with two buttons on it:
  > *"the home page thing for todo is like okayish, since only two you did till now, but
  > what about other windows for each of kanban board, Calendar, etc etc — we dont have any
  > windows or options to use those, also in the command window too"*

  Two labelled 34px `Button`s already fill a good part of home's footer. **Seven will not
  fit** — `NEW SPREADSHEET` alone is ~150px against a 420px plate — and the command palette
  still offers only `New note`, `New todo` and `Settings` (§9's three defaults, written when
  three was the whole list).
  **Built 2026-08-26**, both decisions taken by the user when asked:
  **home's footer is a strip of 16px §5.6 glyph buttons**, one per built type — the row list
  already uses those exact glyphs to mean those exact types and the header cluster is
  already unlabelled icons, so it is a language already agreed to; and **an unbuilt type is
  hidden, not disabled**, so nothing is ever offered that does not open.
  Both surfaces read `canvas-app/src/widget_types.rs`, one `BUILT` table of
  `{kind, name, label, shortcut}`. Home renders a glyph per row; the palette renders
  `New <name>` with the kind as its icon key — which works because `PaletteGlyph` and home's
  `KindGlyph` were both already keyed on the kind string. Adding a type is one row.
  **The glyph strip has no labels, so it grew a hover readout**: a single interface-face-11
  `dim` line to the right of the strip that says `NEW` at rest and `NEW CALENDAR` while a
  glyph is hovered or keyboard-focused. Position never moves, only the string changes
  (§7.4), and it elides rather than colliding with the `N WIDGETS · N OPEN` count. It is
  driven by Slint's `changed hovered` / `changed focused` handlers per loop instance —
  **there is no way to aggregate `has-hover` across a `for` loop**, and a strip-wide
  `TouchArea` behind the buttons is exactly §13.3's trap.
  **Palette action ids are now `new:<kind>`**, resolved through `kind_for_action`, replacing
  the hand-written `ACTION_NEW_NOTE` / `ACTION_NEW_TODO` constants. A widget id can never
  collide with one, which is what keeps the `_` arm — open an existing widget — correct.
  **`CTRL+T` is shown against `New todo` but is not actually registered** — only Ctrl+K and
  Ctrl+N are (`hotkey.rs`). That is §9's own text and predates this task; `document` and
  `calendar` show no shortcut rather than inventing one.
  *DoD:* every built type is creatable from home and from the palette, both lists come from
  one place in Rust, and adding the next type touches that table only.
  ✅ **the "one place" half is enforced by tests, not by hope** — `home_and_the_palette_offer_the_same_list`
  compares the two rendered lists, and `a_kind_that_gains_a_document_must_be_added_to_this_table`
  fails the moment `WidgetDoc::load` learns a kind that `BUILT` does not carry. 143 tests pass.
  🟡 **the on-screen half is not watched** — the box stays open. See the VERIFICATION QUEUE.

- [ ] **SW-03 · The four missing library components** — `M`
  Needs: S1-13. `controls.slint` currently has `IconButton` and `Button` only.
  Build `Checkbox` (§6.4 — radius **2px**, not 8px: 8px on a 16×16 box renders as a disc,
  and there is **no tick glyph**, the fill *is* the state), `Tag` (§6.5), `Divider` (§6.7),
  `TextField` (§6.6).
  **Built 2026-08-25**, all four in `theme/controls.slint`, plus the §6 numbers they need
  as tokens in `theme/layout.slint` (`checkbox-size/gap/row-h`, `tag-h`, `tag-pad-x`,
  `field-h`, `field-pad-x`, `field-gap`). Every one is a number §6 already states — none
  invented. Labels use `Typography.body-face`, which the override table makes Silkscreen,
  not the spec's system sans.
  **`TextField` needs a hover border, and that looked like it collided with trap 6.** It
  does not: `TouchArea::input_event_filter_before_children` returns
  `ForwardAndInterceptGrab`, so a `TouchArea` **forwards pointer events to its own
  children** and only claims the grab if no child takes it. Trap 6 is about *sibling*
  areas overlapping, not a parent wrapping its child. So the `TextInput` sits inside the
  hover `TouchArea` and still gets its own clicks and caret placement.
  **The DoD needs somewhere to look, so `--gallery` exists.** `cargo run -p canvas-app --
  --gallery` opens a dev-only window showing all four in every state §6 lists, including
  the ones no real widget will reach for a while (`Tag` active, `TextField` error and
  disabled). It is a harness, not product UI, and it is the only way this DoD can be
  checked before SW-05 exists.
  *DoD:* each rendered against its §6 table by eye, in every state the table lists.
  🟡 **needs eyes** — the window opens and stays up, but nothing in it has been looked at.
  In the verification queue.

### Wb — Todo

- [x] **SW-04 · `TodoListDoc` in `canvas-doc`** — `M`
  Needs: S0-2. §11.2's array-of-maps pattern, which is already verified-compiling against
  yrs 0.19.2 — use it as written. `{id, text, done}` only; **do not** build `due` or
  `assignee` speculatively. **Remember the trap:** yrs indexes by UTF-8 byte offset, so any
  text indexing inside an item must walk to a `char_boundary`.
  **Done 2026-08-25** in `canvas-doc/src/todo.rs`, 16 tests. §11.2 used as written, with
  three additions its sketch does not cover: `set_text` and `set_done` return `None` when
  the value is unchanged (matching `StickyNoteDoc::set_text`, so a no-op never writes a
  revision row), `counts()` returns `(open, done)` because §13.6 says a Slint binding
  cannot loop, and `add_item` returns the new id alongside the update so the caller does
  not have to re-scan the array to find what it just made. **Trap 1 does not bite here** —
  an item's text is replaced wholesale via `map.insert`, never indexed into, so there is
  no byte offset to get wrong. It becomes live the moment SW-05 does per-keystroke diffing
  inside an item.
  *DoD:* unit tests matching `StickyNoteDoc`'s — two docs adding and toggling concurrently
  converge, snapshot round-trips, garbage bytes rejected without a panic.
  ✅ all three, and more — `cargo test --workspace`, 57 tests pass. Convergence is covered
  three ways: concurrent adds (nothing lost), a concurrent toggle against a rename of the
  **same item** (both survive), and a concurrent delete against an edit of a sibling.

### Wc — Shared Document

- [ ] **SW-06 · Shared Document widget** — `M`
  Needs: SW-02. `520×420`, **24px padding** (the only type that is not 16), paragraph gap
  16. Single-player this is a sticky note at a larger size with wider padding, so it reuses
  `StickyNoteDoc` and most of `sticky-note.slint`.
  **The collaborator cursors (§10.2) are NOT in this task** — they need a peer and land
  after Stage 3, via relative positions, never absolute offsets.
  **Done 2026-08-26, and it reuses the note component itself rather than copying it.**
  `StickyNoteWindow` gained `in property <length> pad` and `in property <string>
  delete-label`; every `Layout.pad` inside it became `root.pad`, and `Layout.title-x`
  became `root.pad + Layout.strip-w + 12px` so the title still sits 12px past the strip at
  either padding. §5.3 is explicit that padding drives **both** header and body — the glyph
  strip and the body text share a left edge — so a document could not simply have a wider
  body box. `KIND_DOCUMENT` therefore builds a `StickyNoteWindow` with `pad = 24` and
  `delete-label = "DELETE DOCUMENT"`, and maps to `StickyNoteDoc` in `WidgetDoc`. The
  `520×420` size already came from `canvas_storage::default_size`.
  *DoD:* creates, edits, persists. Cursor work explicitly recorded as deferred.
  🟡 **needs eyes** — clippy-clean, the app launches. **SW-17 closed the create gap on
  2026-08-26**: the doc glyph in home's footer strip and `New document` in the palette both
  make one, so this no longer needs seeding by hand. Cursors remain deferred to after
  Stage 3, as the task says.

### Wd — Calendar

- [x] **SW-07 · `CalendarDoc`** — `M`
  Needs: SW-04. Array of maps `{id, title, start, end, attendees}` (§11.3).
  **Done 2026-08-25** in `canvas-doc/src/calendar.rs`, 18 tests. §11.3's field names exactly.
  Decisions the spec leaves to the implementation, all recorded rather than assumed:
  **`start`/`end` are epoch milliseconds as `Any::BigInt`**, matching
  `canvas_storage::now_millis()` — deliberately **not** `Any::Number`, which is `f64` and
  would round a real timestamp; `int_field` still reads a `Number` if one ever turns up, so
  a hand-edited or foreign document degrades instead of returning 0.
  **`attendees` is `Any::Array` of strings.** **`events()` returns sorted by
  `(start, id)`** so SW-08's columns do not have to sort, and the tie-break on id keeps the
  order stable across peers. `duration()` is the input SW-08 needs for its 24/32/40/56px
  chip heights. `events_between(from, until)` selects anything **overlapping** the window,
  not merely contained by it, so an event straddling a day boundary appears in that day.
  `add_event` and `set_span` clamp `end` up to `start` rather than storing a negative span.
  *DoD:* same convergence and round-trip tests as `TodoListDoc`.
  ✅ all of them — `cargo test --workspace`, 75 tests pass. Convergence covered by
  concurrent adds, a three-way add, and a **concurrent move against a rename of the same
  event** (both survive).

- [ ] **SW-08 · Calendar widget window** — `L`
  Needs: SW-07. `480×360`. Seven equal day columns, gap 8. Day label interface-face-11
  `dim`; **today's label steps to `text` with a 1px `text` rule above it, and every other
  day gets a 1px `edge` rule in the same spot — it is a rule, not a wash.**
  Event chips: radius 8, height by duration (24/32/40/56px), fill from the peer-grey steps,
  vertical position = offset from column top.
  **Chips carry the 24-hour start time only, never the event name** — at a ~52px column an
  11px Silkscreen label holds five characters and a truncated name reads worse than none.
  Grey level carries duration. A day view is what is needed if names are ever required.
  **Finished 2026-08-26** — the half-built shell from the session before is now wired.
  `Calendar` arms in `WidgetDoc`/`WidgetWindow`, `wire_calendar` and `refresh_calendar` in
  `app.rs`, and a new `canvas-app/src/calendar_time.rs` holding the whole week/day/clock
  calculation with 12 tests.
  Decisions the spec leaves open, all recorded rather than assumed:
  **The week runs Monday→Sunday in LOCAL time.** `start`/`end` are epoch millis (UTC) per
  SW-07, so every day boundary has to cross a timezone. `local_offset_ms()` reads
  `GetTimeZoneInformation` — and note that **`TIME_ZONE_ID_UNKNOWN` (0) still carries a
  valid `Bias`**: it means "this zone has no DST transitions this year", which is exactly
  what India returns. Treating 0 as "no offset" silently put the whole grid 5½ hours out,
  and it was only caught because the seeded Monday landed on an exact UTC midnight.
  **`top` is a fraction of the day (0–1), not a pixel offset**, multiplied in Slint by the
  track height less the chip's own height. That is what makes a resize spread the chips
  instead of stranding them, and it keeps the last chip of the day fully on screen.
  Heights stay absolute because §10.4 fixes them at 24/32/40/56px.
  **Duration drives height and tone together**: ≤30min → 24px/tone 1 (darkest), ≤60 →
  32/2, ≤120 → 40/3, longer → 56/4 (lightest). `plate-tone(1)` is `peer-4`, so tone rises
  with duration and the longest event is the lightest plate — which is what the artboard
  shows for its 3-hour WED block.
  **An event straddling the week boundary is drawn from the week's own edge** — day 0,
  fraction 0 — but keeps its real start time as the label. `events_between` returns
  overlapping events, so this case arrives whether or not it is wanted.
  **The chip `TouchArea` and the `chip-activated`/`slot-activated` callbacks were removed.**
  Two chips at nearby times overlap, and overlapping `TouchArea`s are §13.3's documented
  trap. Nothing was reading either callback, so they were dead weight pointing at a design
  that does not exist yet — see **open question 10**.
  The day rollover is picked up by the existing 700ms pin poll, which compares the window's
  `today` against a freshly computed one and only refreshes when they differ.
  `cargo run -p canvas-app --example insert_calendar` seeds a week matching the artboard;
  `-- --remove` clears it.
  *DoD:* a week renders with correctly positioned and sized chips; today is marked; events
  survive a restart.
  🟡 **Not watched working — the box stays open.** What *is* confirmed: 134 tests pass and
  clippy is clean; the 12 `calendar_time` tests pin the week/day/clock maths; and the chip
  mapping was run end-to-end against the seeded document, printing
  `today=2` (Wednesday, 2026-08-26) and the artboard's exact eight chips —
  MON 09:30, TUE 11:00, WED 09:30 + 14:00, THU 09:30 + 16:00, FRI 13:00, SUN 18:00, SAT
  empty, with the 3-hour block at 56px/tone 4 and the stand-ups at 24px/tone 1.
  What is **not** confirmed is anything on screen. See the VERIFICATION QUEUE.

### We — Kanban

- [x] **SW-09 · `KanbanDoc` with fractional indexing** — `L`
  Needs: SW-04. Columns as an array of maps; cards keyed by id with a **fractional-index**
  position field. **Never nested-array reordering** — it is a known CRDT failure mode that
  duplicates and loses cards under concurrent moves, and it must not ship even as a
  single-player shortcut, because it would have to be rewritten at Stage 3 anyway.
  **Done 2026-08-25**: `canvas-doc/src/fractional.rs` (the index algorithm, 8 tests) and
  `canvas-doc/src/kanban.rs` (`KanbanDoc`, 22 tests). Cards live in a **`yrs::Map` keyed by
  card id**, so a card is one entry that two peers can only ever converge on — there is no
  array position to reorder and therefore nothing to duplicate. Columns stay a
  `yrs::Array` of `{id, title}`. A card is `{id, column, title, assignee, due, pos}`;
  `assignee` and `due` are there because §10.3's card meta line is `ASSIGNEE · DATE`, and
  `due` is epoch millis as `Any::BigInt` to match `CalendarDoc`.
  **The fractional index is hand-written — no new dependency** (Hard Constraint 6). Base-36
  digits `0-9a-z`, which sort lexicographically in the same order as their values, treated
  as the fraction after an implied decimal point. `key_between(lower, upper)` returns a key
  strictly between two neighbours, or `None` if the range is inverted or a key ends in `0`
  — the invariant the algorithm rests on. **No integer part is needed**: keys are always
  strictly inside `0 < k < 1`, so both ends can be subdivided forever.
  `move_card(id, column, index)` is the API a drop gives you — it finds the neighbours in
  the destination column itself, so callers never handle keys.
  *DoD:* unit tests — a card moved between columns keeps its identity; two docs moving the
  same card concurrently converge with no duplication and no loss.
  ✅ both, plus more — `cargo test --workspace`, 105 tests pass.
  `a_card_moved_between_columns_keeps_its_identity` checks the id, title **and** assignee
  survive the move; `two_docs_moving_the_same_card_converge_without_duplication_or_loss`
  moves one card to two different columns concurrently and asserts the boards match, the
  card exists exactly once, and the bystander card did not move. Also covered: concurrent
  moves of *different* cards both landing, a move against a rename, a **delete against a
  move** (the card is not resurrected), a three-way move, and 200 moves into the same gap
  staying strictly ordered.

- [ ] **SW-10 · Kanban widget window + card drag** — `L`
  Needs: SW-09. `640×400`. Three equal columns, gap 12. Column header interface-face-11
  `text` + count `dim`, gap 8, then a 1px `edge` rule. Cards: `elev-raised`, 1px `edge`,
  radius 8, padding 12, gap 8, title body-face-14, meta interface-face-11 `dim`
  (`ASSIGNEE · DATE` or `UNASSIGNED`). Dragged card gets `shadow-drag` **only** — no tilt,
  no scale (rule 5).
  **The drop target is a 1px `text` outline around the whole column. No fill, no tint.**
  Card drag needs disjoint `TouchArea`s per §13.3 — do not layer one over the column.
  **Built 2026-08-26.** `ui/kanban.slint`, `wire_kanban`/`refresh_kanban` in `app.rs`, and a
  `Kanban` arm everywhere `WidgetDoc`/`WidgetWindow` has one. Added to `BUILT` as `board`,
  so it is creatable from home's glyph strip and the palette the same day SW-17 landed.
  Decisions this task had to take, all recorded rather than assumed:
  **A new board is seeded with three columns — `TODO · DOING · DONE`.** `KanbanDoc::new()`
  makes an empty document with *no* columns, and §10.3 says "three equal columns" without
  naming them; the artboard's `SPEC · BUILD · SHIPPED` is that mockup's project content, not
  a product default. `KanbanDoc::seeded()` is the named constructor, and `WidgetDoc::seed()`
  is the general hook `App::create_widget` calls, so **the columns are written to disk at
  creation**, not lazily. That matters: if they were minted on each open, a board that was
  never closed would come back with **different column ids** and orphan every card.
  **The `+` row lives outside the board rectangle, not inside a column.** It is one
  `IconButton` per column in its own strip below the board, and the board's height is
  reduced to make room. Put inside the column it would eventually sit under a card once the
  column filled up — two overlapping `TouchArea`s, §13.3's trap, exactly the one that got
  the Calendar's chip handler deleted in SW-08.
  **The drop column is worked out in the card's own `moved` handler**, from the dragged
  card's centre against the column pitch, not from a second region. `col-at`/`row-at` are
  `pure function`s on the window so the geometry is stated once. **There is only ever one
  `TouchArea` per card**, and it stops short of the card's delete icon the way `ItemRow`'s
  does in the Todo window (`todo-row-tail`).
  **The window never touches a fractional key.** `move_card(id, column_id, index)` finds the
  neighbours in the destination column and mints the key itself, so the drop handler passes
  a column index and a row index and nothing else.
  **Card meta is `ASSIGNEE · DATE`, uppercased, falling back to `UNASSIGNED`** — §10.3's own
  wording. The date is `day_stamp`, new in `calendar_time`, built on Howard Hinnant's
  `civil_from_days`; it reads `26 AUG`, matching the artboard's `MIRA · 25 AUG`.
  🟡 **Cards overflow the bottom of a tall column and are clipped — there is no scroll.**
  §10.3 does not mention one and the `ScrollBar` component would need a per-column
  `Flickable`, which is its own task. Noted, not silently ignored.
  *DoD:* cards drag between columns, land in the right order, and survive a restart.
  ✅ the CRDT half — 150 tests pass, including three new ones that pin the seeding:
  a seeded board opens with exactly those three titles, its column ids survive a reload, and
  two separately seeded boards never share a column id.
  🟡 **the drag has not been driven by hand, and it is the whole point of the task.** The box
  stays open. See the VERIFICATION QUEUE.

### Wf — Whiteboard

- [x] **SW-11 · `WhiteboardDoc`** — `L`
  Needs: SW-04. Map of shapes `{type, x, y, w, h, rotation, style}` + arrays of point lists
  for strokes. **Strokes must be appended incrementally as points arrive**, never
  re-encoded as a full canvas — that shape is what makes Stage 3 traffic scale with stroke
  length instead of canvas size, so it has to be right from the start.
  **Done 2026-08-26** in `canvas-doc/src/whiteboard.rs`, 17 tests. §11.3's shape exactly:
  `shapes` is a `yrs::Map` keyed by shape id, `strokes` is a `yrs::Map` keyed by stroke id
  with a `yrs::Array` of points inside each. **Points are stored flat — `x, y, x, y, …` in
  one array, not a map or a nested array per point.** A point is then two array appends and
  nothing else, which is what keeps an appended point's update near its own size; a map per
  point would carry key strings for every sample of a freehand line.
  `begin_stroke` / `extend_stroke(id, &[Point])` is the API a pointer-move handler wants —
  a stroke opens once and grows, so nothing ever re-sends what is already drawn.
  *DoD:* unit tests — appending 1000 points produces updates proportional to the points
  added, not to the canvas contents.
  ✅ both halves, tested separately, because they are different claims.
  `appending_1000_points_costs_traffic_proportional_to_the_points_added` grows a stroke to
  1000 points and asserts one further point costs less than a twentieth of the whole-canvas
  snapshot, and that ten points cost under twelve times one.
  `stroke_traffic_does_not_grow_with_the_canvas` draws the same ten points on an empty
  board and on one holding 200 shapes and 4000 other stroke points, and asserts the cost
  barely moves while the snapshot is more than twenty times either. Also covered: two peers
  drawing at once keeping both strokes whole, and **two peers extending the same stroke
  losing no points** (20 points survive, none duplicated).


- [ ] **SW-12 · Whiteboard widget window**, **SW-13 · `SheetDoc`**, **SW-14 · Spreadsheet
  widget window** — all three **moved to Stage W2 below**, rewritten on 2026-08-26 with the
  user's answers to open questions 4, 5, 6 and 7. Do not build from this section.

## Stage W2 — the rework the user asked for on 2026-08-26

The user walked the running app and answered ten questions. Three things came out of it:
**two live bugs**, **a whole editing layer that did not exist** (§10.3 and §10.4 specify
readouts and no way to put anything into them), and **the Spreadsheet unblocked**. The
Whiteboard is explicitly deferred.

Work these in order. `SW-22` first because it is small and it is the same root cause that
would otherwise be re-introduced three more times.

### W2a — the bug that is already shipped

- [ ] **SW-22 · Typing into a repeater row dies after one character** — `S`
  Needs: nothing. **Reported by the user 2026-08-26**, on the Kanban:
  > *"each card is only letting me type a single letter and nothing more than that"*

  **Root cause, and it is not specific to the Kanban.** Every keystroke fires
  `on_card_title_edited` → `kanban_edit` → `refresh_kanban`, which builds a **new
  `VecModel` and calls `set_cards`**. Slint's repeater tears down and rebuilds its items,
  so the `TextInput` the cursor was in is destroyed and recreated with `text: card.title`.
  Focus is lost and the next keypress goes nowhere.
  **The Todo widget has the identical shape** — `on_item_text_edited` → `todo_edit` →
  `refresh_todo` → `set_items`. It has never been watched, so assume it is broken the same
  way and fix both in this task.
  **The fix is to stop rebuilding the model while a row is being edited.** Write the
  keystroke into the CRDT (that part is right and must not change — Hard Constraint 2 says
  the local doc applies first), but skip the `set_*` call whenever `editing-id` is non-empty,
  and do one refresh on commit. Do **not** fix it by debouncing or by making the model
  mutable in place — a repeater rebuild under a live caret is the thing to avoid, not to
  make faster.
  *DoD:* type a full sentence into a Kanban card and into a Todo row without losing the
  caret; the text is in the document afterwards; a restart still shows it.
  ⚠ A regression test is hard here (it needs an event loop). At minimum add a test that
  `editing-id` suppresses the refresh path, and put the click-through in the queue.
  🟡 **Built 2026-08-26, not watched.** `rows_locked_by_editor` guards `refresh_todo` and
  `refresh_kanban`: the CRDT write still happens on every keystroke, but the `set_items` /
  `set_columns` + `set_cards` calls are skipped while `editing-id` is non-empty, and
  `edit-committed` now clears the id **and then refreshes**, which is the one rebuild the
  row gets. Add/toggle/move/remove call `end_*_edit` first, so a structural change is never
  swallowed by a live caret. Box stays open until the click-through below passes.

### W2b — the editing layer

- [ ] **SW-18 · `Sheet` — the in-widget overlay every editor uses** — `M`
  Needs: SW-03 (`TextField`, `Button`). **Answering open question 10**, user 2026-08-26:
  > *"overlay inside widget but seamless and awesome"*

  A form that slides over the widget's own plate. **Not a separate window and not a modal
  over the desktop** — §7.6 state 10 already establishes the in-widget pattern for delete
  confirm ("no modal, no desktop dimming") and this is the same idea at full height.
  Anatomy, all from tokens that already exist:
  - Covers the plate **below the 32px header band**, so the glyph strip, title and the
    three controls stay visible and the widget still reads as itself.
  - Fill `plate` **opaque** — it must fully hide the board/grid behind it, not veil it.
    No scrim, no blur, no dimming (rule 5).
  - 1px `edge` hairline along its top, flush with the header rule already there.
  - Opens over 160ms, opacity 0→1 **plus the 6px rise** (§5.5's widget-open values — reuse
    `Motion.open` and `Motion.open-rise`, do not invent a new duration). Closes at 100ms,
    opacity only.
  - Body: a `VerticalLayout` of `TextField`s (§6.6) at 16px padding, `field-gap` between.
  - Footer row pinned to the bottom: `SAVE` (`primary`) + `CANCEL` (`default`), gap 8, and
    `DELETE` (`destructive`) **right-aligned and only when editing something that exists**.
  - `Esc` cancels, `Enter` in any single-line field saves. Focus starts in the first field.
  **The sizing decision, made here so nothing re-derives it:** five fields plus a button row
  is ~320px of content and the Calendar plate is 360px tall. **The sheet scrolls** — wrap
  the field column in a `Flickable` with the existing `ScrollBar` from `theme/controls.slint`
  (the same 4px thumb the note body and home list use). **The widget must not resize itself
  to fit the form** — a window that grows under the pointer is a surprise, and §7.3 gives
  the user the resize border if they want more room.
  Build it generic — `in property <[SheetField]>` is tempting but a typed component per
  editor is less indirection; what belongs in `Sheet` is the **chrome**: the slide-over
  rectangle, the hairline, the scroll, the footer buttons and the key handling.
  *DoD:* a throwaway sheet with three fields opens over the Kanban, scrolls, saves, cancels
  on `Esc`, and never lets the board behind it show through.
  🟡 **Built 2026-08-26, partly watched.** `theme/sheet.slint`, chrome only: the slide-over
  rectangle, the top hairline, the scrolling field column, the footer buttons and the key
  handling. Callers pass their own typed `TextField`s as `@children` and set `base-y` to the
  bottom of their header band. Two contracts a caller must honour, written here so SW-19,
  SW-20 and SW-21 do not each rediscover them:
  1. **Wire `accepted` on every field to the sheet's `saved()`.** `TextInput` consumes
     `Return` itself, so Enter cannot be caught by an ancestor. `Escape` bubbles and the
     sheet catches it.
  2. **Disable the TouchAreas behind the sheet while it is open.** An opaque rectangle does
     not block a `TouchArea` under it (§13.3), so the board would still take clicks through
     the sheet. `enabled: !sheet-open` on the board, the same way `confirming-delete`
     already gates things.
  The demo is in `cargo run -p canvas-app -- --gallery`, over a mock Kanban plate at the
  real 640×400 with three columns and a card behind it, rather than a throwaway inside the
  real Kanban that SW-21 would delete two tasks later.
  **Photographed: the sheet covers the plate below the header band, the glyph strip and
  title stay visible, the board behind is completely hidden, the five labelled fields
  scroll with the 4px thumb, and the footer reads `SAVE` / `CANCEL` with `DELETE CARD`
  right-aligned in `signal`.** Opening motion, `Esc`, save and drag-scroll still need
  clicks — the queue lines below.

- [ ] **SW-19 · Date and time pickers in this design language** — `L`
  Needs: SW-18. **Answering the user's Q3, 2026-08-26**, who sent two Android screenshots
  (Material 3 clock face and month grid) and said:
  > *"i want time to be selected like android widget, like in the images, use how you want
  > but style should be matching our app"*

  So: **that interaction, none of that skin.** The reference images are purple-on-grey with
  large type; both of those are ours to replace.
  **`TimePicker`** — the Material layout, monochrome:
  - Two tapping targets at the top, `HH` and `MM`, split by a `:`. The **selected** one is
    filled `text` with `base` numerals; the other is `elev-recessed` with `text` numerals,
    1px `edge`. This is the checkbox language (§6.4) — *the fill is the state*, no accent.
  - **Numerals at 22px interface face.** The reference shows them much larger; §5.2 allows
    the interface face at **11px and 22px only** and no new size gets invented (§5). 22px is
    the ceiling and it is what the widget title already uses.
  - A circular face below: 24-hour double ring like the screenshot — outer ring `00`–`11`,
    inner ring `12`–`23` — labels at 11px interface face `dim`. The selected hour is a
    filled `text` disc with a `base` numeral; the hand is a **1px `text` line** from centre
    to that disc, and a 3px `text` dot at the centre (the dot-cell shape again).
  - Minutes ring in 5s, same treatment, with in-between minutes snapping to the nearest
    label but keeping their real value in the readout.
  - Footer: `CANCEL` (`default`) + `OK` (`primary`), gap 8. The reference's keyboard-entry
    toggle is worth having as a fourth thing later; not in v1.
  **`DatePicker`** — the month grid:
  - Header: the full date in 22px interface face (`WED, 26 AUG`), a 1px `edge` rule under it.
  - Month name + year at 11px interface face, `‹` and `›` stepping months. Use the existing
    16px glyph geometry rather than a new chevron if one fits; otherwise this is the **one**
    place a new §5.6 glyph is justified — draw it on the 16-unit grid, 1.5px, square caps.
  - `S M T W T F S` column heads at 11px interface face `dim`.
  - Day numbers at **body face 14** — they are content, not labels.
  - **Today** gets the Calendar widget's own treatment: label steps to `text` with a 1px
    `text` rule above it (§10.4), so the two surfaces agree.
  - The **selected** day is a filled `text` disc with a `base` numeral. Nothing else fills.
  ⚠ **Verify before writing the face:** the circular layout needs `sin()`/`cos()` in Slint
  expressions. Confirm they exist in **1.17.1** and what unit they take (`deg` vs `rad`)
  before building on them — §4.1's rule. If they are missing, precompute the 12/24 label
  positions as a constant table in Rust and pass them in; do not fake a circle with a
  rotated `HorizontalLayout`.
  *DoD:* pick a time and a date, both round-trip to epoch millis correctly across a
  timezone (reuse `calendar_time`, do not write a second date conversion), and neither
  picker introduces a colour, radius, duration or font size not already in §5.

### W2c — Calendar, made usable

- [ ] **SW-20 · Calendar: event editor, live event, and the info bar** — `L`
  Needs: SW-18, SW-19. **Answering open question 10**, user 2026-08-26:
  > *"they should be able to click on a day or something and then it should open a popup of
  > similar style of the rest of app to ask us like google calendar, name, timings, location
  > etc etc all important stuff, and then all will be placed exactly, we can have sparky or
  > highlighted card if currently that is going on based on the time and day and like at the
  > top or bottom of the widget we can see the info, like name, timings and what time is
  > left etc etc, something really cool"*

  **Fields: `name`, `start`, `end`, `location`, `notes`.** Confirmed by the user. `name`,
  `start` and `end` are required; `location` and `notes` are optional and render nothing
  when empty. `CalendarDoc` already stores `{id, title, start, end, attendees}` — **add
  `location` and `notes` as string fields**; `attendees` stays in the document unused rather
  than being removed, since §11.3 names it and Stage 3 will want it.
  **Creating:** click anywhere in a day column → the sheet opens with `start` pre-filled to
  the clicked position rounded to the nearest 30 minutes on that day, and `end` at +1 hour.
  Clicking an existing chip opens the same sheet on that event, with `DELETE` shown.
  ⚠ **Hit-testing: one `TouchArea` per day column and resolve chip-vs-empty in Rust from the
  pointer's y, or the chips get their own areas and overlap each other.** Chips at nearby
  times *do* overlap — that is why SW-08 deleted the chip handler in the first place. This
  is §13.3 and it is the single most likely thing to be got wrong here.
  **The live event.** An event whose `start ≤ now < end` on today's column gets:
  a **`signal` 3×3 dot** at the chip's leading edge, the same 3×3 cell the glyph strip uses,
  plus the chip's border stepping from `edge` to `text`.
  ⚠ **This is a deliberate override of rule 2** ("red appears twice in the whole product").
  The user was told and asked for it anyway — 2026-08-26, *"can use red etc to show the dot
  or something unique, i want it to be visually beautiful"*. It is **override 15** in
  `progress.md`. Do not extend it: red on the live dot, the error strip and the destructive
  button, and nowhere else.
  **The info bar** sits **at the bottom of the plate**, above the 16px padding, 1px `edge`
  rule above it — the same slot and the same rule as the Todo widget's `N OPEN · N DONE`
  footer, so the two widgets agree. Two states:
  - Something running: `NOW` in `text`, then the name, then `09:30–10:00`, then
    `12 MIN LEFT`, all 11px interface face. `NOW` and the name at `text`, the rest `dim`.
  - Nothing running: `NEXT UP` in **`dim`** rather than `text`, then the same three parts.
    That is the *"slightly different visual"* the user asked for — the label's weight of
    attention changes, nothing moves and nothing else recolours.
  - Nothing today and nothing ahead this week: the bar holds `NOTHING SCHEDULED` in `dim`.
  It must **tick** — the minutes-left number is wrong the moment it is painted otherwise.
  Reuse the existing 700ms poll in `wire_calendar` (which already recomputes `today`);
  refresh the bar when the displayed minute changes, not every tick.
  *DoD:* create an event by clicking a day, it lands on the right day at the right height,
  survives a restart, is editable and deletable from the same sheet; a currently-running
  event shows the red dot and the outline; the bar counts down and flips to `NEXT UP`.

### W2d — Kanban, fixed and finished

- [ ] **SW-21 · Kanban: drag that actually tracks, and a card editor** — `L`
  Needs: SW-18, SW-19, SW-22. **Three defects the user reported 2026-08-26:**
  > *"its too glitchy to move the cards around etc and the text alignments etc too and its
  > not smooth at all to move the cards around"*

  **(1) The drag. Root cause is known and it is the same one `S1-51` already fixed once for
  window drag.** The card's `TouchArea` is *on the card*, and the card moves by `held-dx`.
  So `self.mouse-x - self.pressed-x` measures the pointer against a target that is running
  away from it — the delta feeds back into the thing it is measured from and the card
  stutters and drifts.
  **The fix: the touch region must not move.** Leave the card `Rectangle` at its model
  position (drop it to `dim` or hide it while held) and draw a **separate floating card at
  board level** that follows the pointer. The `TouchArea` stays where it was, so
  `mouse-x - pressed-x` stays honest for the whole gesture. Do **not** try to fix this by
  anchoring to `absolute-position` or by accumulating smaller deltas — trap 4 and `S1-51`
  are both about exactly that and it is written down twice already.
  **(2) Text alignment — the user says the column header is what looked wrong.** Check the
  title/count row: the count is right-aligned into a `Text` whose width is intrinsic, and
  the title's width subtracts `count-text.width`, so a two-digit count shifts the title. Also
  check the card title against its 22px line box and the meta against its 16px one.
  **(3) The card editor.** Same `Sheet` as the Calendar: **`title`, `assignee`, `due date`,
  `description`** — confirmed by the user. Opening it replaces the inline `TextInput`
  entirely, which also retires the single-letter bug on this widget (`SW-22` still has to fix
  the Todo). `KanbanDoc` has `set_title`/`set_assignee`/`set_due` already; **add
  `description`**. The meta line keeps §10.3's `ASSIGNEE · DATE` / `UNASSIGNED`.
  *DoD:* drag a card across three columns without a stutter and drop it exactly where the
  outline says; the column header holds still as counts change; a card's four fields are
  editable from the sheet and survive a restart.

### W2e — Spreadsheet, unblocked

Open questions 5, 6 and 7 were answered by the user on 2026-08-26. All three answers are
folded into the two tasks below; the questions are closed.

- [ ] **SW-13 · `SheetDoc`** — `L`
  Needs: SW-04. **Answer to open question 6 — the one that decides the shape:**
  > *"yes can do anything possible"* — rows and columns can be inserted **mid-sheet**.

  So a cell can **not** be keyed by `(row_index, col_index)`: inserting row 3 would have to
  rewrite every key below it, and two peers inserting at once would collide. The shape is:
  - `rows`: a **`yrs::Array` of maps `{id}`** in visual order. Insertion anywhere is an array
    insert, and `yrs::Array` is already a sequence CRDT — **this is why no fractional index
    is needed here**, unlike the Kanban, where cards live in a `Map` precisely because they
    move *between* parents. Rows and columns never change parent, only position.
  - `cols`: the same.
  - `cells`: a **`yrs::Map` keyed by `"<row_id>:<col_id>"`** → `{value}`. A cell's key never
    changes when the sheet is reshaped, which is the whole point.
  - `merges`: a `yrs::Map` keyed by the **anchor** cell key → `{rows, cols}` span. Covered
    cells keep their values in `cells` and are simply not painted.
  - `col_w` / `row_h`: `yrs::Map`s keyed by col/row id → pixels. Absent means the default.
  **Answer to open question 5:** *"for now let it be just normal values and later we can
  expand to formulas etc"* — **values only.** Store `value` as a plain string and do not add
  a `formula` field speculatively; the shape above takes one without disturbance later.
  *DoD:* insert a row in the middle of a filled sheet and every cell below keeps its value ·
  two docs inserting rows at the same index converge with no loss · a merge survives a
  reload · deleting a row does not orphan a merge anchored on it.

- [ ] **SW-14 · Spreadsheet widget window** — `L`
  Needs: SW-13, SW-18. **Answer to open question 7:**
  > *"can have something decent enough to have 6-7 rows and columns and of course should be
  > able to merge them or expand the cells and correspondingly the size of widget as well"*

  **Default `640×360`, a `7×7` grid** — 7 columns at 80px plus a 36px row gutter is 596,
  which inside 16px padding wants a 628px plate; 640 is the number already on the scale and
  already used by the Kanban. 7 rows at 28px under a 24px column-head band fits 360 with the
  48px header. Add `sheet-w`/`sheet-h`/`sheet-col-w`/`sheet-row-h`/`sheet-gutter-w` to
  `theme/layout.slint`; invent no numbers elsewhere.
  - Row/column headers: interface face 11 UPPERCASE `dim`, `A B C…` and `1 2 3…`.
  - Cell contents: body face 14/22.
  - **The selected cell is a 1px `text` outline — no fill, no tint**, matching §10.3's
    Kanban drop target. There is no accent colour to select with.
  - The editing cell reuses `TextField`'s focus treatment from SW-03.
  - Scrolling: `Flickable` plus the existing `ScrollBar`.
  - **Merge and resize are in scope for v1** — the user said *"do all those now only"*.
    Merge: select a range, a `MERGE` control in the sheet's own footer or on a right-click
    equivalent. Resize: drag the boundary in the row/column header band.
  ⚠ **Cell hit-testing must use ONE `TouchArea` over the grid** that resolves the cell from
  `mouse-x`/`mouse-y`, never one per cell — 49 cells is fine, a scrolled sheet is thousands,
  and that is §13.3's failure mode at scale plus a per-cell allocation the 5–15 MB budget
  will not absorb. The **column-boundary resize handles are the exception that needs care**:
  give them their own thin band in the header, disjoint from the grid area, the same way
  SW-10's `+` row had to move out of the columns.
  *DoD:* type into cells, navigate with arrows/Tab/Enter, merge a range, drag a column
  wider, resize the window, restart — contents, merges and geometry all intact.

### W2f — deferred by the user

- [ ] **SW-12 · Whiteboard widget window** — `L`
  **Deferred 2026-08-26 at the user's request** — *"for now ignore whiteboard, it would be
  done later, no need of that right now"*, and the same for its tool set.
  `WhiteboardDoc` is built and tested (SW-11) and stays unreachable from the app; it is
  deliberately absent from `BUILT` in `widget_types.rs`. **Open question 4 stays open** — it
  still has no artboard, and that is the first thing to settle when it comes back.

### W2g — making a session able to check its own work

- [x] **SW-23 · `--open` dev flag, so a session can put a widget on screen to photograph** — `S`
  Needs: nothing. **Screenshots now work** (see the recipe in `progress.md`'s START HERE),
  but a session can only photograph what is already on screen, and the only ways to open a
  widget are a mouse click on home or the palette — neither of which a session has.
  Add `cargo run -p canvas-app -- --open <widget-id>` and `--open-kind <kind>` (creating one
  first if none of that kind exists), which set the open flag and launch normally.
  This is the difference between "I built a Kanban and cannot see it" and "here is a picture
  of the Kanban". It should be built **before** SW-20 and SW-21, not after, because both of
  those are visual tasks whose whole risk is on-screen.
  *DoD:* `--open-kind kanban` launches with a board on screen; the screenshot recipe
  captures it; the flag never changes what a normal launch does.
  ✅ **Done and watched 2026-08-26.** `OpenRequest::from_args` parses both flags before the
  backend is chosen, `App::boot` takes the request and `honour_open_request` resolves it to
  one widget id — creating and seeding one if that kind has none — then sets its open flag,
  so the existing boot loop does the opening and no second code path exists. `--open-kind`
  takes either word for a type (`kanban` or `board`). All four paths were run:
  `--open-kind kanban` put the board on screen and it was photographed;
  `--open-kind whiteboard` was refused with the list of built types; `--open-kind` with no
  value said what it wanted; `--open <unknown-id>` printed `--open ignored` and launched
  normally. No flag, no behaviour change.
### W2h — the user's call, mid-run on 2026-08-26

- [ ] **SW-24 · Rip out the `document` widget type** — `S`
  Needs: nothing. **The user, watching SW-18 land:**
  > *"i actually think there is no point of having document widget and all its code as note
  > is the same as it rightyy?? answer me if no difference rip it off completely"*

  **They are right, and the answer they were given was: there is no difference worth a type.**
  `KIND_DOCUMENT` shares `StickyNoteDoc` and `StickyNoteWindow` with the sticky note. The
  only three things that differ are `set_pad(24px)`, a `DELETE DOCUMENT` label, and a
  520×420 default size. That is a preset, not a widget type.
  §10.2's real Shared Document — paragraph gap 16, collaborator cursors on **relative
  positions** — is Stage 3 work and none of it exists. When it comes back it comes back as
  its own window with its own document, not as three property overrides on a note.
  **Their database has no `document` rows** (checked by `--dump` on 2026-08-26), so nothing
  of theirs is lost. A row of that type arriving from somewhere else must still not crash
  the app: leave it falling through to "no document type for this widget", the same path
  every unbuilt kind already takes.
  Remove: the entry in `widget_types::BUILT`, the `KIND_DOCUMENT` arms in `WidgetDoc::load`
  and `WidgetWindow::create`, `DOC_PAD` and the delete-label override, the document glyph
  mapping in home and the palette, the `document` default size in `WindowPlacement`, and
  `KIND_DOCUMENT` itself if nothing else needs it. `widget_types`' own tests already assert
  that `BUILT` and the set of kinds with documents agree, so they will catch a half-removal.
  *DoD:* home's footer strip and the palette both offer four types, not five; the tests
  still pass; a fresh launch still opens every remaining type.

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
- ~~Do the other widget types need their own header controls?~~ **Answered 2026-08-26:
  three controls on every type — `pin · delete · close` — and the colour control is
  DROPPED, including from the sticky note.** The user chose this knowing it regresses a
  control they had previously confirmed working; that explicit word is what the DO NOT
  REGRESS floor requires, and it is recorded in the override table in `progress.md`.
  Consequences already applied in **SW-16**: `Layout.cluster-w` is **64px**, not 88 —
  every header layout derives from it, so nothing else needed moving. `Palette.plate-tone`
  and the `tone` column in `window_positions` are **kept**: a note already carrying a tone
  still renders it, there is simply no control to change it. Reinstating colour means
  putting `colour-btn` back and returning `cluster-w` to 88px.

**Still open**

1. **What should pin do beyond the desktop layer?** Every note is now non-floating, so
   pin's only current effect is the bottom z-order. Should a pinned note also be exempt
   from close, or always reopen at launch? *Blocks:* nothing yet; decide by S1-27.
2. **Is light mode in scope, or dark only?** Both token sets are built and the three
   documented light corrections are applied. The open part is only whether a switcher
   ships, and where its control lives now that a home window exists.
3. **`handoff/screenshots/light-mode.png` contains "the written verdict on what breaks"**
   (§15) and has still not been read. Read it before shipping light mode.
4. ~~**Does the Whiteboard have a design?**~~ **DEFERRED by the user, 2026-08-26** —
   *"for now ignore whiteboard, it would be done later, no need of that right now"*, and the
   same answer for its tool set. The question itself is **still unanswered** and still real:
   §10.5 describes the Whiteboard in one paragraph and there is **no artboard** for it —
   `widget-types.png` covers Todo, Shared Document, Kanban and Calendar only. Tool dock
   contents, shape set and stroke style remain unspecified.
   *Blocks:* SW-12, which is parked. **Ask again when the Whiteboard comes back; do not
   invent a tool palette in the meantime.**

5. ~~**Does the Spreadsheet support formulas?**~~ **ANSWERED 2026-08-26** —
   *"for now let it be just normal values and later we can expand to formulas etc"*.
   **Values only.** `SheetDoc` stores `value` as a plain string and gets **no** speculative
   `formula` field; the cell shape in SW-13 takes one later without disturbance. Folded into
   SW-13. Closed.

6. ~~**Can Spreadsheet rows and columns be inserted and deleted mid-sheet?**~~
   **ANSWERED 2026-08-26** — *"yes can do anything possible"*. **Mid-sheet insertion and
   deletion are in.** This was the one that decides the CRDT shape, and it rules out keying
   cells by `(row_index, col_index)`: rows and columns become `yrs::Array`s of `{id}` and
   cells a `yrs::Map` keyed by `"<row_id>:<col_id>"`, so a cell's key survives any reshape.
   Full reasoning and the merge/size maps are in SW-13. Closed.

7. ~~**What is the Spreadsheet's default size?**~~ **ANSWERED 2026-08-26** — *"something
   decent enough to have 6-7 rows and columns and of course should be able to merge them or
   expand the cells and correspondingly the size of widget as well"*.
   **`640×360`, a `7×7` grid, with merge and drag-resize in v1** (*"do all those now only"*).
   The arithmetic behind 640×360 is in SW-14. Closed.

8. **What should pin do beyond holding a widget at the desktop layer?** Asked 2026-08-26;
   the user said *"i have no idea yet"*. Still open, still not blocking anything.

9. **How should the palette search note bodies — a `text` column on `widgets`, or FTS5?**
   Asked again 2026-08-26; the user said *"i have no idea yet"*, so the recommendation
   stands unclaimed. S1-34 matches titles only. Matching bodies the obvious way costs a
   `get_document` plus a `StickyNoteDoc::load` for **every** note **on every keystroke**,
   which is the disk-I/O-on-the-typing-path that Hard Constraint 2 and trap 7 exist to stop.
   **(a)** a plain `text` column on `widgets`, written by the same 500ms flush that already
   writes the snapshot — a few lines, no new table, and it doubles as the home list's
   preview source; or **(b)** an FTS5 virtual table, a real index that scales to thousands
   but is a second copy to keep in step and needs an extension the bundled SQLite may not
   have compiled in. **(a) is still the recommendation** — 200 notes is the stated target.
   *Blocks:* S1-48. **Ask before starting.**

10. ~~**How does a Calendar event get created, moved or deleted from inside the widget?**~~
   **ANSWERED 2026-08-26**, and it grew into most of Stage W2. The user asked for a
   **click-a-day → in-widget overlay editor** (*"overlay inside widget but seamless and
   awesome"*) with **name, start, end, location, notes**; **Android-style date and time
   pickers restyled to this design language**; a **live-event highlight**; and a **bottom
   info bar** showing what is on now or next and how long is left.
   Split across **SW-18** (the `Sheet` overlay), **SW-19** (the two pickers) and **SW-20**
   (the Calendar work itself). The same `Sheet` covers the Kanban card editor in **SW-21**.
   Closed.

11. **Is `CTRL+T` meant to exist?** The palette prints `CTRL+T` against `New todo` but only
   Ctrl+K and Ctrl+N are registered in `hotkey.rs`. It is §9's own text, so the label is not
   a mistake — the missing registration might be. Asked 2026-08-26; the user said *"i have
   no idea yet"*. Either register it or drop the label; do not leave it printed and dead
   once someone decides. Not blocking.
---

## Session-end checklist

Before a session ends — especially one running low on context:

1. `TASKS.md`: boxes ticked, **Current task** line updated.
2. `progress.md`: the 🟢 GO block rewritten to name the exact next action, and a dated
   entry appended with honest `✅` / `🟡` / `❌` markers.
3. `git commit` — the working tree should never be left dirty at a session boundary.
4. Anything half-finished: leave a `// TODO(<task-id>)` in the code **and** a note under
   that task here.
