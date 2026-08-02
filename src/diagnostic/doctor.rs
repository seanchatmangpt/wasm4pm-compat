//! Deterministic compatibility doctor, capability negotiation, and repair plans.
//!
//! The doctor answers four questions without crossing the structure-only fence:
//!
//! 1. Which compatibility capabilities are present in this build?
//! 2. Which requested capabilities are blocked by feature selection?
//! 3. Which intents belong in `wasm4pm` or an external standing verifier?
//! 4. What is the smallest reversible repair that closes each blocked edge?
//!
//! It does **not** discover models, execute conformance, replay evidence, mutate
//! the repository, or award `ALIVE`. Its strongest self-issued standing is
//! [`DoctorStanding::PartialAlive`]; exact-tree `ALIVE` remains external.

use crate::diagnostic::{CompatDiagnostic, DiagnosticSeverity};
use crate::hash::{blake3_string, canonical_json};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fmt::{Display, Write as _};

/// Stable schema identifier for machine-readable doctor reports.
pub const DOCTOR_SCHEMA: &str = "https://chatmangpt.com/ns/wasm4pm-compat/doctor/v1";

/// Version of the doctor report schema.
pub const DOCTOR_SCHEMA_VERSION: u32 = 1;

/// Bounded diagnostic profile selected by a caller.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DoctorProfile {
    /// Core typestate, witness, refusal, and diagnostic vocabulary.
    Core,
    /// Core plus receipt, deterministic digest, and DfCM boundary surfaces.
    Boundary,
    /// Boundary plus import/export and round-trip contracts.
    Interop,
    /// Boundary plus the engine graduation bridge.
    Graduation,
    /// Full compat-side Vision 2030 posture, including lawful external routes.
    Vision2030,
}

impl DoctorProfile {
    /// Every supported profile in deterministic order.
    pub const ALL: [Self; 5] = [
        Self::Core,
        Self::Boundary,
        Self::Interop,
        Self::Graduation,
        Self::Vision2030,
    ];

    /// Stable lowercase profile name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Core => "core",
            Self::Boundary => "boundary",
            Self::Interop => "interop",
            Self::Graduation => "graduation",
            Self::Vision2030 => "vision2030",
        }
    }

    /// Parse a CLI/API profile name.
    pub fn parse(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "core" => Some(Self::Core),
            "boundary" | "default" => Some(Self::Boundary),
            "interop" | "formats" => Some(Self::Interop),
            "graduation" | "wasm4pm" => Some(Self::Graduation),
            "vision2030" | "vision-2030" | "2030" | "all" => Some(Self::Vision2030),
            _ => None,
        }
    }

    fn requirements(self) -> &'static [Capability] {
        const CORE: &[Capability] = &[
            Capability::TypedEvidence,
            Capability::NamedRefusals,
            Capability::Diagnostics,
        ];
        const BOUNDARY: &[Capability] = &[
            Capability::TypedEvidence,
            Capability::NamedRefusals,
            Capability::Diagnostics,
            Capability::ReceiptShapes,
            Capability::DeterministicDigests,
            Capability::Dfcm,
            Capability::Doctor,
        ];
        const INTEROP: &[Capability] = &[
            Capability::TypedEvidence,
            Capability::NamedRefusals,
            Capability::Diagnostics,
            Capability::ReceiptShapes,
            Capability::DeterministicDigests,
            Capability::Dfcm,
            Capability::Doctor,
            Capability::Formats,
        ];
        const GRADUATION: &[Capability] = &[
            Capability::TypedEvidence,
            Capability::NamedRefusals,
            Capability::Diagnostics,
            Capability::ReceiptShapes,
            Capability::DeterministicDigests,
            Capability::Dfcm,
            Capability::Doctor,
            Capability::GraduationBridge,
        ];
        const VISION_2030: &[Capability] = &[
            Capability::TypedEvidence,
            Capability::NamedRefusals,
            Capability::Diagnostics,
            Capability::ReceiptShapes,
            Capability::DeterministicDigests,
            Capability::Dfcm,
            Capability::Doctor,
            Capability::Formats,
            Capability::StrictBoundary,
            Capability::GraduationBridge,
            Capability::EngineExecution,
            Capability::StandingAuthority,
        ];

        match self {
            Self::Core => CORE,
            Self::Boundary => BOUNDARY,
            Self::Interop => INTEROP,
            Self::Graduation => GRADUATION,
            Self::Vision2030 => VISION_2030,
        }
    }
}

