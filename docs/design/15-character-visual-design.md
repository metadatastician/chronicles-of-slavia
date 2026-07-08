# Character Visual Design — Anya & Donna

Their look is culturally grounded (`09-anya-and-donna-backstories.md`: Anya
Ukrainian/Carpathian, Donna Bulgarian/Rhodope) and their *motion* encodes their
natures (`06`, `13`: Anya = energy/chaos, Donna = stability/order).

## Anya — Hutsul vyshyvanka (Carpathian, Ukraine)

The Hutsuls are a highland people of the Ukrainian Carpathians; their dress is
vivid and ornate — the visual of **energy**.

- **Sorochka** — white linen chemise, sleeves and chest heavy with **geometric
  cross-stitch** in dominant **red and black** (rhombs, crosses, zig-zags), with
  gold accents.
- **Keptar** — embroidered sheepskin vest, tan with red trim.
- **Skirt / zapaska** — deep red with woven gold stripes and an embroidered hem.
- **Namysto** — strings of red coral beads at the neck.
- **Vinok** — a flower wreath worn by unmarried girls, with long trailing
  **ribbons**.

## Donna — Rodopska nosia (Rhodope mountains, Bulgaria)

Note the terminology: **Северняшка / Severnyashka** is the folk region of
*Northern* Bulgaria (the two-apron costume) — **not** the Rhodopes. Donna is
canonically Rhodope, so her costume is the **Родопска / Rodopska** complex, which
is also the sober, grounded one — the visual of **stability**.

- **Sukman** — a dark (near-black), heavy, sleeveless pinafore dress over the
  chemise; ankle-length and grounded.
- **Riza** — white chemise beneath, embroidered sleeves showing.
- **Woven apron (prestilka)** — earthy Rhodope reds/blacks in geometric stripes
  (the Rhodopes are known for their woven wool textiles).
- **Woven belt** + **headscarf** — hair covered and neat; little movement.

## Motion encodes personality (the reactive rig)

Both girls share one procedural rig; a per-character **`verve`** value scales how
energetically the body reacts — Anya `1.0`, Donna `0.42`.

| Reactive element | How it responds | Anya (high verve) | Donna (low verve) |
|---|---|---|---|
| Body lean | leans into horizontal velocity | pronounced | slight |
| Walk bounce | vertical bob on the walk cycle | springy | planted |
| Jump height | scaled by verve | springs high | low, heavy |
| Skirt | narrows rising, **billows** falling | flares freely | heavy, damped |
| Hair / ribbons | trail with speed, lift when falling | stream far | (scarf) barely moves |
| Hem sway | sways with velocity + walk | lively | subdued |
| Reaction speed | how fast vx eases to input | quick | measured |

Actions wired: walk, **run** (Shift), **jump** (Space), **duck** (↓, squashes),
land, fall. "Lift" (a Donna ability, `04`) is not in Zone A yet.

## Status / notes

Implemented in the JS prototype (`prototype/zone-a/index.html`) as schematic but
detailed procedural art at ~92 px tall. It is an *evocation* of the embroidery at
game scale, not a literal reproduction — a higher-fidelity portrait/sprite pass
is a later art task. The same `verve`-scaled motion model should carry into
whatever L3 engine M2 chooses (`13`), since it is view-layer, not rules.
