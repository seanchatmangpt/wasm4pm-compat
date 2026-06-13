//! Compile-time law kernel — `ConstParamTy` enums, bounds machinery, and
//! type-level invariants derived from process-mining papers and the Blue River
//! Dam covenant.
//!
//! ## What this module **IS**
//!
//! The machine room for this crate's nightly type law:
//!
//! - [`crate::law::Assert`] / [`crate::law::IsTrue`] / [`crate::law::Require`] — the "must be true" compile-time
//!   gate. Any where-bound `Require<{ EXPR }>: IsTrue` becomes a compile error
//!   when `EXPR` evaluates to `false`. Requires `generic_const_exprs`.
//! - [`crate::law::ConditionCell`] — the *Need9 means split* law: at most 8 primary
//!   condition bits. `ConditionCell<9>` does **not compile**.
//! - [`crate::law::Between01`] — metrics that are provably in `[0, 1]` at the
//!   type level. `Between01<2, 1>` does **not compile**.
//! - [`core::marker::ConstParamTy`] enum set — every lawful state, mode, law, and kind
//!   that travels as a const generic parameter across this crate.
//!
//! ## What this module is **NOT**
//!
//! - **Not** a runtime type; every item here is zero-cost or phantom.
//! - **Not** an engine. Nothing here discovers, replays, or conforms.
//!
//! ## Graduation
//!
//! This module has no graduation path — it is purely structural law. The types
//! that USE these laws graduate to `wasm4pm`.

use core::marker::ConstParamTy;

// ── Compile-time bounds machinery ───────────────────────────────────────────

/// Type-level boolean assertion. `Assert<true>` is inhabited; `Assert<false>`
/// is not (no `IsTrue` impl).
///
/// Used via the [`crate::law::Require`] alias and [`crate::law::IsTrue`] trait to turn boolean const
/// expressions into compile-time law gates.
///
/// ```
/// use wasm4pm_compat::law::{Assert, IsTrue};
/// fn only_when_true<const OK: bool>() where Assert<OK>: IsTrue {}
/// only_when_true::<true>();  // compiles
/// ```
pub struct Assert<const OK: bool>;

/// The inhabited side of [`crate::law::Assert`]: only `Assert<true>` satisfies this bound.
///
/// Write where-bounds as `where Require<{ EXPR }>: IsTrue` to get a compile
/// error when `EXPR` is false.
///
/// ```
/// # #![feature(generic_const_exprs)]
/// # #![allow(incomplete_features)]
/// use wasm4pm_compat::law::{Assert, IsTrue};
/// fn require_non_zero<const N: usize>() where Assert<{ N > 0 }>: IsTrue {}
/// require_non_zero::<1>();  // ok
/// ```
pub trait IsTrue {}

impl IsTrue for Assert<true> {}

/// Alias for [`crate::law::Assert`]; read as "this must be true".
///
/// ```
/// # #![feature(generic_const_exprs)]
/// # #![allow(incomplete_features)]
/// use wasm4pm_compat::law::{Require, IsTrue};
/// fn fits_in_byte<const N: usize>() where Require<{ N <= 255 }>: IsTrue {}
/// fits_in_byte::<100>();
/// ```
pub type Require<const OK: bool> = Assert<OK>;

// ── Need-9 law ───────────────────────────────────────────────────────────────

/// A condition cell of at most **8 primary bits** — the *Need9 means split*
/// law from the Blue River Dam covenant.
///
/// `ConditionCell<9>` does **not compile**: the `BITS <= 8` constraint fails.
/// When you need 9 primary condition bits, split into two cells.
///
/// ```
/// # #![feature(generic_const_exprs)]
/// # #![allow(incomplete_features)]
/// use wasm4pm_compat::law::ConditionCell;
/// let _: ConditionCell<8> = ConditionCell::new();  // 8 bits: lawful
/// ```
///
/// ```compile_fail
/// use wasm4pm_compat::law::ConditionCell;
/// let _: ConditionCell<9>;  // 9 bits: Need9 compile error
/// ```
pub struct ConditionCell<const BITS: usize>
where
    // DEVELOPER NOTE — if this bound fails you will see:
    //   "the trait bound `Require<false>: IsTrue` is not satisfied"
    // This means: BITS > 8 violates the *Need9 means split* law (Blue River
    // Dam covenant). A condition cell may hold at most 8 primary bits.
    // Fix: split your cell into two ConditionCell<N> values where N ≤ 8 each.
    Require<{ BITS <= 8 }>: IsTrue,
{
    _private: (),
}