impl Display for DoctorProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Owner that lawfully provides a capability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityOwner {
    /// The structure-only compatibility crate.
    Compat,
    /// The active process-intelligence execution engine.
    Wasm4pm,
    /// The exact-tree verifier that alone may award standing.
    ExternalVerifier,
}

impl CapabilityOwner {
    /// Stable owner name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Compat => "wasm4pm-compat",
            Self::Wasm4pm => "wasm4pm",
            Self::ExternalVerifier => "external-verifier",
        }
    }
}

impl Display for CapabilityOwner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Capability understood by the compatibility doctor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Capability {
    /// Typestate evidence and witness-bound values.
    TypedEvidence,
    /// Specific typed refusal reasons.
    NamedRefusals,
    /// Structured diagnostics and repair vocabulary.
    Diagnostics,
    /// Receipt envelopes and replay-hint shapes.
    ReceiptShapes,
    /// Canonical JSON and deterministic BLAKE3 structural identities.
    DeterministicDigests,
    /// Design-for-Combinatorial-Maximality matrices and reports.
    Dfcm,
    /// The deterministic compatibility doctor itself.
    Doctor,
    /// Import/export, projection, and round-trip contracts.
    Formats,
    /// Strict boundary judgment.
    StrictBoundary,
    /// Structural graduation candidates for the engine.
    GraduationBridge,
    /// Active discovery/conformance/replay/optimization execution.
    EngineExecution,
    /// Exact-tree `ALIVE` standing authority.
    StandingAuthority,
}

impl Capability {
    /// Every capability in deterministic display order.
    pub const ALL: [Self; 12] = [
        Self::TypedEvidence,
        Self::NamedRefusals,
        Self::Diagnostics,
        Self::ReceiptShapes,
        Self::DeterministicDigests,
        Self::Dfcm,
        Self::Doctor,
        Self::Formats,
        Self::StrictBoundary,
        Self::GraduationBridge,
        Self::EngineExecution,
        Self::StandingAuthority,
    ];

    /// Stable machine code.
    pub const fn code(self) -> &'static str {
        match self {
            Self::TypedEvidence => "typed_evidence",
            Self::NamedRefusals => "named_refusals",
            Self::Diagnostics => "diagnostics",
            Self::ReceiptShapes => "receipt_shapes",
            Self::DeterministicDigests => "deterministic_digests",
            Self::Dfcm => "dfcm",
            Self::Doctor => "doctor",
            Self::Formats => "formats",
            Self::StrictBoundary => "strict_boundary",
            Self::GraduationBridge => "graduation_bridge",
            Self::EngineExecution => "engine_execution",
            Self::StandingAuthority => "standing_authority",
        }
    }

    /// Human-readable capability summary.
    pub const fn summary(self) -> &'static str {
        match self {
            Self::TypedEvidence => "witness-bound typestate evidence",
            Self::NamedRefusals => "specific typed refusal laws",
            Self::Diagnostics => "diagnostic and repair vocabulary",
            Self::ReceiptShapes => "receipt and replay-hint shapes",
            Self::DeterministicDigests => "canonical JSON and BLAKE3 identities",
            Self::Dfcm => "combinatorial coverage matrices",
            Self::Doctor => "deterministic doctor and route planner",
            Self::Formats => "import, export, projection, and round-trip contracts",
            Self::StrictBoundary => "strict admission and refusal surfaces",
            Self::GraduationBridge => "structural graduation candidates",
            Self::EngineExecution => "discovery, conformance, replay, and optimization",
            Self::StandingAuthority => "exact-tree external standing judgment",
        }
    }

    /// Lawful owner for this capability.
    pub const fn owner(self) -> CapabilityOwner {
        match self {
            Self::EngineExecution => CapabilityOwner::Wasm4pm,
            Self::StandingAuthority => CapabilityOwner::ExternalVerifier,
            _ => CapabilityOwner::Compat,
        }
    }
}

/// Observed availability of a capability in the current build.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityState {
    /// Available in this build and owned by compat.
    Available,
    /// Lawful compat capability exists but its Cargo feature is disabled.
    Blocked,
    /// Capability is available only through its named external owner.
    Routed,
}

