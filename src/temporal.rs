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

pub trait TimeUnit {
    const UNIT_NAME: &'static str;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Seconds;
impl TimeUnit for Seconds {
    const UNIT_NAME: &'static str = "seconds";
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Milliseconds;
impl TimeUnit for Milliseconds {
    const UNIT_NAME: &'static str = "milliseconds";
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Nanoseconds;
impl TimeUnit for Nanoseconds {
    const UNIT_NAME: &'static str = "nanoseconds";
}

/// A duration / time distance between two activities.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TimeDelta<Unit = Seconds>(pub f64, pub PhantomData<Unit>);

impl<Unit> TimeDelta<Unit> {
    pub fn new(val: f64) -> Self {
        Self(val, PhantomData)
    }
}

impl<Unit> Default for TimeDelta<Unit> {
    fn default() -> Self {
        Self(0.0, PhantomData)
    }
}

/// A deviation score in standard deviations from the temporal profile mean.
///
/// Grounded in Stertz et al. (2020), conformance checking computes a Z-score
/// representing the time distance deviation.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ZScore(pub f64);

pub struct ZScoreConst<const NUM: u64, const DEN: u64>
where
    crate::law::Require<{ DEN > 0 }>: crate::law::IsTrue,
{
    _private: (),
}

impl<const NUM: u64, const DEN: u64> ZScoreConst<NUM, DEN>
where
    crate::law::Require<{ DEN > 0 }>: crate::law::IsTrue,
{
    pub fn new() -> Self {
        Self { _private: () }
    }

    pub fn to_f64(&self) -> f64 {
        NUM as f64 / DEN as f64
    }
}

impl<const NUM: u64, const DEN: u64> Default for ZScoreConst<NUM, DEN>
where
    crate::law::Require<{ DEN > 0 }>: crate::law::IsTrue,
{
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ActivityPair<A, B> {
    pub _a: PhantomData<A>,
    pub _b: PhantomData<B>,
}

impl<A, B> ActivityPair<A, B> {
    pub fn new() -> Self {
        Self {
            _a: PhantomData,
            _b: PhantomData,
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
/// use wasm4pm_compat::temporal::{TemporalProfile, TimeDelta, Seconds, ActivityPair};
/// let pair = ActivityPair::<String, String>::new();
/// let profile = TemporalProfile::new(
///     TimeDelta::<Seconds>::new(1.0),
///     TimeDelta::<Seconds>::new(0.5),
///     pair,
/// );
/// ```
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct TemporalProfile<Pair, Unit = Seconds> {
    pub avg: TimeDelta<Unit>,
    pub std: TimeDelta<Unit>,
    pub pair: Pair,
}

impl<Pair, Unit> TemporalProfile<Pair, Unit> {
    pub fn new(avg: TimeDelta<Unit>, std: TimeDelta<Unit>, pair: Pair) -> Self {
        Self { avg, std, pair }
    }
}
