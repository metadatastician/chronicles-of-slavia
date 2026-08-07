# Architecture

> Authoritative companion docs: ADR-0002 (layered game-as-data), ADR-0003 (Bevy as
> L3), ADR-0004 (hybrid FSM/ESM), ADR-0006 (the SGS is the public authoring schema),
> and `engine/README.adoc`.

## The one idea

**The game's meaning lives in data and a pure rules core; the renderer is a skin.**

Everything below follows from that. It is why the design canon in `docs/design/` can
be compiled into data rather than scattered through renderer code, why the rules core
can be proven by tests before any pixel exists, and why Bevy is a *choice* rather than
an owner.

## Three layers

```
┌─ L1 ── Slavia Game Spec (SGS) ─────────────────────────────────┐
│  engine/slavia-core/data/zone-a.sgs.toml                       │
│  Declarative TOML: characters, animals, beats.                 │
│  Change the game by changing this data. (ADR-0006)             │
└────────────────────────────┬───────────────────────────────────┘
                             │ parsed by
┌────────────────────────────▼───────────────────────────────────┐
│  L2 ── slavia-core          (the sacred logic)                 │
│  world.rs · character.rs · animal.rs · spec.rs · esm/          │
│  The emotional grammar, living taxis, spec parsing, the ESM.   │
│  Deterministic. Renders nothing. serde + toml ONLY.            │
└────────────────────────────┬───────────────────────────────────┘
                             │ consumed by
┌────────────────────────────▼───────────────────────────────────┐
│  L3 ── slavia-zone-a        (a replaceable skin)   (ADR-0003)  │
│  main.rs · state.rs · render.rs · session.rs · save.rs · menu/ │
│  Bevy 0.15. Owns the window, the camera, input, and drawing.   │
└────────────────────────────────────────────────────────────────┘
```

`engine/` is a Cargo workspace (`resolver = "2"`) with exactly these two members.

### Why L2 has only two dependencies

`slavia-core` depends on `serde` and `toml`, and nothing else — deliberately. That
keeps it wasm-friendly, keeps its test suite fast enough to run on every change, and
makes the determinism claim checkable: same seed and same inputs give the same result,
with no runtime, scheduler, or renderer in the way. Adding a dependency here is an
architectural decision, not a convenience.

### The renderer boundary

`session.rs` is the *only* bridge between L2 and L3. It models Zone A as a spatial
walk over the seven beats — a float position per girl in "beat units" — and gates each
interaction (`approach_birds`, `settle_crossing`, `cross`) on standing at the right
beat. It is fully headless-testable, which is why `just test` can exercise gameplay
logic with no display attached.

`render.rs` derives beat position from pixel X against a `BEAT_X` table. That is the
whole coupling: pixels in, beat units out.

## Runtime flow

```
AppState::Opening  →  MenuShell  →  Playing
   title card         8 panels      render::ZoneAPlugin
                      PT Serif      (runs only in AppState::Playing)
                      save/load
```

Defined in `engine/slavia-zone-a/src/state.rs`; `menu::MenuPlugin` drives the
transitions. A single persistent `Camera2d` is spawned once in `main.rs`.

## What is deliberately not wired

The **ESM** (`engine/slavia-core/src/esm/` — `belief`, `kanren`, `intent`, `decay`) is
complete and passes its testbed, but is connected to no scene. That is ADR-0004's
decision, not an oversight: it is the substrate for theory-of-mind NPCs
(`docs/design/20`), and `docs/design/23` names segment A5 as the place it should first
earn its keep. Until then, the honest description is *proven headless, never used*.

Doctrine says "unwired is not done". This is tracked as debt in
`docs/status/DEBT.adoc`, not presented as a finished feature.

## Known architectural limits

Two facts about the current engine constrain everything past Zone A, and are recorded
here so nobody discovers them mid-build (both are named in `docs/design/23`):

1. **The camera never moves.** `render.rs::setup` spawns one `Camera2d` and no
   camera-follow system exists. Horizontal movement is hard-clamped to `-620.0..620.0`
   — about 1240px, all visible at once. There is no code path for "the level is wider
   than one screen."
2. **`Session` has no concept of a room.** Position is one float per girl along one
   flat `Vec<Beat>`. There is no room, branch, or "which area am I in" — only "how far
   along the line."

`docs/design/23` calls for discrete connected rooms, each potentially larger than a
screen. That requires two levels of position (which room, and where within it) and a
moving camera. Neither exists yet; both are prerequisites for A2–A5 and for Zone B.

## Verification

`verification/proofs/` carries proofs across five provers — Idris2, Lean4, Agda, Coq,
and TLA+ — gated by `.github/workflows/proof-gate.yml`, which treats a *missing prover*
as fatal rather than skippable. Per-prover manifests (`verification/proofs/<prover>/MANIFEST`)
mark each module `gated` (must compile) or `quarantine` (known-broken, must keep
failing), and `scripts/check-proofs.sh` is the single source of truth both CI and a
local `just proof-check-all` call.

Scope note, stated plainly: these proofs are currently about a **generic C ABI/FFI
seam**, not about Slavia's game logic. The game's own invariants are held by the Rust
test suite. Closing that gap is tracked in `docs/status/PROOF-NEEDS.adoc` and the debt
register.

## Supporting trees

| Path | What it is |
|---|---|
| `src/interface/` | Zig FFI stub and Idris2 ABI types — RSR template scaffolding for the C ABI seam, not game code. |
| `scripts/` | Estate gates: `check-proofs.sh`, `check-no-md-in-docs.sh`, `check-root-shape.sh`, `scan-dangerous.sh`. |
| `build/just/` | Justfile fragments, imported with `import?` — including `proofs.just`. |
| `.machine_readable/` | Descriptiles (what-is), contractiles (normative set-point), k9 (validation). |
| `prototype/zone-a/` | The superseded browser/canvas mock, kept as visual reference. |

## Design principles in force

- **Game as data.** If a change can be made in the SGS, it belongs in the SGS.
- **Renderer-neutral core.** L2 never learns what a pixel is.
- **Determinism.** Same seed, same inputs, same result — this is testable and tested.
- **Honest gates.** A check that cannot fail is a defect, not a pass. Missing prover =
  failure, never skip.
- **Fail forward.** No temporal rollback; failure has consequences that stand
  (ADR-0005).