/// Deterministic observation of one capability edge.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityObservation {
    /// Capability being described.
    pub capability: Capability,
    /// Stable capability code.
    pub code: String,
    /// Lawful owner.
    pub owner: CapabilityOwner,
    /// Current availability.
    pub state: CapabilityState,
    /// Cargo feature that opens the edge, when applicable.
    pub required_feature: Option<String>,
    /// Why this state is correct.
    pub reason: String,
}

impl CapabilityObservation {
    fn available(capability: Capability) -> Self {
        Self {
            capability,
            code: capability.code().to_string(),
            owner: capability.owner(),
            state: CapabilityState::Available,
            required_feature: None,
            reason: format!("{} is present in the compat core", capability.summary()),
        }
    }

    fn feature(capability: Capability, feature: &str, enabled: bool) -> Self {
        let state = if enabled {
            CapabilityState::Available
        } else {
            CapabilityState::Blocked
        };
        let reason = if enabled {
            format!("Cargo feature `{feature}` is enabled")
        } else {
            format!("Cargo feature `{feature}` is disabled")
        };
        Self {
            capability,
            code: capability.code().to_string(),
            owner: CapabilityOwner::Compat,
            state,
            required_feature: Some(feature.to_string()),
            reason,
        }
    }

    fn routed(capability: Capability, reason: &str) -> Self {
        Self {
            capability,
            code: capability.code().to_string(),
            owner: capability.owner(),
            state: CapabilityState::Routed,
            required_feature: None,
            reason: reason.to_string(),
        }
    }
}

/// Snapshot every known capability in deterministic order.
pub fn capability_snapshot() -> Vec<CapabilityObservation> {
    vec![
        CapabilityObservation::available(Capability::TypedEvidence),
        CapabilityObservation::available(Capability::NamedRefusals),
        CapabilityObservation::available(Capability::Diagnostics),
        CapabilityObservation::available(Capability::ReceiptShapes),
        CapabilityObservation::available(Capability::DeterministicDigests),
        CapabilityObservation::available(Capability::Dfcm),
        CapabilityObservation::available(Capability::Doctor),
        CapabilityObservation::feature(Capability::Formats, "formats", cfg!(feature = "formats")),
        CapabilityObservation::feature(
            Capability::StrictBoundary,
            "strict",
            cfg!(feature = "strict"),
        ),
        CapabilityObservation::feature(
            Capability::GraduationBridge,
            "wasm4pm",
            cfg!(feature = "wasm4pm"),
        ),
        CapabilityObservation::routed(
            Capability::EngineExecution,
            "active execution is owned by wasm4pm; compat only prepares graduation candidates",
        ),
        CapabilityObservation::routed(
            Capability::StandingAuthority,
            "ALIVE is awarded only by an exact-tree external verifier",
        ),
    ]
}

/// State of one doctor check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DoctorCheckState {
    /// Required compat capability is available.
    Pass,
    /// The capability is intentionally routed to another authority.
    Advisory,
    /// A reversible local prerequisite is absent.
    Blocked,
    /// The bounded doctor does not implement this capability.
    Unsupported,
    /// A named law rejected the requested route.
    Refused,
}

impl DoctorCheckState {
    /// Stable uppercase state name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pass => "PASS",
            Self::Advisory => "ADVISORY",
            Self::Blocked => "BLOCKED",
            Self::Unsupported => "UNSUPPORTED",
            Self::Refused => "REFUSED",
        }
    }
}

impl Display for DoctorCheckState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Standing the doctor may issue for its bounded report.
///
/// `ALIVE` is deliberately absent. Only the external exact-tree verifier may
/// mint that state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DoctorStanding {
    /// Evidence is missing or no checks were selected.
    Unknown,
    /// Bounded checks passed, but crown standing remains external.
    PartialAlive,
    /// A required feature or prerequisite is absent.
    Blocked,
    /// Source reached execution but compilation or tests failed.
    BuildBroken,
    /// The bounded doctor does not model a requested surface.
    Unsupported,
}

impl DoctorStanding {
    /// Stable uppercase state name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "UNKNOWN",
            Self::PartialAlive => "PARTIAL_ALIVE",
            Self::Blocked => "BLOCKED",
            Self::BuildBroken => "BUILD_BROKEN",
            Self::Unsupported => "UNSUPPORTED",
        }
    }
}

