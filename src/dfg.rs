//! Directly-Follows Graph (DFG) **shape** — the graph, not the discovery
//! algorithm.
//!
//! A DFG records, for a process, which activities directly follow which, and how
//! often. This module models the *graph value*: a [`Dfg`] is a set of
//! [`DfgNode`]s (activities) joined by weighted [`DfgEdge`]s, each carrying a
//! [`DfgWeight`] (a directly-follows frequency).
//!
//! ## Structure only — no discovery
//!
//! This crate does **not** discover a DFG from a log. Computing directly-follows
//! relations and frequencies *is* a discovery engine and graduates to
//! `wasm4pm`. To make that boundary unmistakable, asking a DFG to behave as if
//! it had been discovered when it is empty is refused as the named law
//! [`DfgRefusal::DiscoveryRequired`].
//!
//! [`Dfg::validate`] checks only *graph* shape: edges reference declared nodes,
//! weights are non-negative, and the graph is non-empty.
//!
//! ## Graduation to `wasm4pm`
//!
//! DFG *discovery* (from an [`crate::eventlog::EventLog`] or [`crate::ocel::OcelLog`]),
//! filtering, and DFG-based conformance graduate to `wasm4pm`. This crate only
//! represents and structurally validates an already-known DFG so it can travel
//! across the compat boundary.

/// A typed activity identifier: a `&'static str` newtype that names a specific
/// activity in a DFG at the type level.
///
/// `DfgActivityId` differs from the bare `String` stored inside [`DfgNode`]: it
/// is a *zero-cost* wrapper intended for code that must name a specific activity
/// at compile time (e.g. a test fixture, a hard-coded process template) rather
/// than build one from runtime data.
///
/// Structure-only: a labeled name, not a mined entity.
///
/// ```
/// use wasm4pm_compat::dfg::DfgActivityId;
/// const SHIP: DfgActivityId = DfgActivityId::new("ship");
/// assert_eq!(SHIP.as_str(), "ship");
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DfgActivityId(pub &'static str);

impl DfgActivityId {
    /// Construct a typed activity identifier from a static string.
    ///
    /// ```
    /// use wasm4pm_compat::dfg::DfgActivityId;
    /// let id = DfgActivityId::new("pay");
    /// assert_eq!(id.as_str(), "pay");
    /// ```
    #[must_use]
    pub const fn new(name: &'static str) -> Self {
        DfgActivityId(name)
    }

    /// The activity name.
    ///
    /// ```
    /// use wasm4pm_compat::dfg::DfgActivityId;
    /// assert_eq!(DfgActivityId::new("audit").as_str(), "audit");
    /// ```
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        self.0
    }

    /// Consumes `self` and returns the underlying `&'static str`.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::dfg::DfgActivityId;
    /// assert_eq!(DfgActivityId::new("pay").into_inner(), "pay");
    /// ```
    #[inline]
    #[must_use]
    pub const fn into_inner(self) -> &'static str {
        self.0
    }

    /// Borrows the underlying `&'static str`.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::dfg::DfgActivityId;
    /// assert_eq!(DfgActivityId::new("pay").as_inner(), "pay");
    /// ```
    #[inline]
    #[must_use]
    pub const fn as_inner(&self) -> &'static str {
        self.0
    }
}

impl From<&'static str> for DfgActivityId {
    /// Wraps a static string as a [`DfgActivityId`]. Infallible.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::dfg::DfgActivityId;
    /// let id: DfgActivityId = "pay".into();
    /// assert_eq!(id.as_str(), "pay");
    /// ```
    #[inline]
    fn from(s: &'static str) -> Self {
        DfgActivityId(s)
    }
}

impl AsRef<str> for DfgActivityId {
    /// Borrows the activity name as a `&str`.
    #[inline]
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl core::fmt::Display for DfgActivityId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.0)
    }
}

// ── Typed endpoint role markers ───────────────────────────────────────────────

/// A zero-sized type-level marker asserting that a type parameter plays the
/// **source** role in a directed DFG edge.
///
/// `DfgSourceMarker` and [`DfgTargetMarker`] are phantom-data tags used with
/// [`DfgTypedEdge`] to make the directionality of an edge visible to the
/// compiler. Code that accepts a source activity but not a target activity can
/// bound on [`IsDfgSource`].
///
/// Structure-only: carries no data, no graph semantics.
///
/// ```
/// use wasm4pm_compat::dfg::{DfgSourceMarker, IsDfgSource};
/// fn source_slot<S: IsDfgSource>() {}
/// source_slot::<DfgSourceMarker>();
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct DfgSourceMarker;

