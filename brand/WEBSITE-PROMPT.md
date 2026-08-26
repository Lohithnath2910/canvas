# Website build prompt

**NAME: `TACK`** — change here and search-replace if you renamed.

**How to use this file:** paste everything between the `═══` markers into Claude (or
another coding AI) as a single message. It is written to be self-contained — the receiving
model has never seen this repo. Do not paste this header.

---

═══════════════════════ START OF PROMPT ═══════════════════════

# Build the marketing site for TACK

You are building a single-page marketing website for a desktop app called **TACK**. Read
this entire brief before writing code. The design system is unusual and deliberate — the
most likely way to fail here is to build a competent, generic SaaS landing page. That would
be a failure even if the code is perfect.

## 1. What TACK actually is

TACK is a **native desktop widget platform**, written in Rust, that puts small always-on-top
panels on your desktop. Sticky notes, todo lists, documents, calendars, kanban boards.

Three things make it not-another-notes-app:

1. **A widget is not a window — it is a readout.** Each panel is a flat monochrome plate
   that shows one thing and reports its own state. There is no chrome, no toolbar, no menu
   bar. The panel *is* the content.
2. **It is native, not a browser.** No Electron, no Chromium, no webview. A single widget
   uses 5–15 MB. The whole app idles at 20–40 MB and cold-starts in under 300ms. Compare
   that to any Electron notes app, which starts at 200 MB.
3. **It is offline-first and CRDT-backed.** Edits apply locally and paint before anything
   touches a network. Two machines editing the same note converge without conflicts and
   without a "last write wins" data loss. There is no save button anywhere — persistence is
   automatic and invisible.

Positioning line to work from (do not use verbatim, write something better):
*"Your desktop, instrumented."*

**Be honest in the copy.** Multiplayer sync is architecturally in place (the CRDT layer is
built and tested) but the networking is not shipped yet. Do not claim real-time
collaboration works today. Say what is true: the data model is built for it.

## 2. The design system — this is the whole job

TACK's identity is: **black, white, one red, dot-matrix type, and light that reports state
instead of icons that describe it.** Reference points are Nothing OS, Braun instrumentation,
split-flap departure boards, and monochrome LCD test equipment.

**The site must look like the product.** Someone who visits the site and then opens the app
should not experience a change of language.

### Five rules — breaking any one is a failed build

1. **Monochrome plus one red.** If something seems to need a second hue, it needs a
   different layout instead.
2. **Red (`#D71921`) appears at most twice on the entire page.** Never as an accent, never
   as a link colour, never as a focus ring, never as decoration. It marks one thing.
3. **Dot cells are binary** — on, dim, or off. Never a fade, never a tween between states,
   never a glow.
4. **Flat and opaque.** No blur, no gradient, no glassmorphism, no soft shadows, no glow,
   no tilt, no scale-on-hover, no bounce.
5. **The display typeface renders at exactly two sizes: 11px and 22px.** Never any other
   size, never below 11px. This is not a suggestion — it is what makes the product look
   like an instrument rather than a website.

### Tokens — use these exact values, invent none

```css
--base:          #000000;   /* page ground */
--plate:         #121212;   /* every card/panel surface */
--text:          #FFFFFF;
--dim:           rgba(255,255,255,0.52);   /* secondary text */
--edge:          rgba(255,255,255,0.14);   /* every hairline and border */
--signal:        #D71921;   /* at most twice on the page */
--elev-raised:   #1C1C1C;   /* button rest fill */
--elev-recessed: #0C0C0C;   /* input fill */

/* grey steps, for panel variety only */
--grey-1: #3A3A3A;  --grey-2: #2E2E2E;  --grey-3: #242424;  --grey-4: #1C1C1C;

--radius-control: 8px;
--radius-plate:   20px;
--radius-overlay: 28px;
--radius-pill:    999px;

/* spacing scale — use nothing else */
4px, 8px, 12px, 16px, 24px, 32px, 48px

--ease: cubic-bezier(0.2, 0, 0, 1);   /* the ONLY easing curve on the site */
```

