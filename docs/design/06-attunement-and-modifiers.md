# Attunement & Modifiers — how the living world is moved

Builds on `03-living-taxis.md`. That doc says animals respond to Anya and Donna.
This doc says *why*, *who else can*, and *what changes the strength*.

## Attunement is a signature ability, not a disposition

Affecting animals is a **special ability Anya and Donna carry** — it is part of
who they are, not a generic property that any calm or excitable character would
have. It is **passive**: they change the rhythm of the world by being present.

- **Anya** stirs — excites, quickens, emboldens, agitates.
- **Donna** settles — calms, slows, quietens, reassures.

The animal's own nature still decides the result (`03-living-taxis.md`), and the
Rift still overrides everything once awake (`02-zone-a-design.md`, Fracture).

## Others can be attuned too

Attunement is a *capability an actor holds*, and Anya and Donna are not the only
ones who hold it. **Enemies may manipulate animals** with different effect kinds:

- making them **ferocious** (enrage)
- making them **obedient** (dominate/compel)

So the model is: an actor applies an *effect kind* to an animal. Anya = stir,
Donna = settle, an enemy = enrage / dominate / … . This is why the core must
treat animal-affecting as an ability an actor *has*, not as a side effect of a
stir/settle temperament.

## Magnitude — experience magnifies and hones

The ability is not fixed. With experience it can be **magnified and honed** — a
mastery/strength dimension that grows over the Chronicle. Base strength at the
start; stronger, sharper, more selective later.

## Terrain — home amplifies, opponent attenuates

Each girl is *of* a place, and the land answers her differently:

| Girl  | Home terrain (amplifies) | Opponent terrain (attenuates) |
|-------|--------------------------|-------------------------------|
| Anya  | forest                   | tundra                        |
| Donna | mountains                | swampland                     |

On home terrain the effect is stronger; on opponent terrain it is diminished or
nullified; on neutral ground it is base.

## Objects and curses — the modifier pipeline

On top of magnitude and terrain sit item- and curse-based modifiers:

- **Objects** can **magnify** an effect.
- **Curses** can **attenuate** or even **reverse** an effect (a reversed Anya
  settles; a reversed Donna stirs; or an animal's response is flipped).

## Target resolution pipeline (not yet built)

    effect(actor) × magnitude(mastery) × terrain × objects × curses
      ── gated by ── animal nature
      ── overridden by ── the Rift

## Zone A scope — base only

None of the above is active in Zone A. Zone A ships **only** the base case:
Anya stirs / Donna settles, natural birds, neutral ground, no mastery, no
objects, no curses, no enemy attunement — then the Rift disrupts. Everything
else here is design canon for **later** levels (see `05-parking-lot.md`). Do not
implement the modifier pipeline until Zone A's base beats feel clear.
