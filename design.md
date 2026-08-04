# CANVAS — Premium UI/UX Design System
**Companion to CANVAS_Master_Blueprint.md — hand both files to the same Antigravity session**

---

## 0. The Concept: "Instrument Glass"

Most AI-generated "premium minimal" UI collapses into one of three defaults: warm cream + serif + terracotta, near-black + acid-green, or a hairline-heavy newspaper grid. Canvas should avoid all three — not because they're bad, but because none of them are *about* anything specific to this product.

Canvas widgets sit directly on a desktop, on top of whatever wallpaper the user has — they can't rely on a fixed background color the way a webpage can. That constraint is the design opportunity: **the widgets should read as physical instruments made of glass and metal, not as app windows.** Think a precision optical device or a fine drafting tool — a loupe, a rangefinder, a watch face — reimagined as software. Glass for the body (translucent, adaptive to whatever is behind it), brass for the moments that matter (active states, focus), and a single signature detail — a living light at the edge of every widget — that replaces almost all other status chrome.

This gives you a design language that is: quiet by default, tactile rather than flat, and legible against literally any wallpaper.

---

## 1. Color System

Six named tokens. No more. Accent color must never cover more than roughly 5% of any widget's visible surface — it marks moments, it doesn't decorate.

| Token | Value | Role |
|---|---|---|
| `canvas-paper` | `#F4F5F3` | Light-mode glass base (cool, slightly green-gray — not warm cream) |
| `canvas-graphite` | `#14171A` | Dark-mode glass base (soft near-black, not pure black) |
| `canvas-ink` | `#1B1F1E` (light mode text) / `#EDEFEC` (dark mode text) | Primary text, adaptive |
| `canvas-verdigris` | `#3E7C74` | Primary accent — focus states, primary actions, the Aura Edge's resting glow, success |
| `canvas-brass` | `#C9A16B` | Secondary accent, used sparingly — sync-in-progress, hover glints, warning |
| `canvas-brick` | `#B5544A` (desaturated) | Danger/error only — oxidized-copper red, not a bright alert red |

Glass base tones are applied at **70–78% opacity** over a blurred backdrop (see Section 4 for platform feasibility), never fully opaque — opacity this high on a translucent surface is what keeps text legible without killing the "glass" read.

---

## 2. Typography

**Deliberate choice: no custom display font.** A tool built on "native first, no Electron, no webview" should borrow the OS's own type system rather than ship a foreign face — that's not a default, it's the same principle as the rest of the architecture applied to type. Personality comes from the type *scale*, weight, and one specific flourish (below), not from typeface selection.

| Platform | System face |
|---|---|
| macOS | SF Pro (via native text rendering) |
| Windows | Segoe UI Variable |
| Linux | Inter (bundled fallback) or Noto Sans |

**The one flourish:** all metadata — timestamps, sync state, item counts, presence labels — renders in a **monospace face** (`SF Mono` / `Cascadia Mono` / `JetBrains Mono` fallback stack), slightly letter-spaced, like a gauge readout on an instrument panel. This is the single typographic signature. Everything else stays in the system sans.

| Role | Size / line-height | Weight | Face |
|---|---|---|---|
| Widget title | 20/28 | 600 | System sans |
| Body | 14/20 | 400 | System sans |
| Secondary / caption | 12/16 | 400, `canvas-ink` at 60% | System sans |
| Metadata readout (timestamps, sync status, counts) | 11/16, +0.02em tracking | 500 | Monospace |

---

## 3. Materials: Glass, Blur & Elevation

**Read this before implementing — it's a feasibility flag, not just a style spec.**

True "live" background blur (seeing a soft-focus version of whatever is behind the window, updating in real time) is an OS compositor feature, not something a UI framework draws itself:
- **macOS**: `NSVisualEffectView` — native, well-supported.
- **Windows**: Acrylic/Mica via DWM composition APIs — native, well-supported.
- **Linux**: compositor-dependent (works under KWin with blur enabled, does **not** work under many Wayland/GNOME setups) — treat as a progressive enhancement, not a guarantee.

Slint itself does not provide cross-platform backdrop blur out of the box. **Implementation path:** use each OS's native windowing hook to apply true vibrancy/acrylic where available (platform-specific code behind a shared trait), and fall back to a static frosted-glass *appearance* (semi-transparent flat fill + a subtle inner highlight, no live blur) on platforms where it isn't available. Never block a milestone on live blur — it's a per-OS enhancement layered on top of a design that already looks intentional without it.

