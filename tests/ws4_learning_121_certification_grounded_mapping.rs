use wasm4pm_compat::certification::{CertificationFramework, ControlId, ControlMapping};

#[test]
fn control_mapping_with_named_evidence_is_grounded() {
    let mapping = ControlMapping {
        control: ControlId("AC-2"),
        framework: CertificationFramework::FedrampRev5,
        satisfied_by: vec!["receipt:ac-2:42"],
    };
    assert!(mapping.is_grounded());
}
