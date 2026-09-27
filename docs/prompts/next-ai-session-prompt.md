# Next AI Session Prompt — Chronicles of Slavia

> Copy this entire document into your next AI session as the opening prompt.
> It is designed to be self-contained: read it, then explore the repo.

---

## Context

You are working on **Chronicles of Slavia** — a mythic 2D puzzle-platformer
about two girls, Anya and Donna, whose bond shapes a world of memory,
medicine, folklore, and moral choice. The central strapline is:

> *What do you become when the world breaks?*

The game is built in **Rust** with **Bevy 0.15** as the L3 renderer, over a
pure, deterministic, renderer-neutral rules core (`slavia-core`). The
architecture is three-layered (ADR-0002):

1. **L1 — SGS (Slavia Game Spec)**: declarative TOML data files
2. **L2 — slavia-core**: the sacred logic (serde + toml only, no renderer)
3. **L3 — renderer crates**: Bevy skins per zone, consuming L2

The repo has been through a major infrastructure overhaul (see
`ULTRAPLAN-2026-09-27.adoc` for what was done). All standards, CI/CD,
template residue, vestigial trees, and structural issues have been resolved.
The directory structure now reflects the full game's eventual shape — every
system, zone, asset type, and documentation area has a home.

## What is built and working

- **Zone A segment A1** (Border Path): playable Bevy renderer, 52 tests passing
- **slavia-core**: emotional grammar, living taxis, SGS parsing, ESM (belief/kanren/intent/decay) — all headless-tested
- **Design canon**: 26 cross-referenced design documents in `docs/design/`
- **8 ADRs** (+ 2 new: ADR-0009 proof slimming, ADR-0010 integration surface)
- **UMS profile**: validated Zone A / Border Path profile (descriptive, no runtime loader)

## What is scaffolded but empty

The following systems have Rust module stubs (compile, but `todo!()` or empty
structs) and SGS data placeholders. They are ready to be filled in:

### slavia-core modules (L2)
- `attunement/` — Aura, taxis, terrain, mastery (docs/design/06)
- `emotional/` — Personal state, relational bond, world reaction (docs/design/10)
- `abilities/anya/` — Dash Burst, Impulse Jump, Time Flicker, Chaos Pulse, Momentum Chains
- `abilities/donna/` — Brace, Anchor Step, Heavy Lift, Reinforce, Logic Link
- `abilities/combined/` — Synergy, duo actions, resonance powers (docs/design/11)
- `abilities/classes/` — Runner, Flicker, Wildheart, Pillar, Architect, Guardian (docs/design/04)
- `clothing/` — Repair, patterns, motifs, social reading (docs/design/17)
- `flora/` — Harvest types, tropism (dormant), folk symbols (docs/design/16)
- `spirits/` — Ukrainian, Bulgarian, Rift spirits (docs/design/16)
- `items/` — Consumables, artefacts, crafting
- `npc/` — Disposition, dialogue, faction (docs/design/20, 23)
- `rift/` — Corruption, nodes, distortion (docs/design/14)
- `progression/` — Marking, memory, chronicle state (ADR-0005)

### New engine crates
- `slavia-renderer/` — Camera follow, room management, sprites, parallax, manpu, lighting, transitions
- `slavia-config/` — Settings, accessibility, keybindings, profiles (with default TOML configs)
- `slavia-ai/` — Director, NPC brain, animal AI, pathfinding, Enaction Engine integration
- `slavia-ffi/` — Cleave-aligned integration surface (empty, no FFI needed today)
- `slavia-zone-b/` through `slavia-zone-e/` — Zone stubs

### Zone A renderer additions (slavia-zone-a)
- `src/ui/` — HUD, dialogue, manpu, indicators, inventory
- `src/audio/` — Music, SFX, ambient, voice
- `src/cutscenes/` — Cut scene system
- `src/zones/a1/` through `a5/` — Per-segment logic

### SGS data (engine/slavia-core/data/)
- `zones/zone-a/` — Segments A1-A5 (A1 has real data, A2-A5 are placeholders)
- `zones/zone-b/` through `zone-e/` — Placeholder
- `chronicles/` — Chronicle I/II/III placeholders
- `characters/` — Anya, Donna, Guardian Spirit definitions
- `bestiary/` — Ordinary, domesticated, supernatural, rift-touched
- `flora/` — Food, craft, pattern, sacred plants
- `motifs/` — Berehynia, Kanatitsa, Elbetitsa embroidery patterns
- `schema/` — SGS schema definition

### Assets (assets/)
- `art/concept/` — Concept art directories for all zones, characters, creatures
- `art/sprites/` — Production sprite directories
- `art/tiles/` — Per-zone tileset directories
- `art/patterns/` — Embroidery pattern reference
- `audio/music/` — Per-zone music directories
- `audio/sfx/` — Sound effect categories
- `audio/ambient/` — Per-zone ambient directories
- `video/` — Cutscenes and reference
- `branding/` — Logos, colour palette, style guide

### Documentation
- `docs/encyclopedia/` — Game bible (world, characters, creatures, mechanics, items, culture)
- `docs/narrative/` — Scripts and outlines per Chronicle
- `docs/wikis/` — Player guide, developer guide, modding guide

## Key design principles (read these before writing any game code)

