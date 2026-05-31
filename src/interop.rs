//! Always-on **adoption grammar** for interoperating with external process-mining
//! ecosystems (PM4Py, PMAx, and friends) — *without* cloning their internals.
//!
//! This module is the smallest, always-available vocabulary a host tool needs to
//! describe what it is *handing across the boundary* and what it is *claiming*
//! about that handoff. It is **structure only**: it carries no PM4Py objects, runs
//! no discovery, no conformance checking, no replay, and no statistics. It merely
//! lets an adopter *name* the shapes it intends to admit, and *refuse* — with a
//! specific named law — when a claim is not grounded.
//!
//! ## What this module **IS**
//!
//! - A set of small, transparent *shape descriptors* ([`Pm4pyShape`],
//!   [`FilterShape`], [`SummaryShape`], [`ConformanceTriple`]) that describe the
//!   *kind* of artifact being adopted, never its bytes or its computed values.
//! - An [`ArtifactGrounding`] that ties a claimed artifact to the evidence that
//!   justifies admitting it.
//! - A first-class refusal enum, [`InteropRefusal`], for when a claim cannot be
//!   grounded.
//!
//! ## What this module is **NOT**
//!
//! - **Not** a PM4Py / PMAx re-implementation. There are no DataFrames, no
//!   PetriNet objects, no alignment matrices here.
//! - **Not** an engine. Nothing here *computes* a summary, a filter result, or a
//!   conformance triple — it only describes their *shape* so the boundary can
//!   reason about admission and refusal.
//!
//! ## Graduation
//!
//! These descriptors should graduate to `wasm4pm` the moment a host needs the
//! values behind them *computed or executed* (an actual filter applied, an actual
//! summary tallied, an actual conformance triple measured). At that point the
//! shape becomes a job, and a job belongs to the engine — see the `wasm4pm`
//! feature and the `graduation` module.

use core::marker::PhantomData;

/// The *kind* of PM4Py-flavoured artifact an adopter is presenting at the boundary.
///
/// This describes **what category of thing** is crossing the boundary so the
/// compat layer can pick the right admission law. It does **not** hold the
/// artifact itself, and it is **structure only** — naming a `PetriNet` here does
/// not bring a Petri-net engine with it.
///
/// Graduate to `wasm4pm` when you need the artifact *materialized and executed*
/// rather than merely *named and admitted*.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Pm4pyShape {
    /// A flat (single-case-notion) event log, à la PM4Py `EventLog`.
    EventLog,
    /// An object-centric event log, à la PM4Py OCEL.
    ObjectCentricLog,
    /// A Petri net model surface (places/transitions), structure only.
    PetriNet,
    /// A process tree model surface, structure only.
    ProcessTree,
    /// A BPMN model surface, structure only.
    Bpmn,
    /// A directly-follows graph, structure only.
    DirectlyFollowsGraph,
    /// A declarative (Declare) constraint surface, structure only.
    Declare,
}

impl Pm4pyShape {
    /// A short, stable, machine-readable tag for this shape.
    ///
    /// ```
    /// use wasm4pm_compat::interop::Pm4pyShape;
    /// assert_eq!(Pm4pyShape::EventLog.tag(), "event_log");
    /// assert_eq!(Pm4pyShape::ObjectCentricLog.tag(), "ocel");
    /// ```
    #[must_use]
    pub const fn tag(self) -> &'static str {
        match self {
            Pm4pyShape::EventLog => "event_log",
            Pm4pyShape::ObjectCentricLog => "ocel",
            Pm4pyShape::PetriNet => "petri_net",
            Pm4pyShape::ProcessTree => "process_tree",
            Pm4pyShape::Bpmn => "bpmn",
            Pm4pyShape::DirectlyFollowsGraph => "dfg",
            Pm4pyShape::Declare => "declare",
        }
    }

    /// Whether this shape is object-centric (multiple object notions) rather than
    /// flat (a single case notion).
    ///
    /// ```
    /// use wasm4pm_compat::interop::Pm4pyShape;
    /// assert!(Pm4pyShape::ObjectCentricLog.is_object_centric());
    /// assert!(!Pm4pyShape::EventLog.is_object_centric());
    /// ```
    #[must_use]
    pub const fn is_object_centric(self) -> bool {
        matches!(self, Pm4pyShape::ObjectCentricLog)
    }
}

/// The *shape* of a log filter an adopter intends to declare at the boundary.
///
/// This names the **dimension** a filter ranges over (activity, time, variant, …)
/// — it carries **no predicate, no threshold, and no filtered result**. It exists
/// so the compat layer can record *that a filter was claimed* without pretending
/// to *apply* one.
///
/// Graduate to `wasm4pm` when the filter must actually *select* events/traces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum FilterShape {
    /// Filter by activity / event name.
    Activity,
    /// Filter by timestamp window.
    Timeframe,
    /// Filter by trace variant.
    Variant,
    /// Filter by attribute presence or value class (no value held here).
    Attribute,
    /// Filter by object type (object-centric).
    ObjectType,
}

