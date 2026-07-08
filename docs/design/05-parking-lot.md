# Parking Lot

Good ideas not for the current prototype.

## Apothecary

A post-Level-1 NPC who introduces class paths through virtue questions, medicine, and diagnosis.

## Home hearth — a safe place they leave from and return to

An Ultima VII-style structure: the girls start from, and periodically return to,
a **safe home place** — a hearth/hub between excursions, rather than a level
that is simply left behind.

The place may be watched over by a **household/hearth spirit** (the domovoi
idea) — a Slavic house-guardian figure. Which specific figure (domovoi, a
Bulgarian *stopan*, or an invented Slavia-native equivalent) is an open detail,
not yet decided; the scene matters more than the name right now. Whatever it
becomes, it should sit comfortably as culturally-grounded Slavic/Balkan
household folklore rather than a generic "wise NPC."

The apothecary (or an alchemist/fortune-teller framing of the same role — detail,
not yet fixed) may live at or near this hearth, so the class-diagnosis scene and
the home-base scene could be the same place.

## Mirror of Misalignment

A corruption boss that weaponizes Anya and Donna's flaws against each other.

## Absence mechanic

If Anya is missing, the world becomes sedated and over-settled.

If Donna is missing, the world becomes turbulent and over-stirred.

Elaborated as a capture/rescue difficulty + reunion mechanic in
`06-attunement-and-modifiers.md` ("Separation tilts the world").

## NPC factions

Forest spirits, mountain spirits, mixed spirits, corrupted spirits.

## Attunement modifiers (see 06-attunement-and-modifiers.md)

Deferred subsystems of the animal ability:

- Enemy animal-manipulation (ferocious / obedient).
- Mastery — the ability magnified and honed with experience.
- Terrain traits — home amplifies, opponent attenuates (Anya forest/tundra, Donna mountains/swampland).
- Object magnifiers.
- Curses that attenuate or reverse effects.

Zone A ships the base case only.

## Tactical freeze / bullet-time (seeded in the prototype)

Only the active girl is simulated, so an inactive girl **holds her pose and
position** — mid-jump she hangs in the air; mid-duck she stays crouched. The
prototype keeps this deliberately (duck now persists across a switch, matching
the jump-hold). With jump→switch→jump you can even stage both girls suspended
in the air together (bounded — you can't infinite-jump, since a jump needs solid
ground).

Full mechanic (later): position each girl tactically while held, then a
**trigger event** springs them all into action at once — cf. *Mutant Year Zero:
Road to Eden*. A natural home for set-piece puzzle/encounter beats; pairs with
the combined-ability system (`11`).

## Water has NATURES — the swimming problem, solved (design note)

The tension: it's out of character for two capable girls to "not swim," but a
game where they swim everything loses water as a barrier. Resolution — **don't
make a blanket no-swimming rule; give water *natures*, exactly like animals have
natures** (`03`, `07`). They swim ordinary water freely; only *specific* water
is a hazard:

- **Shallow / ordinary water = free traversal** — wade, cool off, splash. This
  is where the girls visibly ARE competent swimmers, so the character holds.
  (Ducking in waist-deep submerges to the head — a natural **stealth/hide**
  move, e.g. from the marauder.)
- **Deep "abyss" water = a specific lethal hazard, refused by *judgment* not
  incompetence.** Framings (pick/mix): (1) **Rift-charged** — the deep channels
  are where the world's tear runs; that water is "between", and to enter is to
  be *taken* by the Rift (not drowned — worse/mythic). Ties straight to the core
  theme, and makes **Donna's stilling** the answer (she calms the fracture into
  a crossing). (2) **Cold, fast Carpathian/Rhodope snowmelt torrent** — genuinely
  lethal to anyone; refusing is smart. So the tiered gorge already tells the
  story: they wade and cool off happily; they simply won't dive a black,
  bottomless Rift-torrent.
- **Means open deep water later** (so it's a gated puzzle space, never a flat
  wall): **reeds** to grab, a raft/log or buoyant object, a calm stretch, Donna
  *stilling* a span, currents, hazards. Deep water becomes traversable with the
  right tool — never "you can't swim", always "not THIS, not without means".

Prototype now: gorge is tiered wet(0) → waist(-1, head-if-ducking) → neck(-2) →
abyss(-3, black/menacing, refused). Only Donna's bridge crosses.

## Full Chronicle I

Not yet. First make Zone A work.