1. **Game as data.** If a change can be made in the SGS, it belongs in the SGS (ADR-0006).
2. **Renderer-neutral core.** L2 never learns what a pixel is (ADR-0002).
3. **Determinism.** Same seed + same inputs = same result. No ambient RNG or clocks in L2 (ADR-0004).
4. **Fail forward.** No temporal rollback; failure has consequences (ADR-0005).
5. **Slavia owns its vocabulary.** Enaction Engine supplies neutral primitives; Slavia keeps its narrative terms (ADR-0007).
6. **The opposition rule is sacred.** What Anya stirs, Donna stills. Neither can do the other's act. This is the game's core mechanic.
7. **Living taxis is nature-gated.** The animal's own essence determines the result. If it doesn't respond naturally, something is wrong.
8. **Unwired is not done.** The ESM is complete but connected to no scene. It earns its place in segment A5.

## What to build next (priority order)

### Immediate: Engine substrate for A2-A5
Two things block every zone larger than one screen:
1. **Camera follow system** — `render.rs` spawns one static `Camera2d`. Implement camera tracking with smoothing.
2. **Room/area model in Session** — Position is one float along one flat `Vec<Beat>`. Needs two levels: which room, and where within it.

### Then: Zone A segments A2-A5
Per `docs/design/23-level-scope-and-pacing.md`:
- **A2** — Animals and plants (functional plants, pattern plants, signature recharge items)
- **A3** — NPCs and clothing as legible identity (artefact obtained, differential NPC reaction)
- **A4** — Puzzle dynamics (wire the named ability rosters from the Little Books)
- **A5** — Synthesis + theory of mind (wire the ESM to a real scene, use the A3 artefact)

### Then: Zone B
Gated on 8 owner rulings (`docs/design/25-zone-b-decision-sheet.md`).

## Owner decisions still outstanding (do not resolve these yourself)

1. Zone B's eight rulings (CAN-3) — the owner must decide these
2. Co-op vs switch-only for Chronicle II (CAN-1) — engine-touching decision
3. Zone B's tone and the terrible ending (CAN-2)
4. `gv-clade-index` registration (CAN-4) — outward-facing

## How to run the game

```bash
cd engine
cargo run -p slavia-zone-a
```

## How to run tests

```bash
cd engine
cargo test -p slavia-core                    # L2 tests (46 tests)
cargo test -p slavia-zone-a --no-default-features  # L3 headless tests (6 tests)
```

Or from the repo root:
```bash
just test          # Both L2 and L3
just test-smoke    # Zone A five-beat smoke test only
```

## Key files to read first

1. `README.adoc` — Project overview
2. `ARCHITECTURE.md` — Three-layer architecture
3. `docs/design/00-start-here.md` — Game grip
4. `docs/design/23-level-scope-and-pacing.md` — Zone A's full scope
5. `docs/design/20-cognitive-npcs-and-theory-of-mind.md` — ESM and ToM
6. `docs/design/06-attunement-and-modifiers.md` — The core mechanic
7. `docs/design/17-clothing-repair-and-pattern-weaving.md` — Identity system
8. `ULTRAPLAN-2026-09-27.adoc` — What was done in the infrastructure overhaul
9. `docs/status/DEBT.adoc` — Remaining debt (mostly resolved now)
10. `docs/status/ROADMAP.adoc` — What's next

## Repository structure map

```
chronicles-of-slavia/
├── engine/                    # Rust/Bevy game code (Cargo workspace)
│   ├── slavia-core/           # L2 — the sacred logic
│   │   ├── src/               # Rules, systems, ESM
│   │   ├── data/              # SGS data files
│   │   └── tests/             # Headless tests
│   ├── slavia-renderer/       # Shared rendering infrastructure
│   ├── slavia-config/         # Game settings
│   ├── slavia-ai/             # Game AI + Enaction integration
│   ├── slavia-ffi/            # Integration surface (empty)
│   ├── slavia-zone-a/         # Zone A (L3, playable)
│   ├── slavia-zone-b/         # Zone B (L3, stub)
│   ├── slavia-zone-c/         # Zone C (L3, stub)
│   ├── slavia-zone-d/         # Zone D (L3, stub)
│   └── slavia-zone-e/         # Zone E (L3, stub)
├── assets/                    # Art, audio, video, fonts, branding
├── docs/
│   ├── design/                # 26 design documents (the canon)
│   ├── decisions/             # ADRs (10)
│   ├── encyclopedia/          # Game bible
│   ├── narrative/             # Scripts and outlines
│   ├── characters/            # Little Books of Anya and Donna
│   ├── wikis/                 # Player, developer, modding guides
│   └── status/                # DEBT, ROADMAP, READINESS, TEST-NEEDS
├── prototype/                 # Superseded browser/canvas mock
├── verification/              # Proofs (Creusot + TLA+)
├── scripts/                   # Estate gates and checks
├── .machine_readable/         # RSR metadata (descriptiles, contractiles)
└── Justfile                   # Build/test/run commands
```

## Important constraints

- **No TypeScript, Python, Node, Go.** Estate language policy. Rust only for game code.
- **L2 dependencies are serde + toml ONLY.** Adding a dependency to slavia-core is an architectural decision.
- **Tests must stay green.** The 52 existing tests are the contract. New code adds tests, never breaks them.
- **Design canon is authoritative.** If code and docs disagree, the docs win (they were decided by the owner).
- **Do not resolve owner-only decisions.** Mark them as outstanding and move on.
