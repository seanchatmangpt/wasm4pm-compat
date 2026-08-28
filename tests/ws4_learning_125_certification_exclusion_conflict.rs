use wasm4pm_compat::certification::{CertificationEnvelope, CertificationFramework, CertificationRefusal, ControlId, ControlMapping};

#[test]
fn excluded_control_cannot_also_be_claimed_satisfied() {
    let control = ControlId("A.9.2");
    let envelope = CertificationEnvelope {
        framework: CertificationFramework::Iso27001,
        mappings: vec![ControlMapping {
            control,
            framework: CertificationFramework::Iso27001,
            satisfied_by: vec!["receipt:a-9-2"],
        }],
        exclusions: vec![control],
    };
    assert_eq!(
        envelope.validate(),
        Err(vec![CertificationRefusal::ExcludedControlClaimed])
    );
}
