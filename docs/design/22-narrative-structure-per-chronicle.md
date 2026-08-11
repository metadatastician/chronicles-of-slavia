# Narrative structure — a different shape per Chronicle

Companion to `13-chronicles-trilogy-structure.md` (what each Chronicle *is*) and
`19-chronicle-detail-imported.md` (imported detail and its conflicts). This doc
is about **shape**: the structural form each Chronicle takes, and what that form
obliges mechanically.

**Status: proposal.** Owner-authored blueprint, recorded for preservation.
Nothing here is scheduled; built scope remains Zone A (`00-start-here.md`).

## The three shapes

| Chronicle | Structure | The move |
|---|---|---|
| **I — The Broken Frontier** | **Kishōtenketsu** | The Grounding & The Realignment |
| **II — The Echo Engine** | **Chiasmus / Ring Composition** | The Symmetrical Descent |
| **III — The Two Who Were Needed** | **Fichtean Curve** | The Compounding Shatter |

### Chronicle I — Kishōtenketsu

Establish deep systemic roots, durable state changes, and anchor the player
thoroughly into the world's standard operating rules **without burning them out
on immediate, exhausting world-ending stakes**.

*Ki* (introduction) and *Shō* (development) let the player inhabit the system
naturally — building nuanced character networks, ambient training loops,
authentic connections, stable baselines. The ***Ten*** **need not be violent**: it
can be a paradigm-shifting realization, or a deep architectural change in the
state of the world. The *Ketsu* leaves the world **changed but structurally
balanced** — a rich baseline for what comes after.

The essential property: Kishōtenketsu is a four-part structure that does **not
require conflict** to generate meaning. The twist reframes rather than defeats.

### Chronicle II — Chiasmus / Ring Composition

The middle Chronicle is a deliberate, echoing reflection of the first, pulling
inward toward an inevitable core.

Where Chronicle I went **A → B → C**, Chronicle II mirrors back **C → B → A** —
same events, locations, and psychological milestones, but with altered
architectural variables and everything structurally unravelling. The **exact
centre of the ring** is both the mechanical and thematic turning point: the
realization that the stable system established in Chronicle I is **collapsing
under its own weight**.

The feeling to produce: *elegant, deterministic containment* — right before the
dam breaks.

### Chronicle III — Fichtean Curve

The final Chronicle shatters the luxury of reflection. The player begins
**already mid-crisis**, handling the direct fallout of Chronicle II's closing
mirror. Pacing becomes a series of rapid, compounding peaks where **every attempt
to stabilize the world triggers a more intense reactive complication**.

## Mechanical pacing must match the shape

The structure is not a description of the plot; it dictates the loop.

* **Kishōtenketsu (I)** — high agency, wide exploration windows, deep systemic
  experimentation, and **extensive downtime for systemic integration**. The
  player is being taught a world, not chased through one.
* **Chiasmus (II)** — structural *predictability* mixed with growing
  environmental and behavioural **decay**. The player revisits known systems and
  states, but the input variables are changing or corrupting underneath them.
* **Fichtean (III)** — **shrinking decision windows**, high-frequency systemic
  stressors, and mechanics driven by **mitigation rather than optimization**. The
  player is managing a cascading failure state, not playing well.

That last line is the sharpest thing in the blueprint: *mitigation rather than
optimization* is a complete re-specification of what skill means, and it is
achievable only if Chronicle I taught optimization first.

## State-driven continuity — the seams carry the weight

Because the structural paradigm shifts between entries, the transitional
**seams must be rock-solid**. A Fichtean third Chronicle only lands if the player
feels the direct, compounding weight of the **durable state changes they
engineered** during Chronicle II's symmetrical loops. The world must not reset:
Chronicle III's crisis points should read as the **logical, systemic consequences
of earlier choices**.

### This is ADR-0005 at trilogy scale

`docs/decisions/0005-fail-forward-no-temporal-rollback.adoc` bans temporal
rollback *within* a game: the world only moves forward, failure has consequences
that stand, and the bonfire resets *diegetically* rather than by rewinding.

The blueprint's "the architecture of the world shouldn't just reset" is **the
same principle at three times the scale**. Fail-forward across a scene; durable
state across a Chronicle; compounding consequence across the trilogy. One design
philosophy, applied at every altitude. That consistency is a good sign, and it
means ADR-0005's cost (every failure needs an authored destination) is buying
something at trilogy scale too.

## Thematic friction is the story

Moving from the contemplative, non-conflict space of Kishōtenketsu to the
rapid-fire crises of a Fichtean curve is a **massive stylistic shift**. Frame that
structural friction **as part of the story**: the loss of the world's initial
structural harmony (I) into frantic, reactive survival (III) *is* the plot of the
trilogy, told through form rather than dialogue.

## Where this rhymes — form becoming content

