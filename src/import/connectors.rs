//! Structure-only connector descriptors for the canonical external formats.
//!
//! A connector in this crate is a boundary contract, not a live transport. It
//! names the format, direction, and transport shape that a host intends to use.
//! It never opens a file, socket, database, process, or network connection.

use crate::formats::FormatKind;

/// Direction in which a connector may carry process evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ConnectorDirection {
    /// External representation into compat.
    Import,
    /// Compat representation toward an external representation.
    Export,
    /// Both import and export are structurally supported.
    Bidirectional,
}

impl ConnectorDirection {
    /// Whether this declared direction admits `requested`.
    #[must_use]
    pub const fn allows(self, requested: Self) -> bool {
        matches!(
            (self, requested),
            (Self::Bidirectional, _)
                | (Self::Import, Self::Import)
                | (Self::Export, Self::Export)
        )
    }
}

/// Transport shape named by a connector contract.
///
/// These variants describe a host boundary only; selecting one performs no I/O.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ConnectorTransport {
    /// In-memory byte buffer, such as a [`crate::formats::FormatEnvelope`].
    Bytes,
    /// Filesystem path supplied by a host.
    Path,
    /// Standard input/output supplied by a host process.
    Stdio,
    /// Host-owned callback or FFI boundary.
    Host,
}

/// Immutable descriptor for a structure-only format connector.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConnectorSpec {
    /// Stable connector identifier.
    pub id: &'static str,
    /// External format carried by the connector.
    pub format: FormatKind,
    /// Permitted direction.
    pub direction: ConnectorDirection,
    /// Host transport shape.
    pub transport: ConnectorTransport,
    /// Informational media type.
    pub media_type: &'static str,
    /// Conventional extension without a leading dot.
    pub extension: &'static str,
}

impl ConnectorSpec {
    /// Construct a connector descriptor. This is structure-only and performs no I/O.
    #[must_use]
    pub const fn new(
        id: &'static str,
        format: FormatKind,
        direction: ConnectorDirection,
        transport: ConnectorTransport,
        media_type: &'static str,
        extension: &'static str,
    ) -> Self {
        Self {
            id,
            format,
            direction,
            transport,
            media_type,
            extension,
        }
    }
}

/// A zero-cost connector type publishes one immutable descriptor.
pub trait Connector {
    /// Connector contract associated with this marker type.
    const SPEC: ConnectorSpec;
}

/// OCEL 2.0 JSON byte-envelope connector.
pub const OCEL_JSON: ConnectorSpec = ConnectorSpec::new(
    "ocel-json",
    FormatKind::OcelJson,
    ConnectorDirection::Bidirectional,
    ConnectorTransport::Bytes,
    "application/json",
    "json",
);
/// OCEL 2.0 XML byte-envelope connector.
pub const OCEL_XML: ConnectorSpec = ConnectorSpec::new(
    "ocel-xml",
    FormatKind::OcelXml,
    ConnectorDirection::Bidirectional,
    ConnectorTransport::Bytes,
    "application/xml",
    "xml",
);
/// OCEL 2.0 SQLite byte-envelope connector.
pub const OCEL_SQLITE: ConnectorSpec = ConnectorSpec::new(
    "ocel-sqlite",
    FormatKind::OcelSqlite,
    ConnectorDirection::Bidirectional,
    ConnectorTransport::Bytes,
    "application/vnd.sqlite3",
    "sqlite",
);
/// XES XML byte-envelope connector.
pub const XES_XML: ConnectorSpec = ConnectorSpec::new(
    "xes-xml",
    FormatKind::XesXml,
    ConnectorDirection::Bidirectional,
    ConnectorTransport::Bytes,
    "application/xml",
    "xes",
);
/// BPMN 2.0 XML byte-envelope connector.
pub const BPMN_XML: ConnectorSpec = ConnectorSpec::new(
    "bpmn-xml",
    FormatKind::BpmnXml,
    ConnectorDirection::Bidirectional,
    ConnectorTransport::Bytes,
    "application/xml",
    "bpmn",
);
/// PNML byte-envelope connector.
pub const PETRI_PNML: ConnectorSpec = ConnectorSpec::new(
    "petri-pnml",
    FormatKind::PetriPnml,
    ConnectorDirection::Bidirectional,
    ConnectorTransport::Bytes,
    "application/xml",
    "pnml",
);
/// POWL JSON byte-envelope connector.
pub const POWL_JSON: ConnectorSpec = ConnectorSpec::new(
    "powl-json",
    FormatKind::PowlJson,
    ConnectorDirection::Bidirectional,
    ConnectorTransport::Bytes,
    "application/json",
    "json",
);

