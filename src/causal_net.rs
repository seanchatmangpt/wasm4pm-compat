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
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CausalNet {
    /// The activity tasks that form the nodes of the causal net.
    pub nodes: Vec<String>,
    /// Dependency measures between tasks: (source_task, target_task, score).
    pub dependency_measures: Vec<(String, String, f64)>,
    /// The input binding obligations for each task.
    pub inputs: Vec<CausalBinding>,
    /// The output binding obligations for each task.
    pub outputs: Vec<CausalBinding>,
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
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CausalBinding {
    /// The source tasks in the binding.
    pub source_tasks: Vec<String>,
    /// The target tasks in the binding.
    pub target_tasks: Vec<String>,
}


/// An input binding: a conjunction of predecessor tasks that must have fired to
/// activate the task that owns this binding.
///
/// `A` is the *source* task marker type; `B` is the *target* (activated) task
/// marker type. Both are phantom structural tags — `InputBinding<A, B>` is a
/// named edge shape, not an executable activation.
///
/// Structure-only: directed edge shape. Graduate to `wasm4pm` for binding
/// evaluation and reachability.
///
/// ## Paper
///
/// Weijters & Ribeiro (2011) — binding obligations in the FHM C-net definition.
///
/// ```
/// use wasm4pm_compat::causal_net::InputBinding;
/// struct SourceTask; struct TargetTask;
/// let b = InputBinding(SourceTask, TargetTask);
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InputBinding<A, B>(pub A, pub B);

/// An output binding: a conjunction of successor tasks that this task activates
/// when it fires.
///
/// `A` is the *source* (firing) task marker type; `B` is the *target* (activated)
/// task marker type. Symmetric to [`InputBinding`] in direction.
///
/// Structure-only: directed edge shape. Graduate to `wasm4pm` for binding
/// evaluation.
///
/// ## Paper
///
/// Weijters & Ribeiro (2011) — output binding obligations in the FHM C-net.
///
/// ```
/// use wasm4pm_compat::causal_net::OutputBinding;
/// struct SourceTask; struct TargetTask;
/// let b = OutputBinding(SourceTask, TargetTask);
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutputBinding<A, B>(pub A, pub B);

/// An arc dependency measure: the causal strength between two activities.
///
/// In Heuristics Miner, the *dependency measure* between activity `a` and
/// activity `b` is a real value in `[0, 1]` computed from the event log —
/// higher values indicate stronger evidence of a causal relationship.
///
/// `DependencyMeasure` wraps the `f64` score as a named structural annotation.
/// It is not computed here: the computation (the ≥2, ≥3, and long-distance
/// heuristics) graduates to `wasm4pm`.
///
/// ## Paper
///
/// Weijters & Ribeiro (2011) — Section 2 (dependency measure formulae).
///
/// ```
/// use wasm4pm_compat::causal_net::DependencyMeasure;
/// let dm = DependencyMeasure(0.85);
/// assert_eq!(dm.0, 0.85);
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DependencyMeasure(pub f64);

/// First-class refusal law for Causal Net shapes.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CausalNetRefusal {
    /// A task has an empty/missing activity label.
    MissingActivity,
    /// An arc has a dependency measure score outside the valid range [0, 1].
    InvalidDependencyScore,
    /// The graph is structurally disconnected.
    DisconnectedGraph,
}

impl core::fmt::Display for CausalNetRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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

