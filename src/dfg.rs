pub use crate::models::{DFGEdge, DFGNode, DFG};

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