impl Display for DoctorStanding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Smallest reversible action that closes a blocked capability edge.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepairAction {
    /// Stable repair code.
    pub code: String,
    /// Owner responsible for the repair.
    pub owner: CapabilityOwner,
    /// Human-readable repair.
    pub summary: String,
    /// Optional copy-paste command.
    pub command: Option<String>,
    /// Whether the repair preserves all prior lawful possibilities.
    pub reversible: bool,
}

impl RepairAction {
    fn enable_feature(feature: &str) -> Self {
        Self {
            code: format!("ENABLE_FEATURE_{}", feature.to_ascii_uppercase()),
            owner: CapabilityOwner::Compat,
            summary: format!("enable Cargo feature `{feature}` for this consumer"),
            command: Some(format!(
                "cargo run --features {feature} --bin wasm4pm-compat -- doctor"
            )),
            reversible: true,
        }
    }
}

/// One bounded doctor check.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoctorCheck {
    /// Stable check identifier.
    pub code: String,
    /// Capability under observation.
    pub capability: Option<Capability>,
    /// Check outcome.
    pub state: DoctorCheckState,
    /// Concise result.
    pub summary: String,
    /// Why the result holds.
    pub detail: String,
    /// Minimal repair, when blocked.
    pub repair: Option<RepairAction>,
}

/// Machine-readable compatibility doctor report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoctorReport {
    /// Report schema identifier.
    pub schema: String,
    /// Report schema version.
    pub schema_version: u32,
    /// Crate package name.
    pub crate_name: String,
    /// Crate package version.
    pub crate_version: String,
    /// Requested profile.
    pub profile: DoctorProfile,
    /// Bounded standing. Never `ALIVE`.
    pub standing: DoctorStanding,
    /// Deterministically ordered checks.
    pub checks: Vec<DoctorCheck>,
    /// Complete capability snapshot.
    pub capabilities: Vec<CapabilityObservation>,
    /// Deduplicated minimal repair set.
    pub repairs: Vec<RepairAction>,
}

impl DoctorReport {
    /// Canonical JSON representation suitable for replay comparison.
    pub fn canonical_json(&self) -> Result<String, serde_json::Error> {
        canonical_json(self)
    }

    /// Deterministic BLAKE3 identity of the canonical report body.
    ///
    /// This is a content identity, not an authoritative standing receipt.
    pub fn fingerprint(&self) -> Result<String, serde_json::Error> {
        Ok(blake3_string(&self.canonical_json()?))
    }

    /// Whether any check blocks the requested profile.
    pub fn is_blocked(&self) -> bool {
        self.checks
            .iter()
            .any(|check| check.state == DoctorCheckState::Blocked)
    }

    /// Process exit code recommended to CLI adapters.
    pub const fn exit_code(&self) -> u8 {
        match self.standing {
            DoctorStanding::PartialAlive => 0,
            DoctorStanding::Unknown => 1,
            DoctorStanding::Blocked => 2,
            DoctorStanding::BuildBroken => 3,
            DoctorStanding::Unsupported => 4,
        }
    }

    /// Human-readable deterministic report.
    pub fn render_text(&self) -> String {
        let mut output = String::new();
        writeln!(
            &mut output,
            "wasm4pm-compat doctor {} ({})",
            self.profile, self.crate_version
        )
        .expect("writing to String cannot fail");
        writeln!(&mut output, "standing: {}", self.standing)
            .expect("writing to String cannot fail");
        for check in &self.checks {
            writeln!(
                &mut output,
                "[{}] {} — {}",
                check.state, check.code, check.summary
            )
            .expect("writing to String cannot fail");
            writeln!(&mut output, "  {}", check.detail)
                .expect("writing to String cannot fail");
        }
        if !self.repairs.is_empty() {
            output.push_str("repairs:\n");
            for repair in &self.repairs {
                writeln!(&mut output, "- {}: {}", repair.code, repair.summary)
                    .expect("writing to String cannot fail");
                if let Some(command) = &repair.command {
                    writeln!(&mut output, "  {}", command)
                        .expect("writing to String cannot fail");
                }
            }
        }
        output
    }
}

/// Structure-only doctor entry point.
#[derive(Debug, Default, Clone, Copy)]
pub struct CompatDoctor;

