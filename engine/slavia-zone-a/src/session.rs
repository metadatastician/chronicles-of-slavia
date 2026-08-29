// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>

//! The play session — the *only* place the renderer touches the rules.
//!
//! `Session` owns a [`slavia_core::World`] and models Zone A as a **spatial walk
//! through its seven beats** (`docs/design/02-zone-a-design.md`): each girl has a
//! position along the path, interactions are gated on being at the right beat,
//! and beats reveal as they are reached. It holds **no rendering** and no Bevy
//! types — the whole traversal is testable headlessly, and it survives any
//! future engine choice (M2) because it is engine-agnostic.

use crate::save::SaveData;
use serde::{Deserialize, Serialize};
use slavia_core::{zone_a, Beat, Character, CrossError, Response, World};
use std::collections::HashMap;

/// Zone A's single animal, and the beats that gate interactions.
pub const GROVE_BIRDS: &str = "grove-birds";
const BIRD_GROVE: &str = "bird-grove";
const STREAM_BRIDGE: &str = "stream-bridge";

/// The visible mood of the grove birds, from the last time a girl reached them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BirdState {
    Neutral,
    Stirred,
    Settled,
    Disrupted,
}

impl BirdState {
    fn from_response(r: &Response) -> BirdState {
        match r {
            Response::Stirred => BirdState::Stirred,
            Response::Settled => BirdState::Settled,
            Response::Disrupted => BirdState::Disrupted,
            _ => BirdState::Neutral,
        }
    }
}

/// The five things a player should come to understand in Zone A
/// (`docs/design/00-start-here.md`), tracked as they actually happen.
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct Beats {
    pub anya_stirred: bool,
    pub donna_settled: bool,
    pub crossed: bool,
    pub rift_disrupted: bool,
}

impl Beats {
    pub fn nature_answered(&self) -> bool {
        self.anya_stirred && self.donna_settled
    }

    pub fn all(&self) -> bool {
        self.anya_stirred && self.donna_settled && self.crossed && self.rift_disrupted
    }

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

/// Outcome of trying to settle the crossing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Settle {
    /// Donna lowered the crossing's unsteady motion — it is now passable.
    Settled,
    /// The active girl cannot settle (she raises taxis, she does not lower).
    WrongGift,
    /// Not standing at the crossing.
    NotHere,
}

/// Outcome of trying to cross.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Crossing {
    Crossed,
    /// The crossing has not been made passable yet.
    Unpassable,
    /// Not standing at the crossing.
    NotHere,
}

/// A live Zone A play session.
pub struct Session {
    world: World,
    /// Grove-bird mood (view state).
    pub birds: BirdState,
    /// Which of the five success-condition beats have happened.
    pub beats: Beats,
    /// Each girl's position along the path, in beat units (`0.0 ..= last beat`).
    pos: HashMap<String, f32>,
    /// Which spec beats have been reached, by index (for narration / reveals).
    pub revealed: Vec<bool>,
}

impl Session {
    /// Start a fresh Zone A session — both girls at the forest entrance, Anya active.
    pub fn new() -> Self {
        let world = World::new(zone_a());
        let pos = world
            .spec()
            .characters
            .iter()
            .map(|c| (c.id.clone(), 0.0))
            .collect();
        let mut revealed = vec![false; world.spec().beats.len()];
        if let Some(first) = revealed.first_mut() {
            *first = true;
        }
        Session {
            world,
            birds: BirdState::Neutral,
            beats: Beats::default(),
            pos,
            revealed,
        }
    }

    /// Snapshot everything needed to resume this session later.
    pub fn to_save_data(&self) -> SaveData {
        SaveData {
            active_id: self.active_id().to_string(),
            rift_active: self.world.rift_active,
            bridge_stable: self.world.bridge_stable,
            crossed: self.world.crossed,
            birds: self.birds,
            beats: self.beats,
            pos: self.pos.clone(),
            revealed: self.revealed.clone(),
            summary: format!("{}, at {}", self.active_name(), self.current_beat().title),
        }
    }

