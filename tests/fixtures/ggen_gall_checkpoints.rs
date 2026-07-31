// First-class Rust projection manufactured from ggen/ontology/standing-law.ttl.
// Change the canonical graph/query/template and regenerate this file together.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GallCheckpointSpec {
    pub rank: u8,
    pub code: &'static str,
    pub name: &'static str,
    pub depends_on: &'static str,
    pub description: &'static str,
}

pub const GALL_CHECKPOINTS: &[GallCheckpointSpec] = &[
    GallCheckpointSpec {
        rank: 1,
        code: "GALL-CP-001",
        name: "OBSERVATION_ADMITTED",
        depends_on: "ROOT",
        description: "All canonical graph, query, template, manifest, and committed projection observations exist and are bounded.",
    },
    GallCheckpointSpec {
        rank: 2,
        code: "GALL-CP-002",
        name: "AUTHORITY_BOUND",
        depends_on: "GALL-CP-001",
        description: "The canonical graph is the declared authority and the ggen standard version is exact.",
    },
    GallCheckpointSpec {
        rank: 3,
        code: "GALL-CP-003",
        name: "ROUTE_DETERMINISTIC",
        depends_on: "GALL-CP-002",
        description: "Every committed-output SPARQL route defines a deterministic total ordering.",
    },
    GallCheckpointSpec {
        rank: 4,
        code: "GALL-CP-004",
        name: "PROJECTION_CLOSED",
        depends_on: "GALL-CP-003",
        description: "The standing projection is complete, contiguous, uniquely named, and descriptively closed.",
    },
    GallCheckpointSpec {
        rank: 5,
        code: "GALL-CP-005",
        name: "REFUSALS_TYPED",
        depends_on: "GALL-CP-004",
        description: "Required refusal codes are explicit, unique, and mechanically addressable.",
    },
    GallCheckpointSpec {
        rank: 6,
        code: "GALL-CP-006",
        name: "ACTUATION_FENCED",
        depends_on: "GALL-CP-005",
        description: "Generation is confined to first-class repository source and contains no external or developer-specific actuation path.",
    },
    GallCheckpointSpec {
        rank: 7,
        code: "GALL-CP-007",
        name: "NEGATIVE_WITNESS",
        depends_on: "GALL-CP-006",
        description: "A controlled projection mutation is distinguishable and maps to a named drift refusal.",
    },
    GallCheckpointSpec {
        rank: 8,
        code: "GALL-CP-008",
        name: "RECEIPT_BOUND",
        depends_on: "GALL-CP-007",
        description: "Every canonical input and committed projection is bound into the machine-readable receipt by BLAKE3 identity.",
    },
    GallCheckpointSpec {
        rank: 9,
        code: "GALL-CP-009",
        name: "REPLAY_EQUIVALENT",
        depends_on: "GALL-CP-008",
        description: "Repeated verification over the same admitted tree produces byte-equivalent identities and checkpoint outcomes.",
    },
    GallCheckpointSpec {
        rank: 10,
        code: "GALL-CP-010",
        name: "CROWN_EXTERNAL",
        depends_on: "GALL-CP-009",
        description: "ALIVE is reachable only through an external exact-tree verifier after all lanes and all prior Gall checkpoints succeed.",
    },
];
