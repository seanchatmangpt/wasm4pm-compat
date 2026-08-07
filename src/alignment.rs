//! Alignment claims — the **move-classification shape** of a conformance
//! alignment, with **no cost or optimality computed**.
//!
//! [`crate::interop::ConformanceTriple`] states *which* conformance dimensions
//! (fitness/precision/generalization) are claimed, with no values measured.
//! This module gives the *fitness* dimension in particular a named shape one
//! level deeper: a claimed alignment is a sequence of moves between a log and
//! a model, each classified as synchronous (both agree), log-only (the log
//! did something the model does not permit), or model-only (the model
//! required something the log does not show). This is the shape a real
//! alignment computation would produce — **never the cost-optimal search, the
//! cost function, or the optimality proof itself**.
//!
//! ## What this module **IS**
//!
//! - [`MoveKind`]: the three move classes from alignment theory.
//! - [`AlignmentClaim`]: a claimed move sequence grounded to a process
//!   reference, with move-kind counts (never costs).
//! - [`AlignmentRefusal`]: first-class, specifically named refusals.
//!
//! ## What this module is **NOT**
//!
//! - **Not** an alignment algorithm. It never searches for a cost-optimal
//!   alignment, never assigns a move cost, and never proves optimality.
//!
//! ## Graduation
//!
//! Actually computing a cost-optimal alignment between a log and a model is a
//! `wasm4pm` job. This module only states and refuses the alignment *claim
//! shape*.

/// The three move classes from alignment theory.
///
/// No cost is attached to any variant — a move's cost is an engine concern.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum MoveKind {
    /// The log and model agree at this step.
    Synchronous,
    /// The log did something the model does not permit here.
    LogOnly,
    /// The model required something the log does not show here.
    ModelOnly,
}

/// A claimed alignment: a move sequence grounded to a process reference.
///
/// No cost values, no optimality claim — matching
/// [`crate::interop::ConformanceTriple`]'s "no values measured" discipline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlignmentClaim {
    /// The claimed move sequence, in order.
    pub moves: Vec<MoveKind>,
    /// An opaque reference naming the process instance this alignment is
    /// claimed against (structure-only: never dereferenced here).
    pub process_ref: String,
}

impl AlignmentClaim {
    /// Claims `moves` against `process_ref`.
    ///
    /// ```
    /// use wasm4pm_compat::alignment::{AlignmentClaim, MoveKind};
    /// let a = AlignmentClaim::new(vec![MoveKind::Synchronous], "case:1");
    /// assert_eq!(a.moves.len(), 1);
    /// ```
    #[must_use]
    pub fn new(moves: Vec<MoveKind>, process_ref: impl Into<String>) -> Self {
        Self {
            moves,
            process_ref: process_ref.into(),
        }
    }

    /// Whether the claim carries a non-empty `process_ref`.
    ///
    /// ```
    /// use wasm4pm_compat::alignment::AlignmentClaim;
    /// assert!(!AlignmentClaim::new(vec![], "").is_grounded());
    /// ```
    #[must_use]
    pub fn is_grounded(&self) -> bool {
        !self.process_ref.trim().is_empty()
    }

    /// Count of [`MoveKind::Synchronous`] moves.
    ///
    /// ```
    /// use wasm4pm_compat::alignment::{AlignmentClaim, MoveKind};
    /// let a = AlignmentClaim::new(vec![MoveKind::Synchronous, MoveKind::LogOnly], "case:1");
    /// assert_eq!(a.synchronous_count(), 1);
    /// ```
    #[must_use]
    pub fn synchronous_count(&self) -> usize {
        self.moves
            .iter()
            .filter(|m| matches!(m, MoveKind::Synchronous))
            .count()
    }

    /// Count of [`MoveKind::LogOnly`] moves.
    #[must_use]
    pub fn log_only_count(&self) -> usize {
        self.moves
            .iter()
            .filter(|m| matches!(m, MoveKind::LogOnly))
            .count()
    }

    /// Count of [`MoveKind::ModelOnly`] moves.
    #[must_use]
    pub fn model_only_count(&self) -> usize {
        self.moves
            .iter()
            .filter(|m| matches!(m, MoveKind::ModelOnly))
            .count()
    }

    /// Admits this claim, or refuses with a specific named law.
    ///
    /// A zero-length alignment ([`AlignmentRefusal::EmptyAlignment`]) is a
    /// vacuous claim, same discipline as
    /// [`crate::interop::ConformanceTriple::is_grounded`].
    ///
    /// ```
    /// use wasm4pm_compat::alignment::{AlignmentClaim, AlignmentRefusal};
    /// let bad = AlignmentClaim::new(vec![], "case:1");
    /// assert_eq!(bad.admit_flat(), Err(AlignmentRefusal::EmptyAlignment));
    /// ```
    #[must_use = "check the admission result"]
    pub fn admit_flat(&self) -> Result<(), AlignmentRefusal> {
        if !self.is_grounded() {
            return Err(AlignmentRefusal::UngroundedAlignment);
        }
        if self.moves.is_empty() {
            return Err(AlignmentRefusal::EmptyAlignment);
        }
        Ok(())
    }
}

/// First-class, specifically named refusals for the alignment-claim grammar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AlignmentRefusal {
    /// An [`AlignmentClaim`] carries an empty `process_ref`.
    UngroundedAlignment,
    /// An [`AlignmentClaim`] carries a zero-length `moves` sequence — a
    /// vacuous claim.
    EmptyAlignment,
}

impl core::fmt::Display for AlignmentRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let law = match self {
            AlignmentRefusal::UngroundedAlignment => "UngroundedAlignment",
            AlignmentRefusal::EmptyAlignment => "EmptyAlignment",
        };
        write!(f, "alignment refusal: {law}")
    }
}
