//! PC-POWL2 proof-carrying workflow wire contracts.
//!
//! This module is deliberately **structure only**. It carries the exact objects
//! required to move a proof-carrying POWL 2 certificate across the
//! `wasm4pm-compat -> wasm4pm` graduation boundary:
//!
//! - referenced assertions and variants;
//! - compositional proof terms;
//! - bounded verification parameters;
//! - explicit execution selections;
//! - broker authorization envelopes;
//! - observed receipt shapes;
//! - named refusal laws.
//!
//! It does not execute an action, decide an assertion, compute a digest, mint an
//! authorization, or verify a receipt. Those capabilities belong to `wasm4pm`.

use crate::powl::{Powl, PowlNodeId};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt;

pub const PC_POWL2_SCHEMA: &str = "urn:mfw:pc-powl2:certificate:v1";
pub const PC_POWL2_VERSION: &str = "1.0";

/// Stable reference to an assertion interpreted by an admitted finite domain.
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AssertionRef(pub String);

impl AssertionRef {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn is_well_shaped(&self) -> bool {
        !self.0.trim().is_empty()
    }
}

/// Stable reference to a natural-number ranking function.
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct VariantRef(pub String);

impl VariantRef {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn is_well_shaped(&self) -> bool {
        !self.0.trim().is_empty()
    }
}

/// Explicit bounds defining the admitted finite observation space O*.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerificationBounds {
    pub max_states: usize,
    pub max_proof_depth: usize,
    pub max_selection_depth: usize,
    pub max_trace_steps: usize,
    pub max_choice_visits: usize,
}

impl Default for VerificationBounds {
    fn default() -> Self {
        Self {
            max_states: 4096,
            max_proof_depth: 64,
            max_selection_depth: 64,
            max_trace_steps: 4096,
            max_choice_visits: 4096,
        }
    }
}

impl VerificationBounds {
    pub fn is_well_shaped(&self) -> bool {
        self.max_states > 0
            && self.max_proof_depth > 0
            && self.max_selection_depth > 0
            && self.max_trace_steps > 0
            && self.max_choice_visits > 0
    }
}

/// Claimed standing of the certificate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificateClaim {
    /// Every admitted finite execution preserves the stated contract.
    FiniteTraceSafety,
    /// Safety plus a decreasing variant for every cyclic component.
    TotalCorrectness,
}

/// Exhaustive commutation obligation for two incomparable POWL children.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommutationWitness {
    pub left: PowlNodeId,
    pub right: PowlNodeId,
}

/// A bridge assertion required by one directed choice-graph edge.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EdgeContract {
    pub from: PowlNodeId,
    pub to: PowlNodeId,
}

/// Proof for one executable node inside a choice graph.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphNodeProof {
    pub node: PowlNodeId,
    pub before: AssertionRef,
    pub after: AssertionRef,
    pub proof: Box<ProofTerm>,
}

/// Cycle evidence attached to a choice graph.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum CycleWitness {
    /// The graph is claimed to be acyclic.
    Acyclic,
    /// All finite prefixes preserve the invariant; termination is not claimed.
    Invariant {
        invariant: AssertionRef,
    },
    /// Finite-prefix safety plus a natural-number variant for total correctness.
    Variant {
        invariant: AssertionRef,
        variant: VariantRef,
    },
}

/// Curry-Howard proof-term shape for the executable PC-POWL2 fragment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "rule", rename_all = "snake_case")]
pub enum ProofTerm {
    /// Start, end, or silent boundary preserving one assertion.
    Boundary {
        node: PowlNodeId,
        assertion: AssertionRef,
    },
    /// One atomic action contract.
    Atom {
        node: PowlNodeId,
        pre: AssertionRef,
        post: AssertionRef,
    },
    /// Universal-linearization proof for one partial-order node.
    PartialOrder {
        node: PowlNodeId,
        pre: AssertionRef,
        post: AssertionRef,
        canonical: Vec<PowlNodeId>,
        children: Vec<ProofTerm>,
        commutations: Vec<CommutationWitness>,
    },
    /// Every finite start-to-finish walk is composed from local node and edge contracts.
    ChoiceGraph {
        node: PowlNodeId,
        pre: AssertionRef,
        post: AssertionRef,
        nodes: Vec<GraphNodeProof>,
        edges: Vec<EdgeContract>,
        cycle: CycleWitness,
    },
    /// Hoare consequence rule.
    Consequence {
        node: PowlNodeId,
        pre: AssertionRef,
        post: AssertionRef,
        inner_pre: AssertionRef,
        inner_post: AssertionRef,
        inner: Box<ProofTerm>,
    },
}

