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

use crate::save::{SaveData, SavedPosition};
use slavia_core::room::{Position, Room};
use serde::{Deserialize, Serialize};
use slavia_core::{zone_a, Beat, Character, CrossError, Response, Spec, World};
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

/// Why a room transition was refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TravelError {
    UnknownExit,
    NotHere,
}

/// A live Zone A play session.
pub struct Session {
    world: World,
    /// Grove-bird mood (view state).
    pub birds: BirdState,
    /// Which of the five success-condition beats have happened.
    pub beats: Beats,
    /// Each girl's room and local position in beat units.
    pos: HashMap<String, Position>,
    /// Which spec beats have been reached, by index (for narration / reveals).
    pub revealed: Vec<bool>,
}

impl Session {
    /// Start a fresh Zone A session — both girls at the forest entrance, Anya active.
    pub fn new() -> Self {
        Self::from_spec(zone_a())
    }

    fn from_spec(spec: Spec) -> Self {
        let start_room = spec.rooms.first().expect("a session needs a room").id.clone();
        let world = World::new(spec);
        let pos = world
            .spec()
            .characters
            .iter()
            .map(|c| (c.id.clone(), Position { room: start_room.clone(), beat: 0.0 }))
            .collect();
        let mut revealed = vec![false; world.spec().beats.len()];
        let first = world.spec().beats.iter()
            .position(|b| b.id == world.spec().rooms[0].beats[0])
            .expect("validated room beat");
        revealed[first] = true;
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
            pos: self.pos.iter().map(|(id, p)|
                (id.clone(), SavedPosition::Room(p.clone()))).collect(),
            revealed: self.revealed.clone(),
            summary: format!("{}, at {}", self.active_name(), self.current_beat().title),
        }
    }

    /// Restore a validated save. Disk reads call `try_restore` before offering
    /// Continue; this convenience is also used by the headless contract tests.
    pub fn restore(data: &SaveData) -> Session {
        Self::try_restore(data).expect("save must be validated before continuing")
    }

    pub fn try_restore(data: &SaveData) -> Result<Session, String> {
        Self::restore_in(data, zone_a())
    }

    fn restore_in(data: &SaveData, spec: Spec) -> Result<Session, String> {
        let mut session = Self::from_spec(spec);
        if !session.switch(&data.active_id)
            || data.pos.len() != session.pos.len()
            || data.revealed.len() != session.revealed.len()
        {
            return Err("save cast or beat layout does not match this SGS".into());
        }
        for (id, saved) in &data.pos {
            let position = match saved {
                SavedPosition::Room(p) => p.clone(),
                // Pre-room saves used one float per girl along A1's path.
                SavedPosition::Legacy(beat) => Position {
                    room: session.world.spec().rooms[0].id.clone(),
                    beat: *beat,
                },
            };
            let room = session.world.spec().rooms.iter().find(|r| r.id == position.room)
                .ok_or_else(|| format!("unknown saved room: {}", position.room))?;
            if !position.beat.is_finite() || position.beat < 0.0
                || position.beat > (room.beats.len() - 1) as f32
            {
                return Err("saved position is outside its room".into());
            }
            let dest = session.pos.get_mut(id)
                .ok_or_else(|| format!("unknown saved character: {id}"))?;
            *dest = position;
        }
        session.world.rift_active = data.rift_active;
        session.world.bridge_stable = data.bridge_stable;
        session.world.crossed = data.crossed;
        session.birds = data.birds;
        session.beats = data.beats;
        session.revealed = data.revealed.clone();
        Ok(session)
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

    /// Zone A's cast, as declared in the spec — for a renderer to spawn.
    pub fn characters(&self) -> &[Character] {
        &self.world.spec().characters
    }

    pub fn active_room(&self) -> &Room {
        let position = &self.pos[self.active_id()];
        self.world.spec().rooms.iter().find(|r| r.id == position.room)
            .expect("session positions reference validated rooms")
    }

    pub fn position_of(&self, id: &str) -> Option<&Position> {
        self.pos.get(id)
    }

    /// Local position, never distance along a concatenation of rooms.
    pub fn active_pos(&self) -> f32 {
        self.pos_of(self.active_id())
    }

    pub fn pos_of(&self, id: &str) -> f32 {
        self.pos.get(id).map_or(0.0, |p| p.beat)
    }

    fn local_beat_index(&self) -> usize {
        (self.active_pos().round() as usize).min(self.active_room().beats.len() - 1)
    }

    /// Global narrative index for reveal/progress storage. Movement stays local.
    pub fn nearest_beat_index(&self) -> usize {
        let id = &self.active_room().beats[self.local_beat_index()];
        self.world.spec().beats.iter().position(|b| &b.id == id)
            .expect("validated room beat")
    }

    pub fn current_beat(&self) -> &Beat {
        &self.world.spec().beats[self.nearest_beat_index()]
    }

    pub fn move_active(&mut self, dx: f32) -> Option<usize> {
        self.set_active_pos(self.active_pos() + dx)
    }

    /// Clamp within this room. Crossing a boundary requires an explicit exit.
    /// Non-finite renderer input is rejected without changing session state.
    pub fn set_active_pos(&mut self, beat_units: f32) -> Option<usize> {
        if !beat_units.is_finite() {
            return None;
        }
        let max = (self.active_room().beats.len() - 1) as f32;
        let id = self.active_id().to_string();
        self.pos.get_mut(&id).expect("active character has a position").beat =
            beat_units.clamp(0.0, max);
        self.reveal_current()
    }

    fn reveal_current(&mut self) -> Option<usize> {
        let index = self.nearest_beat_index();
        let first_visit = !self.revealed[index];
        self.revealed[index] = true;
        first_visit.then_some(index)
    }

    /// Traverse a declared directed exit at the current beat. The other girl,
    /// birds, cooperation state and Rift consequences are all left untouched.
    pub fn take_exit(&mut self, id: &str) -> Result<(), TravelError> {
        let exit = self.active_room().exits.iter().find(|e| e.id == id)
            .ok_or(TravelError::UnknownExit)?;
        if self.local_beat_index() != exit.at {
            return Err(TravelError::NotHere);
        }
        let position = Position { room: exit.to.clone(), beat: exit.arrival as f32 };
        let active = self.active_id().to_string();
        self.pos.insert(active, position);
        self.reveal_current();
        Ok(())
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

#[cfg(test)]
mod room_tests {
    use super::*;
    use slavia_core::room::Exit;

    // Synthetic topology over the unchanged A1 vocabulary. This is a test
    // fixture, not authored A2 content or a ruling about future zone layout.
    fn split_spec() -> Spec {
        let mut spec = zone_a();
        spec.rooms = vec![
            Room {
                id: "woods".into(),
                beats: spec.beats[..3].iter().map(|b| b.id.clone()).collect(),
                exits: vec![Exit {
                    id: "crossing".into(), at: 2, to: "river".into(), arrival: 0,
                }],
            },
            Room {
                id: "river".into(),
                beats: spec.beats[3..].iter().map(|b| b.id.clone()).collect(),
                exits: vec![Exit {
                    id: "return".into(), at: 0, to: "woods".into(), arrival: 2,
                }],
            },
        ];
        spec
    }

    #[test]
    fn traversal_is_explicit_local_and_does_not_move_the_other_girl() {
        let mut s = Session::from_spec(split_spec());
        assert_eq!(s.take_exit("crossing"), Err(TravelError::NotHere));
        assert_eq!(s.take_exit("missing"), Err(TravelError::UnknownExit));
        s.move_active(999.0);
        assert_eq!(s.active_room().id, "woods");
        assert_eq!(s.active_pos(), 2.0);
        s.take_exit("crossing").unwrap();
        assert_eq!(s.active_room().id, "river");
        assert_eq!(s.active_pos(), 0.0);
        assert_eq!(s.current_beat().id, "stream-bridge");
        assert!(s.revealed[3]);
        assert_eq!(s.approach_birds(), None);
        s.move_active(1.0); // same local index as the grove, different room
        assert_eq!(s.approach_birds(), None);
        s.toggle_character();
        assert_eq!(s.active_room().id, "woods");
        assert_eq!(s.active_pos(), 0.0);
        s.move_active(1.0);
        assert_eq!(s.approach_birds(), Some(Response::Settled));
    }

    #[test]
    fn round_trip_between_rooms_preserves_consequences_and_both_positions() {
        let mut s = Session::from_spec(split_spec());
        s.set_active_pos(2.0);
        s.take_exit("crossing").unwrap();
        s.awaken_rift();
        s.take_exit("return").unwrap();
        assert!(s.rift_active());
        assert_eq!(s.active_pos(), 2.0);
        s.take_exit("crossing").unwrap();
        s.move_active(1.25);
        let text = toml::to_string(&s.to_save_data()).unwrap();
        let data: SaveData = toml::from_str(&text).unwrap();
        let restored = Session::restore_in(&data, split_spec()).unwrap();
        for id in ["anya", "donna"] {
            assert_eq!(restored.position_of(id), s.position_of(id));
        }
        assert_eq!(restored.revealed, s.revealed);
        assert!(restored.rift_active());
    }

    #[test]
    fn legacy_scalar_saves_migrate_to_a1() {
        let mut data = Session::new().to_save_data();
        data.pos.insert("anya".into(), SavedPosition::Legacy(1.5));
        data.pos.insert("donna".into(), SavedPosition::Legacy(3.0));
        let text = toml::to_string(&data).unwrap();
        let data: SaveData = toml::from_str(&text).unwrap();
        let s = Session::try_restore(&data).unwrap();
        assert_eq!(s.active_room().id, "border-path");
        assert_eq!(s.pos_of("anya"), 1.5);
        assert_eq!(s.pos_of("donna"), 3.0);
    }

    #[test]
    fn invalid_saves_are_rejected_instead_of_panicking_during_play() {
        for beat in [f32::NAN, f32::INFINITY, -1.0, 7.0] {
            let mut data = Session::new().to_save_data();
            data.pos.insert("anya".into(), SavedPosition::Legacy(beat));
            assert!(Session::try_restore(&data).is_err());
        }
        let mut data = Session::new().to_save_data();
        data.pos.insert("anya".into(), SavedPosition::Room(Position {
            room: "missing".into(), beat: 0.0,
        }));
        assert!(Session::try_restore(&data).is_err());
        let mut data = Session::new().to_save_data();
        data.revealed.clear();
        assert!(Session::try_restore(&data).is_err());
        let mut data = Session::new().to_save_data();
        data.pos.remove("donna");
        assert!(Session::try_restore(&data).is_err());
        let mut data = Session::new().to_save_data();
        data.active_id = "missing".into();
        assert!(Session::try_restore(&data).is_err());
    }

    #[test]
    fn reveals_are_first_visit_only_and_nonfinite_motion_is_ignored() {
        let mut s = Session::new();
        assert_eq!(s.move_active(1.0), Some(1));
        assert_eq!(s.move_active(-1.0), None);
        assert_eq!(s.move_active(1.0), None);
        s.move_active(f32::NAN);
        assert_eq!(s.active_pos(), 1.0);
    }
}
