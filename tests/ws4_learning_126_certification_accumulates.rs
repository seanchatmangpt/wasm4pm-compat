use wasm4pm_compat::certification::{CertificationEnvelope, CertificationFramework, CertificationRefusal, ControlId, ControlMapping};

#[test]
fn one_mapping_can_trigger_all_independent_certification_refusals() {
    let control = ControlId("AC-2");
    let envelope = CertificationEnvelope {
        framework: CertificationFramework::Iso27001,
        mappings: vec![ControlMapping {
            control,
            framework: CertificationFramework::FedrampRev5,
            satisfied_by: vec![],
        }],
        exclusions: vec![control],
    };
    assert_eq!(
        envelope.validate(),
        Err(vec![
            CertificationRefusal::UnmappedControl,
            CertificationRefusal::UngroundedSatisfaction,
            CertificationRefusal::ExcludedControlClaimed,
        ])
    );
}