impl ProofTerm {
    pub fn node(&self) -> PowlNodeId {
        match self {
            Self::Boundary { node, .. }
            | Self::Atom { node, .. }
            | Self::PartialOrder { node, .. }
            | Self::ChoiceGraph { node, .. }
            | Self::Consequence { node, .. } => *node,
        }
    }

    pub fn contract(&self) -> (&AssertionRef, &AssertionRef) {
        match self {
            Self::Boundary { assertion, .. } => (assertion, assertion),
            Self::Atom { pre, post, .. }
            | Self::PartialOrder { pre, post, .. }
            | Self::ChoiceGraph { pre, post, .. }
            | Self::Consequence { pre, post, .. } => (pre, post),
        }
    }

    pub fn depth(&self) -> usize {
        match self {
            Self::Boundary { .. } | Self::Atom { .. } => 1,
            Self::Consequence { inner, .. } => 1 + inner.depth(),
            Self::PartialOrder { children, .. } => {
                1 + children.iter().map(Self::depth).max().unwrap_or(0)
            }
            Self::ChoiceGraph { nodes, .. } => {
                1 + nodes
                    .iter()
                    .map(|node| node.proof.depth())
                    .max()
                    .unwrap_or(0)
            }
        }
    }

    fn validate_shape(&self) -> Result<(), PcpRefusal> {
        let (pre, post) = self.contract();
        if !pre.is_well_shaped() {
            return Err(PcpRefusal::MissingAssertion {
                role: "pre".to_string(),
                node: self.node(),
            });
        }
        if !post.is_well_shaped() {
            return Err(PcpRefusal::MissingAssertion {
                role: "post".to_string(),
                node: self.node(),
            });
        }

        match self {
            Self::Boundary { .. } | Self::Atom { .. } => Ok(()),
            Self::Consequence {
                inner_pre,
                inner_post,
                inner,
                ..
            } => {
                if !inner_pre.is_well_shaped() {
                    return Err(PcpRefusal::MissingAssertion {
                        role: "inner_pre".to_string(),
                        node: self.node(),
                    });
                }
                if !inner_post.is_well_shaped() {
                    return Err(PcpRefusal::MissingAssertion {
                        role: "inner_post".to_string(),
                        node: self.node(),
                    });
                }
                inner.validate_shape()
            }
            Self::PartialOrder {
                canonical,
                children,
                commutations,
                ..
            } => {
                if children.is_empty() {
                    return Err(PcpRefusal::MissingChildProofs { node: self.node() });
                }
                if canonical.len() != children.len() {
                    return Err(PcpRefusal::CanonicalCoverageMismatch {
                        node: self.node(),
                        canonical: canonical.len(),
                        children: children.len(),
                    });
                }
                let canonical_set: HashSet<_> = canonical.iter().copied().collect();
                if canonical_set.len() != canonical.len() {
                    return Err(PcpRefusal::DuplicateCanonicalNode { node: self.node() });
                }
                let child_set: HashSet<_> = children.iter().map(Self::node).collect();
                if canonical_set != child_set {
                    return Err(PcpRefusal::CanonicalCoverageMismatch {
                        node: self.node(),
                        canonical: canonical_set.len(),
                        children: child_set.len(),
                    });
                }
                for witness in commutations {
                    if witness.left == witness.right {
                        return Err(PcpRefusal::InvalidCommutationWitness {
                            left: witness.left,
                            right: witness.right,
                        });
                    }
                }
                for child in children {
                    child.validate_shape()?;
                }
                Ok(())
            }
            Self::ChoiceGraph {
                nodes,
                cycle,
                ..
            } => {
                if nodes.is_empty() {
                    return Err(PcpRefusal::MissingChildProofs { node: self.node() });
                }
                let mut ids = HashSet::new();
                for graph_node in nodes {
                    if graph_node.node != graph_node.proof.node() {
                        return Err(PcpRefusal::ProofNodeMismatch {
                            expected: graph_node.node,
                            actual: graph_node.proof.node(),
                        });
                    }
                    if !ids.insert(graph_node.node) {
                        return Err(PcpRefusal::DuplicateGraphContract {
                            node: graph_node.node,
                        });
                    }
                    if !graph_node.before.is_well_shaped() || !graph_node.after.is_well_shaped() {
                        return Err(PcpRefusal::MissingAssertion {
                            role: "graph_node".to_string(),
                            node: graph_node.node,
                        });
                    }
                    graph_node.proof.validate_shape()?;
                }
                match cycle {
                    CycleWitness::Acyclic => {}
                    CycleWitness::Invariant { invariant } => {
                        if !invariant.is_well_shaped() {
                            return Err(PcpRefusal::MissingCycleInvariant { node: self.node() });
                        }
                    }
                    CycleWitness::Variant { invariant, variant } => {
                        if !invariant.is_well_shaped() {
                            return Err(PcpRefusal::MissingCycleInvariant { node: self.node() });
                        }
                        if !variant.is_well_shaped() {
                            return Err(PcpRefusal::MissingCycleVariant { node: self.node() });
                        }
                    }
                }
                Ok(())
            }
        }
    }
}

