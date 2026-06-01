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

use crate::law::{IsTrue, ProcessTreeOperatorKind, Require};

// ── Operator minimum-arity helper ─────────────────────────────────────────────

/// Returns the minimum child arity for a given [`ProcessTreeOperatorKind`].
///
/// This is a **compile-time-observable** constant function. It encodes the
/// structural arity law for each operator:
///
/// | Operator  | Minimum arity | Law                              |
/// |-----------|:-------------:|----------------------------------|
/// | Sequence  |       2       | ordering over one element is trivial |
/// | Xor       |       2       | choice between one is trivial    |
/// | Parallel  |       2       | concurrency of one is trivial    |
/// | Loop      |       2       | do-body + redo-branch (Leemans)  |
/// | Silent    |       0       | tau carries no children          |
/// | Or        |       2       | inclusive choice of one is trivial |
///
/// ```
/// use wasm4pm_compat::process_tree::operator_minimum_arity;
/// use wasm4pm_compat::law::ProcessTreeOperatorKind;
/// assert_eq!(operator_minimum_arity(ProcessTreeOperatorKind::Loop), 2);
/// assert_eq!(operator_minimum_arity(ProcessTreeOperatorKind::Silent), 0);
/// assert_eq!(operator_minimum_arity(ProcessTreeOperatorKind::Xor), 2);
/// ```
pub const fn operator_minimum_arity(kind: ProcessTreeOperatorKind) -> usize {
    match kind {
        ProcessTreeOperatorKind::Sequence => 2,
        ProcessTreeOperatorKind::Xor => 2,
        ProcessTreeOperatorKind::Parallel => 2,
        ProcessTreeOperatorKind::Loop => 2,
        ProcessTreeOperatorKind::Silent => 0,
        ProcessTreeOperatorKind::Or => 2,
    }
}

/// Returns the maximum child arity for a given [`ProcessTreeOperatorKind`], or
/// `usize::MAX` when the operator is unbounded.
///
/// | Operator  | Maximum arity | Law                                  |
/// |-----------|:-------------:|--------------------------------------|
/// | Sequence  |   unbounded   | arbitrarily long sequences allowed   |
/// | Xor       |   unbounded   | n-ary exclusive choice               |
/// | Parallel  |   unbounded   | n-ary parallel composition           |
/// | Loop      |       2       | exactly do-body + redo (Leemans)     |
/// | Silent    |       0       | tau has no children                  |
/// | Or        |   unbounded   | n-ary inclusive choice               |
///
/// ```
/// use wasm4pm_compat::process_tree::operator_maximum_arity;
/// use wasm4pm_compat::law::ProcessTreeOperatorKind;
/// assert_eq!(operator_maximum_arity(ProcessTreeOperatorKind::Loop), 2);
/// assert_eq!(operator_maximum_arity(ProcessTreeOperatorKind::Silent), 0);
/// assert_eq!(operator_maximum_arity(ProcessTreeOperatorKind::Sequence), usize::MAX);
/// ```
pub const fn operator_maximum_arity(kind: ProcessTreeOperatorKind) -> usize {
    match kind {
        ProcessTreeOperatorKind::Sequence => usize::MAX,
        ProcessTreeOperatorKind::Xor => usize::MAX,
        ProcessTreeOperatorKind::Parallel => usize::MAX,
        ProcessTreeOperatorKind::Loop => 2,
        ProcessTreeOperatorKind::Silent => 0,
        ProcessTreeOperatorKind::Or => usize::MAX,
    }
}

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

// ── OR operator node (type-law surface) ──────────────────────────────────────

/// An inclusive-OR operator node with arity encoded as a const generic
/// parameter.
///
/// OR requires **at least 2** children — an inclusive choice between one
/// thing is trivially degenerate. `TypedOrNode<_, 1>` does **not compile**.
///
/// ## What this is NOT
///
/// Structure only. Does not execute, replay, or discover. Graduate to `wasm4pm`.
///
/// ```
/// # #![feature(generic_const_exprs)]
/// # #![allow(incomplete_features)]
/// use wasm4pm_compat::process_tree::TypedOrNode;
/// let _: TypedOrNode<[&str; 2], 2> = TypedOrNode::new(["some", "other"]);
/// ```
///
/// ```compile_fail
/// # #![feature(generic_const_exprs)]
/// # #![allow(incomplete_features)]
/// use wasm4pm_compat::process_tree::TypedOrNode;
/// // OR with arity 1 is degenerate — compile error.
/// let _: TypedOrNode<[&str; 1], 1> = TypedOrNode::new(["only"]);
/// ```
pub struct TypedOrNode<Children, const ARITY: usize>
where
    Require<{ ARITY >= 2 }>: IsTrue,
{
    /// The inclusive-OR branches.
    pub children: Children,
}

