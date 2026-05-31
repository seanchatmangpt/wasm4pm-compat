//! BPMN model **shape** — structure only, no execution semantics.
//!
//! BPMN (Business Process Model and Notation) is a graphical process-modeling
//! language. This module models its *graph shape*: a [`BpmnProcess`] is a set of
//! [`BpmnNode`]s (each a [`BpmnTask`], [`BpmnGateway`], or [`BpmnEvent`])
//! connected by directed [`BpmnEdge`]s (sequence flows).
//!
//! ## Structure only — no token semantics
//!
//! This crate does **not** execute BPMN. It does not propagate tokens through
//! gateways, evaluate conditions, simulate, or convert BPMN to a Petri net for
//! analysis. [`BpmnProcess::validate`] checks only *graph* laws: nodes are
//! identified, edges connect declared nodes, there is a start and an end.
//! Execution, simulation, and BPMN↔Petri conversion graduate to `wasm4pm`.
//!
//! ## Graduation to `wasm4pm`
//!
//! An admitted [`BpmnProcess`] is a well-shaped model graph. Soundness analysis,
//! token-flow simulation, and conformance of a log against the BPMN model
//! graduate to the `wasm4pm` engine.

/// The kind of a BPMN gateway (how flow diverges/converges at the node).
///
/// This enumerates gateway *kinds* as graph annotations; it does **not** define
/// their execution semantics (that is a `wasm4pm` concern).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum BpmnGateway {
    /// Exclusive (XOR) — one outgoing branch is taken.
    Exclusive,
    /// Parallel (AND) — all outgoing branches are taken.
    Parallel,
    /// Inclusive (OR) — one or more outgoing branches are taken.
    Inclusive,
    /// Event-based — branch chosen by a downstream event.
    EventBased,
    /// Complex — a modeler-defined activation condition.
    Complex,
}

/// The kind of a BPMN event node.
///
/// Annotates the event's position/trigger as a graph label; carries no
/// execution meaning here.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum BpmnEvent {
    /// A start event (process entry).
    Start,
    /// An intermediate event (catch/throw mid-process).
    Intermediate,
    /// An end event (process exit).
    End,
    /// A boundary event attached to an activity.
    Boundary,
}

/// A BPMN task node (a unit of work / activity).
///
/// Holds the task's display `name` (the activity it represents). Identity is
/// supplied by the enclosing [`BpmnNode`].
///
/// Structure-only: it is a labeled vertex, not an executable activity.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BpmnTask {
    name: String,
}

impl BpmnTask {
    /// Construct a task node with an activity name.
    ///
    /// ```
    /// use wasm4pm_compat::bpmn::BpmnTask;
    /// assert_eq!(BpmnTask::new("approve").name(), "approve");
    /// ```
    pub fn new(name: impl Into<String>) -> Self {
        BpmnTask { name: name.into() }
    }

    /// The task's activity name.
    ///
    /// ```
    /// use wasm4pm_compat::bpmn::BpmnTask;
    /// assert_eq!(BpmnTask::new("a").name(), "a");
    /// ```
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// A BPMN node: an identified vertex that is a task, a gateway, or an event.
///
/// The `id` keys the node in the [`BpmnProcess`] graph; edges reference these
/// ids. The [`BpmnNodeKind`] says what the node *is*.
///
/// Structure-only: a typed, identified vertex with no behavior.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BpmnNode {
    id: String,
    kind: BpmnNodeKind,
}

/// What a [`BpmnNode`] is: a task, a gateway, or an event.
///
/// Structure-only graph tagging; no execution semantics attached.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BpmnNodeKind {
    /// A task / activity node.
    Task(BpmnTask),
    /// A gateway node of the given kind.
    Gateway(BpmnGateway),
    /// An event node of the given kind.
    Event(BpmnEvent),
}

impl BpmnNode {
    /// Construct a task node with the given id.
    ///
    /// ```
    /// use wasm4pm_compat::bpmn::{BpmnNode, BpmnTask};
    /// let n = BpmnNode::task("t1", BpmnTask::new("approve"));
    /// assert_eq!(n.id(), "t1");
    /// ```
    pub fn task(id: impl Into<String>, task: BpmnTask) -> Self {
        BpmnNode {
            id: id.into(),
            kind: BpmnNodeKind::Task(task),
        }
    }