/// Complete structure-only certificate handed to the `wasm4pm` checker.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertifiedPowl {
    pub schema: String,
    pub version: String,
    pub subject: String,
    pub domain_digest: String,
    pub model_digest: String,
    pub proof_digest: String,
    pub claim: CertificateClaim,
    pub bounds: VerificationBounds,
    pub model: Powl,
    pub proof: ProofTerm,
}

impl CertifiedPowl {
    pub fn validate_shape(&self) -> Result<(), PcpRefusal> {
        if self.schema != PC_POWL2_SCHEMA {
            return Err(PcpRefusal::UnsupportedSchema {
                found: self.schema.clone(),
            });
        }
        if self.version != PC_POWL2_VERSION {
            return Err(PcpRefusal::UnsupportedVersion {
                found: self.version.clone(),
            });
        }
        if self.subject.trim().is_empty() {
            return Err(PcpRefusal::MissingSubject);
        }
        for (role, digest) in [
            ("domain", &self.domain_digest),
            ("model", &self.model_digest),
            ("proof", &self.proof_digest),
        ] {
            if digest.trim().is_empty() {
                return Err(PcpRefusal::MissingDigest {
                    role: role.to_string(),
                });
            }
        }
        if !self.bounds.is_well_shaped() {
            return Err(PcpRefusal::InvalidBounds);
        }
        self.model
            .validate()
            .map_err(|reason| PcpRefusal::ModelMalformed {
                reason: format!("{reason:?}"),
            })?;
        let root = self.model.root.ok_or(PcpRefusal::MissingModelRoot)?;
        if self.proof.node() != root {
            return Err(PcpRefusal::ProofNodeMismatch {
                expected: root,
                actual: self.proof.node(),
            });
        }
        let depth = self.proof.depth();
        if depth > self.bounds.max_proof_depth {
            return Err(PcpRefusal::ProofDepthExceeded {
                actual: depth,
                maximum: self.bounds.max_proof_depth,
            });
        }
        self.proof.validate_shape()
    }
}

