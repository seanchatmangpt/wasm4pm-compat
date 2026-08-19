//! Federated protocol substrate — structure only.
//!
//! These types encode the smallest portable law needed for independently
//! implemented systems to interoperate without inheriting ambient authority:
//!
//! `public semantics -> capability contract -> surface projection ->
//! SELECT | CONSTRUCT | DO -> authority decision -> receipt -> replay`.
//!
//! This module deliberately contains no transport server, planner, broker,
//! actuator, hash implementation, receipt verifier, or OCEL writer. It names
//! the contracts that an execution engine such as `wasm4pm` may implement.

use core::marker::PhantomData;
use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// The four common protocol projections. A capability may support or refuse any
/// one of them, but every surface must be represented explicitly.
pub const PROTOCOL_SURFACES: [SurfaceKind; 4] = [
    SurfaceKind::Cli,
    SurfaceKind::HttpApi,
    SurfaceKind::Mcp,
    SurfaceKind::A2a,
];

/// Consequence phase. The type law keeps selection, construction, and
/// consequential action distinct.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ConsequenceClass {
    Select,
    Construct,
    Do,
}

/// Evidence standing for one exact subject.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProtocolStanding {
    Unknown,
    PartialAlive,
    Alive,
    Blocked,
    BuildBroken,
    Unsupported,
    Refused,
}

/// A transport projection of one semantic capability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceKind {
    Cli,
    HttpApi,
    Mcp,
    A2a,
}

/// Where consequential authority must come from.
///
/// `ExternalDecision` and `Brokered` describe required authority shapes; they
/// do not assert that a particular caller actually possesses authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityMode {
    None,
    ExternalDecision,
    Brokered,
}

/// Whether a consequence path is structurally required to emit a receipt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReceiptPolicy {
    Optional,
    Required,
}

/// Public event-wire family. OCEL 2 is the initial neutral event shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum EventWireFormat {
    Ocel2,
}

/// Portable semantic contract for one capability.
///
/// `public_semantic_iri` is mandatory. A custom semantic IRI is optional and is
/// therefore the explicit remainder rather than the primary meaning carrier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityContract {
    pub id: String,
    pub public_semantic_iri: String,
    pub custom_semantic_iri: Option<String>,
    pub semantic_digest: String,
    pub input_type_iri: String,
    pub output_type_iri: String,
    pub consequence: ConsequenceClass,
    pub authority_mode: AuthorityMode,
    pub receipt_policy: ReceiptPolicy,
    pub event_wire: EventWireFormat,
    pub ocel_event_type: String,
}

impl CapabilityContract {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        public_semantic_iri: impl Into<String>,
        semantic_digest: impl Into<String>,
        input_type_iri: impl Into<String>,
        output_type_iri: impl Into<String>,
        consequence: ConsequenceClass,
        authority_mode: AuthorityMode,
        receipt_policy: ReceiptPolicy,
        ocel_event_type: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            public_semantic_iri: public_semantic_iri.into(),
            custom_semantic_iri: None,
            semantic_digest: semantic_digest.into(),
            input_type_iri: input_type_iri.into(),
            output_type_iri: output_type_iri.into(),
            consequence,
            authority_mode,
            receipt_policy,
            event_wire: EventWireFormat::Ocel2,
            ocel_event_type: ocel_event_type.into(),
        }
    }

    #[must_use]
    pub fn with_custom_semantic_iri(mut self, iri: impl Into<String>) -> Self {
        self.custom_semantic_iri = Some(iri.into());
        self
    }

    /// Structural validation only. This never evaluates runtime authority.
    #[must_use]
    pub fn validate(&self) -> Vec<ProtocolRefusal> {
        let mut refusals = Vec::new();
        if self.id.trim().is_empty() {
            refusals.push(ProtocolRefusal::EmptyCapabilityId);
        }
        if self.public_semantic_iri.trim().is_empty() {
            refusals.push(ProtocolRefusal::MissingPublicSemantic {
                capability_id: self.id.clone(),
            });
        }
        if self.semantic_digest.trim().is_empty() {
            refusals.push(ProtocolRefusal::MissingSemanticDigest {
                capability_id: self.id.clone(),
            });
        }
        if self.input_type_iri.trim().is_empty() {
            refusals.push(ProtocolRefusal::MissingInputType {
                capability_id: self.id.clone(),
            });
        }
        if self.output_type_iri.trim().is_empty() {
            refusals.push(ProtocolRefusal::MissingOutputType {
                capability_id: self.id.clone(),
            });
        }
        if self.ocel_event_type.trim().is_empty() {
            refusals.push(ProtocolRefusal::MissingOcelEventType {
                capability_id: self.id.clone(),
            });
        }
        if self.consequence == ConsequenceClass::Do && self.authority_mode == AuthorityMode::None {
            refusals.push(ProtocolRefusal::DoWithoutAuthority {
                capability_id: self.id.clone(),
            });
        }
        if self.consequence == ConsequenceClass::Do
            && self.receipt_policy != ReceiptPolicy::Required
        {
            refusals.push(ProtocolRefusal::DoWithoutRequiredReceipt {
                capability_id: self.id.clone(),
            });
        }
        refusals
    }
}