    /// Construct a gateway node with the given id and kind.
    ///
    /// ```
    /// use wasm4pm_compat::bpmn::{BpmnNode, BpmnGateway};
    /// let n = BpmnNode::gateway("g1", BpmnGateway::Exclusive);
    /// assert!(matches!(n.kind(), wasm4pm_compat::bpmn::BpmnNodeKind::Gateway(_)));
    /// ```
    pub fn gateway(id: impl Into<String>, gateway: BpmnGateway) -> Self {
        BpmnNode {
            id: id.into(),
            kind: BpmnNodeKind::Gateway(gateway),
        }
    }

    /// Construct an event node with the given id and kind.
    ///
    /// ```
    /// use wasm4pm_compat::bpmn::{BpmnNode, BpmnEvent};
    /// let n = BpmnNode::event("s", BpmnEvent::Start);
    /// assert_eq!(n.id(), "s");
    /// ```
    pub fn event(id: impl Into<String>, event: BpmnEvent) -> Self {
        BpmnNode {
            id: id.into(),
            kind: BpmnNodeKind::Event(event),
        }
    }

    /// The node's id.
    ///
    /// ```
    /// use wasm4pm_compat::bpmn::{BpmnNode, BpmnEvent};
    /// assert_eq!(BpmnNode::event("s", BpmnEvent::Start).id(), "s");
    /// ```
    pub fn id(&self) -> &str {
        &self.id
    }

    /// What the node is.
    ///
    /// ```
    /// use wasm4pm_compat::bpmn::{BpmnNode, BpmnEvent, BpmnNodeKind};
    /// let n = BpmnNode::event("s", BpmnEvent::Start);
    /// assert!(matches!(n.kind(), BpmnNodeKind::Event(BpmnEvent::Start)));
    /// ```
    pub fn kind(&self) -> &BpmnNodeKind {
        &self.kind
    }
}

/// A BPMN sequence flow: a directed edge from one node id to another.
///
/// An edge whose endpoints are not declared nodes is refused as
/// [`BpmnRefusal::DanglingEdge`].
///
/// Structure-only: a directed graph edge, no condition evaluation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BpmnEdge {
    source: String,
    target: String,
}

impl BpmnEdge {
    /// Construct a sequence flow from `source` to `target`.
    ///
    /// ```
    /// use wasm4pm_compat::bpmn::BpmnEdge;
    /// let e = BpmnEdge::new("a", "b");
    /// assert_eq!(e.source(), "a");
    /// assert_eq!(e.target(), "b");
    /// ```
    pub fn new(source: impl Into<String>, target: impl Into<String>) -> Self {
        BpmnEdge {
            source: source.into(),
            target: target.into(),
        }
    }

    /// The edge's source node id.
    ///
    /// ```
    /// use wasm4pm_compat::bpmn::BpmnEdge;
    /// assert_eq!(BpmnEdge::new("a", "b").source(), "a");
    /// ```
    pub fn source(&self) -> &str {
        &self.source
    }

    /// The edge's target node id.
    ///
    /// ```
    /// use wasm4pm_compat::bpmn::BpmnEdge;
    /// assert_eq!(BpmnEdge::new("a", "b").target(), "b");
    /// ```
    pub fn target(&self) -> &str {
        &self.target
    }
}

/// A BPMN process: a graph of [`BpmnNode`]s joined by [`BpmnEdge`]s.
///
/// [`BpmnProcess::validate`] checks *graph* shape only: nodes have unique ids,
/// edges connect declared nodes, the process has a start event and an end event.
/// It does not analyze soundness or simulate token flow — those graduate to
/// `wasm4pm`.
///
/// Structure-only: an admitted process is a model graph, not an executable.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BpmnProcess {
    nodes: Vec<BpmnNode>,
    edges: Vec<BpmnEdge>,
}

impl BpmnProcess {
    /// Construct a process from nodes and edges.
    ///
    /// ```
    /// use wasm4pm_compat::bpmn::{BpmnProcess, BpmnNode, BpmnEdge, BpmnEvent, BpmnTask};
    /// let p = BpmnProcess::new(
    ///     [
    ///         BpmnNode::event("s", BpmnEvent::Start),
    ///         BpmnNode::task("t", BpmnTask::new("work")),
    ///         BpmnNode::event("e", BpmnEvent::End),
    ///     ],
    ///     [BpmnEdge::new("s", "t"), BpmnEdge::new("t", "e")],
    /// );
    /// assert!(p.validate().is_ok());
    /// ```
    pub fn new(
        nodes: impl IntoIterator<Item = BpmnNode>,
        edges: impl IntoIterator<Item = BpmnEdge>,
    ) -> Self {
        BpmnProcess {
            nodes: nodes.into_iter().collect(),
            edges: edges.into_iter().collect(),
        }
    }

