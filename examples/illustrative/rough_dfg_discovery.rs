//! Rough DFG (Directly-Follows Graph) Discovery.
//!
//! This example implements a "rough" DFG miner that takes an XES-style EventLog
//! as input and produces a DFG model.
//!
//! Run with: `cargo run --example rough_dfg_discovery`

use std::collections::HashMap;
use wasm4pm_compat::event_log::{
    AttributeValue, Attributes, Event, EventLog, Trace, XESEditableAttribute,
};
use wasm4pm_compat::models::{DFGEdge, DFGNode, DFG};

/// A "rough" DFG miner implementation.
///
/// It iterates through traces and events to count activity frequencies
/// and directly-follows relationships.
fn discover_rough_dfg(log: &EventLog) -> DFG {
    let mut node_freq = HashMap::new();
    let mut edge_freq = HashMap::new();
    let mut start_activities = HashMap::new();
    let mut end_activities = HashMap::new();

    for trace in &log.traces {
        // We extract activity names from the "concept:name" attribute,
        // which is standard for XES event logs.
        let activities: Vec<String> = trace
            .events
            .iter()
            .filter_map(|e| e.get_activity("concept:name"))
            .collect();

        if activities.is_empty() {
            continue;
        }

        // Count start activity
        if let Some(first) = activities.first() {
            *start_activities.entry(first.clone()).or_insert(0) += 1;
        }

        // Count end activity
        if let Some(last) = activities.last() {
            *end_activities.entry(last.clone()).or_insert(0) += 1;
        }

        // Count node and edge frequencies
        let mut prev_activity: Option<String> = None;
        for activity in activities {
            *node_freq.entry(activity.clone()).or_insert(0) += 1;

            if let Some(prev) = prev_activity {
                *edge_freq.entry((prev, activity.clone())).or_insert(0) += 1;
            }
            prev_activity = Some(activity);
        }
    }

    // Transform internal maps into the canonical DFG structure.
    let nodes = node_freq
        .into_iter()
        .map(|(activity, frequency)| DFGNode::new(activity, frequency))
        .collect();

    let edges = edge_freq
        .into_iter()
        .map(|((source, target), frequency)| DFGEdge::new(source, target, frequency))
        .collect();

    DFG {
        nodes,
        edges,
        start_activities: start_activities.into_keys().collect(),
        end_activities: end_activities.into_keys().collect(),
    }
}

fn create_sample_log() -> EventLog {
    let mut log = EventLog::default();

    // Helper to create an event with an activity name
    let event = |name: &str| {
        let mut attrs: Attributes = Vec::new();
        attrs.add_to_attributes(
            "concept:name".to_string(),
            AttributeValue::String(name.to_string()),
        );
        Event::new(attrs)
    };

    // Trace 1: A -> B -> C
    log.traces.push(Trace::new(
        "case-1".to_string(),
        vec![event("A"), event("B"), event("C")],
    ));

    // Trace 2: A -> C
    log.traces.push(Trace::new(
        "case-2".to_string(),
        vec![event("A"), event("C")],
    ));

    // Trace 3: A -> B -> B -> C
    log.traces.push(Trace::new(
        "case-3".to_string(),
        vec![event("A"), event("B"), event("B"), event("C")],
    ));

    log
}

fn main() {
    println!("== wasm4pm-compat: Rough DFG Discovery ==");

    let log = create_sample_log();
    println!(
        "Log created with {} traces and {} events.",
        log.len(),
        log.event_count()
    );

    let dfg = discover_rough_dfg(&log);

    println!("\nDiscovered DFG Summary:");
    println!("Nodes (Activities): {}", dfg.nodes.len());
    for node in &dfg.nodes {
        println!("  - {}: freq={}", node.activity, node.frequency);
    }

    println!("Edges (Directly-Follows): {}", dfg.edges.len());
    for edge in &dfg.edges {
        println!(
            "  - {} -> {}: freq={}",
            edge.source, edge.target, edge.frequency
        );
    }

    println!("\nStart Activities: {:?}", dfg.start_activities);
    println!("End Activities: {:?}", dfg.end_activities);

    println!("\nDiscovery complete. (Rough structural implementation)");
}