/// Whether a transport surface projects, explicitly does not support, or
/// refuses one capability.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "disposition", rename_all = "snake_case")]
pub enum SurfaceDisposition {
    Projected {
        input_schema: String,
        output_schema: String,
    },
    Unsupported {
        reason: String,
    },
    Refused {
        reason: String,
    },
}

/// One transport binding for one capability.
///
/// `ambient_authority` exists as an adversarial field: a conforming bundle must
/// set it to false. A CLI/API/MCP/A2A projection can expose a capability but
/// cannot grant authority merely by exposing it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceBinding {
    pub capability_id: String,
    pub surface: SurfaceKind,
    pub semantic_digest: String,
    pub disposition: SurfaceDisposition,
    pub ambient_authority: bool,
}

impl SurfaceBinding {
    #[must_use]
    pub fn projected(
        capability_id: impl Into<String>,
        surface: SurfaceKind,
        semantic_digest: impl Into<String>,
        input_schema: impl Into<String>,
        output_schema: impl Into<String>,
    ) -> Self {
        Self {
            capability_id: capability_id.into(),
            surface,
            semantic_digest: semantic_digest.into(),
            disposition: SurfaceDisposition::Projected {
                input_schema: input_schema.into(),
                output_schema: output_schema.into(),
            },
            ambient_authority: false,
        }
    }

    #[must_use]
    pub fn unsupported(
        capability_id: impl Into<String>,
        surface: SurfaceKind,
        semantic_digest: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            capability_id: capability_id.into(),
            surface,
            semantic_digest: semantic_digest.into(),
            disposition: SurfaceDisposition::Unsupported {
                reason: reason.into(),
            },
            ambient_authority: false,
        }
    }

    #[must_use]
    pub fn refused(
        capability_id: impl Into<String>,
        surface: SurfaceKind,
        semantic_digest: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            capability_id: capability_id.into(),
            surface,
            semantic_digest: semantic_digest.into(),
            disposition: SurfaceDisposition::Refused {
                reason: reason.into(),
            },
            ambient_authority: false,
        }
    }
}

/// Dependency-closed portable protocol bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProtocolBundle {
    pub protocol_id: String,
    pub version: String,
    pub capabilities: Vec<CapabilityContract>,
    pub surfaces: Vec<SurfaceBinding>,
}

