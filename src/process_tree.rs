//! Process tree shape — **structure only**.
//!
//! This module represents the *shape* of a block-structured process tree: a
//! recursively nested model built from sequence, exclusive choice, parallel,
//! loop, and silent operators over activity leaves.
//!
//! ## What this module **IS**
//!
//! - The structural vocabulary of process trees: [`ProcessTree`],
//!   [`ProcessTreeNode`], and the closed [`ProcessTreeOperator`] enumeration.
//! - A first-class [`ProcessTreeRefusal`] surface naming exactly why a tree
//!   shape is inadmissible.
//!
//! ## What this module is **NOT**
//!
//! - **Not** an inductive miner, a tree player, a simplifier, or a conformance
//!   checker. It builds and refuses *shapes*; it never *executes* them.
//! - **Not** a substitute for [`crate::powl`]. A process tree is strictly
//!   block-structured; POWL partial orders that exceed block structure cannot be
//!   represented here, and projection POWL → process tree is a *named, refusable*
//!   operation.
//!
//! ## Graduation
//!
//! When you need to **discover, replay, simplify, or measure** a process tree,
//! graduate this shape to the `wasm4pm` engine (via the `wasm4pm` feature). This
//! module only certifies that the *structure* is well-formed.

use crate::law::{IsTrue, Require};

// ── Arity-typed loop node (type-law surface) ─────────────────────────────────

/// A loop node with its arity encoded as a const generic parameter.
///
/// Paper: Leemans (2013) inductive miner — a loop operator has exactly 2
/// children: the `do` body and the `redo` branch.
/// `TypedLoopNode<_, 3>` does **not compile**: `ARITY == 2` is violated.
///
/// ```
/// # #![feature(generic_const_exprs)]
/// # #![allow(incomplete_features)]
/// use wasm4pm_compat::process_tree::TypedLoopNode;
/// let _: TypedLoopNode<(), 2> = TypedLoopNode::new(());  // arity 2: lawful
/// ```
///
/// ```compile_fail
/// use wasm4pm_compat::process_tree::TypedLoopNode;
/// let _: TypedLoopNode<(), 3> = TypedLoopNode::new(());  // arity 3: compile error
/// ```
pub struct TypedLoopNode<Children, const ARITY: usize>
where
    Require<{ ARITY == 2 }>: IsTrue,
{
    /// The loop children (do body + redo branch), provided by the caller.
    pub children: Children,
}

impl<Children, const ARITY: usize> TypedLoopNode<Children, ARITY>
where
    Require<{ ARITY == 2 }>: IsTrue,
{
    /// Constructs a `TypedLoopNode` — only possible when `ARITY == 2`.
    ///
    /// ```
    /// # #![feature(generic_const_exprs)]
    /// # #![allow(incomplete_features)]
    /// use wasm4pm_compat::process_tree::TypedLoopNode;
    /// let node: TypedLoopNode<[&str; 2], 2> = TypedLoopNode::new(["do", "redo"]);
    /// assert_eq!(node.children, ["do", "redo"]);
    /// ```
    pub fn new(children: Children) -> Self {
        TypedLoopNode { children }
    }
}

// ── XOR operator node (type-law surface) ─────────────────────────────────────

/// An exclusive-choice (XOR) operator node with arity encoded as a const
/// generic parameter.
///
/// XOR requires **at least 2** children (an exclusive choice between one
/// thing is trivially degenerate). `TypedXorNode<_, 1>` does **not compile**.
///
/// ## Paper
///
/// Leemans (2013) inductive miner — the `×` (exclusive-choice) operator.
///
/// ## What this is NOT
///
/// Structure only. Does not execute, replay, or discover. Graduate to `wasm4pm`.
///
/// ```
/// # #![feature(generic_const_exprs)]
/// # #![allow(incomplete_features)]
/// use wasm4pm_compat::process_tree::TypedXorNode;
/// let _: TypedXorNode<[&str; 2], 2> = TypedXorNode::new(["branch_a", "branch_b"]);
/// ```
///
/// ```compile_fail
/// # #![feature(generic_const_exprs)]
/// # #![allow(incomplete_features)]
/// use wasm4pm_compat::process_tree::TypedXorNode;
/// // XOR with arity 1 is degenerate — compile error.
/// let _: TypedXorNode<[&str; 1], 1> = TypedXorNode::new(["only"]);
/// ```
pub struct TypedXorNode<Children, const ARITY: usize>
where
    Require<{ ARITY >= 2 }>: IsTrue,
{
    /// The exclusive-choice branches.
    pub children: Children,
}

