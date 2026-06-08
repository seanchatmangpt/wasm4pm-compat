//! POWL (Partially Ordered Workflow Language) shape — **first-class, structure only**.
//!
//! This module represents the *shape* of a POWL model: a partially ordered
//! workflow built from atoms, partial orders, exclusive choices, and loops,
//! with explicit silent steps. POWL is treated as a **first-class** canon
//! member here — it is **not** forced into [`crate::process_tree`], because POWL
//! can express partial orders that no block-structured process tree can.
//!
//! ## What this module **IS**
//!
//! - The structural vocabulary of POWL: [`crate::powl::PowlNode`], [`crate::powl::OrderEdge`], [`crate::powl::Powl`].
//! - Witness markers describing *which POWL fragment* a node represents
//!   ([`crate::powl::Atom`], [`crate::powl::PartialOrder`], [`crate::powl::Choice`], [`crate::powl::Loop`], [`crate::powl::Silent`],
//!   [`crate::powl::Irreducible`]) and *whether it can graduate downward* into a process
//!   tree ([`crate::powl::ProcessTreeProjectable`], [`crate::powl::ExceedsProcessTree`]).
//! - A first-class [`crate::powl::PowlRefusal`] surface naming exactly why a POWL shape is
//!   inadmissible.
//!
//! ## What this module is **NOT**
//!
//! - **Not** a POWL discovery algorithm, language player, simplifier, or
//!   conformance checker. It builds and refuses *shapes*; it never *executes*
//!   them.
//! - **Not** a process tree in disguise. Projection POWL → process tree is a
//!   *named, refusable* operation, never an implicit coercion.
//!
//! ## Graduation
//!
//! When you need to **replay, discover, simplify, or measure** a POWL model,
//! graduate this shape to the `wasm4pm` engine (via the `wasm4pm` feature):
//! this module only certifies that the *structure* is well-formed and names the
//! law under which it would be refused.

use crate::law::{IsTrue, Require};
use core::marker::PhantomData;

// ── Witness markers: which POWL fragment a node is ──────────────────────────

/// Witness: the node is an **atom** (a single activity / leaf task).
///
/// Structure-only marker; carries no behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Atom;

/// Witness: the node is a **partial order** over child nodes (a DAG of
/// precedence edges, not a total sequence).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct PartialOrder;

/// Witness: the node is an **exclusive choice** (`xor`) among child branches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Choice;

/// Witness: the node is a **loop** (`do` body with an optional `redo`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Loop;

/// Witness: the node is a **silent** step (tau / no observable activity).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Silent;

/// Witness: the node is **irreducible** — a partial order that cannot be split
/// into block-structured operators without language loss.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Irreducible;

/// Witness: the partial order has been **proven acyclic** — all precedence edges
/// form a DAG (directed acyclic graph).
///
/// A [`PartialOrder`] is *structurally* a partial order, but this marker records
/// that acyclicity has been *asserted*: no node appears as both predecessor and
/// successor of another on any path. A [`PowlNode`] carrying [`AcyclicPartialOrder`]
/// has passed the structural law that POWL partial orders must be DAGs.
///
/// Obtaining this marker does **not** run a cycle-detection algorithm — that
/// graduates to `wasm4pm`. It records that the caller has asserted the invariant.
/// The assertion gate is [`assert_acyclic`].
///
/// Paper: Kourani et al. (2026) §3 — a POWL partial order `P(M⁺, ≺)` requires
/// `≺` to be a strict partial order (irreflexive, asymmetric, transitive), which
/// implies acyclicity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct AcyclicPartialOrder;

/// Graduation witness: the POWL fragment **can** be projected, losslessly, into
/// a block-structured [`crate::process_tree::ProcessTree`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ProcessTreeProjectable;

/// Graduation witness: the POWL fragment **exceeds** any process tree — its
/// partial order has no block-structured equivalent, so projection would lose
/// language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ExceedsProcessTree;

// ── AcyclicWitness sealed trait ──────────────────────────────────────────────

mod acyclic_witness_seal {
    pub trait Sealed {}
    impl Sealed for super::AcyclicPartialOrder {}
    // PartialOrder without the acyclicity claim deliberately NOT sealed here.
}

/// Sealed marker: only [`AcyclicPartialOrder`] satisfies this bound.
///
/// A function requiring `W: AcyclicWitness` can only be called with a marker
/// that carries the acyclicity assertion. [`PartialOrder`] alone does **not**
/// satisfy this bound — it is only a structural claim of kind, not a claim
/// about DAG shape.
///
/// Use [`assert_acyclic`] to obtain a witness value at a known assertion site.
///
/// ```
/// use wasm4pm_compat::powl::{assert_acyclic, AcyclicPartialOrder};
/// assert_acyclic(AcyclicPartialOrder);  // ok
/// ```
///
/// ```compile_fail
/// use wasm4pm_compat::powl::{assert_acyclic, PartialOrder};
/// assert_acyclic(PartialOrder);  // compile error: not AcyclicWitness
/// ```
pub trait AcyclicWitness: acyclic_witness_seal::Sealed {}
impl AcyclicWitness for AcyclicPartialOrder {}