/// Complete basic connector catalog, one descriptor for every canonical [`FormatKind`].
pub const BASIC_CONNECTORS: &[ConnectorSpec] = &[
    OCEL_JSON,
    OCEL_XML,
    OCEL_SQLITE,
    XES_XML,
    BPMN_XML,
    PETRI_PNML,
    POWL_JSON,
];

/// Find the basic connector for a canonical format.
#[must_use]
pub fn basic_connector(format: FormatKind) -> Option<&'static ConnectorSpec> {
    BASIC_CONNECTORS.iter().find(|spec| spec.format == format)
}

/// Named connector-selection refusals.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ConnectorRefusal {
    /// Selection has no evidence/receipt reference grounding the handoff.
    UngroundedSelection,
    /// The selected connector does not carry the claimed format.
    FormatMismatch {
        /// Format the connector declares.
        expected: FormatKind,
        /// Format claimed by the caller.
        observed: FormatKind,
    },
    /// The connector does not permit the requested direction.
    DirectionMismatch {
        /// Direction the connector supports.
        supported: ConnectorDirection,
        /// Direction requested by the caller.
        requested: ConnectorDirection,
    },
}

impl ConnectorRefusal {
    /// Stable named law suitable for diagnostics and receipts.
    #[must_use]
    pub const fn law(self) -> &'static str {
        match self {
            Self::UngroundedSelection => "UngroundedConnectorSelection",
            Self::FormatMismatch { .. } => "ConnectorFormatMismatch",
            Self::DirectionMismatch { .. } => "ConnectorDirectionMismatch",
        }
    }
}

/// A request to select a connector for a grounded boundary handoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConnectorRequest<'a> {
    /// Selected connector descriptor.
    pub connector: &'static ConnectorSpec,
    /// Format claimed by the boundary request.
    pub claimed_format: FormatKind,
    /// Requested direction.
    pub direction: ConnectorDirection,
    /// Opaque evidence or receipt reference grounding the handoff.
    pub evidence_ref: &'a str,
}

impl<'a> ConnectorRequest<'a> {
    /// Admit the connector selection without performing transport.
    pub fn admit(self) -> Result<AdmittedConnector<'a>, ConnectorRefusal> {
        if self.evidence_ref.trim().is_empty() {
            return Err(ConnectorRefusal::UngroundedSelection);
        }
        if self.connector.format != self.claimed_format {
            return Err(ConnectorRefusal::FormatMismatch {
                expected: self.connector.format,
                observed: self.claimed_format,
            });
        }
        if !self.connector.direction.allows(self.direction) {
            return Err(ConnectorRefusal::DirectionMismatch {
                supported: self.connector.direction,
                requested: self.direction,
            });
        }
        Ok(AdmittedConnector {
            connector: self.connector,
            direction: self.direction,
            evidence_ref: self.evidence_ref,
        })
    }
}

/// Admitted structure-only connector selection.
///
/// This authorizes only the *selection*. It does not authorize or perform I/O.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedConnector<'a> {
    /// Selected connector descriptor.
    pub connector: &'static ConnectorSpec,
    /// Admitted direction.
    pub direction: ConnectorDirection,
    /// Evidence or receipt reference that grounded the selection.
    pub evidence_ref: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_covers_every_canonical_format() {
        let formats = [
            FormatKind::OcelJson,
            FormatKind::OcelXml,
            FormatKind::OcelSqlite,
            FormatKind::XesXml,
            FormatKind::BpmnXml,
            FormatKind::PetriPnml,
            FormatKind::PowlJson,
        ];
        assert_eq!(BASIC_CONNECTORS.len(), formats.len());
        for format in formats {
            assert_eq!(basic_connector(format).map(|spec| spec.format), Some(format));
        }
    }

    #[test]
    fn selection_requires_grounding_and_matching_contract() {
        let request = ConnectorRequest {
            connector: &XES_XML,
            claimed_format: FormatKind::XesXml,
            direction: ConnectorDirection::Import,
            evidence_ref: "blake3:fixture",
        };
        assert!(request.admit().is_ok());

        let ungrounded = ConnectorRequest {
            evidence_ref: "",
            ..request
        };
        assert_eq!(
            ungrounded.admit(),
            Err(ConnectorRefusal::UngroundedSelection)
        );
    }
}