impl ProtocolBundle {
    #[must_use]
    pub fn validate(&self) -> Vec<ProtocolRefusal> {
        let mut refusals = Vec::new();
        if self.protocol_id.trim().is_empty() {
            refusals.push(ProtocolRefusal::EmptyProtocolId);
        }
        if self.version.trim().is_empty() {
            refusals.push(ProtocolRefusal::EmptyProtocolVersion);
        }

        let mut capability_ids = BTreeSet::new();
        for capability in &self.capabilities {
            refusals.extend(capability.validate());
            if !capability_ids.insert(capability.id.clone()) {
                refusals.push(ProtocolRefusal::DuplicateCapabilityId {
                    capability_id: capability.id.clone(),
                });
            }
        }

        let mut bindings = BTreeSet::new();
        for binding in &self.surfaces {
            let key = (binding.capability_id.clone(), binding.surface);
            if !bindings.insert(key) {
                refusals.push(ProtocolRefusal::DuplicateSurfaceBinding {
                    capability_id: binding.capability_id.clone(),
                    surface: binding.surface,
                });
            }

            let Some(capability) = self
                .capabilities
                .iter()
                .find(|candidate| candidate.id == binding.capability_id)
            else {
                refusals.push(ProtocolRefusal::UnknownCapabilityProjection {
                    capability_id: binding.capability_id.clone(),
                    surface: binding.surface,
                });
                continue;
            };

            if binding.ambient_authority {
                refusals.push(ProtocolRefusal::AmbientAuthorityOnSurface {
                    capability_id: binding.capability_id.clone(),
                    surface: binding.surface,
                });
            }
            if binding.semantic_digest != capability.semantic_digest {
                refusals.push(ProtocolRefusal::ProjectionSemanticDrift {
                    capability_id: binding.capability_id.clone(),
                    surface: binding.surface,
                });
            }
            match &binding.disposition {
                SurfaceDisposition::Projected {
                    input_schema,
                    output_schema,
                } if input_schema.trim().is_empty() || output_schema.trim().is_empty() => {
                    refusals.push(ProtocolRefusal::EmptySurfaceSchema {
                        capability_id: binding.capability_id.clone(),
                        surface: binding.surface,
                    });
                }
                SurfaceDisposition::Unsupported { reason }
                | SurfaceDisposition::Refused { reason }
                    if reason.trim().is_empty() =>
                {
                    refusals.push(ProtocolRefusal::EmptyDispositionReason {
                        capability_id: binding.capability_id.clone(),
                        surface: binding.surface,
                    });
                }
                _ => {}
            }
        }

        for capability in &self.capabilities {
            for surface in PROTOCOL_SURFACES {
                if !bindings.contains(&(capability.id.clone(), surface)) {
                    refusals.push(ProtocolRefusal::MissingSurfaceBinding {
                        capability_id: capability.id.clone(),
                        surface,
                    });
                }
            }
        }

        refusals
    }
}

/// Exact subject to which a consequence applies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubjectRef {
    pub subject_id: String,
    pub subject_digest: String,
}

impl SubjectRef {
    #[must_use]
    pub fn new(subject_id: impl Into<String>, subject_digest: impl Into<String>) -> Self {
        Self {
            subject_id: subject_id.into(),
            subject_digest: subject_digest.into(),
        }
    }
}

/// Opaque reference to an authority decision made outside compat.
///
/// Carrying this value does not prove or grant authority. The runtime must
/// independently verify it before consequential DO.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorityDecisionRef {
    pub authority_id: String,
    pub capability_id: String,
    pub subject_digest: String,
    pub decision_digest: String,
}

impl AuthorityDecisionRef {
    #[must_use]
    pub fn new(
        authority_id: impl Into<String>,
        capability_id: impl Into<String>,
        subject_digest: impl Into<String>,
        decision_digest: impl Into<String>,
    ) -> Self {
        Self {
            authority_id: authority_id.into(),
            capability_id: capability_id.into(),
            subject_digest: subject_digest.into(),
            decision_digest: decision_digest.into(),
        }
    }
}

/// Receiptability contract required before consequential execution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReceiptRequirement {
    pub receipt_version: String,
    pub digest_algorithm: String,
    pub replay_contract: String,
    pub parent_receipt_digest: Option<String>,
}

impl ReceiptRequirement {
    #[must_use]
    pub fn new(
        receipt_version: impl Into<String>,
        digest_algorithm: impl Into<String>,
        replay_contract: impl Into<String>,
    ) -> Self {
        Self {
            receipt_version: receipt_version.into(),
            digest_algorithm: digest_algorithm.into(),
            replay_contract: replay_contract.into(),
            parent_receipt_digest: None,
        }
    }

    #[must_use]
    pub fn with_parent(mut self, digest: impl Into<String>) -> Self {
        self.parent_receipt_digest = Some(digest.into());
        self
    }
}

mod sealed {
    pub trait Sealed {}
}