impl<const BITS: usize> Default for ConditionCell<BITS>
where
    // DEVELOPER NOTE — see the struct definition for the human-readable
    // violation message; this repetition is required by the compiler.
    Require<{ BITS <= 8 }>: IsTrue,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<const BITS: usize> ConditionCell<BITS>
where
    // DEVELOPER NOTE — same as the struct-level bound above; repeated because
    // Rust requires all where-clauses to be restated on each impl block.
    Require<{ BITS <= 8 }>: IsTrue,
{
    /// Constructs a `ConditionCell<BITS>` — only possible when `BITS <= 8`.
    ///
    /// ```
    /// # #![feature(generic_const_exprs)]
    /// # #![allow(incomplete_features)]
    /// use wasm4pm_compat::law::ConditionCell;
    /// let _: ConditionCell<1> = ConditionCell::new();
    /// let _: ConditionCell<8> = ConditionCell::new();
    /// ```
    pub const fn new() -> Self {
        ConditionCell { _private: () }
    }
}

// ── Between-0-and-1 metric bound ─────────────────────────────────────────────

/// A rational metric `NUM / DEN` that is provably in `[0, 1]` at the type level.
///
/// `Between01<2, 1>` does **not compile**: `NUM <= DEN` is violated.
/// `Between01<0, 0>` does **not compile**: `DEN > 0` is required.
///
/// Used to make out-of-range fitness/precision/F1 a compile
/// error rather than a runtime panic.
///
/// ```
/// # #![feature(generic_const_exprs)]
/// # #![allow(incomplete_features)]
/// use wasm4pm_compat::law::Between01;
/// let _: Between01<3, 4> = Between01::new();  // 3/4 ∈ [0,1]: lawful
/// let _: Between01<0, 1> = Between01::new();  // 0/1 = 0: lawful
/// let _: Between01<1, 1> = Between01::new();  // 1/1 = 1: lawful
/// ```
///
/// ```compile_fail
/// use wasm4pm_compat::law::Between01;
/// let _: Between01<2, 1> = Between01::new();  // 2/1 > 1: compile error
/// ```
pub struct Between01<const NUM: u64, const DEN: u64>
where
    // DEVELOPER NOTE — if this bound fails you will see:
    //   "the trait bound `Require<false>: IsTrue` is not satisfied"
    // This means: DEN == 0 — a zero denominator makes the rational undefined.
    // Fix: choose DEN ≥ 1. A valid metric value requires a non-zero denominator.
    Require<{ DEN > 0 }>: IsTrue,
    // DEVELOPER NOTE — if this bound fails you will see:
    //   "the trait bound `Require<false>: IsTrue` is not satisfied"
    // This means: NUM > DEN — the rational NUM/DEN exceeds 1.0, which violates
    // the [0, 1] metric range law for fitness, precision, F1, generalization,
    // and simplicity. Fix: ensure NUM ≤ DEN (e.g. use 3/4 for 0.75, not 4/3).
    Require<{ NUM <= DEN }>: IsTrue,
{
    _private: (),
}

impl<const NUM: u64, const DEN: u64> Default for Between01<NUM, DEN>
where
    // DEVELOPER NOTE — see the struct definition for human-readable violation
    // messages. These bounds are repeated here as required by the compiler.
    Require<{ DEN > 0 }>: IsTrue,
    Require<{ NUM <= DEN }>: IsTrue,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<const NUM: u64, const DEN: u64> Between01<NUM, DEN>
