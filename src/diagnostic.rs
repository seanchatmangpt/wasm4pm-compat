//! Compatibility diagnostics — the named laws of a well-formed compat surface.
//!
//! A [`CompatDiagnostic`] names a *structural law* about how evidence crosses
//! this crate's boundary. Each variant is a specific, auditable accusation —
//! "this surface flattened in secret", "this raw value was exported as if
//! admitted" — together with the action that satisfies the law.
//!
//! These diagnostics are the vocabulary a linter, CI gate, doctor, or
//! graduation reviewer uses to decide whether a compat boundary is honest.
//! They are **structure only**: each names a law and its remedy; none runs an
//! engine. When the remedy is "verify it for real", graduate to `wasm4pm`.

/// Deterministic capability doctor, route planner, and repair-plan surface.
pub mod doctor;

pub use doctor::{CompatDoctor, DoctorProfile, DoctorReport, Intent, RoutePlan};

/// A named law a compatibility surface may violate, and how to satisfy it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompatDiagnostic {
    /// Every admitted/projected surface answers to a named witness.
    MissingWitness,
    /// A round-trip claim has no import→export→compare fixture.
    MissingRoundTripFixture,
    /// Raw evidence is leaving the boundary as if it were admitted.
    RawEvidenceExportedAsAdmitted,
    /// A lossy projection has no explicit loss policy.
    LossyProjectionWithoutPolicy,
    /// Structure was flattened without a named loss report.
    HiddenFlattening,
    /// A serious surface has no specific typed refusal path.
    MissingRefusalPath,
    /// Provenance-bearing evidence has no receipt shape.
    MissingReceiptShape,
    /// A canon primitive is declared but connected to no lawful route.
    UnreachablePrimitive,
    /// The surface now requires active execution and should graduate.
    MigrationRecommended,
}

impl CompatDiagnostic {
    /// Complete catalog in stable order.
    pub const ALL: [Self; 9] = [
        Self::MissingWitness,
        Self::MissingRoundTripFixture,
        Self::RawEvidenceExportedAsAdmitted,
        Self::LossyProjectionWithoutPolicy,
        Self::HiddenFlattening,
        Self::MissingRefusalPath,
        Self::MissingReceiptShape,
        Self::UnreachablePrimitive,
        Self::MigrationRecommended,
    ];