impl<Children, const ARITY: usize> TypedXorNode<Children, ARITY>
where
    Require<{ ARITY >= 2 }>: IsTrue,
{
    /// Constructs a `TypedXorNode` — only possible when `ARITY >= 2`.
    ///
    /// ```
    /// # #![feature(generic_const_exprs)]
    /// # #![allow(incomplete_features)]
    /// use wasm4pm_compat::process_tree::TypedXorNode;
    /// let node: TypedXorNode<[&str; 3], 3> = TypedXorNode::new(["a", "b", "c"]);
    /// assert_eq!(node.children.len(), 3);
    /// ```
    pub fn new(children: Children) -> Self {
        TypedXorNode { children }
    }
}

// ── AND (Parallel) operator node (type-law surface) ──────────────────────────

/// A parallel (AND) operator node with arity encoded as a const generic
/// parameter.
///
/// AND requires **at least 2** children — a parallel composition of one thing
/// is trivially degenerate. `TypedAndNode<_, 1>` does **not compile**.
///
/// ## Paper
///
/// Leemans (2013) inductive miner — the `∧` (parallel / and) operator.
///
/// ## What this is NOT
///
/// Structure only. Does not execute, replay, or discover. Graduate to `wasm4pm`.
///
/// ```
/// # #![feature(generic_const_exprs)]
/// # #![allow(incomplete_features)]
/// use wasm4pm_compat::process_tree::TypedAndNode;
/// let _: TypedAndNode<[&str; 2], 2> = TypedAndNode::new(["left", "right"]);
/// ```
///
/// ```compile_fail
/// # #![feature(generic_const_exprs)]
/// # #![allow(incomplete_features)]
/// use wasm4pm_compat::process_tree::TypedAndNode;
/// // AND with arity 1 is degenerate — compile error.
/// let _: TypedAndNode<[&str; 1], 1> = TypedAndNode::new(["only"]);
/// ```
pub struct TypedAndNode<Children, const ARITY: usize>
where
    Require<{ ARITY >= 2 }>: IsTrue,
{
    /// The concurrent branches.
    pub children: Children,
}

impl<Children, const ARITY: usize> TypedAndNode<Children, ARITY>
where
    Require<{ ARITY >= 2 }>: IsTrue,
{
    /// Constructs a `TypedAndNode` — only possible when `ARITY >= 2`.
    ///
    /// ```
    /// # #![feature(generic_const_exprs)]
    /// # #![allow(incomplete_features)]
    /// use wasm4pm_compat::process_tree::TypedAndNode;
    /// let node: TypedAndNode<[&str; 2], 2> = TypedAndNode::new(["step_a", "step_b"]);
    /// assert_eq!(node.children[0], "step_a");
    /// ```
    pub fn new(children: Children) -> Self {
        TypedAndNode { children }
    }
}

// ── SEQ (Sequence) operator node (type-law surface) ──────────────────────────

/// A sequence (SEQ) operator node with arity encoded as a const generic
/// parameter.
///
/// SEQ requires **at least 2** children — a sequence of one element has no
/// ordering content. `TypedSeqNode<_, 1>` does **not compile**.
///
/// ## Paper
///
/// Leemans (2013) inductive miner — the `→` (sequence) operator.
///
/// ## What this is NOT
///
/// Structure only. Does not execute, replay, or discover. Graduate to `wasm4pm`.
///
/// ```
/// # #![feature(generic_const_exprs)]
/// # #![allow(incomplete_features)]
/// use wasm4pm_compat::process_tree::TypedSeqNode;
/// let _: TypedSeqNode<[&str; 2], 2> = TypedSeqNode::new(["first", "second"]);
/// ```
///
/// ```compile_fail
/// # #![feature(generic_const_exprs)]
/// # #![allow(incomplete_features)]
/// use wasm4pm_compat::process_tree::TypedSeqNode;
/// // SEQ with arity 1 is degenerate — compile error.
/// let _: TypedSeqNode<[&str; 1], 1> = TypedSeqNode::new(["only"]);
/// ```
pub struct TypedSeqNode<Children, const ARITY: usize>
where
    Require<{ ARITY >= 2 }>: IsTrue,
{
    /// The ordered children in declared execution order.
    pub children: Children,
}

impl<Children, const ARITY: usize> TypedSeqNode<Children, ARITY>
where
    Require<{ ARITY >= 2 }>: IsTrue,
{
    /// Constructs a `TypedSeqNode` — only possible when `ARITY >= 2`.
    ///
    /// ```
    /// # #![feature(generic_const_exprs)]
    /// # #![allow(incomplete_features)]
    /// use wasm4pm_compat::process_tree::TypedSeqNode;
    /// let node: TypedSeqNode<[&str; 3], 3> = TypedSeqNode::new(["a", "b", "c"]);
    /// assert_eq!(node.children[2], "c");
    /// ```
    pub fn new(children: Children) -> Self {
        TypedSeqNode { children }
    }
}

// ── Identifier and operator types ────────────────────────────────────────────