/// Structural gate: only markers that carry the acyclicity assertion pass.
///
/// This is **not** a cycle-detection algorithm. It proves the acyclicity law
/// at the type level — the marker `W` must be [`AcyclicPartialOrder`].
///
/// Pass this gate at the site where acyclicity was asserted (e.g., after a
/// topological sort succeeds). The gate records the assertion; the actual
/// detection graduates to `wasm4pm`.
///
/// ```
/// use wasm4pm_compat::powl::{assert_acyclic, AcyclicPartialOrder};
/// let ok = assert_acyclic(AcyclicPartialOrder);
/// assert!(ok);
/// ```
pub fn assert_acyclic<W: AcyclicWitness>(_marker: W) -> bool {
    true
}

// ── TreeProjectable sealed trait ─────────────────────────────────────────────

mod tree_projectable_seal {
    pub trait Sealed {}
    impl Sealed for super::ProcessTreeProjectable {}
    // ExceedsProcessTree deliberately NOT sealed here → cannot impl TreeProjectable.
}

/// Sealed marker: only [`ProcessTreeProjectable`] satisfies this bound.
///
/// A function requiring `P: TreeProjectable` cannot be called with
/// [`ExceedsProcessTree`] — that marker represents a POWL fragment whose
/// partial order has no process-tree equivalent.
///
/// ```
/// use wasm4pm_compat::powl::{assert_tree_projectable, ProcessTreeProjectable};
/// assert_tree_projectable(ProcessTreeProjectable);  // ok
/// ```
///
/// ```compile_fail
/// use wasm4pm_compat::powl::{assert_tree_projectable, ExceedsProcessTree};
/// assert_tree_projectable(ExceedsProcessTree);  // compile error: not TreeProjectable
/// ```
pub trait TreeProjectable: tree_projectable_seal::Sealed {}
impl TreeProjectable for ProcessTreeProjectable {}

/// Structural gate: only POWL markers that are tree-projectable pass.
///
/// This is **not** a discovery function. It proves the projection law at the
/// type level — the marker `P` must be [`ProcessTreeProjectable`].
///
/// ```
/// use wasm4pm_compat::powl::{assert_tree_projectable, ProcessTreeProjectable};
/// let result = assert_tree_projectable(ProcessTreeProjectable);
/// assert!(result);
/// ```
pub fn assert_tree_projectable<P: TreeProjectable>(_marker: P) -> bool {
    true
}

// ── Identifier wrapper ──────────────────────────────────────────────────────

/// Zero-cost identifier for a [`PowlNode`] within a [`Powl`] model.
///
/// `#[repr(transparent)]` over `usize`: structural, comparable, and free.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PowlNodeId(pub usize);

// ── Core shapes ─────────────────────────────────────────────────────────────

/// The kind of a POWL node — a closed enumeration of the POWL operators.
///
/// This is **structure only**: it records *what the node is*, never *how it
/// runs*. It does NOT execute, replay, or unfold the operator.
///
/// The [`PowlNodeKind::ChoiceGraph`] variant represents the POWL 2.0
/// choice-graph operator (Kourani et al., 2026), which replaces the flat
/// `Choice` and `Loop` operators with a directed-graph structure capable of
/// expressing non-block-structured decisions and cycles.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PowlNodeKind {
    /// A single activity leaf. Carries the activity label.
    Atom(String),
    /// A silent (tau) step.
    Silent,
    /// An exclusive choice among child node ids (POWL 1.0 flat XOR).
    Choice(Vec<PowlNodeId>),
    /// A loop: a `do` body and an optional `redo` body (POWL 1.0 loop).
    Loop {
        /// The mandatory loop body.
        body: PowlNodeId,
        /// The optional re-do body (`None` => `do` once, no rework).
        redo: Option<PowlNodeId>,
    },
    /// A partial order over child node ids; precedence lives in [`OrderEdge`]s.
    PartialOrder(Vec<PowlNodeId>),
    /// A POWL 2.0 choice graph `γ = (N, E)` (Kourani et al., 2026 Def. 3.6).
    ///
    /// The choice graph replaces the flat `×` (XOR) and `↺` (loop) operators
    /// with a directed graph over decision nodes `X`, a unique start node `▷`
    /// (represented by the first element of `nodes` by convention), and a
    /// unique end node `□` (last element). Every node must lie on a connected
    /// path from start to end; structural disconnection is refused as
    /// [`PowlRefusal::ChoiceGraphDisconnected`].
    ///
    /// Connectivity checking and replay graduate to `wasm4pm`.
    ChoiceGraph {
        /// The node ids forming the choice-graph node set `N = X ∪ {▷, □}`.
        nodes: Vec<PowlNodeId>,
        /// The directed edges `E` of the choice graph.
        edges: Vec<ChoiceGraphEdge>,
    },
}

/// A single node of a POWL model, tagged with a witness `W`.
///
/// The witness `W` is a zero-sized type marker (e.g. [`Atom`],
/// [`PartialOrder`]) recording the structural family of the node at the type
/// level. It represents the node's *shape* and does **not** confer any
/// execution capability.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PowlNode<W = ()> {
    /// The node's identifier within its model.
    pub id: PowlNodeId,
    /// The structural kind of the node.
    pub kind: PowlNodeKind,
    /// Type-level witness of the node's structural family.
    pub witness: PhantomData<W>,
}

