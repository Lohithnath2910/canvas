# Image generation prompts — ChatGPT and Gemini

**NAME: `TACK`** — if you renamed, change it here and search-replace below.

## How to use this file

Every asset below has **two prompts**: one for **ChatGPT** (`gpt-image-1`, i.e. images in
ChatGPT) and one for **Gemini** (Nano Banana / Imagen). They are genuinely different, not
copy-paste twins, because the two models fail in different ways:

- **ChatGPT** follows precise instructions and renders **text** far better. Give it exact
  hex values, exact counts, and it will mostly obey. It over-adds texture and soft shadow
  unless told not to, repeatedly.
- **Gemini** wants a **described scene** rather than a spec sheet. Bare constraint lists
  give bland output. It is weaker at small text — never rely on it for a wordmark.

**Rules for both, every time:**
- Ask for **one** image per generation. Both models degrade when asked for a sheet of
  variants.
- Regenerate rather than refine for the first three tries; ChatGPT in particular anchors
  hard on its first attempt.
- **Everything must be flat.** The single most common failure is the model adding a soft
  drop shadow, a subtle gradient, or a glassy highlight. Every prompt below ends with a
  negative list. Do not delete it — it is the load-bearing part.
- Check output against the five rules in `../CLAUDE.md` §1 before you accept anything.

## The palette, for pasting

```
True black      #000000
Plate           #121212
White / text    #FFFFFF
Dim white       rgba(255,255,255,0.52)
Edge / hairline rgba(255,255,255,0.14)
Signal red      #D71921   ← the ONLY colour. Appears once, never twice.
Greys           #1C1C1C  #242424  #2E2E2E  #3A3A3A
```

Radii: plate **20px**, control **8px**, overlay **28px**.
Typeface: **Silkscreen** (Google Fonts, SIL OFL), uppercase, letter-spaced. The whole
product uses it — labels *and* body.

---

# 1. The symbol / app mark

The five-cell glyph strip. This is the product's real signature, not a logo invented for
marketing. **Do not let either model redesign it** — it is a spec, not a brief.

Honestly: this one is so geometrically exact that **you should draw it in SVG rather than
generate it.** Five 8×8 squares, 4px apart, 56×8 total; fourth square `#D71921`, rest
`#FFFFFF`. That is nine lines of SVG and it will be pixel-perfect, which no image model will
be. The prompts below are for exploring *treatments* of it, not for producing the final file.

### ChatGPT
```
A flat vector logo mark on a pure #000000 square background.

The mark is exactly five small squares in a single horizontal row, evenly spaced.
Each square is the same size. The gap between squares is half the width of a square.
Four squares are pure white #FFFFFF. The fourth square from the left is #D71921 red.
All squares have perfectly sharp 90-degree corners — they are squares, not rounded,
not circles.

The row of five squares is centred and occupies about 40% of the image width.
Nothing else appears in the image. No text, no border, no frame.

Style: absolutely flat 2D vector. Pure solid fills only.
Do NOT add: gradients, drop shadows, glow, bevel, emboss, reflection, texture, noise,
grain, 3D perspective, lighting effects, rounded corners, outlines, or any additional
colour. The image contains exactly three colours: #000000, #FFFFFF, #D71921.
```

### Gemini
```
Picture the status indicator on a piece of precision laboratory equipment, photographed
straight on in the dark: a single row of five tiny square lamps set into a matte black
panel. Four of them are lit clean white. One — the fourth from the left — is lit red.
Nothing else is on the panel.

Render this as a completely flat graphic, not a photograph: solid colour shapes on a
pure black field, the way a technical drawing or a screen-printed icon would show it.
The squares have hard sharp corners and sit in a straight even row, centred in the frame.

Colours are exactly: black #000000 background, white #FFFFFF squares, one red #D71921
square. Only those three.

Keep it flat and graphic. No gradients, no glow around the lamps, no soft shadows, no
reflections, no metallic sheen, no depth or perspective, no text, no border.
```

---

# 2. Wordmark