impl CompatDoctor {
    /// Run one bounded diagnostic profile.
    pub fn run(profile: DoctorProfile) -> DoctorReport {
        let capabilities = capability_snapshot();
        let mut checks = Vec::new();

        for capability in profile.requirements() {
            let observation = capabilities
                .iter()
                .find(|candidate| candidate.capability == *capability);
            match observation {
                Some(observation) => checks.push(check_from_observation(observation)),
                None => checks.push(DoctorCheck {
                    code: format!("CAPABILITY_{}_UNMODELED", capability.code().to_ascii_uppercase()),
                    capability: Some(*capability),
                    state: DoctorCheckState::Unsupported,
                    summary: format!("{} is not modeled", capability.summary()),
                    detail: "the bounded doctor has no observation for this capability".to_string(),
                    repair: None,
                }),
            }
        }

        checks.push(DoctorCheck {
            code: "COMPAT_DOCTOR_NO_ALIVE_AUTHORITY".to_string(),
            capability: Some(Capability::StandingAuthority),
            state: DoctorCheckState::Advisory,
            summary: "crown standing remains external".to_string(),
            detail: "the doctor can issue PARTIAL_ALIVE but cannot award ALIVE".to_string(),
            repair: None,
        });
        checks.push(DoctorCheck {
            code: "COMPAT_DOCTOR_ZERO_ACTUATION".to_string(),
            capability: Some(Capability::Doctor),
            state: DoctorCheckState::Pass,
            summary: "doctor performs no process or repository actuation".to_string(),
            detail: "all outputs are inert reports, route decisions, and reversible repair plans"
                .to_string(),
            repair: None,
        });

        let standing = standing_from_checks(&checks);
        let repairs = collect_repairs(&checks);

        DoctorReport {
            schema: DOCTOR_SCHEMA.to_string(),
            schema_version: DOCTOR_SCHEMA_VERSION,
            crate_name: env!("CARGO_PKG_NAME").to_string(),
            crate_version: env!("CARGO_PKG_VERSION").to_string(),
            profile,
            standing,
            checks,
            capabilities,
            repairs,
        }
    }

    /// Build a lawful route plan for requested intents.
    pub fn plan(intents: impl IntoIterator<Item = Intent>) -> RoutePlan {
        RoutePlan::new(intents)
    }
}

fn check_from_observation(observation: &CapabilityObservation) -> DoctorCheck {
    let (state, repair) = match observation.state {
        CapabilityState::Available => (DoctorCheckState::Pass, None),
        CapabilityState::Blocked => (
            DoctorCheckState::Blocked,
            observation
                .required_feature
                .as_deref()
                .map(RepairAction::enable_feature),
        ),
        CapabilityState::Routed => (DoctorCheckState::Advisory, None),
    };

    DoctorCheck {
        code: format!("CAPABILITY_{}", observation.code.to_ascii_uppercase()),
        capability: Some(observation.capability),
        state,
        summary: observation.capability.summary().to_string(),
        detail: observation.reason.clone(),
        repair,
    }
}

fn standing_from_checks(checks: &[DoctorCheck]) -> DoctorStanding {
    if checks.is_empty() {
        return DoctorStanding::Unknown;
    }
    if checks
        .iter()
        .any(|check| check.state == DoctorCheckState::Unsupported)
    {
        return DoctorStanding::Unsupported;
    }
    if checks.iter().any(|check| {
        matches!(
            check.state,
            DoctorCheckState::Blocked | DoctorCheckState::Refused
        )
    }) {
        return DoctorStanding::Blocked;
    }
    DoctorStanding::PartialAlive
}

fn collect_repairs(checks: &[DoctorCheck]) -> Vec<RepairAction> {
    let mut seen = BTreeSet::new();
    let mut repairs = Vec::new();
    for repair in checks.iter().filter_map(|check| check.repair.clone()) {
        if seen.insert(repair.code.clone()) {
            repairs.push(repair);
        }
    }
    repairs
}

/// Requested operation used by the route planner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Intent {
    /// Admit raw evidence under a witness.
    Admit,
    /// Perform strict boundary admission.
    StrictAdmit,
    /// Import an external representation.
    Import,
    /// Project admitted evidence under an explicit loss policy.
    Project,
    /// Export admitted evidence.
    Export,
    /// Build a receipt shape.
    Receipt,
    /// Diagnose a compatibility boundary.
    Diagnose,
    /// Prepare a graduation candidate.
    Graduate,
    /// Discover a process model.
    Discover,
    /// Execute conformance checking.
    Conformance,
    /// Replay evidence.
    Replay,
    /// Optimize or plan an active process.
    Optimize,
    /// Verify exact-tree standing.
    VerifyStanding,
}

