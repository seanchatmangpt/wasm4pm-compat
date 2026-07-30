//! Exact token-replay count arithmetic for the D1 correspondence perimeter.
//!
//! The floating-point facade remains outside this crate. This module computes
//! the two exact rational components of the ProcInt token-replay fitness law:
//!
//! `((1 - missing / consumed) / 2) + ((1 - remaining / produced) / 2)`.
//!
//! Keeping the components separate is load-bearing: one reduced combined
//! denominator can require 129 bits for lawful `u64` inputs, while each component
//! fits losslessly in a `u128` numerator/denominator pair.

/// A validated set of token-replay counts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReplayCounts {
    produced: u64,
    consumed: u64,
    missing: u64,
    remaining: u64,
}

/// A named refusal emitted by the D1 count admission boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayCountRefusal {
    MissingExceedsConsumed { missing: u64, consumed: u64 },
    RemainingExceedsProduced { remaining: u64, produced: u64 },
}

/// An exact, not-necessarily-reduced nonnegative rational.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExactRatio {
    numerator: u128,
    denominator: u128,
}

impl ExactRatio {
    pub const fn new(numerator: u128, denominator: u128) -> Self {
        Self {
            numerator,
            denominator,
        }
    }

    pub const fn numerator(self) -> u128 {
        self.numerator
    }

    pub const fn denominator(self) -> u128 {
        self.denominator
    }
}

/// Exact representation of the two halves of token-replay fitness.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExactFitness {
    consumed_component: ExactRatio,
    produced_component: ExactRatio,
}

impl ExactFitness {
    pub const fn new(consumed_component: ExactRatio, produced_component: ExactRatio) -> Self {
        Self {
            consumed_component,
            produced_component,
        }
    }

    pub const fn consumed_component(self) -> ExactRatio {
        self.consumed_component
    }

    pub const fn produced_component(self) -> ExactRatio {
        self.produced_component
    }
}

impl ReplayCounts {
    pub const fn try_new(
        produced: u64,
        consumed: u64,
        missing: u64,
        remaining: u64,
    ) -> Result<Self, ReplayCountRefusal> {
        if missing > consumed {
            return Err(ReplayCountRefusal::MissingExceedsConsumed { missing, consumed });
        }
        if remaining > produced {
            return Err(ReplayCountRefusal::RemainingExceedsProduced {
                remaining,
                produced,
            });
        }
        Ok(Self {
            produced,
            consumed,
            missing,
            remaining,
        })
    }

    pub const fn produced(self) -> u64 {
        self.produced
    }

    pub const fn consumed(self) -> u64 {
        self.consumed
    }

    pub const fn missing(self) -> u64 {
        self.missing
    }

    pub const fn remaining(self) -> u64 {
        self.remaining
    }

    /// Compute a total exact representation of ProcInt token-replay fitness.
    pub const fn exact_fitness(self) -> ExactFitness {
        ExactFitness::new(
            fitness_component(self.consumed, self.missing),
            fitness_component(self.produced, self.remaining),
        )
    }
}

const fn fitness_component(total: u64, deviation: u64) -> ExactRatio {
    if total == 0 {
        // Lean's rational division is total: `0 / 0 = 0`, so this half is 1/2.
        ExactRatio::new(1, 2)
    } else {
        ExactRatio::new((total - deviation) as u128, (total as u128) * 2)
    }
}

#[cfg(test)]
mod tests {
    use super::{ExactFitness, ExactRatio, ReplayCountRefusal, ReplayCounts};

    #[test]
    fn perfect_replay_is_two_exact_halves() {
        let counts = ReplayCounts::try_new(4, 4, 0, 0).expect("valid counts");
        assert_eq!(
            counts.exact_fitness(),
            ExactFitness::new(ExactRatio::new(4, 8), ExactRatio::new(4, 8))
        );
    }

    #[test]
    fn symmetric_missing_and_remaining_is_two_three_eighths() {
        let counts = ReplayCounts::try_new(4, 4, 1, 1).expect("valid counts");
        assert_eq!(
            counts.exact_fitness(),
            ExactFitness::new(ExactRatio::new(3, 8), ExactRatio::new(3, 8))
        );
    }

    #[test]
    fn zero_denominators_follow_total_rational_semantics() {
        let empty = ReplayCounts::try_new(0, 0, 0, 0).expect("valid counts");
        assert_eq!(
            empty.exact_fitness(),
            ExactFitness::new(ExactRatio::new(1, 2), ExactRatio::new(1, 2))
        );

        let no_consumption = ReplayCounts::try_new(4, 0, 0, 1).expect("valid counts");
        assert_eq!(
            no_consumption.exact_fitness(),
            ExactFitness::new(ExactRatio::new(1, 2), ExactRatio::new(3, 8))
        );

        let no_production = ReplayCounts::try_new(0, 4, 1, 0).expect("valid counts");
        assert_eq!(
            no_production.exact_fitness(),
            ExactFitness::new(ExactRatio::new(3, 8), ExactRatio::new(1, 2))
        );
    }

    #[test]
    fn invalid_count_relations_are_refused() {
        assert_eq!(
            ReplayCounts::try_new(1, 1, 2, 0),
            Err(ReplayCountRefusal::MissingExceedsConsumed {
                missing: 2,
                consumed: 1,
            })
        );
        assert_eq!(
            ReplayCounts::try_new(1, 1, 0, 2),
            Err(ReplayCountRefusal::RemainingExceedsProduced {
                remaining: 2,
                produced: 1,
            })
        );
    }

    #[test]
    fn maximum_u64_counts_remain_exact_without_overflow() {
        let counts = ReplayCounts::try_new(u64::MAX, u64::MAX, 0, 0).expect("valid counts");
        let expected = ExactRatio::new(u128::from(u64::MAX), u128::from(u64::MAX) * 2);
        assert_eq!(
            counts.exact_fitness(),
            ExactFitness::new(expected, expected)
        );
    }
}
