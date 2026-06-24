//! # Multi-Perspective Process Evidence
//!
//! Typed shapes for multi-perspective process analysis.
//! The four perspectives of process mining are control-flow, data,
//! resource, and time. This module provides witness markers for each.
//!
//! ## What this module IS
//!
//! - Zero-cost perspective marker types for use in generic bounds and `PhantomData`
//!   positions, following Mannhardt et al. (2016) "Balanced Multi-Perspective
//!   Checking of Process Conformance".
//! - A `MultiPerspectiveEvidence<T, Perspectives>` carrier that threads a
//!   perspective combination through the type system.
//! - Structure only. No alignment computation, no conformance checking.
//!
//! ## What this module is NOT
//!
//! - Not a conformance checker. Per-perspective cost weighting and multi-perspective
//!   alignment execution graduate to `wasm4pm`.
//! - Not a runtime value store. All perspective markers are zero-sized.
//!
//! ## Paper anchor
//!
//! Mannhardt, F., de Leoni, M., Reijers, H. A., & van der Aalst, W. M. P. (2016).
//! "Balanced Multi-Perspective Checking of Process Conformance."
//! *Computing*, 98(4), 407–437.
//!
//! ## Graduation
//!
//! When you need to *compute* per-perspective alignment costs or *check* multi-
//! perspective conformance, graduate to `wasm4pm`.

use core::marker::PhantomData;

/// The four classic process mining perspectives (van der Aalst).
///
/// ### Representation
/// Each variant names one of the four canonical analysis dimensions in the
/// Mannhardt et al. (2016) balanced multi-perspective conformance framework.
///
/// ### Structure-only
/// This is structure only; it names a perspective but does not analyse it.
/// Zero-sized marker variants or light enum discriminant.
///
/// ### Graduation
/// When you need to *compute* per-perspective alignment costs or *check* multi-
/// perspective conformance, graduate to `wasm4pm`.
///
/// # Examples
///
/// ```
/// use wasm4pm_compat::multiperspective::ProcessPerspective;
///
/// let p = ProcessPerspective::ControlFlow;
/// assert_eq!(format!("{}", p), "control-flow");
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum ProcessPerspective {
    /// The ordering and routing of activities (what happens and in what order).
    ControlFlow,
    /// Data attributes carried by events and objects (what data is recorded).
    Data,
    /// Organisational resources — who performs what activity.
    Resource,
    /// Temporal information: timestamps, durations, waiting times.
    Time,
}

impl core::fmt::Display for ProcessPerspective {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ControlFlow => write!(f, "control-flow"),
            Self::Data => write!(f, "data"),
            Self::Resource => write!(f, "resource"),
            Self::Time => write!(f, "time"),
        }
    }
}

/// Marker that evidence covers the control-flow perspective.
///
/// ### Representation
/// Zero-sized marker type representing the control-flow perspective, which focuses on the ordering, routing, and synchronization of activities.
/// Used in `PhantomData` positions to assert that a value is typed
/// against the control-flow perspective of the Mannhardt et al. (2016) framework.
///
/// ### Structure-only
/// Structure-only perspective marker. No alignment computation, no conformance checking, and zero runtime storage cost.
///
/// ### Graduation
/// Graduate to `wasm4pm` when control-flow alignment cost computation is required.
///
/// # Examples
///
/// ```
/// use wasm4pm_compat::multiperspective::ControlFlowPerspective;
/// use core::marker::PhantomData;
///
/// struct MyEvidence<P>(PhantomData<P>);
/// let _ev: MyEvidence<ControlFlowPerspective> = MyEvidence(PhantomData);
/// ```
pub struct ControlFlowPerspective;

/// Marker that evidence covers the data perspective.
///
/// ### Representation
/// Zero-sized marker type representing the data perspective, which focuses on case and event attributes, variable values, and guards.
/// Asserts that a value is typed against the data perspective
/// (event/object attributes) in the multi-perspective framework.
///
/// ### Structure-only
/// Structure-only perspective marker. No condition evaluation or guard checking, and zero runtime storage cost.
///
/// ### Graduation
/// Graduate to `wasm4pm` when data-condition guard evaluation is required.
///
/// # Examples
///
/// ```
/// use wasm4pm_compat::multiperspective::DataPerspective;
/// use core::marker::PhantomData;
///
/// struct MyEvidence<P>(PhantomData<P>);
/// let _ev: MyEvidence<DataPerspective> = MyEvidence(PhantomData);
/// ```
pub struct DataPerspective;

/// Marker that evidence covers the resource perspective.
///
/// ### Representation
/// Zero-sized marker type representing the resource perspective, focusing on organizational roles, resources, departments, and performers.
/// Asserts that a value is typed against the resource perspective
/// (`org:resource` or equivalent organisational attribute) in the multi-perspective
/// framework.
///
/// ### Structure-only
/// Structure-only perspective marker. No resource allocation checks or conformance cost evaluation, and zero runtime storage cost.
///
/// ### Graduation
/// Graduate to `wasm4pm` when resource-based conformance cost computation is required.
///
/// # Examples
///
/// ```
/// use wasm4pm_compat::multiperspective::ResourcePerspective;
/// use core::marker::PhantomData;
///
/// struct MyEvidence<P>(PhantomData<P>);
/// let _ev: MyEvidence<ResourcePerspective> = MyEvidence(PhantomData);
/// ```
pub struct ResourcePerspective;