impl Intent {
    /// Stable intent name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Admit => "admit",
            Self::StrictAdmit => "strict_admit",
            Self::Import => "import",
            Self::Project => "project",
            Self::Export => "export",
            Self::Receipt => "receipt",
            Self::Diagnose => "diagnose",
            Self::Graduate => "graduate",
            Self::Discover => "discover",
            Self::Conformance => "conformance",
            Self::Replay => "replay",
            Self::Optimize => "optimize",
            Self::VerifyStanding => "verify_standing",
        }
    }

    /// Parse a CLI/API intent name.
    pub fn parse(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().replace('-', "_").as_str() {
            "admit" | "admission" => Some(Self::Admit),
            "strict" | "strict_admit" | "strict_admission" => Some(Self::StrictAdmit),
            "import" => Some(Self::Import),
            "project" | "projection" => Some(Self::Project),
            "export" => Some(Self::Export),
            "receipt" | "receipt_shape" => Some(Self::Receipt),
            "diagnose" | "doctor" => Some(Self::Diagnose),
            "graduate" | "graduation" => Some(Self::Graduate),
            "discover" | "discovery" => Some(Self::Discover),
            "conformance" | "conform" => Some(Self::Conformance),
            "replay" => Some(Self::Replay),
            "optimize" | "optimise" | "plan" => Some(Self::Optimize),
            "verify_standing" | "standing" | "alive" => Some(Self::VerifyStanding),
            _ => None,
        }
    }
}

impl Display for Intent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Target selected by the lawful route planner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteTarget {
    /// Stay inside the structure-only compatibility court.
    Compat,
    /// Graduate to the active execution engine.
    Wasm4pm,
    /// Submit exact-tree evidence to the standing verifier.
    ExternalVerifier,
}

impl From<RouteTarget> for CapabilityOwner {
    fn from(value: RouteTarget) -> Self {
        match value {
            RouteTarget::Compat => Self::Compat,
            RouteTarget::Wasm4pm => Self::Wasm4pm,
            RouteTarget::ExternalVerifier => Self::ExternalVerifier,
        }
    }
}

/// Outcome of one routing decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteState {
    /// Intent is admitted to its selected owner.
    Admitted,
    /// A reversible prerequisite is absent.
    Blocked,
    /// The intent is intentionally delegated to another owner.
    Routed,
}

/// One lawful route decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteDecision {
    /// Requested intent.
    pub intent: Intent,
    /// Selected owner boundary.
    pub target: RouteTarget,
    /// Route state.
    pub state: RouteState,
    /// Why the route is lawful.
    pub reason: String,
    /// Optional feature prerequisite.
    pub required_feature: Option<String>,
    /// Optional repair for a blocked edge.
    pub repair: Option<RepairAction>,
}

/// Deterministic plan over one or more requested intents.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoutePlan {
    /// Plan schema identifier.
    pub schema: String,
    /// Bounded plan standing. Never `ALIVE`.
    pub standing: DoctorStanding,
    /// Decisions in request order.
    pub decisions: Vec<RouteDecision>,
    /// Deduplicated repairs.
    pub repairs: Vec<RepairAction>,
}

impl RoutePlan {
    fn new(intents: impl IntoIterator<Item = Intent>) -> Self {
        let decisions: Vec<_> = intents.into_iter().map(route_intent).collect();
        let standing = if decisions.is_empty() {
            DoctorStanding::Unknown
        } else if decisions
            .iter()
            .any(|decision| decision.state == RouteState::Blocked)
        {
            DoctorStanding::Blocked
        } else {
            DoctorStanding::PartialAlive
        };

        let mut seen = BTreeSet::new();
        let mut repairs = Vec::new();
        for repair in decisions
            .iter()
            .filter_map(|decision| decision.repair.clone())
        {
            if seen.insert(repair.code.clone()) {
                repairs.push(repair);
            }
        }

        Self {
            schema: format!("{DOCTOR_SCHEMA}/route-plan"),
            standing,
            decisions,
            repairs,
        }
    }

    /// Canonical JSON representation suitable for replay comparison.
    pub fn canonical_json(&self) -> Result<String, serde_json::Error> {
        canonical_json(self)
    }

    /// Deterministic BLAKE3 identity of the canonical plan body.
    pub fn fingerprint(&self) -> Result<String, serde_json::Error> {
        Ok(blake3_string(&self.canonical_json()?))
    }