⚠ **Use ChatGPT for this. Gemini will misspell it.** And genuinely — the correct wordmark is
`TACK` typed in Silkscreen with +0.06em tracking. Generating it is for exploring lockups
(mark + word together), not for the final asset. Set the real one in Figma in ten seconds.

### ChatGPT
```
A flat logo lockup on a pure #000000 background, centred.

On the left: five small white squares in a horizontal row, evenly spaced, with the
fourth square from the left in #D71921 red. Sharp 90-degree corners.

To the right of that row, separated by a clear gap: the word "TACK" in capital letters,
in a pixelated bitmap typeface — the kind used on 1980s computer terminals and dot-matrix
displays, where each letterform is built from visible square pixels. The letters are pure
white #FFFFFF with generous letter-spacing between them. The word is optically the same
height as it is legible — chunky, square, mechanical.

The word must be spelled exactly T-A-C-K. Four letters. No other text anywhere.

Style: flat 2D vector, solid fills, hard edges.
Do NOT add: gradients, shadows, glow, bevel, outline, texture, grain, 3D, italic slant,
serifs, smooth curved letterforms, or any colour beyond #000000, #FFFFFF and #D71921.
```

### Gemini
```
A dot-matrix display panel in the dark showing a single short word in blocky pixel
letters, rendered as a flat graphic rather than a photo. To the left of the word sits a
row of five small square indicator lamps, four white and one red.

The word is built from visible square pixels, like text on an old airport departure board
or a vintage computer terminal — chunky, upright, mechanical, widely letter-spaced.

Everything sits on pure black. The pixels of the letters are clean white. Exactly one
lamp is red. Flat solid colour throughout, hard square edges, no softness anywhere.

Avoid: gradients, glowing halos, soft shadows, reflections, metal texture, depth,
perspective, decorative flourishes.
```

---

# 3. App icon (Windows `.ico`, macOS `.icns`, taskbar, tray)

### ChatGPT
```
A flat app icon, square, 1024x1024.

The icon is a rounded square plate filling most of the frame, with a corner radius of
about 20% of its width. The plate is very dark near-black #121212, sitting on a pure
#000000 background. The plate has a 1-pixel hairline border in very faint white
(about 14% opacity white).

Centred inside the plate: a single horizontal row of five small squares, evenly spaced.
Four are pure white #FFFFFF. The fourth from the left is #D71921 red. Sharp corners.
The row is small relative to the plate — it occupies roughly one third of the plate width,
leaving generous empty space around it.

Nothing else. No text, no letters, no additional shapes.

Style: completely flat 2D. Solid fills only.
Do NOT add: gradients, drop shadows, inner shadows, glow, bevel, emboss, glass or gloss
highlights, reflections, texture, noise, 3D depth, or perspective. Only three colours
plus the hairline: #000000, #121212, #FFFFFF, #D71921.
```

### Gemini
```
A minimalist app icon: a smooth-cornered dark charcoal tile, almost black, floating on a
pure black field. The tile's edge is traced by a single hairline of very faint white,
barely visible — just enough to separate it from the background.

Dead centre on the tile, small and deliberate, sits a row of five square indicator
lamps: four white, and the fourth one red. They are the only thing on the tile. Lots of
quiet empty space around them.

The whole thing is flat and graphic — a screen-printed icon, not a rendered object.
Solid colours, hard edges, no illusion of depth or material.

Avoid: gradients, glossy highlights, soft shadows, inner glow, bevelled edges, plastic or
glass appearance, reflections, texture, any text or lettering.
```

**16×16 favicon: do not generate.** At 16px the plate and radius disappear. Hand-draw an
SVG of just the five cells — four white, one red — on transparent. That is the favicon.

---

# 4. Social / OG card (1200×630)

