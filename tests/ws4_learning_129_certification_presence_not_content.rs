use wasm4pm_compat::certification::{CertificationFramework, ControlId, ControlMapping};

#[test]
fn grounding_at_compat_layer_is_reference_presence_not_reference_validation() {
    let mapping = ControlMapping {
        control: ControlId("CC6.1"),
        framework: CertificationFramework::Custom("SOC2-2026"),
        satisfied_by: vec![""],
    };
    assert!(mapping.is_grounded());
}
