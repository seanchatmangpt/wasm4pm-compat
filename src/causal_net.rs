//! Causal net structural shapes — Weijters & Ribeiro (2011) Heuristics Miner output.
//!
//! A *causal net* (C-net) is a graph model produced by the Heuristics Miner
//! algorithm (Weijters & Ribeiro, 2011). Unlike a Petri net, arcs in a C-net
//! carry *dependency measures* (floating-point scores in [0, 1]) that reflect
//! the observed causal strength between activities in an event log.
//!
//! Each task in a C-net is associated with a set of *input bindings* and a set
//! of *output bindings* — structured conjunctions and disjunctions of
//! predecessor/successor tasks. A binding records *which combination* of
//! incoming (or outgoing) arcs activates (or is produced by) the task.
//!
//! ## What this module IS
//!
//! - The **shape** of a causal net: nodes, arcs, dependency scores, and bindings.
//! - Structure-only: no mining, no score computation, no replay. Heuristics Miner
//!   execution graduates to `wasm4pm`.
//!
//! ## What this module is **NOT**
//!
//! - Not a miner. `CausalNet` is the *output shape* of Heuristics Miner, not the
//!   miner itself.
//! - Not an executable graph. `DependencyMeasure` is a score annotation; no arc
//!   fires, and no binding is evaluated here.
//!
//! ## Graduation to `wasm4pm`
//!
//! Dependency measure computation (the ≥2, ≥3, and long-distance dependency
//! heuristics), binding-set construction, replay, and conformance checking all
//! graduate to `wasm4pm`.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt;
use std::marker::PhantomData;

use crate::law::{Between01, IsTrue, Require};

// ── DependencyMeasure ─────────────────────────────────────────────────────────

/// An arc dependency measure: the causal strength between two activities,
/// represented as a rational fraction in `[0, 1]` at the type level.
///
/// Under nightly features `generic_const_exprs` and `adt_const_params`,
/// this type guarantees at compile-time that the measure is in the range `[0, 1]`.
/// `DependencyMeasure<2, 1>` does not compile.
///
/// ## Paper
///
/// Weijters & Ribeiro (2011) — Section 2 (dependency measure formulae).
///
/// ```
/// use wasm4pm_compat::causal_net::DependencyMeasure;
/// let dm: DependencyMeasure<4, 5> = DependencyMeasure::new();
/// assert_eq!(dm.num(), 4);
/// assert_eq!(dm.den(), 5);
/// assert_eq!(dm.as_f64(), 0.8);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DependencyMeasure<const NUM: u64, const DEN: u64>
where
    Require<{ DEN > 0 }>: IsTrue,
    Require<{ NUM <= DEN }>: IsTrue,
{
    _measure: Between01<NUM, DEN>,
}

impl<const NUM: u64, const DEN: u64> DependencyMeasure<NUM, DEN>
where
    Require<{ DEN > 0 }>: IsTrue,
    Require<{ NUM <= DEN }>: IsTrue,
{
    /// Construct a new `DependencyMeasure`.
    pub const fn new() -> Self {
        DependencyMeasure {
            _measure: Between01::new(),
        }
    }

    /// Accessor for the numerator.
    pub const fn num(&self) -> u64 {
        NUM
    }

    /// Accessor for the denominator.
    pub const fn den(&self) -> u64 {
        DEN
    }

    /// Convert the static fraction into a floating point score.
    pub fn as_f64(&self) -> f64 {
        NUM as f64 / DEN as f64
    }
}

impl<const NUM: u64, const DEN: u64> Default for DependencyMeasure<NUM, DEN>
where
    Require<{ DEN > 0 }>: IsTrue,
    Require<{ NUM <= DEN }>: IsTrue,
{
    fn default() -> Self {
        Self::new()
    }
}

// ── Bindings (Compile-Time / Static) ─────────────────────────────────────────

