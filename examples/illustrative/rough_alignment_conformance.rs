//! # Rough Alignment Conformance Example
//!
//! This example implements a "rough" alignment-based conformance generator.
//! Alignment is the process of finding the most likely sequence of moves
//! (Synchronous, Log-only, or Model-only) that explains a trace given a Petri net.
//!
//! This implementation is for illustrative and type-testing purposes.
//! Production-grade alignment belongs in the `wasm4pm` engine.

use std::collections::{HashMap, HashSet, VecDeque};
use wasm4pm_compat::conformance::ConformanceResult;
use wasm4pm_compat::eventlog::{Event, Trace};
use wasm4pm_compat::petri::{Marking, PetriNet};

// ── Move Types ──────────────────────────────────────────────────────────────

/// A single move in an alignment.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Move {
    /// Synchronous move: The log and the model agree on an activity.
    Sync(String),
    /// Log-only move: The log contains an activity that the model does not expect (Insertion).
    Log(String),
    /// Model-only move: The model requires an activity that is missing from the log (Skip).
    Model(String),
}

// ── Alignment State ─────────────────────────────────────────────────────────

/// A state in the alignment search space (marking + trace progress).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct AlignmentState {
    marking: Vec<(String, usize)>, // Simplified marking for hashing
    trace_idx: usize,
}

impl AlignmentState {
    fn new(marking: HashMap<String, usize>, trace_idx: usize) -> Self {
        let mut m: Vec<_> = marking.into_iter().collect();
        m.sort_by(|a, b| a.0.cmp(&b.0));
        AlignmentState {
            marking: m,
            trace_idx,
        }
    }

    fn to_marking_map(&self) -> HashMap<String, usize> {
        self.marking.iter().cloned().collect()
    }
}

// ── Alignment Generator ─────────────────────────────────────────────────────

/// Computes a rough alignment using a simple Breadth-First Search (BFS).
///
/// Costs:
/// - Sync move: 0
/// - Log move: 1
/// - Model move: 1 (0 for silent/invisible transitions)
pub fn compute_rough_alignment(trace: &Trace, net: &PetriNet) -> (Vec<Move>, f64) {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    let initial_marking = get_initial_marking(net);
    let initial_state = AlignmentState::new(initial_marking, 0);

    queue.push_back((initial_state.clone(), Vec::new(), 0));
    visited.insert(initial_state);

    let mut best_alignment: Option<(Vec<Move>, usize)> = None;

    while let Some((current_state, moves, cost)) = queue.pop_front() {
        // Goal check: All events processed AND reachable final marking (if any).
        if current_state.trace_idx == trace.len() {
            if is_final_marking(&current_state.to_marking_map(), net) {
                if best_alignment.is_none() || cost < best_alignment.as_ref().unwrap().1 {
                    best_alignment = Some((moves.clone(), cost));
                }
                if cost == 0 {
                    break; // Optimal found
                }
            }
        }

        // Safety limit for BFS in this illustrative example
        if visited.len() > 2000 {
            break;
        }

        let marking = current_state.to_marking_map();

        // 1. Try Sync Moves
        if current_state.trace_idx < trace.len() {
            let event = &trace.events()[current_state.trace_idx];
            let activity = event.activity();
            for t in net.transitions() {
                if t.label() == activity && is_enabled(t, &marking, net) {
                    let mut next_marking = marking.clone();
                    fire_transition(t, &mut next_marking, net);
                    let next_state = AlignmentState::new(next_marking, current_state.trace_idx + 1);
                    if !visited.contains(&next_state) {
                        visited.insert(next_state.clone());
                        let mut next_moves = moves.clone();
                        next_moves.push(Move::Sync(activity.to_string()));
                        queue.push_back((next_state, next_moves, cost));
                    }
                }
            }
        }

        // 2. Try Model Moves (Model-only)
        for t in net.transitions() {
            if is_enabled(t, &marking, net) {
                let mut next_marking = marking.clone();
                fire_transition(t, &mut next_marking, net);
                let next_state = AlignmentState::new(next_marking, current_state.trace_idx);
                if !visited.contains(&next_state) {
                    visited.insert(next_state.clone());
                    let mut next_moves = moves.clone();
                    next_moves.push(Move::Model(t.id().to_string()));
                    let move_cost = if t.is_silent() { 0 } else { 1 };
                    queue.push_back((next_state, next_moves, cost + move_cost));
                }
            }
        }

        // 3. Try Log Moves (Log-only)
        if current_state.trace_idx < trace.len() {
            let event = &trace.events()[current_state.trace_idx];
            let activity = event.activity();
            let next_state = AlignmentState::new(marking.clone(), current_state.trace_idx + 1);
            if !visited.contains(&next_state) {
                visited.insert(next_state.clone());
                let mut next_moves = moves.clone();
                next_moves.push(Move::Log(activity.to_string()));
                queue.push_back((next_state, next_moves, cost + 1));
            }
        }
    }

    let (final_moves, final_cost) = best_alignment.unwrap_or_else(|| (vec![], trace.len() + 1));

    // Calculate a rough fitness score: 1.0 - (cost / max_possible_cost)
    let max_cost = (trace.len() + 1) as f64;
    let fitness = 1.0 - (final_cost as f64 / max_cost).min(1.0);

    (final_moves, fitness)
}