impl<W> PowlNode<W> {
    /// Construct a witnessed node from an id and kind.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::powl::{PowlNode, PowlNodeId, PowlNodeKind, Atom};
    /// let n = PowlNode::<Atom>::new(PowlNodeId(0), PowlNodeKind::Atom("a".into()));
    /// assert_eq!(n.id, PowlNodeId(0));
    /// ```
    pub fn new(id: PowlNodeId, kind: PowlNodeKind) -> Self {
        Self {
            id,
            kind,
            witness: PhantomData,
        }
    }
}

// ── Typed choice and loop node structs ──────────────────────────────────────

/// A typed exclusive-choice node: a POWL XOR operator with its branch ids.
///
/// A well-formed choice node requires **at least two branches** — a single
/// branch is a no-op that degrades to a plain sequence and is refused as
/// [`PowlRefusal::InvalidChoice`]. An empty branch list is also refused.
///
/// This struct is distinct from the [`Choice`] witness marker: [`Choice`]
/// records the *kind* of a [`PowlNode`] at the type level; `PowlChoiceNode`
/// is the concrete value that carries the branch list.
///
/// Structure-only: records which nodes are branches of this choice. No
/// decision semantics, no replay. Graduate to `wasm4pm` to execute.
///
/// # Examples
///
/// ```
/// use wasm4pm_compat::powl::{PowlChoiceNode, PowlNodeId};
/// let c = PowlChoiceNode::new(vec![PowlNodeId(1), PowlNodeId(2)]);
/// assert_eq!(c.branch_count(), 2);
/// assert!(c.is_well_formed());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PowlChoiceNode {
    /// The branch node ids (must contain ≥ 2 to be well-formed).
    pub branches: Vec<PowlNodeId>,
}

impl PowlChoiceNode {
    /// Construct a choice node from a branch list.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::powl::{PowlChoiceNode, PowlNodeId};
    /// let c = PowlChoiceNode::new(vec![PowlNodeId(0), PowlNodeId(1)]);
    /// assert_eq!(c.branch_count(), 2);
    /// ```
    pub fn new(branches: Vec<PowlNodeId>) -> Self {
        Self { branches }
    }

    /// Number of branches in this choice.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::powl::{PowlChoiceNode, PowlNodeId};
    /// let c = PowlChoiceNode::new(vec![PowlNodeId(0), PowlNodeId(1), PowlNodeId(2)]);
    /// assert_eq!(c.branch_count(), 3);
    /// ```
    #[inline]
    pub fn branch_count(&self) -> usize {
        self.branches.len()
    }

    /// Returns `true` when the choice node has at least two branches.
    ///
    /// A choice with fewer than two branches violates the POWL law
    /// (it degrades to a trivial projection and is refused as
    /// [`PowlRefusal::InvalidChoice`]).
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::powl::{PowlChoiceNode, PowlNodeId};
    /// let ok  = PowlChoiceNode::new(vec![PowlNodeId(0), PowlNodeId(1)]);
    /// let bad = PowlChoiceNode::new(vec![PowlNodeId(0)]);
    /// assert!(ok.is_well_formed());
    /// assert!(!bad.is_well_formed());
    /// ```
    #[inline]
    pub fn is_well_formed(&self) -> bool {
        self.branches.len() >= 2
    }

    /// Attempt to validate the choice node, returning the branches on success
    /// or [`PowlRefusal::InvalidChoice`] when fewer than two branches are present.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::powl::{PowlChoiceNode, PowlNodeId, PowlRefusal};
    /// let bad = PowlChoiceNode::new(vec![PowlNodeId(0)]);
    /// assert_eq!(bad.validate(), Err(PowlRefusal::InvalidChoice));
    /// let ok  = PowlChoiceNode::new(vec![PowlNodeId(0), PowlNodeId(1)]);
    /// assert!(ok.validate().is_ok());
    /// ```
    #[must_use = "check the shape-check result"]
    pub fn validate(&self) -> Result<&[PowlNodeId], PowlRefusal> {
        if self.is_well_formed() {
            Ok(&self.branches)
        } else {
            Err(PowlRefusal::InvalidChoice)
        }
    }
}

/// A typed loop node with its arity enforced as a const generic parameter.
///
/// Paper: Kourani et al. (2026) §3 — a POWL loop `L(M₁, M₂)` has exactly
/// **two** children: the mandatory `do` body (`M₁`) and the `redo` body (`M₂`).
/// `TypedPowlLoopNode<_, 3>` does **not compile**: `ARITY == 2` is violated.
///
/// This mirrors [`crate::process_tree::TypedLoopNode`] but lives in the POWL
/// domain, where the arity law applies to the POWL loop operator specifically.
///
/// Structure-only: the arity constraint is a type-law receipt. It does not
/// replay, unfold, or execute the loop. Graduate to `wasm4pm` for execution.
///
/// ```
/// # #![feature(generic_const_exprs)]
/// # #![allow(incomplete_features)]
/// use wasm4pm_compat::powl::TypedPowlLoopNode;
/// let _: TypedPowlLoopNode<(), 2> = TypedPowlLoopNode::new(());  // arity 2: lawful
/// ```
///
/// ```compile_fail
/// use wasm4pm_compat::powl::TypedPowlLoopNode;
/// let _: TypedPowlLoopNode<(), 3> = TypedPowlLoopNode::new(());  // arity 3: compile error
/// ```
pub struct TypedPowlLoopNode<Children, const ARITY: usize>
where
    Require<{ ARITY == 2 }>: IsTrue,
{
    /// The loop children (do body + redo body), provided by the caller.
    pub children: Children,
}