    /// The process's nodes.
    pub fn nodes(&self) -> &[BpmnNode] {
        &self.nodes
    }

    /// The process's edges (sequence flows).
    pub fn edges(&self) -> &[BpmnEdge] {
        &self.edges
    }

    /// Structurally validate the BPMN graph shape.
    ///
    /// Checks, in order:
    /// - at least one node exists ([`BpmnRefusal::EmptyProcess`]);
    /// - node ids are unique ([`BpmnRefusal::DuplicateNodeId`]);
    /// - a start event is present ([`BpmnRefusal::MissingStartEvent`]) and an
    ///   end event is present ([`BpmnRefusal::MissingEndEvent`]);
    /// - every edge connects two declared nodes ([`BpmnRefusal::DanglingEdge`]).
    ///
    /// This is a graph-shape check, not soundness analysis and not execution.
    ///
    /// ```
    /// use wasm4pm_compat::bpmn::{BpmnProcess, BpmnNode, BpmnEdge, BpmnEvent, BpmnRefusal};
    /// // Edge to an undeclared node "ghost".
    /// let p = BpmnProcess::new(
    ///     [BpmnNode::event("s", BpmnEvent::Start), BpmnNode::event("e", BpmnEvent::End)],
    ///     [BpmnEdge::new("s", "ghost")],
    /// );
    /// assert_eq!(p.validate(), Err(BpmnRefusal::DanglingEdge));
    /// ```
    pub fn validate(&self) -> Result<(), BpmnRefusal> {
        use std::collections::HashSet;

        if self.nodes.is_empty() {
            return Err(BpmnRefusal::EmptyProcess);
        }

        let mut ids: HashSet<&str> = HashSet::new();
        let mut has_start = false;
        let mut has_end = false;
        for n in &self.nodes {
            if !ids.insert(n.id()) {
                return Err(BpmnRefusal::DuplicateNodeId);
            }
            match n.kind() {
                BpmnNodeKind::Event(BpmnEvent::Start) => has_start = true,
                BpmnNodeKind::Event(BpmnEvent::End) => has_end = true,
                _ => {}
            }
        }
        if !has_start {
            return Err(BpmnRefusal::MissingStartEvent);
        }
        if !has_end {
            return Err(BpmnRefusal::MissingEndEvent);
        }

        for e in &self.edges {
            if !ids.contains(e.source()) || !ids.contains(e.target()) {
                return Err(BpmnRefusal::DanglingEdge);
            }
        }

        Ok(())
    }
}

// ── BPMN 2.0 Pool and Lane shapes ────────────────────────────────────────────

/// A BPMN 2.0 lane: a swimlane subdividing a pool for an organisational role.
///
/// Real-Life BPMN (4th ed.) defines a lane as a subdivision of a pool that
/// organises flow nodes by responsibility. A lane names which node ids
/// (from its enclosing pool's process) it claims. This is a structural
/// assignment claim; it does **not** change the flow semantics.
///
/// [`BpmnLane::validate`] checks that every declared `node_id` exists in the
/// enclosing process's node set, refusing with
/// [`BpmnRefusal::LaneNodeNotDeclared`] if not.
///
/// Structure-only: an organisational label, never an execution boundary.
///
/// ```
/// use wasm4pm_compat::bpmn::BpmnLane;
/// let lane = BpmnLane::new("lane-ops", "Operations", ["t1", "t2"]);
/// assert_eq!(lane.id(), "lane-ops");
/// assert_eq!(lane.node_ids().len(), 2);
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BpmnLane {
    id: String,
    name: String,
    node_ids: Vec<String>,
}

impl BpmnLane {
    /// Construct a lane with an id, display name, and the node ids it claims.
    ///
    /// ```
    /// use wasm4pm_compat::bpmn::BpmnLane;
    /// let lane = BpmnLane::new("l1", "Finance", ["t1"]);
    /// assert_eq!(lane.name(), "Finance");
    /// ```
    pub fn new<I, S>(id: impl Into<String>, name: impl Into<String>, node_ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        BpmnLane {
            id: id.into(),
            name: name.into(),
            node_ids: node_ids.into_iter().map(Into::into).collect(),
        }
    }