where
    // DEVELOPER NOTE — same bounds as the struct; repeated per Rust impl rules.
    Require<{ DEN > 0 }>: IsTrue,
    Require<{ NUM <= DEN }>: IsTrue,
{
    /// Constructs a `Between01<NUM, DEN>` — only possible when `DEN > 0` and
    /// `NUM <= DEN`.
    ///
    /// ```
    /// # #![feature(generic_const_exprs)]
    /// # #![allow(incomplete_features)]
    /// use wasm4pm_compat::law::Between01;
    /// let _: Between01<1, 2> = Between01::new();
    /// ```
    pub const fn new() -> Self {
        Between01 { _private: () }
    }

    /// The numerator.
    ///
    /// ```
    /// use wasm4pm_compat::law::Between01;
    /// assert_eq!(Between01::<3, 4>::new().num(), 3);
    /// ```
    pub const fn num(&self) -> u64 {
        NUM
    }

    /// The denominator.
    ///
    /// ```
    /// use wasm4pm_compat::law::Between01;
    /// assert_eq!(Between01::<3, 4>::new().den(), 4);
    /// ```
    pub const fn den(&self) -> u64 {
        DEN
    }
}

// ── GCD normalization for metrics ────────────────────────────────────────────

/// Euclidean GCD — reduces metric fractions to canonical form.
///
/// `gcd(3, 6) == 3`, `gcd(1, 2) == 1`, `gcd(0, N) == N`.
/// Used by [`NormedBetween01`] to ensure `Between01<3, 6>` and `Between01<1, 2>`
/// are the same type.
///
/// ```
/// use wasm4pm_compat::law::gcd;
/// assert_eq!(gcd(3, 6), 3);
/// assert_eq!(gcd(5, 5), 5);
/// assert_eq!(gcd(7, 1), 1);
/// ```
pub const fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

/// `Between01` normalized by GCD so equivalent ratios share the same type.
///
/// `NormedBetween01<3, 6>` and `NormedBetween01<1, 2>` are the **same type**
/// because both reduce to `Between01<1, 2>`. Without normalization, `Between01<3,
/// 6>` and `Between01<1, 2>` are distinct types even though they represent the
/// same rational value, causing spurious mismatches in conformance pipelines.
///
/// ```
/// # #![feature(generic_const_exprs, adt_const_params)]
/// # #![allow(incomplete_features)]
/// use wasm4pm_compat::law::NormedBetween01;
/// let _a: NormedBetween01<3, 6> = Default::default();
/// let _b: NormedBetween01<1, 2> = Default::default();
/// // _a and _b are the same type — Between01<1, 2>
/// ```
pub type NormedBetween01<const N: u64, const D: u64> =
    Between01<{ N / gcd(N, D) }, { D / gcd(N, D) }>;

// ── ConstParamTy enum set (adt_const_params) ─────────────────────────────────

/// Evidence lifecycle mode — what stage of the admission pipeline a value is at.
///
/// Used as a const generic parameter on types that carry lifecycle state.
#[derive(ConstParamTy, PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum EvidenceMode {
    /// Untrusted input, not yet checked.
    Raw,
    /// Structurally well-formed, not yet admitted.
    Parsed,
    /// Crossed the named boundary law.
    Admitted,
    /// Declined at the named boundary law (terminal).
    Refused,
    /// Produced by a named, accounted lossy projection.
    Projected,
    /// Cleared to leave the crate boundary.
    Exportable,
    /// Carries a soundness / provenance witness.
    Witnessed,
    /// Sealed in a provenance-bearing receipt envelope.
    Receipted,
}

/// The projection law governing how loss is accounted during a transformation.
#[derive(ConstParamTy, PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum ProjectionLaw {
    /// No evidence may be lost.
    Lossless,
    /// Loss is allowed under a named projection.
    NamedProjection,
    /// Loss is allowed and must be itemised in a `LossReport`.
    LossReportRequired,
    /// Loss is forbidden; refuse if it would occur.
    RefuseLoss,
}

