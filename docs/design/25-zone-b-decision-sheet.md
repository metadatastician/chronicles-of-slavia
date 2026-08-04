# Zone B — decision sheet for the eight open questions

## What this is

`24-zone-b-fracture-line.md` closed with eight open questions and the
statement that the draft "should be read as pending all of them." This doc
puts each question in front of the owner as a decision: the context, the
live options, a recommendation with its reasoning, and a blank ruling slot.

**Status: decision sheet awaiting owner rulings — nothing here is canon
until its ruling is filled in.** A recommendation is an argument, not a
default; an unfilled slot means undecided, and doc 24 stays provisional
until every slot is filled. Where a question here restates a standing
question from another doc (Q1 and Q8 both live in
`22-narrative-structure-per-chronicle.md`), the ruling should be recorded
there too — one ruling, applied at source, echoed here.

Per doc 24's own scope line, none of this schedules a build: Zone B remains
unscheduled until Zone A holds (`08-level1-five-zone-map.md`), and A2–A5 do
not exist yet (`23-level-scope-and-pacing.md`).

---

## Q1 — Chronicle I's tone (the standing conflict)

**Context.** `13-chronicles-trilogy-structure.md` says Chronicle I is
"surreal, dangerous, fast-paced"; the owner-authored structural blueprint in
`22` assigns Chronicle I Kishōtenketsu — high agency, wide exploration,
extensive downtime, explicitly no world-ending burnout — and `19`'s imported
material sides with `13`. Two of three documents say fast; the blueprint
says calm. But the **built game already votes**: shipped Zone A is "calm,
folkloric" (`08`), its final beat is a foreshadowing tremor rather than the
rupture, and doc 24's whole Zone B draft is written in the calm register
under provisional authorization.

**Options.**
- (a) Rule the calm / Kishōtenketsu reading canon for Chronicle I; amend
  `13`'s tone line to match.
- (b) Rule fast/dangerous canon; doc 24's B-i–B-iii compress hard, B-iv
  expands, and the shipped Zone A register needs revisiting.

**Recommendation: (a).** The blueprint is owner-authored where `13`'s line
is inherited; the built artifact — the only thing that has actually been
play-tested — backs it; and Kishōtenketsu does not forbid dread or rupture,
it locates them (the *Ten*), so doc 24's terrible ending is fully available
inside the calm form. Ruling (b) means re-registering a shipped zone to
match a sentence, rather than re-writing a sentence to match a shipped
zone. If (a) is ruled: edit `13`'s tone line for Chronicle I to "calm,
folkloric, with a paradigm-breaking turn" (or the owner's words), and mark
`22` open question 1 resolved.

**RULING:**

---

## Q2 — Is the crux playable or authored?

**Context.** B-iv's loyalty trap: one act each, each girl spends hers on
the other, the birch falls unheld. Doc 24 offers (a) the grammar genuinely
forbids defection — separation is systemic; or (b) the player can try to
defect, and failing costs something durable into C/D. ADR-0005
(fail-forward, no rollback) prices every failure at one authored
destination.

**A constraint doc 24 doesn't name: the engine is switch-only.** `08` and
the built engine are single-active-character (the `13` "dual-simultaneous"
line is a flagged, unresolved contradiction — deliberately not decided
here). The crux as drafted has both girls acting in the same beat, which
switch-only control cannot make fully playable. Some part of the crux is
authored no matter what is ruled; the question is really *which part*.

**Options.**
- (a) Systemic-forbid: the player, as the active girl, chooses freely among
  acts — but every act her verb set physically permits at the crux targets
  the *other* girl's peril (Anya cannot settle the birch, Donna cannot stir
  the rockfall — the same verb hole from `06` the whole zone runs on). The
  other girl's reciprocal act is authored. No input is ever rejected;
  defection is not refused, it is *grammatically inexpressible*.
- (b) Attemptable defection: the player can sprint the birch with Anya;
  the attempt fails (canon `09` — "neither abandons the other" — means it
  must), and the failure writes a durable cost into C/D. Under ADR-0005
  this is a full authored consequence branch spanning two zones.

**Recommendation: (a), implemented through the verb hole.** It is the same
mechanism the zone's whole design turns on — the Rift opened the seam along
the hole in the grammar, and the crux closes through the identical hole, so
the separation reads as systemic *twice over*. Option (b) buys "braver" at
the price of a two-zone consequence branch AND an attempt that must be made
to fail — which risks reading as the game overriding input, the one thing
worse than not offering the input at all. The braver-feeling move that (a)
still allows: let the player *see* the self-saving act (the birch is right
there) and understand, in their hands, that their girl has no verb for it.
That understanding, not a refused button, is the trap landing.

