// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>

//! # `esm` — the Epistemic State Machine (the NPC *mind*)
//!
//! Architecture: `docs/decisions/0004-hybrid-fsm-esm-mind-body-architecture.adoc`.
//! Design: `docs/design/20-cognitive-npcs-and-theory-of-mind.md`.
//! Authoring boundary: `docs/decisions/0006-sgs-is-the-public-authoring-schema.adoc`.
//!
//! ## The one idea
//!
//! **An NPC knows what it has seen, heard, or been told — and nothing else.**
//!
//! There is no omniscient global trigger. A guard is not hostile because the
//! engine knows you took the artifact; he is hostile because *he* came to
//! believe it, and the guard next to him may be wrong about it forever.
//!
//! This makes `03-living-taxis.md`'s word "witnesses" executable, and turns its
//! diagnostic rule (*if an animal does not respond naturally, this is
//! meaningful*) from a per-encounter authored flag into a **comparison**: does
//! this creature's belief predict the response its essence
//! (`07-species-essence-and-control.md`) says it should have?
//!
//! ## The shape
//!
//! | Module | Is |
//! |---|---|
//! | [`kanren`] | The relational solver. microKanren: unify / fresh / conj / disj / run. **The load-bearing, fiddly part.** |
//! | [`belief`] | Global Baseline + per-entity Deltas. An entity's belief is `baseline + its own deltas`, full stop. |
//! | [`intent`] | Reading a mind's guess about the player. Confidence *is* the solution count. |
//! | [`decay`] | Memory eroding with motion, deterministically. |
//!
//! ## Two rules that are not negotiable
//!
//! **1. Determinism.** No ambient RNG, no clock, no global state. Randomness is
//! *injected* as a seeded [`decay::Rng`]. Same seed + same inputs = same beliefs,
//! always. This is what lets L2 be proven headlessly (ADR-0004), what makes
//! authored content behave identically for every player (ADR-0006), and — for
//! free — what any future lockstep netcode would require.
//!
//! **2. Event-gated, never per-frame.** The solver runs only on *epistemic
//! events* (a visual or acoustic change). The FSM (L3/Bevy) ticks at 60 Hz; the
//! ESM only *notices*. Do not call [`kanren::run`] from a frame loop.
//!
//! ## Mechanism vs. content (ADR-0006)
//!
//! Per ADR-0006 §3, **AI authoring is explicitly out of scope for UMS here.**
//! That fixes the boundary:
//!
//! * **Mechanism (this module, Rust):** the solver, the relations, decay, intent
//!   classification. Modders do not write new reasoning.
//! * **Content (SGS data):** *which* entities have minds, and *what they start
//!   believing*. A modder placing a suspicious miller writes TOML.
//!
//! So: never enumerate facts, entities, or scene specifics in here. This module
//! supplies the *engine of thought*; the scene supplies the *thoughts*.
//!
//! ## Status — what is built, and what is deliberately not
//!
//! Built and proven against a synthetic cast in `tests/esm_testbed.rs`. Zone A's
//! five beats are untouched; nothing here is wired to a zone yet, by design.
//!
//! **Deliberately not built** (each marked at its site where one exists):
//!
//! * **Factional reputation / the rotating sentry.** `00-start-here.md` parks
//!   "all NPC factions", and asymmetric memory across outposts *structurally*
//!   needs a second location — it is the natural trigger for Zone B, once Zone A
//!   holds. Note the epistemics are already free: asymmetric memory simply *is*
//!   per-entity deltas ([`belief::Mind`]).
//! * **Spatial / semantic ambiguity** (`20-cognitive-npcs...` §C) — "turn right
//!   at the gap", "crimson flower" vs. "scarlet weed". Needs the flora
//!   vocabulary of `16-flora-and-non-animal-npcs.md` to be real first.
//! * **Lazy streams.** See [`kanren`]'s note. Eager `Vec` is a deliberate,
//!   documented trade (ADR-0004), not an oversight.
//! * **Wiring to a zone / SGS parsing of minds.** The `spec` module does not yet
//!   read baselines or minds. That is the next real step, and it is small.
//!
//! The Guilty Conscience trap (`20` §D) is *not* a module on purpose: it must
//! **emerge** from belief + intent rather than be coded, or the design's central
//! claim is false. It is proven as a test in `tests/esm_testbed.rs`.

pub mod belief;
pub mod decay;
pub mod intent;
pub mod kanren;

pub use belief::{Baseline, Delta, Fact, Mind};
pub use decay::{retained_instructions, Rng};
pub use intent::{read_intent, Read};
pub use kanren::{conj, conj_all, disj, disj_any, eq, fresh, membero, run, Goal, State, Term, Var};
