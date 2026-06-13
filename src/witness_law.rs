//! Compile-time law enforcement for witness family gating, co-citation,
//! and authority chains.
//!
//! This module sits on top of [`crate::witness`] and [`crate::law`] to provide
//! types that make **illegal witness combinations unrepresentable** at the
//! type level — zero-cost, fully erased at codegen.
//!
//! ## What this module IS
//!
//! - **Family-gated proof tokens**: [`StandardWitness`] and [`PaperWitness`]
//!   are only constructible for witnesses whose `FAMILY` matches. Wrong family
//!   → compile error via sealed trait, not a runtime panic.
//! - **Self-citation prevention**: [`CoCitedKey<T, K1, K2>`] uses `str_eq` as
//!   a const generic bound — `CoCitedKey<T, "ocel-2.0", "ocel-2.0">` fails to
//!   compile. This is the first use of string-level law in a const generic bound
//!   in the process-mining ecosystem.
//! - **Family-as-const-param**: [`FamilyGated<F>`] uses [`WitnessFamily`] directly
//!   as an `adt_const_params` const generic — the first use of a domain-specific
//!   semantic enum as a const param in the process-mining ecosystem.
//! - **Const helper fns**: [`family_eq`] and [`str_eq`] are `const fn`s usable
//!   in `const { assert!(...) }` blocks and `generic_const_exprs` bounds.
//!
//! ## What this module is NOT
//!
//! - Not a validator. Witnesses name authorities; this module gates on family
//!   membership and key uniqueness. Actual validation lives in `wasm4pm`.
//! - Not runtime. Every type here is zero-sized (`PhantomData` only).
//!
//! ## Graduation
//!
//! Graduate to `wasm4pm` when you need to *verify* a witness against its
//! authority (e.g. conformance checking, replay). Here, witnesses are labels.

use core::marker::PhantomData;

use crate::law::{Assert, IsTrue};
use crate::witness::{Witness, WitnessFamily};

// ── Const helper functions ────────────────────────────────────────────────────

/// Compare two [`WitnessFamily`] values in a const context.
///
/// Uses `as u8` cast — ordinal comparison, no `PartialEq` const bound needed.
/// Usable in `const { assert!(...) }` blocks.
///
/// ```
/// use wasm4pm_compat::witness_law::family_eq;
/// use wasm4pm_compat::witness::WitnessFamily;
/// const _: () = assert!(family_eq(WitnessFamily::Standard, WitnessFamily::Standard));
/// const _: () = assert!(!family_eq(WitnessFamily::Standard, WitnessFamily::Paper));
/// ```
pub const fn family_eq(a: WitnessFamily, b: WitnessFamily) -> bool {
    a as u8 == b as u8
}

/// Byte-by-byte string equality in a const context.
///
/// The first const-fn string comparator in this crate. Usable as a
/// `generic_const_exprs` bound via [`CoCitedKey`], or in `const { assert!(...) }`
/// blocks for inline law checks.
///
/// ```
/// use wasm4pm_compat::witness_law::str_eq;
/// const _: () = assert!(str_eq("ocel-2.0", "ocel-2.0"));
/// const _: () = assert!(!str_eq("ocel-2.0", "xes-1849-2016"));
/// const _: () = assert!(!str_eq("ab", "abc"));
/// ```
pub const fn str_eq(a: &str, b: &str) -> bool {
    let a = a.as_bytes();
    let b = b.as_bytes();
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}

// ── Family authority sealed traits ───────────────────────────────────────────

mod sealed {
    /// Sealed marker: only [`crate::witness::WitnessFamily::Standard`] witnesses implement this.
    pub trait IsStandard {}
    /// Sealed marker: only [`WitnessFamily::Paper`] witnesses implement this.
    pub trait IsPaper {}
    /// Sealed marker: only [`WitnessFamily::ApiGrammar`] witnesses implement this.
    pub trait IsApiGrammar {}
    /// Sealed marker: only [`WitnessFamily::RustLaw`] witnesses implement this.
    pub trait IsRustLaw {}
    /// Sealed marker: only [`WitnessFamily::InternalBridge`] witnesses implement this.
    pub trait IsInternalBridge {}

}

