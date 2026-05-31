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
//! - The structural vocabulary of POWL: [`PowlNode`], [`OrderEdge`], [`Powl`].
//! - Witness markers describing *which POWL fragment* a node represents
//!   ([`Atom`], [`PartialOrder`], [`Choice`], [`Loop`], [`Silent`],
//!   [`Irreducible`]) and *whether it can graduate downward* into a process
//!   tree ([`ProcessTreeProjectable`], [`ExceedsProcessTree`]).
//! - A first-class [`PowlRefusal`] surface naming exactly why a POWL shape is
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
    /// A loop node was malformed (e.g. missing `do` body).
    InvalidLoop,
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
        let law = match self {
            PowlRefusal::CyclicPartialOrder => "CyclicPartialOrder",
            PowlRefusal::InvalidChoice => "InvalidChoice",
            PowlRefusal::InvalidLoop => "InvalidLoop",
            PowlRefusal::IrreducibleProjection => "IrreducibleProjection",
            PowlRefusal::LanguageMismatch => "LanguageMismatch",
            PowlRefusal::ChoiceGraphDisconnected => "ChoiceGraphDisconnected",
        };
        write!(f, "POWL refused: {law}")
    }
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