/// A zero-sized type-level marker asserting that a type parameter plays the
/// **target** role in a directed DFG edge.
///
/// Paired with [`DfgSourceMarker`]; see its documentation.
///
/// ```
/// use wasm4pm_compat::dfg::{DfgTargetMarker, IsDfgTarget};
/// fn target_slot<T: IsDfgTarget>() {}
/// target_slot::<DfgTargetMarker>();
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct DfgTargetMarker;

mod dfg_endpoint_seal {
    pub trait SourceSeal {}
    pub trait TargetSeal {}
    impl SourceSeal for super::DfgSourceMarker {}
    impl TargetSeal for super::DfgTargetMarker {}
}

/// Sealed trait — only [`DfgSourceMarker`] is a DFG source endpoint kind.
///
/// ```
/// use wasm4pm_compat::dfg::{DfgSourceMarker, IsDfgSource};
/// fn needs_source<S: IsDfgSource>(_: S) {}
/// needs_source(DfgSourceMarker);
/// ```
pub trait IsDfgSource: dfg_endpoint_seal::SourceSeal {}
impl IsDfgSource for DfgSourceMarker {}

/// Sealed trait — only [`DfgTargetMarker`] is a DFG target endpoint kind.
///
/// ```
/// use wasm4pm_compat::dfg::{DfgTargetMarker, IsDfgTarget};
/// fn needs_target<T: IsDfgTarget>(_: T) {}
/// needs_target(DfgTargetMarker);
/// ```
pub trait IsDfgTarget: dfg_endpoint_seal::TargetSeal {}
impl IsDfgTarget for DfgTargetMarker {}

/// A directly-follows frequency weight on a [`DfgEdge`].
///
/// A zero-cost `#[repr(transparent)]` wrapper over a `u64` count. Negative
/// frequencies are impossible by construction; the
/// [`DfgRefusal::NegativeWeight`] law exists for boundaries that admit
/// weights from signed external representations.
///
/// Structure-only: it is a labeled count, not a mined statistic.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct DfgWeight(pub u64);

impl DfgWeight {
    /// The underlying frequency count.
    ///
    /// ```
    /// use wasm4pm_compat::dfg::DfgWeight;
    /// assert_eq!(DfgWeight(7).count(), 7);
    /// ```
    pub fn count(self) -> u64 {
        self.0
    }

    /// Convert this `DfgWeight` into a [`DfgFrequency`] (a semantically named
    /// alias for the same count).
    ///
    /// ```
    /// use wasm4pm_compat::dfg::{DfgWeight, DfgFrequency};
    /// assert_eq!(DfgWeight(3).into_frequency(), DfgFrequency(3));
    /// ```
    #[must_use]
    pub fn into_frequency(self) -> DfgFrequency {
        DfgFrequency(self.0)
    }
}

/// A DFG node: a single activity in the directly-follows graph.
///
/// Holds the activity name. An empty name is refused as
/// [`DfgRefusal::MissingActivity`] at validation time.
///
/// Structure-only: a labeled vertex.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DfgNode {
    activity: String,
}

impl DfgNode {
    /// Construct a DFG node for an activity.
    ///
    /// ```
    /// use wasm4pm_compat::dfg::DfgNode;
    /// assert_eq!(DfgNode::new("ship").activity(), "ship");
    /// ```
    pub fn new(activity: impl Into<String>) -> Self {
        DfgNode {
            activity: activity.into(),
        }
    }

    /// The node's activity name.
    ///
    /// ```
    /// use wasm4pm_compat::dfg::DfgNode;
    /// assert_eq!(DfgNode::new("a").activity(), "a");
    /// ```
    pub fn activity(&self) -> &str {
        &self.activity
    }
}

/// A DFG edge: a directly-follows relation `from → to` with a [`DfgWeight`].
///
/// An edge whose endpoints are not declared nodes is refused as
/// [`DfgRefusal::DanglingEdge`].
///
/// Structure-only: a weighted directed edge, not a mined dependency.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DfgEdge {
    from: String,
    to: String,
    weight: DfgWeight,
}

impl DfgEdge {
    /// Construct a directly-follows edge `from → to` with a frequency.
    ///
    /// ```
    /// use wasm4pm_compat::dfg::DfgEdge;
    /// let e = DfgEdge::new("a", "b", 3);
    /// assert_eq!(e.from(), "a");
    /// assert_eq!(e.to(), "b");
    /// assert_eq!(e.weight().count(), 3);
    /// ```
    pub fn new(from: impl Into<String>, to: impl Into<String>, weight: u64) -> Self {
        DfgEdge {
            from: from.into(),
            to: to.into(),
            weight: DfgWeight(weight),
        }
    }