/// An input binding: represents a predecessor task or group of predecessor tasks
/// `A` that must execute before the target task `B`.
///
/// In a C-net, bindings represent logical combinations (AND/XOR).
/// `A` can be a single task type, or a tuple of task types representing a conjunction (AND-join).
/// Multiple `InputBinding`s for the same target represent disjunctions (XOR-joins).
///
/// ## Paper
///
/// Weijters & Ribeiro (2011) — binding obligations in the FHM C-net definition.
///
/// ```
/// use wasm4pm_compat::causal_net::InputBinding;
/// struct TaskA; struct TaskB;
/// let b = InputBinding::new(TaskA, TaskB);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InputBinding<A, B> {
    pub source: A,
    pub target: B,
}

impl<A, B> InputBinding<A, B> {
    /// Construct a new static `InputBinding`.
    pub const fn new(source: A, target: B) -> Self {
        InputBinding { source, target }
    }
}

/// An output binding: represents a successor task or group of successor tasks
/// `B` that is activated after the source task `A` fires.
///
/// Symmetric to [`InputBinding`] in direction.
///
/// ## Paper
///
/// Weijters & Ribeiro (2011) — output binding obligations in the FHM C-net.
///
/// ```
/// use wasm4pm_compat::causal_net::OutputBinding;
/// struct TaskA; struct TaskB;
/// let b = OutputBinding::new(TaskA, TaskB);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OutputBinding<A, B> {
    pub source: A,
    pub target: B,
}

impl<A, B> OutputBinding<A, B> {
    /// Construct a new static `OutputBinding`.
    pub const fn new(source: A, target: B) -> Self {
        OutputBinding { source, target }
    }
}

// ── CausalNetConst (Compile-Time ZST) ────────────────────────────────────────

/// A compile-time, zero-cost Causal Net shape.
///
/// All nodes, arcs, and bindings are encoded in the type parameters.
/// Useful for compile-time validation and verification of process properties.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CausalNetConst<Nodes, Arcs, Inputs, Outputs> {
    _nodes: PhantomData<Nodes>,
    _arcs: PhantomData<Arcs>,
    _inputs: PhantomData<Inputs>,
    _outputs: PhantomData<Outputs>,
}

impl<Nodes, Arcs, Inputs, Outputs> CausalNetConst<Nodes, Arcs, Inputs, Outputs> {
    /// Construct a new `CausalNetConst`.
    pub const fn new() -> Self {
        CausalNetConst {
            _nodes: PhantomData,
            _arcs: PhantomData,
            _inputs: PhantomData,
            _outputs: PhantomData,
        }
    }
}

impl<Nodes, Arcs, Inputs, Outputs> Default for CausalNetConst<Nodes, Arcs, Inputs, Outputs> {
    fn default() -> Self {
        Self::new()
    }
}

// ── CausalNet (Dynamic / Runtime) ────────────────────────────────────────────

/// The causal net graph shape: a set of tasks connected by causal arcs.
///
/// A `CausalNet` is the top-level container produced by Heuristics Miner.
/// It names the graph structure (nodes and arcs with dependency measures)
/// without executing it.
///
/// Structure-only: no mining, no replay, no conformance. Graduate to `wasm4pm`.
///
/// ## Paper
///
/// Weijters, A.J.M.M. & Ribeiro, J.T.S. (2011) — *Flexible Heuristics Miner
/// (FHM)*. IEEE Symposium on Computational Intelligence and Data Mining (CIDM).
///
/// ```
/// use wasm4pm_compat::causal_net::CausalNet;
/// let _: CausalNet;
/// ```
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CausalNet {
    /// The activity tasks that form the nodes of the causal net.
    pub nodes: Vec<String>,
    /// The starting activity task.
    pub initial_node: Option<String>,
    /// The final activity task.
    pub final_node: Option<String>,
    /// Dependency measures between tasks: (source_task, target_task, score).
    pub dependency_measures: Vec<(String, String, f64)>,
    /// The input binding obligations for each task.
    pub inputs: Vec<CausalBinding>,
    /// The output binding obligations for each task.
    pub outputs: Vec<CausalBinding>,
    /// Short loops of length 1 (L1L): (activity, score).
    pub loops_len1: Vec<(String, f64)>,
    /// Short loops of length 2 (L2L): (source, target, score).
    pub loops_len2: Vec<(String, String, f64)>,
}