/// Type-level phase marker for consequence intents.
pub trait PhaseMarker: sealed::Sealed {
    const CLASS: ConsequenceClass;
    const REVERSIBLE: bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectPhase;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstructPhase;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DoPhase;

impl sealed::Sealed for SelectPhase {}
impl sealed::Sealed for ConstructPhase {}
impl sealed::Sealed for DoPhase {}

impl PhaseMarker for SelectPhase {
    const CLASS: ConsequenceClass = ConsequenceClass::Select;
    const REVERSIBLE: bool = true;
}
impl PhaseMarker for ConstructPhase {
    const CLASS: ConsequenceClass = ConsequenceClass::Construct;
    const REVERSIBLE: bool = true;
}
impl PhaseMarker for DoPhase {
    const CLASS: ConsequenceClass = ConsequenceClass::Do;
    const REVERSIBLE: bool = false;
}

/// Typed consequence intent. A naked `Intent<DoPhase>` cannot be publicly
/// constructed; callers must use [`DoEnvelope::try_new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Intent<P: PhaseMarker> {
    capability_id: String,
    semantic_digest: String,
    subject: SubjectRef,
    input_digest: String,
    phase: PhantomData<P>,
}

impl<P: PhaseMarker> Intent<P> {
    fn from_contract(
        contract: &CapabilityContract,
        subject: SubjectRef,
        input_digest: impl Into<String>,
    ) -> Result<Self, Vec<ProtocolRefusal>> {
        let input_digest = input_digest.into();
        let mut refusals = contract.validate();
        if contract.consequence != P::CLASS {
            refusals.push(ProtocolRefusal::ConsequenceClassMismatch {
                capability_id: contract.id.clone(),
                expected: P::CLASS,
                actual: contract.consequence,
            });
        }
        validate_subject_and_input(&subject, &input_digest, &mut refusals);
        if refusals.is_empty() {
            Ok(Self {
                capability_id: contract.id.clone(),
                semantic_digest: contract.semantic_digest.clone(),
                subject,
                input_digest,
                phase: PhantomData,
            })
        } else {
            Err(refusals)
        }
    }

    #[must_use]
    pub fn capability_id(&self) -> &str {
        &self.capability_id
    }

    #[must_use]
    pub fn semantic_digest(&self) -> &str {
        &self.semantic_digest
    }

    #[must_use]
    pub fn subject(&self) -> &SubjectRef {
        &self.subject
    }

    #[must_use]
    pub fn input_digest(&self) -> &str {
        &self.input_digest
    }

    #[must_use]
    pub const fn consequence_class(&self) -> ConsequenceClass {
        P::CLASS
    }

    #[must_use]
    pub const fn reversible(&self) -> bool {
        P::REVERSIBLE
    }
}

impl Intent<SelectPhase> {
    pub fn try_new(
        contract: &CapabilityContract,
        subject: SubjectRef,
        input_digest: impl Into<String>,
    ) -> Result<Self, Vec<ProtocolRefusal>> {
        Self::from_contract(contract, subject, input_digest)
    }
}

impl Intent<ConstructPhase> {
    pub fn try_new(
        contract: &CapabilityContract,
        subject: SubjectRef,
        input_digest: impl Into<String>,
    ) -> Result<Self, Vec<ProtocolRefusal>> {
        Self::from_contract(contract, subject, input_digest)
    }
}

/// The only public construction path for a typed consequential DO intent.
///
/// This is still only a structural envelope. `wasm4pm` must independently
/// verify the authority decision, execute through its brokered boundary, emit
/// the receipt, and replay/verify the result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoEnvelope {
    intent: Intent<DoPhase>,
    pub authority: AuthorityDecisionRef,
    pub receipt: ReceiptRequirement,
}

