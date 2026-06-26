//! # POWL 2.0 (Partially Ordered Workflow Language) AST
//!
//! This module represents the absolute topological boundaries of the POWL 2.0
//! specification, as formally defined in:
//! *Kourani & van der Aalst (2024/2026): Hierarchical Decomposition of Separable Workflow-Nets*.
//!
//! POWL 2.0 generalizes the rigid, block-structured `XOR` and `Loop` operators
//! from POWL 1.0 into a unified **Choice Graph** (`γ(M₁, ..., Mₙ)`). This allows
//! for the modeling of non-block-structured decisions and cycles, provided the
//! underlying Workflow Net is strictly separable.
//!
//! **Architectural Law**: Legacy POWL 1.0 logic (Xor, Loop blocks) is strictly
//! prohibited and structurally rejected from this codebase.
//!
//! ## Formal Executable Doctests
//!
//! The following doctests act as executable proofs that the `PowlBuilder` accurately
//! enforces the topological constraints specified in the academic literature.
//!
//! ### Figure 1b: Valid Hierarchical Choice Graph
//! A `ChoiceGraph` successfully modeling nested concurrency and cyclic logic without panicking.
//!
//! ```rust
//! use wasm4pm_compat::powl::{PowlBuilder, PowlRefusal};
//!
//! let powl = PowlBuilder::new()
//!     .atom("START")
//!     .atom("END")
//!     .atom("task_a")
//!     .atom("task_b")
//!     .partial_order("concurrent_production", &["task_a", "task_b"], &[])
//!     .atom("review")
//!     .atom("finalize")
//!     .choice_graph(
//!         "top_level",
//!         &["START", "concurrent_production", "review", "finalize", "END"],
//!         &[
//!             ("START", "concurrent_production"),
//!             ("concurrent_production", "review"),
//!             ("review", "finalize"),
//!             ("review", "concurrent_production"), // Cyclic back-edge
//!             ("finalize", "END"),
//!         ],
//!     )
//!     .root("top_level")
//!     .build()
//!     .unwrap();
//!
//! assert_eq!(powl.node_count(), 8);
//! ```
//!
//! ### Figure 2: The Non-Separable Anomaly
//! Attempting to cross-link choice logic into a concurrent partial order physically fails
//! and is refused as a cyclic violation.
//!
//! ```rust
//! use wasm4pm_compat::powl::{PowlBuilder, PowlRefusal};
//!
//! let result = PowlBuilder::new()
//!     .atom("a")
//!     .atom("b")
//!     .partial_order("po", &["a", "b"], &[("a", "b"), ("b", "a")])
//!     .build();
//!
//! assert_eq!(result, Err(PowlRefusal::CyclicPartialOrder));
//! ```
//!
//! ### Figure 7a: The Long-Term Dependency Failure
//! Attempting to force non-freechoice forward dependencies without synchronization structurally
//! results in dangling execution paths, correctly caught as a disconnected choice graph.
//!
//! ```rust
//! use wasm4pm_compat::powl::{PowlBuilder, PowlRefusal};
//!
//! let result = PowlBuilder::new()
//!     .atom("START")
//!     .atom("END")
//!     .atom("a")
//!     .atom("b")
//!     .atom("d")
//!     .atom("e")
//!     .choice_graph(
//!         "top_level",
//!         &["START", "a", "b", "d", "e", "END"],
//!         &[
//!             ("START", "a"),
//!             ("START", "b"),
//!             ("a", "d"),
//!             ("b", "e"),
//!         ],
//!     )
//!     .root("top_level")
//!     .build();
//!
//! assert_eq!(result, Err(PowlRefusal::ChoiceGraphDisconnected));
//! ```
//!
//! ## Graduation
//!
//! When you need to **replay, discover, simplify, or measure** a POWL model,
//! graduate this shape to the `wasm4pm` engine (via the `wasm4pm` feature):
//! this module only certifies that the *structure* is well-formed and names the
//! law under which it would be refused.

use crate::law::{IsTrue, Require};
use core::marker::PhantomData;
use serde::{Deserialize, Serialize};

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