**Elevation / shadow recipe** (three-layer, applied to every glass panel):
1. Contact shadow: `0px 1px 2px rgba(0,0,0,0.06)`
2. Ambient shadow: `0px 8px 24px rgba(20,23,26,0.18)`
3. Edge definition: 1px hairline border, `rgba(255,255,255,0.14)` in dark mode / `rgba(20,23,26,0.10)` in light mode

No single heavy drop shadow — the layered soft shadows are what read as "glass resting on a surface" rather than "card with a shadow."

---

## 4. Shape Language

A consistent radius *family*, not one radius reused everywhere:

| Element | Radius |
|---|---|
| Small controls (buttons, chips, checkboxes) | 10px |
| Sticky Note widget | 16px |
| Standard widget body (Todo, Kanban card, Calendar) | 18px |
| Command palette / modal / large overlay | 26px |

---

## 5. Spacing & Layout Grid

Base unit: **8px**, with 4px reserved only for tight icon-to-label gaps.
Scale: `4, 8, 12, 16, 24, 32, 48`.
Widget internal padding: 16px standard, 24px for the Shared Document widget (reading comfort matters more there).

---

## 6. Iconography

Thin-to-medium stroke weight, consistent 1.5px stroke, rounded line caps, no filled icons except for the single active/selected state of toggle-style controls. Icon sizes: 16px inline, 20px in widget headers, 24px only in the command palette. Icons should read as *engraved* rather than *printed* — slightly desaturated relative to body text, brightening to full `canvas-ink` only on hover/active.

---

## 7. Motion & Micro-interactions

Motion communicates state truthfully — it should never run longer or fancier than the state change it represents.

| Interaction | Duration | Easing | Notes |
|---|---|---|---|
| Widget open | 260ms | `cubic-bezier(0.16, 1, 0.3, 1)` | Scale 0.96→1, fade in, 4px upward drift, originates from cursor/launch point |
| Widget close | 180ms | same, reversed | Faster than open — closing shouldn't linger |
| Checkbox / button press | 120ms | `cubic-bezier(0.34, 1.56, 0.64, 1)` (slight overshoot) | Overshoot reserved for small controls only — never on whole-window motion |
| Drag | instant follow | — | 2° max tilt, shadow layer 2 intensifies slightly during drag, springs back over 320ms on release |
| Presence cursor movement | 80ms linear interpolation | — | Never snaps |
| Reduced-motion mode | 100ms crossfade only | — | All scale/rotation/sweep motion disabled when OS reduce-motion is set — this is a requirement, not optional polish |

---

## 8. The Aura Edge — Signature Element

The one bold, memorable design move. A **1.5px animated gradient rim** traces the border of every glass widget and is the *only* status indicator most widgets need — no separate sync icon, no separate "online" dot cluttering the corner.

| State | Aura Edge behavior |
|---|---|
| Idle, unfocused | Static neutral hairline (`rgba(255,255,255,0.14)`) |
| Focused / actively editing | Steady `canvas-verdigris` glow, full rim |
| Syncing | Slow (1800ms, linear, infinite while active) gradient sweep from `canvas-brass` to `canvas-verdigris` around the rim — stops the instant sync completes, never fades out artificially |
| Another collaborator editing | Rim segment nearest their cursor tints to their presence color |
| Offline | Rim simply dims to ~40% opacity — no red, no alarm; offline is a normal state, not an error |
| Error / conflict that needs attention | Rim pulses once in `canvas-brick`, then settles to a steady low-opacity brick tint until resolved |

Build this as a single reusable `AuraEdge` component (Section 10) that every widget wraps itself in — it should be structurally impossible for a widget to exist without it, which is what keeps the status language consistent app-wide.

---

## 9. Widget-by-Widget Notes

