// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>

//! # microKanren — the relational solver
//!
//! A small, complete relational engine: `unify` / `fresh` / `conj` / `disj` /
//! `run`. It is the reasoning substrate the ESM thinks with.
//!
//! ## Why relational logic, and not `if` statements
//!
//! Because **the number of answers is the confidence signal**, for free:
//!
//! | Answers | The NPC's situation |
//! |---|---|
//! | many | *Ambiguous* — could be the USB-stick or the note. Cover the bottleneck. |
//! | one  | *Unified* — high confidence. Intercept. |
//! | zero | *Contradiction* — an assumption just broke. "Wait, I've been tricked!" |
//!
//! No separate confidence heuristic to write, tune, or keep honest. It falls out
//! of the solver. That is the whole reason this module exists rather than a pile
//! of hand-rolled conditionals — see [`crate::esm::intent`].
//!
//! ## How it works, for whoever reads this next
//!
//! Four ideas, and they compose into everything else:
//!
//! 1. A [`Term`] is a variable, an atom (a ground symbol like `"usb-stick"`), a
//!    pair, or nil. Pairs + nil build lists; lists represent facts, e.g.
//!    `(lies-toward usb-stick north)`.
//! 2. A [`Subst`] maps variables to terms. [`Subst::walk`] follows a variable
//!    through the chain of bindings until it reaches something that is not a
//!    bound variable.
//! 3. [`unify`] asks "can these two terms be made equal?" and answers with an
//!    *extended substitution* (yes, and here's what that costs) or `None` (no).
//! 4. A [`Goal`] is a function from one state to *zero or more* states. Zero =
//!    failure. Many = several ways to be true. [`conj`] chains goals; [`disj`]
//!    offers alternatives.
//!
//! `run` makes a fresh query variable, runs the goal, and reports what that
//! variable became in each surviving state.
//!
//! ## The deliberate limitation: eager streams
//!
//! Real microKanren returns a *lazy, interleaved* stream, which is what lets it
//! express productively-infinite relations and recursive rules that would
//! otherwise diverge. **This implementation returns an eager [`Vec`]** (ADR-0004,
//! recorded there as a known negative).
//!
//! That is right for this game: every question we ask is over a **finite,
//! small candidate set** (which of these four targets? which of these six
//! instructions survived?), and eager code is dramatically easier to read, debug,
//! and hand on than a hand-rolled lazy stream in Rust.
//!
//! **What it forbids, concretely:** do not write a recursive relation whose
//! search space is unbounded — it will not be lazily cut short, it will hang.
//! Prefer [`membero`] over a hand-written recursive list relation. If a relation
//! ever genuinely needs to be productively infinite, that is the signal to
//! revisit this decision — not to work around it locally.
//!
//! ## Other deliberate simplifications
//!
//! * **No occurs check** in [`unify`] — classic microKanren omits it, and the
//!   cyclic terms it guards against cannot arise from the finite, ground facts
//!   the scene supplies. If you ever unify two open structures against each
//!   other, revisit this.
//! * **`Subst` is a cloned `HashMap`.** Branching clones the map. At game scale
//!   (tens of facts, single-digit candidates, event-gated) this is irrelevant. A
//!   persistent assoc-list would be the optimisation, and it would cost
//!   readability; do not pay that until a profile asks.

use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

/// A logic variable. Just an index; meaning comes from the [`Subst`].
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct Var(pub usize);

/// A term: the thing the solver reasons about.
///
/// Facts are represented as proper lists of atoms, e.g.
/// `(is-hiding-secret hostage)` — build them with [`Term::list`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Term {
    /// An unknown, to be solved for.
    Var(Var),
    /// A ground symbol: `"usb-stick"`, `"north"`, `"expose-secret"`.
    Atom(String),
    /// Cons cell. `Pair(head, tail)`.
    Pair(Rc<Term>, Rc<Term>),
    /// The empty list; terminates a proper list.
    Nil,
}

impl Term {
    /// A ground symbol.
    pub fn atom(s: impl Into<String>) -> Term {
        Term::Atom(s.into())
    }