    /// Rebuild a session from a save — both girls' positions and the
    /// world's transition state restored exactly as they were.
    pub fn restore(data: &SaveData) -> Session {
        let mut world = World::new(zone_a());
        world.switch_to(&data.active_id);
        world.rift_active = data.rift_active;
        world.bridge_stable = data.bridge_stable;
        world.crossed = data.crossed;
        Session {
            world,
            birds: data.birds,
            beats: data.beats,
            pos: data.pos.clone(),
            revealed: data.revealed.clone(),
        }
    }

    pub fn active_id(&self) -> &str {
        self.world.active().id.as_str()
    }

    pub fn active_name(&self) -> &str {
        self.world.active().name.as_str()
    }

    pub fn switch(&mut self, id: &str) -> bool {
        self.world.switch_to(id)
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

    // --- space -------------------------------------------------------------

    pub fn beats_slice(&self) -> &[Beat] {
        &self.world.spec().beats
    }

    /// Zone A's cast, as declared in the spec — for a renderer to spawn.
    pub fn characters(&self) -> &[Character] {
        &self.world.spec().characters
    }

    fn last_index(&self) -> usize {
        self.beats_slice().len().saturating_sub(1)
    }

    /// The active girl's position along the path, in beat units.
    pub fn active_pos(&self) -> f32 {
        self.pos_of(self.active_id())
    }

    /// Any girl's position along the path.
    pub fn pos_of(&self, id: &str) -> f32 {
        *self.pos.get(id).unwrap_or(&0.0)
    }

    /// The beat the active girl is currently standing at (the nearest one).
    pub fn nearest_beat_index(&self) -> usize {
        (self.active_pos().round() as i32).clamp(0, self.last_index() as i32) as usize
    }

    pub fn current_beat(&self) -> &Beat {
        &self.beats_slice()[self.nearest_beat_index()]
    }

    /// Move the active girl by `dx` beat units. Returns the index of a *newly
    /// reached* beat, if this move stepped onto one.
    #[cfg_attr(not(test), allow(dead_code))]
    pub fn move_active(&mut self, dx: f32) -> Option<usize> {
        self.set_active_pos(self.active_pos() + dx)
    }

    /// Set the active girl's absolute position, in beat units, clamped to the
    /// path's extent. Returns the index of a *newly reached* beat, if this
    /// lands on one for the first time. For a renderer that simulates a
    /// continuous space (e.g. pixels) and derives beat position from it,
    /// rather than moving in beat units directly.
    pub fn set_active_pos(&mut self, beat_units: f32) -> Option<usize> {
        let id = self.active_id().to_string();
        let max = self.last_index() as f32;
        let before = self.nearest_beat_index();
        if let Some(p) = self.pos.get_mut(&id) {
            *p = beat_units.clamp(0.0, max);
        }
        let after = self.nearest_beat_index();
        if after != before {
            self.revealed[after] = true;
            Some(after)
        } else {
            None
        }
    }

    // --- interactions (gated on location) ----------------------------------

    /// Reach out to the grove birds — only possible while standing at the grove.
    /// `None` means there are no birds here.
    pub fn approach_birds(&mut self) -> Option<Response> {
        if self.current_beat().id != BIRD_GROVE {
            return None;
        }
        let r = self
            .world
            .approach(GROVE_BIRDS)
            .expect("Zone A always has the grove birds");
        self.birds = BirdState::from_response(&r);
        match &r {
            Response::Stirred => self.beats.anya_stirred = true,
            Response::Settled => self.beats.donna_settled = true,
            Response::Disrupted => self.beats.rift_disrupted = true,
            _ => {}
        }
        Some(r)
    }

    /// Settle the crossing — only at the bridge, and only a lowering gift (Donna).
    pub fn settle_crossing(&mut self) -> Settle {
        if self.current_beat().id != STREAM_BRIDGE {
            return Settle::NotHere;
        }
        if self.world.stabilize_bridge() {
            Settle::Settled
        } else {
            Settle::WrongGift
        }
    }

    /// Cross — only at the bridge, and only once it has been made passable.
    pub fn cross(&mut self) -> Crossing {
        if self.current_beat().id != STREAM_BRIDGE {
            return Crossing::NotHere;
        }
        match self.world.cross() {
            Ok(()) => {
                self.beats.crossed = true;
                Crossing::Crossed
            }
            Err(CrossError::BridgeUnstable) => Crossing::Unpassable,
        }
    }

    /// The Fracture: awaken the Rift.
    pub fn awaken_rift(&mut self) {
        self.world.awaken_rift();
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

    /// The whole Zone A arc as a spatial walk — driven through the exact intents
    /// the renderer calls. A headless regression test for the rules and
    /// traversal wiring; it is not a universal or formal proof.
    #[test]
    fn walks_zone_a_in_space() {
        let mut s = Session::new();
        assert_eq!(s.active_name(), "Anya");

        // Away from the grove, there are no birds to reach.
        assert_eq!(s.approach_birds(), None);

        // 1. Anya walks to the grove and stirs the birds.
        s.move_active(1.0);
        assert_eq!(s.current_beat().id, "bird-grove");
        assert_eq!(s.approach_birds(), Some(Response::Stirred));
        assert_eq!(s.birds, BirdState::Stirred);

        // 2. Donna walks to the grove and settles them.
        s.toggle_character();
        assert_eq!(s.active_name(), "Donna");
        s.move_active(1.0);
        assert_eq!(s.approach_birds(), Some(Response::Settled));
        assert!(s.beats.nature_answered()); // 4. Nature answered.

        // 3. Donna steadies the crossing; Anya crosses it.
        s.move_active(2.0); // Donna: grove(1) -> bridge(3)
        assert_eq!(s.current_beat().id, "stream-bridge");
        assert_eq!(s.settle_crossing(), Settle::Settled);
        s.toggle_character(); // Anya, still at the grove
        s.move_active(2.0); // Anya: grove(1) -> bridge(3)
        assert_eq!(s.cross(), Crossing::Crossed);
        assert!(s.beats.crossed);

        // 5. The Rift interrupts the answer.
        s.awaken_rift();
        s.move_active(-2.0); // Anya back to the grove
        assert_eq!(s.approach_birds(), Some(Response::Disrupted));
        assert_eq!(s.birds, BirdState::Disrupted);

        assert!(s.beats.all());
        assert_eq!(s.beats.count(), 5);
    }

    #[test]
    fn cannot_settle_away_from_the_bridge() {
        let mut s = Session::new();
        assert_eq!(s.settle_crossing(), Settle::NotHere);
    }

    #[test]
    fn anya_at_the_bridge_cannot_settle_it() {
        let mut s = Session::new();
        s.move_active(3.0); // Anya to the bridge
        assert_eq!(s.current_beat().id, "stream-bridge");
        assert_eq!(s.settle_crossing(), Settle::WrongGift);
    }

    #[test]
    fn cannot_cross_an_unsteadied_crossing() {
        let mut s = Session::new();
        s.move_active(3.0);
        assert_eq!(s.cross(), Crossing::Unpassable);
    }

    /// A save round-trips through an actual TOML string (not just the
    /// struct) and restores an in-progress walk exactly: both girls'
    /// positions, revealed beats, and world transition state.
    #[test]
    fn save_round_trips_through_toml() {
        let mut s = Session::new();
        s.move_active(1.0); // Anya to the grove
        assert_eq!(s.approach_birds(), Some(Response::Stirred));
        s.toggle_character();
        s.move_active(3.0); // Donna: entrance(0) -> bridge(3)
        assert_eq!(s.settle_crossing(), Settle::Settled);

        let data = s.to_save_data();
        let toml_text = toml::to_string(&data).expect("serializes");
        let restored: crate::save::SaveData = toml::from_str(&toml_text).expect("deserializes");
        let s2 = Session::restore(&restored);

        assert_eq!(s2.active_name(), "Donna");
        assert_eq!(s2.active_pos(), 3.0);
        assert_eq!(s2.pos_of("anya"), 1.0);
        assert!(s2.beats.anya_stirred);
        assert!(s2.crossing_passable());
        assert!(s2.revealed[1]); // the grove, reached along the way
        assert_eq!(s2.birds, BirdState::Stirred);
    }

    #[test]
    fn reaching_the_shrine_reveals_its_words() {
        let mut s = Session::new();
        assert!(!s.revealed[2]);
        s.move_active(2.0); // to the shrine
        assert_eq!(s.current_beat().id, "shrine");
        assert!(s.revealed[2]);
        assert_eq!(
            s.current_beat().text.as_deref(),
            Some("Two lands, one heart.")
        );
    }
}
