//! PDDL8 canonical types for wasm4pm-compat.
//!
//! PDDL8 is the bounded subset of PDDL 3.1 that maps directly onto the
//! Prolog8 + POWL runtime:
//!
//! - Arity ≤ 8, body/precondition conjuncts ≤ 8, parameters ≤ 8
//! - Only STRIPS subset: conjunctive positive preconditions, add/delete effects
//! - Delete effects become epoch seals (immutable history), not silent erasure
//! - Plan depth ≤ 64
//!
//! These types are the canonical cross-crate representation. `bcinr-pddl`
//! parses PDDL 3.1 text (via the `pddl` crate) and lowers it into these
//! types. `wasm4pm-cognition` breeds (STRIPS, POP, HTN) map onto these via
//! `BreedInput` → `Pddl8Problem` conversion.
//!
//! # BRCE position
//! ```text
//! PDDL8Domain + Pddl8Problem  = G_F^B constructor (candidate-future grammar)
//! GroundAction.preconditions   = Prolog8 R ⊢ A query (may_fire)
//! GroundAction.add_effects     = facts loaded after firing
//! GroundAction.del_effects     = epoch advance (sealed, not erased)
//! ExecutionTrace               = OCEL 2.0 (wasm4pm-compat::ocel)
//! ```

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Structural bounds
// ---------------------------------------------------------------------------

/// Maximum predicate arity in PDDL8.
pub const PDDL8_MAX_ARITY: usize = 8;
/// Maximum conjuncts in a precondition or effect list.
pub const PDDL8_MAX_CONJUNCTS: usize = 8;
/// Maximum action parameters (variables per schema).
pub const PDDL8_MAX_PARAMS: usize = 8;
/// Maximum plan depth in forward/BFS search.
pub const PDDL8_MAX_PLAN_DEPTH: usize = 64;
/// Default maximum ground action count (combinatorial explosion guard).
pub const PDDL8_MAX_GROUND: usize = 4096;

// ---------------------------------------------------------------------------
// Schema-level types (ungrounded)
// ---------------------------------------------------------------------------

/// A predicate atom that may contain variables (`?name`) or constants.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Pddl8Atom {
    pub pred: String,
    /// Each entry is `?varname` (variable) or a bare constant.
    pub args: Vec<String>,
}

impl Pddl8Atom {
    pub fn is_variable(arg: &str) -> bool {
        arg.starts_with('?')
    }

    pub fn arity(&self) -> usize {
        self.args.len()
    }
}

/// A PDDL8 action schema — bounded to PDDL8 caps.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pddl8ActionSchema {
    pub name: String,
    /// Parameter names (all start with `?`).
    pub params: Vec<String>,
    /// Conjunctive precondition (positive atoms only in STRIPS8).
    pub preconditions: Vec<Pddl8Atom>,
    /// Atoms added to state after action fires.
    pub add_effects: Vec<Pddl8Atom>,
    /// Atoms removed (become epoch-sealed in BRCE execution model).
    pub del_effects: Vec<Pddl8Atom>,
}

/// A PDDL8 domain: predicates + action schemas.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pddl8Domain {
    pub name: String,
    /// `(predicate_name, arity)` pairs for all declared predicates.
    pub predicates: Vec<(String, u8)>,
    pub actions: Vec<Pddl8ActionSchema>,
}

/// A PDDL8 problem: objects + initial state + goal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pddl8Problem {
    pub name: String,
    pub domain: String,
    pub objects: Vec<String>,
    /// Ground atoms true in S₀.
    pub init: Vec<Pddl8Atom>,
    /// Conjunctive goal — all must hold in Sₙ.
    pub goal: Vec<Pddl8Atom>,
}

// ---------------------------------------------------------------------------
// Ground-level types (after grounding = variable substitution)
// ---------------------------------------------------------------------------

/// A fully ground atom — all arguments are concrete constants.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub struct Pddl8GroundAtom {
    pub pred: String,
    pub args: Vec<String>,
}

impl Pddl8GroundAtom {
    pub fn label(&self) -> String {
        if self.args.is_empty() {
            self.pred.clone()
        } else {
            format!("{}({})", self.pred, self.args.join(","))
        }
    }
}

/// A fully ground action — ready for applicability testing and execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pddl8GroundAction {
    pub schema_name: String,
    /// Human-readable: `schema_name(arg0,arg1,...)`
    pub label: String,
    pub preconditions: Vec<Pddl8GroundAtom>,
    pub add_effects: Vec<Pddl8GroundAtom>,
    /// Delete effects — become epoch seals in the BRCE execution model.
    pub del_effects: Vec<Pddl8GroundAtom>,
}

impl Pddl8GroundAction {
    /// Test applicability: all preconditions must be in the current state set.
    pub fn is_applicable<S>(&self, state: &S) -> bool
    where
        S: Contains<Pddl8GroundAtom>,
    {
        self.preconditions.iter().all(|p| state.contains_atom(p))
    }
}

