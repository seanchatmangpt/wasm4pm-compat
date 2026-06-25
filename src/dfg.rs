//! Directly-follows graph (DFG) — shapes and native discovery from OCEL.
//!
//! ## What this module IS
//!
//! - The structural shape of a DFG: activities as nodes, directly-follows
//!   relations as weighted edges, and named refusals for malformed graphs.
//! - `discover_ocel_dfg` — native Rust DFG miner over `OCEL` (no WASM required).
//!   This is the authoritative process-intelligence entry point for non-WASM consumers.
//!
//! The algorithm is the same as `wasm4pm::discovery::discover_ocel_dfg_pure` but
//! uses `wasm4pm_compat` types throughout so native binaries can depend on this
//! crate without pulling in `wasm-bindgen`.

use std::collections::BTreeMap;

pub use crate::models::{DFGEdge, DFGNode, DFG};
use crate::ocel::OCEL;

/// Discover a Directly-Follows Graph from an OCEL 2.0 log.
///
/// Groups events by shared object membership, sorts by timestamp, and counts
/// directly-follows pairs. This is the native (non-WASM) authoritative miner —
/// equivalent to `wasm4pm::discovery::discover_ocel_dfg_pure`.
pub fn discover_ocel_dfg(ocel: &OCEL) -> DFG {
    let mut dfg = DFG::new();

    // Register activity nodes, initialising frequencies at zero.
    for et in &ocel.event_types {
        dfg.nodes.push(DFGNode::new(et.name.clone(), 0));
    }

    // Count activity occurrences.
    for event in &ocel.events {
        if let Some(node) = dfg
            .nodes
            .iter_mut()
            .find(|n| n.activity == event.event_type)
        {
            node.frequency += 1;
        }
    }

    // Group event indices by the objects they reference.
    let mut events_by_object: BTreeMap<String, Vec<(i64, &str)>> = BTreeMap::new();
    for event in &ocel.events {
        let ts = event.time.timestamp_millis();
        for rel in &event.relationships {
            events_by_object
                .entry(rel.object_id.clone())
                .or_default()
                .push((ts, event.event_type.as_str()));
        }
    }

    // Sort each object's event list by timestamp.
    for events in events_by_object.values_mut() {
        events.sort_unstable_by_key(|(ts, _)| *ts);
    }

    // Build edge frequency map.
    let mut edge_map: BTreeMap<(&str, &str), usize> = BTreeMap::new();
    for events in events_by_object.values() {
        for pair in events.windows(2) {
            *edge_map.entry((pair[0].1, pair[1].1)).or_default() += 1;
        }
    }

    // Materialise edges (BTreeMap iterates in sorted key order — no extra sort).
    for ((from, to), frequency) in &edge_map {
        dfg.edges
            .push(DFGEdge::new(from.to_string(), to.to_string(), *frequency));
    }

    // Start / end activity sets.
    for events in events_by_object.values() {
        if let Some((_, first)) = events.first() {
            if !dfg.start_activities.contains(&first.to_string()) {
                dfg.start_activities.push(first.to_string());
            }
        }
        if let Some((_, last)) = events.last() {
            if !dfg.end_activities.contains(&last.to_string()) {
                dfg.end_activities.push(last.to_string());
            }
        }
    }

    dfg
}

/// Compute variant traces from an OCEL log.
///
/// Returns a list of activity sequences, one per case object, sorted by
/// timestamp. Duplicate sequences are preserved — use a frequency count
/// over the returned vec to get variant counts.
pub fn extract_ocel_variants(ocel: &OCEL) -> Vec<Vec<String>> {
    let mut events_by_object: BTreeMap<String, Vec<(i64, String)>> = BTreeMap::new();
    for event in &ocel.events {
        let ts = event.time.timestamp_millis();
        for rel in &event.relationships {
            events_by_object
                .entry(rel.object_id.clone())
                .or_default()
                .push((ts, event.event_type.clone()));
        }
    }
    events_by_object
        .into_values()
        .filter(|v| !v.is_empty())
        .map(|mut v| {
            v.sort_unstable_by_key(|(ts, _)| *ts);
            v.into_iter().map(|(_, act)| act).collect()
        })
        .collect()
}

/// Fitness: fraction of normative DFG arcs present in the observed DFG.
pub fn dfg_fitness(observed: &DFG, normative_arcs: &[(String, String)]) -> f64 {
    if normative_arcs.is_empty() {
        return 1.0;
    }
    let present = normative_arcs
        .iter()
        .filter(|(src, tgt)| {
            observed
                .edges
                .iter()
                .any(|e| e.source == *src && e.target == *tgt)
        })
        .count();
    present as f64 / normative_arcs.len() as f64
}

/// Precision: fraction of observed DFG arcs that appear in the normative model.
pub fn dfg_precision(observed: &DFG, normative_arcs: &[(String, String)]) -> f64 {
    if observed.edges.is_empty() {
        return 1.0;
    }
    let normative: std::collections::HashSet<_> = normative_arcs.iter().collect();
    let in_model = observed
        .edges
        .iter()
        .filter(|e| normative.contains(&(e.source.clone(), e.target.clone())))
        .count();
    in_model as f64 / observed.edges.len() as f64
}