impl<Children, const ARITY: usize> TypedOrNode<Children, ARITY>
where
    Require<{ ARITY >= 2 }>: IsTrue,
{
    /// Constructs a `TypedOrNode` — only possible when `ARITY >= 2`.
    ///
    /// ```
    /// # #![feature(generic_const_exprs)]
    /// # #![allow(incomplete_features)]
    /// use wasm4pm_compat::process_tree::TypedOrNode;
    /// let node: TypedOrNode<[&str; 2], 2> = TypedOrNode::new(["branch_a", "branch_b"]);
    /// assert_eq!(node.children[0], "branch_a");
    /// ```
    pub fn new(children: Children) -> Self {
        TypedOrNode { children }
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

    /// Perform a **structural shape admission check** on this process tree.
    ///
    /// Returns `Ok(())` when the shape is well-formed, or `Err(ProcessTreeRefusal)`
    /// naming the specific structural law that was violated. This is not a full
    /// semantic verification — it is a first-pass structural gate.
    ///
    /// Checked laws:
    /// - [`ProcessTreeRefusal::MissingRoot`] — nodes present but no root declared
    /// - [`ProcessTreeRefusal::DanglingNodeReference`] — a child id is out of bounds
    /// - [`ProcessTreeRefusal::TauLeafWithChildren`] — Silent node has children
    /// - [`ProcessTreeRefusal::BelowMinimumArity`] — Sequence/Xor/Parallel/Or with < 2 children
    /// - [`ProcessTreeRefusal::InvalidArity`] — Loop with ≠ 2 children
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::process_tree::{
    ///     ProcessTree, ProcessTreeNode, ProcessTreeNodeId, ProcessTreeOperator,
    ///     ProcessTreeRefusal,
    /// };
    ///
    /// // Well-formed Sequence(a, b) passes.
    /// let mut t = ProcessTree::new();
    /// t.nodes.push(ProcessTreeNode::Activity("a".into()));
    /// t.nodes.push(ProcessTreeNode::Activity("b".into()));
    /// t.nodes.push(ProcessTreeNode::Operator {
    ///     operator: ProcessTreeOperator::Sequence,
    ///     children: vec![ProcessTreeNodeId(0), ProcessTreeNodeId(1)],
    /// });
    /// t.root = Some(ProcessTreeNodeId(2));
    /// assert_eq!(t.admit_shape(), Ok(()));
    ///
    /// // Non-empty tree with no root is refused.
    /// let mut t2 = ProcessTree::new();
    /// t2.nodes.push(ProcessTreeNode::Activity("a".into()));
    /// assert_eq!(t2.admit_shape(), Err(ProcessTreeRefusal::MissingRoot));
    /// ```
    #[must_use = "check the shape-check result"]
    pub fn admit_shape(&self) -> Result<(), ProcessTreeRefusal> {
        // A non-empty tree must declare a root.
        if !self.nodes.is_empty() && self.root.is_none() {
            return Err(ProcessTreeRefusal::MissingRoot);
        }

        for node in &self.nodes {
            match node {
                ProcessTreeNode::Activity(_) => {}
                ProcessTreeNode::Operator { operator, children } => {
                    // Check all child ids are in-bounds.
                    for child_id in children {
                        if child_id.0 >= self.nodes.len() {
                            return Err(ProcessTreeRefusal::DanglingNodeReference);
                        }
                    }
                    match operator {
                        ProcessTreeOperator::Silent => {
                            if !children.is_empty() {
                                return Err(ProcessTreeRefusal::TauLeafWithChildren);
                            }
                        }
                        ProcessTreeOperator::Loop => {
                            if children.len() != 2 {
                                return Err(ProcessTreeRefusal::InvalidArity);
                            }
                        }
                        ProcessTreeOperator::Sequence
                        | ProcessTreeOperator::Xor
                        | ProcessTreeOperator::Parallel
                        | ProcessTreeOperator::Or => {
                            if children.len() < 2 {
                                return Err(ProcessTreeRefusal::BelowMinimumArity);
                            }
                        }
                    }
                }
            }
        }
        Ok(())
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
