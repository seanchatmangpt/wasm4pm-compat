use wasm4pm_compat::certification::{CertificationFramework, ControlId, ControlMapping};

#[test]
fn control_mapping_without_evidence_is_not_grounded() {
    let mapping = ControlMapping {
        control: ControlId("AC-2"),
        framework: CertificationFramework::FedrampRev5,
        satisfied_by: vec![],
    };
    assert!(!mapping.is_grounded());
}