### ChatGPT
```
A 1200x630 landscape social media banner on a pure #000000 background.

Left half: the word "TACK" in large capital letters in a pixelated bitmap typeface built
from visible square pixels, pure white #FFFFFF, widely letter-spaced. Directly beneath it,
much smaller, in the same pixel typeface but in dimmer grey-white: the line
"WIDGETS THAT REPORT THEMSELVES". Spell both exactly as written.

Right half: three small dark rounded-corner rectangular panels, arranged loosely and
overlapping slightly, as if scattered on a desk. Each panel is #121212 with a 1-pixel
faint white hairline border and a 20-pixel corner radius. Each panel has a row of five
tiny squares in its top-left corner. On one panel only, one of those five squares is
#D71921 red; on the others all five are white or grey. The panels contain a few short
horizontal grey lines suggesting text, but no readable words.

Generous empty black space. Nothing is centred or symmetrical — it should feel like an
instrument panel, not a poster.

Style: flat 2D vector. Solid fills, hard edges.
Do NOT add: gradients, drop shadows, glow, blur, glass, 3D perspective, tilt or rotation
of the panels, texture, noise, or any colour other than black, white, greys, and the one
red square.
```

### Gemini
```
A wide dark banner that looks like a control surface. On the left, a short word set in
chunky pixel letters glows clean white against pure black, with a small dim caption
underneath it in the same pixel type.

On the right, three small charcoal panels with softly rounded corners lie scattered and
slightly overlapping, like cards left on a black desk. Each is outlined by a single
hairline so faint it is almost not there. Each carries a row of five tiny square lamps in
its upper-left corner; on exactly one panel, one lamp burns red. Faint grey lines inside
the panels hint at writing without being readable.

Lots of black emptiness. The composition is off-centre and instrument-like, calm and
technical rather than decorative.

Everything flat and printed, no depth. Avoid gradients, glows, blurred edges, soft
shadows, glassy surfaces, tilted or rotated panels, and any colour beyond black, white,
grey and that one red lamp.
```

---

# 5. Website hero image

The hero's job is to show **the product as it actually is**: small monochrome plates
floating on a real desktop, reporting state. Resist anything cinematic.

### ChatGPT
```
A 2400x1400 flat illustration of a computer desktop, viewed straight on with no
perspective or tilt.

The background is a very dark, almost black desktop with no wallpaper imagery — just a
flat near-black field.

Floating on it, at different positions and slightly overlapping, are five small
rectangular panels of different sizes. Every panel is #121212 with a 20-pixel corner
radius and a 1-pixel hairline border in white at about 14% opacity.

Every panel has, in its top-left corner, a row of five tiny 8-pixel squares with 4-pixel
gaps — the panels' status indicator. On four panels these squares are white or dim grey.
On exactly ONE panel, the middle square is #D71921 red and the rest are dim.

The panels show, suggested only, never legible: one with several short horizontal grey
lines (a note); one with a column of small squares beside short lines (a checklist);
one with seven narrow vertical columns containing small rounded grey blocks at different
heights (a week calendar); one with three columns of small stacked cards (a board);
one small and nearly empty.

All panel text is suggested with grey bars, not real letters, EXCEPT each panel's title
which is a short word in a pixelated bitmap typeface in white capitals.

Style: completely flat 2D vector. Straight-on orthographic view.
Do NOT add: gradients, drop shadows, glow, blur, glass, reflections, 3D perspective,
isometric angles, tilted or rotated panels, desk objects, plants, coffee cups, hands,
mice, keyboards, wallpaper photographs, or any colour beyond black, greys, white and the
single red square.
```

### Gemini
```
Look at a dark computer screen straight on. There is no wallpaper — just deep black. A
handful of small, quiet, charcoal-grey panels of different sizes float on it, overlapping
each other here and there like cards laid on a desk. Each has gently rounded corners and
is traced by a hairline so faint you barely register it.

Each panel wears the same small badge in its top-left corner: a row of five tiny square
lamps. On most panels they sit dim and white. On one panel alone, a single lamp burns red.

The panels are clearly different tools — one holds a few lines of writing, one a list with
small square checkboxes, one a week laid out in seven narrow columns with little blocks
sitting at different heights, one a board of three columns of stacked cards. You can read
their short pixel-lettered titles; the rest of their contents is only suggested by soft
grey bars.

The whole scene is flat and graphic, seen face-on, calm and instrumental — like looking at
a well-designed control panel rather than a photograph of a desk.

Avoid: perspective, isometric or tilted views, drop shadows, glows, blur, glassy or
reflective surfaces, gradients, desk clutter, hands, plants, cups, wallpaper photos, and
any colour except black, grey, white and the one red lamp.
```