impl<Children, const ARITY: usize> TypedPowlLoopNode<Children, ARITY>
where
    Require<{ ARITY == 2 }>: IsTrue,
{
    /// Construct a `TypedPowlLoopNode` — only possible when `ARITY == 2`.
    ///
    /// ```
    /// # #![feature(generic_const_exprs)]
    /// # #![allow(incomplete_features)]
    /// use wasm4pm_compat::powl::TypedPowlLoopNode;
    /// let node: TypedPowlLoopNode<[&str; 2], 2> = TypedPowlLoopNode::new(["do", "redo"]);
    /// assert_eq!(node.children, ["do", "redo"]);
    /// ```
    pub fn new(children: Children) -> Self {
        TypedPowlLoopNode { children }
    }
}

/// A directed precedence edge inside a [`PowlNodeKind::PartialOrder`].
///
/// `from` must complete before `to` may start. This is a *structural* claim of
/// precedence; it is never *enforced* by execution here.
///
/// This type is **distinct** from [`ChoiceGraphEdge`]: an `OrderEdge` expresses
/// sequential precedence inside a partial order; a [`ChoiceGraphEdge`] expresses
/// a directed transition inside a POWL 2.0 choice graph. They are not
/// interchangeable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrderEdge {
    /// The predecessor node id.
    pub from: PowlNodeId,
    /// The successor node id.
    pub to: PowlNodeId,
}

impl OrderEdge {
    /// Construct a precedence edge `from -> to`.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::powl::{OrderEdge, PowlNodeId};
    /// let e = OrderEdge::new(PowlNodeId(0), PowlNodeId(1));
    /// assert_eq!(e.from, PowlNodeId(0));
    /// assert_eq!(e.to, PowlNodeId(1));
    /// ```
    pub fn new(from: PowlNodeId, to: PowlNodeId) -> Self {
        Self { from, to }
    }
}

/// A directed edge inside a [`PowlNodeKind::ChoiceGraph`].
///
/// Kourani et al. (2026) Definition 3.6 introduces the choice graph
/// `γ = (N, E)` where `N = X ∪ {▷, □}` and `E` is a set of directed arcs.
/// Each `ChoiceGraphEdge` is one such arc: a directed step from one choice-graph
/// node to another.
///
/// This type is **structurally distinct** from [`OrderEdge`]: a
/// `ChoiceGraphEdge` is a transition inside a choice graph (decision/cyclic
/// logic), while an `OrderEdge` is a precedence constraint inside a partial
/// order (scheduling logic). The types are not interchangeable at the call site;
/// a function accepting `ChoiceGraphEdge` will not compile with `OrderEdge`.
///
/// Structure-only: a typed directed arc. No decision semantics.
///
/// ```
/// use wasm4pm_compat::powl::{ChoiceGraphEdge, PowlNodeId};
/// let e = ChoiceGraphEdge::new(PowlNodeId(0), PowlNodeId(1));
/// assert_eq!(e.from, PowlNodeId(0));
/// assert_eq!(e.to, PowlNodeId(1));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChoiceGraphEdge {
    /// The source node id in the choice graph.
    pub from: PowlNodeId,
    /// The target node id in the choice graph.
    pub to: PowlNodeId,
}

impl ChoiceGraphEdge {
    /// Construct a choice-graph edge `from -> to`.
    ///
    /// ```
    /// use wasm4pm_compat::powl::{ChoiceGraphEdge, PowlNodeId};
    /// let e = ChoiceGraphEdge::new(PowlNodeId(2), PowlNodeId(3));
    /// assert_eq!(e.from, PowlNodeId(2));
    /// ```
    pub fn new(from: PowlNodeId, to: PowlNodeId) -> Self {
        Self { from, to }
    }
}

/// A complete POWL model: a set of nodes, the precedence edges among them, and
/// the designated root.
///
/// This is the top-level **shape** of a POWL model. It represents a
/// partially-ordered workflow and does **NOT** discover, simplify, replay, or
/// measure conformance against it. When execution is required, graduate to
/// `wasm4pm`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Powl {
    /// All nodes, in id order (untyped at the collection level).
    pub nodes: Vec<PowlNode>,
    /// Precedence edges referenced by partial-order nodes.
    pub edges: Vec<OrderEdge>,
    /// The root node id, if the model is non-empty.
    pub root: Option<PowlNodeId>,
}

