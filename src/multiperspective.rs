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
/// Each variant names one of the four canonical analysis dimensions in the
/// Mannhardt et al. (2016) balanced multi-perspective conformance framework.
/// This is structure only; it names a perspective but does not analyse it.
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
/// Zero-sized. Used in `PhantomData` positions to assert that a value is typed
/// against the control-flow perspective of the Mannhardt et al. (2016) framework.
///
/// Structure-only perspective marker. Graduate to `wasm4pm` when control-flow
/// alignment cost computation is required.
pub struct ControlFlowPerspective;

/// Marker that evidence covers the data perspective.
///
/// Zero-sized. Asserts that a value is typed against the data perspective
/// (event/object attributes) in the multi-perspective framework.
///
/// Structure-only perspective marker. Graduate to `wasm4pm` when data-condition
/// guard evaluation is required.
pub struct DataPerspective;

/// Marker that evidence covers the resource perspective.
///
/// Zero-sized. Asserts that a value is typed against the resource perspective
/// (`org:resource` or equivalent organisational attribute) in the multi-perspective
/// framework.
///
/// Structure-only perspective marker. Graduate to `wasm4pm` when resource-based
/// conformance cost computation is required.
pub struct ResourcePerspective;

/// Marker that evidence covers the time perspective.
///
/// Zero-sized. Asserts that a value is typed against the temporal perspective
/// (timestamps, durations, sojourn times) in the multi-perspective framework.
///
/// Structure-only perspective marker. Graduate to `wasm4pm` when temporal
/// conformance checking (e.g. temporal profile comparison) is required.
pub struct TimePerspective;

/// Evidence enriched with perspective markers.
///
/// `MultiPerspectiveEvidence<T, Perspectives>` wraps an inner value `T` and
/// threads a perspective combination (e.g. `PerspectiveCombination<ControlFlowPerspective,
/// DataPerspective>`) through the type system as a zero-sized phantom.
///
/// The `Perspectives` type parameter is intentionally open — callers compose
/// [`PerspectiveCombination`] types to declare which perspectives are present.
///
/// This is structure only; no engine logic belongs here.
pub struct MultiPerspectiveEvidence<T, Perspectives> {
    /// The inner evidence value.
    pub inner: T,
    _perspectives: PhantomData<Perspectives>,
}

impl<T, Perspectives> MultiPerspectiveEvidence<T, Perspectives> {
    /// Wrap a value with the given perspective combination marker.
    pub fn new(inner: T) -> Self {
        Self { inner, _perspectives: PhantomData }
    }
}

/// A combination of two perspectives for multi-perspective analysis.
///
/// Used as the `Perspectives` parameter in [`MultiPerspectiveEvidence`] when
/// evidence covers exactly two perspectives. For three or four perspectives,
/// nest: `PerspectiveCombination<A, PerspectiveCombination<B, C>>`.
///
/// Zero-sized.
pub struct PerspectiveCombination<A, B> {
    _a: PhantomData<A>,
    _b: PhantomData<B>,
}
