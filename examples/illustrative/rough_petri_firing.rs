//! Example: Rough Petri Net token simulator
//!
//! Demonstrates a simple "rough" token simulation over the `PetriNet`,
//! `Place`, and `Transition` types from `wasm4pm-compat`.
//!
//! Run: cargo run --example rough_petri_firing

use std::collections::HashMap;
use wasm4pm_compat::petri::{Arc, PetriNet, Place, Transition};

/// A simple Petri Net simulator.
struct RoughSimulator<'a> {
    net: &'a PetriNet,
    marking: HashMap<String, usize>,
}

impl<'a> RoughSimulator<'a> {
    fn new(net: &'a PetriNet) -> Self {
        let mut marking = HashMap::new();
        // Initialize from the net's initial marking
        for place in net.places() {
            let tokens = net.initial_marking().tokens_on(place.id());
            if tokens > 0 {
                marking.insert(place.id().to_string(), tokens);
            }
        }
        Self { net, marking }
    }

    /// Check if a transition is enabled.
    fn is_enabled(&self, transition_id: &str) -> bool {
        // Find all input arcs to this transition
        for arc in self.net.arcs() {
            if arc.to == transition_id && arc.is_place_to_transition {
                let weight = arc.weight.unwrap_or(1);
                let tokens = self.marking.get(&arc.from).copied().unwrap_or(0);
                if tokens < weight {
                    return false;
                }
            }
        }
        true
    }

    /// Fire a transition if enabled.
    fn fire(&mut self, transition_id: &str) {
        if !self.is_enabled(transition_id) {
            println!("Transition {} is NOT enabled!", transition_id);
            return;
        }

        println!("Firing transition: {}", transition_id);

        // 1. Consume tokens from input places
        for arc in self.net.arcs() {
            if arc.to == transition_id && arc.is_place_to_transition {
                let weight = arc.weight.unwrap_or(1);
                let tokens = self.marking.get_mut(&arc.from).expect("Place must exist");
                *tokens -= weight;
            }
        }

        // 2. Produce tokens in output places
        for arc in self.net.arcs() {
            if arc.from == transition_id && !arc.is_place_to_transition {
                let weight = arc.weight.unwrap_or(1);
                let tokens = self.marking.entry(arc.to.clone()).or_insert(0);
                *tokens += weight;
            }
        }
    }

    fn print_marking(&self) {
        let mut places: Vec<_> = self.marking.iter().filter(|(_, &v)| v > 0).collect();
        places.sort_by_key(|(k, _)| *k);
        println!("  Current marking: {:?}", places);
    }
}

fn main() {
    println!("=== Rough Petri Net Firing Simulator ===\n");

    // 1. Initialize a simple Petri net: p1 -> t1 -> p2 -> t2 -> p3
    //                                       p2 -> t3 -> p4
    let places = vec![
        Place::new("p1"),
        Place::new("p2"),
        Place::new("p3"),
        Place::new("p4"),
    ];

    let transitions = vec![
        Transition::new("t1", "Task 1"),
        Transition::new("t2", "Task 2"),
        Transition::new("t3", "Task 3"),
    ];

    let arcs = vec![
        Arc::place_to_transition("p1", "t1"),
        Arc::transition_to_place("t1", "p2"),
        Arc::place_to_transition("p2", "t2"),
        Arc::transition_to_place("t2", "p3"),
        Arc::place_to_transition("p2", "t3"),
        Arc::transition_to_place("t3", "p4"),
    ];

    // Initial marking: 1 token on p1
    // PetriNet::new expects (places, transitions, arcs, initial_marking)
    // Note: wasm4pm_compat::petri::Marking is used for initial_marking in the constructor
    use wasm4pm_compat::petri::Marking;
    let initial = Marking::new([("p1".to_string(), 1)]);

    let net = PetriNet::new(places, transitions, arcs, initial);

    let mut sim = RoughSimulator::new(&net);

    println!("Starting simulation...");
    sim.print_marking();

    // Try to fire t2 (should fail)
    println!("\nAttempting to fire t2...");
    sim.fire("t2");
    sim.print_marking();

    // Fire t1
    println!("\nAttempting to fire t1...");
    sim.fire("t1");
    sim.print_marking();

    // Now t2 and t3 should be enabled. Let's fire t3.
    println!("\nAttempting to fire t3...");
    sim.fire("t3");
    sim.print_marking();

    println!("\nSimulation complete.");
}
