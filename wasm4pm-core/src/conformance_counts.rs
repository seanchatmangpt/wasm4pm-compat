//! Exact token-replay count arithmetic for the D1 correspondence perimeter.
//!
//! The floating-point facade remains outside this crate. This module computes
//! the exact rational value of the ProcInt token-replay fitness formula:
//!
//! `((1 - missing / consumed) / 2) + ((1 - remaining / produced) / 2)`.
//!
//! Division by zero follows Lean's rational convention. The constructor laws
//! force the corresponding numerator to zero whenever a denominator is zero.

/// A validated set of token-replay counts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReplayCounts {
    produced: u64,
    consumed: u64,
    missing: u64,
    remaining: u64,
}

/// A named refusal emitted by the D1 count kernel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayCountRefusal {
    MissingExceedsConsumed { missing: u64, consumed: u64 },
    RemainingExceedsProduced { remaining: u64, produced: u64 },
    ArithmeticOverflow { operation: &'static str },
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

    pub fn equivalent_to(self, other: Self) -> Result<bool, ReplayCountRefusal> {
        let left = self.numerator.checked_mul(other.denominator).ok_or(
            ReplayCountRefusal::ArithmeticOverflow {
                operation: "ratio cross multiplication (left)",
            },
        )?;
        let right = other.numerator.checked_mul(self.denominator).ok_or(
            ReplayCountRefusal::ArithmeticOverflow {
                operation: "ratio cross multiplication (right)",
            },
        )?;
        Ok(left == right)
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

    /// Compute the exact ProcInt token-replay fitness as a rational pair.
    pub fn fitness_ratio(self) -> Result<ExactRatio, ReplayCountRefusal> {
        let produced = u128::from(self.produced);
        let consumed = u128::from(self.consumed);
        let missing = u128::from(self.missing);
        let remaining = u128::from(self.remaining);

        match (consumed, produced) {
            (0, 0) => Ok(ExactRatio::new(1, 1)),
            (0, p) => {
                let denominator = checked_twice(p, "2 * produced")?;
                let numerator = denominator.checked_sub(remaining).ok_or(
                    ReplayCountRefusal::ArithmeticOverflow {
                        operation: "2 * produced - remaining",
                    },
                )?;
                Ok(ExactRatio::new(numerator, denominator))
            }
            (c, 0) => {
                let denominator = checked_twice(c, "2 * consumed")?;
                let numerator = denominator.checked_sub(missing).ok_or(
                    ReplayCountRefusal::ArithmeticOverflow {
                        operation: "2 * consumed - missing",
                    },
                )?;
                Ok(ExactRatio::new(numerator, denominator))
            }
            (c, p) => {
                let product = c
                    .checked_mul(p)
                    .ok_or(ReplayCountRefusal::ArithmeticOverflow {
                        operation: "consumed * produced",
                    })?;
                let denominator = checked_twice(product, "2 * consumed * produced")?;
                let missing_term =
                    missing
                        .checked_mul(p)
                        .ok_or(ReplayCountRefusal::ArithmeticOverflow {
                            operation: "missing * produced",
                        })?;
                let remaining_term =
                    remaining
                        .checked_mul(c)
                        .ok_or(ReplayCountRefusal::ArithmeticOverflow {
                            operation: "remaining * consumed",
                        })?;
                let numerator = denominator
                    .checked_sub(missing_term)
                    .and_then(|value| value.checked_sub(remaining_term))
                    .ok_or(ReplayCountRefusal::ArithmeticOverflow {
                        operation: "2cp - mp - rc",
                    })?;
                Ok(ExactRatio::new(numerator, denominator))
            }
        }
    }
}

fn checked_twice(value: u128, operation: &'static str) -> Result<u128, ReplayCountRefusal> {
    value
        .checked_mul(2)
        .ok_or(ReplayCountRefusal::ArithmeticOverflow { operation })
}

#[cfg(test)]
mod tests {
    use super::{ExactRatio, ReplayCountRefusal, ReplayCounts};

    #[test]
    fn perfect_replay_is_exactly_one() {
        let counts = ReplayCounts::try_new(4, 4, 0, 0).expect("valid counts");
        let fitness = counts.fitness_ratio().expect("fitness is representable");
        assert!(fitness
            .equivalent_to(ExactRatio::new(1, 1))
            .expect("cross products fit"));
    }

    #[test]
    fn symmetric_missing_and_remaining_is_three_quarters() {
        let counts = ReplayCounts::try_new(4, 4, 1, 1).expect("valid counts");
        let fitness = counts.fitness_ratio().expect("fitness is representable");
        assert!(fitness
            .equivalent_to(ExactRatio::new(3, 4))
            .expect("cross products fit"));
    }

    #[test]
    fn zero_denominators_follow_total_rational_semantics() {
        let empty = ReplayCounts::try_new(0, 0, 0, 0).expect("valid counts");
        assert_eq!(empty.fitness_ratio(), Ok(ExactRatio::new(1, 1)));

        let no_consumption = ReplayCounts::try_new(4, 0, 0, 1).expect("valid counts");
        assert_eq!(no_consumption.fitness_ratio(), Ok(ExactRatio::new(7, 8)));

        let no_production = ReplayCounts::try_new(0, 4, 1, 0).expect("valid counts");
        assert_eq!(no_production.fitness_ratio(), Ok(ExactRatio::new(7, 8)));
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
    fn unrepresentable_u128_formula_refuses_instead_of_saturating() {
        let counts = ReplayCounts::try_new(u64::MAX, u64::MAX, 0, 0).expect("valid counts");
        assert_eq!(
            counts.fitness_ratio(),
            Err(ReplayCountRefusal::ArithmeticOverflow {
                operation: "2 * consumed * produced",
            })
        );
    }
}