// ── Petri Net Helpers ───────────────────────────────────────────────────────

fn get_initial_marking(net: &PetriNet) -> HashMap<String, usize> {
    let mut marking = HashMap::new();
    for (_, id, count) in net.initial_marking.iter() {
        if *count > 0 {
            marking.insert(id.clone(), *count);
        }
    }
    marking
}

fn is_final_marking(marking: &HashMap<String, usize>, net: &PetriNet) -> bool {
    if net.final_markings.is_empty() {
        return true; // Weak goal: just finish the trace
    }
    for fm in &net.final_markings {
        let mut matched = true;
        for (_, id, count) in fm.iter() {
            if marking.get(id).cloned().unwrap_or(0) != *count {
                matched = false;
                break;
            }
        }
        if matched {
            return true;
        }
    }
    false
}

fn is_enabled(
    t: &wasm4pm_compat::models::Transition,
    marking: &HashMap<String, usize>,
    net: &PetriNet,
) -> bool {
    for arc in net.arcs() {
        if arc.is_place_to_transition && arc.to == t.id() {
            let weight = arc.weight.unwrap_or(1);
            if marking.get(&arc.from).cloned().unwrap_or(0) < weight {
                return false;
            }
        }
    }
    true
}

fn fire_transition(
    t: &wasm4pm_compat::models::Transition,
    marking: &mut HashMap<String, usize>,
    net: &PetriNet,
) {
    for arc in net.arcs() {
        if arc.is_place_to_transition && arc.to == t.id() {
            let weight = arc.weight.unwrap_or(1);
            let entry = marking.entry(arc.from.clone()).or_insert(0);
            *entry = entry.saturating_sub(weight);
        } else if !arc.is_place_to_transition && arc.from == t.id() {
            let weight = arc.weight.unwrap_or(1);
            let entry = marking.entry(arc.to.clone()).or_insert(0);
            *entry += weight;
        }
    }
}

// ── Main ────────────────────────────────────────────────────────────────────

fn main() {
    // 1. Define a simple linear Petri net: (p1) --[t1: "A"]--> (p2)
    let p1 = wasm4pm_compat::models::Place::new("p1");
    let p2 = wasm4pm_compat::models::Place::new("p2");
    let t1 = wasm4pm_compat::models::Transition::new("t1", "A");
    let a1 = wasm4pm_compat::models::Arc::place_to_transition("p1", "t1");
    let a2 = wasm4pm_compat::models::Arc::transition_to_place("t1", "p2");

    let initial_marking = Marking::new([("p1".to_string(), 1)]);
    let mut net = PetriNet::new([p1, p2], [t1], [a1, a2], initial_marking);

    // Set final marking: p2 has 1 token
    let mut fm = wasm4pm_compat::dense_kernel::PackedKeyTable::new();
    fm.insert(
        wasm4pm_compat::dense_kernel::fnv1a_64(b"p2"),
        "p2".to_string(),
        1,
    );
    net.final_markings.push(fm);

    // 2. Define a trace that fits: [A]
    let event_a = Event::new("A");
    let fitting_trace = Trace::from_events([event_a]);

    println!("--- Fitting Trace ---");
    let (alignment, fitness) = compute_rough_alignment(&fitting_trace, &net);
    println!("Alignment: {:?}", alignment);
    println!("Fitness: {:.2}", fitness);

    let result = ConformanceResult::new(fitness, 1, 1, 0);
    println!("Conformance Result: {:?}\n", result);

    // 3. Define a trace that deviates: [B]
    let event_b = Event::new("B");
    let deviating_trace = Trace::from_events([event_b]);

    println!("--- Deviating Trace ---");
    let (alignment, fitness) = compute_rough_alignment(&deviating_trace, &net);
    println!("Alignment: {:?}", alignment);
    println!("Fitness: {:.2}", fitness);

    let result = ConformanceResult::new(fitness, 1, 0, 1);
    println!("Conformance Result: {:?}", result);
}