// ── Van der Aalst-grounded DFG types (camelCase, OCEL-compatible) ────────────

/// A single node in a Directly-Follows Graph — one activity class.
#[derive(Debug, Clone)]
pub struct DfgNode {
    activity: String,
}

impl DfgNode {
    pub fn new(activity: &str) -> Self {
        DfgNode {
            activity: activity.to_owned(),
        }
    }
    pub fn activity(&self) -> &str {
        &self.activity
    }
}

/// The weight of a DFG edge — observed co-occurrence count.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DfgWeight(pub u64);

impl DfgWeight {
    pub fn count(&self) -> u64 {
        self.0
    }
}

/// A directed edge in a Directly-Follows Graph.
#[derive(Debug, Clone)]
pub struct DfgEdge {
    source: String,
    target: String,
    weight: DfgWeight,
}

impl DfgEdge {
    pub fn new(source: &str, target: &str, count: u64) -> Self {
        DfgEdge {
            source: source.to_owned(),
            target: target.to_owned(),
            weight: DfgWeight(count),
        }
    }
    pub fn source(&self) -> &str {
        &self.source
    }
    pub fn target(&self) -> &str {
        &self.target
    }
    pub fn weight(&self) -> &DfgWeight {
        &self.weight
    }
}

/// A Directly-Follows Graph — the minimal process evidence structure
/// per van der Aalst's process mining theory.
#[derive(Debug, Clone)]
pub struct Dfg {
    nodes: Vec<DfgNode>,
    edges: Vec<DfgEdge>,
}

impl Dfg {
    pub fn new(
        nodes: impl IntoIterator<Item = DfgNode>,
        edges: impl IntoIterator<Item = DfgEdge>,
    ) -> Self {
        Dfg {
            nodes: nodes.into_iter().collect(),
            edges: edges.into_iter().collect(),
        }
    }

    pub fn nodes(&self) -> &[DfgNode] {
        &self.nodes
    }

    pub fn edges(&self) -> &[DfgEdge] {
        &self.edges
    }

    pub fn validate(&self) -> Result<(), DfgRefusal> {
        if self.nodes.is_empty() {
            return Err(DfgRefusal::EmptyGraph);
        }
        let activities: std::collections::HashSet<&str> =
            self.nodes.iter().map(|n| n.activity.as_str()).collect();
        for edge in &self.edges {
            if !activities.contains(edge.source.as_str())
                || !activities.contains(edge.target.as_str())
            {
                return Err(DfgRefusal::DanglingEdge);
            }
        }
        Ok(())
    }
}

/// Named refusal variants for DFG validation laws.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DfgRefusal {
    /// An edge references an activity not present in the node set.
    DanglingEdge,
    /// The DFG has no nodes — an empty graph cannot represent process behaviour.
    EmptyGraph,
}

impl std::fmt::Display for DfgRefusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DfgRefusal::DanglingEdge => write!(f, "DanglingEdge"),
            DfgRefusal::EmptyGraph => write!(f, "EmptyGraph"),
        }
    }
}

impl std::error::Error for DfgRefusal {}

// ── ObjectCentricDfg and DfgEdgeFull ──────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DfgFrequency(pub u64);

#[derive(Debug, Clone)]
pub struct DfgEdgeFull {
    source: String,
    target: String,
    frequency: DfgFrequency,
    duration_ns: Option<u64>,
}

impl DfgEdgeFull {
    pub fn new(source: &str, target: &str, frequency: u64) -> Self {
        DfgEdgeFull {
            source: source.to_owned(),
            target: target.to_owned(),
            frequency: DfgFrequency(frequency),
            duration_ns: None,
        }
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn target(&self) -> &str {
        &self.target
    }

    pub fn frequency(&self) -> DfgFrequency {
        self.frequency
    }

    pub fn duration_ns(&self) -> Option<u64> {
        self.duration_ns
    }

    pub fn with_duration_ns(mut self, ns: u64) -> Self {
        self.duration_ns = Some(ns);
        self
    }
}

use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct ObjectCentricDfg {
    pub per_type: HashMap<String, Dfg>,
    keys: Vec<String>,
}

impl ObjectCentricDfg {
    pub fn new() -> Self {
        ObjectCentricDfg {
            per_type: HashMap::new(),
            keys: Vec::new(),
        }
    }

    pub fn get(&self, object_type: &str) -> Option<&Dfg> {
        self.per_type.get(object_type)
    }

    pub fn with_type_dfg(mut self, object_type: &str, dfg: Dfg) -> Self {
        if !self.per_type.contains_key(object_type) {
            self.keys.push(object_type.to_owned());
        }
        self.per_type.insert(object_type.to_owned(), dfg);
        self
    }

    pub fn object_types(&self) -> impl Iterator<Item = &str> {
        self.keys.iter().map(|s| s.as_str())
    }
}