**Dark mode is the only mode.** Do not build a light theme or a theme toggle. This product
is black.

### Typography

**Display / interface face: Silkscreen** (Google Fonts, SIL OFL, Regular only — there is no
bold). Self-host it; do not hotlink Google Fonts.

| Role | Size / line | Treatment |
|---|---|---|
| Section heading, hero headline | **22px / 28px** | UPPERCASE, `letter-spacing: 0.06em` |
| Label, metadata, nav, button text, eyebrow | **11px / 16px** | UPPERCASE, `letter-spacing: 0.08em` |

**Body face: Inter** (also self-hosted), 14px / 22px, sentence case, weight 400, 600 for
emphasis. Captions 12px / 18px in `--dim`.

⚠ **The hero headline is 22px.** Not 72px. This is the single hardest instruction in this
brief and the one you will be most tempted to ignore. A giant hero headline is what every
other landing page does and it would destroy the identity instantly. The hero gets its
impact from **enormous black space** and from the product art, not from type size. If you
feel the hero is under-powered, add space and reduce content — do not add font size.

Long-form paragraphs use Inter and may be larger than 14px if genuinely needed for reading
comfort (16px max). The **Silkscreen** face never leaves 11 and 22.

### The signature element: the glyph strip

Five squares in a row — **8×8px each, 4px gaps, 56×8px total**. In the app it is the only
status indicator that exists; every widget wears one and nothing else reports state.

**Reproduce it exactly on the site** and use it as the brand device: in the nav, as section
markers, in the footer, on every product panel you draw. Its states:

| State | Appearance |
|---|---|
| idle | cell 1 `--dim`, cells 2–5 `--edge` |
| focused | all 5 `--text` |
| syncing | one `--text` cell advances 1→5 and wraps, **120ms per step, hard stepped, never interpolated** |
| offline | all 5 `--dim` |
| error | cell 3 `--signal` red, rest `--edge` |

Build it as a React component with a `state` prop. Use it liberally — it is the logo.

## 3. Stack, and exactly how to use each piece

**Next.js (App Router) + TypeScript + Tailwind CSS + shadcn/ui + GSAP.** Static export is
fine; there is no backend.

### Tailwind
Put every token above into `tailwind.config.ts` as named theme values (`bg-plate`,
`text-dim`, `border-edge`, `rounded-plate`). **Then ban arbitrary values in review** — if a
colour or radius isn't in the config, it isn't in this design system. Delete the default
Tailwind palette entirely so nobody can reach for `blue-500`.

### shadcn/ui — install it, then strip it
shadcn is the right choice for accessible primitives (Dialog, Accordion, Tabs, Tooltip) but
**its default styling is the exact opposite of this design system**: soft rings, muted
accent colours, `rounded-md` everywhere, subtle shadows. You are using it for **behaviour
and accessibility, not for looks.**

Concretely:
- Set the shadcn CSS variables to the tokens above. `--background: 0 0% 0%`,
  `--foreground: 0 0% 100%`, `--border` to the edge value, and so on.
- **`--ring` must be white, not the accent colour.** The focus ring in this product is
  **2px `--text` at 2px offset**, everywhere, on everything. Never red — red means
  destructive, and a red focus ring reads as an error.
- Delete every `shadow-*` class shadcn ships with. There are no soft shadows here.
- Override `rounded-md`/`rounded-lg` to the three radii above.
- Use: **Dialog** (download modal), **Accordion** (FAQ), **Tabs** (use-case switcher),
  **Tooltip**, **Button** as a base only.
- **Do not use** shadcn's Card — build your own `Panel` component instead (see below), or
  the site will end up with two different card languages.

### React Bits — use two effects, refuse the rest
React Bits is full of flashy components. **Almost all of them break rule 4 or rule 1**:
anything with a gradient, a blur, a glow, a spotlight-follows-cursor, a tilt, a magnetic
hover, or animated colour is out. Do not install the whole library and browse for something
cool.

