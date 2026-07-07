// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>

//! Animals ("living taxis") and the heart of the emotional grammar:
//! how an animal answers a character's reach.

use crate::character::Influence;
use serde::Deserialize;

/// An animal's own nature. "The animal's own nature determines the result."
///
/// Only [`Nature::Natural`] animals answer the girls normally. Every other
/// nature is a *diagnostic signal* (`docs/design/03-living-taxis.md`): an
/// animal that does not respond naturally is telling you something —
/// possession, corruption, falseness, a masked presence, a tie to the Rift.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Nature {
    Natural,
    Possessed,
    Corrupted,
    False,
    Enchanted,
    /// Afraid beyond normal fear.
    AfraidBeyond,
    /// Masking another presence.
    Masking,
    /// Connected to the Rift.
    RiftTouched,
}

impl Nature {
    /// Whether this animal answers the girls the way an untroubled creature would.
    pub fn is_natural(self) -> bool {
        matches!(self, Nature::Natural)
    }
}

/// A creature in the world.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Animal {
    pub id: String,
    pub species: String,
    pub nature: Nature,
    /// Where the animal lives, if the spec pins it to a place.
    #[serde(default)]
    pub location: Option<String>,
}

/// How an animal answers a character reaching out to it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Response {
    /// A natural animal quickened by a stirring influence.
    Stirred,
    /// A natural animal calmed by a settling influence.
    Settled,
    /// The animal did not answer naturally — meaningful. Carries the nature
    /// that broke the expectation, so the game can act on *why*.
    Unnatural(Nature),
    /// The Rift has awakened; the animal no longer answers normally at all,
    /// whoever reaches out.
    Disrupted,
}

impl Animal {
    /// Resolve how this animal answers `influence`, given whether the Rift is active.
    ///
    /// This single function is the emotional grammar of Slavia. Ordering is
    /// deliberate and load-bearing:
    ///
    /// 1. **The Rift overrides everything.** Once it wakes, "they stop
    ///    answering normally" — natural or not, the answer is [`Response::Disrupted`].
    ///    (The Rift thus also *masks* corruption, which is intended.)
    /// 2. **A non-natural nature betrays itself** by failing to answer
    ///    naturally → [`Response::Unnatural`].
    /// 3. **A natural animal answers the influence** — stirred or settled.
    pub fn respond(&self, influence: Influence, rift_active: bool) -> Response {
        if rift_active {
            return Response::Disrupted;
        }
        if !self.nature.is_natural() {
            return Response::Unnatural(self.nature);
        }
        match influence {
            Influence::Stir => Response::Stirred,
            Influence::Settle => Response::Settled,
        }
    }
}