/// The admission law that gates `Raw → Admitted` transitions.
#[derive(ConstParamTy, PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum AdmissionLaw {
    /// No check performed (dangerous; marked explicitly).
    Unchecked,
    /// Structural parsing only — well-formed but not judged.
    ParsedOnly,
    /// A named witness must thread through the admission.
    WitnessRequired,
    /// The admission must carry a first-class refusal path.
    RefusalRequired,
    /// A loss policy must be declared before any lossy admission.
    LossPolicyRequired,
    /// A receipt must accompany the admitted value.
    ReceiptRequired,
}

/// The kind of an external format envelope.
#[derive(ConstParamTy, PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum FormatKindConst {
    /// OCEL 2.0 JSON.
    OcelJson,
    /// OCEL 2.0 XML.
    OcelXml,
    /// OCEL 2.0 SQLite.
    OcelSqlite,
    /// OCEL 2.0 NDJSON.
    OcelNdjson,
    /// XES XML (IEEE 1849).
    XesXml,
    /// BPMN 2.0 XML.
    BpmnXml,
    /// Petri net PNML.
    Pnml,
    /// POWL JSON.
    PowlJson,
    /// Native compat binary.
    CompatNative,
}

/// Whether a Petri-net node is a place or a transition.
///
/// Used as a const param to enforce bipartite arc law at the type level.
#[derive(ConstParamTy, PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum EndpointKind {
    /// A token-holding place.
    Place,
    /// A firing-element transition.
    Transition,
}

/// The direction of a Petri-net arc (bipartite law).
#[derive(ConstParamTy, PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum ArcDirectionConst {
    /// Place → Transition (pre-incidence).
    PlaceToTransition,
    /// Transition → Place (post-incidence).
    TransitionToPlace,
}

/// The soundness state of a WF-net — tracks witnessing at the type level.
///
/// `SoundnessState::Witnessed` is the non-forgeable state. Only the sanctioned
/// `witness_soundness` path in `petri` produces a `WfNetConst<{Witnessed}>`.
#[derive(ConstParamTy, PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum SoundnessState {
    /// Soundness not yet asserted.
    Unknown,
    /// Soundness claimed by upstream (unproven here).
    Claimed,
    /// Soundness witnessed via `wasm4pm` engine output.
    Witnessed,
}

/// Whether a POWL model can be projected to a block-structured process tree.
#[derive(ConstParamTy, PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum PowlProjectionState {
    /// Projectability not yet determined.
    Unknown,
    /// The POWL fragment can project losslessly to a process tree.
    ProcessTreeProjectable,
    /// The partial order exceeds any process tree; projection would lose language.
    ExceedsProcessTree,
    /// Projection was attempted and explicitly refused.
    RefusedProjection,
}

/// A quality metric kind for conformance verdicts.
#[derive(ConstParamTy, PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum QualityMetricKind {
    /// Token-replay or alignment-based fitness.
    Fitness,
    /// Precision (how much of the model is used).
    Precision,
    /// F1 harmonic mean of fitness and precision.
    F1,
    /// Generalization (how well the model covers unseen traces).
    Generalization,
    /// Simplicity (structural parsimony).
    Simplicity,
}

impl core::fmt::Display for QualityMetricKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let name = match self {
            QualityMetricKind::Fitness => "Fitness",
            QualityMetricKind::Precision => "Precision",
            QualityMetricKind::F1 => "F1",
            QualityMetricKind::Generalization => "Generalization",
            QualityMetricKind::Simplicity => "Simplicity",
        };
        f.write_str(name)
    }
}

// ── Display for SoundnessState ───────────────────────────────────────────────

