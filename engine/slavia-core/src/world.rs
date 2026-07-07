// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>

//! The live world: which girl is active, what the Rift has done, and the
//! handful of state transitions that make up Zone A.

use crate::animal::Response;
use crate::character::{Character, Influence};
use crate::error::CrossError;
use crate::spec::Spec;

/// The mutable state of a play session over a [`Spec`].
///
/// The core exposes only the transitions Zone A needs. Everything the renderer
/// wants to *show* it reads back off these fields; everything it wants to *do*
/// goes through these methods, so the rules can never be bypassed.
#[derive(Debug, Clone)]
pub struct World {
    spec: Spec,
    active_id: String,
    /// Whether the Rift has awakened (the Fracture beat).
    pub rift_active: bool,
    /// Whether the bridge has been steadied by a settling character.
    pub bridge_stable: bool,
    /// Whether a stirring character has crossed the steadied bridge.
    pub crossed: bool,
}

impl World {
    /// Begin a session. The first character in the spec starts active.
    pub fn new(spec: Spec) -> Self {
        let active_id = spec
            .characters
            .first()
            .map(|c| c.id.clone())
            .unwrap_or_default();
        World {
            spec,
            active_id,
            rift_active: false,
            bridge_stable: false,
            crossed: false,
        }
    }

    /// The spec this world is playing.
    pub fn spec(&self) -> &Spec {
        &self.spec
    }

    /// The character the player currently controls.
    pub fn active(&self) -> &Character {
        self.spec
            .character(&self.active_id)
            .expect("active character id is always a valid spec character")
    }

    /// Switch control to another character. Returns `false` (and changes
    /// nothing) if no such character exists.
    pub fn switch_to(&mut self, id: &str) -> bool {
        if self.spec.character(id).is_some() {
            self.active_id = id.to_string();
            true
        } else {
            false
        }
    }

    /// The active character reaches out to an animal. Returns `None` if there
    /// is no such animal; otherwise the animal's [`Response`] under the current
    /// world state (notably whether the Rift is active).
    pub fn approach(&self, animal_id: &str) -> Option<Response> {
        let animal = self.spec.animal(animal_id)?;
        Some(animal.respond(self.active().influence, self.rift_active))
    }

    /// Steady the bridge. Only a *settling* character can — this is Donna's act.
    /// Returns `false` if the active character stirs rather than settles.
    pub fn stabilize_bridge(&mut self) -> bool {
        if self.active().influence == Influence::Settle {
            self.bridge_stable = true;
            true
        } else {
            false
        }
    }

    /// Cross the bridge. Only a *stirring* character crosses, and only what a
    /// settling character has already steadied — "Donna steadies what Anya
    /// crosses."
    pub fn cross(&mut self) -> Result<(), CrossError> {
        if self.active().influence != Influence::Stir {
            return Err(CrossError::NotTheCrosser);
        }
        if !self.bridge_stable {
            return Err(CrossError::BridgeUnstable);
        }
        self.crossed = true;
        Ok(())
    }

    /// The Fracture: the Rift awakens. From here, animals stop answering normally.
    pub fn awaken_rift(&mut self) {
        self.rift_active = true;
    }
}