    /// The source activity.
    ///
    /// ```
    /// use wasm4pm_compat::dfg::DfgEdge;
    /// assert_eq!(DfgEdge::new("a", "b", 1).from(), "a");
    /// ```
    pub fn from(&self) -> &str {
        &self.from
    }

    /// The target activity.
    ///
    /// ```
    /// use wasm4pm_compat::dfg::DfgEdge;
    /// assert_eq!(DfgEdge::new("a", "b", 1).to(), "b");
    /// ```
    pub fn to(&self) -> &str {
        &self.to
    }

    /// The directly-follows frequency weight.
    ///
    /// ```
    /// use wasm4pm_compat::dfg::DfgEdge;
    /// assert_eq!(DfgEdge::new("a", "b", 5).weight().count(), 5);
    /// ```
    pub fn weight(&self) -> DfgWeight {
        self.weight
    }
}

/// A directly-follows graph: nodes (activities) and weighted directly-follows
/// edges.
///
/// [`Dfg::validate`] checks *graph* shape only (non-empty, named activities,
/// edges between declared nodes). It does **not** discover the graph — that
/// graduates to `wasm4pm`.
///
/// Structure-only: an admitted `Dfg` is an interchange value, not a discovery
/// result computed here.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Dfg {
    nodes: Vec<DfgNode>,
    edges: Vec<DfgEdge>,
}

impl Dfg {
    /// Construct a DFG from nodes and edges.
    ///
    /// ```
    /// use wasm4pm_compat::dfg::{Dfg, DfgNode, DfgEdge};
    /// let g = Dfg::new(
    ///     [DfgNode::new("a"), DfgNode::new("b")],
    ///     [DfgEdge::new("a", "b", 4)],
    /// );
    /// assert!(g.validate().is_ok());
    /// ```
    pub fn new(
        nodes: impl IntoIterator<Item = DfgNode>,
        edges: impl IntoIterator<Item = DfgEdge>,
    ) -> Self {
        Dfg {
            nodes: nodes.into_iter().collect(),
            edges: edges.into_iter().collect(),
        }
    }

    /// The DFG nodes (activities).
    pub fn nodes(&self) -> &[DfgNode] {
        &self.nodes
    }

    /// The DFG edges (directly-follows relations).
    pub fn edges(&self) -> &[DfgEdge] {
        &self.edges
    }

    /// Structurally validate the DFG graph shape.
    ///
    /// Checks, in order:
    /// - the graph is non-empty ([`DfgRefusal::EmptyGraph`]);
    /// - every node names a non-empty activity ([`DfgRefusal::MissingActivity`]);
    /// - every edge connects two declared nodes ([`DfgRefusal::DanglingEdge`]).
    ///
    /// Weights are non-negative by construction; [`DfgRefusal::NegativeWeight`]
    /// and [`DfgRefusal::DiscoveryRequired`] are boundary laws for admission and
    /// graduation, not produced by this structural check. This is a shape check,
    /// not discovery.
    ///
    /// ```
    /// use wasm4pm_compat::dfg::{Dfg, DfgNode, DfgEdge, DfgRefusal};
    /// // Edge to undeclared node "ghost".
    /// let g = Dfg::new([DfgNode::new("a")], [DfgEdge::new("a", "ghost", 1)]);
    /// assert_eq!(g.validate(), Err(DfgRefusal::DanglingEdge));
    /// ```
    #[must_use = "check the shape-check result"]
    pub fn validate(&self) -> Result<(), DfgRefusal> {
        use std::collections::HashSet;
        if self.nodes.is_empty() {
            return Err(DfgRefusal::EmptyGraph);
        }
        let mut acts: HashSet<&str> = HashSet::new();
        for n in &self.nodes {
            if n.activity().is_empty() {
                return Err(DfgRefusal::MissingActivity);
            }
            acts.insert(n.activity());
        }
        for e in &self.edges {
            if !acts.contains(e.from()) || !acts.contains(e.to()) {
                return Err(DfgRefusal::DanglingEdge);
            }
        }
        Ok(())
    }
}