impl core::fmt::Display for SoundnessState {
    /// Returns the human-readable soundness state name.
    ///
    /// ```
    /// use wasm4pm_compat::law::SoundnessState;
    /// assert_eq!(SoundnessState::Unknown.to_string(), "Unknown");
    /// assert_eq!(SoundnessState::Claimed.to_string(), "Claimed");
    /// assert_eq!(SoundnessState::Witnessed.to_string(), "Witnessed");
    /// ```
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let name = match self {
            SoundnessState::Unknown => "Unknown",
            SoundnessState::Claimed => "Claimed",
            SoundnessState::Witnessed => "Witnessed",
        };
        f.write_str(name)
    }
}

/// The kind of process-boundary a host declares it crosses.
///
/// Used in `src/strict.rs` for const-generic boundary declarations.
#[derive(ConstParamTy, PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum BoundaryClaimKind {
    /// Emits event records.
    EmitsEvents,
    /// Emits object-to-event relations.
    EmitsObjects,
    /// Emits object-to-object relations.
    EmitsObjectRelations,
    /// Imports an external format.
    ImportsFormat,
    /// Exports to an external format.
    ExportsFormat,
    /// Claims conformance verdict.
    ClaimsConformance,
    /// Claims replay capability.
    ClaimsReplay,
    /// Claims receipt production.
    ClaimsReceipt,
    /// Claims general process-mining support.
    ClaimsProcessMiningSupport,
}

// ── Export boundary const-generic type-law gate ──────────────────────────────

/// A compile-time export boundary declaration.
///
/// Both `HAS_WITNESS` and `HAS_ROUND_TRIP` are const bool parameters.
/// `enforce_export_round_trip` requires `B: HasRoundTripFixture`, which is
/// only satisfied when both are `true`. Passing
/// `ExportBoundaryConst<true, false>` fails to compile — it is missing the
/// declared round-trip fixture.
///
/// ```
/// use wasm4pm_compat::law::{ExportBoundaryConst, enforce_export_round_trip};
/// enforce_export_round_trip(&ExportBoundaryConst::<true, true>);
/// ```
///
/// ```compile_fail
/// use wasm4pm_compat::law::{ExportBoundaryConst, enforce_export_round_trip};
/// // has_witness=true, has_round_trip=false: MissingRoundTripFixture law
/// enforce_export_round_trip(&ExportBoundaryConst::<true, false>);
/// ```
pub struct ExportBoundaryConst<const HAS_WITNESS: bool, const HAS_ROUND_TRIP: bool>;

mod export_boundary_seal {
    pub trait Sealed {}
    impl Sealed for super::ExportBoundaryConst<true, true> {}
    // No other combination is sealed.
}

/// Sealed marker: only `ExportBoundaryConst<true, true>` satisfies this bound.
pub trait HasRoundTripFixture: export_boundary_seal::Sealed {}
impl HasRoundTripFixture for ExportBoundaryConst<true, true> {}

// ── Display for EvidenceMode ─────────────────────────────────────────────────

impl core::fmt::Display for EvidenceMode {
    /// Returns the human-readable stage name — e.g. `"Raw"`, `"Admitted"`.
    ///
    /// ```
    /// use wasm4pm_compat::law::EvidenceMode;
    /// assert_eq!(EvidenceMode::Raw.to_string(), "Raw");
    /// assert_eq!(EvidenceMode::Admitted.to_string(), "Admitted");
    /// assert_eq!(EvidenceMode::Refused.to_string(), "Refused");
    /// ```
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let name = match self {
            EvidenceMode::Raw => "Raw",
            EvidenceMode::Parsed => "Parsed",
            EvidenceMode::Admitted => "Admitted",
            EvidenceMode::Refused => "Refused",
            EvidenceMode::Projected => "Projected",
            EvidenceMode::Exportable => "Exportable",
            EvidenceMode::Witnessed => "Witnessed",
            EvidenceMode::Receipted => "Receipted",
        };
        f.write_str(name)
    }
}