- **Sticky Note** — minimal chrome. Controls (color, pin, delete) appear only on hover, in a thin top strip that fades in over 100ms. Typography-first; the note itself has no visible "editor" framing.
- **Todo** — checkbox check is a small drawn checkmark animation (~150ms), not a checkbox-fill swap. Completed items: text fades to 50% opacity and gets a strike-through that draws left-to-right over 200ms, not an instant line.
- **Shared Document** — collaborator cursors: thin colored line + small nameplate pill that only shows on cursor movement, fading out after 2s of inactivity so it doesn't clutter static reading.
- **Whiteboard** — floating tool palette, not a docked toolbar — a small pill-shaped dock that follows near the cursor's working area. Infinite canvas background stays neutral glass, never a bright grid.
- **Kanban** — card lift shadow increases (layer 2 intensifies) during drag; drop target column gets a subtle `canvas-verdigris`-tinted outline, not a full highlight fill.
- **Calendar** — event chips use per-calendar accent tint at low saturation; today's date column gets a faint verdigris wash, not a bold block.
- **Global launcher / dock** — a command-palette-style summon (global hotkey) for spawning new widgets and searching across them, styled as the "Command palette / modal" radius tier (26px), Glass-Thick material.

---

## 10. States, Empty States & Microcopy

Voice: plain, active, says what happened and what to do next — never apologetic, never exclamation points.

| Moment | Copy |
|---|---|
| Empty desktop (first launch) | Headline: "Nothing here yet." Body: "Press ⌘K to create your first note." |
| Offline | No banner. The dimmed Aura Edge *is* the indicator. If the user opens settings while offline: "You're offline. Changes are saved and will sync when you're back online." |
| Sync conflict | "Two edits happened at once here — both are kept below." (never "error," never blames a user) |
| Widget deleted by a collaborator | "This note was deleted by [name]." with an "Undo" action, not a silent disappearance |

---

## 11. Accessibility Requirements

- Text contrast against the translucent glass base must hit **WCAG AA (4.5:1)** even at the glass's minimum opacity — verify against both the lightest and darkest realistic wallpaper condition, adding an auto-adjusting scrim behind text if a given backdrop is too busy.
- Every interactive element gets a visible focus ring (`canvas-verdigris`, 2px, offset 2px) — never rely on the Aura Edge alone for keyboard focus, it's a widget-level signal, not a control-level one.
- Full keyboard navigation: every widget must be creatable, editable, and closable without a mouse.
- Reduced-motion respected everywhere (Section 7) — this is required for M2 sign-off, not deferred polish.

---

## 12. Nano Banana Image Generation Prompts (copy-paste ready)

### Prompt A — Hero shot (desktop scene, multiple widgets)

```
A high-fidelity product mockup of a minimalist desktop productivity app called
"Canvas." Wide desktop screenshot composition, straight-on camera angle, soft
even studio lighting.

Scene: a clean desktop background with a subtle soft gradient wallpaper in
muted deep teal-gray tones. Floating on top of it are three translucent
frosted-glass widget panels, each with rounded corners (approx 16-18px
radius), soft layered drop shadows, and a very thin glowing edge outline in a
muted teal-green color tracing each panel's border.

Widget 1 (top left): a small sticky-note-style panel with a few short lines of
soft gray placeholder text, no visible toolbar chrome.
Widget 2 (center): a slightly larger panel showing a simple checklist with 4
rounded checkbox items, one checked with a small teal checkmark, clean
generous spacing between rows.
Widget 3 (bottom right): a compact kanban-style panel with three narrow
columns and a few small rounded cards inside, one card mid-drag with a
slightly larger shadow beneath it.

Materials and mood: think precision optical instrument — glass and brushed
metal, not plastic. A small warm brass-gold accent appears only once, as a
subtle glowing dot in the corner of the checklist widget. Overall palette:
cool paper-white and graphite glass tones, one muted teal-green accent, one
small warm brass accent, nothing else. Extremely minimal, generous negative
space, soft ambient occlusion, high production value like a premium tech
product page render.

Style: photorealistic UI mockup render, soft cinematic lighting, shallow
depth of field with the center widget in sharpest focus, 16:9 aspect ratio,
ultra-clean, no clutter.

Avoid: no legible paragraphs of real text anywhere (use abstract soft gray
line placeholders instead of actual words), no logos, no brand names, no
extra widgets beyond the three described, no bright saturated colors, no
neon, no dark mode in this shot (render light mode only), no visible operating
system taskbar or menu bar.
```

### Prompt B — Close-up material detail shot

