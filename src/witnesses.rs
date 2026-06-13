










//! Witness marker declarations — compiled from `wasm4pm-compat.ttl`.
//!
//! Every entry here is derived from a `compat:WitnessMarker` instance in the ontology.
//! To add a witness: declare it in the TTL, run `ggen sync`, this file updates.
//! To change a witness: change the TTL, run `ggen sync`.
//! This file and `witness.rs` are the same kind of source — one happens to be compiled
//! from the ontology, one defines the trait. Neither is second-class.

use crate::witness::WitnessFamily;


witness_marker!(
    /// Aggregation view witness — process cube projection to the aggregated (statistical/summary) perspective: summary metrics or aggregate results over a cell.
    AggregationView,
    "process-cube-aggregation-view",
    WitnessFamily::Paper,
    "Process Cube aggregation view",
    Some(2013)
);

witness_marker!(
    /// Alignment-based conformance checking (van Dongen, de Medeiros and Wen, 2008). Optimal and heuristic alignments between event log and process model. Graduate to wasm4pm for actual alignment computation.
    AlignmentPaper,
    "alignment-paper",
    WitnessFamily::Paper,
    "Alignment-Based Conformance Checking",
    Some(2008)
);

witness_marker!(
    /// Alpha Algorithm (van der Aalst, Weijters and Maruster, 2004). Causal-matrix based process discovery producing a WF-net from an event log. Distinct from InductiveMiner.
    AlphaMiner,
    "alpha-miner",
    WitnessFamily::Paper,
    "Alpha Algorithm (van der Aalst, Weijters and Maruster)",
    Some(2004)
);

witness_marker!(
    /// Analytical view witness — process cube projection to the analytical (model-level) perspective: discovered process model for a sub-population.
    AnalyticalView,
    "process-cube-analytical-view",
    WitnessFamily::Paper,
    "Process Cube analytical view",
    Some(2013)
);

witness_marker!(
    /// Causal consistency authority — cross-object causal links are mutually consistent. Per the process-mining Chicago TDD doctrine: if the event log cannot prove a lawful causal process happened, then it did not happen.
    CausalConsistencyWitness,
    "causal-consistency",
    WitnessFamily::Paper,
    "Causal consistency (cross-object causal ordering verified)",
    None
);

witness_marker!(
    /// wasm4pm conformance authority witness. Bridge marker for model-to-log alignment, fitness calculation, and conformance checking.
    ConformanceWitness,
    "wasm4pm-conformance",
    WitnessFamily::InternalBridge,
    "wasm4pm Conformance Authority",
    None
);

witness_marker!(
    /// Control-flow perspective authority (Mannhardt et al., 2016). Covers activity ordering and routing in the balanced multi-perspective conformance framework.
    ControlFlowPerspectiveWitness,
    "cf-perspective",
    WitnessFamily::Paper,
    "Control-Flow Perspective (Mannhardt et al. 2016)",
    Some(2016)
);

witness_marker!(
    /// OC-PM convergence detection authority (paper #49). Convergence: divergent execution paths reconverge at a synchronisation point.
    ConvergenceWitness,
    "oc-pm-convergence",
    WitnessFamily::Paper,
    "OC-PM convergence detection (paper #49)",
    None
);

witness_marker!(
    /// Cross-log correlation authority — events from two or more logs have been correlated under a named schema. Provenance spans multiple logs.
    CrossLogCorrelationWitness,
    "cross-log-correlation",
    WitnessFamily::Paper,
    "Cross-log correlation (multi-log provenance)",
    None
);

witness_marker!(
    /// Data perspective authority (Mannhardt et al., 2016). Covers event and object attribute values in the balanced multi-perspective conformance framework.
    DataPerspectiveWitness,
    "data-perspective",
    WitnessFamily::Paper,
    "Data Perspective (Mannhardt et al. 2016)",
    Some(2016)
);

witness_marker!(
    /// Declare constraint-template language (Pesic and van der Aalst, 2006). Individual constraint surface: Response, Precedence, Chain-Succession templates and LTL semantics. Distinct from DeclareFamily (whole model).
    DeclareConstraints,
    "declare-constraints",
    WitnessFamily::Paper,
    "Declare constraint-template language",
    Some(2006)
);

witness_marker!(
    /// The Declare constraint-template family (declarative process modeling).
    DeclareFamily,
    "declare-family",
    WitnessFamily::Paper,
    "Declare constraint family",
    Some(2007)
);

witness_marker!(
    /// OC-PM divergence detection authority (paper #49). Divergence: object type participates in mutually exclusive execution paths that cannot be merged without information loss.
    DivergenceWitness,
    "oc-pm-divergence",
    WitnessFamily::Paper,
    "OC-PM divergence detection (paper #49)",
    None
);

witness_marker!(
    /// Inductive Miner family of process discovery algorithms (Leemans, Fahland and van der Aalst, 2013). Graduate to wasm4pm when the miner must actually execute.
    InductiveMiner,
    "inductive-miner",
    WitnessFamily::Paper,
    "Inductive Miner (Leemans, Fahland and van der Aalst)",
    Some(2013)
);

