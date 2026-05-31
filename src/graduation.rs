//! The **graduation bridge** toward the full `wasm4pm` execution engine.
//!
//! `wasm4pm-compat` is, by design, a *structure-only* crate: it admits, refuses,
//! and round-trips the *shapes* of process evidence, but it executes nothing. This
//! module is the **only** public bridge that makes the boundary to the real engine
//! explicit. It implements **none** of `wasm4pm` — it merely lets a compat value
//! *declare itself a graduation candidate* and name *why* it must leave the compat
//! layer.
//!
//! ## The covenant
//!
//! > **Compat carries the evidence. `wasm4pm` adjudicates it.**
//!
//! When a host hits a wall that structure cannot answer — it needs a model
//! *discovered*, a conformance result *computed*, a log *replayed*, a receipt
//! *minted*, a benchmark *gated*, an object-centric query *executed*, or it finds
//! itself *rebuilding process mining locally* — that is not a bug in compat. It is
//! the signal to **graduate**. This module turns that signal into a typed,
//! reviewable [`GraduationCandidate`].
//!
//! ## What this module is **NOT**
//!
//! - **Not** an engine, and **not** a dependency on one. There is no `wasm4pm`
//!   import here; the bridge is a *trait surface* a host (or the engine itself)
//!   implements at the seam.
//! - **Not** an automatic escalator. Producing a candidate does not *perform*
//!   graduation; it makes the case for it.

/// Why a compat value must graduate to the `wasm4pm` execution engine.
///
/// Each variant is a *trigger sign* — a capability that structure alone cannot
/// provide. These are the lawful reasons to cross out of the compat layer; naming
/// one is how a host says "I have reached the edge of structure." It is
/// **structure only**: a reason explains a need, it does not satisfy it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum GraduationReason {
    /// A process *model must be discovered* from a log — an algorithmic job.
    NeedsDiscovery,
    /// A conformance result must be *computed* (replay/alignment), not merely
    /// claimed.
    NeedsConformanceExecution,
    /// A log must be *replayed* against a model.
    NeedsReplay,
    /// Provenance receipts must be *minted and chained*, not merely shaped.
    NeedsReceipts,
    /// A benchmark gate must be *run* to admit a result.
    NeedsBenchmarkGate,
    /// An object-centric query (OCPQ) must be *executed*, not merely declared.
    NeedsObjectCentricQueryExecution,
    /// The host has started *rebuilding process mining locally* — the strongest
    /// sign that it should adopt the engine instead of re-implementing it.
    RebuildingProcessMiningLocally,
}

impl GraduationReason {
    /// The stable tag for this reason.
    ///
    /// ```
    /// use wasm4pm_compat::graduation::GraduationReason;
    /// assert_eq!(GraduationReason::NeedsDiscovery.tag(), "needs_discovery");
    /// ```
    #[must_use]
    pub const fn tag(self) -> &'static str {
        match self {
            GraduationReason::NeedsDiscovery => "needs_discovery",
            GraduationReason::NeedsConformanceExecution => "needs_conformance_execution",
            GraduationReason::NeedsReplay => "needs_replay",
            GraduationReason::NeedsReceipts => "needs_receipts",
            GraduationReason::NeedsBenchmarkGate => "needs_benchmark_gate",
            GraduationReason::NeedsObjectCentricQueryExecution => {
                "needs_object_centric_query_execution"
            }
            GraduationReason::RebuildingProcessMiningLocally => "rebuilding_process_mining_locally",
        }
    }

    /// Whether this reason is a *hard* graduation signal — one that means the host
    /// is already past the compat layer's mandate (i.e. it is executing, or
    /// re-implementing, process mining).
    ///
    /// ```
    /// use wasm4pm_compat::graduation::GraduationReason;
    /// assert!(GraduationReason::RebuildingProcessMiningLocally.is_hard_signal());
    /// assert!(!GraduationReason::NeedsBenchmarkGate.is_hard_signal());
    /// ```
    #[must_use]
    pub const fn is_hard_signal(self) -> bool {
        matches!(
            self,
            GraduationReason::NeedsDiscovery
                | GraduationReason::NeedsConformanceExecution
                | GraduationReason::NeedsReplay
                | GraduationReason::NeedsObjectCentricQueryExecution
                | GraduationReason::RebuildingProcessMiningLocally
        )
    }
}

