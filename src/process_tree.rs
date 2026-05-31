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
}

impl core::fmt::Display for ProcessTreeRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let law = match self {
            ProcessTreeRefusal::InvalidArity => "InvalidArity",
            ProcessTreeRefusal::InvalidLoop => "InvalidLoop",
            ProcessTreeRefusal::UnsupportedProjection => "UnsupportedProjection",
            ProcessTreeRefusal::LanguageMismatch => "LanguageMismatch",
        };
        write!(f, "process tree refused: {law}")
    }
}
