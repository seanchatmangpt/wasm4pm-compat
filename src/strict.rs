//! **Strict mode**: an opt-in, build-facing *covenant of judgment* over process
//! boundaries.
//!
//! Strict mode does not *do* anything to your data. It is a **declaration and
//! check surface**: a host *declares* the [`ProcessBoundary`]s it crosses, and
//! strict mode *checks* that each declaration carries the obligations the boundary
//! covenant demands — a witness, a round-trip fixture, a loss policy, a refusal
//! path, conformance fields, a receipt shape — and that no process-mining
//! *capability* has grown in secretly without graduating to `wasm4pm`.
//!
//! ## Why opt-in
//!
//! Most adopters start permissive: import a format, hold a shape, move on. Strict
//! mode is the switch you flip when a boundary becomes *load-bearing* — when other
//! systems will trust your admission and refusal verdicts. At that point the
//! covenant must be *enforced at build/declaration time*, not hoped for.
//!
//! ## What this module is **NOT**
//!
//! - **Not** an engine. [`StrictCheck::check`] inspects *declarations*, never data.
//!   It never replays a log, never measures fitness, never discovers a model.
//! - **Not** a runtime validator of bytes. It validates that the *boundary was
//!   honestly declared*, which is a structural property.
//!
//! ## Graduation
//!
//! [`StrictViolation::HiddenProcessMiningGrowth`] is the tripwire: if a boundary
//! starts *claiming* discovery/conformance/replay capability, strict mode refuses
//! the declaration and points you at the `wasm4pm` graduation bridge. Strict mode's
//! whole job is to keep the compat layer honest about *not* being the engine.

/// The kind of boundary a host declares it crosses.
///
/// Each variant names a distinct way a system can *touch the process world*. The
/// covenant attaches different obligations to different kinds (e.g. anything that
/// `ImportsFormat` owes a round-trip fixture; anything that `ClaimsConformance`
/// owes the conformance fields). It is **structure only** — declaring
/// `EmitsEvents` does not emit any events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ProcessBoundaryKind {
    /// The system emits event records into the process world.
    EmitsEvents,
    /// The system emits object-to-event relations (object-centric edges).
    EmitsObjectRelations,
    /// The system imports an external format across the boundary.
    ImportsFormat,
    /// The system exports to an external format across the boundary.
    ExportsFormat,
    /// The system makes a conformance claim about a log/model pair.
    ClaimsConformance,
    /// The system emits receipt-shaped evidence.
    ClaimsReceipt,
    /// The system claims replay of a model against a log.
    ClaimsReplay,
    /// The system advertises general process-mining support.
    ClaimsProcessMiningSupport,
}

impl ProcessBoundaryKind {
    /// The stable tag for this boundary kind.
    ///
    /// ```
    /// use wasm4pm_compat::strict::ProcessBoundaryKind;
    /// assert_eq!(ProcessBoundaryKind::ImportsFormat.tag(), "imports_format");
    /// ```
    #[must_use]
    pub const fn tag(self) -> &'static str {
        match self {
            ProcessBoundaryKind::EmitsEvents => "emits_events",
            ProcessBoundaryKind::EmitsObjectRelations => "emits_object_relations",
            ProcessBoundaryKind::ImportsFormat => "imports_format",
            ProcessBoundaryKind::ExportsFormat => "exports_format",
            ProcessBoundaryKind::ClaimsConformance => "claims_conformance",
            ProcessBoundaryKind::ClaimsReceipt => "claims_receipt",
            ProcessBoundaryKind::ClaimsReplay => "claims_replay",
            ProcessBoundaryKind::ClaimsProcessMiningSupport => "claims_process_mining_support",
        }
    }
}