The ones that genuinely fit, because they are monochrome and text-mechanical:
- **Decrypted / scramble text** — on the hero headline only, once, on load. It reads like a
  split-flap board settling. Perfect fit. Do not put it on more than one element.
- **Count-up** — for the memory and startup numbers in the stats row. Stepped, not eased.

Anything else you want from React Bits, reimplement in 20 lines of your own CSS rather than
pulling its styling in.

### GSAP — motion is stepped and short
One easing curve site-wide: `cubic-bezier(0.2, 0, 0, 1)`. In GSAP terms, register
`CustomEase.create("tack", "0.2, 0, 0, 1")` and use it for **everything**.

Durations, from the product's own motion spec:

| Interaction | Duration |
|---|---|
| Panel enters | 160ms, opacity 0→1 plus 6px rise. **No scale.** |
| Panel exits | 100ms, opacity only |
| Hover reveal | 100ms, opacity or fill only |
| Press | 80ms, background steps one grey level |
| **Glyph strip cell change** | **120ms per cell, STEPPED — `ease: "steps(1)"`, never a tween** |

Use **ScrollTrigger** for section entrances only: fade plus a 6px rise, staggered ~60ms.
That is the entire scroll animation budget.

**Explicitly forbidden**, however good it looks in isolation: parallax, scroll-jacking,
pinned horizontal scroll, scale-on-scroll, rotation, skew, blur transitions, letter-by-letter
staggers on body copy, and anything with a bounce or elastic ease.

**`prefers-reduced-motion` is mandatory, not a nice-to-have.** When set: all travel and
stepping stops, transitions collapse to a 100ms crossfade, the glyph strip holds its final
state, and the hero scramble does not run. Wire this with `gsap.matchMedia()`.

### The one component to build first

```tsx
<Panel>  // bg-plate, rounded-[20px], border border-edge, no shadow
```

Everything on the page that is a surface — feature cards, the download box, use-case
panels, the FAQ container — is this component. One surface language, like the app.

## 4. Page structure

Single page, anchor-linked nav. Sections in order:

**Nav (sticky, 56px, `--base`, 1px `--edge` bottom rule)**
Glyph strip + `TACK` wordmark left. Right: `FEATURES` `USE CASES` `DOWNLOAD` in 11px
Silkscreen uppercase, `--dim`, stepping to `--text` on hover. One `DOWNLOAD` button.

**Hero**
Enormous vertical space. 22px Silkscreen headline (scramble-in once). One Inter paragraph at
`--dim` explaining what it is in two sentences. Two buttons: `DOWNLOAD FOR WINDOWS`
(primary — white fill, black text) and `SEE HOW IT WORKS` (default — `--elev-raised` fill,
`--edge` border). Below: the hero product art (see `VISUAL-PROMPTS.md` asset 5, or better,
recreate the panels in live HTML/CSS — they are just divs, and live panels with a running
glyph strip animation are far more convincing than a static image).

**The stats row**
Four numbers, count-up on scroll, 22px Silkscreen with 11px `--dim` labels beneath:
`5–15 MB` per widget · `<300 ms` cold start · `0` save buttons · `0` browser engines.
Hairline `--edge` dividers between them. This row does a lot of work — it is the whole
"not Electron" argument in one line.

**Features** — 5–6 `Panel`s, each with a small diagram, a 22px Silkscreen heading and an
Inter paragraph:
- *A widget is a readout, not a window* — the glyph strip, state at a glance
- *Native, not a browser* — the memory argument
- *Offline is normal, not an error* — CRDTs, no conflicts, no lost writes
- *No save button, ever* — persistence is invisible
- *Five widget types, one language* — notes, todos, documents, calendars, boards
- *Keyboard-first* — a global command palette; usable with no mouse

**Use cases** — shadcn Tabs, one panel each. Audience is broad, so cover:
- **Developers** — a scratch note pinned over the terminal; a board for the current branch
- **Designers** — reference notes and a task list that never cover the canvas
- **Students** — lecture notes, a deadline calendar, a revision checklist that stays visible
- **Everyone else** — the shopping list, the reminder, the thing you need on screen

