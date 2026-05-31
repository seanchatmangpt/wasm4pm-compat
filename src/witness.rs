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