/// Marker that evidence covers the time perspective.
///
/// ### Representation
/// Zero-sized marker type representing the time perspective, focusing on timestamps, activity durations, waiting times, and performance metrics.
/// Asserts that a value is typed against the temporal perspective
/// (timestamps, durations, sojourn times) in the multi-perspective framework.
///
/// ### Structure-only
/// Structure-only perspective marker. No temporal profile validation, sojourn time checks, or performance calculations.
///
/// ### Graduation
/// Graduate to `wasm4pm` when temporal conformance checking (e.g. temporal profile comparison) is required.
///
/// # Examples
///
/// ```
/// use wasm4pm_compat::multiperspective::TimePerspective;
/// use core::marker::PhantomData;
///
/// struct MyEvidence<P>(PhantomData<P>);
/// let _ev: MyEvidence<TimePerspective> = MyEvidence(PhantomData);
/// ```
pub struct TimePerspective;

/// Evidence enriched with perspective markers.
///
/// ### Representation
/// `MultiPerspectiveEvidence<T, Perspectives>` wraps an inner value `T` and
/// threads a perspective combination (e.g. `PerspectiveCombination<ControlFlowPerspective,
/// DataPerspective>`) through the type system as a zero-sized phantom.
///
/// The `Perspectives` type parameter is intentionally open — callers compose
/// [`PerspectiveCombination`] types to declare which perspectives are present.
///
/// ### Structure-only
/// This is structure only; no engine logic or conformance algorithms belong here.
/// It introduces zero runtime cost or extra storage overhead.
///
/// ### Graduation
/// Graduate to `wasm4pm` when you need to calculate multi-perspective alignment
/// scores or run multi-perspective conformance checking.
///
/// # Examples
///
/// ```
/// use wasm4pm_compat::multiperspective::{MultiPerspectiveEvidence, ControlFlowPerspective};
///
/// let evidence = MultiPerspectiveEvidence::<_, ControlFlowPerspective>::new(42);
/// assert_eq!(evidence.inner, 42);
/// ```
pub struct MultiPerspectiveEvidence<T, Perspectives> {
    /// The inner evidence value.
    pub inner: T,
    _perspectives: PhantomData<Perspectives>,
}

impl<T, Perspectives> MultiPerspectiveEvidence<T, Perspectives> {
    /// Wrap a value with the given perspective combination marker.
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            _perspectives: PhantomData,
        }
    }
}

/// A combination of two perspectives for multi-perspective analysis.
///
/// ### Representation
/// Used as the `Perspectives` parameter in [`MultiPerspectiveEvidence`] when
/// evidence covers exactly two perspectives. For three or four perspectives,
/// nest: `PerspectiveCombination<A, PerspectiveCombination<B, C>>`.
///
/// ### Structure-only
/// Zero-sized marker type for type-level generic combination, carrying no runtime representation
/// or data storage overhead.
///
/// ### Graduation
/// Graduate to `wasm4pm` to analyze processes along the combined dimensions.
///
/// # Examples
///
/// ```
/// use wasm4pm_compat::multiperspective::{
///     PerspectiveCombination, ControlFlowPerspective, DataPerspective, MultiPerspectiveEvidence
/// };
///
/// type FlowAndData = PerspectiveCombination<ControlFlowPerspective, DataPerspective>;
/// let evidence = MultiPerspectiveEvidence::<_, FlowAndData>::new("evidence");
/// assert_eq!(evidence.inner, "evidence");
/// ```
pub struct PerspectiveCombination<A, B> {
    _a: PhantomData<A>,
    _b: PhantomData<B>,
}

pub struct ParityComparer;

impl ParityComparer {
    pub fn assert_epsilon_close(actual: f64, expected: f64) {
        let diff = (actual - expected).abs();
        assert!(
            diff < 1e-6,
            "Parity violation: actual {}, expected {}, diff {} (exceeds epsilon 1e-6)",
            actual,
            expected,
            diff
        );
    }
}

use crate::law::{Between01, IsTrue, Require};

/// A perspective weight represented as a type-level fraction in `[0, 1]`.
///
/// Enforces at compile time that the weight is a valid rational number in `[0, 1]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PerspectiveWeight<const NUM: u64, const DEN: u64>
where
    Require<{ DEN > 0 }>: IsTrue,
    Require<{ NUM <= DEN }>: IsTrue,
{
    _inner: Between01<NUM, DEN>,
}

impl<const NUM: u64, const DEN: u64> Default for PerspectiveWeight<NUM, DEN>
where
    Require<{ DEN > 0 }>: IsTrue,
    Require<{ NUM <= DEN }>: IsTrue,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<const NUM: u64, const DEN: u64> PerspectiveWeight<NUM, DEN>