/// The specific, named laws under which DFG structure is refused.
///
/// Each variant cites a distinct law — never a bare "invalid input".
/// [`DfgRefusal::DiscoveryRequired`] is the boundary law that keeps discovery
/// out of this crate: a DFG that must be *discovered* (e.g. requested from an
/// empty graph as if mining had occurred) is refused here and graduates to
/// `wasm4pm`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DfgRefusal {
    /// A node names an empty activity.
    MissingActivity,
    /// A weight admitted from a signed external source was negative.
    NegativeWeight,
    /// An edge references an undeclared node.
    DanglingEdge,
    /// The graph has no nodes.
    EmptyGraph,
    /// Discovery is required to produce this DFG; it cannot be synthesized here.
    /// Graduate to `wasm4pm`.
    DiscoveryRequired,
    /// An [`ObjectCentricDfg`] declares two DFGs for the same object type.
    ///
    /// The per-type map must be injective: each object type names exactly one DFG.
    /// A duplicate registration is refused under this law.
    InconsistentObjectType,
}

impl core::fmt::Display for DfgRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let law = match self {
            DfgRefusal::MissingActivity => "MissingActivity",
            DfgRefusal::NegativeWeight => "NegativeWeight",
            DfgRefusal::DanglingEdge => "DanglingEdge",
            DfgRefusal::EmptyGraph => "EmptyGraph",
            DfgRefusal::DiscoveryRequired => "DiscoveryRequired",
            DfgRefusal::InconsistentObjectType => "InconsistentObjectType",
        };
        write!(f, "DFG refused by law: {law}")
    }
}

/// A named frequency count on a DFG edge — a semantically distinct alias for
/// [`DfgWeight`] that makes the carrier's meaning explicit at call sites.
///
/// `DfgFrequency` is the number of times one activity directly follows another
/// across the traces (or objects) in the log. It is zero-cost over a `u64`.
///
/// Structure-only: it is a labeled count, not a mined statistic.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct DfgFrequency(pub u64);

impl DfgFrequency {
    /// The underlying frequency count.
    ///
    /// ```
    /// use wasm4pm_compat::dfg::DfgFrequency;
    /// assert_eq!(DfgFrequency(12).count(), 12);
    /// ```
    #[must_use]
    pub fn count(self) -> u64 {
        self.0
    }

    /// Convert from a [`DfgWeight`] (the two are structurally identical).
    ///
    /// ```
    /// use wasm4pm_compat::dfg::{DfgFrequency, DfgWeight};
    /// assert_eq!(DfgFrequency::from_weight(DfgWeight(5)).count(), 5);
    /// ```
    #[must_use]
    pub fn from_weight(w: DfgWeight) -> Self {
        DfgFrequency(w.0)
    }

    /// Convert this `DfgFrequency` into a [`DfgWeight`] (the inverse of
    /// [`DfgFrequency::from_weight`]).
    ///
    /// ```
    /// use wasm4pm_compat::dfg::{DfgFrequency, DfgWeight};
    /// assert_eq!(DfgFrequency(8).into_weight(), DfgWeight(8));
    /// ```
    #[must_use]
    pub fn into_weight(self) -> DfgWeight {
        DfgWeight(self.0)
    }
}

/// A mean-duration annotation on a DFG edge, in nanoseconds.
///
/// `DfgDuration` records the mean time between one activity's completion and the
/// next activity's start, across the directly-follows pairs in the log. It is
/// zero-cost over an `i64` (negative durations are representable for admission
/// of externally-sourced data; the boundary law is
/// [`DfgRefusal::NegativeWeight`]).
///
/// Structure-only: it is a labeled duration, not a mined performance metric.
/// Performance mining (median, percentile, histogram) graduates to `wasm4pm`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct DfgDuration(pub i64);

impl DfgDuration {
    /// The duration in nanoseconds.
    ///
    /// ```
    /// use wasm4pm_compat::dfg::DfgDuration;
    /// assert_eq!(DfgDuration(1_000_000).ns(), 1_000_000);
    /// ```
    #[must_use]
    pub fn ns(self) -> i64 {
        self.0
    }

    /// Returns `true` if this duration is strictly negative.
    ///
    /// A negative duration is representable (for admission of external signed
    /// data) but should be refused by [`DfgRefusal::NegativeWeight`] at the
    /// boundary.
    ///
    /// ```
    /// use wasm4pm_compat::dfg::DfgDuration;
    /// assert!(DfgDuration(-1).is_negative());
    /// assert!(!DfgDuration(0).is_negative());
    /// assert!(!DfgDuration(42).is_negative());
    /// ```
    #[must_use]
    pub fn is_negative(self) -> bool {
        self.0 < 0
    }

