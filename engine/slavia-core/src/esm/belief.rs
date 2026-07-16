// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>

//! # Belief — Global Baseline plus per-entity Deltas
//!
//! The data half of the ESM. [`kanren`](super::kanren) is how a mind *thinks*;
//! this is what it thinks *about*.
//!
//! ## The model
//!
//! One [`Baseline`] of facts that are simply true of the world, plus, per entity,
//! a list of [`Delta`]s — the deviations *that entity actually acquired*, by
//! seeing, hearing, or being told.
//!
//! ```text
//! belief(entity) = baseline + entity's own deltas
//! ```
//!
//! And nothing else. Two guards in the same room may hold contradictory beliefs;
//! that is the design working, not a bug to reconcile.
//!
//! ## Asymmetric memory is free
//!
//! `20-cognitive-npcs-and-theory-of-mind.md` §E describes the rotating sentry:
//! catch the player in Chronicle I and *only that guard* carries the memory; move
//! him to a new outpost and he is the sole active threat among neutral
//! colleagues.
//!
//! Note there is nothing to build for that — **asymmetric memory simply *is*
//! per-entity deltas.** A [`Mind`] is already private. What is parked is
//! *factions* and *rotation* (which need a second location, and which
//! `00-start-here.md` parks explicitly), not the epistemics underneath.
//!
//! ## Content vs. mechanism (ADR-0006 §3)
//!
//! [`Baseline`] contents and a [`Mind`]'s starting deltas are **content** — they
//! come from SGS data, because they are just facts about a scene. Nothing in this
//! file should ever enumerate a specific fact, entity, or scene. If you find
//! yourself writing `"hostage"` in here, it belongs in TOML.

use std::collections::BTreeSet;

use super::kanren::{disj_any, eq, Goal, Term};

/// A single proposition: a predicate applied to ground arguments.
///
/// `Fact::new("is-hiding-secret", ["hostage"])` is `(is-hiding-secret hostage)`.
///
/// Deliberately typed rather than a bare [`Term`]: it is far easier to read, and
/// facts from a scene are always ground. [`Fact::to_term`] crosses into the
/// solver when reasoning is needed.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Fact {
    pub predicate: String,
    pub args: Vec<String>,
}

impl Fact {
    pub fn new(
        predicate: impl Into<String>,
        args: impl IntoIterator<Item = impl Into<String>>,
    ) -> Fact {
        Fact {
            predicate: predicate.into(),
            args: args.into_iter().map(Into::into).collect(),
        }
    }

    /// As a solver term: a proper list `(predicate arg…)`.
    pub fn to_term(&self) -> Term {
        let mut items = vec![Term::atom(self.predicate.as_str())];
        items.extend(self.args.iter().map(|a| Term::atom(a.as_str())));
        Term::list(items)
    }
}

/// The facts that are true of the world, before anyone's private experience.
///
/// The baseline is *truth*, not *common knowledge* — an entity does not believe
/// a baseline fact because it is true, but because nothing in its own history
/// contradicts it. Withhold facts from the baseline if no one should start out
/// knowing them.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Baseline {
    facts: BTreeSet<Fact>,
}

impl Baseline {
    pub fn new(facts: impl IntoIterator<Item = Fact>) -> Baseline {
        Baseline {
            facts: facts.into_iter().collect(),
        }
    }

    pub fn contains(&self, f: &Fact) -> bool {
        self.facts.contains(f)
    }

    pub fn facts(&self) -> &BTreeSet<Fact> {
        &self.facts
    }
}

/// One deviation from the baseline, acquired by an individual.
///
/// [`Delta::Unlearned`] is not "forgot" — it is *came to believe otherwise*: he
/// looked in the vault and the artifact was **gone**. For forgetting, see
/// [`decay`](super::decay).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Delta {
    Learned(Fact),
    Unlearned(Fact),
}

/// One entity's private epistemic state.
///
/// Deltas are kept **in order** and applied in order, so a later observation
/// overrides an earlier one — which is what makes being tricked, and then
/// discovering it, expressible.
#[derive(Clone, Debug)]
pub struct Mind {
    pub id: String,
    deltas: Vec<Delta>,
}

impl Mind {
    pub fn new(id: impl Into<String>) -> Mind {
        Mind {
            id: id.into(),
            deltas: Vec::new(),
        }
    }

    /// Record an epistemic event. This is the *only* way belief changes.
    pub fn observe(&mut self, delta: Delta) {
        self.deltas.push(delta);
    }

