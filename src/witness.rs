//! Witness markers — type-level proof carriers naming the canon a value answers to.
//!
//! A *witness* is a zero-sized marker type that names **which standard, paper,
//! API grammar, Rust law, or internal bridge** a piece of evidence is being
//! admitted, projected, or graduated *against*. Witnesses do not carry data and
//! do not run anything: they exist purely to make the boundary law legible at
//! the type level, so an `Admission<T, Ocel20>` cannot be silently mistaken for
//! an `Admission<T, Xes1849>`.
//!
//! ## What witnesses ARE
//!
//! - Compile-time tags that thread a *named authority* through
//!   [`crate::evidence::Evidence`], [`crate::admission::Admission`], and
//!   [`crate::admission::Refusal`].
//! - Carriers of human-facing metadata ([`Witness::KEY`], [`Witness::TITLE`],
//!   [`Witness::YEAR`], [`Witness::FAMILY`]) so a diagnostic can explain *what*
//!   was being checked.
//!
//! ## What witnesses are **NOT**
//!
//! - **Not** validators. A witness names the authority; it never checks
//!   conformance to it. Checking is an engine concern that belongs in `wasm4pm`,
//!   never in this structure-only crate.
//! - **Not** runtime values. They are empty `enum`s — uninhabited and zero-cost.
//!
//! ## Graduation
//!
//! When a surface stops being a *compatibility* surface and needs to actually
//! verify a standard (e.g. truly check OCEL 2.0 object-event link integrity),
//! the witness travels with the value into `wasm4pm`, where a real engine
//! consumes it. Here, it is only a label.

/// The family a [`Witness`] belongs to — what *kind* of authority it names.
///
/// Families let diagnostics and indexes group witnesses by provenance without
/// hard-coding each marker. This is structure only: a family is a label, not a
/// capability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WitnessFamily {
    /// A published interchange/data standard (e.g. OCEL 2.0, XES 1849-2016).
    Standard,
    /// An academic paper defining a model or model family (e.g. POWL, WF-net
    /// soundness, OC-Petri-nets, OCPQ).
    Paper,
    /// An API grammar a consumer must speak to interoperate (e.g. the `pm4py`
    /// call shape, or a `pmax` consumer grammar).
    ApiGrammar,
    /// A Rust-language law this crate enforces structurally (e.g. typestate
    /// admission, `forbid(unsafe_code)`).
    RustLaw,
    /// An internal bridge toward graduation (e.g. the `wasm4pm` engine bridge).
    InternalBridge,
}

/// A type-level proof carrier naming a single standard, paper, grammar, or law.
///
/// Implementors are empty enums (uninhabited, zero-cost). The associated
/// constants are the *only* observable content — they let a [`crate::diagnostic`]
/// surface explain which authority a value was admitted or refused against.
///
/// This trait represents an **authority label**. It does **not** represent the
/// authority's checking logic; this crate is structure-only. A value tagged with
/// a witness should graduate to `wasm4pm` when it needs to be *verified* against
/// that authority rather than merely *named* by it.
///
/// # Examples
///
/// ```
/// use wasm4pm_compat::witness::{Witness, WitnessFamily, Ocel20};
///
/// assert_eq!(Ocel20::KEY, "ocel-2.0");
/// assert_eq!(Ocel20::TITLE, "OCEL 2.0");
/// assert_eq!(Ocel20::YEAR, Some(2023));
/// assert_eq!(Ocel20::FAMILY, WitnessFamily::Standard);
/// ```
pub trait Witness {
    /// A stable, lowercase, machine-facing key (e.g. `"ocel-2.0"`).
    const KEY: &'static str;
    /// The family this witness belongs to.
    const FAMILY: WitnessFamily;
    /// A human-facing title (e.g. `"OCEL 2.0"`).
    const TITLE: &'static str;
    /// The publication year, if the authority has a dated edition.
    const YEAR: Option<u16>;
}

