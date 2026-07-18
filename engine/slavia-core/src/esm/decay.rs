// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>

//! # Decay — memory eroding with motion
//!
//! Design: `docs/design/20-cognitive-npcs-and-theory-of-mind.md` §B.
//!
//! Instructions given to a companion NPC decay over distance walked or time
//! elapsed. The **first instruction survives longest; the tail erodes.** The
//! relation this implements, as specified:
//!
//! ```scheme
//! (defrel (retained-instructions StepsRemaining FullList RetainedList)
//!   (conde
//!     [(== StepsRemaining 0) (== RetainedList FullList)]
//!     [(> StepsRemaining 0)
//!      (fresh (head tail decayed-tail)
//!        (== FullList (cons head tail))
//!        (decay-chance StepsRemaining tail decayed-tail)
//!        (== RetainedList (cons head decayed-tail)))]))
//! ```
//!
//! ## One mechanic, two scales
//!
//! This is *also* Chronicle II's **Accelerated Forgetting**
//! (`19-chronicle-detail-imported.md`): Anya moving too fast prunes her own trail
//! of ghost platforms. Instruction decay over steps and trail decay over speed are
//! the same idea — **memory erodes with motion** — and they should keep sharing
//! this implementation rather than growing into two systems.
//!
//! ## Why the RNG lives here and is passed in
//!
//! `decay-chance` is stochastic, and stochastic code in L2 would destroy the one
//! property that makes L2 worth having: that it is **provable**. So randomness is
//! *injected*, never ambient (ADR-0004). Same seed + same inputs = same memory,
//! always.
//!
//! [`Rng`] is ~10 lines rather than a dependency, because `slavia-core` is
//! deliberately tiny and must keep compiling to `wasm32-unknown-unknown`
//! unchanged. It is **not** cryptographic and must never be used as though it
//! were.

/// A small, fast, fully deterministic PRNG (SplitMix64).
///
/// Not cryptographic. Its only jobs are to be reproducible from a seed and to
/// need no dependency.
#[derive(Clone, Debug)]
pub struct Rng {
    state: u64,
}

impl Rng {
    /// Seed it. The same seed always replays the same sequence — that guarantee
    /// is the whole reason this type exists.
    pub fn seeded(seed: u64) -> Rng {
        Rng { state: seed }
    }

    pub fn next_u64(&mut self) -> u64 {
        // SplitMix64: strong mixing, trivial state, no dependency.
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// A value in `0..n`. Has a negligible modulo bias, which does not matter for
    /// forgetting an errand.
    pub fn below(&mut self, n: u64) -> u64 {
        if n == 0 {
            return 0;
        }
        self.next_u64() % n
    }
}

/// Never forget *everything* — an NPC who has lost the plot entirely is a bug
/// report, not a character.
const FORGET_CEILING_PCT: u64 = 95;

/// Which instructions does the companion still have, after walking `steps`?
///
/// The head always survives; each later instruction is likelier to be lost the
/// deeper it sits and the further we walked. Order is preserved — what is
/// retained is still remembered *in sequence*.
///
/// `steps == 0` retains everything, per the relation above.
pub fn retained_instructions(steps: u32, full: &[String], rng: &mut Rng) -> Vec<String> {
    if steps == 0 || full.is_empty() {
        return full.to_vec();
    }

    let mut retained = Vec::with_capacity(full.len());
    retained.push(full[0].clone()); // (cons head decayed-tail) — the head is kept

    for (depth, instruction) in full.iter().enumerate().skip(1) {
        if !forgets(rng, steps, depth) {
            retained.push(instruction.clone());
        }
    }

    retained
}

/// `decay-chance`, made concrete.
///
/// Pressure compounds with distance *and* with depth in the list: the fifth
/// errand after a long walk is far more fragile than the second after a short
/// one.
///
/// The curve is **a placeholder for design tuning**, not a considered model of
/// human memory. It is deliberately legible so it can be replaced by whatever
/// playtesting actually wants. Keep it a pure function of `(steps, depth, rng)`
/// so that replacing it cannot break determinism.
fn forgets(rng: &mut Rng, steps: u32, depth: usize) -> bool {
    let pressure = (steps as u64).saturating_mul(depth as u64);
    let chance_pct = pressure.min(FORGET_CEILING_PCT);
    rng.below(100) < chance_pct
}

#[cfg(test)]
mod tests {
    use super::*;

    fn errands() -> Vec<String> {
        [
            "fetch water",
            "mind the goat",
            "greet the miller",
            "bar the gate",
            "wait for me",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }

    #[test]
    fn standing_still_forgets_nothing() {
        let mut rng = Rng::seeded(1);
        assert_eq!(retained_instructions(0, &errands(), &mut rng), errands());
    }

    #[test]
    fn an_empty_errand_list_stays_empty() {
        let mut rng = Rng::seeded(1);
        assert!(retained_instructions(99, &[], &mut rng).is_empty());
    }

    /// The load-bearing test. If this ever fails, L2 has stopped being provable
    /// and ADR-0004's determinism rule has been broken somewhere.
    #[test]
    fn the_same_seed_remembers_the_same_things() {
        for seed in 0..50 {
            let mut a = Rng::seeded(seed);
            let mut b = Rng::seeded(seed);
            assert_eq!(
                retained_instructions(20, &errands(), &mut a),
                retained_instructions(20, &errands(), &mut b),
                "seed {seed} must replay identically"
            );
        }
    }

    #[test]
    fn the_first_instruction_always_survives_however_far_we_walk() {
        for seed in 0..100 {
            let mut rng = Rng::seeded(seed);
            let kept = retained_instructions(1_000, &errands(), &mut rng);
            assert_eq!(kept.first().map(String::as_str), Some("fetch water"));
        }
    }

    #[test]
    fn what_survives_stays_in_order() {
        for seed in 0..100 {
            let mut rng = Rng::seeded(seed);
            let kept = retained_instructions(30, &errands(), &mut rng);
            let full = errands();
            // Every retained item appears in the original order.
            let positions: Vec<usize> = kept
                .iter()
                .map(|k| full.iter().position(|f| f == k).unwrap())
                .collect();
            let mut sorted = positions.clone();
            sorted.sort_unstable();
            assert_eq!(positions, sorted, "order must be preserved (seed {seed})");
        }
    }

    /// Statistical rather than per-seed, so it cannot flake.
    #[test]
    fn walking_further_erodes_more() {
        let total = |steps: u32| -> usize {
            (0..200)
                .map(|seed| {
                    let mut rng = Rng::seeded(seed);
                    retained_instructions(steps, &errands(), &mut rng).len()
                })
                .sum()
        };
        assert!(
            total(1) > total(50),
            "a long walk must cost more memory than a short one"
        );
    }
}
