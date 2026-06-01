//! # Temporal Ordering Law
//!
//! Typed shapes for temporal event ordering and temporal profiles.
//!
//! ## What this is
//!
//! The structural vocabulary for reasoning about *when* events occur relative
//! to each other — temporal order relationships, profile shapes over traces,
//! and evidence wrappers that carry a temporal context. These are the shapes
//! that a temporal conformance or temporal profile analysis engine produces
//! and consumes.
//!
//! ## What this is not
//!
//! The temporal analysis engine. Computing sojourn times, deriving temporal
//! profiles from event logs, performing temporal conformance checking against
//! a reference model, or detecting temporal anomalies all graduate to
//! `wasm4pm`. This module carries the shapes those operations produce.
//!
//! ## Paper authority
//!
//! The temporal profile conformance framework is described in:
//!
//! Adriansyah, A., Munoz-Gama, J., Carmona, J., van Dongen, B., van der Aalst,
//! W.M.P. (2015). *Measuring Precision of Modeled Behavior.* Information
//! Systems and e-Business Management.
//!
//! Also see van der Aalst (2013) Process Cubes for the time dimension as a
//! first-class cube axis.
//!
//! ## Graduate to `wasm4pm`
//!
//! When you need to *compute* temporal orders, derive sojourn times, or run
//! temporal conformance checking, graduate to `wasm4pm`.

use core::marker::PhantomData;

/// Temporal ordering relationship between two events.
///
/// ## What this is
///
/// An enumeration of the four canonical temporal relations that can hold
/// between two events in a trace: `Before`, `After`, `Concurrent` (no strict
/// order), and `Unknown` (ordering not determinable from available data).
///
/// ## What this is not
///
/// Not a timestamp comparison engine. The relation is the *result* of a
/// comparison that belongs in `wasm4pm`. This type carries the structural
/// result shape only.
///
/// ## Graduate to `wasm4pm`
///
/// Deriving the ordering relation from event timestamps, handling time zones,
/// handling clock drift, and detecting impossible orderings all graduate to
/// `wasm4pm`.
///
/// # Examples
///
/// ```ignore
/// use wasm4pm_compat::temporal::TemporalOrder;
/// let order = TemporalOrder::Before;
/// assert_eq!(format!("{}", order), "before");
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum TemporalOrder {
    /// This event strictly precedes the other.
    Before,
    /// This event strictly follows the other.
    After,
    /// The two events are concurrent — no strict temporal ordering is established.
    Concurrent,
    /// The ordering is not determinable from the available event data.
    Unknown,
}

impl core::fmt::Display for TemporalOrder {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Before => write!(f, "before"),
            Self::After => write!(f, "after"),
            Self::Concurrent => write!(f, "concurrent"),
            Self::Unknown => write!(f, "unknown"),
        }
    }
}

/// A temporal profile for a trace — structural shape (not computed).
///
/// ## What this is
///
/// The structural shape of a temporal profile for a trace. `Trace` is the
/// type parameter naming the kind of trace this profile is derived from. The
/// profile itself is a collection of pairwise temporal ordering relations
/// between events in the trace — the shape that a temporal profile engine
/// produces and that a temporal conformance checker consumes.
///
/// ## What this is not
///
/// Not the profile derivation algorithm. Computing pairwise temporal relations
/// from timestamps, handling repeated activities, or computing average sojourn
/// times all graduate to `wasm4pm`.
///
/// ## Graduate to `wasm4pm`
///
/// Profile derivation, temporal conformance checking, and profile comparison
/// all graduate to `wasm4pm`.
///
/// # Examples
///
/// ```ignore
/// use wasm4pm_compat::temporal::TemporalProfile;
/// use core::marker::PhantomData;
/// struct MyTrace;
/// let profile: TemporalProfile<MyTrace> = TemporalProfile { trace: PhantomData };
/// ```
pub struct TemporalProfile<Trace> {
    /// Phantom binding to the trace type this profile was derived from.
    pub trace: PhantomData<Trace>,
}

