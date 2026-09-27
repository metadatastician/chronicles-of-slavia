//! Renderer-neutral room topology. Distances are local beat units, never pixels.
//!
//! Rooms group the existing narrative beats; exits are explicit, directed links.
//! Walking to an edge does not teleport, rewind, or move the other heroine.

use crate::spec::Spec;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Deserialize)]
pub struct Room {
    pub id: String,
    pub beats: Vec<String>,
    #[serde(default)]
    pub exits: Vec<Exit>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Exit {
    pub id: String,
    /// Local beat index where this exit can be used.
    pub at: usize,
    pub to: String,
    /// Local beat index in the destination room.
    pub arrival: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub room: String,
    pub beat: f32,
}

/// Complete legacy flat SGS files as one room, then validate all references.
/// No rooms are inferred for an empty spec (which is still useful to L2 tests).
pub(crate) fn prepare(spec: &mut Spec) -> Result<(), String> {
    if spec.rooms.is_empty() && !spec.beats.is_empty() {
        spec.rooms.push(Room {
            id: "main".into(),
            beats: spec.beats.iter().map(|b| b.id.clone()).collect(),
            exits: vec![],
        });
    }
    let mut beat_ids = BTreeSet::new();
    for beat in &spec.beats {
        if beat.id.is_empty() || !beat_ids.insert(beat.id.as_str()) {
            return Err(format!("empty or duplicate beat id: {}", beat.id));
        }
    }
    let mut room_ids = BTreeSet::new();
    let mut assigned = BTreeSet::new();
    for room in &spec.rooms {
        if room.id.is_empty() || !room_ids.insert(room.id.as_str()) {
            return Err(format!("empty or duplicate room id: {}", room.id));
        }
        if room.beats.is_empty() {
            return Err(format!("room {} has no beats", room.id));
        }
        for beat in &room.beats {
            if !beat_ids.contains(beat.as_str()) || !assigned.insert(beat.as_str()) {
                return Err(format!("unknown or multiply assigned beat: {beat}"));
            }
        }
        let mut exits = BTreeSet::new();
        for exit in &room.exits {
            if exit.id.is_empty() || !exits.insert(exit.id.as_str()) {
                return Err(format!("empty or duplicate exit id in {}", room.id));
            }
            let destination = spec.rooms.iter().find(|r| r.id == exit.to);
            if exit.at >= room.beats.len()
                || destination.is_none_or(|r| exit.arrival >= r.beats.len())
            {
                return Err(format!("invalid exit {} in room {}", exit.id, room.id));
            }
        }
    }
    if assigned.len() != beat_ids.len() {
        return Err("every beat must belong to exactly one room".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{zone_a, Spec};

    fn two_rooms() -> String {
        r#"
[meta]
level = "test"
zone = "test"
name = "Synthetic topology, not Zone A content"
[[beats]]
id = "one"
title = "One"
[[beats]]
id = "two"
title = "Two"
[[rooms]]
id = "left"
beats = ["one"]
[[rooms.exits]]
id = "door"
at = 0
to = "right"
arrival = 0
[[rooms]]
id = "right"
beats = ["two"]
"#.into()
    }

    #[test]
    fn zone_a_keeps_its_seven_beats_in_one_authored_room() {
        let spec = zone_a();
        assert_eq!(spec.rooms.len(), 1);
        assert_eq!(spec.rooms[0].id, "border-path");
        assert_eq!(spec.rooms[0].beats.len(), 7);
        assert!(spec.rooms[0].exits.is_empty());
    }

    #[test]
    fn legacy_flat_spec_becomes_one_room() {
        let source = two_rooms();
        let spec = Spec::from_toml(source.split("[[rooms]]").next().unwrap()).unwrap();
        assert_eq!(spec.rooms[0].id, "main");
        assert_eq!(spec.rooms[0].beats, ["one", "two"]);
    }

    #[test]
    fn directed_connections_are_data() {
        let spec = Spec::from_toml(&two_rooms()).unwrap();
        assert_eq!(spec.rooms[0].exits[0].to, "right");
        assert!(spec.rooms[1].exits.is_empty());
    }

    #[test]
    fn rejects_invalid_topology() {
        for source in [
            two_rooms().replace("to = \"right\"", "to = \"missing\""),
            two_rooms().replace("arrival = 0", "arrival = 9"),
            two_rooms().replace("at = 0", "at = 9"),
            two_rooms().replace("beats = [\"two\"]", "beats = []"),
            two_rooms().replace("beats = [\"two\"]", "beats = [\"one\"]"),
            two_rooms().replace("beats = [\"two\"]", "beats = [\"missing\"]"),
            two_rooms().replace("id = \"right\"", "id = \"left\""),
            two_rooms().replace("id = \"two\"", "id = \"one\""),
        ] {
            assert!(Spec::from_toml(&source).is_err(), "accepted: {source}");
        }
    }
}