**RULING:**

---

## Q3 — Which girl on which shore?

**Context.** The draft sorts each girl onto her own land: Anya → Ukrainian
forest (tilting over-agitated), Donna → Bulgarian mountain (tilting
over-ordered) — "her own nature, unbalanced." The inverse — each stranded
in the *other's* land — reads as estrangement instead of excess. `08` does
not rule on geography; the choice shapes all of C and D's art direction.

**Recommendation: own land, as drafted.** Three reasons. First, `06`
("Separation tilts the world") describes the tilt as toward each girl's
*own* excess — the own-land sorting is the geographic reading of a
mechanic that already exists. Second, the solo zones' pedagogy (`14`'s
Personality Zones: puzzles that punish hesitation / punish rushing) lands
hardest when each girl is drowning in *herself* — A5 already taught
knowing-the-other (theory of mind); C/D teaching know-thyself is new
material, where the inverse sorting would re-teach A5's lesson in costume.
Third, it keeps each land's visual identity coherent with its tilt
(forest/over-agitation, mountain/over-order) instead of requiring each art
direction to express the *other* girl's excess in the wrong landscape.
The cost is symmetry — "trapped in the alien land" is the more
conventionally dramatic image — but this game's drama has never come from
alienness; it comes from the familiar answering wrongly.

**Sub-ruling this decides with it:** B-v's shore hum (Anya's shore high,
Donna's low) and the tilt directions in the handoff contract stand as
drafted.

**RULING:**

---

## Q4 — The marauder thread: closed or dormant?

**Context.** A1's closing beat spends the marauder as a triumph (`23`);
doc 24 deliberately does not reuse them. Do they reappear at all?

**Options.**
- (a) Fully closed at A1 — never seen again in Level 1.
- (b) Dormant-witness — one non-interactive sighting (doc 24's own
  suggestion: witnessed watching the fold from a ridge in B-iv), no
  mechanical role; thread stays available for C/D/E without obligation.
- (c) Active return in Zone B — a human threat during the rupture.

**Recommendation: (b).** (c) is what doc 24's whole inciting-mechanism
argument exists to prevent — a stronger-marauder Ten hollows A1
retroactively. (a) is clean but wasteful: a named human thread that simply
stops existing invites "whatever happened to—" at exactly the moment the
story asks the player to care about consequence. The single witnessed
silhouette does three jobs for one authored beat: it scale-checks the
rupture (someone *outside* the bond sees it — this is not the girls'
private weather), it keeps the human world alive through an inhuman event
(the "rift between nations" of `14` needs humans to be between), and it
plants a free hook C/D/E may take or leave. It must stay non-interactive
and unexplained in B — a figure, then gone.

**RULING:**

---

## Q5 — How legible is the Rift's intent in Zone B?

**Context.** The draft lets the player infer deliberateness ("it needed
them apart") but never states it; `13` holds "the Rift is calling them"
for later in Chronicle I. Should Zone B say it out loud?

**Recommendation: hold the reveal — Zone B may raise the question, never
answer it.** Concretely: everything inferential stays (the two-pitch hum,
the seam feeding on Anya's stir, the mirror-reversed votives, the withdraw
of the animals); no diegetic text confirms intent. The girls may voice
suspicion *as a question* — Anya's register would even carry it lightly —
but nothing on screen states "it wanted this." The draft's own line "the
seam knew they would" must remain authorial/design-doc voice, not surfaced
narration. Reasons: the Kishōtenketsu *Ten* works by realization, and a
stated intent converts realization into information; C and D's dread runs
on the unconfirmed suspicion (each girl alone with the question is solo-zone
fuel); and Zone E needs a revelation payload for the reunion —
`08`'s Rift Node Chamber is where the answer belongs. This also matches the
game's general show-don't-tell contract (`21-manpu-legibility.md`) and the
belief-not-fact epistemology the ESM runs on (`20`): the player holds a
belief about the Rift, exactly as NPCs hold beliefs about the girls.

**RULING:**

---

## Q6 — Zone B's size and room count

**Context.** Per `23`'s answered questions: zones vary in length (majority
long, minority short), built as discrete connected rooms, where a room may
itself be a larger planar space with camera movement. Is Zone B long or
short, and how do B-i…B-v map to spatial structure?

**Recommendation: medium — four rooms, one movement.** Zone B should be
one of the *shorter* zones (the minority class `23` provides for), but not
minimal: doc 24 itself notes the dread movement wants room to breathe.
Proposed mapping:

