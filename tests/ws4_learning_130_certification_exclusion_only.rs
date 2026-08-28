use wasm4pm_compat::certification::{CertificationEnvelope, CertificationFramework, ControlId};

#[test]
fn exclusion_only_envelope_is_noncontradictory_without_satisfaction_claims() {
    let envelope = CertificationEnvelope {
        framework: CertificationFramework::PciDss,
        mappings: vec![],
        exclusions: vec![ControlId("3.4.1"), ControlId("8.4.2")],
    };
    assert_eq!(envelope.validate(), Ok(()));
}