    /// Build a proper list: `list([a, b])` is `(a b)`, i.e. `Pair(a, Pair(b, Nil))`.
    pub fn list(items: impl IntoIterator<Item = Term>) -> Term {
        let items: Vec<Term> = items.into_iter().collect();
        items.into_iter().rev().fold(Term::Nil, |tail, head| {
            Term::Pair(Rc::new(head), Rc::new(tail))
        })
    }

    /// The atom's text, if this is an atom. Convenience for reading answers out.
    pub fn as_atom(&self) -> Option<&str> {
        match self {
            Term::Atom(s) => Some(s.as_str()),
            _ => None,
        }
    }
}

impl fmt::Display for Term {
    /// Renders in Scheme-ish notation so debug output matches the design docs.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Var(Var(n)) => write!(f, "_{n}"),
            Term::Atom(s) => write!(f, "{s}"),
            Term::Nil => write!(f, "()"),
            Term::Pair(_, _) => {
                write!(f, "(")?;
                let mut cur = self.clone();
                let mut first = true;
                loop {
                    match cur {
                        Term::Pair(head, tail) => {
                            if !first {
                                write!(f, " ")?;
                            }
                            write!(f, "{head}")?;
                            first = false;
                            cur = (*tail).clone();
                        }
                        Term::Nil => break,
                        // Improper list: show the dotted tail rather than lying.
                        other => {
                            write!(f, " . {other}")?;
                            break;
                        }
                    }
                }
                write!(f, ")")
            }
        }
    }
}

/// A substitution: what each variable has been bound to so far.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Subst(HashMap<Var, Term>);

impl Subst {
    pub fn new() -> Self {
        Subst(HashMap::new())
    }

    /// Follow a term through variable bindings until it is no longer a *bound*
    /// variable. Returns either a non-variable term, or an unbound variable.
    ///
    /// This is shallow: it does not walk inside pairs. [`Subst::reify`] does that.
    pub fn walk(&self, t: &Term) -> Term {
        let mut cur = t.clone();
        while let Term::Var(v) = cur {
            match self.0.get(&v) {
                Some(next) => cur = next.clone(),
                None => return Term::Var(v),
            }
        }
        cur
    }

    /// A copy of this substitution with one more binding. Never mutates in place:
    /// a failed unification must leave the caller's substitution untouched.
    fn extend(&self, v: Var, t: Term) -> Subst {
        let mut m = self.0.clone();
        m.insert(v, t);
        Subst(m)
    }

    /// Resolve a term as far as possible, *including inside pairs*.
    ///
    /// Unbound variables are left as [`Term::Var`] — the caller can spot them.
    pub fn reify(&self, t: &Term) -> Term {
        let t = self.walk(t);
        match t {
            Term::Pair(head, tail) => {
                Term::Pair(Rc::new(self.reify(&head)), Rc::new(self.reify(&tail)))
            }
            other => other,
        }
    }
}

/// Can `u` and `v` be made equal? If so, at what cost to the substitution?
///
/// Returns the extended substitution, or `None` for "these can never be equal"
/// — which is the solver's `failo`, and the raw material of
/// [`crate::esm::intent::Read::Contradiction`].
pub fn unify(u: &Term, v: &Term, s: &Subst) -> Option<Subst> {
    let u = s.walk(u);
    let v = s.walk(v);
    match (&u, &v) {
        // Same variable: already equal, nothing to record.
        (Term::Var(a), Term::Var(b)) if a == b => Some(s.clone()),
        // An unbound variable can become anything.
        (Term::Var(a), _) => Some(s.extend(*a, v.clone())),
        (_, Term::Var(b)) => Some(s.extend(*b, u.clone())),
        // Two ground atoms unify only if identical.
        (Term::Atom(a), Term::Atom(b)) if a == b => Some(s.clone()),
        (Term::Nil, Term::Nil) => Some(s.clone()),
        // Structural: heads must unify, then tails, threading the substitution.
        (Term::Pair(ua, ud), Term::Pair(va, vd)) => {
            let s = unify(ua, va, s)?;
            unify(ud, vd, &s)
        }
        _ => None,
    }
}

