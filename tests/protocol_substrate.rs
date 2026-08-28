use wasm4pm_compat::prelude::protocol::*;

fn contract(class: ConsequenceClass) -> CapabilityContract {
    let (authority, receipt) = match class {
        ConsequenceClass::Do => (AuthorityMode::Brokered, ReceiptPolicy::Required),
        ConsequenceClass::Select | ConsequenceClass::Construct => {
            (AuthorityMode::None, ReceiptPolicy::Optional)
        }
    };
    CapabilityContract::new(
        "deploy.application",
        "https://www.w3.org/ns/prov#Activity",
        "blake3:semantic-v1",
        "urn:example:DeployInput",
        "urn:example:DeployOutput",
        class,
        authority,
        receipt,
        "deploy.application",
    )
}

fn all_projected(capability: &CapabilityContract) -> Vec<SurfaceBinding> {
    PROTOCOL_SURFACES
        .into_iter()
        .map(|surface| {
            SurfaceBinding::projected(
                capability.id.clone(),
                surface,
                capability.semantic_digest.clone(),
                "urn:example:DeployInput",
                "urn:example:DeployOutput",
            )
        })
        .collect()
}

fn valid_bundle() -> ProtocolBundle {
    let capability = contract(ConsequenceClass::Do);
    let surfaces = all_projected(&capability);
    ProtocolBundle {
        protocol_id: "example-consequence/1".into(),
        version: "1".into(),
        capabilities: vec![capability],
        surfaces,
    }
}

#[test]
fn complete_bundle_is_structurally_admitted() {
    assert!(valid_bundle().validate().is_empty());
}

#[test]
fn every_transport_must_be_explicit_even_when_unsupported() {
    let capability = contract(ConsequenceClass::Select);
    let mut surfaces = all_projected(&capability);
    surfaces.retain(|binding| binding.surface != SurfaceKind::A2a);
    surfaces.push(SurfaceBinding::unsupported(
        capability.id.clone(),
        SurfaceKind::A2a,
        capability.semantic_digest.clone(),
        "implementation intentionally exposes no A2A transport",
    ));

    let bundle = ProtocolBundle {
        protocol_id: "example-select/1".into(),
        version: "1".into(),
        capabilities: vec![capability],
        surfaces,
    };
    assert!(bundle.validate().is_empty());
}

#[test]
fn missing_transport_binding_is_refused() {
    let mut bundle = valid_bundle();
    bundle
        .surfaces
        .retain(|binding| binding.surface != SurfaceKind::Mcp);

    assert!(bundle
        .validate()
        .contains(&ProtocolRefusal::MissingSurfaceBinding {
            capability_id: "deploy.application".into(),
            surface: SurfaceKind::Mcp,
        }));
}

#[test]
fn transport_projection_cannot_confer_ambient_authority() {
    let mut bundle = valid_bundle();
    let binding = bundle
        .surfaces
        .iter_mut()
        .find(|binding| binding.surface == SurfaceKind::HttpApi)
        .unwrap();
    binding.ambient_authority = true;

    assert!(bundle
        .validate()
        .contains(&ProtocolRefusal::AmbientAuthorityOnSurface {
            capability_id: "deploy.application".into(),
            surface: SurfaceKind::HttpApi,
        }));
}

#[test]
fn semantic_drift_between_surfaces_is_refused() {
    let mut bundle = valid_bundle();
    let binding = bundle
        .surfaces
        .iter_mut()
        .find(|binding| binding.surface == SurfaceKind::Cli)
        .unwrap();
    binding.semantic_digest = "blake3:different-semantics".into();

    assert!(bundle
        .validate()
        .contains(&ProtocolRefusal::ProjectionSemanticDrift {
            capability_id: "deploy.application".into(),
            surface: SurfaceKind::Cli,
        }));
}

#[test]
fn consequential_do_requires_authority_and_receiptability() {
    let invalid = CapabilityContract::new(
        "deploy.application",
        "https://www.w3.org/ns/prov#Activity",
        "blake3:semantic-v1",
        "urn:example:DeployInput",
        "urn:example:DeployOutput",
        ConsequenceClass::Do,
        AuthorityMode::None,
        ReceiptPolicy::Optional,
        "deploy.application",
    );
    let refusals = invalid.validate();

    assert!(refusals.contains(&ProtocolRefusal::DoWithoutAuthority {
        capability_id: "deploy.application".into(),
    }));
    assert!(
        refusals.contains(&ProtocolRefusal::DoWithoutRequiredReceipt {
            capability_id: "deploy.application".into(),
        })
    );
}

#[test]
fn select_and_construct_remain_reversible_type_phases() {
    let subject = SubjectRef::new("service:payments", "blake3:subject");
    let select = Intent::<SelectPhase>::try_new(
        &contract(ConsequenceClass::Select),
        subject.clone(),
        "blake3:input",
    )
    .unwrap();
    let construct = Intent::<ConstructPhase>::try_new(
        &contract(ConsequenceClass::Construct),
        subject,
        "blake3:input",
    )
    .unwrap();

    assert_eq!(select.consequence_class(), ConsequenceClass::Select);
    assert!(select.reversible());
    assert_eq!(construct.consequence_class(), ConsequenceClass::Construct);
    assert!(construct.reversible());
}

#[test]
fn do_envelope_binds_exact_subject_authority_and_receipt_contract() {
    let capability = contract(ConsequenceClass::Do);
    let subject = SubjectRef::new("service:payments", "blake3:subject");
    let authority = AuthorityDecisionRef::new(
        "authority:decision:42",
        capability.id.clone(),
        subject.subject_digest.clone(),
        "blake3:authority-decision",
    );
    let receipt = ReceiptRequirement::new("ce-receipt/1", "blake3", "ce-replay/1");

    let envelope =
        DoEnvelope::try_new(&capability, subject, "blake3:input", authority, receipt).unwrap();

    assert_eq!(envelope.intent().consequence_class(), ConsequenceClass::Do);
    assert!(!envelope.intent().reversible());
    assert_eq!(envelope.authority.capability_id, "deploy.application");
}

#[test]
fn authority_for_another_subject_is_refused_before_runtime() {
    let capability = contract(ConsequenceClass::Do);
    let subject = SubjectRef::new("service:payments", "blake3:subject");
    let authority = AuthorityDecisionRef::new(
        "authority:decision:42",
        capability.id.clone(),
        "blake3:other-subject",
        "blake3:authority-decision",
    );
    let receipt = ReceiptRequirement::new("ce-receipt/1", "blake3", "ce-replay/1");

    let refusals =
        DoEnvelope::try_new(&capability, subject, "blake3:input", authority, receipt).unwrap_err();

    assert!(refusals
        .iter()
        .any(|refusal| matches!(refusal, ProtocolRefusal::AuthoritySubjectMismatch { .. })));
}
