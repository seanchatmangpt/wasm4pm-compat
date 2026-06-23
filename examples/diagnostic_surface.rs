//! Compatibility diagnostic vocabulary (`src/diagnostic.rs`).
//!
//! This example exercises both pub items in the `diagnostic` module:
//!   - `CompatDiagnostic` enum — 9 named structural law violations + Display
//!   - `DiagnosticSeverity` enum — Error / Warning / Info + Display
//!
//! Invariants proven:
//!   - All 9 CompatDiagnostic variants Display starting with "[Error]" or "[Info]"
//!   - MigrationRecommended is Info; all others are Error
//!   - DiagnosticSeverity Display: "Error" / "Warning" / "Info"
//!   - All variants are Copy, Clone, PartialEq, Hash (stored in sets)

use wasm4pm_compat::diagnostic::{CompatDiagnostic, DiagnosticSeverity};

fn main() {
    // ── DiagnosticSeverity — three levels + Display ───────────────────────────
    println!("== DiagnosticSeverity: three levels ==");

    let sevs = [
        (DiagnosticSeverity::Error, "Error"),
        (DiagnosticSeverity::Warning, "Warning"),
        (DiagnosticSeverity::Info, "Info"),
    ];
    for (sev, expected) in &sevs {
        let displayed = format!("{sev}");
        assert_eq!(&displayed, expected, "DiagnosticSeverity Display mismatch");
        println!("  {:?} -> \"{displayed}\"", sev);
    }

    // Copy + Hash
    let s = DiagnosticSeverity::Error;
    let s2 = s;
    assert_eq!(s, s2, "DiagnosticSeverity is Copy");
    let mut sev_set = std::collections::HashSet::new();
    sev_set.extend([
        DiagnosticSeverity::Error,
        DiagnosticSeverity::Warning,
        DiagnosticSeverity::Info,
    ]);
    assert_eq!(sev_set.len(), 3);
    println!("  All 3 severity levels hash distinctly");

    // ── CompatDiagnostic — 9 variants + Display ───────────────────────────────
    println!("\n== CompatDiagnostic: 9 named law violations ==");

    let diags = [
        CompatDiagnostic::MissingWitness,
        CompatDiagnostic::MissingRoundTripFixture,
        CompatDiagnostic::RawEvidenceExportedAsAdmitted,
        CompatDiagnostic::LossyProjectionWithoutPolicy,
        CompatDiagnostic::HiddenFlattening,
        CompatDiagnostic::MissingRefusalPath,
        CompatDiagnostic::MissingReceiptShape,
        CompatDiagnostic::UnreachablePrimitive,
        CompatDiagnostic::MigrationRecommended,
    ];

    for d in &diags {
        let s = format!("{d}");
        // All diagnostics start with [Error] or [Info]
        assert!(
            s.starts_with("[Error]") || s.starts_with("[Info]"),
            "CompatDiagnostic Display must start with [Error] or [Info]: got {s:?}"
        );
        println!("  {:?}", d);
        println!("    -> {s}");
    }

    // MigrationRecommended is Info; all others are Error
    let migration_display = format!("{}", CompatDiagnostic::MigrationRecommended);
    assert!(
        migration_display.starts_with("[Info]"),
        "MigrationRecommended must be [Info], got: {migration_display}"
    );
    for d in &diags[..8] {
        let s = format!("{d}");
        assert!(
            s.starts_with("[Error]"),
            "structural law violation must be [Error]: {s}"
        );
    }
    println!("\n  MigrationRecommended is [Info]; all 8 others are [Error] ✓");

    // Copy + Hash: all 9 variants stored in a set
    let mut diag_set = std::collections::HashSet::new();
    diag_set.extend(diags.iter().copied());
    assert_eq!(diag_set.len(), 9, "all 9 variants hash distinctly");
    println!("  All 9 variants hash distinctly: {}", diag_set.len());

    // Clone round-trip
    let d = CompatDiagnostic::MissingWitness;
    assert_eq!(d, d.clone(), "CompatDiagnostic Clone == Copy");

    println!("\nEXIT 0");
}