/// A point in the search: what is known, and where the next fresh variable starts.
#[derive(Clone, Debug, Default)]
pub struct State {
    pub subst: Subst,
    /// Counter for handing out fresh variables. Threaded through so that goals
    /// stay pure functions of a state rather than needing shared mutation.
    next: usize,
}

impl State {
    pub fn new() -> Self {
        State {
            subst: Subst::new(),
            next: 0,
        }
    }

    /// A copy of this state, plus a brand-new variable nothing has bound yet.
    pub fn with_fresh(&self) -> (State, Var) {
        let v = Var(self.next);
        (
            State {
                subst: self.subst.clone(),
                next: self.next + 1,
            },
            v,
        )
    }
}

/// A goal: from one state, produce every state in which the goal holds.
///
/// Empty vec = failure. Several = several ways of being true. **That count is
/// the confidence signal** the whole design leans on.
pub type Goal = Rc<dyn Fn(&State) -> Vec<State>>;

/// `u` and `v` are the same thing. The primitive goal; everything builds on it.
pub fn eq(u: Term, v: Term) -> Goal {
    Rc::new(move |st: &State| match unify(&u, &v, &st.subst) {
        Some(subst) => vec![State {
            subst,
            next: st.next,
        }],
        None => vec![],
    })
}

/// Always succeeds, changing nothing. Identity for [`conj_all`].
pub fn succeed() -> Goal {
    Rc::new(|st: &State| vec![st.clone()])
}

/// Always fails. Identity for [`disj_any`].
pub fn fail() -> Goal {
    Rc::new(|_st: &State| Vec::new())
}

/// Introduce a new unknown, then build a goal that talks about it.
///
/// ```ignore
/// fresh(|x| eq(x, Term::atom("north")))
/// ```
pub fn fresh<F>(f: F) -> Goal
where
    F: Fn(Term) -> Goal + 'static,
{
    Rc::new(move |st: &State| {
        let (st, v) = st.with_fresh();
        f(Term::Var(v))(&st)
    })
}

/// Both goals hold. Each state `a` produces is fed through `b`.
pub fn conj(a: Goal, b: Goal) -> Goal {
    Rc::new(move |st: &State| a(st).iter().flat_map(|s| b(s)).collect())
}

/// Either goal holds. The results are concatenated.
///
/// Eager, so ordering is simply a-then-b — there is no interleaving to reason
/// about, and no fairness to preserve. See this module's note on eager streams.
pub fn disj(a: Goal, b: Goal) -> Goal {
    Rc::new(move |st: &State| {
        let mut out = a(st);
        out.extend(b(st));
        out
    })
}

/// All of these goals hold. Empty = [`succeed`].
pub fn conj_all(goals: Vec<Goal>) -> Goal {
    goals.into_iter().fold(succeed(), conj)
}

/// Any of these goals holds. Empty = [`fail`]. This is `conde`, for a finite set.
pub fn disj_any(goals: Vec<Goal>) -> Goal {
    goals.into_iter().fold(fail(), disj)
}

/// `x` is one of `items`.
///
/// The finite-domain workhorse. Prefer this to a hand-rolled recursive list
/// relation — with eager streams, recursion has nothing to cut it short.
pub fn membero(x: Term, items: impl IntoIterator<Item = Term>) -> Goal {
    disj_any(items.into_iter().map(|item| eq(x.clone(), item)).collect())
}

/// Solve for one query variable, and report what it became in each answer.
///
/// `limit` caps the answers returned; `None` means all of them (`run*`).
///
/// ```ignore
/// let answers = run(None, |q| membero(q, vec![Term::atom("a"), Term::atom("b")]));
/// // => [a, b]  — two answers, therefore *ambiguous*
/// ```
pub fn run<F>(limit: Option<usize>, f: F) -> Vec<Term>
where
    F: Fn(Term) -> Goal,
{
    let (st, q) = State::new().with_fresh();
    let query = Term::Var(q);
    let goal = f(query.clone());
    let mut answers: Vec<Term> = goal(&st).iter().map(|s| s.subst.reify(&query)).collect();
    if let Some(n) = limit {
        answers.truncate(n);
    }
    answers
}

