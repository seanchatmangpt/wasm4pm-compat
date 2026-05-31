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
}

impl core::fmt::Display for DfgRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let law = match self {
            DfgRefusal::MissingActivity => "MissingActivity",
            DfgRefusal::NegativeWeight => "NegativeWeight",
            DfgRefusal::DanglingEdge => "DanglingEdge",
            DfgRefusal::EmptyGraph => "EmptyGraph",
            DfgRefusal::DiscoveryRequired => "DiscoveryRequired",
        };
        write!(f, "DFG refused by law: {law}")
    }
}