    /// The lane's id.
    ///
    /// ```
    /// use wasm4pm_compat::bpmn::BpmnLane;
    /// assert_eq!(BpmnLane::new("l1", "Ops", ["t1"]).id(), "l1");
    /// ```
    pub fn id(&self) -> &str {
        &self.id
    }

    /// The lane's display name.
    ///
    /// ```
    /// use wasm4pm_compat::bpmn::BpmnLane;
    /// assert_eq!(BpmnLane::new("l1", "Finance", ["t1"]).name(), "Finance");
    /// ```
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The ids of nodes assigned to this lane.
    ///
    /// ```
    /// use wasm4pm_compat::bpmn::BpmnLane;
    /// let lane = BpmnLane::new("l1", "Ops", ["t1", "t2"]);
    /// assert_eq!(lane.node_ids(), &["t1".to_string(), "t2".to_string()]);
    /// ```
    pub fn node_ids(&self) -> &[String] {
        &self.node_ids
    }

    /// Validate that all node ids declared by this lane exist in `known_ids`.
    ///
    /// Returns [`BpmnRefusal::LaneNodeNotDeclared`] if any id is absent.
    ///
    /// ```
    /// use std::collections::HashSet;
    /// use wasm4pm_compat::bpmn::{BpmnLane, BpmnRefusal};
    /// let lane = BpmnLane::new("l1", "Ops", ["t1", "ghost"]);
    /// let known: HashSet<&str> = ["t1"].into_iter().collect();
    /// assert_eq!(lane.validate(&known), Err(BpmnRefusal::LaneNodeNotDeclared));
    /// ```
    pub fn validate(&self, known_ids: &std::collections::HashSet<&str>) -> Result<(), BpmnRefusal> {
        for nid in &self.node_ids {
            if !known_ids.contains(nid.as_str()) {
                return Err(BpmnRefusal::LaneNodeNotDeclared);
            }
        }
        Ok(())
    }
}

/// A BPMN 2.0 pool: the top-level container for a process participant.
///
/// Real-Life BPMN (4th ed.) defines a pool as the shape that represents a
/// single participant in a collaboration. Each pool encloses exactly one
/// [`BpmnProcess`] and optionally subdivides it into [`BpmnLane`]s.
///
/// [`BpmnPool::validate`] checks that the enclosed process is structurally
/// valid and that every lane's node ids are declared in the process.
///
/// Structure-only: participant identity and swimlane assignment; no inter-pool
/// message-flow semantics (those graduate to `wasm4pm`).
///
/// ```
/// use wasm4pm_compat::bpmn::{BpmnPool, BpmnProcess, BpmnNode, BpmnEdge, BpmnEvent, BpmnTask, BpmnLane};
/// let process = BpmnProcess::new(
///     [
///         BpmnNode::event("s", BpmnEvent::Start),
///         BpmnNode::task("t", BpmnTask::new("approve")),
///         BpmnNode::event("e", BpmnEvent::End),
///     ],
///     [BpmnEdge::new("s", "t"), BpmnEdge::new("t", "e")],
/// );
/// let pool = BpmnPool::new("pool-1", "Claims", process, [BpmnLane::new("l1", "Ops", ["t"])]);
/// assert!(pool.validate().is_ok());
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BpmnPool {
    id: String,
    name: String,
    process: BpmnProcess,
    lanes: Vec<BpmnLane>,
}

impl BpmnPool {
    /// Construct a pool with an id, display name, process, and lanes.
    ///
    /// ```
    /// use wasm4pm_compat::bpmn::{BpmnPool, BpmnProcess, BpmnNode, BpmnEdge, BpmnEvent, BpmnTask};
    /// let process = BpmnProcess::new(
    ///     [BpmnNode::event("s", BpmnEvent::Start), BpmnNode::event("e", BpmnEvent::End)],
    ///     [BpmnEdge::new("s", "e")],
    /// );
    /// let pool = BpmnPool::new("p1", "Ops", process, []);
    /// assert_eq!(pool.id(), "p1");
    /// ```
    pub fn new<I>(
        id: impl Into<String>,
        name: impl Into<String>,
        process: BpmnProcess,
        lanes: I,
    ) -> Self
    where
        I: IntoIterator<Item = BpmnLane>,
    {
        BpmnPool {
            id: id.into(),
            name: name.into(),
            process,
            lanes: lanes.into_iter().collect(),
        }
    }

