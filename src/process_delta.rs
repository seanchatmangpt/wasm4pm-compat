//! Process deltas — the **online/incremental** counterpart of
//! [`crate::interop::ConformanceTriple`]'s batch conformance claim.
//!
//! `ConformanceTriple` states which conformance *dimensions* a batch claim
//! covers, with no values measured. This module states which *kind of
//! deviation* an incremental process-intelligence step is claiming to have
//! observed against an admitted process model — again with **no values
//! measured, no correlation performed, no engine run**.
//!
//! ## What this module **IS**
//!
//! - [`ProcessDeltaKind`]: a first-class, named vocabulary of the ways an
//!   observed execution step can relate to an admitted process — conformant,
//!   or one of several specifically named deviation classes (including
//!   deliberately *unresolved* classes like [`ProcessDeltaKind::IncompleteTrace`]
//!   and [`ProcessDeltaKind::TelemetryGap`], so that incomplete evidence is
//!   never silently promoted into a conformance verdict).
//! - [`ProcessDelta`]: pairs a [`ProcessDeltaKind`] with an optional witness
//!   naming the process model it is claimed against.
//! - [`ProcessDeltaRefusal`]: first-class, specifically named refusals.
//!
//! ## What this module is **NOT**
//!
//! - **Not** a correlator, a conformance checker, or a streaming engine. It
//!   never inspects an [`crate::streaming::EventWindow`], never computes a
//!   delta from live events, and never decides *which* [`ProcessDeltaKind`]
//!   applies to a real trace.
//!
//! ## Graduation
//!
//! Computing an actual delta from live/streamed events — i.e. running
//! correlation and incremental conformance checking over an
//! [`crate::streaming::EventWindow`] to *produce* a [`ProcessDelta`] — is a
//! `wasm4pm` job. This module only states and refuses the delta *problem
//! shape*, following the same discipline as [`crate::prediction::PredictionProblem`].

use core::marker::PhantomData;

/// The named vocabulary of ways an observed execution step can relate to an
/// admitted process model.
///
/// Deliberately distinguishes an *unresolved evidence* class
/// ([`ProcessDeltaKind::IncompleteTrace`], [`ProcessDeltaKind::CorrelationAmbiguity`],
/// [`ProcessDeltaKind::TelemetryGap`]) from a *positive deviation claim*
/// ([`ProcessDeltaKind::UnexpectedTransition`] and friends): absence of evidence
/// is never, by construction, the same variant as evidence of a forbidden
/// transition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ProcessDeltaKind {
    /// The observed step matches an admitted transition of the process model.
    Conformant,
    /// The observed step is a transition the admitted model does not permit
    /// from the current state.
    UnexpectedTransition,
    /// An admitted model requires a transition that did not occur.
    MissingRequiredTransition,
    /// The observed step occurred without the authority the admitted model
    /// requires for it.
    AuthorityDeviation,
    /// The observed step occurred outside an admitted temporal bound.
    TemporalDeviation,
    /// The observed step violates a named process invariant.
    InvariantViolation,
    /// No admitted transition is currently enabled (the process is stuck).
    Deadlock,
    /// The trace evidence available is insufficient to classify the step.
    IncompleteTrace,
    /// The observed step cannot be unambiguously correlated to one process
    /// instance.
    CorrelationAmbiguity,
    /// A gap in telemetry prevents observing whether a transition occurred.
    TelemetryGap,
}

impl ProcessDeltaKind {
    /// Whether this kind represents *unresolved evidence* rather than a
    /// positive claim about the world (conformant or deviant).
    ///
    /// ```
    /// use wasm4pm_compat::process_delta::ProcessDeltaKind;
    /// assert!(ProcessDeltaKind::TelemetryGap.is_unresolved());
    /// assert!(!ProcessDeltaKind::UnexpectedTransition.is_unresolved());
    /// ```
    #[must_use]
    pub const fn is_unresolved(self) -> bool {
        matches!(
            self,
            ProcessDeltaKind::IncompleteTrace
                | ProcessDeltaKind::CorrelationAmbiguity
                | ProcessDeltaKind::TelemetryGap
        )
    }
}

/// A claimed process delta: a [`ProcessDeltaKind`] plus an optional witness
/// naming the process model the claim is made against.
///
/// No measured values, no correlated events — matching
/// [`crate::interop::ConformanceTriple`]'s "no values measured" discipline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessDelta<T = ()> {
    /// The claimed kind of this delta.
    pub kind: ProcessDeltaKind,
    /// An opaque reference naming the process instance this delta is claimed
    /// against (structure-only: never dereferenced here).
    pub process_ref: String,
    /// Type-level witness marker naming the process model authority. Zero-cost.
    pub witness: PhantomData<T>,
}

impl<T> ProcessDelta<T> {
    /// Claims `kind` against `process_ref`.
    ///
    /// ```
    /// use wasm4pm_compat::process_delta::{ProcessDelta, ProcessDeltaKind};
    /// let d = ProcessDelta::<()>::new(ProcessDeltaKind::Conformant, "case:42");
    /// assert_eq!(d.kind, ProcessDeltaKind::Conformant);
    /// ```
    #[must_use]
    pub fn new(kind: ProcessDeltaKind, process_ref: impl Into<String>) -> Self {
        Self {
            kind,
            process_ref: process_ref.into(),
            witness: PhantomData,
        }
    }

    /// Admits this delta, or refuses with a specific named law.
    ///
    /// This is the always-on, structure-only admission gate: it enforces that
    /// the claim is grounded to a non-empty `process_ref`. It does not, and
    /// cannot, validate that the claimed [`ProcessDeltaKind`] is *correct* —
    /// that is a `wasm4pm` job.
    ///
    /// ```
    /// use wasm4pm_compat::process_delta::{ProcessDelta, ProcessDeltaKind, ProcessDeltaRefusal};
    /// let bad = ProcessDelta::<()>::new(ProcessDeltaKind::Conformant, "");
    /// assert_eq!(bad.admit_flat(), Err(ProcessDeltaRefusal::UngroundedDelta));
    /// ```
    #[must_use = "check the admission result"]
    pub fn admit_flat(&self) -> Result<(), ProcessDeltaRefusal> {
        if self.process_ref.trim().is_empty() {
            return Err(ProcessDeltaRefusal::UngroundedDelta);
        }
        Ok(())
    }
}

/// First-class, specifically named refusals for the process-delta grammar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ProcessDeltaRefusal {
    /// A [`ProcessDelta`] carries an empty `process_ref` — an ungrounded claim.
    UngroundedDelta,
    /// A [`ProcessDeltaKind::CorrelationAmbiguity`] delta was promoted into a
    /// positive conformance/deviation claim without resolving the ambiguity
    /// first.
    AmbiguousCorrelation,
}

impl core::fmt::Display for ProcessDeltaRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let law = match self {
            ProcessDeltaRefusal::UngroundedDelta => "UngroundedDelta",
            ProcessDeltaRefusal::AmbiguousCorrelation => "AmbiguousCorrelation",
        };
        write!(f, "process delta refusal: {law}")
    }
}