/// The *shape* of a log summary an adopter intends to declare.
///
/// Names the **family of statistic** being summarized (counts, variants,
/// durations, …). It holds **no tallies**: a `SummaryShape::TraceVariants` does
/// not contain the variants, only the assertion that variant-summary is the
/// claimed shape.
///
/// Graduate to `wasm4pm` when the summary must actually be *tallied*.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SummaryShape {
    /// Counts of events / traces / objects.
    Counts,
    /// The set of distinct trace variants.
    TraceVariants,
    /// Activity / event-name distribution.
    ActivityDistribution,
    /// Throughput / cycle-time profile.
    TimingProfile,
    /// Object-type distribution (object-centric).
    ObjectTypeDistribution,
}

/// A *shape* describing the three classic conformance quality dimensions, with
/// **no values measured**.
///
/// In full process mining a conformance result reports fitness, precision, and
/// (optionally) generalization. Here we only assert *which dimensions are being
/// claimed* — the booleans say "this dimension is part of the claim", they do
/// **not** carry the measured score. Measuring belongs to the engine.
///
/// Graduate to `wasm4pm` when the actual fitness/precision/generalization numbers
/// must be *computed by replay or alignment*.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConformanceTriple {
    /// The claim includes a fitness dimension.
    pub claims_fitness: bool,
    /// The claim includes a precision dimension.
    pub claims_precision: bool,
    /// The claim includes a generalization dimension.
    pub claims_generalization: bool,
}

impl ConformanceTriple {
    /// A triple that claims fitness and precision but not generalization — the
    /// most common conformance shape.
    ///
    /// ```
    /// use wasm4pm_compat::interop::ConformanceTriple;
    /// let t = ConformanceTriple::fitness_and_precision();
    /// assert!(t.claims_fitness && t.claims_precision);
    /// assert!(!t.claims_generalization);
    /// ```
    #[must_use]
    pub const fn fitness_and_precision() -> Self {
        Self {
            claims_fitness: true,
            claims_precision: true,
            claims_generalization: false,
        }
    }

    /// How many of the three dimensions this triple claims.
    ///
    /// ```
    /// use wasm4pm_compat::interop::ConformanceTriple;
    /// assert_eq!(ConformanceTriple::fitness_and_precision().claimed_count(), 2);
    /// ```
    #[must_use]
    pub const fn claimed_count(self) -> u8 {
        (self.claims_fitness as u8)
            + (self.claims_precision as u8)
            + (self.claims_generalization as u8)
    }

    /// Whether the triple claims at least one dimension. An empty triple is a
    /// vacuous conformance claim and should be refused at admission.
    ///
    /// ```
    /// use wasm4pm_compat::interop::ConformanceTriple;
    /// let empty = ConformanceTriple { claims_fitness: false, claims_precision: false, claims_generalization: false };
    /// assert!(!empty.is_grounded());
    /// ```
    #[must_use]
    pub const fn is_grounded(self) -> bool {
        self.claimed_count() > 0
    }
}

/// Ties a claimed PM4Py-flavoured artifact to the *evidence handle* that justifies
/// admitting it across the boundary.
///
/// `ArtifactGrounding` is the unit of honesty in the interop grammar: an adopter
/// may not merely *assert* "I have a Petri net"; it must point at the evidence
/// (named by `evidence_ref`) that grounds the assertion. The generic `W` is a
/// witness family marker (see [`crate::witness`]) carried at the type level only —
/// `PhantomData`, zero runtime cost.
///
/// It is **structure only**: it never holds the artifact, and grounding it does
/// not validate it. Validation/execution is a `wasm4pm` job.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactGrounding<W> {
    /// The kind of artifact being grounded.
    pub shape: Pm4pyShape,
    /// An opaque reference to the evidence that grounds this artifact (e.g. a
    /// content hash, fixture name, or receipt id). Structure-only: the compat
    /// layer does not dereference it.
    pub evidence_ref: String,
    /// Type-level witness family marker. Zero-cost.
    pub witness: PhantomData<W>,
}

impl<W> ArtifactGrounding<W> {
    /// Ground an artifact `shape` against an opaque `evidence_ref`.
    ///
    /// ```
    /// use wasm4pm_compat::interop::{ArtifactGrounding, Pm4pyShape};
    /// let g = ArtifactGrounding::<()>::new(Pm4pyShape::EventLog, "blake3:abc");
    /// assert_eq!(g.shape, Pm4pyShape::EventLog);
    /// assert!(g.is_grounded());
    /// ```
    #[must_use]
    pub fn new(shape: Pm4pyShape, evidence_ref: impl Into<String>) -> Self {
        Self {
            shape,
            evidence_ref: evidence_ref.into(),
            witness: PhantomData,
        }
    }

