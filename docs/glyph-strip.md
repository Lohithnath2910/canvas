# The Glyph Strip

Five dot cells in the top-left of every widget, 8×8 each with 4px gaps.

**This is the only status indicator in the entire product.** No sync icon, no online
dot, no banner, no toast, anywhere — that is rule 3, and it is why the strip has to
carry seven distinct meanings on its own.

Cells are binary: lit, half, red, or off. Nothing in between, no fade, no glow. State
changes step at 120ms per cell and are never interpolated (rule 4).

To see all seven live: focus a note and press **Ctrl+G** to cycle. The title shows which
state you're looking at. Cycling past the last one returns to normal behaviour.

## The four colour values a cell can take

| Value | Dark mode | Meaning |
|---|---|---|
| lit | `#FFFFFF` | full |
| half | 52% white | present but not full |
| red | `#D71921` | error — one of only two places red appears in the product |
| off | 14% white | barely visible, but not invisible: the cell is still *there* |

## The seven states

| State | Cells | What it tells you |
|---|---|---|
| **idle** | 1 half, 4 off | The note exists and is doing nothing. Nobody is in it, nothing is syncing. This is what you see on every note you're not using. |
| **focused** | all 5 lit | Your cursor is in this note. Exactly one widget on the desktop shows this at a time. |
| **syncing** | one lit cell sweeps left→right, 120ms per cell, looping | Changes are in flight to a peer *right now*. It loops only while that's genuinely true — it is never a decorative animation, because an indicator that lies about the network is worse than none. Currently renders as idle, since sync isn't built yet. |
| **offline** | all 5 half | No connection. Deliberately *not* an error — offline is a normal state for an offline-first app, so it reads as "steady, dimmer", not "something broke". Your edits still apply instantly. |
| **peers** | rightmost N lit, filling from the right | N other people are in this note. One cell per person, up to 5. Fills from the right so it's visually distinct from the syncing sweep. |
| **peers-idle** | rightmost N half | Same people, but nobody has touched anything for 30 seconds. Present but quiet. |
| **error** | cell 3 red, rest off | A conflict that needs you. All five step to red once, then cell 3 holds until it's resolved. The only other red in the product is the destructive button. |

## Why you're seeing what you're seeing

The two you noticed are the two most common:

- **One cell filled, dim, when you're not in the note** — that's `idle`. Cell 1 at half,
  cells 2–5 off.
- **All five filled when your cursor is in it** — that's `focused`.

Everything else needs something to actually be happening: a peer connected, a sync in
flight, the network gone, or a conflict. None of those exist yet, so `idle` and `focused`
are currently the only two you can reach without Ctrl+G.

## Deviation from the original spec

§6.2 specifies 3×3 cells with 2px gaps (23×3 total). That proved unreadable on a real
screen, so this build uses **8×8 cells with 4px gaps (56×8 total)** instead. The state
meanings and the binary/stepped rules are unchanged.
