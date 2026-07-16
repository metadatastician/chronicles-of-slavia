// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>

//! # Intent — what does this mind think the player is doing?
//!
//! Design: `docs/design/20-cognitive-npcs-and-theory-of-mind.md` §A.
//!
//! ## Confidence is not a number
//!
//! There is no suspicion float here, no tuned heuristic, no `if distance < 5.0`.
//! **The number of solutions the solver returns *is* the confidence signal**, and
//! [`Read`] is just that count, named:
//!
//! | Answers | [`Read`] | What the guard does |
//! |---|---|---|
//! | many | [`Read::Ambiguous`] | Can't tell — falls back to a bottleneck covering all of them |
//! | one | [`Read::Unified`] | Certain — intercepts |
//! | zero | [`Read::Contradiction`] | *"Wait — I've been tricked!"* Alert, call reinforcements |
//!
//! The zero case is the good one. A contradiction is not an error: it is the
//! moment the NPC **discovers he was deceived**, produced by the player actually
//! deceiving him rather than by a scripted reveal. It is `failo` given a name.
//!
//! ## L3 reads this; it does not compute its own
//!
//! The manpu overlays (`21-manpu-legibility.md`) are a *pure read* of [`Read`]:
//! [`Read::Ambiguous`] is the squiggle/swirl (perplexed, searching),
//! [`Read::Contradiction`] the sweat-drop shock. That mapping lives in L3 — do
//! not import a symbol vocabulary into this crate. If a renderer ever needs a
//! state this enum cannot express, add it *here*, not there.

use super::belief::{holds, Baseline, Mind};
use super::kanren::{conj, membero, run, Term};

/// A mind's reading of the player's intent — the solution count, named.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Read {
    /// Zero solutions (`failo`). A prior assumption just broke.
    Contradiction,
    /// Exactly one solution. High confidence.
    Unified(String),
    /// Several solutions. Genuinely does not know which.
    ///
    /// Sorted and de-duplicated, so this is stable to compare and to display.
    Ambiguous(Vec<String>),
}

/// Turn raw solver answers into a [`Read`].
///
/// Non-atom answers are dropped: an unbound or structured answer is not a target
/// name, and silently keeping it would fake a confidence we do not have.
/// De-duplicates, because several believed facts may point at the same target and
/// that is one candidate, not two.
pub fn classify(answers: Vec<Term>) -> Read {
    let mut names: Vec<String> = answers
        .iter()
        .filter_map(|t| t.as_atom().map(str::to_string))
        .collect();
    names.sort();
    names.dedup();

    match names.len() {
        0 => Read::Contradiction,
        1 => Read::Unified(names.remove(0)),
        _ => Read::Ambiguous(names),
    }
}

/// Which of `candidates` does `mind` consider consistent with the player heading
/// `heading`?
///
/// Reasons **only** from what `mind` believes — specifically from facts of the
/// form `(lies-toward <target> <heading>)`. A guard who has never learned which
/// way the vault lies cannot deduce you are going there, however obvious it is
/// to the player, and that asymmetry is the feature.
///
/// The `lies-toward` predicate is supplied by the scene as content (ADR-0006);
/// this function does not invent geography.
pub fn read_intent(mind: &Mind, baseline: &Baseline, heading: &str, candidates: &[&str]) -> Read {
    let beliefs = mind.beliefs(baseline);
    let candidates: Vec<Term> = candidates.iter().map(|c| Term::atom(*c)).collect();
    let heading = heading.to_string();

    let answers = run(None, |q| {
        conj(
            // q is one of the things we might be heading for …
            membero(q.clone(), candidates.clone()),
            // … and this mind believes q lies that way.
            holds(
                &beliefs,
                Term::list([Term::atom("lies-toward"), q, Term::atom(heading.as_str())]),
            ),
        )
    });

    classify(answers)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::esm::belief::Fact;

    fn f(pred: &str, args: &[&str]) -> Fact {
        Fact::new(pred, args.iter().copied())
    }

    /// Two things lie north; the guard knows both. He cannot tell which.
    #[test]
    fn two_plausible_targets_read_as_ambiguous() {
        let base = Baseline::new([
            f("lies-toward", &["usb-stick", "north"]),
            f("lies-toward", &["secret-note", "north"]),
        ]);
        let guard = Mind::new("guard");

        let read = read_intent(&guard, &base, "north", &["usb-stick", "secret-note"]);
        assert_eq!(
            read,
            Read::Ambiguous(vec!["secret-note".into(), "usb-stick".into()])
        );
    }

    /// Only one candidate lies that way: certainty, and an interception.
    #[test]
    fn a_single_plausible_target_reads_as_unified() {
        let base = Baseline::new([
            f("lies-toward", &["usb-stick", "north"]),
            f("lies-toward", &["secret-note", "south"]),
        ]);
        let guard = Mind::new("guard");

        let read = read_intent(&guard, &base, "north", &["usb-stick", "secret-note"]);
        assert_eq!(read, Read::Unified("usb-stick".into()));
    }

    /// Nothing he believes lies that way. The assumption broke: he's been tricked.
    #[test]
    fn no_plausible_target_reads_as_contradiction() {
        let base = Baseline::new([f("lies-toward", &["usb-stick", "north"])]);
        let guard = Mind::new("guard");

        let read = read_intent(&guard, &base, "west", &["usb-stick"]);
        assert_eq!(read, Read::Contradiction);
    }

    /// Ignorance is local. Same world, same heading, two different readings —
    /// because one guard was told and the other never was.
    #[test]
    fn what_a_guard_has_not_learned_he_cannot_deduce() {
        let base = Baseline::new([]);
        let told = {
            let mut m = Mind::new("told");
            m.learn(f("lies-toward", &["usb-stick", "north"]));
            m
        };
        let ignorant = Mind::new("ignorant");

        assert_eq!(
            read_intent(&told, &base, "north", &["usb-stick"]),
            Read::Unified("usb-stick".into())
        );
        assert_eq!(
            read_intent(&ignorant, &base, "north", &["usb-stick"]),
            Read::Contradiction
        );
    }

    /// Learning a second possibility *reduces* certainty. Knowing more can make
    /// an NPC less decisive, which is exactly right.
    #[test]
    fn learning_more_can_widen_a_read_from_unified_to_ambiguous() {
        let base = Baseline::new([f("lies-toward", &["usb-stick", "north"])]);
        let mut guard = Mind::new("guard");
        assert_eq!(
            read_intent(&guard, &base, "north", &["usb-stick", "secret-note"]),
            Read::Unified("usb-stick".into())
        );

        guard.learn(f("lies-toward", &["secret-note", "north"]));
        assert!(matches!(
            read_intent(&guard, &base, "north", &["usb-stick", "secret-note"]),
            Read::Ambiguous(_)
        ));
    }

    #[test]
    fn duplicate_routes_to_one_target_do_not_fake_ambiguity() {
        // Believed twice via baseline and delta; still one candidate.
        let base = Baseline::new([f("lies-toward", &["usb-stick", "north"])]);
        let mut guard = Mind::new("guard");
        guard.learn(f("lies-toward", &["usb-stick", "north"]));

        assert_eq!(
            read_intent(&guard, &base, "north", &["usb-stick"]),
            Read::Unified("usb-stick".into())
        );
    }
}