    /// Human-readable deterministic route plan.
    pub fn render_text(&self) -> String {
        let mut output = format!("route standing: {}\n", self.standing);
        for decision in &self.decisions {
            writeln!(
                &mut output,
                "[{:?}] {} -> {:?}: {}",
                decision.state, decision.intent, decision.target, decision.reason
            )
            .expect("writing to String cannot fail");
        }
        if !self.repairs.is_empty() {
            output.push_str("repairs:\n");
            for repair in &self.repairs {
                writeln!(&mut output, "- {}: {}", repair.code, repair.summary)
                    .expect("writing to String cannot fail");
            }
        }
        output
    }
}

fn route_intent(intent: Intent) -> RouteDecision {
    match intent {
        Intent::Admit | Intent::Receipt | Intent::Diagnose => RouteDecision {
            intent,
            target: RouteTarget::Compat,
            state: RouteState::Admitted,
            reason: "intent is structure-only and belongs in the compatibility court".to_string(),
            required_feature: None,
            repair: None,
        },
        Intent::StrictAdmit => feature_route(intent, RouteTarget::Compat, "strict"),
        Intent::Import | Intent::Project | Intent::Export => {
            feature_route(intent, RouteTarget::Compat, "formats")
        }
        Intent::Graduate => feature_route(intent, RouteTarget::Compat, "wasm4pm"),
        Intent::Discover | Intent::Conformance | Intent::Replay | Intent::Optimize => {
            if cfg!(feature = "wasm4pm") {
                RouteDecision {
                    intent,
                    target: RouteTarget::Wasm4pm,
                    state: RouteState::Routed,
                    reason: "active execution is fenced out of compat and routed through the graduation bridge"
                        .to_string(),
                    required_feature: Some("wasm4pm".to_string()),
                    repair: None,
                }
            } else {
                RouteDecision {
                    intent,
                    target: RouteTarget::Wasm4pm,
                    state: RouteState::Blocked,
                    reason: "the lawful engine route exists, but the `wasm4pm` graduation feature is disabled"
                        .to_string(),
                    required_feature: Some("wasm4pm".to_string()),
                    repair: Some(RepairAction::enable_feature("wasm4pm")),
                }
            }
        }
        Intent::VerifyStanding => RouteDecision {
            intent,
            target: RouteTarget::ExternalVerifier,
            state: RouteState::Routed,
            reason: "exact-tree standing is external; compat cannot self-promote to ALIVE".to_string(),
            required_feature: None,
            repair: None,
        },
    }
}

fn feature_route(intent: Intent, target: RouteTarget, feature: &str) -> RouteDecision {
    let enabled = match feature {
        "formats" => cfg!(feature = "formats"),
        "strict" => cfg!(feature = "strict"),
        "wasm4pm" => cfg!(feature = "wasm4pm"),
        _ => false,
    };
    if enabled {
        RouteDecision {
            intent,
            target,
            state: RouteState::Admitted,
            reason: format!("Cargo feature `{feature}` admits this structural route"),
            required_feature: Some(feature.to_string()),
            repair: None,
        }
    } else {
        RouteDecision {
            intent,
            target,
            state: RouteState::Blocked,
            reason: format!("Cargo feature `{feature}` is required for this structural route"),
            required_feature: Some(feature.to_string()),
            repair: Some(RepairAction::enable_feature(feature)),
        }
    }
}

/// Serializable catalog entry for one compatibility diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticCatalogEntry {
    /// Stable diagnostic code.
    pub code: String,
    /// Rust variant name.
    pub name: String,
    /// Severity.
    pub severity: String,
    /// Exact accusation.
    pub message: String,
    /// Minimal repair.
    pub repair: String,
}

/// Return the full diagnostic catalog in deterministic order.
pub fn diagnostic_catalog() -> Vec<DiagnosticCatalogEntry> {
    CompatDiagnostic::ALL
        .into_iter()
        .map(|diagnostic| DiagnosticCatalogEntry {
            code: diagnostic.code().to_string(),
            name: diagnostic.name().to_string(),
            severity: diagnostic.severity().to_string(),
            message: diagnostic.message().to_string(),
            repair: diagnostic.repair().to_string(),
        })
        .collect()
}

