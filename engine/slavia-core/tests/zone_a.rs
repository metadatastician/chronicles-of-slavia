// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>

//! Zone A's prototype success condition, encoded as executable assertions.
//!
//! From `docs/design/00-start-here.md`, a player should understand:
//!   1. Anya stirs.
//!   2. Donna settles.
//!   3. Donna steadies what Anya crosses.
//!   4. Nature answers them.
//!   5. The Rift interrupts the answer.
//!
//! If these tests pass, the *sacred logic* underneath those five lines holds —
//! whatever renderer we later put on top.

use slavia_core::{zone_a, Animal, CrossError, Influence, Nature, Response, World};

fn world() -> World {
    World::new(zone_a())
}

#[test]
fn spec_loads_with_two_girls_and_the_grove_birds() {
    let s = zone_a();
    assert_eq!(s.meta.zone, "A");
    assert_eq!(s.characters.len(), 2);
    assert_eq!(s.character("anya").unwrap().influence, Influence::Stir);
    assert_eq!(s.character("donna").unwrap().influence, Influence::Settle);
    assert_eq!(s.animal("grove-birds").unwrap().nature, Nature::Natural);
    assert_eq!(s.beats.len(), 7);
    assert_eq!(
        s.beat("shrine").unwrap().text.as_deref(),
        Some("Two lands, one heart.")
    );
}

/// Beats: Bird grove. Success lines 1, 2 and 4 — Anya stirs, Donna settles,
/// nature answers.
#[test]
fn anya_stirs_and_donna_settles_the_grove_birds() {
    let mut w = world();
    assert_eq!(w.active().id, "anya", "the first character starts active");
    assert_eq!(w.approach("grove-birds"), Some(Response::Stirred));

    assert!(w.switch_to("donna"));
    assert_eq!(w.approach("grove-birds"), Some(Response::Settled));
}

/// Beat: Stream bridge. Success line 3 — Donna steadies what Anya crosses.
#[test]
fn donna_steadies_the_bridge_before_anya_can_cross() {
    let mut w = world();

    // As Anya, an unsteadied bridge cannot be crossed...
    assert_eq!(w.cross(), Err(CrossError::BridgeUnstable));
    // ...and Anya cannot steady it herself (she stirs, she does not settle).
    assert!(!w.stabilize_bridge());
    assert!(!w.bridge_stable);

    // Donna steadies it.
    assert!(w.switch_to("donna"));
    assert!(w.stabilize_bridge());
    assert!(w.bridge_stable);
    // But Donna is not the one who crosses.
    assert_eq!(w.cross(), Err(CrossError::NotTheCrosser));

    // Anya crosses what Donna steadied.
    assert!(w.switch_to("anya"));
    assert_eq!(w.cross(), Ok(()));
    assert!(w.crossed);
}

/// Beat: Fracture. Success line 5 — the Rift interrupts the answer.
#[test]
fn the_rift_interrupts_the_answer() {
    let mut w = world();

    // Before the Fracture, the birds answer naturally.
    assert_eq!(w.approach("grove-birds"), Some(Response::Stirred));

    w.awaken_rift();

    // After: they stop answering normally, whoever reaches out.
    assert_eq!(w.approach("grove-birds"), Some(Response::Disrupted));
    assert!(w.switch_to("donna"));
    assert_eq!(w.approach("grove-birds"), Some(Response::Disrupted));
}

/// The diagnostic rule (not exercised by Zone A's shipped data, but the grammar
/// must already hold for later zones): a non-natural animal betrays itself by
/// failing to answer naturally.
#[test]
fn a_corrupted_animal_does_not_answer_naturally() {
    let corrupted = Animal {
        id: "false-wolf".into(),
        species: "wolf".into(),
        nature: Nature::Corrupted,
        location: None,
    };
    assert_eq!(
        corrupted.respond(Influence::Settle, false),
        Response::Unnatural(Nature::Corrupted),
    );
    assert_eq!(
        corrupted.respond(Influence::Stir, false),
        Response::Unnatural(Nature::Corrupted),
    );
}

/// The whole Zone A arc as one executable walk of the success condition.
#[test]
fn zone_a_success_condition_end_to_end() {
    let mut w = world();

    // 1. Anya stirs.
    assert_eq!(w.approach("grove-birds"), Some(Response::Stirred));
    // 2. Donna settles.
    w.switch_to("donna");
    assert_eq!(w.approach("grove-birds"), Some(Response::Settled));
    // 3. Donna steadies what Anya crosses.
    assert!(w.stabilize_bridge());
    w.switch_to("anya");
    assert_eq!(w.cross(), Ok(()));
    // 4. Nature answered them — established by steps 1 and 2.
    // 5. The Rift interrupts the answer.
    w.awaken_rift();
    assert_eq!(w.approach("grove-birds"), Some(Response::Disrupted));
}