#[cfg(feature = "wasm4pm")]
impl crate::engine_bridge::GraduateToWasm4pm for CertifiedPowl {
    fn candidate(&self) -> crate::engine_bridge::GraduationCandidate {
        crate::engine_bridge::GraduationCandidate::new(
            crate::engine_bridge::GraduationReason::NeedsConformanceExecution,
            self.subject.clone(),
            self.proof_digest.clone(),
        )
    }
}

/// Explicit witness selecting one concrete execution from the POWL language.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ExecutionSelection {
    Boundary {
        node: PowlNodeId,
    },
    Atom {
        node: PowlNodeId,
    },
    PartialOrder {
        node: PowlNodeId,
        children: Vec<ExecutionSelection>,
    },
    ChoicePath {
        node: PowlNodeId,
        path: Vec<ExecutionSelection>,
    },
}

impl ExecutionSelection {
    pub fn node(&self) -> PowlNodeId {
        match self {
            Self::Boundary { node }
            | Self::Atom { node }
            | Self::PartialOrder { node, .. }
            | Self::ChoicePath { node, .. } => *node,
        }
    }

    pub fn depth(&self) -> usize {
        match self {
            Self::Boundary { .. } | Self::Atom { .. } => 1,
            Self::PartialOrder { children, .. } => {
                1 + children.iter().map(Self::depth).max().unwrap_or(0)
            }
            Self::ChoicePath { path, .. } => {
                1 + path.iter().map(Self::depth).max().unwrap_or(0)
            }
        }
    }
}

/// Single-use authorization minted by the broker authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorizationEnvelope {
    pub authorization_id: String,
    pub subject: String,
    pub domain_digest: String,
    pub model_digest: String,
    pub proof_digest: String,
    pub allowed_nodes: Vec<PowlNodeId>,
    pub challenge_nonce: String,
    pub issued_unix_ms: u64,
    pub expires_unix_ms: u64,
    pub single_use: bool,
}

/// One observed atomic transition in an execution receipt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObservedStep {
    pub ordinal: usize,
    pub node: PowlNodeId,
    pub action: String,
    pub before_digest: String,
    pub after_digest: String,
}

/// Immutable receipt shape for an observed PC-POWL2 execution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionReceiptShape {
    pub receipt_id: String,
    pub predecessor_receipt_digest: Option<String>,
    pub subject: String,
    pub domain_digest: String,
    pub model_digest: String,
    pub proof_digest: String,
    pub authorization_id: String,
    pub challenge_nonce: String,
    pub selection: ExecutionSelection,
    pub initial_state: serde_json::Value,
    pub final_state: serde_json::Value,
    pub initial_state_digest: String,
    pub final_state_digest: String,
    pub observed_steps: Vec<ObservedStep>,
    pub observed_trace_digest: String,
    pub receipt_digest: String,
}