```
Extreme close-up macro product shot of the corner of a single translucent
frosted-glass software widget panel, rounded corner at approximately 18px
radius. The glass surface is semi-transparent showing a very soft blurred
hint of a teal-gray gradient background behind it. A thin (roughly 1.5px)
glowing gradient edge-light traces the border, transitioning subtly from a
warm brass-gold tone into a muted teal-green tone, like a slow-moving
status indicator on a precision instrument.

Lighting: soft studio lighting from upper left, subtle specular highlight
along the top edge of the glass, soft layered shadow beneath the panel.
Materials: glass and a hint of brushed brass at the very corner, both cool
and warm tones present but restrained.

Style: photorealistic macro product render, extremely shallow depth of
field, 1:1 square aspect ratio, premium tech-hardware advertising aesthetic
(think a close-up shot of a fine watch or camera, not a screenshot).

Avoid: no visible text, no icons, no UI controls, no logos, no bright or
saturated colors beyond the described brass and teal, no clutter — this is
a materials-and-lighting study, not a full interface.
```

---

## 13. Build Instructions for the Coding Agent (Slint implementation)

### Token file structure (add to `canvas-ui` crate from the master blueprint)

```
crates/canvas-ui/
├── theme/
│   ├── palette.slint      # color tokens from Section 1
│   ├── typography.slint   # type scale from Section 2
│   ├── spacing.slint      # spacing scale from Section 5
│   ├── radius.slint       # radius family from Section 4
│   ├── motion.slint       # durations/easing constants from Section 7
│   └── materials.slint    # AuraEdge + GlassPanel components
```

### Example global token singleton (illustrative — verify exact syntax against the pinned Slint version)

```slint
export global Palette {
    out property <color> canvas-paper: #F4F5F3;
    out property <color> canvas-graphite: #14171A;
    out property <color> canvas-verdigris: #3E7C74;
    out property <color> canvas-brass: #C9A16B;
    out property <color> canvas-brick: #B5544A;
}

export global Radius {
    out property <length> control: 10px;
    out property <length> sticky-note: 16px;
    out property <length> widget: 18px;
    out property <length> overlay: 26px;
}

export global Motion {
    out property <duration> open: 260ms;
    out property <duration> close: 180ms;
    out property <duration> micro: 120ms;
}
```

### `AuraEdge` component — build this once, wrap every widget in it

Requirements for the implementation, not just the visual:
- Accepts a `state` property: `idle | focused | syncing | offline | presence(color) | error`.
- Owns its own animation timeline internally — widgets set state, they never author the animation themselves. This is what keeps the status language consistent across all eight widget types.
- The `syncing` sweep animation must genuinely reflect real sync activity from `canvas-sync` (Section 6 of the master blueprint) — never a decorative loop that runs regardless of actual network state. If it can't be wired to real state yet in an early milestone, default to `idle`, not a fake `syncing` animation.

### Platform blur hookup

Implement a small trait, e.g. `trait NativeVibrancy { fn apply(&self, window_handle) -> BlurSupport }`, with per-OS implementations behind `cfg(target_os = ...)`:
- macOS: apply `NSVisualEffectView` material behind the Slint-rendered content.
- Windows: apply DWM Acrylic/Mica via the relevant Win32 composition API.
- Linux: attempt compositor blur where supported (KWin blur manager protocol); fall back to the static frosted-appearance described in Section 3 otherwise.

Gate this behind a `BlurSupport::Native | BlurSupport::Approximated` enum so the rest of the UI code never needs to know which path was taken — both should look intentional.

---

## 14. Assumptions Made — review before building

1. **Light mode designed first**, with dark mode as a direct token flip (`canvas-graphite`/`canvas-ink` swap) rather than a separately art-directed dark theme. If you want dark-mode-first, the glass opacity and shadow values in Section 3 will need re-tuning — dark glass typically needs slightly lower base opacity to keep the blur read.
2. **Verdigris + brass** was chosen as the signature palette specifically to avoid the three common AI-generated defaults (cream/terracotta, near-black/acid-green, broadsheet). If you have an existing brand color you want reflected instead, swap `canvas-verdigris` and keep everything else — the system is built so one token change re-themes the whole app.
3. **The Aura Edge is load-bearing for the whole status-communication strategy** — if you'd prefer a more conventional status-icon approach, that changes Section 8 and touches every widget spec in Section 9.