    /// Convert a non-negative nanosecond count into a `DfgDuration`, or return
    /// `None` if the value is negative (boundary admission helper).
    ///
    /// ```
    /// use wasm4pm_compat::dfg::DfgDuration;
    /// assert_eq!(DfgDuration::from_ns(500), Some(DfgDuration(500)));
    /// assert_eq!(DfgDuration::from_ns(-1),  None);
    /// ```
    #[must_use]
    pub fn from_ns(ns: i64) -> Option<Self> {
        if ns < 0 {
            None
        } else {
            Some(DfgDuration(ns))
        }
    }
}

/// A DFG edge annotated with both a [`DfgFrequency`] and an optional mean
/// [`DfgDuration`].
///
/// `DfgEdgeFull` is the performance-aware sibling of [`DfgEdge`]. It carries
/// the frequency count and, where available, the mean inter-activity duration.
/// A missing duration simply means the log did not record timestamps.
///
/// Structure-only: it is a weighted, performance-annotated directed edge, not
/// a mined result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DfgEdgeFull {
    from: String,
    to: String,
    frequency: DfgFrequency,
    duration_ns: Option<DfgDuration>,
}

impl DfgEdgeFull {
    /// Construct a frequency-only (no duration) edge.
    ///
    /// ```
    /// use wasm4pm_compat::dfg::{DfgEdgeFull, DfgFrequency};
    /// let e = DfgEdgeFull::new("a", "b", 7);
    /// assert_eq!(e.frequency().count(), 7);
    /// assert!(e.duration_ns().is_none());
    /// ```
    pub fn new(from: impl Into<String>, to: impl Into<String>, freq: u64) -> Self {
        DfgEdgeFull {
            from: from.into(),
            to: to.into(),
            frequency: DfgFrequency(freq),
            duration_ns: None,
        }
    }

    /// Attach a mean duration. Builder-style.
    ///
    /// ```
    /// use wasm4pm_compat::dfg::{DfgEdgeFull, DfgDuration};
    /// let e = DfgEdgeFull::new("a", "b", 3).with_duration_ns(500_000);
    /// assert_eq!(e.duration_ns(), Some(DfgDuration(500_000)));
    /// ```
    pub fn with_duration_ns(mut self, ns: i64) -> Self {
        self.duration_ns = Some(DfgDuration(ns));
        self
    }

    /// The source activity.
    ///
    /// ```
    /// use wasm4pm_compat::dfg::DfgEdgeFull;
    /// assert_eq!(DfgEdgeFull::new("a", "b", 1).from(), "a");
    /// ```
    pub fn from(&self) -> &str {
        &self.from
    }

    /// The target activity.
    ///
    /// ```
    /// use wasm4pm_compat::dfg::DfgEdgeFull;
    /// assert_eq!(DfgEdgeFull::new("a", "b", 1).to(), "b");
    /// ```
    pub fn to(&self) -> &str {
        &self.to
    }

    /// The directly-follows frequency.
    ///
    /// ```
    /// use wasm4pm_compat::dfg::{DfgEdgeFull, DfgFrequency};
    /// assert_eq!(DfgEdgeFull::new("a", "b", 4).frequency(), DfgFrequency(4));
    /// ```
    pub fn frequency(&self) -> DfgFrequency {
        self.frequency
    }

    /// The optional mean inter-activity duration.
    ///
    /// ```
    /// use wasm4pm_compat::dfg::DfgEdgeFull;
    /// assert!(DfgEdgeFull::new("a", "b", 1).duration_ns().is_none());
    /// ```
    #[must_use]
    pub fn duration_ns(&self) -> Option<DfgDuration> {
        self.duration_ns
    }
}

/// A directly-follows edge whose **direction** is enforced at the type level via
/// phantom source and target role markers.
///
/// `DfgTypedEdge<S, T>` is the compile-time-safe sibling of [`DfgEdge`]. The
/// type parameters `S` and `T` must satisfy [`IsDfgSource`] and [`IsDfgTarget`]
/// respectively; in practice they are always [`DfgSourceMarker`] and
/// [`DfgTargetMarker`]. The markers are zero-cost `PhantomData` — they carry no
/// runtime data but prevent accidentally swapping the source and target roles
/// when constructing edge tables.
///
/// Structure-only: a typed directed edge. No discovery, no token semantics.
///
/// ```
/// use wasm4pm_compat::dfg::{DfgTypedEdge, DfgSourceMarker, DfgTargetMarker, DfgWeight};
/// let edge: DfgTypedEdge<DfgSourceMarker, DfgTargetMarker> =
///     DfgTypedEdge::new("a", "b", 5);
/// assert_eq!(edge.from(), "a");
/// assert_eq!(edge.to(), "b");
/// assert_eq!(edge.weight(), DfgWeight(5));
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DfgTypedEdge<S: IsDfgSource, T: IsDfgTarget> {
    from: String,
    to: String,
    weight: DfgWeight,
    _src: core::marker::PhantomData<S>,
    _tgt: core::marker::PhantomData<T>,
}