/// Zero-cost identifier for a [`ProcessTreeNode`].
///
/// `#[repr(transparent)]` over `usize`: structural and free.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProcessTreeNodeId(pub usize);

/// The closed set of block-structured process-tree operators.
///
/// This is **structure only**: it records *what kind of block* a node is, never
/// *how it runs*. It does NOT unfold, replay, or play out the operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProcessTreeOperator {
    /// Strict total order of children (`->`).
    Sequence,
    /// Exclusive choice among children (`x`).
    Xor,
    /// Concurrent / interleaved children (`+`).
    Parallel,
    /// Loop: first child is the `do` body, second the `redo` body (`*`).
    Loop,
    /// Silent leaf (tau) — observable-activity-free step.
    Silent,
    /// Inclusive OR: one or more branches chosen non-deterministically (`o`).
    ///
    /// Not part of the original Leemans (2013) inductive miner base set, but
    /// present in extended process-tree formalisms. Structure-only label; the
    /// semantics are interpreted only by the `wasm4pm` engine on graduation.
    Or,
}

/// A single node of a process tree: either an operator with children, or an
/// activity leaf.
///
/// This represents the node's *shape*; it does **not** execute the operator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcessTreeNode {
    /// An activity leaf carrying its label.
    Activity(String),
    /// An operator node carrying its operator kind and child ids.
    Operator {
        /// The operator kind.
        operator: ProcessTreeOperator,
        /// The child node ids, in declared order.
        children: Vec<ProcessTreeNodeId>,
    },
}

/// A complete process tree: a node arena plus the designated root.
///
/// The top-level **shape** of a block-structured process model. It does **NOT**
/// discover, replay, simplify, or measure conformance. Graduate to `wasm4pm`
/// for any of that.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ProcessTree {
    /// All nodes, in id order.
    pub nodes: Vec<ProcessTreeNode>,
    /// The root node id, if the tree is non-empty.
    pub root: Option<ProcessTreeNodeId>,
}

impl ProcessTree {
    /// Construct an empty process tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::process_tree::ProcessTree;
    /// let t = ProcessTree::new();
    /// assert!(t.root.is_none());
    /// assert_eq!(t.node_count(), 0);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Number of nodes in the tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::process_tree::{ProcessTree, ProcessTreeNode};
    /// let mut t = ProcessTree::new();
    /// t.nodes.push(ProcessTreeNode::Activity("a".into()));
    /// assert_eq!(t.node_count(), 1);
    /// ```
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
}

/// First-class refusal law for process-tree shapes.
///
/// Every variant names a **specific** structural law — never a bare
/// "InvalidInput".
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ProcessTreeRefusal {
    /// An operator received the wrong number of children (e.g. a [`Loop`] with
    /// other than two children).
    ///
    /// [`Loop`]: ProcessTreeOperator::Loop
    InvalidArity,
    /// A loop node was malformed beyond arity (e.g. missing `do` body).
    InvalidLoop,
    /// Projection from another shape (e.g. POWL) into this tree was requested
    /// but is unsupported because it would lose language.
    UnsupportedProjection,
    /// The claimed language of the tree does not match the admitted reference.
    LanguageMismatch,
    /// A tau (silent) leaf was given children — tau carries no children.
    ///
    /// A `Silent` operator with a non-empty child list is structurally invalid.
    TauLeafWithChildren,
    /// A root node is missing from a non-empty tree.
    ///
    /// The tree has nodes but no declared root, making the shape inadmissible.
    MissingRoot,
    /// A node referenced by its id does not exist in the arena.
    ///
    /// A child `ProcessTreeNodeId` refers to an index that is out of bounds.
    DanglingNodeReference,
    /// An operator node received fewer children than its minimum arity.
    ///
    /// XOR, AND, and SEQ all require at least 2 children; Loop requires exactly 2.
    BelowMinimumArity,
    /// Cycles were detected in the child-id graph — process trees are acyclic.
    CycleDetected,
}

impl core::fmt::Display for ProcessTreeRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let law = match self {
            ProcessTreeRefusal::InvalidArity => "InvalidArity",
            ProcessTreeRefusal::InvalidLoop => "InvalidLoop",
            ProcessTreeRefusal::UnsupportedProjection => "UnsupportedProjection",
            ProcessTreeRefusal::LanguageMismatch => "LanguageMismatch",
            ProcessTreeRefusal::TauLeafWithChildren => "TauLeafWithChildren",
            ProcessTreeRefusal::MissingRoot => "MissingRoot",
            ProcessTreeRefusal::DanglingNodeReference => "DanglingNodeReference",
            ProcessTreeRefusal::BelowMinimumArity => "BelowMinimumArity",
            ProcessTreeRefusal::CycleDetected => "CycleDetected",
        };
        write!(f, "process tree refused: {law}")
    }
}