where
    Require<{ DEN > 0 }>: IsTrue,
    Require<{ NUM <= DEN }>: IsTrue,
{
    /// Construct a new perspective weight.
    pub const fn new() -> Self {
        Self {
            _inner: Between01::new(),
        }
    }

    /// Retrieve the weight numerator.
    pub const fn num(&self) -> u64 {
        NUM
    }

    /// Retrieve the weight denominator.
    pub const fn den(&self) -> u64 {
        DEN
    }

    /// Convert the type-level fraction to a runtime `f64` value.
    pub fn to_f64(&self) -> f64 {
        NUM as f64 / DEN as f64
    }
}

/// A configuration of weights for the four process mining perspectives.
///
/// Enforces at compile time that:
/// 1. Each weight is individually in `[0, 1]`.
/// 2. The sum of all four weights is <= 1.0.
pub struct MultiPerspectiveWeightConfig<
    const CF_N: u64,
    const CF_D: u64, // Control-Flow Weight
    const D_N: u64,
    const D_D: u64, // Data Weight
    const R_N: u64,
    const R_D: u64, // Resource Weight
    const T_N: u64,
    const T_D: u64, // Time Weight
> where
    Require<{ CF_D > 0 }>: IsTrue,
    Require<{ CF_N <= CF_D }>: IsTrue,
    Require<{ D_D > 0 }>: IsTrue,
    Require<{ D_N <= D_D }>: IsTrue,
    Require<{ R_D > 0 }>: IsTrue,
    Require<{ R_N <= R_D }>: IsTrue,
    Require<{ T_D > 0 }>: IsTrue,
    Require<{ T_N <= T_D }>: IsTrue,
    // exact rational addition: CF_N/CF_D + D_N/D_D + R_N/R_D + T_N/T_D <= 1
    Require<
        {
            (CF_N * D_D * R_D * T_D)
                + (D_N * CF_D * R_D * T_D)
                + (R_N * CF_D * D_D * T_D)
                + (T_N * CF_D * D_D * R_D)
                <= (CF_D * D_D * R_D * T_D)
        },
    >: IsTrue,
{
    pub cf: PerspectiveWeight<CF_N, CF_D>,
    pub data: PerspectiveWeight<D_N, D_D>,
    pub resource: PerspectiveWeight<R_N, R_D>,
    pub time: PerspectiveWeight<T_N, T_D>,
}

impl<
        const CF_N: u64,
        const CF_D: u64,
        const D_N: u64,
        const D_D: u64,
        const R_N: u64,
        const R_D: u64,
        const T_N: u64,
        const T_D: u64,
    > MultiPerspectiveWeightConfig<CF_N, CF_D, D_N, D_D, R_N, R_D, T_N, T_D>
where
    Require<{ CF_D > 0 }>: IsTrue,
    Require<{ CF_N <= CF_D }>: IsTrue,
    Require<{ D_D > 0 }>: IsTrue,
    Require<{ D_N <= D_D }>: IsTrue,
    Require<{ R_D > 0 }>: IsTrue,
    Require<{ R_N <= R_D }>: IsTrue,
    Require<{ T_D > 0 }>: IsTrue,
    Require<{ T_N <= T_D }>: IsTrue,
    Require<
        {
            (CF_N * D_D * R_D * T_D)
                + (D_N * CF_D * R_D * T_D)
                + (R_N * CF_D * D_D * T_D)
                + (T_N * CF_D * D_D * R_D)
                <= (CF_D * D_D * R_D * T_D)
        },
    >: IsTrue,
{
    pub const fn new() -> Self {
        Self {
            cf: PerspectiveWeight::new(),
            data: PerspectiveWeight::new(),
            resource: PerspectiveWeight::new(),
            time: PerspectiveWeight::new(),
        }
    }
}

impl<
        const CF_N: u64,
        const CF_D: u64,
        const D_N: u64,
        const D_D: u64,
        const R_N: u64,
        const R_D: u64,
        const T_N: u64,
        const T_D: u64,
    > Default for MultiPerspectiveWeightConfig<CF_N, CF_D, D_N, D_D, R_N, R_D, T_N, T_D>
where
    Require<{ CF_D > 0 }>: IsTrue,
    Require<{ CF_N <= CF_D }>: IsTrue,
    Require<{ D_D > 0 }>: IsTrue,
    Require<{ D_N <= D_D }>: IsTrue,
    Require<{ R_D > 0 }>: IsTrue,
    Require<{ R_N <= R_D }>: IsTrue,
    Require<{ T_D > 0 }>: IsTrue,
    Require<{ T_N <= T_D }>: IsTrue,
    Require<
        {
            (CF_N * D_D * R_D * T_D)
                + (D_N * CF_D * R_D * T_D)
                + (R_N * CF_D * D_D * T_D)
                + (T_N * CF_D * D_D * R_D)
                <= (CF_D * D_D * R_D * T_D)
        },
    >: IsTrue,
{
    fn default() -> Self {
        Self::new()
    }
}
