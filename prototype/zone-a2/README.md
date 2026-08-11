<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
<!-- Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk> -->

# Segment A2 — Animals & Plants (interaction mock)

**Status: mock.** Not canon, not engine code, not a build. It exists to make A2's
mechanics playable enough to argue about, the same way `prototype/zone-a/` did for
Zone A before the Bevy implementation and `docs/design/chronicles-landing-page.html`
did for the menu.

## Run it

```sh
xdg-open prototype/zone-a2/index.html     # or just open the file in a browser
```

No build step, no dependencies.

## Test it

```sh
bun prototype/zone-a2/rules.test.js
```

23 assertions drive the mock's actual rule functions headlessly and check them
against the canon they claim to implement — including the *negative* constraints,
which are the ones easiest to violate by accident.

## Controls

| Key | Action |
|---|---|
| `A` / `D` or `←` / `→` | Move |
| `Shift` | Run |
| `Space` / `↑` | Jump |
| `Tab` | Switch between Anya and Donna |
| `E` | Reach out — stir if Anya, settle if Donna |
| `G` | Gather |
| `N` | Design notes overlay |
| `R` | Restart |

## What A2 teaches

Spec: `docs/design/23-level-scope-and-pacing.md`, with `03` (living taxis), `16`
(flora & non-animal beings) and `17` (clothing).

**Animals, beyond the single grove encounter A1 teaches.** The girl chooses the
verb; the creature's own nature chooses the meaning:

- **hare** — stirred it *bolts*; settled it *freezes*. Neither is "calm". This is
  the clearest statement of the rule: stirring is not "make braver", settling is
  not "make happier".
- **goat** — domesticated, so the answer also *reveals* that it is not wild
  (doc 16's tier hierarchy).
- **magpie** — blighted. It does not answer at all, and **the failure is the
  diagnostic**. This is the beat Zone B's dread movement is later built from.

**Plants, in three harvest types** (doc 16's axis B) — functional only:

- **food** — edible *or* poisonous. Helps a little, or harms. Never "healing".
- **craft fibre** — for the physical repair of cloth.
- **pattern dye** — petals and pigment, gathered here and carried into A3, which is
  where weaving happens. One of doc 23's deliberate cross-segment threads.

**Repair and pattern are sourced differently** (ruled 2026-08-07):

| Axis | Materials |
|---|---|
| **Repair** (armour value) | plant craft fibre **and** harmless animal material — **shorn wool** above all |
| **Pattern** (the upgrade layer) | **plants only** — fibres, pigments, petals. No exception. |

The governing test is **cost to the creature, not origin of the material**. Shorn
wool costs a sheep nothing; taking anything at a living animal's expense stays
forbidden. The mock states this without a word of UI: **the ewe stands beside her
own fleece, in full coat, entirely unbothered.** She is never a gather target —
the fleece is a site in the world, already given.

The asymmetry isn't arbitrary. Repair is *material* — any freely-given fibre will
serve. A pattern is a *meaning*, read by NPCs and spirits, and its meaning is
carried by what the land itself yields.

**The two signature restoratives**, which encode the girls' natures in how they are
*obtained* rather than in a stat:

- **Honey — Anya's.** Wild, foraged, found.
- **Yogurt — Donna's.** Made, set, waited for.

The co-op grammar still governs: the hive must be **settled by Donna** before
**Anya** can take the honey. Anya cannot settle it — stirring a hive makes it
worse. The wild item is reachable only through order.

## Deliberately out of scope

- **No taxis on plants, and no tropism.** Doc 23 rules this out of A2 "and for some
  while after": any effect the girls have on plants is a *later*, subtle, untutored
  reveal — and one that **the girls themselves do not know they can do**. Nothing
  here hints at it. The test enforces this.
- **No pattern weaving** — that is A3. A2 only gathers the materials.
- No combat, no spirits, no crafting recipe tree (doc 16 rules the recipe tree out
  explicitly).

## Note for the engine

This mock scrolls a camera across a **2600px world**. The real renderer cannot:
`render.rs` clamps X to ±620 and never moves its camera, and `Session` models
position as one float along one flat beat list with no concept of a room.

Those two limits — `CODE-1` and `CODE-2` in `docs/status/DEBT.adoc` — are the
**actual prerequisites for building A2**, ahead of any content work. The mock is
partly here to make that concrete.

## Canon question this surfaced — now ruled

`docs/design/16` and `docs/design/23` **disagreed about plants**: 16 gave them the
full stir/settle taxis as a deliberate traversal layer (*"Anya grows a vine to bridge
a gap; Donna retracts a thorn-wall to pass"*), while 23 said no taxis at all — any
effect is tropism, arrives much later, and is unknown even to the girls.

> **RULED 2026-08-07 (owner):** the girls *develop the ability later and must discover
> it themselves*, so **on this level only animal taxis matters**.

Doc 23 governs. Doc 16 has been re-scoped at source: its flora-taxis section now reads
as a **late-game** ability rather than early puzzle furniture, and its claim that
deep-water reeds (`05`) were "the first seeded case" is corrected — those reeds are
**grabbed**, not grown, so deep-water traversal is unaffected.

This mock was already built to 23 and needed no change. `rules.test.js` keeps it that
way: the scope guard fails if any plant is given a taxis response or if tropism
vocabulary appears in the rule functions. The ruling is checked, not just written down.