impl Powl {
    /// Construct an empty POWL model.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::powl::Powl;
    /// let p = Powl::new();
    /// assert!(p.root.is_none());
    /// assert_eq!(p.nodes.len(), 0);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Number of nodes in the model.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::powl::Powl;
    /// assert_eq!(Powl::new().node_count(), 0);
    /// ```
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Structurally validate the POWL model, checking Choice nodes, Loop nodes, Partial orders, and Choice graphs.
    pub fn validate(&self) -> Result<(), PowlRefusal> {
        for node in &self.nodes {
            match &node.kind {
                PowlNodeKind::Choice(branches) => {
                    if branches.len() < 2 {
                        return Err(PowlRefusal::InvalidChoice);
                    }
                }
                PowlNodeKind::Loop { body, redo } => {
                    let node_ids: std::collections::HashSet<usize> =
                        self.nodes.iter().map(|n| n.id.0).collect();
                    if !node_ids.contains(&body.0) {
                        return Err(PowlRefusal::InvalidLoop);
                    }
                    if let Some(r) = redo {
                        if !node_ids.contains(&r.0) {
                            return Err(PowlRefusal::InvalidLoop);
                        }
                    }
                }
                PowlNodeKind::PartialOrder(children) => {
                    let child_set: std::collections::HashSet<PowlNodeId> =
                        children.iter().cloned().collect();
                    let mut adj: std::collections::HashMap<PowlNodeId, Vec<PowlNodeId>> =
                        std::collections::HashMap::new();
                    let mut in_degree: std::collections::HashMap<PowlNodeId, usize> =
                        std::collections::HashMap::new();

                    for &c in children {
                        adj.entry(c).or_default();
                        in_degree.entry(c).or_insert(0);
                    }

                    for edge in &self.edges {
                        if child_set.contains(&edge.from) && child_set.contains(&edge.to) {
                            adj.entry(edge.from).or_default().push(edge.to);
                            *in_degree.entry(edge.to).or_insert(0) += 1;
                        }
                    }

                    let mut queue: std::collections::VecDeque<PowlNodeId> = children
                        .iter()
                        .copied()
                        .filter(|c| in_degree.get(c).copied().unwrap_or(0) == 0)
                        .collect();

                    let mut visited = 0;
                    while let Some(u) = queue.pop_front() {
                        visited += 1;
                        if let Some(neighbors) = adj.get(&u) {
                            for &v in neighbors {
                                if let Some(deg) = in_degree.get_mut(&v) {
                                    *deg -= 1;
                                    if *deg == 0 {
                                        queue.push_back(v);
                                    }
                                }
                            }
                        }
                    }

                    if visited != children.len() {
                        return Err(PowlRefusal::CyclicPartialOrder);
                    }
                }
                PowlNodeKind::ChoiceGraph {
                    nodes: cg_nodes,
                    edges: cg_edges,
                } => {
                    if cg_nodes.len() < 2 {
                        return Err(PowlRefusal::ChoiceGraphDisconnected);
                    }
                    let start = cg_nodes[0];
                    let end = cg_nodes[cg_nodes.len() - 1];

                    let node_set: std::collections::HashSet<PowlNodeId> =
                        cg_nodes.iter().cloned().collect();
                    for edge in cg_edges {
                        if !node_set.contains(&edge.from) || !node_set.contains(&edge.to) {
                            return Err(PowlRefusal::ChoiceGraphDisconnected);
                        }
                    }

                    let mut forward_adj: std::collections::HashMap<PowlNodeId, Vec<PowlNodeId>> =
                        std::collections::HashMap::new();
                    let mut backward_adj: std::collections::HashMap<PowlNodeId, Vec<PowlNodeId>> =
                        std::collections::HashMap::new();
                    for edge in cg_edges {
                        forward_adj.entry(edge.from).or_default().push(edge.to);
                        backward_adj.entry(edge.to).or_default().push(edge.from);
                    }

                    let mut forward_visited = std::collections::HashSet::new();
                    let mut queue = std::collections::VecDeque::new();
                    queue.push_back(start);
                    forward_visited.insert(start);
                    while let Some(u) = queue.pop_front() {
                        if let Some(neighbors) = forward_adj.get(&u) {
                            for &v in neighbors {
                                if forward_visited.insert(v) {
                                    queue.push_back(v);
                                }
                            }
                        }
                    }

                    let mut backward_visited = std::collections::HashSet::new();
                    queue.clear();
                    queue.push_back(end);
                    backward_visited.insert(end);
                    while let Some(u) = queue.pop_front() {
                        if let Some(parents) = backward_adj.get(&u) {
                            for &v in parents {
                                if backward_visited.insert(v) {
                                    queue.push_back(v);
                                }
                            }
                        }
                    }

                    for &node in cg_nodes {
                        if !forward_visited.contains(&node) || !backward_visited.contains(&node) {
                            return Err(PowlRefusal::ChoiceGraphDisconnected);
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}

// ── First-class refusal surface ─────────────────────────────────────────────

/// First-class refusal law for POWL shapes.
///
/// Every variant names a **specific** structural law — never a bare
/// "InvalidInput". A refusal is a *verdict about shape*, not a runtime error.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PowlRefusal {
    /// A partial order contained a cycle — precedence must be acyclic.
    CyclicPartialOrder,
    /// A choice node was malformed (e.g. fewer than two branches).
    InvalidChoice,
    /// A choice node had the wrong number of branches — arity violation.
    ///
    /// Distinct from [`PowlRefusal::InvalidChoice`]: this variant names the
    /// arity law specifically (the branch count was structurally wrong, not
    /// merely malformed in some other way). `InvalidChoice` covers the general
    /// case; `InvalidChoiceArity` carries the specific arity violation evidence.
    InvalidChoiceArity {
        /// The number of branches declared.
        declared: usize,
        /// The minimum number of branches required (always ≥ 2).
        required_min: usize,
    },
    /// A loop node was malformed (e.g. missing `do` body).
    InvalidLoop,
    /// A loop node is missing its mandatory `do` body — the first child of a
    /// POWL loop `L(M₁, M₂)` is the `do` body and must always be present.
    ///
    /// Paper: Kourani et al. (2026) §3 — `L(M₁, M₂)` requires `M₁` (do body).
    LoopMissingDoBody,
    /// Projection to a process tree was requested for an
    /// [`Irreducible`] partial order that [`ExceedsProcessTree`].
    IrreducibleProjection,
    /// The claimed language of the POWL model does not match the admitted
    /// reference language.
    LanguageMismatch,
    /// A [`PowlNodeKind::ChoiceGraph`] is disconnected — at least one node is
    /// not on any connected path from the start node `▷` to the end node `□`.
    ///
    /// Law: Kourani et al. (2026) Definition 3.6 — every node in a choice graph
    /// must lie on a path from the unique start node to the unique end node.
    /// Connectivity verification graduates to `wasm4pm`; this refusal is raised
    /// when structural analysis finds a node unreachable from the declared start
    /// or unable to reach the declared end.
    ChoiceGraphDisconnected,
}

impl core::fmt::Display for PowlRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            PowlRefusal::CyclicPartialOrder => write!(f, "POWL refused: CyclicPartialOrder"),
            PowlRefusal::InvalidChoice => write!(f, "POWL refused: InvalidChoice"),
            PowlRefusal::InvalidChoiceArity { declared, required_min } => write!(
                f,
                "POWL refused: InvalidChoiceArity (declared={declared}, required_min={required_min})"
            ),
            PowlRefusal::InvalidLoop => write!(f, "POWL refused: InvalidLoop"),
            PowlRefusal::LoopMissingDoBody => write!(f, "POWL refused: LoopMissingDoBody"),
            PowlRefusal::IrreducibleProjection => {
                write!(f, "POWL refused: IrreducibleProjection")
            }
            PowlRefusal::LanguageMismatch => write!(f, "POWL refused: LanguageMismatch"),
            PowlRefusal::ChoiceGraphDisconnected => {
                write!(f, "POWL refused: ChoiceGraphDisconnected")
            }
        }
    }
}

// ── Standalone ChoiceGraph (mirrors wasm4pm-types choice_graph.rs) ──────────

/// A node in a standalone [`ChoiceGraph`].
///
/// Mirrors `wasm4pm_compat::ChoiceGraphNode`. The [`PowlNodeKind::ChoiceGraph`]
/// variant embeds choice-graph nodes by [`PowlNodeId`] reference; this enum is
/// used when representing a *self-contained* choice graph outside a [`Powl`]
/// arena (e.g. for serialisation, standalone traversal, or interop with the
/// wasm4pm-types crate).
///
/// Variants follow the wasm4pm-types definition:
/// - `Start` / `End` are the unique boundary markers required by Definition 1
///   (Kourani et al., arXiv:2505.07052).
/// - `Activity` is an inline activity label.
/// - `SubModel` references a sub-model by arena index (`u32`), matching the
///   `SubModel(u32)` variant in wasm4pm-types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StandaloneChoiceGraphNode {
    /// Unique start marker — no incoming edges.
    Start,
    /// Unique end marker — no outgoing edges.
    End,
    /// Inline activity by label.
    Activity(String),
    /// Reference to a sub-model by arena root index.
    SubModel(u32),
}

/// Backward-compatible alias — consumers can use `ChoiceGraphNode` after
/// switching to `wasm4pm_compat::powl`. Identical to [`StandaloneChoiceGraphNode`].
pub type ChoiceGraphNode = StandaloneChoiceGraphNode;

/// A standalone choice graph: nodes plus directed index-pair edges, with
/// explicit `start_idx` / `end_idx` fields.
///
/// This type mirrors `wasm4pm_compat::ChoiceGraph` and supports standalone graph
/// traversal via [`ChoiceGraph::successors`], [`ChoiceGraph::predecessors`], and
/// [`ChoiceGraph::has_empty_path`].
///
/// It is **distinct** from the inline [`PowlNodeKind::ChoiceGraph`] variant,
/// which references nodes by [`PowlNodeId`] inside a [`Powl`] arena. Use this
/// type when you need a self-contained, arena-free choice graph — e.g. for
/// direct construction, serialisation, or round-tripping through wasm4pm-types.
///
/// ## Validity
///
/// The struct carries no construction-time validation (unlike the
/// `wasm4pm_compat::ChoiceGraph::new` constructor which enforces Definition 1).
/// Validated construction, cycle-detection, and replay graduate to `wasm4pm`.
///
/// # Examples
///
/// ```
/// use wasm4pm_compat::powl::{ChoiceGraph, StandaloneChoiceGraphNode};
///
/// let cg = ChoiceGraph {
///     nodes: vec![StandaloneChoiceGraphNode::Start, StandaloneChoiceGraphNode::End],
///     edges: vec![(0, 1)],
///     start_idx: 0,
///     end_idx: 1,
/// };
/// assert!(cg.has_empty_path());
/// assert_eq!(cg.successors(0), vec![1]);
/// assert_eq!(cg.predecessors(1), vec![0]);
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChoiceGraph {
    /// The node list. `nodes[start_idx]` is the unique start; `nodes[end_idx]`
    /// is the unique end.
    pub nodes: Vec<StandaloneChoiceGraphNode>,
    /// Directed edges as `(from_index, to_index)` pairs into `nodes`.
    pub edges: Vec<(usize, usize)>,
    /// Index of the unique start node in `nodes`.
    pub start_idx: usize,
    /// Index of the unique end node in `nodes`.
    pub end_idx: usize,
}

impl ChoiceGraph {
    /// Construct from nodes and index-pair edges.
    ///
    /// `start_idx` defaults to 0 (first node); `end_idx` defaults to
    /// `nodes.len() - 1` (last node). Pass `start_idx`/`end_idx` explicitly
    /// via struct literal when the boundary nodes are not at the endpoints.
    ///
    /// ```
    /// use wasm4pm_compat::powl::{ChoiceGraph, StandaloneChoiceGraphNode};
    /// let cg = ChoiceGraph::new(
    ///     vec![StandaloneChoiceGraphNode::Start, StandaloneChoiceGraphNode::End],
    ///     vec![(0, 1)],
    /// );
    /// assert_eq!(cg.start_idx, 0);
    /// assert_eq!(cg.end_idx, 1);
    /// ```
    pub fn new(nodes: Vec<StandaloneChoiceGraphNode>, edges: Vec<(usize, usize)>) -> Self {
        let end_idx = nodes.len().saturating_sub(1);
        ChoiceGraph {
            nodes,
            edges,
            start_idx: 0,
            end_idx,
        }
    }