/// Resolve a diagnostic by stable code or Rust variant name.
pub fn explain_diagnostic(value: &str) -> Option<DiagnosticCatalogEntry> {
    CompatDiagnostic::from_code(value).map(|diagnostic| DiagnosticCatalogEntry {
        code: diagnostic.code().to_string(),
        name: diagnostic.name().to_string(),
        severity: diagnostic.severity().to_string(),
        message: diagnostic.message().to_string(),
        repair: diagnostic.repair().to_string(),
    })
}

/// Convert a diagnostic severity to a doctor check state.
pub const fn check_state_for_severity(severity: DiagnosticSeverity) -> DoctorCheckState {
    match severity {
        DiagnosticSeverity::Error => DoctorCheckState::Refused,
        DiagnosticSeverity::Warning => DoctorCheckState::Blocked,
        DiagnosticSeverity::Info => DoctorCheckState::Advisory,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn capability_snapshot_is_complete_and_unique() {
        let snapshot = capability_snapshot();
        assert_eq!(snapshot.len(), Capability::ALL.len());
        let unique: BTreeSet<_> = snapshot.iter().map(|item| item.code.as_str()).collect();
        assert_eq!(unique.len(), snapshot.len());
    }

    #[test]
    fn core_report_is_bounded_partial_alive() {
        let report = CompatDoctor::run(DoctorProfile::Core);
        assert_eq!(report.standing, DoctorStanding::PartialAlive);
        assert!(!report.is_blocked());
    }

    #[test]
    fn vision_profile_tracks_feature_closure() {
        let report = CompatDoctor::run(DoctorProfile::Vision2030);
        let all_features = cfg!(feature = "formats")
            && cfg!(feature = "strict")
            && cfg!(feature = "wasm4pm");
        assert_eq!(
            report.standing,
            if all_features {
                DoctorStanding::PartialAlive
            } else {
                DoctorStanding::Blocked
            }
        );
    }

    #[test]
    fn fingerprint_is_deterministic_lowercase_blake3() {
        let report = CompatDoctor::run(DoctorProfile::Boundary);
        let first = report.fingerprint().unwrap();
        let second = report.fingerprint().unwrap();
        assert_eq!(first, second);
        assert_eq!(first.len(), 64);
        assert!(
            first
                .chars()
                .all(|ch| ch.is_ascii_digit() || ('a'..='f').contains(&ch))
        );
    }

    #[test]
    fn engine_intents_never_route_to_compat() {
        let plan = CompatDoctor::plan([
            Intent::Discover,
            Intent::Conformance,
            Intent::Replay,
            Intent::Optimize,
        ]);
        assert!(plan
            .decisions
            .iter()
            .all(|decision| decision.target == RouteTarget::Wasm4pm));
    }

    #[test]
    fn standing_verification_is_always_external() {
        let plan = CompatDoctor::plan([Intent::VerifyStanding]);
        assert_eq!(plan.decisions.len(), 1);
        assert_eq!(
            plan.decisions[0].target,
            RouteTarget::ExternalVerifier
        );
        assert_eq!(plan.decisions[0].state, RouteState::Routed);
    }

    #[test]
    fn missing_feature_produces_one_reversible_repair() {
        let intent = if cfg!(feature = "strict") {
            Intent::Graduate
        } else {
            Intent::StrictAdmit
        };
        let plan = CompatDoctor::plan([intent]);
        if plan.standing == DoctorStanding::Blocked {
            assert_eq!(plan.repairs.len(), 1);
            assert!(plan.repairs[0].reversible);
        }
    }

    #[test]
    fn diagnostic_catalog_is_unique_and_resolvable() {
        let catalog = diagnostic_catalog();
        let unique: BTreeSet<_> = catalog.iter().map(|item| item.code.as_str()).collect();
        assert_eq!(catalog.len(), unique.len());
        let first = &catalog[0];
        assert_eq!(explain_diagnostic(&first.code), Some(first.clone()));
        assert_eq!(explain_diagnostic(&first.name), Some(first.clone()));
    }

    #[test]
    fn route_plan_canonicalization_replays() {
        let first = CompatDoctor::plan([Intent::Admit, Intent::VerifyStanding]);
        let second = CompatDoctor::plan([Intent::Admit, Intent::VerifyStanding]);
        assert_eq!(first.canonical_json().unwrap(), second.canonical_json().unwrap());
        assert_eq!(first.fingerprint().unwrap(), second.fingerprint().unwrap());
    }
}