    /// Whether the grounding actually references evidence. An empty
    /// `evidence_ref` is an ungrounded claim and must be refused with
    /// [`InteropRefusal::UngroundedArtifact`].
    ///
    /// ```
    /// use wasm4pm_compat::interop::{ArtifactGrounding, Pm4pyShape};
    /// let bad = ArtifactGrounding::<()>::new(Pm4pyShape::PetriNet, "");
    /// assert!(!bad.is_grounded());
    /// ```
    #[must_use]
    pub fn is_grounded(&self) -> bool {
        !self.evidence_ref.trim().is_empty()
    }

    /// Admit this grounding, or refuse with a *specific named law*.
    ///
    /// This is the always-on, structure-only admission gate of the interop
    /// grammar. It does **not** validate the artifact's contents (that is an
    /// engine job); it only enforces that the claim is *grounded* and that an
    /// object-centric shape was not smuggled in under a flat claim.
    ///
    /// ```
    /// use wasm4pm_compat::interop::{ArtifactGrounding, Pm4pyShape, InteropRefusal};
    /// let g = ArtifactGrounding::<()>::new(Pm4pyShape::ObjectCentricLog, "ocel:fixture-1");
    /// assert!(g.admit_flat().is_err()); // OCEL refused under a flat claim
    /// assert!(matches!(g.admit_flat(), Err(InteropRefusal::FlatClaimOverObjectCentric)));
    /// ```
    pub fn admit_flat(&self) -> Result<(), InteropRefusal> {
        if !self.is_grounded() {
            return Err(InteropRefusal::UngroundedArtifact);
        }
        if self.shape.is_object_centric() {
            return Err(InteropRefusal::FlatClaimOverObjectCentric);
        }
        Ok(())
    }
}

/// First-class, *specifically named* refusals for the interop grammar.
///
/// Refusal is never bare here. Each variant names the exact law violated so a
/// host (and a human) can see *why* the boundary said no. These are
/// **structure-only** verdicts: they describe a boundary judgment, they do not
/// remediate it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum InteropRefusal {
    /// A claimed artifact carried no evidence reference to ground it.
    UngroundedArtifact,
    /// A flat admission path was used for an object-centric artifact, which would
    /// silently collapse object notions. Use the object-centric path instead.
    FlatClaimOverObjectCentric,
    /// A conformance claim named *zero* of fitness/precision/generalization and is
    /// therefore vacuous.
    VacuousConformanceClaim,
    /// A summary or filter shape was declared for a dimension the named artifact
    /// shape cannot possibly carry (e.g. `FilterShape::ObjectType` over a flat
    /// `Pm4pyShape::EventLog`).
    DimensionShapeMismatch,
    /// The adopter tried to interpret raw external bytes as a typed artifact
    /// without first admitting them — raw-to-typed laundering is refused.
    UnadmittedRawInterpretation,
}

impl InteropRefusal {
    /// The stable law-name string for this refusal, suitable for diagnostics and
    /// receipts.
    ///
    /// ```
    /// use wasm4pm_compat::interop::InteropRefusal;
    /// assert_eq!(InteropRefusal::UngroundedArtifact.law(), "UngroundedArtifact");
    /// ```
    #[must_use]
    pub const fn law(self) -> &'static str {
        match self {
            InteropRefusal::UngroundedArtifact => "UngroundedArtifact",
            InteropRefusal::FlatClaimOverObjectCentric => "FlatClaimOverObjectCentric",
            InteropRefusal::VacuousConformanceClaim => "VacuousConformanceClaim",
            InteropRefusal::DimensionShapeMismatch => "DimensionShapeMismatch",
            InteropRefusal::UnadmittedRawInterpretation => "UnadmittedRawInterpretation",
        }
    }
}

impl core::fmt::Display for InteropRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "interop refusal: {}", self.law())
    }
}

/// Check that a [`FilterShape`] is meaningful for a given [`Pm4pyShape`].
///
/// Object-centric filter dimensions (`ObjectType`) are refused over flat shapes;
/// everything else is structurally admissible. This is a *shape* check only — it
/// does not validate that any matching events exist.
///
/// ```
/// use wasm4pm_compat::interop::{check_filter_shape, FilterShape, Pm4pyShape, InteropRefusal};
/// assert!(check_filter_shape(Pm4pyShape::EventLog, FilterShape::Activity).is_ok());
/// assert_eq!(
///     check_filter_shape(Pm4pyShape::EventLog, FilterShape::ObjectType),
///     Err(InteropRefusal::DimensionShapeMismatch)
/// );
/// ```
pub fn check_filter_shape(
    artifact: Pm4pyShape,
    filter: FilterShape,
) -> Result<(), InteropRefusal> {
    if matches!(filter, FilterShape::ObjectType) && !artifact.is_object_centric() {
        return Err(InteropRefusal::DimensionShapeMismatch);
    }
    Ok(())
}