/// A declared crossing of the process boundary, with the obligations the host
/// asserts it has met.
///
/// `ProcessBoundary` is the unit a host hands to strict mode. The booleans are
/// *attestations* — "yes, this import carries a witness", "yes, this export has a
/// loss policy" — which [`StrictCheck`] cross-references against the obligations
/// implied by [`ProcessBoundary::kind`]. Lying in an attestation is caught
/// structurally: e.g. an `ImportsFormat` boundary that attests
/// `has_round_trip_fixture = false` fails the check.
///
/// It is **structure only**: it declares a boundary, it does not operate one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessBoundary {
    /// What kind of boundary this is.
    pub kind: ProcessBoundaryKind,
    /// A human-readable name for the boundary (for diagnostics).
    pub name: String,
    /// Attests a type-level witness threads through this boundary.
    pub has_witness: bool,
    /// Attests a named round-trip fixture exists (required for import/export).
    pub has_round_trip_fixture: bool,
    /// Attests a loss policy governs lossy projection (required for export).
    pub has_loss_policy: bool,
    /// Attests a first-class, *named* refusal path exists.
    pub has_refusal_path: bool,
    /// Attests the conformance fields are present (required for conformance claims).
    pub has_conformance_fields: bool,
    /// Attests a receipt shape is present (required for receipt claims).
    pub has_receipt_shape: bool,
    /// Attests no engine-grade process-mining capability has grown here. `true`
    /// means "this boundary is claiming discovery/conformance/replay *execution*",
    /// which strict mode refuses — such capability must graduate to `wasm4pm`.
    pub exports_raw_evidence: bool,
    /// `true` if this boundary has secretly grown engine capability that should
    /// have graduated. Strict mode treats this as a hard refusal.
    pub hidden_pm_growth: bool,
}

impl ProcessBoundary {
    /// A fully-attested boundary of the given kind and name (every obligation
    /// met, nothing hidden). Callers then *relax* attestations to model reality.
    ///
    /// ```
    /// use wasm4pm_compat::strict::{ProcessBoundary, ProcessBoundaryKind, StrictCheck};
    /// let b = ProcessBoundary::fully_attested(ProcessBoundaryKind::ImportsFormat, "ocel-in");
    /// assert!(b.check().is_ok());
    /// ```
    #[must_use]
    pub fn fully_attested(kind: ProcessBoundaryKind, name: impl Into<String>) -> Self {
        Self {
            kind,
            name: name.into(),
            has_witness: true,
            has_round_trip_fixture: true,
            has_loss_policy: true,
            has_refusal_path: true,
            has_conformance_fields: true,
            has_receipt_shape: true,
            exports_raw_evidence: false,
            hidden_pm_growth: false,
        }
    }
}

/// A *specifically named* strict-mode violation.
///
/// Strict mode never fails with a vague error. Each variant pins the exact
/// obligation that was declared-but-unmet (or the exact dishonesty that was
/// detected). These are **structure-only** verdicts about a *declaration*.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum StrictViolation {
    /// A boundary that must thread a witness declared none.
    MissingWitness,
    /// An import/export boundary declared no round-trip fixture.
    MissingRoundTripFixture,
    /// An export boundary declared no loss policy.
    MissingLossPolicy,
    /// A boundary exports raw evidence rather than admitted/typed evidence.
    RawEvidenceExported,
    /// A serious boundary declared no first-class named refusal path.
    MissingRefusalPath,
    /// A conformance-claiming boundary declared no conformance fields.
    MissingConformanceFields,
    /// A receipt-claiming boundary declared no receipt shape.
    MissingReceiptShape,
    /// Engine-grade process-mining capability has grown here without graduating
    /// to `wasm4pm`.
    HiddenProcessMiningGrowth,
}

impl StrictViolation {
    /// The stable law-name for this violation.
    ///
    /// ```
    /// use wasm4pm_compat::strict::StrictViolation;
    /// assert_eq!(StrictViolation::MissingWitness.law(), "MissingWitness");
    /// ```
    #[must_use]
    pub const fn law(self) -> &'static str {
        match self {
            StrictViolation::MissingWitness => "MissingWitness",
            StrictViolation::MissingRoundTripFixture => "MissingRoundTripFixture",
            StrictViolation::MissingLossPolicy => "MissingLossPolicy",
            StrictViolation::RawEvidenceExported => "RawEvidenceExported",
            StrictViolation::MissingRefusalPath => "MissingRefusalPath",
            StrictViolation::MissingConformanceFields => "MissingConformanceFields",
            StrictViolation::MissingReceiptShape => "MissingReceiptShape",
            StrictViolation::HiddenProcessMiningGrowth => "HiddenProcessMiningGrowth",
        }
    }
}

