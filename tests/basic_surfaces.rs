#![cfg(feature = "formats")]

use wasm4pm_compat::formats::FormatKind;
use wasm4pm_compat::import::connectors::{
    basic_connector, Connector, ConnectorDirection, ConnectorRefusal, ConnectorRequest,
    ConnectorSpec, ConnectorTransport, BASIC_CONNECTORS,
};

const FORMATS: [FormatKind; 7] = [
    FormatKind::OcelJson,
    FormatKind::OcelXml,
    FormatKind::OcelSqlite,
    FormatKind::XesXml,
    FormatKind::BpmnXml,
    FormatKind::PetriPnml,
    FormatKind::PowlJson,
];

const DIRECTIONS: [ConnectorDirection; 3] = [
    ConnectorDirection::Import,
    ConnectorDirection::Export,
    ConnectorDirection::Bidirectional,
];

const TRANSPORTS: [ConnectorTransport; 4] = [
    ConnectorTransport::Bytes,
    ConnectorTransport::Path,
    ConnectorTransport::Stdio,
    ConnectorTransport::Host,
];

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

wasm4pm_compat::compat_connector_set!(
    pub const EMPTY_CONNECTORS = [];
);

#[test]
fn basic_catalog_has_one_connector_per_format() {
    assert_eq!(BASIC_CONNECTORS.len(), FORMATS.len());
    for format in FORMATS {
        assert!(basic_connector(format).is_some(), "missing {format:?}");
    }
}

#[test]
fn basic_catalog_identity_is_total_and_unique() {
    for (index, spec) in BASIC_CONNECTORS.iter().enumerate() {
        assert!(!spec.id.is_empty(), "connector id must be named");
        assert!(!spec.media_type.is_empty(), "connector media type must be named");
        assert!(!spec.extension.is_empty(), "connector extension must be named");

        for other in &BASIC_CONNECTORS[index + 1..] {
            assert_ne!(spec.id, other.id, "connector ids must be unique");
        }
    }
}

#[test]
fn dfcm_constructs_full_format_direction_transport_product_space() {
    let mut cells = 0usize;

    for format in FORMATS {
        for direction in DIRECTIONS {
            for transport in TRANSPORTS {
                let spec = ConnectorSpec::new(
                    "dfcm-cell",
                    format,
                    direction,
                    transport,
                    "application/octet-stream",
                    "bin",
                );

                assert_eq!(spec.format, format);
                assert_eq!(spec.direction, direction);
                assert_eq!(spec.transport, transport);
                cells += 1;
            }
        }
    }

    assert_eq!(cells, FORMATS.len() * DIRECTIONS.len() * TRANSPORTS.len());
    assert_eq!(cells, 84);
}

#[test]
fn dfcm_direction_law_covers_every_declared_pair() {
    for supported in DIRECTIONS {
        for requested in DIRECTIONS {
            let expected = match supported {
                ConnectorDirection::Import => requested == ConnectorDirection::Import,
                ConnectorDirection::Export => requested == ConnectorDirection::Export,
                ConnectorDirection::Bidirectional => true,
                _ => false,
            };
            assert_eq!(
                supported.allows(requested),
                expected,
                "direction cell {supported:?} x {requested:?}"
            );
        }
    }
}

#[test]
fn dfcm_admission_covers_complete_basic_format_matrix() {
    let mut cells = 0usize;

    for connector in BASIC_CONNECTORS {
        for claimed_format in FORMATS {
            let verdict = ConnectorRequest {
                connector,
                claimed_format,
                direction: ConnectorDirection::Import,
                evidence_ref: "receipt:dfcm-format-matrix",
            }
            .admit();

            if claimed_format == connector.format {
                assert!(verdict.is_ok(), "matching format must admit");
            } else {
                assert_eq!(
                    verdict,
                    Err(ConnectorRefusal::FormatMismatch {
                        expected: connector.format,
                        observed: claimed_format,
                    })
                );
            }
            cells += 1;
        }
    }

    assert_eq!(cells, BASIC_CONNECTORS.len() * FORMATS.len());
    assert_eq!(cells, 49);
}

#[test]
fn exported_macros_create_external_connector_contracts() {
    assert_eq!(HostXesConnector::SPEC.id, "host-xes");
    assert_eq!(HOST_CONNECTORS, &[HostXesConnector::SPEC]);
    assert!(EMPTY_CONNECTORS.is_empty());

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
