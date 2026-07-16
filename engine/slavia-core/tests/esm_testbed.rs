// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>

//! # The ESM testbed — a synthetic cast, no zone, no window
//!
//! Design: `docs/design/20-cognitive-npcs-and-theory-of-mind.md`.
//! Architecture: `docs/decisions/0004-hybrid-fsm-esm-mind-body-architecture.adoc`.
//!
//! Zone A's five beats are **deliberately untouched** — `tests/zone_a.rs` still
//! owns the sacred logic, and this file must never disturb it. The cast here
//! (Boris, Pyotr, a corridor, a cellar) is scaffolding for proving the mind
//! works, and is expected to be thrown away when minds reach a real scene.
//!
//! Everything below runs headless: `cargo test -p slavia-core`.
//!
//! ## What is being claimed
//!
//! 1. Belief is **local** — what one NPC learns, another does not.
//! 2. Confidence is the **solution count**, not a tuned number.
//! 3. Deception produces a **contradiction**, and it is generated, not scripted.
//! 4. The **Guilty Conscience trap emerges** from belief + intent rather than
//!    being coded. This is the design's central claim, and if it ever has to be
//!    special-cased into a function, the claim is false.

use slavia_core::esm::decay::retained_instructions;
use slavia_core::esm::intent::read_intent;
use slavia_core::esm::{
    belief::holds,
    kanren::{conj_all, eq, fresh, run, Term},
    Baseline, Fact, Mind, Read, Rng,
};

fn fact(pred: &str, args: &[&str]) -> Fact {
    Fact::new(pred, args.iter().copied())
}

fn atom(s: &str) -> Term {
    Term::atom(s)
}

/// The world as it actually is. Note what is *absent*: nothing here says the
/// manor hides a hostage. That is not public truth — it is Boris's private
/// burden, and it is added to *his* mind alone.
fn manor() -> Baseline {
    Baseline::new([
        fact("lies-toward", &["cellar", "north"]),
        fact("lies-toward", &["storeroom", "north"]),
        fact("lies-toward", &["chapel", "east"]),
        fact("secret-in", &["hostage", "cellar"]),
    ])
}

/// Boris knows what he is hiding. This is the *only* difference between him and
/// any other guard in the manor.
fn boris() -> Mind {
    let mut m = Mind::new("boris");
    m.learn(fact("is-hiding-secret", &["hostage"]));
    m
}

// ---------------------------------------------------------------------------
// 1. Belief is local
// ---------------------------------------------------------------------------

#[test]
fn two_guards_in_one_manor_can_disagree_about_the_world() {
    let base = manor();
    let boris = boris();
    let pyotr = Mind::new("pyotr");

    assert!(boris.believes(&base, &fact("is-hiding-secret", &["hostage"])));
    assert!(!pyotr.believes(&base, &fact("is-hiding-secret", &["hostage"])));

    // Both still agree about the parts of the world nobody hid from them.
    assert!(boris.believes(&base, &fact("lies-toward", &["cellar", "north"])));
    assert!(pyotr.believes(&base, &fact("lies-toward", &["cellar", "north"])));
}

#[test]
fn what_one_guard_witnesses_leaves_the_other_wrong_indefinitely() {
    let base = Baseline::new([fact("at", &["artifact", "vault"])]);
    let mut watchful = Mind::new("watchful");
    let oblivious = Mind::new("oblivious");

    // Only one of them looked in the vault.
    watchful.unlearn(fact("at", &["artifact", "vault"]));
    watchful.learn(fact("gone", &["artifact"]));

    assert!(watchful.believes(&base, &fact("gone", &["artifact"])));
    assert!(oblivious.believes(&base, &fact("at", &["artifact", "vault"])));
    assert!(!oblivious.believes(&base, &fact("gone", &["artifact"])));
}

// ---------------------------------------------------------------------------
// 2. Confidence is the solution count
// ---------------------------------------------------------------------------

#[test]
fn heading_north_is_ambiguous_because_two_things_lie_north() {
    let read = read_intent(
        &boris(),
        &manor(),
        "north",
        &["cellar", "storeroom", "chapel"],
    );
    // He covers the bottleneck; he cannot know which.
    assert_eq!(
        read,
        Read::Ambiguous(vec!["cellar".into(), "storeroom".into()])
    );
}

#[test]
fn heading_east_is_unified_because_only_the_chapel_lies_east() {
    let read = read_intent(
        &boris(),
        &manor(),
        "east",
        &["cellar", "storeroom", "chapel"],
    );
    assert_eq!(read, Read::Unified("chapel".into()));
}

#[test]
fn a_guard_who_never_learned_the_geography_can_deduce_nothing() {
    // Same manor, same heading — but this mind starts from an empty world.
    let read = read_intent(
        &Mind::new("newcomer"),
        &Baseline::new([]),
        "north",
        &["cellar"],
    );
    assert_eq!(read, Read::Contradiction);
}

// ---------------------------------------------------------------------------
// 3. Deception produces a contradiction — generated, not scripted
// ---------------------------------------------------------------------------

