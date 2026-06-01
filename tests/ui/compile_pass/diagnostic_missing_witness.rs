// COMPILE-PASS: CompatDiagnostic::MissingWitness — proves the variant is
// constructible and used as a diagnostic verdict for surfaces that lack a
// named witness.
//
// Law: MissingWitnessLaw — every admitted/projected surface must answer to a
// named Witness; a surface without a witness tag is structurally incomplete.
use wasm4pm_compat::diagnostic::CompatDiagnostic;

fn audit_witness_presence(witness_tagged: bool) -> Vec<CompatDiagnostic> {
    let mut diags = vec![];
    if !witness_tagged {
        diags.push(CompatDiagnostic::MissingWitness);
    }
    diags
}

fn main() {
    let diags = audit_witness_presence(false);
    assert_eq!(diags.len(), 1);
    assert_eq!(diags[0], CompatDiagnostic::MissingWitness);

    let clean = audit_witness_presence(true);
    assert!(clean.is_empty());
}
