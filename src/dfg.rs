pub use crate::models::{DFG, DFGNode, DFGEdge};

// ── Van der Aalst-grounded DFG types (camelCase, OCEL-compatible) ────────────

/// A single node in a Directly-Follows Graph — one activity class.
#[derive(Debug, Clone)]
pub struct DfgNode {
    activity: String,
}

impl DfgNode {
    pub fn new(activity: &str) -> Self { DfgNode { activity: activity.to_owned() } }
    pub fn activity(&self) -> &str { &self.activity }
}

/// The weight of a DFG edge — observed co-occurrence count.
#[derive(Debug, Clone)]
pub struct DfgWeight {
    count: u32,
}

impl DfgWeight {
    pub fn count(&self) -> u32 { self.count }
}

/// A directed edge in a Directly-Follows Graph.
#[derive(Debug, Clone)]
pub struct DfgEdge {
    source: String,
    target: String,
    weight: DfgWeight,
}

impl DfgEdge {
    pub fn new(source: &str, target: &str, count: u32) -> Self {
        DfgEdge {
            source: source.to_owned(),
            target: target.to_owned(),
            weight: DfgWeight { count },
        }
    }
    pub fn source(&self) -> &str { &self.source }
    pub fn target(&self) -> &str { &self.target }
    pub fn weight(&self) -> &DfgWeight { &self.weight }
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
        Dfg { nodes: nodes.into_iter().collect(), edges: edges.into_iter().collect() }
    }

    pub fn nodes(&self) -> &[DfgNode] { &self.nodes }
    pub fn edges(&self) -> &[DfgEdge] { &self.edges }

    #[must_use]
    pub fn validate(&self) -> Result<(), DfgRefusal> {
        if self.nodes.is_empty() {
            return Err(DfgRefusal::EmptyGraph);
        }
        let activities: std::collections::HashSet<&str> =
            self.nodes.iter().map(|n| n.activity.as_str()).collect();
        for edge in &self.edges {
            if !activities.contains(edge.source.as_str()) || !activities.contains(edge.target.as_str()) {
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
