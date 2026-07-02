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

/// A precondition that must be satisfied for an action to fire.
/// Maps from praxis_core::Obligation::Precondition.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Precondition {
    /// Identifier for the predicate being checked.
    pub predicate_id: String,
    /// Hash of the parameters passed to the predicate.
    pub params_hash: Option<[u8; 32]>,
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
    // PDDL 3.1 extended fields
    #[serde(default)]
    pub typed_params: Vec<(String, String)>,   // (?var, type) pairs
    #[serde(default)]
    pub condition: Option<PddlCondition>,       // full condition algebra
    #[serde(default)]
    pub effects: Vec<PddlEffect>,               // full effect algebra
    #[serde(default)]
    pub numeric_effects: Vec<NumericEffect>,
}

/// A PDDL8 domain: predicates + action schemas.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pddl8Domain {
    pub name: String,
    /// `(predicate_name, arity)` pairs for all declared predicates.
    pub predicates: Vec<(String, u8)>,
    pub actions: Vec<Pddl8ActionSchema>,
    // PDDL 3.1 extended fields (additive, default to empty)
    #[serde(default)]
    pub types: Vec<PddlType>,
    #[serde(default)]
    pub functions: Vec<PddlFunction>,
    #[serde(default)]
    pub durative_actions: Vec<DurativeAction>,
    #[serde(default)]
    pub derived: Vec<DerivedPredicate>,
    #[serde(default)]
    pub constraints: Vec<PddlConstraint>,
    #[serde(default)]
    pub processes: Vec<PddlProcess>,
    #[serde(default)]
    pub events: Vec<PddlEvent>,
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
    // PDDL 3.1 extended fields (additive, default to empty/None)
    /// `(object_name, type)` pairs — sibling of `objects`, used to restrict
    /// grounding to type-compatible bindings. Empty for untyped domains.
    #[serde(default)]
    pub object_types: Vec<(String, String)>,
    #[serde(default)]
    pub fn_values: Vec<(PddlFunction, f64)>,
    #[serde(default)]
    pub timed_inits: Vec<TimedLiteral>,
    #[serde(default)]
    pub preferences: Vec<PddlPreference>,
    #[serde(default)]
    pub metric: Option<Metric>,
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
        typed_params: vec![],
        condition: None,
        effects: vec![],
        numeric_effects: vec![],
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

// ─────────────────────────────────────────────────────────────────────────────
// PDDL 3.1 Extended Types
// These are additive — Pddl8* types above are preserved for backwards compat.
// ─────────────────────────────────────────────────────────────────────────────

// --- Type hierarchy -----------------------------------------------------------

/// A PDDL type declaration: (vehicle) or (truck - vehicle)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PddlType {
    pub name: String,
    pub parent: Option<String>,
}

// --- Condition algebra --------------------------------------------------------

/// Full PDDL 3.1 condition (superset of Pddl8Atom conjunction).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PddlCondition {
    Atom(Pddl8Atom),
    Not(Box<PddlCondition>),
    And(Vec<PddlCondition>),
    Or(Vec<PddlCondition>),
    Forall { vars: Vec<(String, String)>, body: Box<PddlCondition> },
    Exists { vars: Vec<(String, String)>, body: Box<PddlCondition> },
    Imply(Box<PddlCondition>, Box<PddlCondition>),
    /// (at start cond) | (over all cond) | (at end cond) inside durative conditions
    Timed(TimeSpecifier, Box<PddlCondition>),
    /// Numeric fluent comparison, e.g. `(>= (available-workers) 1)`.
    Compare(NumericExpr, CompareOp, NumericExpr),
}

/// Comparison operator for a numeric fluent precondition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompareOp { Ge, Le, Gt, Lt, Eq }

impl Default for PddlCondition {
    fn default() -> Self { PddlCondition::And(vec![]) }
}

// --- Numeric fluents ----------------------------------------------------------

/// A function symbol (fuel-level ?v) or (total-cost)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PddlFunction {
    pub name: String,
    pub params: Vec<String>,
}

/// Arithmetic expression over numeric fluents.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NumericExpr {
    Number(f64),
    FunctionTerm(String, Vec<String>),   // (fuel-level ?v)
    BinOp { op: NumericOp, lhs: Box<NumericExpr>, rhs: Box<NumericExpr> },
    Neg(Box<NumericExpr>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NumericOp { Add, Sub, Mul, Div }

/// Effect that modifies a numeric fluent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NumericEffect {
    Assign(PddlFunction, NumericExpr),
    Increase(PddlFunction, NumericExpr),
    Decrease(PddlFunction, NumericExpr),
    ScaleUp(PddlFunction, NumericExpr),
    ScaleDown(PddlFunction, NumericExpr),
}

// --- Effect algebra -----------------------------------------------------------

/// A general PDDL effect (propositional, numeric, conditional, or timed).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PddlEffect {
    Add(Pddl8Atom),
    Del(Pddl8Atom),
    Numeric(NumericEffect),
    /// at start / at end wrapper
    Timed(TimeSpecifier, Box<PddlEffect>),
    Forall { vars: Vec<(String, String)>, effects: Vec<PddlEffect> },
    When { condition: PddlCondition, effects: Vec<PddlEffect> },
}

// --- Temporal PDDL 2.1 -------------------------------------------------------

/// Temporal qualifier for durative action conditions/effects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TimeSpecifier { AtStart, AtEnd, OverAll }

/// Duration constraint: (= ?duration X), (<= ...), (>= ...), (and ...)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DurationConstraint {
    Eq(NumericExpr),
    Lte(NumericExpr),
    Gte(NumericExpr),
    And(Vec<DurationConstraint>),
}