1. **Room 1 — B-i**, the morning after. Small, warm, walkable; the manpu
   lateness happens here, unremarked.
2. **Room 2 — B-ii**, the wrong answers. The largest exploratory room;
   the mirrored grammar is *played* across several optional encounters —
   this is where "explorable" earns its keep, because dread discovered at
   the player's own pace is worth more than dread on rails.
3. **Room 3 — B-iii + B-iv**, the seam and the fold. One continuous larger
   planar room (camera-following, per `23`'s answered shape): approach,
   the two grammar attempts, then the fold traversal and the crux, with no
   room boundary — and **no save point — between seeing the seam and
   losing the birch.** ADR-0005 fail-forward holds within it; the
   traversal's authored destinations catch failure diegetically.
4. **Room 4 — B-v**, two shores. Small and still; the tableau, the tilt
   beginning, the two exits (one per girl, into C and D).

Save points at the three room boundaries, matching `23`'s
segment-boundary convention. Not A-style internal segments: A1–A5 are five
*lessons*; B is one *movement*, and its rooms are paragraphs, not chapters.

**RULING:**

---

## Q7 — The A3 artefact across the split

**Context.** The girls obtain an artefact from an NPC in A3, inert until
A5, which *uses* it (`23`). Nothing says whether A5 spends it. If it
persists, who carries it across the separation matters for C/D/E; doc 24
also floats the fold splitting it.

**Options.**
- (a) Spent at A5 — Zone B carries no artefact question.
- (b) Persists, carried by one girl across the split.
- (c) Split by the fold — each girl carries half.

**Recommendation: (b), with a dependency flag.** (a) deletes a
cross-segment thread at the exact moment `23`'s fluidity principle ("planted
in one segment, paying off in a later one") would have it pay off again —
the artefact is the only *object* that has lived through the whole level,
which makes it the natural reunion token for E. (c) is the romantic option
but doubles the design load (two halves needing two functions) and risks
gimmick — the *bond* is already the thing the chasm cannot cut (`09`); the
artefact does not need to duplicate that job. Under (b), the sub-question
— which girl — should wait for Q3's ruling and A3's own design (where the
artefact comes from decides what carrying it *means*); the sheet only
recommends: whichever girl's shore is farther from the artefact's origin
carries it, so each solo zone holds one anchor of the other land. Decide
the carrier when A3 is designed, not here.

**RULING:**

---

## Q8 — Kishōtenketsu at level scale (the fractal question)

**Context.** Doc 24 treats Zone B as Level 1's *Ten*, quietly endorsing the
fractal reading `22` (open question 5) flags: A calm → B rupture → C/D
consequences → E reunion has "a suspiciously four-part feel," and `22` asks
that the shape be stated deliberately rather than discovered by accident.

**Recommendation: rule it deliberately fractal, and name the mapping.**
Level 1 runs: **Ki** = A1–A2 (inhabiting the world), **Shō** = A3–A5
(deepening into people, tools, and each other), **Ten** = Zone B, **Ketsu**
= the C/D/E arc, with Zone E the Ketsu proper — changed-but-structurally-
balanced, which is exactly what `08`'s reunion-plus-still-split-world
already describes. What the ruling buys, concretely: **C and D get pacing
guidance for free** — they are not new escalations but consequence-dwelling
(the elaboration of the Ten), which is precisely the register their
know-thyself pedagogy wants; and **E is constrained to reframe, not
defeat** — no boss-fight ending for Level 1, the reunion *is* the
resolution. One guard rail to write into the ruling: fractality applies at
level scale; it does not oblige every individual zone to be internally
four-part (Zone B's own B-i…B-v is five beats and should stay whatever
shape serves it). If ruled: mark `22` open question 5 resolved and note
the mapping there.

**RULING:**

---

## Dependencies between rulings

- **Q1 gates the draft's pacing.** A fast/dangerous ruling reshapes doc 24
  before anything else matters.
- **Q3 feeds Q7's carrier sub-question** (and C/D art direction).
- **Q2's implementation leans on the switch-only engine fact** — if the
  `13` dual-simultaneous contradiction is ever resolved toward
  simultaneous control, Q2 deserves re-opening.
- **Q8 constrains Zones C, D, and E** — it is the ruling with the longest
  reach, and the cheapest to make now.

## Scope of this doc

Documentation only. No SGS content, no engine change, no Zone B build. When
all eight slots are filled, doc 24 stops being provisional; the rulings that
touch `13` and `22` should be applied in those files too (solutions at
source), with this sheet keeping the record of the reasoning.
