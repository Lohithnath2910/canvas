# brand/

Launch material. Written 2026-08-26, **before** the app is polished — Stage W2 in
[../TASKS.md](../TASKS.md) comes first. Nothing here is on the build backlog; these are
documents to hand to other tools when the time comes.

| File | What it is | Where it goes |
|---|---|---|
| [NAME.md](NAME.md) | Why "Canvas" cannot ship, a five-name shortlist, and the argument for **TACK** | A decision for the user. **Still open** |
| [VISUAL-PROMPTS.md](VISUAL-PROMPTS.md) | Image-generation prompts, one pair per asset — one written for **ChatGPT**, one for **Gemini** | Paste into ChatGPT / Gemini |
| [WEBSITE-PROMPT.md](WEBSITE-PROMPT.md) | One self-contained brief for building the marketing site, including how to use shadcn, React Bits and GSAP without breaking the design system | Paste into Claude or another coding AI |

## The one decision blocking both

**The name.** Both prompt files are written against `TACK` and both carry a `NAME:` line at
the top. Pick the name first — it drives the logo, the domain, and every line of copy, and
changing it afterwards means rewriting both files.

## The through-line

Everything here rests on one fact that was confirmed in the code rather than assumed:
**the entire app is Silkscreen** — `body-face` is Silkscreen too, not just labels
(`crates/canvas-ui/ui/theme/typography.slint`). A product rendered wholly in a dot-matrix
face is rare and completely ownable, so the identity is not invented for the launch:

- The **wordmark** is the product name set in Silkscreen. Not drawn — set.
- The **symbol** is the five-cell glyph strip already shipping in every widget: 8×8px cells,
  4px gaps, 56×8 total, one cell in `#D71921`.
- The **site** reuses the app's own tokens, so opening the app after visiting the site is
  not a change of language.

## Do not generate these

Draw them by hand — image models cannot hit them and they are trivial: the five-cell mark,
the wordmark, the 16px favicon, and real product screenshots. `VISUAL-PROMPTS.md` says which
assets are worth generating and which are not, and why.