impl<S: IsDfgSource, T: IsDfgTarget> DfgTypedEdge<S, T> {
    /// Construct a typed directed DFG edge with a frequency weight.
    ///
    /// ```
    /// use wasm4pm_compat::dfg::{DfgTypedEdge, DfgSourceMarker, DfgTargetMarker};
    /// let e = DfgTypedEdge::<DfgSourceMarker, DfgTargetMarker>::new("place", "pay", 3);
    /// assert_eq!(e.from(), "place");
    /// assert_eq!(e.to(), "pay");
    /// ```
    pub fn new(from: impl Into<String>, to: impl Into<String>, weight: u64) -> Self {
        DfgTypedEdge {
            from: from.into(),
            to: to.into(),
            weight: DfgWeight(weight),
            _src: core::marker::PhantomData,
            _tgt: core::marker::PhantomData,
        }
    }

    /// The source activity name.
    ///
    /// ```
    /// use wasm4pm_compat::dfg::{DfgTypedEdge, DfgSourceMarker, DfgTargetMarker};
    /// assert_eq!(DfgTypedEdge::<DfgSourceMarker, DfgTargetMarker>::new("a", "b", 1).from(), "a");
    /// ```
    pub fn from(&self) -> &str {
        &self.from
    }

    /// The target activity name.
    ///
    /// ```
    /// use wasm4pm_compat::dfg::{DfgTypedEdge, DfgSourceMarker, DfgTargetMarker};
    /// assert_eq!(DfgTypedEdge::<DfgSourceMarker, DfgTargetMarker>::new("a", "b", 1).to(), "b");
    /// ```
    pub fn to(&self) -> &str {
        &self.to
    }

    /// The directly-follows frequency weight.
    ///
    /// ```
    /// use wasm4pm_compat::dfg::{DfgTypedEdge, DfgSourceMarker, DfgTargetMarker, DfgWeight};
    /// assert_eq!(
    ///     DfgTypedEdge::<DfgSourceMarker, DfgTargetMarker>::new("a", "b", 9).weight(),
    ///     DfgWeight(9)
    /// );
    /// ```
    pub fn weight(&self) -> DfgWeight {
        self.weight
    }

    /// Erase the typed markers and produce a plain [`DfgEdge`].
    ///
    /// Useful when a downstream API requires [`DfgEdge`] but the edge was
    /// constructed via the typed interface.
    ///
    /// ```
    /// use wasm4pm_compat::dfg::{DfgTypedEdge, DfgSourceMarker, DfgTargetMarker};
    /// let typed = DfgTypedEdge::<DfgSourceMarker, DfgTargetMarker>::new("a", "b", 2);
    /// let plain = typed.into_edge();
    /// assert_eq!(plain.from(), "a");
    /// ```
    pub fn into_edge(self) -> DfgEdge {
        DfgEdge {
            from: self.from,
            to: self.to,
            weight: self.weight,
        }
    }
}

/// A typed object-type label for use in [`ObjectCentricDfg`] construction.
///
/// `DfgObjectType` is a zero-cost `&'static str` newtype that names a specific
/// object type in an object-centric DFG at compile time (e.g. a test fixture or
/// a hard-coded OC-process template). It is the OC-DFG counterpart of
/// [`DfgActivityId`]: both exist to make fixture code self-documenting and to
/// prevent bare-string confusion between object types and activity names.
///
/// Structure-only: a labeled name, not a mined entity.
///
/// ```
/// use wasm4pm_compat::dfg::DfgObjectType;
/// const ORDER: DfgObjectType = DfgObjectType::new("order");
/// assert_eq!(ORDER.as_str(), "order");
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DfgObjectType(pub &'static str);

impl DfgObjectType {
    /// Construct a typed object-type label from a static string.
    ///
    /// ```
    /// use wasm4pm_compat::dfg::DfgObjectType;
    /// let t = DfgObjectType::new("item");
    /// assert_eq!(t.as_str(), "item");
    /// ```
    #[must_use]
    pub const fn new(name: &'static str) -> Self {
        DfgObjectType(name)
    }

    /// The object type name.
    ///
    /// ```
    /// use wasm4pm_compat::dfg::DfgObjectType;
    /// assert_eq!(DfgObjectType::new("delivery").as_str(), "delivery");
    /// ```
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        self.0
    }

    /// Consumes `self` and returns the underlying `&'static str`.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::dfg::DfgObjectType;
    /// assert_eq!(DfgObjectType::new("order").into_inner(), "order");
    /// ```
    #[inline]
    #[must_use]
    pub const fn into_inner(self) -> &'static str {
        self.0
    }

    /// Borrows the underlying `&'static str`.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::dfg::DfgObjectType;
    /// assert_eq!(DfgObjectType::new("order").as_inner(), "order");
    /// ```
    #[inline]
    #[must_use]
    pub const fn as_inner(&self) -> &'static str {
        self.0
    }
}