/// Declares an empty-enum witness marker with metadata and a short rustdoc line.
macro_rules! witness_marker {
    ($(#[$meta:meta])* $name:ident, $key:literal, $family:expr, $title:literal, $year:expr) => {
        $(#[$meta])*
        ///
        /// Structure-only authority label; see [`Witness`]. Graduate to
        /// `wasm4pm` when this authority must be *verified*, not merely *named*.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $name {}

        impl Witness for $name {
            const KEY: &'static str = $key;
            const FAMILY: WitnessFamily = $family;
            const TITLE: &'static str = $title;
            const YEAR: Option<u16> = $year;
        }
    };
}

witness_marker!(
    /// OCEL 2.0 — the object-centric event log standard.
    Ocel20, "ocel-2.0", WitnessFamily::Standard, "OCEL 2.0", Some(2023)
);
witness_marker!(
    /// IEEE 1849-2016 (XES) — the eXtensible Event Stream interchange standard.
    Xes1849, "xes-1849-2016", WitnessFamily::Standard, "XES (IEEE 1849-2016)", Some(2016)
);
witness_marker!(
    /// The `pm4py` API call grammar a consumer must speak to interoperate.
    Pm4pyApiGrammar, "pm4py-api-grammar", WitnessFamily::ApiGrammar, "pm4py API grammar", None
);
witness_marker!(
    /// A `pmax`-style consumer grammar a downstream caller must satisfy.
    PmaxConsumerGrammar, "pmax-consumer-grammar", WitnessFamily::ApiGrammar, "pmax consumer grammar", None
);
witness_marker!(
    /// POWL — Partially Ordered Workflow Language (Kourani & van Zelst).
    PowlPaper, "powl-paper", WitnessFamily::Paper, "POWL: Partially Ordered Workflow Language", Some(2023)
);
witness_marker!(
    /// Object-centric Petri nets (van der Aalst & Berti).
    ObjectCentricPetriNetPaper,
    "oc-petri-net-paper",
    WitnessFamily::Paper,
    "Discovering Object-Centric Petri Nets",
    Some(2020)
);
witness_marker!(
    /// WF-net soundness (van der Aalst) — the soundness criterion for workflow nets.
    WfNetSoundnessPaper,
    "wfnet-soundness-paper",
    WitnessFamily::Paper,
    "The Application of Petri Nets to Workflow Management (soundness)",
    Some(1998)
);
witness_marker!(
    /// OCPQ — Object-Centric Process Querying.
    OcpqPaper, "ocpq-paper", WitnessFamily::Paper, "Object-Centric Process Querying", Some(2024)
);
witness_marker!(
    /// The Declare constraint-template family (declarative process modeling).
    DeclareFamily, "declare-family", WitnessFamily::Paper, "Declare constraint family", Some(2007)
);
witness_marker!(
    /// The predictive (business) process monitoring problem family.
    PredictiveMonitoringFamily,
    "predictive-monitoring-family",
    WitnessFamily::Paper,
    "Predictive Process Monitoring family",
    Some(2018)
);
witness_marker!(
    /// Receipt-shaped, provenance-bearing evidence (the receipt family).
    ReceiptFamily, "receipt-family", WitnessFamily::Paper, "Receipt-shaped evidence family", None
);
witness_marker!(
    /// The Rust typestate law: states are tracked at the type level and illegal
    /// transitions are unrepresentable.
    RustTypestateLaw, "rust-typestate-law", WitnessFamily::RustLaw, "Rust typestate law", None
);
witness_marker!(
    /// The internal bridge toward the `wasm4pm` execution engine (graduation).
    Wasm4pmBridge, "wasm4pm-bridge", WitnessFamily::InternalBridge, "wasm4pm graduation bridge", None
);
witness_marker!(
    /// YAWL — Yet Another Workflow Language (van der Aalst & ter Hofstede, 2004).
    ///
    /// Covers typed routing constructs (AND/XOR/OR split/join), cancellation
    /// regions, and multiple-instance tasks. An `Admission<T, YawlPaper>` is
    /// distinguishable at the type level from `Admission<T, WfNetSoundnessPaper>`.
    YawlPaper,
    "yawl-paper",
    WitnessFamily::Paper,
    "YAWL: Yet Another Workflow Language",
    Some(2004)
);
witness_marker!(
    /// Hierarchical Decomposition of Separable WF-nets (Kourani et al., 2026).
    ///
    /// Covers the separable WF-net subclass and the WF-net → POWL 2.0
    /// transformation theorem. Needed to tag admissions against this authority.
    SeparableWfNetPaper,
    "separable-wfnet-paper",
    WitnessFamily::Paper,
    "Hierarchical Decomposition of Separable Workflow-Nets",
    Some(2026)
);
witness_marker!(
    /// Workflow Patterns: The Definitive Guide (Russell, van der Aalst & ter Hofstede, 2016).
    ///
    /// Covers the canonical set of named workflow patterns (WP-1 through WP-43+).
    /// Needed to tag pattern-coverage claims against this authority.
    WorkflowPatternsPaper,
    "workflow-patterns-paper",
    WitnessFamily::Paper,
    "Workflow Patterns: The Definitive Guide",
    Some(2016)
);
witness_marker!(
    /// Inductive Miner (Leemans, Fahland & van der Aalst, 2013).
    ///
    /// Names the Inductive Miner family of process discovery algorithms. An
    /// `Admission<T, InductiveMiner>` is distinguishable from
    /// `Admission<T, AlphaMiner>` — both are discovery algorithms but name
    /// orthogonal authorities. Graduate to `wasm4pm` when the miner must
    /// actually execute.
    InductiveMiner,
    "inductive-miner",
    WitnessFamily::Paper,
    "Inductive Miner (Leemans, Fahland & van der Aalst)",
    Some(2013)
);
witness_marker!(
    /// Declare constraint-template language (Pesic & van der Aalst, 2006).
    ///
    /// Names the individual *constraint* surface of Declare — the set of
    /// constraint templates (Response, Precedence, Chain-Succession, …) and
    /// their LTL semantics. Distinct from [`DeclareFamily`], which names the
    /// broader *model family*. Use `DeclareConstraints` when admitting a
    /// single constraint or a constraint-binding; use `DeclareFamily` when
    /// admitting a whole model.
    DeclareConstraints,
    "declare-constraints",
    WitnessFamily::Paper,
    "Declare constraint-template language",
    Some(2006)
);
witness_marker!(
    /// Alignment-based conformance checking (van Dongen, de Medeiros & Wen, 2008).
    ///
    /// Names the alignment approach to conformance — the authority behind
    /// optimal and heuristic alignments between an event log and a process
    /// model. Distinct from [`WfNetSoundnessPaper`] (structural soundness)
    /// and [`OcpqPaper`] (querying). Graduate to `wasm4pm` when an actual
    /// alignment computation is needed.
    AlignmentPaper,
    "alignment-paper",
    WitnessFamily::Paper,
    "Alignment-Based Conformance Checking",
    Some(2008)
);
witness_marker!(
    /// Object-centric Petri nets — the notation authority.
    ///
    /// Names the OC-Petri-net model class itself (object types, variable arcs,
    /// binding elements), independently of the discovery algorithm described in
    /// the van der Aalst & Berti (2020) paper. Use `OcPetriNets` when tagging
    /// a *model structure*; use [`ObjectCentricPetriNetPaper`] when tagging a
    /// *discovery output* traceable to that paper's algorithm.
    OcPetriNets,
    "oc-petri-nets",
    WitnessFamily::Paper,
    "Object-Centric Petri Nets (notation)",
    Some(2020)
);
witness_marker!(
    /// Log Skeleton (Verbeek & Leemans, 2018).
    ///
    /// Names the Log Skeleton declarative model: a set of six relations
    /// (always-before, always-after, never-together, …) mined directly from
    /// an event log. An `Admission<T, LogSkeleton>` is distinguishable from
    /// `Admission<T, DeclareConstraints>` — both are declarative but name
    /// orthogonal authorities and different relation vocabularies.
    LogSkeleton,
    "log-skeleton",
    WitnessFamily::Paper,
    "Log Skeleton (Verbeek & Leemans)",
    Some(2018)
);
witness_marker!(
    /// Alpha Algorithm (van der Aalst, Weijters & Maruster, 2004).
    ///
    /// Names the Alpha Miner process discovery algorithm — the causal-matrix
    /// based approach producing a WF-net from an event log. Distinct from
    /// [`InductiveMiner`] (different algorithm, different guarantees). An
    /// `Admission<T, AlphaMiner>` is distinguishable at the type level from
    /// any other discovery-algorithm witness.
    AlphaMiner,
    "alpha-miner",
    WitnessFamily::Paper,
    "Alpha Algorithm (van der Aalst, Weijters & Maruster)",
    Some(2004)
);
witness_marker!(
    /// XES lifecycle extension authority (IEEE 1849-2016, §lifecycle).
    ///
    /// Names the authority under which a `lifecycle:transition` value is
    /// admitted or refused against the standard alphabet. Distinct from
    /// [`Xes1849`], which names the overall XES standard: `XesLifecycleExt`
    /// names the sub-authority that governs the `lifecycle:transition` alphabet
    /// specifically. Use this when tagging an admission that checks *only*
    /// lifecycle transition values, not the full XES shape.
    ///
    /// Structure-only authority label; see [`Witness`]. Graduate to `wasm4pm`
    /// when lifecycle-ordering enforcement (start before complete) is required.
    XesLifecycleExt,
    "xes-lifecycle-extension",
    WitnessFamily::Standard,
    "XES lifecycle extension (IEEE 1849-2016)",
    Some(2016)
);
witness_marker!(
    /// XES concept extension authority (IEEE 1849-2016, §concept).
    ///
    /// Names the authority under which a `concept:name` value is admitted or
    /// refused. Distinct from [`Xes1849`] (the overall standard) and
    /// [`XesLifecycleExt`] (lifecycle values). Use when tagging an admission
    /// that checks the `concept:name` key specifically.
    XesConceptExt,
    "xes-concept-extension",
    WitnessFamily::Standard,
    "XES concept extension (IEEE 1849-2016)",
    Some(2016)
);
witness_marker!(
    /// OCEL 2.0 object-type namespace witness.
    ///
    /// Names the authority under which a specific *object type* (e.g. `"order"`,
    /// `"item"`) within an OCEL 2.0 log is admitted or refused. Distinct from
    /// [`Ocel20`], which names the overall standard: `OcelObjectType` names the
    /// sub-authority that governs individual object-type declarations.
    ///
    /// Structure-only authority label; see [`Witness`]. Graduate to `wasm4pm`
    /// when type-level object classification must be *enforced*, not merely named.
    OcelObjectType,
    "ocel-object-type",
    WitnessFamily::Standard,
    "OCEL 2.0 object-type namespace",
    Some(2023)
);
witness_marker!(
    /// OCEL 2.0 event-type (activity) namespace witness.
    ///
    /// Names the authority under which a specific *event type* (activity name,
    /// e.g. `"place_order"`, `"ship"`) within an OCEL 2.0 log is admitted or
    /// refused. Distinct from [`Ocel20`], which names the overall standard:
    /// `OcelEventType` names the sub-authority that governs individual activity
    /// declarations.
    ///
    /// Structure-only authority label; see [`Witness`]. Graduate to `wasm4pm`
    /// when activity-type enforcement is needed.
    OcelEventType,
    "ocel-event-type",
    WitnessFamily::Standard,
    "OCEL 2.0 event-type (activity) namespace",
    Some(2023)
);
witness_marker!(
    /// OCEL 2.0 attribute-type namespace witness.
    ///
    /// Names the authority under which a specific *attribute domain* (e.g.
    /// `"price"`, `"status"`, `"quantity"`) within an OCEL 2.0 object or event
    /// is admitted or refused. Distinct from [`Ocel20`] (the overall standard)
    /// and [`OcelObjectType`] (the object-type namespace): `OcelAttributeType`
    /// governs individual attribute-domain declarations.
    ///
    /// Structure-only authority label; see [`Witness`]. Graduate to `wasm4pm`
    /// when attribute-domain validation is needed.
    OcelAttributeType,
    "ocel-attribute-type",
    WitnessFamily::Standard,
    "OCEL 2.0 attribute-type namespace",
    Some(2023)
);
witness_marker!(
    /// WF-net → POWL 2.0 conversion authority (Kourani, Park & van der Aalst, 2026).
    ///
    /// Names the authority that governs lossless conversion of a *separable*
    /// WF-net into a POWL 2.0 model via the decomposition theorem (Definition 4.1
    /// and Theorem 4.3 of the 2026 paper). An `Admission<T, WfNet2Powl>` asserts
    /// that the admitted value was produced by — or is compatible with — the
    /// Kourani 2026 WF-net→POWL conversion path.
    ///
    /// Distinct from [`SeparableWfNetPaper`], which names the *separability
    /// subclass* authority, and from [`PowlPaper`], which names the POWL language
    /// authority. `WfNet2Powl` names the *conversion* authority: the boundary
    /// where a WF-net *becomes* a POWL model.
    ///
    /// Structure-only authority label; see [`Witness`]. The actual conversion
    /// (decomposition, language-equivalence check) graduates to `wasm4pm`.
    WfNet2Powl,
    "wfnet-to-powl",
    WitnessFamily::Paper,
    "WF-net to POWL 2.0 conversion (Kourani, Park & van der Aalst)",
    Some(2026)
);
witness_marker!(
    /// OC-PM divergence detection authority (paper #49).
    ///
    /// Names the authority under which a *divergence* pattern in an
    /// object-centric process model is admitted or refused. Divergence occurs
    /// when an object type participates in mutually exclusive execution paths
    /// that cannot be merged without information loss.
    ///
    /// Distinct from [`ConvergenceWitness`], which names the *convergence*
    /// authority (paths that reconverge). Use `DivergenceWitness` when tagging
    /// an admission, refusal, or evidence value that specifically concerns the
    /// divergence-detection boundary in OC-PM analysis.
    ///
    /// Paper: OC-PM divergence/convergence detection (#49).
    ///
    /// Structure-only authority label; see [`Witness`]. Graduate to `wasm4pm`
    /// when divergence must be *detected* rather than merely *named*.
    DivergenceWitness,
    "oc-pm-divergence",
    WitnessFamily::Paper,
    "OC-PM divergence detection (paper #49)",
    None
);
witness_marker!(
    /// OC-PM convergence detection authority (paper #49).
    ///
    /// Names the authority under which a *convergence* pattern in an
    /// object-centric process model is admitted or refused. Convergence occurs
    /// when previously divergent execution paths (see [`DivergenceWitness`])
    /// reconverge at a synchronisation point, potentially requiring a join
    /// semantics decision.
    ///
    /// Distinct from [`DivergenceWitness`], which names the *divergence*
    /// authority. Use `ConvergenceWitness` when tagging an admission, refusal,
    /// or evidence value that specifically concerns the convergence-detection
    /// boundary in OC-PM analysis.
    ///
    /// Paper: OC-PM divergence/convergence detection (#49).
    ///
    /// Structure-only authority label; see [`Witness`]. Graduate to `wasm4pm`
    /// when convergence must be *detected* rather than merely *named*.
    ConvergenceWitness,
    "oc-pm-convergence",
    WitnessFamily::Paper,
    "OC-PM convergence detection (paper #49)",
    None
);
