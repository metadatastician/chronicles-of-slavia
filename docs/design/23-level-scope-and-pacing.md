# Level Scope & Pacing

This doc exists to answer one question before any Zone B work starts: **how
big is a "zone" actually meant to be?** Nothing prior to this doc states an
intended playtime, footprint, or spatial shape for a level — Zone A's current
built shape has been silently doing double duty as both "the prototype" and
"the template for scope," and those may not be the same thing.

## Measured baseline — what Zone A actually is today

Not an estimate; read directly from the shipped renderer.

- **The playable space is a single, fixed, non-scrolling screen.**
  `engine/slavia-zone-a/src/render.rs::setup` spawns one `Camera2d` and never
  moves it — there is no camera-follow system anywhere in the file.
- **Horizontal movement is hard-clamped to `-620.0..620.0`**
  (`render.rs::physics`, `g.x = g.x.clamp(-620.0, 620.0)`) — a **~1240px**
  total span, all of it visible at once inside the one static camera frame.
  This is smaller than "a couple of screens" — it *is* one screen.
- **Progression is a single flat line.** `Session` in
  `engine/slavia-zone-a/src/session.rs` models each girl's position as a
  float `0.0..=6.0` ("beat units") along `beats_slice()`, which is just
  `&self.world.spec().beats` — the SGS's flat `Vec<Beat>`, seven entries, no
  grouping, no branching, no rooms.
- **Playtime is a few minutes.** Seven beats (forest entrance → bird grove →
  shrine → stream bridge → ridge path → overlook → fracture prelude), each a
  short interaction or a line of text, walked once, left to right.

This shape was correct for what Zone A was for: proving the emotional grammar
(stir/settle, cooperation, the Rift's interruption) in the smallest possible
space. It was never claimed to be the target size for a shipped level — but
absent this doc, nothing said otherwise either, so it was the *de facto*
scope template by default.

## Stated intent

In design discussion, the owner was explicit that this default should not
carry forward: future zones are meant to be **substantially larger than Zone
A's current footprint** — an explorable space that plays for several minutes
on its own, not a short linear vignette chained to four others. The
comparison offered was deliberately non-literal (not a specific named game to
match beat-for-beat), just a feel: **closer to a proper level you spend real
time in**, not a two-screen scene.

This is recorded here as **direction, not a locked number** — see Open
Questions below for what still needs an actual figure attached to it.

## What this changes, mechanically

Not a call to refactor anything yet — just naming, honestly, what the two
facts above (measured baseline vs. stated intent) collide with, so nobody
discovers it by accident mid-build.

**Design side.** "Several minutes, explorable" is not simply "more beats on
the same line." A seven-beat line stretched to, say, thirty beats would still
be one corridor — likely to feel padded rather than explorable. Zones sized
to the new intent probably need **distinct areas** (a grove, a ravine, a
shrine clearing, each with their own micro-puzzle) connected by traversal,
rather than one long straight path.

**Architecture side.** Two concrete things in the current engine would not
support that shape as-is:
- The renderer's camera never moves and the play area is hard-clamped to a
  single ~1240px span — there is no code path for "the level is wider than
  one screen" today.
- `Session`'s position model is one float per girl along one flat
  `Vec<Beat>` — there is no concept of a room, a branch, or "which area am I
  in," only "how far along the line."

Both are described here as **facts to plan around later**, when Zone B (or a
Zone A extension) is actually scoped — not a prompt to start changing engine
code now.

## Open questions for the owner

These need real answers — a number, a shape, a yes/no — before the next zone
can be built against a genuine target instead of an inherited default:

1. ~~**Rough target playtime per zone.**~~ **Answered.** Not a single figure —
   zones are meant to vary in length, with the majority running **long**
   (several zones' worth of "explorable, several minutes") and a minority
   running **shorter**, closer to Zone A's own scale. No zone should be
   assumed uniform in length by default; each zone's length is a per-zone
   design call, not a fixed template. (This mirrors Zone A's own internal
   asymmetry — its five success-condition beats and seven traversal beats
   are not evenly weighted either.)
2. **Spatial shape.** Is a zone meant to be one continuous scrolling space
   (still one path, just a longer one), a set of **discrete connected rooms**
   (metroidvania-style, separate screens joined by transitions), or a
   **hub-and-spoke** shape (a central point the girls return to between
   excursions)? Each implies a different fix to `Session`'s and `render.rs`'s
   current one-line, one-screen model.
3. **Does the scale-up apply to all five zones?** The five-zone map
   (`08-level1-five-zone-map.md`) lists A through E. Does Zone A stay a
   deliberately compact prologue — the calm, small "before" — while B onward
   grow into the new target size? Or does every zone, A included, scale up
   together?
4. **Does Zone A itself get expanded**, retroactively, to match whatever
   target is set — or is it preserved exactly as built, as a short, settled
   introduction that the rest of the game grows out from?

## Scope of this doc

Documentation only. No SGS schema change, no Rust change, no Zone B content
proposed here — this exists solely to put a real decision in front of the
owner before Zone B (or any zone-sized engine work) gets planned against
Zone A's shape by default.
