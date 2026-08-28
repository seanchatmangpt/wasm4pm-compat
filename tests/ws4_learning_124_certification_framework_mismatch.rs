use wasm4pm_compat::certification::{CertificationEnvelope, CertificationFramework, CertificationRefusal, ControlId, ControlMapping};

#[test]
fn mapping_from_another_framework_is_refused() {
    let envelope = CertificationEnvelope {
        framework: CertificationFramework::Iso27001,
        mappings: vec![ControlMapping {
            control: ControlId("AC-2"),
            framework: CertificationFramework::FedrampRev5,
            satisfied_by: vec!["receipt:ac-2"],
        }],
        exclusions: vec![],
    };
    assert_eq!(
        envelope.validate(),
        Err(vec![CertificationRefusal::UnmappedControl])
    );
}