witness_marker!(
    /// wasm4pm lifecycle authority witness. Bridge marker for object-centric lifecycle tracking, state transitions, and artifact provenance.
    LifecycleWitness,
    "wasm4pm-lifecycle",
    WitnessFamily::InternalBridge,
    "wasm4pm Lifecycle Authority",
    None
);

witness_marker!(
    /// Log Skeleton declarative model (Verbeek and Leemans, 2018). Six relations: always-before, always-after, never-together, etc. Distinct from DeclareConstraints.
    LogSkeleton,
    "log-skeleton",
    WitnessFamily::Paper,
    "Log Skeleton (Verbeek and Leemans)",
    Some(2018)
);

witness_marker!(
    /// wasm4pm mining authority witness. Bridge marker for process discovery algorithms (inductive, heuristic, alpha mining, variant analysis, trace abstraction).
    MiningWitness,
    "wasm4pm-mining",
    WitnessFamily::InternalBridge,
    "wasm4pm Mining Authority",
    None
);

witness_marker!(
    /// Object-centric Petri nets (van der Aalst and Berti).
    ObjectCentricPetriNetPaper,
    "oc-petri-net-paper",
    WitnessFamily::Paper,
    "Discovering Object-Centric Petri Nets",
    Some(2020)
);

witness_marker!(
    /// Object-centric Petri nets notation authority — model structure (object types, variable arcs, binding elements). Distinct from ObjectCentricPetriNetPaper (discovery algorithm).
    OcPetriNets,
    "oc-petri-nets",
    WitnessFamily::Paper,
    "Object-Centric Petri Nets (notation)",
    Some(2020)
);

witness_marker!(
    /// OCEL 2.0 — the object-centric event log standard.
    Ocel20,
    "ocel-2.0",
    WitnessFamily::Standard,
    "OCEL 2.0",
    Some(2023)
);

witness_marker!(
    /// OCEL 2.0 attribute-type namespace witness. Governs individual attribute-domain declarations. Distinct from Ocel20 and OcelObjectType.
    OcelAttributeType,
    "ocel-attribute-type",
    WitnessFamily::Standard,
    "OCEL 2.0 attribute-type namespace",
    Some(2023)
);

witness_marker!(
    /// OCEL 2.0 event-type (activity) namespace witness. Sub-authority governing individual activity declarations. Distinct from Ocel20.
    OcelEventType,
    "ocel-event-type",
    WitnessFamily::Standard,
    "OCEL 2.0 event-type (activity) namespace",
    Some(2023)
);

witness_marker!(
    /// OCEL 2.0 object-type namespace witness. Sub-authority governing individual object-type declarations. Distinct from Ocel20 (the overall standard).
    OcelObjectType,
    "ocel-object-type",
    WitnessFamily::Standard,
    "OCEL 2.0 object-type namespace",
    Some(2023)
);

witness_marker!(
    /// OCPQ — Object-Centric Process Querying.
    OcpqPaper,
    "ocpq-paper",
    WitnessFamily::Paper,
    "Object-Centric Process Querying",
    Some(2024)
);

witness_marker!(
    /// Operational view witness — process cube projection to the operational (execution-level) perspective: concrete execution traces for a sub-population.
    OperationalView,
    "process-cube-operational-view",
    WitnessFamily::Paper,
    "Process Cube operational view",
    Some(2013)
);

witness_marker!(
    /// The pm4py API call grammar a consumer must speak to interoperate.
    Pm4pyApiGrammar,
    "pm4py-api-grammar",
    WitnessFamily::ApiGrammar,
    "pm4py API grammar",
    None
);

witness_marker!(
    /// A pmax-style consumer grammar a downstream caller must satisfy.
    PmaxConsumerGrammar,
    "pmax-consumer-grammar",
    WitnessFamily::ApiGrammar,
    "pmax consumer grammar",
    None
);

witness_marker!(
    /// POWL — Partially Ordered Workflow Language (Kourani and van Zelst).
    PowlPaper,
    "powl-paper",
    WitnessFamily::Paper,
    "POWL: Partially Ordered Workflow Language",
    Some(2023)
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
    /// Process Cube framework (van der Aalst, 2013). Slicing, dicing, rolling up, drilling down event data for process mining.
    ProcessCubePaper,
    "process-cube-paper",
    WitnessFamily::Paper,
    "Process Cubes (van der Aalst, 2013)",
    Some(2013)
);

witness_marker!(
    /// Receipt-shaped, provenance-bearing evidence (the receipt family).
    ReceiptFamily,
    "receipt-family",
    WitnessFamily::Paper,
    "Receipt-shaped evidence family",
    None
);

witness_marker!(
    /// wasm4pm replay authority witness. Bridge marker for token-based replay, simulation, and path finding.
    ReplayWitness,
    "wasm4pm-replay",
    WitnessFamily::InternalBridge,
    "wasm4pm Replay Authority",
    None
);