impl DoEnvelope {
    pub fn try_new(
        contract: &CapabilityContract,
        subject: SubjectRef,
        input_digest: impl Into<String>,
        authority: AuthorityDecisionRef,
        receipt: ReceiptRequirement,
    ) -> Result<Self, Vec<ProtocolRefusal>> {
        let mut refusals = Vec::new();
        let input_digest = input_digest.into();

        let intent = match Intent::<DoPhase>::from_contract(contract, subject, input_digest) {
            Ok(intent) => Some(intent),
            Err(errors) => {
                refusals.extend(errors);
                None
            }
        };

        if authority.authority_id.trim().is_empty() {
            refusals.push(ProtocolRefusal::MissingAuthorityId);
        }
        if authority.decision_digest.trim().is_empty() {
            refusals.push(ProtocolRefusal::MissingAuthorityDecisionDigest);
        }
        if authority.capability_id != contract.id {
            refusals.push(ProtocolRefusal::AuthorityCapabilityMismatch {
                expected: contract.id.clone(),
                actual: authority.capability_id.clone(),
            });
        }
        if let Some(intent) = &intent {
            if authority.subject_digest != intent.subject.subject_digest {
                refusals.push(ProtocolRefusal::AuthoritySubjectMismatch {
                    expected: intent.subject.subject_digest.clone(),
                    actual: authority.subject_digest.clone(),
                });
            }
        }
        if receipt.receipt_version.trim().is_empty() {
            refusals.push(ProtocolRefusal::MissingReceiptVersion);
        }
        if receipt.digest_algorithm.trim().is_empty() {
            refusals.push(ProtocolRefusal::MissingReceiptDigestAlgorithm);
        }
        if receipt.replay_contract.trim().is_empty() {
            refusals.push(ProtocolRefusal::MissingReplayContract);
        }

        if refusals.is_empty() {
            Ok(Self {
                intent: intent.expect("intent exists when refusals are empty"),
                authority,
                receipt,
            })
        } else {
            Err(refusals)
        }
    }

    #[must_use]
    pub fn intent(&self) -> &Intent<DoPhase> {
        &self.intent
    }
}

fn validate_subject_and_input(
    subject: &SubjectRef,
    input_digest: &str,
    refusals: &mut Vec<ProtocolRefusal>,
) {
    if subject.subject_id.trim().is_empty() {
        refusals.push(ProtocolRefusal::MissingSubjectId);
    }
    if subject.subject_digest.trim().is_empty() {
        refusals.push(ProtocolRefusal::MissingSubjectDigest);
    }
    if input_digest.trim().is_empty() {
        refusals.push(ProtocolRefusal::MissingInputDigest);
    }
}

/// Named structural refusals for the federated protocol grammar.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "law", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProtocolRefusal {
    EmptyProtocolId,
    EmptyProtocolVersion,
    EmptyCapabilityId,
    DuplicateCapabilityId {
        capability_id: String,
    },
    MissingPublicSemantic {
        capability_id: String,
    },
    MissingSemanticDigest {
        capability_id: String,
    },
    MissingInputType {
        capability_id: String,
    },
    MissingOutputType {
        capability_id: String,
    },
    MissingOcelEventType {
        capability_id: String,
    },
    DoWithoutAuthority {
        capability_id: String,
    },
    DoWithoutRequiredReceipt {
        capability_id: String,
    },
    UnknownCapabilityProjection {
        capability_id: String,
        surface: SurfaceKind,
    },
    DuplicateSurfaceBinding {
        capability_id: String,
        surface: SurfaceKind,
    },
    MissingSurfaceBinding {
        capability_id: String,
        surface: SurfaceKind,
    },
    AmbientAuthorityOnSurface {
        capability_id: String,
        surface: SurfaceKind,
    },
    ProjectionSemanticDrift {
        capability_id: String,
        surface: SurfaceKind,
    },
    EmptySurfaceSchema {
        capability_id: String,
        surface: SurfaceKind,
    },
    EmptyDispositionReason {
        capability_id: String,
        surface: SurfaceKind,
    },
    ConsequenceClassMismatch {
        capability_id: String,
        expected: ConsequenceClass,
        actual: ConsequenceClass,
    },
    MissingSubjectId,
    MissingSubjectDigest,
    MissingInputDigest,
    MissingAuthorityId,
    MissingAuthorityDecisionDigest,
    AuthorityCapabilityMismatch {
        expected: String,
        actual: String,
    },
    AuthoritySubjectMismatch {
        expected: String,
        actual: String,
    },
    MissingReceiptVersion,
    MissingReceiptDigestAlgorithm,
    MissingReplayContract,
}