    /// Came to believe `f`.
    pub fn learn(&mut self, f: Fact) {
        self.observe(Delta::Learned(f));
    }

    /// Came to believe `f` is *not* so.
    pub fn unlearn(&mut self, f: Fact) {
        self.observe(Delta::Unlearned(f));
    }

    pub fn deltas(&self) -> &[Delta] {
        &self.deltas
    }

    /// Everything this entity currently believes: `baseline + own deltas`.
    ///
    /// Recomputed each call. At game scale (tens of facts, event-gated) that is
    /// irrelevant; caching it would be the optimisation, and it would need
    /// invalidation. Do not pay that until a profile asks.
    pub fn beliefs(&self, baseline: &Baseline) -> BTreeSet<Fact> {
        let mut set = baseline.facts.clone();
        for delta in &self.deltas {
            match delta {
                Delta::Learned(f) => {
                    set.insert(f.clone());
                }
                Delta::Unlearned(f) => {
                    set.remove(f);
                }
            }
        }
        set
    }

    pub fn believes(&self, baseline: &Baseline, f: &Fact) -> bool {
        self.beliefs(baseline).contains(f)
    }
}

/// Goal: `pattern` unifies with *something this mind believes*.
///
/// The bridge from belief into the solver. Take the belief set from
/// [`Mind::beliefs`] once and reuse it — this builds one alternative per fact.
///
/// A mind believing nothing yields [`fail`](super::kanren::fail), which is
/// correct: it can conclude nothing.
pub fn holds(beliefs: &BTreeSet<Fact>, pattern: Term) -> Goal {
    disj_any(
        beliefs
            .iter()
            .map(|f| eq(pattern.clone(), f.to_term()))
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn f(pred: &str, args: &[&str]) -> Fact {
        Fact::new(pred, args.iter().copied())
    }

    #[test]
    fn a_fact_renders_as_the_design_docs_write_it() {
        assert_eq!(
            f("is-hiding-secret", &["hostage"]).to_term().to_string(),
            "(is-hiding-secret hostage)"
        );
    }

    #[test]
    fn an_untouched_mind_believes_exactly_the_baseline() {
        let base = Baseline::new([f("at", &["artifact", "vault"])]);
        let mind = Mind::new("guard");
        assert!(mind.believes(&base, &f("at", &["artifact", "vault"])));
        assert_eq!(mind.beliefs(&base).len(), 1);
    }

    #[test]
    fn observing_overrides_the_baseline_for_that_mind_only() {
        let base = Baseline::new([f("at", &["artifact", "vault"])]);
        let mut seen = Mind::new("guard-who-looked");
        let blind = Mind::new("guard-who-did-not");

        seen.unlearn(f("at", &["artifact", "vault"]));
        seen.learn(f("gone", &["artifact"]));

        // The one who looked knows. The one who didn't is still wrong — forever,
        // unless something tells him. This is the entire point.
        assert!(!seen.believes(&base, &f("at", &["artifact", "vault"])));
        assert!(seen.believes(&base, &f("gone", &["artifact"])));
        assert!(blind.believes(&base, &f("at", &["artifact", "vault"])));
        assert!(!blind.believes(&base, &f("gone", &["artifact"])));
    }

    #[test]
    fn later_observations_override_earlier_ones() {
        let base = Baseline::new([]);
        let mut mind = Mind::new("guard");
        mind.learn(f("at", &["player", "corridor"]));
        mind.unlearn(f("at", &["player", "corridor"]));
        mind.learn(f("at", &["player", "corridor"]));
        assert!(mind.believes(&base, &f("at", &["player", "corridor"])));
    }

    #[test]
    fn holds_finds_a_believed_fact_and_binds_its_argument() {
        use super::super::kanren::{run, Term};
        let base = Baseline::new([f("lies-toward", &["usb-stick", "north"])]);
        let mind = Mind::new("guard");
        let beliefs = mind.beliefs(&base);

        let answers = run(None, |q| {
            holds(
                &beliefs,
                Term::list([Term::atom("lies-toward"), q, Term::atom("north")]),
            )
        });
        assert_eq!(answers, vec![Term::atom("usb-stick")]);
    }

    #[test]
    fn a_mind_believing_nothing_concludes_nothing() {
        use super::super::kanren::run;
        let beliefs = BTreeSet::new();
        let answers = run(None, |q| holds(&beliefs, q));
        assert!(answers.is_empty());
    }
}