Write these as concrete scenarios, not feature lists.

**Download**
Honest platform state:
- **Windows** — available, primary button, show the version and file size
- **macOS** — `COMING SOON`, `--dim`, disabled button at 0.40 opacity
- **Linux** — `COMING SOON`, same treatment
Note that mac and Linux builds are coming via GitHub Actions.
**TACK is free.** State that plainly. It is **not open source** — do not add a GitHub star
button, a "star us" CTA, or a repo link. No pricing table, no tiers, no "contact sales".

**FAQ** — shadcn Accordion, `--edge` hairlines, no card fills:
Is it really free? · Is it open source? (not yet) · Where is my data? (local SQLite, on
your machine) · Does it need an account? (no) · Does sync work today? (the data model is
built for it; networking is not shipped) · Why is the text pixelated? (it is the design —
Silkscreen, on purpose) · How much memory? · macOS/Linux when?

**Footer** — thin. Glyph strip, wordmark, copyright, privacy link. 11px, `--dim`.

## 5. Copy voice

Plain, technical, unhurried, slightly severe. The product is Braun instrumentation, and the
copy should read like a well-written manual for a good instrument.

- **No exclamation marks. No emoji. No "supercharge", "seamless", "effortless",
  "revolutionize", "game-changing", "10x", "AI-powered".**
- Short declarative sentences. Say the number instead of the adjective: not *"incredibly
  lightweight"* but *"5 to 15 MB per widget."*
- Never oversell. If something is not shipped, say so. The audience for a product like this
  can smell a claim that is not true, and honesty is on-brand here in a way it is not for
  most products.

## 6. Non-negotiables checklist

Before you call this done, verify every line:

- [ ] Silkscreen appears at **11px and 22px only**. Grep the CSS for any other size on it.
- [ ] `#D71921` appears **at most twice** in the rendered page.
- [ ] Zero gradients. Zero `backdrop-filter`. Zero `box-shadow` softer than the one flat
      panel shadow. Zero glassmorphism.
- [ ] Nothing scales, tilts, rotates or bounces on hover or scroll.
- [ ] Every focus ring is **2px white at 2px offset**. Tab through the whole page.
- [ ] The glyph strip animates **stepped**, never eased.
- [ ] `prefers-reduced-motion` genuinely disables all travel and stepping.
- [ ] Every colour, radius and spacing value comes from the token list.
- [ ] Contrast passes WCAG AA for body text.
- [ ] Fonts are self-hosted, subset, `font-display: swap`.
- [ ] Lighthouse ≥ 95 on performance and accessibility.
- [ ] Works at 360px wide. The glyph strip does not shrink below 56×8.
- [ ] Real `<title>`, meta description, OG image, favicon.

## 7. Deliverable

A complete, runnable Next.js project. Include `README.md` with run instructions, a
`tailwind.config.ts` carrying every token, self-hosted fonts, and placeholder image files
with the correct dimensions where art has not been supplied yet.

Build the `GlyphStrip` and `Panel` components first and get them exactly right — everything
else on the page is made of those two things.

═══════════════════════ END OF PROMPT ═══════════════════════

---

## Notes for you (not part of the prompt)

**Live panels beat generated art.** The hero panels are just styled divs. If the receiving
model builds them in HTML with a running glyph strip, the hero shows the actual product
language moving — far more convincing than a static generated image, and it will never
have the flaws image models introduce. Push for that.

**The 22px headline will be argued with.** Every model that receives this brief will want to
make the hero headline large, because that is what landing pages do. If it comes back at
48px or 72px, that is the single most important thing to send back — it is the difference
between "this looks like an instrument" and "this looks like every other startup".

**Real screenshots later.** Once the app is polished, replace the illustrated panels with
genuine screenshots. The recipe for capturing them is in `../progress.md` under START HERE.

**Order of work.** The site is downstream of the app. Stage W2 in `../TASKS.md` is the
polish you named — the Kanban is currently broken and the Calendar has no way to add an
event. A launch site pointing at that would be premature.