    /// Collect the indices of all direct successors of `node_idx`.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::powl::{ChoiceGraph, StandaloneChoiceGraphNode};
    /// let cg = ChoiceGraph {
    ///     nodes: vec![
    ///         StandaloneChoiceGraphNode::Start,
    ///         StandaloneChoiceGraphNode::Activity("a".into()),
    ///         StandaloneChoiceGraphNode::End,
    ///     ],
    ///     edges: vec![(0, 1), (1, 2)],
    ///     start_idx: 0,
    ///     end_idx: 2,
    /// };
    /// assert_eq!(cg.successors(0), vec![1]);
    /// assert_eq!(cg.successors(1), vec![2]);
    /// ```
    pub fn successors(&self, node_idx: usize) -> Vec<usize> {
        self.edges
            .iter()
            .filter_map(|&(a, b)| if a == node_idx { Some(b) } else { None })
            .collect()
    }

    /// Collect the indices of all direct predecessors of `node_idx`.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::powl::{ChoiceGraph, StandaloneChoiceGraphNode};
    /// let cg = ChoiceGraph {
    ///     nodes: vec![
    ///         StandaloneChoiceGraphNode::Start,
    ///         StandaloneChoiceGraphNode::Activity("a".into()),
    ///         StandaloneChoiceGraphNode::End,
    ///     ],
    ///     edges: vec![(0, 1), (1, 2)],
    ///     start_idx: 0,
    ///     end_idx: 2,
    /// };
    /// assert_eq!(cg.predecessors(2), vec![1]);
    /// ```
    pub fn predecessors(&self, node_idx: usize) -> Vec<usize> {
        self.edges
            .iter()
            .filter_map(|&(a, b)| if b == node_idx { Some(a) } else { None })
            .collect()
    }