/// Named refusal surface shared by structure admission and engine verification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "law", rename_all = "snake_case")]
#[non_exhaustive]
pub enum PcpRefusal {
    UnsupportedSchema { found: String },
    UnsupportedVersion { found: String },
    MissingSubject,
    MissingDigest { role: String },
    InvalidBounds,
    ModelMalformed { reason: String },
    MissingModelRoot,
    ProofNodeMismatch { expected: PowlNodeId, actual: PowlNodeId },
    ProofDepthExceeded { actual: usize, maximum: usize },
    SelectionDepthExceeded { actual: usize, maximum: usize },
    MissingAssertion { role: String, node: PowlNodeId },
    MissingChildProofs { node: PowlNodeId },
    DuplicateCanonicalNode { node: PowlNodeId },
    CanonicalCoverageMismatch {
        node: PowlNodeId,
        canonical: usize,
        children: usize,
    },
    InvalidCommutationWitness { left: PowlNodeId, right: PowlNodeId },
    DuplicateGraphContract { node: PowlNodeId },
    MissingCycleInvariant { node: PowlNodeId },
    MissingCycleVariant { node: PowlNodeId },
    DomainStateSpaceEmpty,
    DomainStateBoundExceeded { actual: usize, maximum: usize },
    DomainDigestMismatch,
    ModelDigestMismatch,
    ProofDigestMismatch,
    UnknownNode { node: PowlNodeId },
    RuleDoesNotMatchNode { node: PowlNodeId },
    AssertionRefused { assertion: String },
    ActionRefused { node: PowlNodeId, reason: String },
    AtomicContractFailed { node: PowlNodeId },
    ConsequencePreconditionFailed { node: PowlNodeId },
    ConsequencePostconditionFailed { node: PowlNodeId },
    CanonicalOrderInvalid { node: PowlNodeId },
    CanonicalContractFailed { node: PowlNodeId },
    MissingCommutationWitness { left: PowlNodeId, right: PowlNodeId },
    IndependentActionsDoNotCommute { left: PowlNodeId, right: PowlNodeId },
    GraphContractCoverageMismatch { node: PowlNodeId },
    GraphEdgeContractMissing { from: PowlNodeId, to: PowlNodeId },
    GraphEdgeBridgeFailed { from: PowlNodeId, to: PowlNodeId },
    ChoiceGraphCycleContradictsAcyclicWitness { node: PowlNodeId },
    CycleInvariantFailed { node: PowlNodeId },
    CycleTerminationUnproved { node: PowlNodeId },
    CycleVariantDidNotDecrease { from: PowlNodeId, to: PowlNodeId },
    SelectionNotAdmitted { node: PowlNodeId },
    TraceStepBoundExceeded { actual: usize, maximum: usize },
    ChoiceVisitBoundExceeded { actual: usize, maximum: usize },
    AuthorizationMissing,
    AuthorizationExpired,
    AuthorizationAlreadyConsumed,
    AuthorizationDigestMismatch,
    AuthorizationSubjectMismatch,
    AuthorizationNodeDenied { node: PowlNodeId },
    ChallengeNonceMissing,
    InitialEvidenceMissing,
    FinalGoalNotObserved,
    ReceiptSerializationFailed { reason: String },
    ReceiptDigestMismatch,
    ReplayStateMismatch,
    ReplayTraceMismatch,
}

impl fmt::Display for PcpRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for PcpRefusal {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::powl::{PowlBuilder, PowlNodeId};

    #[test]
    fn rejects_unbounded_certificate_shape() {
        let model = PowlBuilder::new()
            .atom("a")
            .root("a")
            .build()
            .expect("valid POWL");
        let certificate = CertifiedPowl {
            schema: PC_POWL2_SCHEMA.to_string(),
            version: PC_POWL2_VERSION.to_string(),
            subject: "toy".to_string(),
            domain_digest: "blake3:domain".to_string(),
            model_digest: "blake3:model".to_string(),
            proof_digest: "blake3:proof".to_string(),
            claim: CertificateClaim::FiniteTraceSafety,
            bounds: VerificationBounds {
                max_states: 0,
                ..VerificationBounds::default()
            },
            model,
            proof: ProofTerm::Atom {
                node: PowlNodeId(0),
                pre: AssertionRef::new("true"),
                post: AssertionRef::new("true"),
            },
        };
        assert_eq!(certificate.validate_shape(), Err(PcpRefusal::InvalidBounds));
    }

    #[test]
    fn admits_atomic_certificate_shape() {
        let model = PowlBuilder::new()
            .atom("a")
            .root("a")
            .build()
            .expect("valid POWL");
        let certificate = CertifiedPowl {
            schema: PC_POWL2_SCHEMA.to_string(),
            version: PC_POWL2_VERSION.to_string(),
            subject: "toy".to_string(),
            domain_digest: "blake3:domain".to_string(),
            model_digest: "blake3:model".to_string(),
            proof_digest: "blake3:proof".to_string(),
            claim: CertificateClaim::FiniteTraceSafety,
            bounds: VerificationBounds::default(),
            model,
            proof: ProofTerm::Atom {
                node: PowlNodeId(0),
                pre: AssertionRef::new("true"),
                post: AssertionRef::new("true"),
            },
        };
        assert!(certificate.validate_shape().is_ok());
    }
}