/// A causal binding: an input/output set of tasks that form a binding obligation.
///
/// In a C-net, each task has a set of *input bindings* (which predecessors must
/// have fired to activate this task) and *output bindings* (which successors this
/// task activates).
///
/// Structure-only: a binding shape. Graduate to `wasm4pm` for binding evaluation.
///
/// ```
/// use wasm4pm_compat::causal_net::CausalBinding;
/// let _: CausalBinding;
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CausalBinding {
    /// The source tasks in the binding.
    pub source_tasks: Vec<String>,
    /// The target tasks in the binding.
    pub target_tasks: Vec<String>,
}

impl CausalBinding {
    /// Construct a new `CausalBinding`.
    pub fn new(
        sources: impl IntoIterator<Item = String>,
        targets: impl IntoIterator<Item = String>,
    ) -> Self {
        CausalBinding {
            source_tasks: sources.into_iter().collect(),
            target_tasks: targets.into_iter().collect(),
        }
    }
}

// ── CausalNetRefusal ──────────────────────────────────────────────────────────

/// First-class refusal law for Causal Net shapes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum CausalNetRefusal {
    /// A task has an empty/missing activity label.
    MissingActivity,
    /// An arc or loop has a dependency measure score outside the valid range [0, 1].
    InvalidDependencyScore,
    /// The graph is structurally disconnected (isolated tasks that are not
    /// referenced in any dependency arcs or short loops).
    DisconnectedGraph,
}

impl CausalNet {
    /// Validate the causal net's structural law, returning the first violated law.
    ///
    /// Checks (in order): `MissingActivity`, `InvalidDependencyScore`,
    /// `DisconnectedGraph` (isolated-node variant — a node present in `nodes`
    /// but absent from all dependency-measure arcs and short loops).
    ///
    /// ```
    /// use wasm4pm_compat::causal_net::{CausalNet, CausalNetRefusal};
    /// let mut net = CausalNet::default();
    /// net.nodes.push("".into());
    /// assert_eq!(net.validate(), Err(CausalNetRefusal::MissingActivity));
    /// ```
    pub fn validate(&self) -> Result<(), CausalNetRefusal> {
        // 1. Missing Activity check
        for node in &self.nodes {
            if node.trim().is_empty() {
                return Err(CausalNetRefusal::MissingActivity);
            }
        }

        // 2. Invalid Dependency Score check
        for (_, _, score) in &self.dependency_measures {
            if score.is_nan() || *score < 0.0 || *score > 1.0 {
                return Err(CausalNetRefusal::InvalidDependencyScore);
            }
        }
        for (_, score) in &self.loops_len1 {
            if score.is_nan() || *score < 0.0 || *score > 1.0 {
                return Err(CausalNetRefusal::InvalidDependencyScore);
            }
        }
        for (_, _, score) in &self.loops_len2 {
            if score.is_nan() || *score < 0.0 || *score > 1.0 {
                return Err(CausalNetRefusal::InvalidDependencyScore);
            }
        }

        // 3. Isolated-node check (aware of both dependency arcs and short loops)
        if self.nodes.len() > 1 {
            let mut arc_nodes: HashSet<&str> = self
                .dependency_measures
                .iter()
                .flat_map(|(src, tgt, _)| [src.as_str(), tgt.as_str()])
                .collect();

            // Add short-loop nodes
            for (node, _) in &self.loops_len1 {
                arc_nodes.insert(node.as_str());
            }
            for (src, tgt, _) in &self.loops_len2 {
                arc_nodes.insert(src.as_str());
                arc_nodes.insert(tgt.as_str());
            }

            for node in &self.nodes {
                if !arc_nodes.contains(node.as_str()) {
                    return Err(CausalNetRefusal::DisconnectedGraph);
                }
            }
        }
        Ok(())
    }
}

impl fmt::Display for CausalNetRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CausalNetRefusal::MissingActivity => write!(f, "Causal net refused: MissingActivity"),
            CausalNetRefusal::InvalidDependencyScore => {
                write!(f, "Causal net refused: InvalidDependencyScore")
            }
            CausalNetRefusal::DisconnectedGraph => {
                write!(f, "Causal net refused: DisconnectedGraph")
            }
        }
    }
}

impl std::error::Error for CausalNetRefusal {}