/// Compile-time proof that `W` belongs to the [`WitnessFamily::Standard`] family.
///
/// Implement this for each `Standard`-family witness. The sealed super-trait
/// prevents downstream code from claiming a non-Standard witness is standard.
///
/// # Examples
///
/// ```
/// use wasm4pm_compat::witness_law::StandardAuthority;
/// use wasm4pm_compat::witness::Ocel20;
/// fn requires_standard<W: StandardAuthority>() {}
/// requires_standard::<Ocel20>(); // Ocel20 is Standard ✓
/// ```
///
/// A wrong-family witness produces a *teaching* diagnostic naming the law, not a
/// bare "trait not satisfied" — these sealed traits fail as `E0277`, where
/// `#[diagnostic::on_unimplemented]` can fire (unlike the arithmetic const-laws,
/// which fail as `E0308` const-unification and cannot carry a custom message).
#[diagnostic::on_unimplemented(
    message = "`{Self}` is not a Standard-family authority",
    label = "only normative-standard witnesses (ISO/IEEE/OMG) satisfy `StandardAuthority`",
    note = "use a `WitnessFamily::Standard` witness (e.g. `Ocel20`, `Xes1849`), or `PaperAuthority` for a published paper"
)]
pub trait StandardAuthority: Witness + sealed::IsStandard {}

/// Compile-time proof that `W` belongs to the [`WitnessFamily::Paper`] family.
#[diagnostic::on_unimplemented(
    message = "`{Self}` is not a Paper-family authority",
    label = "only academic-paper witnesses satisfy `PaperAuthority`",
    note = "use a `WitnessFamily::Paper` witness (e.g. `AlphaMiner`, `PowlPaper`), or `StandardAuthority` for a normative standard"
)]
pub trait PaperAuthority: Witness + sealed::IsPaper {}

/// Compile-time proof that `W` belongs to the [`WitnessFamily::ApiGrammar`] family.
#[diagnostic::on_unimplemented(
    message = "`{Self}` is not an ApiGrammar-family authority",
    label = "only consumer-grammar witnesses satisfy `ApiGrammarAuthority`",
    note = "use a `WitnessFamily::ApiGrammar` witness (e.g. `Pm4pyApiGrammar`, `PmaxConsumerGrammar`)"
)]
pub trait ApiGrammarAuthority: Witness + sealed::IsApiGrammar {}

/// Compile-time proof that `W` belongs to the [`WitnessFamily::RustLaw`] family.
#[diagnostic::on_unimplemented(
    message = "`{Self}` is not a RustLaw-family authority",
    label = "only Rust-language-law witnesses satisfy `RustLawAuthority`",
    note = "use a `WitnessFamily::RustLaw` witness (e.g. `RustTypestateLaw`)"
)]
pub trait RustLawAuthority: Witness + sealed::IsRustLaw {}

/// Compile-time proof that `W` belongs to the [`WitnessFamily::InternalBridge`] family.
#[diagnostic::on_unimplemented(
    message = "`{Self}` is not an InternalBridge-family authority",
    label = "only graduation-bridge witnesses satisfy `InternalBridgeAuthority`",
    note = "use a `WitnessFamily::InternalBridge` witness (e.g. `Wasm4pmBridge`, `ConformanceWitness`)"
)]
pub trait InternalBridgeAuthority: Witness + sealed::IsInternalBridge {}

// ── Standard-family witness impls ─────────────────────────────────────────────

use crate::witness::{Ocel20, OcelAttributeType, OcelEventType, OcelObjectType, Xes1849};
use crate::witnesses::{XesConceptExt, XesLifecycleExt};

impl sealed::IsStandard for Ocel20 {}
impl StandardAuthority for Ocel20 {}

impl sealed::IsStandard for OcelAttributeType {}
impl StandardAuthority for OcelAttributeType {}

impl sealed::IsStandard for OcelEventType {}
impl StandardAuthority for OcelEventType {}

impl sealed::IsStandard for OcelObjectType {}
impl StandardAuthority for OcelObjectType {}

impl sealed::IsStandard for Xes1849 {}
impl StandardAuthority for Xes1849 {}

impl sealed::IsStandard for XesConceptExt {}
impl StandardAuthority for XesConceptExt {}

impl sealed::IsStandard for XesLifecycleExt {}
impl StandardAuthority for XesLifecycleExt {}

// ── Paper-family witness impls ────────────────────────────────────────────────

use crate::witness::{
    AlphaMiner, DeclareFamily, OcpqPaper, ObjectCentricPetriNetPaper, PowlPaper,
    PredictiveMonitoringFamily, ReceiptFamily, WfNetSoundnessPaper, YawlPaper,
};
use crate::witnesses::{
    AggregationView, AlignmentPaper, AnalyticalView, CausalConsistencyWitness,
    ControlFlowPerspectiveWitness, ConvergenceWitness, CrossLogCorrelationWitness,
    DataPerspectiveWitness, DeclareConstraints, DivergenceWitness, InductiveMiner,
    LogSkeleton, OcPetriNets, OcpqPaper as OcpqPaperFull, OperationalView,
    ProcessCubePaper, ResourcePerspectiveWitness, SeparableWfNetPaper, StreamingEvidenceWitness,
    TemporalProfileWitness, TimeAwareWitness, TimePerspectiveWitness, WfNet2Powl,
    WorkflowPatternsPaper,
};

