use wasm4pm_compat::prelude::protocol::*;

fn select_contract() -> CapabilityContract {
    CapabilityContract::new(
        "inspect.subject",
        "https://www.w3.org/ns/prov#Activity",
        "blake3:inspect-semantics",
        "urn:example:InspectInput",
        "urn:example:InspectOutput",
        ConsequenceClass::Select,
        AuthorityMode::None,
        ReceiptPolicy::Optional,
        "inspect.subject",
    )
}

#[test]
fn refused_surface_requires_typed_code_and_remains_distinct_from_unsupported() {
    let capability = select_contract();
    let refused = SurfaceBinding::refused(
        capability.id.clone(),
        SurfaceKind::Mcp,
        capability.semantic_digest.clone(),
        "NO_AUTHORITY",
        "MCP exposure is refused for this capability",
    );
    let unsupported = SurfaceBinding::unsupported(
        capability.id.clone(),
        SurfaceKind::A2a,
        capability.semantic_digest.clone(),
        "implementation has no A2A transport",
    );

    assert!(matches!(
        refused.disposition,
        SurfaceDisposition::Refused { ref code, .. } if code == "NO_AUTHORITY"
    ));
    assert!(matches!(
        unsupported.disposition,
        SurfaceDisposition::Unsupported { .. }
    ));
}

#[test]
fn empty_refusal_code_is_structurally_refused() {
    let capability = select_contract();
    let mut surfaces = PROTOCOL_SURFACES
        .into_iter()
        .map(|surface| {
            SurfaceBinding::projected(
                capability.id.clone(),
                surface,
                capability.semantic_digest.clone(),
                "urn:example:InspectInput",
                "urn:example:InspectOutput",
            )
        })
        .collect::<Vec<_>>();

    let mcp = surfaces
        .iter_mut()
        .find(|binding| binding.surface == SurfaceKind::Mcp)
        .expect("MCP binding exists in the fixed protocol surface set");
    *mcp = SurfaceBinding::refused(
        capability.id.clone(),
        SurfaceKind::Mcp,
        capability.semantic_digest.clone(),
        "",
        "typed refusal requires a code",
    );

    let bundle = ProtocolBundle {
        protocol_id: "example-inspect/1".into(),
        version: "1".into(),
        capabilities: vec![capability],
        surfaces,
    };

    assert!(bundle
        .validate()
        .contains(&ProtocolRefusal::EmptyRefusalCode {
            capability_id: "inspect.subject".into(),
            surface: SurfaceKind::Mcp,
        }));
}

#[test]
fn standing_vocabulary_serializes_without_a_refused_state() {
    let statuses = [
        ProtocolStanding::Unknown,
        ProtocolStanding::PartialAlive,
        ProtocolStanding::Alive,
        ProtocolStanding::Blocked,
        ProtocolStanding::BuildBroken,
        ProtocolStanding::Unsupported,
    ];
    let encoded = serde_json::to_string(&statuses).unwrap();

    assert!(encoded.contains("UNKNOWN"));
    assert!(encoded.contains("PARTIAL_ALIVE"));
    assert!(encoded.contains("BUILD_BROKEN"));
    assert!(encoded.contains("UNSUPPORTED"));
    assert!(!encoded.contains("REFUSED"));
}
