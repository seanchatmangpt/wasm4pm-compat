//! Status-driven system shapes — closed-form task-success probability
//! structural surface (Qi et al. 2025, *Closed-Form and Boundary Expressions
//! for Task-Success Probability in Status-Driven Systems*).
//!
//! ## What this module IS
//!
//! - The **shape** of a status-driven system: a finite set of named `Status`
//!   states, and the subset of transitions between them that are *active*
//!   (permitted) at any given moment — [`StatusDrivenSystem`].
//! - [`TaskSuccessProbability`] — a `Between01`-bounded metric type for the
//!   probability that a task reaches a success status, structurally distinct
//!   from any other `[0, 1]` metric in this crate ([`crate::law::Between01`]
//!   siblings like `StochasticArcWeight`, `DependencyMeasure`).
//!
//! ## What this module is **NOT**
//!
//! - Not a computation. It does not derive the closed-form or boundary
//!   probability expressions from the paper's Markov-chain formulation; it
//!   only names the state/transition/probability shapes those expressions are
//!   computed over. Graduate to `wasm4pm` for the actual probability
//!   derivation.
//!
//! ## Paper
//!
//! Qi et al. (2025). *Closed-Form and Boundary Expressions for Task-Success
//! Probability in Status-Driven Systems*. See
//! [`crate::witnesses_domain::ClosedFormTaskSuccessBeamPaper`] for the witness
//! marker citation.

use crate::law::{Between01, IsTrue, Require};

// ── Status ───────────────────────────────────────────────────────────────────

/// A named state in a status-driven system.
///
/// Structure-only: a status is an opaque label plus a `terminal` flag (whether
/// the status is a success/failure sink with no further active transitions).
/// The paper's specific status vocabulary (e.g. `Queued`, `Running`,
/// `Succeeded`, `Failed`) is domain-defined; this type names the shared shape.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Status {
    name: String,
    terminal: bool,
}

impl Status {
    pub fn new(name: impl Into<String>) -> Self {
        Status {
            name: name.into(),
            terminal: false,
        }
    }

    /// Marks this status as terminal (a success or failure sink).
    pub fn terminal(mut self) -> Self {
        self.terminal = true;
        self
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_terminal(&self) -> bool {
        self.terminal
    }
}

// ── Active transition constraint ────────────────────────────────────────────

/// A named constraint that gates whether a transition between two [`Status`]
/// indices is currently *active* (permitted), as opposed to merely
/// structurally reachable.
///
/// The paper distinguishes the full transition graph from the *active*
/// sub-graph at a given boundary condition — this witness names that
/// distinction at the type level without evaluating the constraint.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ActiveTransitionConstraint {
    pub from: usize,
    pub to: usize,
    /// Human-readable name of the boundary condition gating this transition
    /// (e.g. `"resource_available"`, `"deadline_not_exceeded"`).
    pub constraint_name: &'static str,
}

impl ActiveTransitionConstraint {
    pub const fn new(from: usize, to: usize, constraint_name: &'static str) -> Self {
        ActiveTransitionConstraint {
            from,
            to,
            constraint_name,
        }
    }
}

// ── StatusDrivenSystem ───────────────────────────────────────────────────────

/// A status-driven system: a finite [`Status`] set plus the
/// [`ActiveTransitionConstraint`]s that gate movement between them.
///
/// Structure-only: this type names which transitions are *claimed* active. It
/// does not evaluate a constraint against live state, nor does it compute any
/// success probability — see [`TaskSuccessProbability`] for the metric shape,
/// and graduate to `wasm4pm` for the Markov-chain probability derivation
/// itself.
#[derive(Debug, Clone, Default)]
pub struct StatusDrivenSystem {
    statuses: Vec<Status>,
    active_transitions: Vec<ActiveTransitionConstraint>,
}

impl StatusDrivenSystem {
    pub fn new(statuses: impl IntoIterator<Item = Status>) -> Self {
        StatusDrivenSystem {
            statuses: statuses.into_iter().collect(),
            active_transitions: Vec::new(),
        }
    }

