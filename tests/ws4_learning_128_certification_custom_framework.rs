use wasm4pm_compat::certification::{CertificationEnvelope, CertificationFramework, ControlId, ControlMapping};

#[test]
fn custom_framework_mapping_is_admitted_when_identity_matches() {
    let framework = CertificationFramework::Custom("SOC2-2026");
    let envelope = CertificationEnvelope {
        framework,
        mappings: vec![ControlMapping {
            control: ControlId("CC6.1"),
            framework,
            satisfied_by: vec!["receipt:cc6-1"],
        }],
        exclusions: vec![],
    };
    assert_eq!(envelope.validate(), Ok(()));
}