witness_marker!(
    /// Resource perspective authority (Mannhardt et al., 2016). Covers who performs each activity (org:resource) in the balanced multi-perspective conformance framework.
    ResourcePerspectiveWitness,
    "resource-perspective",
    WitnessFamily::Paper,
    "Resource Perspective (Mannhardt et al. 2016)",
    Some(2016)
);

witness_marker!(
    /// The Rust typestate law: states are tracked at the type level and illegal transitions are unrepresentable.
    RustTypestateLaw,
    "rust-typestate-law",
    WitnessFamily::RustLaw,
    "Rust typestate law",
    None
);

witness_marker!(
    /// Hierarchical Decomposition of Separable WF-nets (Kourani et al., 2026). Separable WF-net subclass and WF-net to POWL 2.0 transformation theorem.
    SeparableWfNetPaper,
    "separable-wfnet-paper",
    WitnessFamily::Paper,
    "Hierarchical Decomposition of Separable Workflow-Nets",
    Some(2026)
);

witness_marker!(
    /// Streaming evidence authority — evidence collected from a live event stream. Distinct from CausalConsistencyWitness (causal ordering) and CrossLogCorrelationWitness (multi-log provenance).
    StreamingEvidenceWitness,
    "streaming-evidence",
    WitnessFamily::Paper,
    "Streaming evidence (online collection context)",
    None
);

witness_marker!(
    /// Temporal profile authority — full temporal profile (statistical distribution of time distances between activity pairs) computed and attached. Grounded in Stertz, Rinderle-Ma and Rinderle (2020).
    TemporalProfileWitness,
    "temporal-profile-witness",
    WitnessFamily::Paper,
    "Temporal profile (AVG/STD per activity-pair — Stertz et al. 2020)",
    Some(2020)
);

witness_marker!(
    /// Time-aware evidence authority — temporal ordering relations between events have been derived. Distinct from TemporalProfileWitness (full statistical profile computed).
    TimeAwareWitness,
    "time-aware-witness",
    WitnessFamily::Paper,
    "Time-aware evidence (temporal ordering established)",
    Some(2020)
);

witness_marker!(
    /// Time perspective authority (Mannhardt et al., 2016). Covers timestamps, durations, and sojourn times in the balanced multi-perspective conformance framework.
    TimePerspectiveWitness,
    "time-perspective",
    WitnessFamily::Paper,
    "Time Perspective (Mannhardt et al. 2016)",
    Some(2016)
);

witness_marker!(
    /// The internal bridge toward the wasm4pm execution engine (graduation).
    Wasm4pmBridge,
    "wasm4pm-bridge",
    WitnessFamily::InternalBridge,
    "wasm4pm graduation bridge",
    None
);

witness_marker!(
    /// WF-net to POWL 2.0 conversion authority (Kourani, Park and van der Aalst, 2026). Definition 4.1 and Theorem 4.3 of the 2026 paper. Distinct from SeparableWfNetPaper (separability subclass) and PowlPaper (language authority).
    WfNet2Powl,
    "wfnet-to-powl",
    WitnessFamily::Paper,
    "WF-net to POWL 2.0 conversion (Kourani, Park and van der Aalst)",
    Some(2026)
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
    /// Workflow Patterns: The Definitive Guide (Russell, van der Aalst and ter Hofstede, 2016). Canonical set of named workflow patterns WP-1 through WP-43+.
    WorkflowPatternsPaper,
    "workflow-patterns-paper",
    WitnessFamily::Paper,
    "Workflow Patterns: The Definitive Guide",
    Some(2016)
);

witness_marker!(
    /// IEEE 1849-2016 (XES) — the eXtensible Event Stream interchange standard.
    Xes1849,
    "xes-1849-2016",
    WitnessFamily::Standard,
    "XES (IEEE 1849-2016)",
    Some(2016)
);

witness_marker!(
    /// XES concept extension authority (IEEE 1849-2016, concept section). Tags admissions checking concept:name key specifically.
    XesConceptExt,
    "xes-concept-extension",
    WitnessFamily::Standard,
    "XES concept extension (IEEE 1849-2016)",
    Some(2016)
);

witness_marker!(
    /// XES lifecycle extension authority (IEEE 1849-2016, lifecycle section). Tags admissions checking only lifecycle:transition values, not the full XES shape.
    XesLifecycleExt,
    "xes-lifecycle-extension",
    WitnessFamily::Standard,
    "XES lifecycle extension (IEEE 1849-2016)",
    Some(2016)
);

witness_marker!(
    /// YAWL — Yet Another Workflow Language (van der Aalst and ter Hofstede, 2004). Typed routing constructs, cancellation regions, multiple-instance tasks.
    YawlPaper,
    "yawl-paper",
    WitnessFamily::Paper,
    "YAWL: Yet Another Workflow Language",
    Some(2004)
);