    /// Returns `true` iff there is a direct edge from `start_idx` to `end_idx`
    /// (the empty path — a choice that can be skipped entirely).
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::powl::{ChoiceGraph, StandaloneChoiceGraphNode};
    /// let cg = ChoiceGraph {
    ///     nodes: vec![StandaloneChoiceGraphNode::Start, StandaloneChoiceGraphNode::End],
    ///     edges: vec![(0, 1)],
    ///     start_idx: 0,
    ///     end_idx: 1,
    /// };
    /// assert!(cg.has_empty_path());
    /// ```
    pub fn has_empty_path(&self) -> bool {
        self.edges
            .iter()
            .any(|&(a, b)| a == self.start_idx && b == self.end_idx)
    }
}

// ── RefusedProjection marker ──────────────────────────────────────────────────

/// Typed marker carrying the **named reason** a POWL projection was refused.
///
/// A POWL projection (e.g. POWL → process tree) that cannot proceed is not
/// an untyped error — it is a *named refusal*. `RefusedProjection<R>` carries
/// the specific [`PowlRefusal`] reason `R` as a zero-cost `PhantomData` type
/// parameter, making the refusal auditable without heap allocation.
///
/// The concrete reason value is carried alongside: callers can inspect both
/// the type-level `R` (for static dispatch) and the runtime `.reason()`.
///
/// Structure-only: a `RefusedProjection` is a verdict, not a recovery tool.
/// Graduate to `wasm4pm` to act on admitted shapes.
///
/// # Examples
///
/// ```
/// use wasm4pm_compat::powl::{RefusedProjection, PowlRefusal};
/// let r = RefusedProjection::new(PowlRefusal::IrreducibleProjection);
/// assert_eq!(r.reason(), &PowlRefusal::IrreducibleProjection);
/// assert_eq!(format!("{}", r), "POWL refused: IrreducibleProjection");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefusedProjection {
    reason: PowlRefusal,
}

impl RefusedProjection {
    /// Construct a refused-projection marker from a named refusal reason.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::powl::{RefusedProjection, PowlRefusal};
    /// let r = RefusedProjection::new(PowlRefusal::CyclicPartialOrder);
    /// assert_eq!(r.reason(), &PowlRefusal::CyclicPartialOrder);
    /// ```
    #[inline]
    pub fn new(reason: PowlRefusal) -> Self {
        Self { reason }
    }

