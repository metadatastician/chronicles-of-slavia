# Combined Ability System

The moment Anya and Donna stop being two separate characters and become one
dual-force puzzle engine. Built on the same theme named in
`01-world-principle.md`: chaos + order = balance, and balance is what
stabilizes the Rift. Three layers, each requiring more of the bond than the last.

## Architecture note — this is already possible

Everything below assumes one girl can act, then the other, while the first
girl's effect **persists as world state** — exactly the pattern already built in
`slavia-core` (`World::bridge_stable` outlives which character is active;
Donna steadies, then the player switches to Anya, who crosses what Donna left
behind). None of this needs simultaneous two-player control. The engine's
switch-based, persistent-state model was the right foundation for this whole
system, not just for one bridge.

## 1. Synergy Boosts — passive, from proximity

Automatic when the girls are near each other or switched between rapidly.

- **Stability Boost** (Donna calms Anya's chaos) — her platforms collapse
  slower, her dash grows more controllable, Time Flicker lasts a little longer.
- **Momentum Boost** (Anya accelerates Donna's order) — she moves heavy objects
  faster, Reinforce charges quicker, Anchor Step lasts longer.
- **Dual Proximity Aura** — standing close together: hazards slow, platforms
  strengthen, puzzle elements glow to invite interaction. The "you're doing the
  right thing" feedback loop, mechanically.

## 2. Duo Actions — active, requiring both

The core of puzzle design; sequential (via switching), not simultaneous.

- **Chaos–Order Resonance** (signature mechanic) — switch near a special
  object: Anya destabilizes it, Donna stabilizes it, and it transforms into a
  usable element (a platform, a gate, a hidden path, an ancient mechanism).
- **Emotional Echo** — one girl's action leaves a usable trace for the other:
  Anya's dash trail becomes a temporary platform Donna can use; Donna's
  reinforced block gives Anya extra bounce height; Anya's jump arc becomes a
  glowing hint path. Built for split-path or sequential levels.
- **Stacking Mechanics** (Head Over Heels-flavoured, personality-driven) —
  **Lift & Launch** (Donna lifts Anya for a super-jump), **Momentum Push**
  (Anya dashes into Donna for a boosted long-jump), **Dual Weight Switch**
  (both stand on a platform to trip a heavy mechanism). Vertical and weight
  puzzles.
- **Combo Actions** (flashy, "wow moment" pairings) — *Chaos Kick + Order
  Brace* breaks reinforced barriers; *Impulse Jump + Anchor Step* creates
  mid-air platforms; *Dual Pulse* clears corrupted zones or stuns Rift
  creatures.

## 3. Resonance Powers — world-changing, tied to the Rift

Used sparingly: level endings and major story beats.

- **Dual-Charge Stabilization** — both on linked platforms; Anya injects
  chaos, Donna injects order, the Rift Node stabilizes, the world calms.
  **The level-completion mechanic** (Zone E, `08-level1-five-zone-map.md`).
- **Balance Surge** — unlocked later in Chronicle I. A shockwave of harmony:
  resets unstable terrain, reveals hidden paths, freezes hazards briefly,
  weakens Rift creatures. The "ultimate ability," for boss encounters.
- **Unity Shift** — unlocked in Chronicle II, foreshadowed here. Switching
  mid-movement creates hybrid actions: Anya jumps, switch, Donna lands with
  reinforced impact; Donna throws, switch, Anya dashes to redirect it; Anya
  wall-runs, switch, Donna stabilizes the wall. The advanced-mastery mechanic —
  pairs naturally with the aura mastery/control arc in `06`.

## Why it's kept, and where it sits

It makes their personalities, their relationship, and their cooperation all
mechanically load-bearing at once, gives a large variety of puzzle types
without inventing new abilities beyond the ones already named
(`04-classes-parking-lot.md`, `09-anya-and-donna-backstories.md`), and its
level-ending mechanic is already anchored to Zone E.

**Not in Zone A.** Zone A's one Duo-Action-shaped beat (Donna steadies, Anya
crosses) stays as-is — the base case, unnamed, ungeneralized. This system is
canon for Chronicle I proper, once there is more than one bridge to apply it to.