#[test]
fn being_tricked_is_a_contradiction_the_player_caused() {
    let base = manor();
    let mut guard = boris();

    // He is told — or infers — that the player is bound for the cellar.
    // (A false trail the player laid: the only northward thing he now credits.)
    guard.unlearn(fact("lies-toward", &["storeroom", "north"]));
    assert_eq!(
        read_intent(&guard, &base, "north", &["cellar", "storeroom"]),
        Read::Unified("cellar".into()),
        "he should be confident, and wrong"
    );

    // Then he witnesses the player heading west instead. Nothing he believes
    // lies west: every assumption he had just broke.
    let read = read_intent(&guard, &base, "west", &["cellar", "storeroom"]);
    assert_eq!(read, Read::Contradiction, "\"Wait — I've been tricked!\"");
}

// ---------------------------------------------------------------------------
// 4. The Guilty Conscience trap — EMERGENT, not coded
// ---------------------------------------------------------------------------

/// The guard's reading of a passer-by, derived purely from what he believes.
///
/// Note carefully: **the player's actual intent is never an input.** This
/// function does not know, and cannot ask, why the player is in the corridor.
/// Everything it returns is a product of the guard's own mind.
///
/// This is written in the test, not in `esm/`, on purpose: if the trap needed a
/// dedicated engine feature, it would not be emergent, and the claim in
/// `20-cognitive-npcs-and-theory-of-mind.md` §D would be false.
fn reads_passerby_as(guard: &Mind, base: &Baseline, corridor: &str) -> Vec<String> {
    let beliefs = guard.beliefs(base);
    let corridor = corridor.to_string();

    let answers = run(None, |q| {
        let beliefs = beliefs.clone();
        let corridor = corridor.clone();
        fresh(move |secret| {
            conj_all(vec![
                // "I am hiding something…"
                holds(
                    &beliefs,
                    Term::list([atom("is-hiding-secret"), secret.clone()]),
                ),
                // "…and it is down there…"
                holds(
                    &beliefs,
                    Term::list([atom("secret-in"), secret.clone(), atom(&corridor)]),
                ),
                // "…and someone is walking down there."
                holds(
                    &beliefs,
                    Term::list([atom("at"), atom("player"), atom(&corridor)]),
                ),
                // Therefore: they must be here to expose it.
                eq(q.clone(), atom("expose-secret")),
            ])
        })
    });

    answers
        .iter()
        .filter_map(|t| t.as_atom().map(str::to_string))
        .collect()
}

#[test]
fn a_guilty_guard_reads_an_innocent_passerby_as_a_threat() {
    let base = manor();
    let mut guard = boris();

    // The player wanders into the cellar corridor. Entirely by accident.
    // Nothing anywhere states the player's intent — because nobody knows it.
    guard.learn(fact("at", &["player", "cellar"]));

    let read = reads_passerby_as(&guard, &base, "cellar");
    assert_eq!(
        read,
        vec!["expose-secret".to_string()],
        "his own guilt convicted an innocent passer-by"
    );
}

#[test]
fn an_innocent_guard_in_the_same_corridor_reads_nothing_at_all() {
    let base = manor();
    let mut pyotr = Mind::new("pyotr");

    // Identical situation. Identical player. Identical corridor.
    pyotr.learn(fact("at", &["player", "cellar"]));

    let read = reads_passerby_as(&pyotr, &base, "cellar");
    assert!(
        read.is_empty(),
        "Pyotr hides nothing, so there is nothing to betray"
    );
}

#[test]
fn the_guards_panic_is_the_only_clue_and_nobody_authored_it() {
    let base = manor();
    let mut guard = boris();
    guard.learn(fact("at", &["player", "cellar"]));

    // The player learns the cellar matters *because Boris overreacted*, and
    // Boris overreacted because of a fact about Boris. No clue was placed.
    let boris_reacts = !reads_passerby_as(&guard, &base, "cellar").is_empty();

    let mut elsewhere = boris();
    elsewhere.learn(fact("at", &["player", "chapel"]));
    let boris_ignores_the_chapel = reads_passerby_as(&elsewhere, &base, "chapel").is_empty();

    assert!(boris_reacts && boris_ignores_the_chapel);
}

// ---------------------------------------------------------------------------
// 5. Decay, deterministically
// ---------------------------------------------------------------------------

#[test]
fn a_companion_sent_far_forgets_the_tail_but_not_the_first_errand() {
    let errands: Vec<String> = [
        "fetch water",
        "mind the goat",
        "bar the gate",
        "wait for me",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    let mut rng = Rng::seeded(20260716);
    let kept = retained_instructions(40, &errands, &mut rng);

    assert_eq!(kept.first().map(String::as_str), Some("fetch water"));
    assert!(
        kept.len() < errands.len(),
        "a long walk must cost something"
    );

    // And the same errand, the same walk, the same seed — the same memory.
    let mut rng = Rng::seeded(20260716);
    assert_eq!(retained_instructions(40, &errands, &mut rng), kept);
}