#[cfg(test)]
mod tests {
    use super::*;

    fn a(s: &str) -> Term {
        Term::atom(s)
    }

    #[test]
    fn an_atom_unifies_only_with_itself() {
        let s = Subst::new();
        assert!(unify(&a("north"), &a("north"), &s).is_some());
        assert!(unify(&a("north"), &a("south"), &s).is_none());
    }

    #[test]
    fn a_variable_takes_the_value_it_is_unified_with() {
        let answers = run(None, |q| eq(q, a("usb-stick")));
        assert_eq!(answers, vec![a("usb-stick")]);
    }

    #[test]
    fn unification_is_structural_through_lists() {
        // (lies-toward ?x north) against (lies-toward usb-stick north) binds ?x.
        let answers = run(None, |q| {
            eq(
                Term::list([a("lies-toward"), q, a("north")]),
                Term::list([a("lies-toward"), a("usb-stick"), a("north")]),
            )
        });
        assert_eq!(answers, vec![a("usb-stick")]);
    }

    #[test]
    fn a_mismatch_anywhere_in_the_structure_fails_the_whole_unification() {
        let answers = run(None, |q| {
            eq(
                Term::list([a("lies-toward"), q, a("north")]),
                Term::list([a("lies-toward"), a("usb-stick"), a("south")]),
            )
        });
        assert!(
            answers.is_empty(),
            "south != north must fail the list unify"
        );
    }

    #[test]
    fn conj_narrows_and_disj_widens() {
        // One answer: the two constraints agree only on "b".
        let unified = run(None, |q| {
            conj(
                membero(q.clone(), [a("a"), a("b")]),
                membero(q, [a("b"), a("c")]),
            )
        });
        assert_eq!(unified, vec![a("b")]);

        // Three answers: alternatives accumulate.
        let ambiguous = run(None, |q| membero(q, [a("a"), a("b"), a("c")]));
        assert_eq!(ambiguous.len(), 3);
    }

    #[test]
    fn contradictory_constraints_yield_no_answers_at_all() {
        // This is `failo` — the raw material of "I've been tricked!".
        let none = run(None, |q| conj(eq(q.clone(), a("a")), eq(q, a("b"))));
        assert!(none.is_empty());
    }

    #[test]
    fn a_variable_bound_transitively_still_walks_to_ground() {
        // q = x, x = north  =>  q reifies to north.
        let answers = run(None, |q| {
            fresh(move |x| conj(eq(q.clone(), x.clone()), eq(x, a("north"))))
        });
        assert_eq!(answers, vec![a("north")]);
    }

    #[test]
    fn fresh_variables_do_not_collide() {
        // Two independent fresh vars must not be forced equal to each other.
        let answers = run(None, |q| {
            fresh(move |x| {
                let q = q.clone();
                fresh(move |y| {
                    conj_all(vec![
                        eq(x.clone(), a("one")),
                        eq(y.clone(), a("two")),
                        eq(q.clone(), Term::list([x.clone(), y])),
                    ])
                })
            })
        });
        assert_eq!(answers, vec![Term::list([a("one"), a("two")])]);
    }

    #[test]
    fn limit_caps_the_answers() {
        let two = run(Some(2), |q| membero(q, [a("a"), a("b"), a("c")]));
        assert_eq!(two.len(), 2);
    }

    #[test]
    fn reify_renders_as_the_design_docs_write_it() {
        let t = Term::list([a("is-hiding-secret"), a("hostage")]);
        assert_eq!(t.to_string(), "(is-hiding-secret hostage)");
    }

    #[test]
    fn an_unbound_query_reports_as_a_variable_not_a_lie() {
        let answers = run(None, |_q| succeed());
        assert_eq!(answers.len(), 1);
        assert!(
            matches!(answers[0], Term::Var(_)),
            "unbound must stay visible"
        );
    }
}