---

# 6. Feature-section illustrations (repeat per feature)

Small square diagrams beside each feature block. Generate one at a time, changing only the
bracketed line.

### ChatGPT — template
```
A flat 1000x1000 square diagram on a pure #000000 background, centred, with generous
empty space around it.

[SUBJECT — replace with ONE of:
 · "two identical small dark panels side by side, each with a row of five white squares
    in its corner, connected by a single thin horizontal white line between them"
    (for: syncing / CRDT)
 · "one small dark panel with a row of five squares in its corner, where all five squares
    are dim grey, and a short stack of three tiny grey rectangles queued beside it"
    (for: works offline)
 · "one small dark panel whose row of five squares shows only the leftmost lit white and
    the other four faint" (for: the glyph strip / state at a glance)
 · "five small dark panels of different sizes overlapping loosely" (for: many widgets)
 · "one small dark panel with a solid white 2-pixel rectangle outline drawn 2 pixels
    outside its edge" (for: keyboard-first)
]

Every panel is #121212 with a 20-pixel corner radius and a 1-pixel hairline border in
white at 14% opacity. Every small square is 8 pixels with 4-pixel gaps, sharp corners.

No text, no labels, no arrows, no icons, no numbers.

Style: completely flat 2D vector, straight-on view, solid fills.
Do NOT add: gradients, shadows, glow, blur, 3D, perspective, tilt, texture, or any colour
beyond #000000, #121212, #FFFFFF and greys. Use #D71921 red ONLY if the subject line above
explicitly mentions red — otherwise no red at all.
```

### Gemini — template
```
A single quiet technical diagram floating in the middle of a pure black square, with a lot
of empty space around it.

[SUBJECT — same options as the ChatGPT template above, phrased as a scene]

Everything is drawn flat, like a diagram in a well-set technical manual: solid charcoal
shapes, hairline outlines, small square lamps with hard corners, clean white marks on
black. No labels, no arrows, no icons, no numbers anywhere.

Avoid: gradients, glows, soft shadows, blur, depth, perspective, tilt, texture, and any
colour at all beyond black, charcoal, grey and white.
```

---

# 7. What to reject

Send it back if you see any of these — they are the five rules being broken, and they will
make the site look like every other SaaS page:

- **A second colour.** Blue, purple, teal, orange. There is one red and it appears once.
- **A gradient anywhere**, including a "subtle" black-to-grey one.
- **A glow or bloom** around the lit squares. They are lit, not glowing.
- **A soft drop shadow** under the panels. The product has one flat shadow and image models
  cannot do it; better to have none.
- **Glass, frost, blur, or translucency.**
- **Tilt, rotation, perspective, or isometric.** Everything is face-on.
- **Rounded dots instead of squares.** The cells are squares. Circles are a different product.
- **Smooth typography.** If the letterforms are not built from visible square pixels, it is
  not Silkscreen and it is not this product.
- **Desk photography clichés** — plants, coffee, hands, mechanical keyboards, window light.

## Final assets you should NOT generate

Draw these by hand; a model will never be exact enough and they are trivial:

| Asset | Why |
|---|---|
| The five-cell mark | Nine lines of SVG, must be pixel-exact |
| The wordmark | It is `TACK` set in Silkscreen. That is the design |
| Favicon (16/32) | Too small for a model to hit |
| Real product screenshots | Use the actual app — see `../progress.md` START HERE for the screenshot recipe |

The generated assets are for the **hero, OG card, and feature diagrams** — places where a
little atmosphere helps and pixel precision does not matter.