/// Type-law gate: only export boundaries that declare both a witness and a
/// round-trip fixture compile through this function.
///
/// ```
/// use wasm4pm_compat::law::{ExportBoundaryConst, enforce_export_round_trip};
/// enforce_export_round_trip(&ExportBoundaryConst::<true, true>);
/// ```
pub fn enforce_export_round_trip<B: HasRoundTripFixture>(_: &B) {}

/// The centricity of a process model or log — whether it is organized around
/// cases, objects, or a mix of both.
///
/// Used as a const generic parameter to track at the type level whether a
/// surface is case-centric (classical XES), object-centric (OCEL 2.0), or an
/// explicit mix, so a function requiring `{ObjectCentric}` rejects
/// `{CaseCentric}` at compile time.
///
/// This is structure-only classification; it does not run discovery or projection.
/// Graduate to `wasm4pm` when centricity must be *converted*, not merely *named*.
#[derive(ConstParamTy, PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum ObjectCentricity {
    /// A single case-id column: classical XES / flat event-log model.
    CaseCentric,
    /// Multiple object types, each with their own lifecycle: OCEL 2.0 model.
    ObjectCentric,
    /// A surface that explicitly bridges or combines both centricities.
    Mixed,
}

/// The directionality of a relation in an object-centric log.
///
/// OCEL 2.0 defines three kinds of typed links:
/// - **event → object** — which objects were involved in this event,
/// - **object → object** — how objects are related to each other,
/// - **object → event** — the reverse read of an event-object link.
///
/// Used as a const generic parameter to prevent event-to-object relation slots
/// from being silently filled with object-to-object relation data. The wrong
/// combination is a compile error, not a runtime assertion failure.
///
/// Structure-only: this enum classifies link direction. Resolving or traversing
/// links is an engine concern — graduate to `wasm4pm`.
#[derive(ConstParamTy, PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum RelationLaw {
    /// A qualified link from an event to the objects it involved.
    EventToObject,
    /// A typed relationship from one object to another (peer relation).
    ObjectToObject,
    /// The reverse read: from an object back to the events it participated in.
    ObjectToEvent,
}

/// The process-evidence shape kind — what structural canon member a type is.
#[derive(ConstParamTy, PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum ProcessShapeKind {
    /// A single event.
    Event,
    /// An ordered trace (sequence of events for one case).
    Trace,
    /// A case-centric event log (collection of traces).
    EventLog,
    /// An unbounded event stream.
    EventStream,
    /// An XES log (IEEE 1849).
    XesLog,
    /// An OCEL 2.0 log.
    OcelLog,
    /// A directly-follows graph.
    DirectlyFollowsGraph,
    /// An object-centric directly-follows graph.
    ObjectCentricDfg,
    /// A Petri net.
    PetriNet,
    /// A workflow net (WF-net).
    WorkflowNet,
    /// An object-centric Petri net.
    ObjectCentricPetriNet,
    /// A process tree.
    ProcessTree,
    /// A POWL model.
    Powl,
    /// A Declare model.
    DeclareModel,
    /// An object-centric Declare model.
    ObjectCentricDeclareModel,
    /// A log-skeleton model.
    LogSkeleton,
    /// An OCPQ query.
    OcpqQuery,
    /// An alignment path.
    Alignment,
    /// A token-replay result.
    TokenReplay,
    /// A conformance verdict.
    ConformanceVerdict,
    /// A prediction problem statement.
    PredictionProblem,
    /// A receipt-shaped evidence envelope.
    Receipt,
}