impl core::fmt::Display for StrictViolation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "strict violation: {}", self.law())
    }
}

/// The strict-mode judgment surface: *declare*, then *check*.
///
/// An implementor exposes [`StrictCheck::check`], returning `Ok(())` when its
/// declaration honours the boundary covenant, or `Err(Vec<StrictViolation>)`
/// naming **every** law it broke (strict mode reports all violations at once, not
/// just the first). It is **structure only** — checking inspects declarations,
/// never data.
pub trait StrictCheck {
    /// Check this declaration against the boundary covenant, collecting *all*
    /// named violations.
    ///
    /// ```
    /// use wasm4pm_compat::strict::{ProcessBoundary, ProcessBoundaryKind, StrictCheck, StrictViolation};
    /// let mut b = ProcessBoundary::fully_attested(ProcessBoundaryKind::ExportsFormat, "xes-out");
    /// b.has_loss_policy = false; // export with no loss policy
    /// let violations = b.check().unwrap_err();
    /// assert!(violations.contains(&StrictViolation::MissingLossPolicy));
    /// ```
    fn check(&self) -> Result<(), Vec<StrictViolation>>;
}

impl StrictCheck for ProcessBoundary {
    fn check(&self) -> Result<(), Vec<StrictViolation>> {
        use ProcessBoundaryKind as K;
        let mut v = Vec::new();

        // Hidden growth and raw-evidence export are refused for ANY boundary kind.
        if self.hidden_pm_growth {
            v.push(StrictViolation::HiddenProcessMiningGrowth);
        }
        if self.exports_raw_evidence {
            v.push(StrictViolation::RawEvidenceExported);
        }

        // A witness is owed by every boundary that emits or translates structure.
        let owes_witness = matches!(
            self.kind,
            K::EmitsEvents
                | K::EmitsObjectRelations
                | K::ImportsFormat
                | K::ExportsFormat
                | K::ClaimsReceipt
        );
        if owes_witness && !self.has_witness {
            v.push(StrictViolation::MissingWitness);
        }

        // Import/export owe a round-trip fixture.
        if matches!(self.kind, K::ImportsFormat | K::ExportsFormat) && !self.has_round_trip_fixture
        {
            v.push(StrictViolation::MissingRoundTripFixture);
        }

        // Export owes a loss policy.
        if matches!(self.kind, K::ExportsFormat) && !self.has_loss_policy {
            v.push(StrictViolation::MissingLossPolicy);
        }

        // Conformance claims owe conformance fields.
        if matches!(self.kind, K::ClaimsConformance) && !self.has_conformance_fields {
            v.push(StrictViolation::MissingConformanceFields);
        }

        // Receipt claims owe a receipt shape.
        if matches!(self.kind, K::ClaimsReceipt) && !self.has_receipt_shape {
            v.push(StrictViolation::MissingReceiptShape);
        }

        // Every serious, trust-bearing boundary owes a first-class refusal path.
        let owes_refusal = matches!(
            self.kind,
            K::ImportsFormat
                | K::ExportsFormat
                | K::ClaimsConformance
                | K::ClaimsReceipt
                | K::ClaimsReplay
                | K::ClaimsProcessMiningSupport
        );
        if owes_refusal && !self.has_refusal_path {
            v.push(StrictViolation::MissingRefusalPath);
        }

        // A boundary that claims replay or general PM support is, by definition,
        // claiming engine capability the compat layer must not host.
        if matches!(self.kind, K::ClaimsReplay | K::ClaimsProcessMiningSupport)
            && !self.hidden_pm_growth
        {
            v.push(StrictViolation::HiddenProcessMiningGrowth);
        }

        if v.is_empty() {
            Ok(())
        } else {
            Err(v)
        }
    }
}