/// A PDDL 2.1 durative action.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DurativeAction {
    pub name: String,
    /// (var_name, type_name) pairs
    pub params: Vec<(String, String)>,
    pub duration: DurationConstraint,
    /// Conditions tagged with at-start / over-all / at-end
    pub conditions: Vec<PddlCondition>,
    /// Effects tagged with at-start / at-end (using PddlEffect::Timed)
    pub effects: Vec<PddlEffect>,
}

/// A timed initial literal: (at <time> <literal>) in the problem :init.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimedLiteral {
    pub time: f64,
    pub atom: Pddl8Atom,
    pub negated: bool,
}

// --- Metric ------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MetricExpr {
    Number(f64),
    FunctionTerm(String, Vec<String>),
    TotalTime,
    IsViolated(String),
    BinOp { op: NumericOp, lhs: Box<MetricExpr>, rhs: Box<MetricExpr> },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MetricDir { Minimize, Maximize }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metric { pub dir: MetricDir, pub expr: MetricExpr }

// --- PDDL 3.x constraints and preferences ------------------------------------

/// PDDL 3.x trajectory constraint (modal operator over plan trajectory).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TrajectoryConstraint {
    Always(Box<PddlCondition>),
    Sometime(Box<PddlCondition>),
    Within(f64, Box<PddlCondition>),
    AtMostOnce(Box<PddlCondition>),
    SometimeBefore(Box<PddlCondition>, Box<PddlCondition>),
    SometimeAfter(Box<PddlCondition>, Box<PddlCondition>),
    AlwaysWithin(f64, Box<PddlCondition>, Box<PddlCondition>),
    HoldDuring(f64, f64, Box<PddlCondition>),
    HoldAfter(f64, Box<PddlCondition>),
    And(Vec<TrajectoryConstraint>),
}

/// A named or anonymous PDDL 3.x preference.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PddlPreference {
    pub name: Option<String>,
    pub constraint: TrajectoryConstraint,
}

/// A domain-level constraint (possibly named).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PddlConstraint {
    pub name: Option<String>,
    pub constraint: TrajectoryConstraint,
}

// --- Derived predicates ------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DerivedPredicate {
    pub head: Pddl8Atom,
    pub body: PddlCondition,
}

// --- PDDL+ processes and events ----------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PddlProcess {
    pub name: String,
    pub params: Vec<(String, String)>,
    pub precondition: PddlCondition,
    pub effects: Vec<NumericEffect>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PddlEvent {
    pub name: String,
    pub params: Vec<(String, String)>,
    pub precondition: PddlCondition,
    pub effects: Vec<PddlEffect>,
}

// --- Extended domain and problem types ---------------------------------------

/// Extended PDDL 3.1 domain (includes all PDDL 3.1 features).
/// Pddl8Domain is retained for STRIPS-only compatibility.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Pddl31Domain {
    pub name: String,
    /// (:requirements ...) flags
    pub requirements: Vec<String>,
    /// (:types ...)
    pub types: Vec<PddlType>,
    /// (:predicates ...) — name + typed param list
    pub predicates: Vec<(String, Vec<(String, String)>)>,
    /// (:functions ...)
    pub functions: Vec<PddlFunction>,
    /// (:action ...) — classical PDDL actions (with full condition algebra)
    pub actions: Vec<Pddl31Action>,
    /// (:durative-action ...)
    pub durative_actions: Vec<DurativeAction>,
    /// (:derived ...)
    pub derived: Vec<DerivedPredicate>,
    /// (:constraints ...)
    pub constraints: Vec<PddlConstraint>,
    /// (:process ...) — PDDL+
    pub processes: Vec<PddlProcess>,
    /// (:event ...) — PDDL+
    pub events: Vec<PddlEvent>,
}

/// A full PDDL 3.1 action schema (richer than Pddl8ActionSchema).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pddl31Action {
    pub name: String,
    /// (var_name, type_name) pairs
    pub params: Vec<(String, String)>,
    pub precondition: PddlCondition,
    pub effect: Vec<PddlEffect>,
}

/// Extended PDDL 3.1 problem.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Pddl31Problem {
    pub name: String,
    pub domain: String,
    /// (:objects ...) — (name, type) pairs
    pub objects: Vec<(String, String)>,
    /// (:init ...) — propositional atoms
    pub init_atoms: Vec<Pddl8Atom>,
    /// (:init ...) — numeric fluent initial values
    pub init_fn_values: Vec<(PddlFunction, f64)>,
    /// (:init (at T lit) ...) — timed initial literals
    pub timed_inits: Vec<TimedLiteral>,
    /// (:goal ...)
    pub goal: PddlCondition,
    /// (:constraints ...) with preferences
    pub preferences: Vec<PddlPreference>,
    /// (:metric ...)
    pub metric: Option<Metric>,
}

// --- Temporal planning runtime types -----------------------------------------

/// Ground action with temporal timing information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalPlanStep {
    pub start_time: f64,
    pub duration: f64,
    pub action_name: String,
    pub args: Vec<String>,
}

/// A PDDL temporal plan (OPTIC/POPF-compatible output format).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TemporalPlan {
    pub steps: Vec<TemporalPlanStep>,
    pub makespan: f64,
    pub metric_value: Option<f64>,
}

/// BLAKE3-chained receipt for temporal plan execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalExecutionReceipt {
    pub plan_root: String,
    pub state_root: String,
    pub goal_root: String,
    pub makespan: f64,
    pub step_count: usize,
    pub requirements: Vec<String>,
    pub goal_reached: bool,
    pub chain_hash: String,
}