    /// The pool's id.
    ///
    /// ```
    /// use wasm4pm_compat::bpmn::{BpmnPool, BpmnProcess, BpmnNode, BpmnEdge, BpmnEvent};
    /// let p = BpmnPool::new("p1", "Ops",
    ///     BpmnProcess::new(
    ///         [BpmnNode::event("s", BpmnEvent::Start), BpmnNode::event("e", BpmnEvent::End)],
    ///         [BpmnEdge::new("s", "e")]),
    ///     []);
    /// assert_eq!(p.id(), "p1");
    /// ```
    pub fn id(&self) -> &str {
        &self.id
    }

    /// The pool's display name.
    ///
    /// ```
    /// use wasm4pm_compat::bpmn::{BpmnPool, BpmnProcess, BpmnNode, BpmnEdge, BpmnEvent};
    /// let p = BpmnPool::new("p1", "Finance",
    ///     BpmnProcess::new(
    ///         [BpmnNode::event("s", BpmnEvent::Start), BpmnNode::event("e", BpmnEvent::End)],
    ///         [BpmnEdge::new("s", "e")]),
    ///     []);
    /// assert_eq!(p.name(), "Finance");
    /// ```
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The process enclosed by this pool.
    pub fn process(&self) -> &BpmnProcess {
        &self.process
    }

    /// The lanes subdividing this pool.
    pub fn lanes(&self) -> &[BpmnLane] {
        &self.lanes
    }

    /// Structurally validate the pool.
    ///
    /// Validates the enclosed process and then checks that every lane's node ids
    /// are declared in the process. Returns [`BpmnRefusal::LaneNodeNotDeclared`]
    /// if a lane references an undeclared node.
    ///
    /// ```
    /// use wasm4pm_compat::bpmn::{BpmnPool, BpmnProcess, BpmnNode, BpmnEdge, BpmnEvent, BpmnTask, BpmnLane, BpmnRefusal};
    /// let process = BpmnProcess::new(
    ///     [BpmnNode::event("s", BpmnEvent::Start), BpmnNode::event("e", BpmnEvent::End)],
    ///     [BpmnEdge::new("s", "e")],
    /// );
    /// // Lane references "ghost" which is not in the process.
    /// let pool = BpmnPool::new("p1", "Ops", process, [BpmnLane::new("l1", "Fin", ["ghost"])]);
    /// assert_eq!(pool.validate(), Err(BpmnRefusal::LaneNodeNotDeclared));
    /// ```
    pub fn validate(&self) -> Result<(), BpmnRefusal> {
        self.process.validate()?;
        let known: std::collections::HashSet<&str> =
            self.process.nodes().iter().map(BpmnNode::id).collect();
        for lane in &self.lanes {
            lane.validate(&known)?;
        }
        Ok(())
    }
}

/// The specific, named laws under which BPMN graph structure is refused.
///
/// Each variant cites a distinct graph law — never a bare "invalid input".
/// These are structural/graph defects, not execution-soundness verdicts (which
/// graduate to `wasm4pm`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum BpmnRefusal {
    /// The process declares no nodes.
    EmptyProcess,
    /// Two nodes share the same id.
    DuplicateNodeId,
    /// No start event is present.
    MissingStartEvent,
    /// No end event is present.
    MissingEndEvent,
    /// An edge references an undeclared source or target node.
    DanglingEdge,
    /// A gateway's branch structure is malformed (e.g. no outgoing flows).
    MalformedGateway,
    /// A node is unreachable from any start event by graph connectivity.
    DisconnectedNode,
    /// A [`BpmnLane`] references a node id not declared in the enclosing process.
    ///
    /// Law: BPMN 2.0 — a lane may only claim nodes that exist within its pool's
    /// process. Undeclared references are structurally invalid.
    LaneNodeNotDeclared,
}

impl core::fmt::Display for BpmnRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let law = match self {
            BpmnRefusal::EmptyProcess => "EmptyProcess",
            BpmnRefusal::DuplicateNodeId => "DuplicateNodeId",
            BpmnRefusal::MissingStartEvent => "MissingStartEvent",
            BpmnRefusal::MissingEndEvent => "MissingEndEvent",
            BpmnRefusal::DanglingEdge => "DanglingEdge",
            BpmnRefusal::MalformedGateway => "MalformedGateway",
            BpmnRefusal::DisconnectedNode => "DisconnectedNode",
            BpmnRefusal::LaneNodeNotDeclared => "LaneNodeNotDeclared",
        };
        write!(f, "BPMN refused by law: {law}")
    }
}