    /// The named refusal reason.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::powl::{RefusedProjection, PowlRefusal};
    /// let r = RefusedProjection::new(PowlRefusal::IrreducibleProjection);
    /// assert_eq!(r.reason(), &PowlRefusal::IrreducibleProjection);
    /// ```
    #[inline]
    pub fn reason(&self) -> &PowlRefusal {
        &self.reason
    }

    /// Consume the marker, yielding the owned refusal reason.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::powl::{RefusedProjection, PowlRefusal};
    /// let r = RefusedProjection::new(PowlRefusal::LanguageMismatch);
    /// assert_eq!(r.into_reason(), PowlRefusal::LanguageMismatch);
    /// ```
    #[inline]
    pub fn into_reason(self) -> PowlRefusal {
        self.reason
    }
}

impl core::fmt::Display for RefusedProjection {
    /// Delegates to [`PowlRefusal`]'s `Display`.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::powl::{RefusedProjection, PowlRefusal};
    /// let r = RefusedProjection::new(PowlRefusal::InvalidChoice);
    /// assert_eq!(format!("{}", r), "POWL refused: InvalidChoice");
    /// ```
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(&self.reason, f)
    }
}

/// The eight POWL operator kinds (POWL8 — van der Aalst 2023 full set).
///
/// Variants beyond the original four (Sequence, XorChoice, Parallel, Loop) add
/// StrictPartialOrder, ChoiceGraph, Silent, and Activity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Powl8Op {
    /// Strict sequential composition.
    Sequence,
    /// Exclusive (XOR) choice.
    XorChoice,
    /// Parallel (AND) split-join.
    Parallel,
    /// Redo loop: body then optional redo branch.
    Loop,
    /// Strict partial order over sub-models.
    StrictPartialOrder,
    /// Non-block-structured choice over a directed acyclic graph of sub-models.
    ChoiceGraph,
    /// Silent (tau) step — no observable activity.
    Silent,
    /// Leaf atom naming a single activity.
    Activity,
}

/// Graduation witness: a `WfNetConst` has been successfully converted to a
/// `Powl` model under the POWL 2.0 decomposition theorem.
///
/// ## Paper
///
/// Kourani, Park & van der Aalst (2026) — Theorem 4.3: a separable WF-net can
/// be converted to a POWL 2.0 model while preserving the process language. This
/// witness records that the conversion took place under the separability
/// precondition (`SeparableWfNet`) and produced an equivalent POWL model.
///
/// ## How to obtain
///
/// A `WfNet2PowlWitness` is only constructible inside this module or via the
/// `wasm4pm` graduation bridge that performs the actual conversion. It cannot
/// be forged externally.
///
/// ## Structure-only
///
/// The witness carries a label naming the conversion context. The POWL model
/// itself is returned separately; this witness travels alongside it as a
/// provenance claim.
mod wfnet2powl_seal {
    /// Private seal for `WfNet2PowlWitness` — prevents external construction.
    pub(super) struct WfNet2PowlSeal;
}

pub struct WfNet2PowlWitness {
    /// A label naming the conversion context (e.g. the WF-net id).
    pub context: String,
    // Private seal — only constructible inside this module or wasm4pm bridge.
    _seal: wfnet2powl_seal::WfNet2PowlSeal,
}

impl WfNet2PowlWitness {
    /// Module-internal constructor — only `powl` and the `wasm4pm` bridge may
    /// produce a witness.
    ///
    /// ```
    /// use wasm4pm_compat::powl::WfNet2PowlWitness;
    /// let w = WfNet2PowlWitness::new_internal("wfnet-42");
    /// assert_eq!(w.context, "wfnet-42");
    /// ```
    pub fn new_internal(context: impl Into<String>) -> Self {
        WfNet2PowlWitness {
            context: context.into(),
            _seal: wfnet2powl_seal::WfNet2PowlSeal,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_powl_validate_empty() {
        let p = Powl::new();
        assert!(p.validate().is_ok());
    }

    #[test]
    fn test_powl_validate_invalid_choice() {
        let mut p = Powl::new();
        p.nodes.push(PowlNode::new(
            PowlNodeId(0),
            PowlNodeKind::Choice(vec![PowlNodeId(1)]),
        ));
        assert_eq!(p.validate(), Err(PowlRefusal::InvalidChoice));
    }

    #[test]
    fn test_powl_validate_cyclic_partial_order() {
        let mut p = Powl::new();
        p.nodes.push(PowlNode::new(
            PowlNodeId(0),
            PowlNodeKind::PartialOrder(vec![PowlNodeId(1), PowlNodeId(2)]),
        ));
        p.edges.push(OrderEdge::new(PowlNodeId(1), PowlNodeId(2)));
        p.edges.push(OrderEdge::new(PowlNodeId(2), PowlNodeId(1)));
        assert_eq!(p.validate(), Err(PowlRefusal::CyclicPartialOrder));
    }

    #[test]
    fn test_powl_validate_choice_graph_disconnected() {
        let mut p = Powl::new();
        let start = PowlNodeId(0);
        let x1 = PowlNodeId(1);
        let end = PowlNodeId(2);
        p.nodes.push(PowlNode::new(
            PowlNodeId(10),
            PowlNodeKind::ChoiceGraph {
                nodes: vec![start, x1, end],
                edges: vec![ChoiceGraphEdge::new(start, end)], // x1 is disconnected
            },
        ));
        assert_eq!(p.validate(), Err(PowlRefusal::ChoiceGraphDisconnected));
    }
}