/// A typed, reviewable case that a compat value should leave the compat layer for
/// the `wasm4pm` engine.
///
/// A `GraduationCandidate` bundles the [`GraduationReason`], a human-readable
/// `subject` naming what is graduating, and an opaque `evidence_ref` pointing at
/// the compat evidence that justifies the move. It is the artifact a reviewer (or
/// the engine's intake) reads to decide whether — and how — to adjudicate.
///
/// It is **structure only**: holding a candidate changes nothing; it is the *case
/// for* graduation, never the act of it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraduationCandidate {
    /// Why graduation is warranted.
    pub reason: GraduationReason,
    /// What is graduating (e.g. "p2p OCEL log", "discovered Petri net").
    pub subject: String,
    /// Opaque reference to the compat evidence that grounds this candidacy.
    pub evidence_ref: String,
}

impl GraduationCandidate {
    /// Build a candidate for `subject`, grounded in `evidence_ref`, for `reason`.
    ///
    /// ```
    /// use wasm4pm_compat::graduation::{GraduationCandidate, GraduationReason};
    /// let c = GraduationCandidate::new(
    ///     GraduationReason::NeedsDiscovery,
    ///     "p2p OCEL log",
    ///     "blake3:deadbeef",
    /// );
    /// assert_eq!(c.reason, GraduationReason::NeedsDiscovery);
    /// assert!(c.is_grounded());
    /// ```
    #[must_use]
    pub fn new(
        reason: GraduationReason,
        subject: impl Into<String>,
        evidence_ref: impl Into<String>,
    ) -> Self {
        Self {
            reason,
            subject: subject.into(),
            evidence_ref: evidence_ref.into(),
        }
    }

    /// Whether the candidate references the evidence that grounds it. An
    /// ungrounded candidate is not reviewable and the engine intake should reject
    /// it.
    ///
    /// ```
    /// use wasm4pm_compat::graduation::{GraduationCandidate, GraduationReason};
    /// let c = GraduationCandidate::new(GraduationReason::NeedsReplay, "log", "");
    /// assert!(!c.is_grounded());
    /// ```
    #[must_use]
    pub fn is_grounded(&self) -> bool {
        !self.evidence_ref.trim().is_empty() && !self.subject.trim().is_empty()
    }
}

/// The single public trait that bridges a compat value to `wasm4pm`.
///
/// An implementor produces a [`GraduationCandidate`] describing why and what it
/// would graduate. The trait deliberately implements **nothing** of the engine: it
/// is the seam where the structure-only world hands a reviewable case to the
/// execution world. The engine (or a host adapter) consumes candidates; compat only
/// produces them.
///
/// It is **structure only**: implementing it makes the boundary explicit, it does
/// not cross it.
pub trait GraduateToWasm4pm {
    /// Produce the graduation case for `self`.
    ///
    /// ```
    /// use wasm4pm_compat::graduation::{GraduateToWasm4pm, GraduationCandidate, GraduationReason};
    ///
    /// struct PendingDiscovery { log_hash: String }
    ///
    /// impl GraduateToWasm4pm for PendingDiscovery {
    ///     fn candidate(&self) -> GraduationCandidate {
    ///         GraduationCandidate::new(
    ///             GraduationReason::NeedsDiscovery,
    ///             "pending discovery",
    ///             self.log_hash.clone(),
    ///         )
    ///     }
    /// }
    ///
    /// let c = PendingDiscovery { log_hash: "blake3:abc".into() }.candidate();
    /// assert_eq!(c.reason, GraduationReason::NeedsDiscovery);
    /// assert!(c.is_grounded());
    /// ```
    fn candidate(&self) -> GraduationCandidate;
}
