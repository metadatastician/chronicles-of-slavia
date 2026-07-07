// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>

//! The play session — the *only* place the renderer touches the rules.
//!
//! `Session` owns a [`slavia_core::World`] and exposes the handful of intents
//! Zone A needs (switch, approach the birds, settle the crossing, cross, awaken
//! the Rift), plus the view-facing state a renderer reads back (bird mood,
//! which beats have happened). It contains **no rendering** and no Bevy types,
//! so it is fully testable headlessly — which is how the five-beat success
//! condition is verified without a display.

use slavia_core::{zone_a, CrossError, Response, World};

/// Zone A's single animal.
pub const GROVE_BIRDS: &str = "grove-birds";

/// The visible mood of the grove birds, derived from the last time a girl
/// reached out to them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BirdState {
    /// Not yet reached — going about their own business.
    Neutral,
    /// Anya raised their taxis.
    Stirred,
    /// Donna lowered their taxis.
    Settled,
    /// The Rift has awoken; they no longer answer normally.
    Disrupted,
}

impl BirdState {
    fn from_response(r: &Response) -> BirdState {
        match r {
            Response::Stirred => BirdState::Stirred,
            Response::Settled => BirdState::Settled,
            Response::Disrupted => BirdState::Disrupted,
            // Unmoved / Unnatural do not occur in Zone A's shipped data.
            _ => BirdState::Neutral,
        }
    }
}

/// The five things a player should come to understand in Zone A
/// (`docs/design/00-start-here.md`), tracked as they actually happen.
#[derive(Debug, Default, Clone, Copy)]
pub struct Beats {
    pub anya_stirred: bool,
    pub donna_settled: bool,
    pub crossed: bool,
    pub rift_disrupted: bool,
}

impl Beats {
    /// "Nature answers them" — established once both girls have been felt.
    pub fn nature_answered(&self) -> bool {
        self.anya_stirred && self.donna_settled
    }

    /// All five beats witnessed.
    pub fn all(&self) -> bool {
        self.anya_stirred && self.donna_settled && self.crossed && self.rift_disrupted
    }

    /// How many of the five have happened (for the on-screen beat pips).
    pub fn count(&self) -> usize {
        [
            self.anya_stirred,
            self.donna_settled,
            self.nature_answered(),
            self.crossed,
            self.rift_disrupted,
        ]
        .iter()
        .filter(|b| **b)
        .count()
    }
}

/// The most recent thing that happened, for console/screen feedback.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Switched,
    Approached(Response),
    Settled,
    CannotSettle,
    Crossed,
    CannotCross(CrossError),
    RiftAwoke,
}

/// A live Zone A play session.
pub struct Session {
    pub world: World,
    pub birds: BirdState,
    pub beats: Beats,
    pub last: Option<Event>,
}

impl Session {
    /// Start a fresh Zone A session (Anya active first).
    pub fn new() -> Self {
        Session {
            world: World::new(zone_a()),
            birds: BirdState::Neutral,
            beats: Beats::default(),
            last: None,
        }
    }

    pub fn active_id(&self) -> &str {
        self.world.active().id.as_str()
    }

    pub fn active_name(&self) -> &str {
        self.world.active().name.as_str()
    }

    /// Switch control to `id`; `false` (no change) if there is no such girl.
    pub fn switch(&mut self, id: &str) -> bool {
        let ok = self.world.switch_to(id);
        if ok {
            self.last = Some(Event::Switched);
        }
        ok
    }

    /// Switch to the other girl (Zone A has exactly two).
    pub fn toggle_character(&mut self) {
        let other = self
            .world
            .spec()
            .characters
            .iter()
            .map(|c| c.id.clone())
            .find(|id| id != self.active_id());
        if let Some(other) = other {
            self.switch(&other);
        }
    }

    /// The active girl reaches out to the grove birds.
    pub fn approach_birds(&mut self) -> Response {
        let r = self
            .world
            .approach(GROVE_BIRDS)
            .expect("Zone A always has the grove birds");
        self.birds = BirdState::from_response(&r);
        match r {
            Response::Stirred => self.beats.anya_stirred = true,
            Response::Settled => self.beats.donna_settled = true,
            Response::Disrupted => self.beats.rift_disrupted = true,
            _ => {}
        }
        self.last = Some(Event::Approached(r.clone()));
        r
    }

    /// Try to settle the crossing (only a lowering gift — Donna — can).
    pub fn settle_crossing(&mut self) -> bool {
        let ok = self.world.stabilize_bridge();
        self.last = Some(if ok {
            Event::Settled
        } else {
            Event::CannotSettle
        });
        ok
    }

    /// Try to cross (needs the crossing to have been made passable).
    pub fn cross(&mut self) -> Result<(), CrossError> {
        let res = self.world.cross();
        match &res {
            Ok(()) => {
                self.beats.crossed = true;
                self.last = Some(Event::Crossed);
            }
            Err(e) => self.last = Some(Event::CannotCross(e.clone())),
        }
        res
    }

    /// The Fracture: awaken the Rift.
    pub fn awaken_rift(&mut self) {
        self.world.awaken_rift();
        self.last = Some(Event::RiftAwoke);
    }

    pub fn crossing_passable(&self) -> bool {
        self.world.bridge_stable
    }

    pub fn rift_active(&self) -> bool {
        self.world.rift_active
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The whole Zone A arc, driven through the exact intents the renderer
    /// calls — the headless proof that the input wiring is correct.
    #[test]
    fn walks_the_five_beats() {
        let mut s = Session::new();

        // 1. Anya stirs.
        assert_eq!(s.active_name(), "Anya");
        assert_eq!(s.approach_birds(), Response::Stirred);
        assert_eq!(s.birds, BirdState::Stirred);
        assert!(s.beats.anya_stirred);

        // 2. Donna settles.
        s.toggle_character();
        assert_eq!(s.active_name(), "Donna");
        assert_eq!(s.approach_birds(), Response::Settled);
        assert_eq!(s.birds, BirdState::Settled);
        assert!(s.beats.donna_settled);

        // 4. Nature answered them (established by 1 & 2).
        assert!(s.beats.nature_answered());

        // 3. Donna steadies what Anya crosses.
        assert!(s.settle_crossing());
        assert!(s.crossing_passable());
        s.toggle_character();
        assert!(s.cross().is_ok());
        assert!(s.beats.crossed);

        // 5. The Rift interrupts the answer.
        s.awaken_rift();
        assert!(s.rift_active());
        assert_eq!(s.approach_birds(), Response::Disrupted);
        assert_eq!(s.birds, BirdState::Disrupted);
        assert!(s.beats.rift_disrupted);

        assert!(s.beats.all());
        assert_eq!(s.beats.count(), 5);
    }

    #[test]
    fn anya_cannot_settle_the_crossing() {
        let mut s = Session::new();
        assert_eq!(s.active_name(), "Anya");
        assert!(!s.settle_crossing());
        assert!(!s.crossing_passable());
        assert_eq!(s.last, Some(Event::CannotSettle));
    }

    #[test]
    fn cannot_cross_an_unsteadied_crossing() {
        let mut s = Session::new();
        assert!(s.cross().is_err());
        assert!(!s.beats.crossed);
    }
}
