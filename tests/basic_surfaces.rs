#![cfg(feature = "formats")]

use wasm4pm_compat::formats::FormatKind;
use wasm4pm_compat::import::connectors::{
    basic_connector, Connector, ConnectorDirection, ConnectorRequest, ConnectorTransport,
    BASIC_CONNECTORS,
};

wasm4pm_compat::compat_connector!(
    pub HostXesConnector,
    "host-xes",
    FormatKind::XesXml,
    ConnectorDirection::Import,
    ConnectorTransport::Host,
    "application/xml",
    "xes",
);

wasm4pm_compat::compat_connector_set!(
    pub const HOST_CONNECTORS = [HostXesConnector];
);

#[test]
fn basic_catalog_has_one_connector_per_format() {
    let expected = [
        FormatKind::OcelJson,
        FormatKind::OcelXml,
        FormatKind::OcelSqlite,
        FormatKind::XesXml,
        FormatKind::BpmnXml,
        FormatKind::PetriPnml,
        FormatKind::PowlJson,
    ];

    assert_eq!(BASIC_CONNECTORS.len(), expected.len());
    for format in expected {
        assert!(basic_connector(format).is_some(), "missing {format:?}");
    }
}

#[test]
fn exported_macros_create_external_connector_contracts() {
    assert_eq!(HostXesConnector::SPEC.id, "host-xes");
    assert_eq!(HOST_CONNECTORS, &[HostXesConnector::SPEC]);

    let admitted = ConnectorRequest {
        connector: &HostXesConnector::SPEC,
        claimed_format: FormatKind::XesXml,
        direction: ConnectorDirection::Import,
        evidence_ref: "receipt:host-xes-fixture",
    }
    .admit()
    .expect("grounded matching selection should admit");

    assert_eq!(admitted.connector.id, "host-xes");
}

#[test]
fn macro_connector_preserves_direction_fence() {
    let refusal = ConnectorRequest {
        connector: &HostXesConnector::SPEC,
        claimed_format: FormatKind::XesXml,
        direction: ConnectorDirection::Export,
        evidence_ref: "receipt:host-xes-fixture",
    }
    .admit()
    .expect_err("import-only connector must refuse export");

    assert_eq!(refusal.law(), "ConnectorDirectionMismatch");
}