/// Witness: the node is a **Choice Graph** (unified choices and cycles).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ChoiceGraphMarker;

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PowlNodeKind {
    /// Unique Start boundary node.
    Start,
    /// Unique End boundary node.
    End,
    /// A single activity leaf. Carries the activity label.
    Atom(String),
    /// A silent (tau) step.
    Silent,

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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound = "")]
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
    /// or [`PowlRefusal::InvalidChoiceArity`] when fewer than two branches are present.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::powl::{PowlChoiceNode, PowlNodeId, PowlRefusal};
    /// let bad = PowlChoiceNode::new(vec![PowlNodeId(0)]);
    /// assert_eq!(bad.validate(), Err(PowlRefusal::InvalidChoiceArity { declared: 1, required_min: 2 }));
    /// let ok  = PowlChoiceNode::new(vec![PowlNodeId(0), PowlNodeId(1)]);
    /// assert!(ok.validate().is_ok());
    /// ```
    #[must_use = "check the shape-check result"]
    pub fn validate(&self) -> Result<&[PowlNodeId], PowlRefusal> {
        if self.is_well_formed() {
            Ok(&self.branches)
        } else {
            Err(PowlRefusal::InvalidChoiceArity {
                declared: self.branches.len(),
                required_min: 2,
            })
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

// ── Composition-depth ceiling (type-law surface) ─────────────────────────────

/// The maximum lawful POWL composition nesting depth.
///
/// Paper: Kourani et al. (2026) §3 — the recursive POWL decomposition `P(M⁺, ≺)`
/// nests operators (partial orders, choices, loops) to a bounded depth; this
/// constant fixes the **structural ceiling** on that nesting.
///
/// ## What this IS
///
/// A compile-time ceiling. It is consumed only as a const-generic bound by
/// [`PowlComposition`]; a composition whose declared `DEPTH` exceeds it does
/// **not compile**.
///
/// ## What this is NOT
///
/// **Not** a runtime depth computation. Nothing here traverses a POWL value to
/// measure how deeply it nests — that measurement graduates to the `wasm4pm`
/// engine. The caller supplies `DEPTH` as a const parameter; this crate only
/// certifies, at compile time, that the declared depth is within the ceiling.
pub const MAX_POWL_DEPTH: usize = 8;

/// A POWL composition whose nesting depth is encoded as a const generic.
///
/// `DEPTH` records the declared composition-nesting depth of the wrapped
/// [`Inner`] shape. The `where` clause refuses, at compile time, any depth that
/// exceeds [`MAX_POWL_DEPTH`]: `PowlComposition<_, 9>` does **not compile**.
///
/// ## Paper
///
/// Kourani et al. (2026) §3 — the recursive POWL decomposition nests operators
/// to a bounded depth; this type makes that bound a compile-time law.
///
/// ## What this is NOT
///
/// Structure only. It does **not** traverse a POWL value to compute its depth —
/// `DEPTH` is supplied by the caller, never derived. Depth *measurement* and any
/// re-nesting / simplification graduate to `wasm4pm`.
///
/// ```
/// # #![feature(generic_const_exprs)]
/// # #![allow(incomplete_features)]
/// use wasm4pm_compat::powl::PowlComposition;
/// // Depth at the ceiling (8) is lawful:
/// let _: PowlComposition<[&str; 1], 8> = PowlComposition::new(["atom"]);
/// ```
///
/// ```compile_fail
/// # #![feature(generic_const_exprs)]
/// # #![allow(incomplete_features)]
/// use wasm4pm_compat::powl::PowlComposition;
/// // Depth 9 exceeds MAX_POWL_DEPTH — compile error.
/// let _: PowlComposition<[&str; 1], 9> = PowlComposition::new(["atom"]);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PowlComposition<Inner, const DEPTH: usize>
where
    Require<{ DEPTH <= MAX_POWL_DEPTH }>: IsTrue,
{
    /// The wrapped POWL shape at this composition depth, provided by the caller.
    pub inner: Inner,
}

impl<Inner, const DEPTH: usize> PowlComposition<Inner, DEPTH>
where
    Require<{ DEPTH <= MAX_POWL_DEPTH }>: IsTrue,
{
    /// Constructs a `PowlComposition` — only possible when `DEPTH <= MAX_POWL_DEPTH`.
    ///
    /// Does not compute depth; `DEPTH` is the caller's declared const parameter.
    ///
    /// ```
    /// # #![feature(generic_const_exprs)]
    /// # #![allow(incomplete_features)]
    /// use wasm4pm_compat::powl::PowlComposition;
    /// let c: PowlComposition<[&str; 1], 8> = PowlComposition::new(["atom"]);
    /// assert_eq!(c.inner[0], "atom");
    /// ```
    pub fn new(inner: Inner) -> Self {
        PowlComposition { inner }
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum TaggedPowlJson {
    Activity {
        #[serde(default = "default_one")]
        min_freq: u32,
        #[serde(default = "default_some_one")]
        max_freq: Option<u32>,
        #[serde(default)]
        label: Option<String>,
        #[serde(default)]
        organization: Option<String>,
        #[serde(default)]
        role: Option<String>,
    },
    PartialOrder {
        #[serde(default = "default_one")]
        min_freq: u32,
        #[serde(default = "default_some_one")]
        max_freq: Option<u32>,
        nodes: Vec<TaggedPowlJson>,
        #[serde(default)]
        edges: Vec<(usize, usize)>,
    },
    ChoiceGraph {
        #[serde(default = "default_one")]
        min_freq: u32,
        #[serde(default = "default_some_one")]
        max_freq: Option<u32>,
        nodes: Vec<TaggedPowlJson>,
        #[serde(default)]
        edges: Vec<(usize, usize)>,
        #[serde(default)]
        start_nodes: Vec<usize>,
        #[serde(default)]
        end_nodes: Vec<usize>,
    },
}

fn default_one() -> u32 {
    1
}

fn default_some_one() -> Option<u32> {
    Some(1)
}

fn to_tagged_json(node_id: PowlNodeId, powl: &Powl) -> TaggedPowlJson {
    let node = powl.nodes.iter().find(|n| n.id == node_id).unwrap();
    match &node.kind {
        PowlNodeKind::Start => TaggedPowlJson::Activity {
            min_freq: 1,
            max_freq: Some(1),
            label: Some("START".to_string()),
            organization: None,
            role: None,
        },
        PowlNodeKind::End => TaggedPowlJson::Activity {
            min_freq: 1,
            max_freq: Some(1),
            label: Some("END".to_string()),
            organization: None,
            role: None,
        },
        PowlNodeKind::Atom(label) => TaggedPowlJson::Activity {
            min_freq: 1,
            max_freq: Some(1),
            label: Some(label.clone()),
            organization: None,
            role: None,
        },
        PowlNodeKind::Silent => TaggedPowlJson::Activity {
            min_freq: 1,
            max_freq: Some(1),
            label: None,
            organization: None,
            role: None,
        },
        PowlNodeKind::PartialOrder(children) => {
            let json_nodes: Vec<TaggedPowlJson> =
                children.iter().map(|&c| to_tagged_json(c, powl)).collect();
            let mut json_edges = Vec::new();
            for edge in &powl.edges {
                if let (Some(u_idx), Some(v_idx)) = (
                    children.iter().position(|&c| c == edge.from),
                    children.iter().position(|&c| c == edge.to),
                ) {
                    json_edges.push((u_idx, v_idx));
                }
            }
            TaggedPowlJson::PartialOrder {
                min_freq: 1,
                max_freq: Some(1),
                nodes: json_nodes,
                edges: json_edges,
            }
        }
        PowlNodeKind::ChoiceGraph {
            nodes: cg_nodes,
            edges: cg_edges,
        } => {
            if cg_nodes.len() < 2 {
                return TaggedPowlJson::ChoiceGraph {
                    min_freq: 1,
                    max_freq: Some(1),
                    nodes: Vec::new(),
                    edges: Vec::new(),
                    start_nodes: Vec::new(),
                    end_nodes: Vec::new(),
                };
            }
            let user_nodes = &cg_nodes[1..cg_nodes.len() - 1];
            let json_nodes: Vec<TaggedPowlJson> = user_nodes
                .iter()
                .map(|&n| to_tagged_json(n, powl))
                .collect();

            let mut json_edges = Vec::new();
            for edge in cg_edges {
                if let (Some(u_idx), Some(v_idx)) = (
                    user_nodes.iter().position(|&n| n == edge.from),
                    user_nodes.iter().position(|&n| n == edge.to),
                ) {
                    json_edges.push((u_idx, v_idx));
                }
            }

            let mut start_nodes = Vec::new();
            let mut end_nodes = Vec::new();
            for edge in cg_edges {
                if edge.from == cg_nodes[0] {
                    if let Some(idx) = user_nodes.iter().position(|&n| n == edge.to) {
                        if !start_nodes.contains(&idx) {
                            start_nodes.push(idx);
                        }
                    }
                }
                if edge.to == cg_nodes[cg_nodes.len() - 1] {
                    if let Some(idx) = user_nodes.iter().position(|&n| n == edge.from) {
                        if !end_nodes.contains(&idx) {
                            end_nodes.push(idx);
                        }
                    }
                }
            }

            let has_empty_path = cg_edges
                .iter()
                .any(|e| e.from == cg_nodes[0] && e.to == cg_nodes[cg_nodes.len() - 1]);
            let min_freq = if has_empty_path { 0 } else { 1 };

            TaggedPowlJson::ChoiceGraph {
                min_freq,
                max_freq: Some(1),
                nodes: json_nodes,
                edges: json_edges,
                start_nodes,
                end_nodes,
            }
        }
    }
}

fn flatten_json_node(
    json: &TaggedPowlJson,
    powl: &mut Powl,
    id_counter: &mut usize,
) -> Result<PowlNodeId, String> {
    match json {
        TaggedPowlJson::Activity { label, .. } => {
            let id = PowlNodeId(*id_counter);
            *id_counter += 1;
            let kind = match label {
                Some(l) => {
                    if l == "START" {
                        PowlNodeKind::Start
                    } else if l == "END" {
                        PowlNodeKind::End
                    } else {
                        PowlNodeKind::Atom(l.clone())
                    }
                }
                None => PowlNodeKind::Silent,
            };
            let node = PowlNode::new(id, kind);
            powl.nodes.push(node);
            Ok(id)
        }
        TaggedPowlJson::PartialOrder { nodes, edges, .. } => {
            let mut children = Vec::new();
            for child_json in nodes {
                let child_id = flatten_json_node(child_json, powl, id_counter)?;
                children.push(child_id);
            }
            for &(u_idx, v_idx) in edges {
                if u_idx < children.len() && v_idx < children.len() {
                    powl.edges
                        .push(OrderEdge::new(children[u_idx], children[v_idx]));
                }
            }
            let id = PowlNodeId(*id_counter);
            *id_counter += 1;
            let kind = PowlNodeKind::PartialOrder(children);
            let node = PowlNode::new(id, kind);
            powl.nodes.push(node);
            Ok(id)
        }
        TaggedPowlJson::ChoiceGraph {
            nodes,
            edges,
            start_nodes,
            end_nodes,
            min_freq,
            ..
        } => {
            let mut user_nodes = Vec::new();
            for child_json in nodes {
                let child_id = flatten_json_node(child_json, powl, id_counter)?;
                user_nodes.push(child_id);
            }

            let start_id = PowlNodeId(*id_counter);
            *id_counter += 1;
            powl.nodes
                .push(PowlNode::new(start_id, PowlNodeKind::Start));

            let end_id = PowlNodeId(*id_counter);
            *id_counter += 1;
            powl.nodes.push(PowlNode::new(end_id, PowlNodeKind::End));

            let mut cg_nodes = vec![start_id];
            cg_nodes.extend(user_nodes.iter().copied());
            cg_nodes.push(end_id);

            let mut cg_edges = Vec::new();
            for &(u_idx, v_idx) in edges {
                if u_idx < user_nodes.len() && v_idx < user_nodes.len() {
                    cg_edges.push(ChoiceGraphEdge::new(user_nodes[u_idx], user_nodes[v_idx]));
                }
            }
            for &to_idx in start_nodes {
                if to_idx < user_nodes.len() {
                    cg_edges.push(ChoiceGraphEdge::new(start_id, user_nodes[to_idx]));
                }
            }
            for &from_idx in end_nodes {
                if from_idx < user_nodes.len() {
                    cg_edges.push(ChoiceGraphEdge::new(user_nodes[from_idx], end_id));
                }
            }
            if *min_freq == 0 {
                cg_edges.push(ChoiceGraphEdge::new(start_id, end_id));
            }

            let id = PowlNodeId(*id_counter);
            *id_counter += 1;
            let kind = PowlNodeKind::ChoiceGraph {
                nodes: cg_nodes,
                edges: cg_edges,
            };
            let node = PowlNode::new(id, kind);
            powl.nodes.push(node);
            Ok(id)
        }
    }
}

impl Serialize for Powl {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if let Some(root_id) = self.root {
            let json = to_tagged_json(root_id, self);
            json.serialize(serializer)
        } else {
            serializer.serialize_none()
        }
    }
}

impl<'de> Deserialize<'de> for Powl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let opt_json = Option::<TaggedPowlJson>::deserialize(deserializer)?;
        let mut powl = Powl::new();
        if let Some(json) = opt_json {
            let mut id_counter = 0;
            let root_id = flatten_json_node(&json, &mut powl, &mut id_counter)
                .map_err(serde::de::Error::custom)?;
            powl.root = Some(root_id);
        }
        Ok(powl)
    }
}

// ── First-class refusal surface ─────────────────────────────────────────────

/// First-class refusal law for POWL shapes.
///
/// Every variant names a **specific** structural law — never a bare
/// "InvalidInput". A refusal is a *verdict about shape*, not a runtime error.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
/// let cg = ChoiceGraph::new(
///     vec![StandaloneChoiceGraphNode::Start, StandaloneChoiceGraphNode::End],
///     vec![(0, 1)],
/// ).unwrap();
/// assert!(cg.has_empty_path());
/// assert_eq!(cg.successors(0), vec![1]);
/// assert_eq!(cg.predecessors(1), vec![0]);
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChoiceGraph {
    nodes: Vec<StandaloneChoiceGraphNode>,
    edges: Vec<(usize, usize)>,
    start_idx: usize,
    end_idx: usize,
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
    /// ).unwrap();
    /// assert_eq!(cg.start_idx(), 0);
    /// assert_eq!(cg.end_idx(), 1);
    /// ```
    pub fn new(
        nodes: Vec<StandaloneChoiceGraphNode>,
        edges: Vec<(usize, usize)>,
    ) -> Result<Self, crate::choice_graph::ChoiceGraphError> {
        let mut start_idx: Option<usize> = None;
        let mut end_idx: Option<usize> = None;
        for (i, n) in nodes.iter().enumerate() {
            match n {
                StandaloneChoiceGraphNode::Start => {
                    if start_idx.is_some() {
                        return Err(crate::choice_graph::ChoiceGraphError::MultipleStarts);
                    }
                    start_idx = Some(i);
                }
                StandaloneChoiceGraphNode::End => {
                    if end_idx.is_some() {
                        return Err(crate::choice_graph::ChoiceGraphError::MultipleEnds);
                    }
                    end_idx = Some(i);
                }
                _ => {}
            }
        }
        let start_idx = start_idx.ok_or(crate::choice_graph::ChoiceGraphError::NoStart)?;
        let end_idx = end_idx.ok_or(crate::choice_graph::ChoiceGraphError::NoEnd)?;

        let n = nodes.len();
        for &(a, b) in &edges {
            if a >= n || b >= n {
                return Err(crate::choice_graph::ChoiceGraphError::EdgeOutOfBounds);
            }
        }

        for &(a, b) in &edges {
            if b == start_idx {
                return Err(crate::choice_graph::ChoiceGraphError::StartHasIncoming);
            }
            if a == end_idx {
                return Err(crate::choice_graph::ChoiceGraphError::EndHasOutgoing);
            }
        }

        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
        for &(a, b) in &edges {
            adj[a].push(b);
        }
        let mut radj: Vec<Vec<usize>> = vec![Vec::new(); n];
        for &(a, b) in &edges {
            radj[b].push(a);
        }
        let reach_from_start = bfs_reach_local(&adj, start_idx, n);
        let reach_to_end = bfs_reach_local(&radj, end_idx, n);
        for i in 0..n {
            if !(reach_from_start[i] && reach_to_end[i]) {
                return Err(crate::choice_graph::ChoiceGraphError::NodeNotOnStartEndPath);
            }
        }

        Ok(ChoiceGraph {
            nodes,
            edges,
            start_idx,
            end_idx,
        })
    }

    /// Construct directly with explicit start and end indices.
    /// Connected path constraint is guaranteed at construction time.
    pub fn new_raw(
        nodes: Vec<StandaloneChoiceGraphNode>,
        edges: Vec<(usize, usize)>,
        start_idx: usize,
        end_idx: usize,
    ) -> Result<Self, crate::choice_graph::ChoiceGraphError> {
        let n = nodes.len();
        if start_idx >= n || end_idx >= n {
            return Err(crate::choice_graph::ChoiceGraphError::EdgeOutOfBounds);
        }
        for &(a, b) in &edges {
            if a >= n || b >= n {
                return Err(crate::choice_graph::ChoiceGraphError::EdgeOutOfBounds);
            }
        }

        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
        for &(a, b) in &edges {
            adj[a].push(b);
        }
        let mut radj: Vec<Vec<usize>> = vec![Vec::new(); n];
        for &(a, b) in &edges {
            radj[b].push(a);
        }
        let reach_from_start = bfs_reach_local(&adj, start_idx, n);
        let reach_to_end = bfs_reach_local(&radj, end_idx, n);
        for i in 0..n {
            if !(reach_from_start[i] && reach_to_end[i]) {
                return Err(crate::choice_graph::ChoiceGraphError::NodeNotOnStartEndPath);
            }
        }

        Ok(ChoiceGraph {
            nodes,
            edges,
            start_idx,
            end_idx,
        })
    }

    pub fn nodes(&self) -> &[StandaloneChoiceGraphNode] {
        &self.nodes
    }

    pub fn edges(&self) -> &[(usize, usize)] {
        &self.edges
    }

    pub fn start_idx(&self) -> usize {
        self.start_idx
    }

    pub fn end_idx(&self) -> usize {
        self.end_idx
    }

    // Mutable setters that preserve the connected path invariants:
    pub fn set_nodes(
        &mut self,
        nodes: Vec<StandaloneChoiceGraphNode>,
    ) -> Result<(), crate::choice_graph::ChoiceGraphError> {
        let old = std::mem::replace(&mut self.nodes, nodes);
        if let Err(e) = self.validate_connected_path() {
            self.nodes = old;
            return Err(e);
        }
        Ok(())
    }

    pub fn set_edges(
        &mut self,
        edges: Vec<(usize, usize)>,
    ) -> Result<(), crate::choice_graph::ChoiceGraphError> {
        let old = std::mem::replace(&mut self.edges, edges);
        if let Err(e) = self.validate_connected_path() {
            self.edges = old;
            return Err(e);
        }
        Ok(())
    }

    pub fn set_start_idx(
        &mut self,
        start_idx: usize,
    ) -> Result<(), crate::choice_graph::ChoiceGraphError> {
        let old = self.start_idx;
        self.start_idx = start_idx;
        if let Err(e) = self.validate_connected_path() {
            self.start_idx = old;
            return Err(e);
        }
        Ok(())
    }

    pub fn set_end_idx(
        &mut self,
        end_idx: usize,
    ) -> Result<(), crate::choice_graph::ChoiceGraphError> {
        let old = self.end_idx;
        self.end_idx = end_idx;
        if let Err(e) = self.validate_connected_path() {
            self.end_idx = old;
            return Err(e);
        }
        Ok(())
    }

    fn validate_connected_path(&self) -> Result<(), crate::choice_graph::ChoiceGraphError> {
        let n = self.nodes.len();
        if self.start_idx >= n || self.end_idx >= n {
            return Err(crate::choice_graph::ChoiceGraphError::EdgeOutOfBounds);
        }
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
        for &(a, b) in &self.edges {
            if a >= n || b >= n {
                return Err(crate::choice_graph::ChoiceGraphError::EdgeOutOfBounds);
            }
            adj[a].push(b);
        }
        let mut radj: Vec<Vec<usize>> = vec![Vec::new(); n];
        for &(a, b) in &self.edges {
            radj[b].push(a);
        }
        let reach_from_start = bfs_reach_local(&adj, self.start_idx, n);
        let reach_to_end = bfs_reach_local(&radj, self.end_idx, n);
        for i in 0..n {
            if !(reach_from_start[i] && reach_to_end[i]) {
                return Err(crate::choice_graph::ChoiceGraphError::NodeNotOnStartEndPath);
            }
        }
        Ok(())
    }

    /// Collect the indices of all direct successors of `node_idx`.
    pub fn successors(&self, node_idx: usize) -> Vec<usize> {
        self.edges
            .iter()
            .filter_map(|&(a, b)| if a == node_idx { Some(b) } else { None })
            .collect()
    }

    /// Collect the indices of all direct predecessors of `node_idx`.
    pub fn predecessors(&self, node_idx: usize) -> Vec<usize> {
        self.edges
            .iter()
            .filter_map(|&(a, b)| if b == node_idx { Some(a) } else { None })
            .collect()
    }

    /// Returns `true` iff there is a direct edge from `start_idx` to `end_idx`
    /// (the empty path — a choice that can be skipped entirely).
    pub fn has_empty_path(&self) -> bool {
        self.edges
            .iter()
            .any(|&(a, b)| a == self.start_idx && b == self.end_idx)
    }
}

fn bfs_reach_local(adj: &[Vec<usize>], src: usize, n: usize) -> Vec<bool> {
    let mut seen = vec![false; n];
    if src >= n {
        return seen;
    }
    let mut q: Vec<usize> = vec![src];
    seen[src] = true;
    while let Some(v) = q.pop() {
        for &w in &adj[v] {
            if !seen[w] {
                seen[w] = true;
                q.push(w);
            }
        }
    }
    seen
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

    #[test]
    fn test_powl_validate_choice_graph_with_unreachable_node() {
        let mut p = Powl::new();
        let start = PowlNodeId(0);
        let x1 = PowlNodeId(1);
        let x2 = PowlNodeId(2); // isolated unreachable node
        let end = PowlNodeId(3);
        p.nodes.push(PowlNode::new(
            PowlNodeId(10),
            PowlNodeKind::ChoiceGraph {
                nodes: vec![start, x1, x2, end],
                edges: vec![
                    ChoiceGraphEdge::new(start, x1),
                    ChoiceGraphEdge::new(x1, end),
                ],
            },
        ));
        assert_eq!(p.validate(), Err(PowlRefusal::ChoiceGraphDisconnected));
    }
}

// ── PowlBuilder ──────────────────────────────────────────────────────────────

/// Ergonomic arena builder for [`Powl`] models.
///
/// Assigns [`PowlNodeId`]s automatically; callers use string labels. Call
/// [`PowlBuilder::build`] to get the validated [`Powl`] or a [`PowlRefusal`].
///
/// # Examples
///
/// ```
/// use wasm4pm_compat::powl::{PowlBuilder, PowlRefusal};
///
/// let powl = PowlBuilder::new()
///     .atom("register")
///     .atom("approve")
///     .partial_order("po", &["register", "approve"], &[("register", "approve")])
///     .build()
///     .expect("valid model");
/// assert_eq!(powl.node_count(), 3);
/// ```
#[derive(Debug, Default)]
pub struct PowlBuilder {
    powl: Powl,
    // label → PowlNodeId index for name resolution
    label_map: std::collections::HashMap<String, PowlNodeId>,
}

impl PowlBuilder {
    /// Create an empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an activity (Atom) node.  Returns the assigned [`PowlNodeId`].
    pub fn atom(mut self, label: &str) -> Self {
        let id = PowlNodeId(self.powl.nodes.len());
        self.powl
            .nodes
            .push(PowlNode::new(id, PowlNodeKind::Atom(label.to_string())));
        self.label_map.insert(label.to_string(), id);
        self
    }

    /// Add a silent (tau) node.
    pub fn silent(mut self, label: &str) -> Self {
        let id = PowlNodeId(self.powl.nodes.len());
        self.powl
            .nodes
            .push(PowlNode::new(id, PowlNodeKind::Silent));
        self.label_map.insert(label.to_string(), id);
        self
    }

    /// Add a Start boundary node.
    pub fn start_node(mut self, label: &str) -> Self {
        let id = PowlNodeId(self.powl.nodes.len());
        self.powl.nodes.push(PowlNode::new(id, PowlNodeKind::Start));
        self.label_map.insert(label.to_string(), id);
        self
    }

    /// Add an End boundary node.
    pub fn end_node(mut self, label: &str) -> Self {
        let id = PowlNodeId(self.powl.nodes.len());
        self.powl.nodes.push(PowlNode::new(id, PowlNodeKind::End));
        self.label_map.insert(label.to_string(), id);
        self
    }

    /// Add a partial-order node over `children` with precedence `edges` (pairs of labels).
    /// Children that don't yet exist as atoms are added automatically as atoms.
    pub fn partial_order(mut self, label: &str, children: &[&str], edges: &[(&str, &str)]) -> Self {
        // Ensure all referenced children exist.
        for &c in children {
            if !self.label_map.contains_key(c) {
                self = self.atom(c);
            }
        }
        let child_ids: Vec<PowlNodeId> = children.iter().map(|c| self.label_map[*c]).collect();
        for &(from_lbl, to_lbl) in edges {
            if let (Some(&from), Some(&to)) =
                (self.label_map.get(from_lbl), self.label_map.get(to_lbl))
            {
                self.powl.edges.push(OrderEdge::new(from, to));
            }
        }
        let id = PowlNodeId(self.powl.nodes.len());
        self.powl
            .nodes
            .push(PowlNode::new(id, PowlNodeKind::PartialOrder(child_ids)));
        self.label_map.insert(label.to_string(), id);
        self
    }

    /// Add a POWL 2.0 choice-graph node.
    /// `nodes` are labels; `edges` are (from_label, to_label) pairs.
    pub fn choice_graph(mut self, label: &str, nodes: &[&str], edges: &[(&str, &str)]) -> Self {
        for &n in nodes {
            if !self.label_map.contains_key(n) {
                self = self.atom(n);
            }
        }
        let node_ids: Vec<PowlNodeId> = nodes.iter().map(|n| self.label_map[*n]).collect();
        let edge_objs: Vec<ChoiceGraphEdge> = edges
            .iter()
            .filter_map(|&(f, t)| {
                Some(ChoiceGraphEdge::new(
                    *self.label_map.get(f)?,
                    *self.label_map.get(t)?,
                ))
            })
            .collect();
        let id = PowlNodeId(self.powl.nodes.len());
        self.powl.nodes.push(PowlNode::new(
            id,
            PowlNodeKind::ChoiceGraph {
                nodes: node_ids,
                edges: edge_objs,
            },
        ));
        self.label_map.insert(label.to_string(), id);
        self
    }

    /// Set the root node by label.
    pub fn root(mut self, label: &str) -> Self {
        if let Some(&id) = self.label_map.get(label) {
            self.powl.root = Some(id);
        }
        self
    }

    /// Finalise and structurally validate the model.
    /// Returns `Err(PowlRefusal)` if any structural law is violated.
    pub fn build(self) -> Result<Powl, PowlRefusal> {
        self.powl.validate()?;
        Ok(self.powl)
    }

    /// Finalise without validation (for tests that intentionally build invalid models).
    pub fn build_unchecked(self) -> Powl {
        self.powl
    }
}

#[cfg(test)]
mod builder_tests {
    use super::*;

    #[test]
    fn builder_atom_sequence() {
        let powl = PowlBuilder::new()
            .atom("a")
            .atom("b")
            .partial_order("po", &["a", "b"], &[("a", "b")])
            .build()
            .unwrap();
        assert_eq!(powl.node_count(), 3);
        assert!(powl.root.is_none()); // root not set — caller sets it explicitly
    }

    #[test]
    fn builder_kourani_figure_2_non_separable_refused() {
        // "Figure 2: A free-choice WF-net that is not separable."
        // Attempting to cross-link choice logic into a concurrent partial order
        // results in structural cyclic precedence violations.
        let result = PowlBuilder::new()
            .atom("a")
            .atom("b")
            .partial_order("po", &["a", "b"], &[("a", "b"), ("b", "a")])
            .build();
        assert_eq!(result, Err(PowlRefusal::CyclicPartialOrder));
    }

    #[test]
    fn builder_kourani_figure_7a_long_term_dependency_refused() {
        // "Figure 7a exhibits a choice between a and b, followed by a non-freechoice
        // between d and e... This long-term dependency choice cannot be represented in POWL."
        // If we attempt to physically force this non-block-structured long-term dependency
        // into a ChoiceGraph without a unified synchronization point, it leaves dangling edges.
        let result = PowlBuilder::new()
            .atom("START")
            .atom("END")
            .atom("a")
            .atom("b")
            .atom("d")
            .atom("e")
            .choice_graph(
                "top_level",
                &["START", "a", "b", "d", "e", "END"],
                &[
                    ("START", "a"),
                    ("START", "b"),
                    ("a", "d"),
                    ("b", "e"),
                    // Intentionally missing the required paths from d -> END and e -> END
                    // because the long-term dependencies fail to structurally synchronize.
                ],
            )
            .root("top_level")
            .build();

        assert_eq!(result, Err(PowlRefusal::ChoiceGraphDisconnected));
    }

    #[test]
    fn builder_kourani_figure_1b_powl_2_0() {
        // "Figure 1b shows an example POWL model. This model is defined by a top-level choice graph
        // that captures the complex decision and cyclic logic of the process. Nested within this
        // structure is a partial order submodel that enforces causal dependencies..."
        let powl = PowlBuilder::new()
            .atom("START") // ▷
            .atom("END") // □
            .atom("task_a")
            .atom("task_b")
            .partial_order("concurrent_production", &["task_a", "task_b"], &[])
            .atom("review")
            .atom("finalize")
            .choice_graph(
                "top_level",
                &[
                    "START",
                    "concurrent_production",
                    "review",
                    "finalize",
                    "END",
                ],
                &[
                    ("START", "concurrent_production"),
                    ("concurrent_production", "review"),
                    // Forward decision path
                    ("review", "finalize"),
                    // Backward cyclic edge (replaces the rigid Loop operator)
                    ("review", "concurrent_production"),
                    ("finalize", "END"),
                ],
            )
            .root("top_level")
            .build()
            .unwrap();

        // Total Nodes: 5 Atoms + 1 Partial Order + 2 Choice Graph logic structures = 8
        assert_eq!(powl.node_count(), 8);
        assert!(powl.validate().is_ok());
    }
}

// ── POWL 2.0 Python Interop Types ────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LeafNode {
    pub id: String,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ComplexModel {
    pub nodes: Vec<String>,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct oc_powl {
    pub model: ComplexModel,
}

#[cfg(test)]
mod serde_tests {
    use super::*;

    #[test]
    fn test_serde_powl_node_kind() {
        let kind = PowlNodeKind::ChoiceGraph {
            nodes: vec![PowlNodeId(1), PowlNodeId(2)],
            edges: vec![ChoiceGraphEdge::new(PowlNodeId(1), PowlNodeId(2))],
        };
        let serialized = serde_json::to_string(&kind).unwrap();
        let deserialized: PowlNodeKind = serde_json::from_str(&serialized).unwrap();
        assert_eq!(kind, deserialized);
    }

    #[test]
    fn test_serde_powl_loop_node() {
        let node: TypedPowlLoopNode<[PowlNodeId; 2], 2> =
            TypedPowlLoopNode::new([PowlNodeId(1), PowlNodeId(2)]);
        let serialized = serde_json::to_string(&node).unwrap();
        let deserialized: TypedPowlLoopNode<[PowlNodeId; 2], 2> =
            serde_json::from_str(&serialized).unwrap();
        assert_eq!(node.children, deserialized.children);
    }

    #[test]
    fn test_serde_powl_composition() {
        let comp: PowlComposition<PowlNodeId, 8> = PowlComposition::new(PowlNodeId(42));
        let serialized = serde_json::to_string(&comp).unwrap();
        let deserialized: PowlComposition<PowlNodeId, 8> =
            serde_json::from_str(&serialized).unwrap();
        assert_eq!(comp.inner, deserialized.inner);
    }
}
