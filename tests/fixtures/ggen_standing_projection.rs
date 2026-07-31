// First-class Rust projection manufactured from ggen/ontology/standing-law.ttl.
// Change the canonical graph/query/template and regenerate this file together.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StandingSpec {
    pub rank: u8,
    pub name: &'static str,
    pub promotable: bool,
    pub terminal: bool,
    pub description: &'static str,
}

pub const GGEN_STANDARD_VERSION: &str = "26.7.31";
pub const AUTHORITY: &str = "canonical_graph";
pub const PROJECTION_MODE: &str = "deterministic_committed";

pub const REQUIRED_OBLIGATIONS: &[&str] = &[
    "positive_execution",
    "negative_refusal",
    "receipt_replay",
];

pub const REFUSAL_CODES: &[&str] = &[
    "GGEN-ACTUATION-001",
    "GGEN-STANDING-001",
    "GGEN-DRIFT-001",
    "GGEN-ADMISSION-001",
];

pub const STANDING: &[StandingSpec] = &[
    StandingSpec {
        rank: 0,
        name: "UNKNOWN",
        promotable: false,
        terminal: false,
        description: "Required execution evidence is absent, stale, incomplete, or bound to another tree.",
    },
    StandingSpec {
        rank: 1,
        name: "UNSUPPORTED",
        promotable: false,
        terminal: true,
        description: "The bounded verifier does not implement the requested surface.",
    },
    StandingSpec {
        rank: 2,
        name: "BLOCKED",
        promotable: false,
        terminal: false,
        description: "A required admission, tool, permission, dependency, or observation is unavailable.",
    },
    StandingSpec {
        rank: 3,
        name: "BUILD_BROKEN",
        promotable: false,
        terminal: false,
        description: "Admitted source reached execution but compilation, tests, lint, or packaging failed.",
    },
    StandingSpec {
        rank: 4,
        name: "PARTIAL_ALIVE",
        promotable: false,
        terminal: false,
        description: "A bounded checkpoint passed but the full crown is incomplete.",
    },
    StandingSpec {
        rank: 5,
        name: "ALIVE",
        promotable: true,
        terminal: true,
        description: "The exact-tree external verifier admitted every positive, negative, and receipt/replay obligation.",
    },
];