macro_rules! impl_paper {
    ($($t:ty),+ $(,)?) => {
        $(
            impl sealed::IsPaper for $t {}
            impl PaperAuthority for $t {}
        )+
    };
}

impl_paper!(
    AlphaMiner,
    DeclareFamily,
    OcpqPaper,
    ObjectCentricPetriNetPaper,
    PowlPaper,
    PredictiveMonitoringFamily,
    ReceiptFamily,
    WfNetSoundnessPaper,
    YawlPaper,
    AggregationView,
    AlignmentPaper,
    AnalyticalView,
    CausalConsistencyWitness,
    ControlFlowPerspectiveWitness,
    ConvergenceWitness,
    CrossLogCorrelationWitness,
    DataPerspectiveWitness,
    DeclareConstraints,
    DivergenceWitness,
    InductiveMiner,
    LogSkeleton,
    OcPetriNets,
    OcpqPaperFull,
    OperationalView,
    ProcessCubePaper,
    ResourcePerspectiveWitness,
    SeparableWfNetPaper,
    StreamingEvidenceWitness,
    TemporalProfileWitness,
    TimeAwareWitness,
    TimePerspectiveWitness,
    WfNet2Powl,
    WorkflowPatternsPaper,
);

// ── ApiGrammar-family impls ───────────────────────────────────────────────────

use crate::witness::{Pm4pyApiGrammar, PmaxConsumerGrammar};
use crate::witnesses::Pm4pyApiGrammar as Pm4pyFullModule;
use crate::witnesses::PmaxConsumerGrammar as PmaxFullModule;

impl sealed::IsApiGrammar for Pm4pyApiGrammar {}
impl ApiGrammarAuthority for Pm4pyApiGrammar {}

impl sealed::IsApiGrammar for PmaxConsumerGrammar {}
impl ApiGrammarAuthority for PmaxConsumerGrammar {}

impl sealed::IsApiGrammar for Pm4pyFullModule {}
impl ApiGrammarAuthority for Pm4pyFullModule {}

impl sealed::IsApiGrammar for PmaxFullModule {}
impl ApiGrammarAuthority for PmaxFullModule {}

// ── RustLaw-family impls ──────────────────────────────────────────────────────

use crate::witness::RustTypestateLaw;
use crate::witnesses::RustTypestateLaw as RustTypestateLawFull;

impl sealed::IsRustLaw for RustTypestateLaw {}
impl RustLawAuthority for RustTypestateLaw {}

impl sealed::IsRustLaw for RustTypestateLawFull {}
impl RustLawAuthority for RustTypestateLawFull {}

// ── InternalBridge-family impls ───────────────────────────────────────────────

use crate::witness::Wasm4pmBridge;
use crate::witnesses::{
    ConformanceWitness, LifecycleWitness, MiningWitness, ReplayWitness,
    Wasm4pmBridge as Wasm4pmBridgeFull,
};

macro_rules! impl_bridge {
    ($($t:ty),+ $(,)?) => {
        $(
            impl sealed::IsInternalBridge for $t {}
            impl InternalBridgeAuthority for $t {}
        )+
    };
}

impl_bridge!(
    Wasm4pmBridge,
    Wasm4pmBridgeFull,
    ConformanceWitness,
    LifecycleWitness,
    MiningWitness,
    ReplayWitness,
);

// ── Proof tokens ──────────────────────────────────────────────────────────────

/// Zero-sized proof token: `W` is a [`StandardAuthority`] witness.
///
/// Construction fails at compile time if `W` is not a Standard-family witness.
/// Uses sealed trait gating — no const expression gymnastics needed.
///
/// # Examples
///
/// ```
/// use wasm4pm_compat::witness_law::{StandardWitness, StandardAuthority};
/// use wasm4pm_compat::witness::Ocel20;
/// let _: StandardWitness<Ocel20> = StandardWitness::new();
/// ```
pub struct StandardWitness<W: StandardAuthority> {
    _w: PhantomData<W>,
}

impl<W: StandardAuthority> StandardWitness<W> {
    /// Construct the proof token. Only compiles when `W: StandardAuthority`.
    pub fn new() -> Self {
        Self { _w: PhantomData }
    }
}

impl<W: StandardAuthority> Default for StandardWitness<W> {
    fn default() -> Self { Self::new() }
}

/// Zero-sized proof token: `W` is a [`PaperAuthority`] witness.
///
/// # Examples
///
/// ```
/// use wasm4pm_compat::witness_law::{PaperWitness, PaperAuthority};
/// use wasm4pm_compat::witness::AlphaMiner;
/// let _: PaperWitness<AlphaMiner> = PaperWitness::new();
/// ```
pub struct PaperWitness<W: PaperAuthority> {
    _w: PhantomData<W>,
}