impl From<&'static str> for DfgObjectType {
    /// Wraps a static string as a [`DfgObjectType`]. Infallible.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::dfg::DfgObjectType;
    /// let t: DfgObjectType = "order".into();
    /// assert_eq!(t.as_str(), "order");
    /// ```
    #[inline]
    fn from(s: &'static str) -> Self {
        DfgObjectType(s)
    }
}

impl AsRef<str> for DfgObjectType {
    /// Borrows the object-type name as a `&str`.
    #[inline]
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl core::fmt::Display for DfgObjectType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.0)
    }
}

/// A per-object-type DFG: one [`Dfg`] (with frequency and optional duration
/// annotations) for each declared object type in an OCEL log.
///
/// Object-centric process mining uses one DFG per object type, not a single
/// flat graph. `ObjectCentricDfg` is the structure-only carrier for that set of
/// graphs. It does **not** discover the DFGs — discovery graduates to `wasm4pm`.
///
/// Structure-only: it holds a labelled map of DFGs, nothing more.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ObjectCentricDfg {
    /// Per-object-type DFGs: `(object_type, dfg)` pairs.
    ///
    /// Using a `Vec` (not a `HashMap`) to keep the type `Eq + Clone` without
    /// a dependency on `std::collections`. Order is by insertion.
    pub per_type: Vec<(String, Dfg)>,
}

impl ObjectCentricDfg {
    /// Construct an empty object-centric DFG set.
    ///
    /// ```
    /// use wasm4pm_compat::dfg::ObjectCentricDfg;
    /// assert!(ObjectCentricDfg::new().per_type.is_empty());
    /// ```
    pub fn new() -> Self {
        ObjectCentricDfg::default()
    }

    /// Add or replace the DFG for the given object type. Builder-style.
    ///
    /// ```
    /// use wasm4pm_compat::dfg::{ObjectCentricDfg, Dfg, DfgNode, DfgEdge};
    /// let oc = ObjectCentricDfg::new().with_type_dfg(
    ///     "order",
    ///     Dfg::new([DfgNode::new("place"), DfgNode::new("pay")], [DfgEdge::new("place", "pay", 3)]),
    /// );
    /// assert_eq!(oc.per_type.len(), 1);
    /// ```
    pub fn with_type_dfg(mut self, object_type: impl Into<String>, dfg: Dfg) -> Self {
        self.per_type.push((object_type.into(), dfg));
        self
    }

    /// Look up the DFG for a given object type, if present.
    ///
    /// ```
    /// use wasm4pm_compat::dfg::{ObjectCentricDfg, Dfg, DfgNode};
    /// let oc = ObjectCentricDfg::new()
    ///     .with_type_dfg("order", Dfg::new([DfgNode::new("a")], []));
    /// assert!(oc.get("order").is_some());
    /// assert!(oc.get("item").is_none());
    /// ```
    #[must_use]
    pub fn get(&self, object_type: &str) -> Option<&Dfg> {
        self.per_type
            .iter()
            .find(|(t, _)| t == object_type)
            .map(|(_, d)| d)
    }

    /// The object types for which a DFG has been registered.
    ///
    /// ```
    /// use wasm4pm_compat::dfg::{ObjectCentricDfg, Dfg, DfgNode};
    /// let oc = ObjectCentricDfg::new()
    ///     .with_type_dfg("order", Dfg::new([DfgNode::new("a")], []));
    /// assert_eq!(oc.object_types().collect::<Vec<_>>(), vec!["order"]);
    /// ```
    pub fn object_types(&self) -> impl Iterator<Item = &str> {
        self.per_type.iter().map(|(t, _)| t.as_str())
    }