/// The closed set of block-structured process-tree operator kinds.
///
/// Used as a const generic parameter to tag typed operator nodes at the type
/// level so that `TypedOperatorNode<_, SEQ, ARITY>` and
/// `TypedOperatorNode<_, XOR, ARITY>` are **different types** — the wrong
/// operator kind passed to a function requiring a specific operator is a
/// compile error, not a runtime panic.
///
/// ## Structure-only
///
/// This enum names operator kinds. It does not unfold, replay, or execute them.
/// Graduate to `wasm4pm` for discovery, simplification, or replay.
///
/// ## Paper
///
/// Leemans (2013) *Discovering Block-Structured Process Models from Event Logs*.
/// The five base operators: sequence, exclusive choice (xor), parallel (and),
/// loop, and silent (tau).
#[derive(ConstParamTy, PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum ProcessTreeOperatorKind {
    /// Strict total order of children (`->`).
    Sequence,
    /// Exclusive choice among children (`x`).
    Xor,
    /// Concurrent / interleaved children (`+`).
    Parallel,
    /// Loop: first child is the `do` body, second the `redo` body (`*`).
    Loop,
    /// Silent leaf (tau) — observable-activity-free step.
    Silent,
    /// Inclusive OR — one or more branches chosen (non-deterministic).
    Or,
}

/// A named control-flow workflow pattern from Russell, van der Aalst & ter Hofstede
/// (2016) — *Workflow Patterns: The Definitive Guide*.
///
/// Each variant names a specific control-flow pattern from the canonical 20-pattern
/// catalogue. A type carrying `WorkflowPattern` as a const param asserts that it
/// models that specific pattern at the type level — so `WfNetConst` claiming
/// `Pattern::ParallelSplit` cannot be silently confused with one claiming
/// `Pattern::ExclusiveChoice`.
///
/// ## Structure-only
///
/// The pattern name is a structural label. Verifying that a Petri net or BPMN
/// process actually realises the claimed pattern is a `wasm4pm` concern. This
/// enum only names the claim.
///
/// ## Paper
///
/// Russell, van der Aalst & ter Hofstede (2016). *Workflow Patterns: The Definitive
/// Guide*. MIT Press. Appendix B lists the 20 basic control-flow patterns.
#[derive(ConstParamTy, PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum WorkflowPattern {
    // Basic control-flow patterns (WCP-1 … WCP-5)
    /// WCP-1: Sequence — activities execute in strict order.
    Sequence,
    /// WCP-2: Parallel Split (AND-split) — all branches fire simultaneously.
    ParallelSplit,
    /// WCP-3: Synchronization (AND-join) — wait for all concurrent branches.
    Synchronization,
    /// WCP-4: Exclusive Choice (XOR-split) — exactly one branch is taken.
    ExclusiveChoice,
    /// WCP-5: Simple Merge (XOR-join) — one of several branches rejoins.
    SimpleMerge,
    // Advanced branching and synchronization (WCP-6 … WCP-9)
    /// WCP-6: Multi-Choice (OR-split) — one or more branches are taken.
    MultiChoice,
    /// WCP-7: Structured Synchronizing Merge — merge with thread-count awareness.
    StructuredSynchronizingMerge,
    /// WCP-8: Multi-Merge — merge that fires for each incoming token.
    MultiMerge,
    /// WCP-9: Structured Discriminator — fire after the first branch, cancel rest.
    StructuredDiscriminator,
    // Structural patterns (WCP-10 … WCP-13)
    /// WCP-10: Arbitrary Cycles — loops without block-structured nesting.
    ArbitraryCycles,
    /// WCP-11: Implicit Termination — case ends when no work remains.
    ImplicitTermination,
    /// WCP-12: Multiple Instances without Synchronization.
    MultipleInstancesWithoutSync,
    /// WCP-13: Multiple Instances with a Priori Design-Time Knowledge.
    MultipleInstancesWithDesignTimeKnowledge,
    // State-based patterns (WCP-16 … WCP-17)
    /// WCP-16: Deferred Choice — choice resolved by external event, not modeller.
    DeferredChoice,
    /// WCP-17: Interleaved Parallel Routing — activities execute in any order but not concurrently.
    InterleavedParallelRouting,
    // Cancellation and force completion (WCP-19 … WCP-20)
    /// WCP-19: Cancel Activity — withdraw a running activity.
    CancelActivity,
    /// WCP-20: Cancel Case — terminate the entire case instance.
    CancelCase,
}