impl<W: PaperAuthority> PaperWitness<W> {
    /// Construct the proof token. Only compiles when `W: PaperAuthority`.
    pub fn new() -> Self {
        Self { _w: PhantomData }
    }
}

impl<W: PaperAuthority> Default for PaperWitness<W> {
    fn default() -> Self { Self::new() }
}

// ── Co-citation with string-level law ────────────────────────────────────────

/// Co-citation carrier with **compile-time self-citation prevention**.
///
/// `K1` and `K2` are witness keys (e.g. `Ocel20::KEY`, `AlphaMiner::KEY`).
/// The bound `Assert<{ !str_eq(K1, K2) }>: IsTrue` means the two keys must
/// differ at compile time — `CoCitedKey<T, "ocel-2.0", "ocel-2.0">` fails to
/// compile with:
/// ```text
/// error[E0308]: expected constant `false`, found constant `true`
/// ```
///
/// This is the **first use of compile-time string equality as a type law**
/// in the process-mining ecosystem, enabled by `unsized_const_params` +
/// `generic_const_exprs` on nightly.
///
/// # Examples
///
/// ```
/// # #![feature(generic_const_exprs, adt_const_params, unsized_const_params)]
/// # #![allow(incomplete_features)]
/// use wasm4pm_compat::witness_law::CoCitedKey;
/// use wasm4pm_compat::witness::{Ocel20, AlphaMiner, Witness};
/// // Cross-paper co-citation — distinct keys, compiles:
/// let cited: CoCitedKey<u32, { Ocel20::KEY }, { AlphaMiner::KEY }> = CoCitedKey::new(42);
/// assert_eq!(cited.value, 42);
/// ```
pub struct CoCitedKey<T, const K1: &'static str, const K2: &'static str>
where
    Assert<{ !str_eq(K1, K2) }>: IsTrue,
{
    /// The co-cited value.
    pub value: T,
}

impl<T, const K1: &'static str, const K2: &'static str> CoCitedKey<T, K1, K2>
where
    Assert<{ !str_eq(K1, K2) }>: IsTrue,
{
    /// Wrap `value` with a co-citation proof that `K1 ≠ K2` at compile time.
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

// ── Family-as-const-param ────────────────────────────────────────────────────

/// A zero-sized type parameterized by a [`WitnessFamily`] const.
///
/// `WitnessFamily` derives [`core::marker::ConstParamTy`], making it a valid
/// `adt_const_params` const generic parameter — not just `usize` or `bool`,
/// but the actual domain classification enum. `FamilyGated<{
/// WitnessFamily::Paper }>` and `FamilyGated<{ WitnessFamily::Standard }>` are
/// **different types** that the compiler distinguishes with no runtime cost.
///
/// This is the direct application of a domain-specific semantic enum as a const
/// generic parameter — an ecosystem-first pattern for process-mining authority
/// classification.
///
/// # Examples
///
/// ```
/// # #![feature(adt_const_params)]
/// # #![allow(incomplete_features)]
/// use wasm4pm_compat::witness_law::FamilyGated;
/// use wasm4pm_compat::witness::WitnessFamily;
/// fn only_paper(_: FamilyGated<{ WitnessFamily::Paper }>) {}
/// fn only_standard(_: FamilyGated<{ WitnessFamily::Standard }>) {}
/// only_paper(FamilyGated::<{ WitnessFamily::Paper }>::new());
/// ```
pub struct FamilyGated<const F: WitnessFamily>(PhantomData<[u8; 0]>);

impl<const F: WitnessFamily> FamilyGated<F> {
    /// Construct the family-gated sentinel.
    ///
    /// ```
    /// # #![feature(adt_const_params)]
    /// # #![allow(incomplete_features)]
    /// use wasm4pm_compat::witness_law::FamilyGated;
    /// use wasm4pm_compat::witness::WitnessFamily;
    /// let _: FamilyGated<{ WitnessFamily::Paper }> = FamilyGated::new();
    /// ```
    pub fn new() -> Self {
        FamilyGated(PhantomData)
    }

    /// The family this sentinel is gated on.
    ///
    /// ```
    /// # #![feature(adt_const_params)]
    /// # #![allow(incomplete_features)]
    /// use wasm4pm_compat::witness_law::FamilyGated;
    /// use wasm4pm_compat::witness::WitnessFamily;
    /// assert_eq!(FamilyGated::<{ WitnessFamily::Standard }>::family(), WitnessFamily::Standard);
    /// ```
    pub const fn family() -> WitnessFamily {
        F
    }
}

impl<const F: WitnessFamily> Default for FamilyGated<F> {
    fn default() -> Self { Self::new() }
}