    /// Structurally validate all per-type DFGs and the OC-DFG map itself.
    ///
    /// Checks, in order:
    /// - no object type is registered more than once
    ///   ([`DfgRefusal::InconsistentObjectType`]);
    /// - every individual per-type [`Dfg`] passes [`Dfg::validate`].
    ///
    /// An empty `ObjectCentricDfg` (no registered types) passes — "no types
    /// known" is a valid initial state, distinct from an individual empty
    /// [`Dfg`] (which is refused as [`DfgRefusal::EmptyGraph`]).
    ///
    /// ```
    /// use wasm4pm_compat::dfg::{ObjectCentricDfg, Dfg, DfgNode, DfgEdge, DfgRefusal};
    ///
    /// // Valid: two object types, each with a non-empty DFG.
    /// let oc = ObjectCentricDfg::new()
    ///     .with_type_dfg("order", Dfg::new([DfgNode::new("place"), DfgNode::new("pay")],
    ///                                      [DfgEdge::new("place", "pay", 2)]))
    ///     .with_type_dfg("item",  Dfg::new([DfgNode::new("pick"), DfgNode::new("ship")],
    ///                                      [DfgEdge::new("pick", "ship", 5)]));
    /// assert!(oc.validate_all().is_ok());
    ///
    /// // Duplicate object type is refused.
    /// let dup = ObjectCentricDfg::new()
    ///     .with_type_dfg("order", Dfg::new([DfgNode::new("a")], []))
    ///     .with_type_dfg("order", Dfg::new([DfgNode::new("b")], []));
    /// assert_eq!(dup.validate_all(), Err(DfgRefusal::InconsistentObjectType));
    /// ```
    #[must_use = "check the shape-check result"]
    pub fn validate_all(&self) -> Result<(), DfgRefusal> {
        use std::collections::HashSet;
        let mut seen: HashSet<&str> = HashSet::new();
        for (object_type, dfg) in &self.per_type {
            if !seen.insert(object_type.as_str()) {
                return Err(DfgRefusal::InconsistentObjectType);
            }
            dfg.validate()?;
        }
        Ok(())
    }
}

/// A typed (from, to) activity pair used as a deduplication key for DFG edges.
///
/// When building a [`Dfg`] from multiple traces, the same `(from, to)` pair may
/// appear many times. `DfgEdgeKey` is the canonical key type for accumulating
/// frequencies: use it as the key in a `HashMap<DfgEdgeKey, DfgFrequency>`
/// before constructing the final [`DfgEdge`] list.
///
/// `DfgEdgeKey` is a value type: `(from, to)` — equal keys are the same
/// directly-follows pair regardless of frequency.
///
/// Structure-only: an equality-comparable pair of activity names.
///
/// ```
/// use wasm4pm_compat::dfg::DfgEdgeKey;
/// let k1 = DfgEdgeKey::new("a", "b");
/// let k2 = DfgEdgeKey::new("a", "b");
/// let k3 = DfgEdgeKey::new("b", "a");
/// assert_eq!(k1, k2);
/// assert_ne!(k1, k3);
/// assert_eq!(k1.from(), "a");
/// assert_eq!(k1.to(), "b");
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DfgEdgeKey {
    from: String,
    to: String,
}

impl DfgEdgeKey {
    /// Construct a DFG edge key from a (from, to) activity pair.
    ///
    /// ```
    /// use wasm4pm_compat::dfg::DfgEdgeKey;
    /// let k = DfgEdgeKey::new("place", "pay");
    /// assert_eq!(k.from(), "place");
    /// assert_eq!(k.to(), "pay");
    /// ```
    pub fn new(from: impl Into<String>, to: impl Into<String>) -> Self {
        DfgEdgeKey {
            from: from.into(),
            to: to.into(),
        }
    }

    /// The source activity name.
    ///
    /// ```
    /// use wasm4pm_compat::dfg::DfgEdgeKey;
    /// assert_eq!(DfgEdgeKey::new("a", "b").from(), "a");
    /// ```
    pub fn from(&self) -> &str {
        &self.from
    }

    /// The target activity name.
    ///
    /// ```
    /// use wasm4pm_compat::dfg::DfgEdgeKey;
    /// assert_eq!(DfgEdgeKey::new("a", "b").to(), "b");
    /// ```
    pub fn to(&self) -> &str {
        &self.to
    }

    /// Produce a [`DfgEdge`] by pairing this key with a frequency count.
    ///
    /// ```
    /// use wasm4pm_compat::dfg::DfgEdgeKey;
    /// let edge = DfgEdgeKey::new("a", "b").into_edge(5);
    /// assert_eq!(edge.weight().count(), 5);
    /// ```
    pub fn into_edge(self, weight: u64) -> DfgEdge {
        DfgEdge {
            from: self.from,
            to: self.to,
            weight: DfgWeight(weight),
        }
    }
}
