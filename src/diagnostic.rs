//! Compatibility diagnostics — the named laws of a well-formed compat surface.
//!
//! A [`CompatDiagnostic`] names a *structural law* about how evidence crosses
//! this crate's boundary. Each variant is a specific, auditable accusation —
//! "this surface flattened in secret", "this raw value was exported as if
//! admitted" — together with the action that *satisfies* the law. Like
//! [`crate::admission::Refusal`], a diagnostic is never a vague "something is
//! wrong"; it points at the exact missing structure.
//!
//! These diagnostics are the vocabulary a linter, a CI gate, or a graduation
//! reviewer uses to decide whether a compat boundary is honest. They are
//! **structure only**: each names a law and its remedy; none of them runs an
//! engine. When the remedy is "verify it for real", the answer is to graduate
//! the surface to `wasm4pm`.

/// A named law a compatibility surface may violate, and how to satisfy it.
///
/// Use these to explain *why* a boundary is rejected as ill-formed, or as the
/// checklist a surface must clear before it is considered paper-complete in
/// structure.
///
/// Structure-only diagnostic vocabulary. A variant names a deficiency in the
/// *shape/protocol* of a compat surface — not a runtime fault in an engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompatDiagnostic {
    /// **Law:** every admitted/projected surface answers to a named
    /// [`crate::witness::Witness`].
    ///
    /// **Satisfied by:** tagging the evidence with the standard/paper/grammar it
    /// is being judged against, so the boundary's authority is explicit.
    MissingWitness,

    /// **Law:** a round-trip claim (import then export) must be backed by a
    /// fixture proving it actually round-trips.
    ///
    /// **Satisfied by:** adding the import→export→compare fixture for the claim.
    MissingRoundTripFixture,

    /// **Law:** [`crate::state::Raw`] evidence may not leave the crate as if it
    /// were [`crate::state::Admitted`].
    ///
    /// **Satisfied by:** routing the value through an [`crate::admission::Admit`]
    /// impl so it becomes genuinely `Admitted` before export.
    RawEvidenceExportedAsAdmitted,

    /// **Law:** any lossy projection is governed by a [`crate::loss::LossPolicy`].
    ///
    /// **Satisfied by:** implementing the transformation via
    /// [`crate::loss::Project`] under an explicit policy instead of an ad-hoc
    /// conversion.
    LossyProjectionWithoutPolicy,

    /// **Law:** structure must not be discarded silently (no secret flattening).
    ///
    /// **Satisfied by:** emitting a [`crate::loss::LossReport`] that itemizes the
    /// discarded evidence under a named [`crate::loss::ProjectionName`].
    HiddenFlattening,

    /// **Law:** every serious surface offers a refusal path with a *specific*
    /// named reason.
    ///
    /// **Satisfied by:** giving the [`crate::admission::Admit`]/
    /// [`crate::loss::Project`] impl a named `Reason` (never "InvalidInput") and
    /// a code path that returns it.
    MissingRefusalPath,

    /// **Law:** evidence that should be provenance-bearing carries a receipt
    /// shape ([`crate::state::Receipted`]).
    ///
    /// **Satisfied by:** wrapping the admitted value in the receipt envelope so
    /// its provenance and witness travel with it.
    MissingReceiptShape,

    /// **Law:** every shape the crate knows is *reachable* — no canon type is
    /// declared yet wired to nothing.
    ///
    /// **Satisfied by:** connecting the orphaned primitive to an admission,
    /// projection, or export contract (or removing it from the canon).
    UnreachablePrimitive,

    /// **Advisory:** this surface has outgrown compatibility and now needs real
    /// execution semantics.
    ///
    /// **Satisfied by:** graduating the surface to `wasm4pm`, where an engine can
    /// discover/conform/replay rather than merely admit and tag.
    MigrationRecommended,
}

/// Severity level for a compatibility diagnostic.
///
/// `Error` means the surface violates a structural law and must be corrected
/// before the boundary is considered honest. `Warning` means the surface is
/// questionable but not outright wrong. `Info` is advisory only.
///
/// Structure-only; no engine semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiagnosticSeverity {
    /// The surface violates a named structural law; must be corrected.
    Error,
    /// The surface is suspect; correction is strongly recommended.
    Warning,
    /// Advisory notice; no law violation.
    Info,
}

impl core::fmt::Display for DiagnosticSeverity {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            DiagnosticSeverity::Error => f.write_str("Error"),
            DiagnosticSeverity::Warning => f.write_str("Warning"),
            DiagnosticSeverity::Info => f.write_str("Info"),
        }
    }
}

impl core::fmt::Display for CompatDiagnostic {
    /// Formats the diagnostic as `[<severity>] <short description>`.
    ///
    /// The severity is inferred from the variant: `MigrationRecommended` is
    /// `Info`; all other variants represent structural law violations and are
    /// `Error`.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let (severity, message) = match self {
            CompatDiagnostic::MissingWitness => (
                DiagnosticSeverity::Error,
                "missing witness: admitted/projected surface must name its authority",
            ),
            CompatDiagnostic::MissingRoundTripFixture => (
                DiagnosticSeverity::Error,
                "missing round-trip fixture: round-trip claim requires an import→export→compare fixture",
            ),
            CompatDiagnostic::RawEvidenceExportedAsAdmitted => (
                DiagnosticSeverity::Error,
                "raw evidence exported as admitted: route through Admit before export",
            ),
            CompatDiagnostic::LossyProjectionWithoutPolicy => (
                DiagnosticSeverity::Error,
                "lossy projection without policy: use Project under an explicit LossPolicy",
            ),
            CompatDiagnostic::HiddenFlattening => (
                DiagnosticSeverity::Error,
                "hidden flattening: emit a LossReport itemising discarded evidence",
            ),
            CompatDiagnostic::MissingRefusalPath => (
                DiagnosticSeverity::Error,
                "missing refusal path: Admit/Project impl must carry a named Reason type",
            ),
            CompatDiagnostic::MissingReceiptShape => (
                DiagnosticSeverity::Error,
                "missing receipt shape: provenance-bearing evidence must be wrapped in Receipted",
            ),
            CompatDiagnostic::UnreachablePrimitive => (
                DiagnosticSeverity::Error,
                "unreachable primitive: connect or remove the orphaned canon type",
            ),
            CompatDiagnostic::MigrationRecommended => (
                DiagnosticSeverity::Info,
                "migration recommended: surface has outgrown compat — graduate to wasm4pm",
            ),
        };
        write!(f, "[{severity}] {message}")
    }
}