**Chiasmus for the Echo Engine is almost too good.** Ring composition *is* an
echo structure. Chronicle II is named for a machine that echoes, is built around
**Echo Imprint Levels** (ghost platforms — echoes of Anya's movement), and is now
proposed to be *shaped* as an echo of Chronicle I. The structural device, the
diegetic machine, and the level mechanic are all the same idea at three scales.
Form is content here, and it wasn't forced.

**Two Fichtes, independently.** `19-chronicle-detail-imported.md` proposes the
**Fichtean Anstoss** (mutual constraint — Anya cannot move without Donna's
boundary) for Chronicle III; this blueprint independently proposes the **Fichtean
Curve** for the same Chronicle. These are *different concepts* — Anstoss is
Fichte's philosophical check on the I; the Fichtean Curve is a creative-writing
term for compounding rising action, named for the dialectic. **They must not be
conflated.** But two sources reaching for Fichte for the same Chronicle is at
minimum a strong signal about its character.

## Conflicts with existing canon

### 1. Chronicle I's tone — **RESOLVED**

`13-chronicles-trilogy-structure.md` currently states Chronicle I's tone as
**"surreal, dangerous, fast-paced."**

Kishōtenketsu asks for the opposite: high agency, wide exploration, extensive
downtime, **explicitly no world-ending burnout**. And the imported Act material
(`19`) sides with `13`, not the blueprint — its Chronicle I has the earth
physically rupturing and a world collapsing.

So two of three sources say Chronicle I is fast and dangerous; the blueprint says
it must be slow and grounding.

**But the built game sides with the blueprint.** `08-level1-five-zone-map.md`
describes the shipped Zone A as **"calm, folkloric"**, and is explicit that its
final beat is a *"Fracture prelude — a foreshadowing tremor, not the rupture
itself"*. The rupture is deferred to Zone B. What actually exists is Ki and Shō,
with the Ten held back.

**RULING: (a) Rule the calm / Kishōtenketsu reading canon for Chronicle I.**
`13`'s tone line has been amended to "calm, folkloric, with a paradigm-breaking
turn". The built Zone A and the owner-authored blueprint both back this reading;
Kishōtenketsu does not forbid dread or rupture, it locates them (the *Ten*), so
doc 24's terrible ending is fully available inside the calm form.

### 2. Where does mirroring live?

`13` puts **Mirror Levels** ("one girl's mistakes become the other's obstacles")
in Chronicle **III**. The blueprint makes *mirroring itself* the organising
structure of Chronicle **II**. Both can be true — chiasmus is macro-structural,
Mirror Levels are a level mechanic — but the vocabulary will collide and should
be disambiguated before either is built.

### 3. Chronicle III: mythic, or frantic?

`13` gives Chronicle III the tone **"emotional, symbolic, mythic"**, ending in
the guardian spirit's reunion and a final quiet choice. A Fichtean curve is
frantic, compounding, mitigation-driven. A cascade of escalating crises is not
obviously the same register as a mythic reunion.

Possible resolution — and this is a *suggestion*, not a finding: the Fichtean
curve is the **body** of Chronicle III, and the guardian-spirit reveal is its
**Ketsu** — the collapse runs until the only remaining move is the one the whole
trilogy was for. That would make Chronicle III structurally Fichtean and tonally
mythic, with the reunion as the release the compounding earns. It also rescues
`19`'s omission of the guardian spirit, which remains the single biggest gap in
the imported material.

## Open questions for the owner

1. **Chronicle I's tone** — does `13`'s "surreal, dangerous, fast-paced" survive,
   or does Kishōtenketsu win? *The built Zone A currently backs Kishōtenketsu.*
2. **Chiasmus needs Chronicle I's route to exist first.** A C→B→A mirror can only
   be designed once A→B→C is real. This makes Chronicle II *structurally
   dependent* on Chronicle I being finished, not merely sequenced after it. Worth
   knowing before anyone plans them in parallel.
3. **Mirror Levels vs. chiasmus** — disambiguate the vocabulary.
4. **Chronicle III's register** — Fichtean body with a mythic Ketsu, or something
   else?
5. **Does the Kishōtenketsu shape also apply at Level scale?** **RESOLVED** — Rule deliberately fractal at level scale. Zone A (calm
   introduction) → Zone B (the rupture) → Zones C/D (living the consequences
   apart) → Zone E (reunion: *"Two were needed"*) has a suspiciously four-part
   feel. **RULING: Level 1 mapping: Ki = A1–A2 (inhabiting the world), Shō = A3–A5
   (deepening into people, tools, and each other), Ten = Zone B, Ketsu = the
   C/D/E arc, with Zone E the Ketsu proper.** Fractality applies at level scale
   only; it does not oblige every individual zone to be internally four-part.
   Zone B's B-i…B-v stays five beats. C and D get pacing guidance: consequence-
dwelling (elaboration of the Ten). E is constrained to reframe, not defeat —
   no boss-fight ending for Level 1.