    /// Stable machine code suitable for JSON, CI annotations, and support links.
    pub const fn code(self) -> &'static str {
        match self {
            Self::MissingWitness => "W4PM_COMPAT_001",
            Self::MissingRoundTripFixture => "W4PM_COMPAT_002",
            Self::RawEvidenceExportedAsAdmitted => "W4PM_COMPAT_003",
            Self::LossyProjectionWithoutPolicy => "W4PM_COMPAT_004",
            Self::HiddenFlattening => "W4PM_COMPAT_005",
            Self::MissingRefusalPath => "W4PM_COMPAT_006",
            Self::MissingReceiptShape => "W4PM_COMPAT_007",
            Self::UnreachablePrimitive => "W4PM_COMPAT_008",
            Self::MigrationRecommended => "W4PM_COMPAT_009",
        }
    }

    /// Rust variant name, kept stable for source-level search.
    pub const fn name(self) -> &'static str {
        match self {
            Self::MissingWitness => "MissingWitness",
            Self::MissingRoundTripFixture => "MissingRoundTripFixture",
            Self::RawEvidenceExportedAsAdmitted => "RawEvidenceExportedAsAdmitted",
            Self::LossyProjectionWithoutPolicy => "LossyProjectionWithoutPolicy",
            Self::HiddenFlattening => "HiddenFlattening",
            Self::MissingRefusalPath => "MissingRefusalPath",
            Self::MissingReceiptShape => "MissingReceiptShape",
            Self::UnreachablePrimitive => "UnreachablePrimitive",
            Self::MigrationRecommended => "MigrationRecommended",
        }
    }

    /// Severity assigned by the compatibility court.
    pub const fn severity(self) -> DiagnosticSeverity {
        match self {
            Self::MigrationRecommended => DiagnosticSeverity::Info,
            _ => DiagnosticSeverity::Error,
        }
    }

    /// Concise accusation without severity prefix.
    pub const fn message(self) -> &'static str {
        match self {
            Self::MissingWitness => {
                "missing witness: admitted/projected surface must name its authority"
            }
            Self::MissingRoundTripFixture => {
                "missing round-trip fixture: round-trip claim requires an import→export→compare fixture"
            }
            Self::RawEvidenceExportedAsAdmitted => {
                "raw evidence exported as admitted: route through Admit before export"
            }
            Self::LossyProjectionWithoutPolicy => {
                "lossy projection without policy: use Project under an explicit LossPolicy"
            }
            Self::HiddenFlattening => {
                "hidden flattening: emit a LossReport itemising discarded evidence"
            }
            Self::MissingRefusalPath => {
                "missing refusal path: Admit/Project impl must carry a named Reason type"
            }
            Self::MissingReceiptShape => {
                "missing receipt shape: provenance-bearing evidence must be wrapped in Receipted"
            }
            Self::UnreachablePrimitive => {
                "unreachable primitive: connect or remove the orphaned canon type"
            }
            Self::MigrationRecommended => {
                "migration recommended: surface has outgrown compat — graduate to wasm4pm"
            }
        }
    }

    /// Minimal lawful repair for the diagnostic.
    pub const fn repair(self) -> &'static str {
        match self {
            Self::MissingWitness => {
                "attach the standard, paper, or grammar witness that governs admission"
            }
            Self::MissingRoundTripFixture => {
                "add an import→export→compare fixture bound to the round-trip claim"
            }
            Self::RawEvidenceExportedAsAdmitted => {
                "route the value through an Admit implementation before export"
            }
            Self::LossyProjectionWithoutPolicy => {
                "implement Project and require an explicit LossPolicy"
            }
            Self::HiddenFlattening => {
                "emit a named LossReport itemizing every discarded evidence item"
            }
            Self::MissingRefusalPath => {
                "add a specific Reason enum and return a typed refusal for each violated law"
            }
            Self::MissingReceiptShape => {
                "wrap admitted evidence in a receipt envelope carrying witness, digest, and replay hint"
            }
            Self::UnreachablePrimitive => {
                "connect the primitive to admission, projection, export, or remove it from the canon"
            }
            Self::MigrationRecommended => {
                "prepare a GraduationCandidate and execute only in wasm4pm"
            }
        }
    }

    /// Resolve a stable machine code or Rust variant name.
    pub fn from_code(value: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|diagnostic| {
            diagnostic.code().eq_ignore_ascii_case(value)
                || diagnostic.name().eq_ignore_ascii_case(value)
        })
    }
}

/// Severity level for a compatibility diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiagnosticSeverity {
    /// The surface violates a named structural law; it must be corrected.
    Error,
    /// The surface is suspect; correction is strongly recommended.
    Warning,
    /// Advisory notice; no law violation.
    Info,
}

impl core::fmt::Display for DiagnosticSeverity {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Error => f.write_str("Error"),
            Self::Warning => f.write_str("Warning"),
            Self::Info => f.write_str("Info"),
        }
    }
}

impl core::fmt::Display for CompatDiagnostic {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "[{}] {}", self.severity(), self.message())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn catalog_codes_and_names_are_unique() {
        let codes: BTreeSet<_> = CompatDiagnostic::ALL
            .into_iter()
            .map(CompatDiagnostic::code)
            .collect();
        let names: BTreeSet<_> = CompatDiagnostic::ALL
            .into_iter()
            .map(CompatDiagnostic::name)
            .collect();
        assert_eq!(codes.len(), CompatDiagnostic::ALL.len());
        assert_eq!(names.len(), CompatDiagnostic::ALL.len());
    }

    #[test]
    fn every_diagnostic_round_trips_by_code_and_name() {
        for diagnostic in CompatDiagnostic::ALL {
            assert_eq!(
                CompatDiagnostic::from_code(diagnostic.code()),
                Some(diagnostic)
            );
            assert_eq!(
                CompatDiagnostic::from_code(diagnostic.name()),
                Some(diagnostic)
            );
        }
    }

    #[test]
    fn display_preserves_existing_human_shape() {
        assert_eq!(
            CompatDiagnostic::MissingWitness.to_string(),
            "[Error] missing witness: admitted/projected surface must name its authority"
        );
        assert_eq!(
            CompatDiagnostic::MigrationRecommended.to_string(),
            "[Info] migration recommended: surface has outgrown compat — graduate to wasm4pm"
        );
    }
}