    pub fn statuses(&self) -> &[Status] {
        &self.statuses
    }

    pub fn with_active_transition(mut self, constraint: ActiveTransitionConstraint) -> Self {
        self.active_transitions.push(constraint);
        self
    }

    pub fn active_transitions(&self) -> &[ActiveTransitionConstraint] {
        &self.active_transitions
    }

    /// Named indices of the terminal (success/failure sink) statuses.
    pub fn terminal_statuses(&self) -> impl Iterator<Item = usize> + '_ {
        self.statuses
            .iter()
            .enumerate()
            .filter(|(_, s)| s.is_terminal())
            .map(|(i, _)| i)
    }

    /// Validates the system: every transition endpoint must reference a valid
    /// status index.
    pub fn validate(&self) -> Result<(), StatusDrivenRefusal> {
        if self.statuses.is_empty() {
            return Err(StatusDrivenRefusal::EmptyStatusSet);
        }
        let n = self.statuses.len();
        for t in &self.active_transitions {
            if t.from >= n || t.to >= n {
                return Err(StatusDrivenRefusal::DanglingTransition);
            }
        }
        Ok(())
    }
}

/// Named refusal variants for [`StatusDrivenSystem`] validation laws.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatusDrivenRefusal {
    /// An active transition references a status index outside the status set.
    DanglingTransition,
    /// The system declares no statuses — a status-driven system needs at
    /// least one state to reach.
    EmptyStatusSet,
}

impl core::fmt::Display for StatusDrivenRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            StatusDrivenRefusal::DanglingTransition => write!(f, "DanglingTransition"),
            StatusDrivenRefusal::EmptyStatusSet => write!(f, "EmptyStatusSet"),
        }
    }
}

impl std::error::Error for StatusDrivenRefusal {}

// ── TaskSuccessProbability ───────────────────────────────────────────────────

/// A closed-form task-success probability, provably in `[0, 1]` at the type
/// level via [`crate::law::Between01`] — the metric shape the paper's
/// closed-form and boundary expressions evaluate to.
///
/// Distinct from other `Between01`-bounded metrics in this crate
/// ([`crate::petri::StochasticArcWeight`], [`crate::causal_net::DependencyMeasure`])
/// so a task-success probability cannot be silently substituted for an arc
/// weight or a dependency measure at a call site expecting this type.
///
/// ```
/// # #![feature(generic_const_exprs, adt_const_params)]
/// # #![allow(incomplete_features)]
/// use wasm4pm_compat::status_driven::TaskSuccessProbability;
/// let p: TaskSuccessProbability<3, 4> = TaskSuccessProbability::new();
/// assert_eq!(p.as_ratio(), (3, 4));
/// ```
///
/// ```compile_fail
/// use wasm4pm_compat::status_driven::TaskSuccessProbability;
/// let _: TaskSuccessProbability<5, 4> = TaskSuccessProbability::new(); // 5/4 > 1
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskSuccessProbability<const NUM: u64, const DEN: u64>
where
    Require<{ DEN > 0 }>: IsTrue,
    Require<{ NUM <= DEN }>: IsTrue,
{
    metric: Between01<NUM, DEN>,
}

impl<const NUM: u64, const DEN: u64> TaskSuccessProbability<NUM, DEN>
where
    Require<{ DEN > 0 }>: IsTrue,
    Require<{ NUM <= DEN }>: IsTrue,
{
    pub const fn new() -> Self {
        TaskSuccessProbability {
            metric: Between01::new(),
        }
    }

    pub const fn as_ratio(&self) -> (u64, u64) {
        (self.metric.num(), self.metric.den())
    }
}

impl<const NUM: u64, const DEN: u64> Default for TaskSuccessProbability<NUM, DEN>
where
    Require<{ DEN > 0 }>: IsTrue,
    Require<{ NUM <= DEN }>: IsTrue,
{
    fn default() -> Self {
        Self::new()
    }
}