impl<Trace> TemporalProfile<Trace> {
    /// Construct a new `TemporalProfile` shape marker.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use wasm4pm_compat::temporal::TemporalProfile;
    /// struct MyTrace;
    /// let profile: TemporalProfile<MyTrace> = TemporalProfile::new();
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self { trace: PhantomData }
    }
}

impl<Trace> Default for TemporalProfile<Trace> {
    fn default() -> Self {
        Self::new()
    }
}

/// Witness that temporal ordering has been established for this evidence.
///
/// ## What this is
///
/// A zero-cost marker type witnessing that temporal ordering has been
/// established — i.e., that the evidence it annotates has had its event-pair
/// ordering relations derived and attached. This is the receipt shape for
/// the ordering derivation step.
///
/// ## What this is not
///
/// Not the ordering derivation algorithm. The algorithm graduates to `wasm4pm`.
///
/// # Examples
///
/// ```ignore
/// use wasm4pm_compat::temporal::TemporalOrderWitness;
/// let _w = TemporalOrderWitness;
/// ```
pub struct TemporalOrderWitness;

/// Witness that sojourn times have been computed for this evidence.
///
/// ## What this is
///
/// A zero-cost marker type witnessing that sojourn times (the time an activity
/// spends in execution) have been computed and attached to the evidence.
/// Sojourn time is a key temporal metric in temporal profile conformance.
///
/// ## What this is not
///
/// Not the sojourn time computation. The computation graduates to `wasm4pm`.
///
/// # Examples
///
/// ```ignore
/// use wasm4pm_compat::temporal::SojournTimeWitness;
/// let _w = SojournTimeWitness;
/// ```
pub struct SojournTimeWitness;

/// A time-aware evidence wrapper adding temporal context to an inner value.
///
/// ## What this is
///
/// A structural wrapper that binds an inner value `T` to a temporal ordering
/// context `Order`. The `Order` type parameter names the temporal context
/// (e.g., `TemporalOrderWitness` for established ordering, `SojournTimeWitness`
/// for sojourn-time-enriched evidence). This allows functions to require that
/// evidence has had temporal context established before it is processed.
///
/// ## What this is not
///
/// Not a timestamp container. Timestamps and their computation graduate to
/// `wasm4pm`. This is the shape that carries already-established temporal
/// context.
///
/// ## Graduate to `wasm4pm`
///
/// All temporal computation (ordering derivation, sojourn time calculation,
/// temporal conformance checking) graduates to `wasm4pm`.
///
/// # Examples
///
/// ```ignore
/// use wasm4pm_compat::temporal::{TimeAwareEvidence, TemporalOrderWitness};
/// use core::marker::PhantomData;
///
/// let evidence: TimeAwareEvidence<u64, TemporalOrderWitness> = TimeAwareEvidence {
///     inner: 42u64,
///     order: PhantomData,
/// };
/// assert_eq!(evidence.inner, 42);
/// ```
pub struct TimeAwareEvidence<T, Order> {
    /// The inner evidence value wrapped with temporal context.
    pub inner: T,
    /// Phantom binding to the temporal ordering context type.
    pub order: PhantomData<Order>,
}

impl<T, Order> TimeAwareEvidence<T, Order> {
    /// Construct a new `TimeAwareEvidence` wrapper.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use wasm4pm_compat::temporal::{TimeAwareEvidence, TemporalOrderWitness};
    /// let tae: TimeAwareEvidence<u64, TemporalOrderWitness> =
    ///     TimeAwareEvidence::new(99u64);
    /// assert_eq!(tae.inner, 99);
    /// ```
    #[inline]
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            order: PhantomData,
        }
    }

    /// Consume this wrapper and return the inner value.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use wasm4pm_compat::temporal::{TimeAwareEvidence, SojournTimeWitness};
    /// let tae: TimeAwareEvidence<String, SojournTimeWitness> =
    ///     TimeAwareEvidence::new("hello".to_string());
    /// assert_eq!(tae.into_inner(), "hello");
    /// ```
    #[inline]
    pub fn into_inner(self) -> T {
        self.inner
    }
}
