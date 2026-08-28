use wasm4pm_compat::certification::{CertificationEnvelope, CertificationFramework, CertificationRefusal, ControlId, ControlMapping};

#[test]
fn each_ungrounded_mapping_retains_its_own_refusal_evidence() {
    let envelope = CertificationEnvelope {
        framework: CertificationFramework::Iso27001,
        mappings: vec![
            ControlMapping {
                control: ControlId("A.1"),
                framework: CertificationFramework::Iso27001,
                satisfied_by: vec![],
            },
            ControlMapping {
                control: ControlId("A.2"),
                framework: CertificationFramework::Iso27001,
                satisfied_by: vec![],
            },
        ],
        exclusions: vec![],
    };
    assert_eq!(
        envelope.validate(),
        Err(vec![
            CertificationRefusal::UngroundedSatisfaction,
            CertificationRefusal::UngroundedSatisfaction,
        ])
    );
}
