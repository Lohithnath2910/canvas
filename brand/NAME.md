# The name

**Status: decision pending. Everything in `VISUAL-PROMPTS.md` and `WEBSITE-PROMPT.md` is
written against `TACK`. If you pick a different one, both files have a `NAME:` line at the
top — change it there and search-replace the rest.**

## Why "Canvas" has to go

Three collisions, any one of which is enough:

- **Instructure Canvas** — the LMS used by most universities on earth. You will never
  outrank it, and your audience includes students.
- **Canva** — one letter away, ~200M users, same broad "make something visual" space.
- **`<canvas>`** — the HTML element. Every developer search is poisoned.

It is also the wrong *meaning*. A canvas is a big empty surface you fill. This product is
the opposite: small fixed plates that each report one thing. The name is fighting the
product.

## What the name has to do

You asked for something that could become the verb, the way *"google it"* replaced
*"search for it"*. That is a real constraint and it rules a lot out:

- **One or two syllables.** Three-syllable verbs don't stick.
- **Already a verb, or trivially verb-able.** "Google" worked because it was a *noun* that
  people forced into a verb; the easier that forcing, the faster it happens.
- **Physical and humble, not abstract and grand.** This product is Braun instrumentation and
  split-flap boards, not a portal or a nexus or a flow. Anything ending in `-ly`, `-ify`,
  `-flow`, `-space` or `-mind` is exactly the AI-generated register you said to avoid.
- **Spellable after hearing it once.**

## The shortlist

| Name | What it means | As a verb | Honest problem |
|---|---|---|---|
| **TACK** ← recommended | A small flat-headed pin. Also: to fasten in place; to change course. | *"tack it to your desktop"*, *"just tack it"* — already a verb, and it already means the exact action | Swedish for "thanks"; a few tiny products use it. Neither is a real blocker |
| **BLIP** | A dot on a radar sweep. Monochrome instrumentation, exactly this product's reference frame | *"blip it"* — verb-able, slightly playful | Reads a touch cute for a precision tool. `blip.tv` is dead but memorable to some |
| **VANE** | A weather vane — a device whose entire job is to **report state at a glance**. Also the individual flap on a split-flap board | *"vane it"* doesn't work | Conceptually the best fit on this list and the worst verb. Fails your main criterion |
| **TALLY** | A running count, a tally board | *"tally it"* — real verb | Means *counting*, so it undersells notes and boards. Two syllables |
| **PERCH** | Where something sits above everything else — literally what an always-on-top widget does | *"perch it there"* — semi-natural | Perch CMS exists. Softer than the product feels |

## Recommendation: **TACK**

It wins on the criterion you actually named. "Tack it" is not a coinage anyone has to be
taught — it is a phrase that already exists and already means *put this small thing where I
can see it*. That is the product in three words.

It also fits the design language without trying. A tack is flat, small, metal, unglamorous,
and completely without ornament. It is the Braun object of stationery. And the head of a
tack, seen straight on, is a single filled dot — which is the product's entire visual
system.

Secondary meaning worth having: *to tack* is also to change course by turning into the wind.
Quietly good for a product about CRDTs converging.

**Domains to check:** `tack.app`, `tackhq.com`, `usetack.com`, `tack.tools`.
**Handles:** `@tackapp`, `@usetack`.
⚠ Run a real trademark search (USPTO / EUIPO / Indian TM registry) before printing anything.
This is a naming argument, not legal advice.

## The wordmark writes itself

The product is **100% Silkscreen** — not just labels, the body text too (`body-face` is
Silkscreen in `theme/typography.slint`). A wholly dot-matrix product is rare and it means
the wordmark is not a design exercise: it is `TACK` set in Silkscreen, uppercase, +0.06em,
white on true black. Nothing else. Any logo that is *drawn* rather than *set* will look
less like the product than the product does.

The **symbol** is already built and shipping: the five-cell **glyph strip** — 5 squares,
8×8px each, 4px gaps, 56×8px total. In the app it is the only status indicator that exists.
As a mark it is abstract, unmistakable, legible at 16px, and it is the one shape a user will
have seen ten thousand times before they ever look at the logo. **One cell in `#D71921`,
four in white** gives it the single point of tension it needs.

That is the whole identity, and none of it was invented for the brand — it was already the
product.
