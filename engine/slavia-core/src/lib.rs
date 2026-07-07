// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>

//! # slavia-core — the Slavia rules core (Layer 2)
//!
//! This crate is the *sacred logic* of Chronicles of Slavia: the emotional
//! grammar that governs how the world answers the two girls. It renders
//! nothing and draws nothing. A renderer (Layer 3 — browser, Godot, native)
//! is a skin over this core; the game spec (Layer 1, `data/*.sgs.toml`) is the
//! data this core interprets.
//!
//! The grammar, taken directly from `docs/design/`:
//!
//! * **Anya stirs** — she excites, quickens, emboldens, agitates.
//! * **Donna settles** — she calms, slows, quietens, reassures.
//! * **The animal's own nature determines the result.** A *natural* animal
//!   answers; a non-natural one (possessed, corrupted, false, …) fails to
//!   answer naturally, and that failure is itself a diagnostic signal.
//! * **Donna steadies what Anya crosses** — the bridge beat.
//! * **The Rift interrupts the answer** — once it awakens, animals stop
//!   answering normally at all.
//!
//! Zone A's five-beat success condition is encoded as executable assertions in
//! `tests/zone_a.rs`.

pub mod animal;
pub mod character;
pub mod error;
pub mod spec;
pub mod world;

pub use animal::{Animal, Nature, Response};
pub use character::{Character, Influence};
pub use error::{CrossError, SpecError};
pub use spec::{Beat, Meta, Spec};
pub use world::World;

/// The Zone A spec, bundled at compile time. Convenience for bring-up and tests.
///
/// Parsing the embedded spec cannot fail in a shipped build — the file is part
/// of the crate — so this panics on a malformed spec rather than making every
/// caller handle an impossible error.
pub fn zone_a() -> Spec {
    Spec::from_toml(include_str!("../data/zone-a.sgs.toml"))
        .expect("bundled zone-a.sgs.toml is a valid Slavia Game Spec")
}
