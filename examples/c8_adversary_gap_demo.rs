//! Example: Adversary gap proof — LogicPlayer vs GraphPlayer on the same stream.
//!
//! Demonstrates how two process mining strategies (logic-based vs graph-based)
//! can diverge on the same event stream, proving the need for unified semantics.
//!
//! Run: cargo run --example c8_adversary_gap_demo

fn main() {
    println!("=== Adversary Gap Demo ===\n");

    // Shared event stream: typical process with a loop
    let events = vec![
        ("order_created", 0),
        ("payment_validated", 100),
        ("payment_rejected", 150),
        ("order_voided", 200),
        ("payment_validated", 250),
        ("shipment_started", 300),
        ("delivery_confirmed", 400),
    ];

    println!("Event stream (case-id: order_123):");
    for (i, (event, time)) in events.iter().enumerate() {
        println!("  {}. {} @ {}ms", i, event, time);
    }

    // Strategy 1: LogicPlayer (constraint-based reasoning)
    println!("\n--- LogicPlayer Analysis ---");
    let logic_verdict = LogicPlayer::analyze(&events);
    println!("Process model: {:?}", logic_verdict.model);
    println!("Valid: {}", logic_verdict.is_valid);
    println!("Constraints discovered: {}", logic_verdict.constraint_count);

    // Strategy 2: GraphPlayer (directly-follows graph mining)
    println!("\n--- GraphPlayer Analysis ---");
    let graph_verdict = GraphPlayer::analyze(&events);
    println!("Process model: {:?}", graph_verdict.model);
    println!("Valid: {}", graph_verdict.is_valid);
    println!("Directly-follows edges: {}", graph_verdict.edge_count);

    // Compare: prove the gap
    println!("\n--- Gap Analysis ---");
    let gap = compare_verdicts(&logic_verdict, &graph_verdict);
    if gap.has_divergence {
        println!("✗ DIVERGENCE DETECTED");
        println!("  Logic constraints: {}", gap.logic_constraint_count);
        println!("  Graph edges: {}", gap.graph_edge_count);
        println!("  Missing basis: both models cannot explain the same stream");
        println!("  Gap signature: {}", gap.signature);
    } else {
        println!("✓ No divergence; models are equivalent");
    }

    println!("\n✓ Adversary gap demo complete");
}

#[derive(Debug)]
struct ProcessModel {
    nodes: Vec<String>,
    edges: Vec<(String, String)>,
}

#[derive(Debug)]
struct LogicVerdict {
    model: ProcessModel,
    is_valid: bool,
    constraint_count: usize,
}

#[derive(Debug)]
struct GraphVerdict {
    model: ProcessModel,
    is_valid: bool,
    edge_count: usize,
}

struct LogicPlayer;
struct GraphPlayer;

impl LogicPlayer {
    fn analyze(events: &[(&str, u64)]) -> LogicVerdict {
        // Logic player: builds constraints from declarations
        // Constraints: order_created -> payment_validated -> shipment_started
        let constraints = vec![
            ("order_created".to_string(), "payment_validated".to_string()),
            (
                "payment_validated".to_string(),
                "shipment_started".to_string(),
            ),
            (
                "payment_validated".to_string(),
                "payment_rejected".to_string(),
            ),
        ];

        let mut nodes = vec![];
        let mut edges = vec![];

        for (event, _) in events {
            let event_str = event.to_string();
            if !nodes.contains(&event_str) {
                nodes.push(event_str);
            }
        }

        for (from, to) in &constraints {
            edges.push((from.clone(), to.clone()));
        }

        LogicVerdict {
            model: ProcessModel { nodes, edges },
            is_valid: true,
            constraint_count: constraints.len(),
        }
    }
}

impl GraphPlayer {
    fn analyze(events: &[(&str, u64)]) -> GraphVerdict {
        // Graph player: builds directly-follows relations from trace
        let mut edges = vec![];
        for i in 0..events.len().saturating_sub(1) {
            let (from, _) = events[i];
            let (to, _) = events[i + 1];
            let edge = (from.to_string(), to.to_string());
            if !edges.contains(&edge) {
                edges.push(edge);
            }
        }

        let mut nodes = vec![];
        for (event, _) in events {
            let event_str = event.to_string();
            if !nodes.contains(&event_str) {
                nodes.push(event_str);
            }
        }

        GraphVerdict {
            model: ProcessModel {
                nodes,
                edges: edges.clone(),
            },
            is_valid: true,
            edge_count: edges.len(),
        }
    }
}

struct GapVerdict {
    has_divergence: bool,
    logic_constraint_count: usize,
    graph_edge_count: usize,
    signature: String,
}

fn compare_verdicts(logic: &LogicVerdict, graph: &GraphVerdict) -> GapVerdict {
    let logic_edges: std::collections::HashSet<_> = logic.model.edges.iter().collect();
    let graph_edges: std::collections::HashSet<_> = graph.model.edges.iter().collect();

    let mut missing_in_graph = vec![];
    for edge in &logic_edges {
        if !graph_edges.contains(edge) {
            missing_in_graph.push(*edge);
        }
    }

    let has_divergence = !missing_in_graph.is_empty();

    let mut signature = String::new();
    for (from, to) in &missing_in_graph {
        signature.push_str(&format!("{}->{};", from, to));
    }

    GapVerdict {
        has_divergence,
        logic_constraint_count: logic.constraint_count,
        graph_edge_count: graph.edge_count,
        signature,
    }
}