/// Trait for testing atom membership — allows callers to use BTreeSet, HashSet, etc.
pub trait Contains<T> {
    fn contains_atom(&self, item: &T) -> bool;
}

impl Contains<Pddl8GroundAtom> for std::collections::BTreeSet<Pddl8GroundAtom> {
    fn contains_atom(&self, item: &Pddl8GroundAtom) -> bool {
        self.contains(item)
    }
}

impl Contains<Pddl8GroundAtom> for std::collections::HashSet<Pddl8GroundAtom> {
    fn contains_atom(&self, item: &Pddl8GroundAtom) -> bool {
        self.contains(item)
    }
}

// ---------------------------------------------------------------------------
// Tape projection (POWL geometry)
// ---------------------------------------------------------------------------

/// One slot on the PDDL8 execution tape.
///
/// Sequential ordering: `pred_mask = 1 << (index - 1)`.
/// Op 0 has no predecessors (`pred_mask = 0`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pddl8TapeOp {
    pub index: u8,
    pub label: String,
    /// Bitmask of ops that must complete before this one is eligible.
    pub pred_mask: u64,
    pub action: Pddl8GroundAction,
}

/// A PDDL8 tape — the POWL geometry for one candidate plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pddl8Tape {
    pub ops: Vec<Pddl8TapeOp>,
}

impl Pddl8Tape {
    /// Project a ground plan (ordered list) into a sequential POWL tape.
    pub fn from_plan(plan: Vec<Pddl8GroundAction>) -> Self {
        let ops = plan
            .into_iter()
            .enumerate()
            .map(|(i, action)| Pddl8TapeOp {
                index: i as u8,
                label: action.label.clone(),
                pred_mask: if i == 0 { 0 } else { 1u64 << (i - 1) },
                action,
            })
            .collect();
        Self { ops }
    }

    pub fn len(&self) -> usize {
        self.ops.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ops.is_empty()
    }
}

// ---------------------------------------------------------------------------
// Execution result types
// ---------------------------------------------------------------------------

/// Outcome of a single tape op execution with Prolog8 admission.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pddl8StepResult {
    pub op_index: u8,
    pub label: String,
    pub admitted: bool,
    pub epoch_after: u64,
    /// BLAKE3 hex of the cumulative receipt chain up to this step.
    pub receipt_hash: String,
}

/// Full execution log for a PDDL8 plan run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pddl8ExecutionLog {
    pub steps: Vec<Pddl8StepResult>,
    pub goal_reached: bool,
    /// Cumulative BLAKE3 chain hash over all step receipts + goal outcome.
    pub chain_hash: String,
}

/// Replayable BLAKE3 receipt for one PDDL8 plan execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pddl8ExecutionReceipt {
    /// BLAKE3 over ordered op labels (plan identity).
    pub plan_root: String,
    /// BLAKE3 over initial state atoms (S₀ identity).
    pub state_root: String,
    /// BLAKE3 over goal atoms (goal identity).
    pub goal_root: String,
    /// Cumulative chain hash from `Pddl8ExecutionLog`.
    pub chain_hash: String,
    pub goal_reached: bool,
    pub step_count: usize,
}

// ---------------------------------------------------------------------------
// Conversion from wasm4pm-cognition BreedInput shapes
// ---------------------------------------------------------------------------

/// Convert a wasm4pm-cognition `Rule` (premise/conclusion/id) into a
/// `Pddl8ActionSchema`. This is the bridge from the Old-AI breed inputs
/// (STRIPS, HTN, POP) into canonical PDDL8 types.
///
/// Convention (from strips.rs / htn_planning.rs):
/// - `rule.id`        → `schema.name`
/// - `rule.premise`   → `schema.preconditions` (each as `pred=val` split)
/// - `rule.conclusion` → semicolon-list of `add1;add2;!del1` effects
pub fn schema_from_rule(
    rule_id: &str,
    premises: &[String],
    conclusion: &str,
) -> Pddl8ActionSchema {
    let preconditions = premises
        .iter()
        .map(|p| atom_from_pred_eq(p))
        .collect();

    let mut add_effects = Vec::new();
    let mut del_effects = Vec::new();
    for tok in conclusion.split(';').map(str::trim).filter(|s| !s.is_empty()) {
        if let Some(rest) = tok.strip_prefix('!') {
            del_effects.push(atom_from_pred_eq(rest));
        } else {
            add_effects.push(atom_from_pred_eq(tok));
        }
    }

    Pddl8ActionSchema {
        name: rule_id.to_string(),
        params: vec![],
        preconditions,
        add_effects,
        del_effects,
    }
}

fn atom_from_pred_eq(s: &str) -> Pddl8Atom {
    // `predicate=value` → Pddl8Atom { pred: "predicate", args: ["value"] }
    if let Some((p, v)) = s.split_once('=') {
        Pddl8Atom { pred: p.to_string(), args: vec![v.to_string()] }
    } else {
        Pddl8Atom { pred: s.to_string(), args: vec![] }
    }
}
