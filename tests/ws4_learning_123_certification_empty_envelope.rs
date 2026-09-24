use wasm4pm_compat::certification::{CertificationEnvelope, CertificationFramework};

#[test]
fn empty_certification_envelope_makes_no_false_satisfaction_claim() {
    let envelope = CertificationEnvelope {
        framework: CertificationFramework::Iso27001,
        mappings: vec![],
        exclusions: vec![],
    };
    assert_eq!(envelope.validate(), Ok(()));
}
