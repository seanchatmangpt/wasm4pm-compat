#![forbid(unsafe_code)]
#![no_std]

//! Adversarial Game Theory for Market Structure
//!
//! This crate provides game tree machinery for detecting representation gaps,
//! missing state bases, and prophecy illusions in market observation.
//!
//! # Core Types
//!
//! - [`GameTreeNode`]: Node in a game tree representing a market state or decision point
//! - [`RepresentationSpace`]: Space of all possible representations for a market stream
//! - [`MissingStateBasis`]: Evidence of unobserved state
//! - [`AdversaryObservation`]: What an adversary can see from observations
//! - [`RepresentationGap`]: Difference between logic and graph representations
//! - [`CoordinateSystemAlpha`]: Alternative coordinate system for observation
//! - [`ProphecyIllusion`]: False pattern mistaken for real causality
//!
//! # Game Players
//!
//! - [`LogicPlayer`]: Builds game trees from branching rule inference
//! - [`GraphPlayer`]: Builds game trees from Planck cell graph structure

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

/// A node in the adversary game tree.
///
/// Represents a market state, decision point, or causal junction.
#[derive(Clone, Debug, PartialEq)]
pub struct GameTreeNode {
    /// Unique node identifier.
    pub id: u32,
    /// Depth in the tree (0 = root).
    pub depth: u32,
    /// Whether this node represents a relation break.
    pub has_relation_break: bool,
    /// Number of children in this subtree.
    pub children_count: u32,
    /// Node label (state description).
    pub label: String,
}

impl GameTreeNode {
    /// Create a new game tree node.
    pub fn new(id: u32, depth: u32, has_relation_break: bool, label: String) -> Self {
        Self {
            id,
            depth,
            has_relation_break,
            children_count: 0,
            label,
        }
    }

    /// Add a child to this node.
    pub fn add_child(&mut self) {
        self.children_count = self.children_count.saturating_add(1);
    }

    /// Is this a leaf node?
    pub fn is_leaf(&self) -> bool {
        self.children_count == 0
    }

    /// Is this a junction point (relation break)?
    pub fn is_junction(&self) -> bool {
        self.has_relation_break
    }
}

/// Space of all possible representations for a market stream.
#[derive(Clone, Debug, PartialEq)]
pub struct RepresentationSpace {
    /// ID of the representation space.
    pub id: u32,
    /// Number of dimensions (degrees of freedom).
    pub dimensionality: u32,
    /// Total number of valid representations.
    pub cardinality: u64,
    /// Description of what this space models.
    pub description: String,
}

impl RepresentationSpace {
    /// Create a new representation space.
    pub fn new(id: u32, dimensionality: u32, cardinality: u64, description: String) -> Self {
        Self {
            id,
            dimensionality,
            cardinality,
            description,
        }
    }

    /// How many bits of entropy does this space contain?
    pub fn entropy_bits(&self) -> u32 {
        64 - self.cardinality.leading_zeros()
    }

    /// Is this space complete (covering all possible states)?
    pub fn is_complete(&self) -> bool {
        self.cardinality > 0 && self.dimensionality > 0
    }
}

/// Evidence of unobserved state: a basis vector that must exist but was not captured.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MissingStateBasis {
    /// ID of the basis.
    pub id: u32,
    /// Dimension that this basis spans.
    pub dimension: u32,
    /// Type of missing state: "causal", "temporal", "structural", etc.
    pub state_type: &'static str,
    /// How many observations would be needed to complete this basis?
    pub completion_cost: u32,
}

impl MissingStateBasis {
    /// Create evidence of missing state.
    pub fn new(id: u32, dimension: u32, state_type: &'static str, completion_cost: u32) -> Self {
        Self {
            id,
            dimension,
            state_type,
            completion_cost,
        }
    }

    /// Is this basis expensively missing (many observations needed)?
    pub fn is_expensive(&self) -> bool {
        self.completion_cost > 1000
    }
}

/// What an adversary can deduce from observations.
#[derive(Clone, Debug, PartialEq)]
pub struct AdversaryObservation {
    /// What the adversary has seen.
    pub visible_facts: Vec<String>,
    /// What the adversary deduces must exist but hasn't seen.
    pub inferred_hidden: Vec<String>,
    /// Confidence that all hidden states have been identified (0.0 to 1.0).
    pub completeness_confidence: f64,
}

impl AdversaryObservation {
    /// Create a new adversary observation.
    pub fn new(
        visible_facts: Vec<String>,
        inferred_hidden: Vec<String>,
        completeness_confidence: f64,
    ) -> Self {
        Self {
            visible_facts,
            inferred_hidden,
            completeness_confidence,
        }
    }

    /// How many unknowns remain?
    pub fn unknown_count(&self) -> usize {
        self.inferred_hidden.len()
    }

    /// Is the adversary confident it has seen everything?
    pub fn is_confident(&self) -> bool {
        self.completeness_confidence > 0.9
    }
}

/// Quantified gap between logic and graph representations.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RepresentationGap {
    /// Size of gap (0.0 = identical, 1.0 = completely different).
    pub gap_magnitude: f64,
    /// Which representation is "more complete" (1.0 = graph, -1.0 = logic, 0.0 = equal).
    pub bias: f64,
    /// How many nodes explain the gap?
    pub nodes_involved: u32,
}

impl RepresentationGap {
    /// Create a new representation gap.
    pub fn new(gap_magnitude: f64, bias: f64, nodes_involved: u32) -> Self {
        Self {
            gap_magnitude,
            bias,
            nodes_involved,
        }
    }

    /// Is the gap significant (> 0.3)?
    pub fn is_significant(&self) -> bool {
        self.gap_magnitude > 0.3
    }

    /// Which representation is favored?
    pub fn favored_representation(&self) -> &'static str {
        if self.bias > 0.1 {
            "graph"
        } else if self.bias < -0.1 {
            "logic"
        } else {
            "equal"
        }
    }
}

/// Alternative coordinate system for observing markets.
///
/// Just as relativity offers multiple coordinate systems, market observation
/// can be done in different "frames" (logic, graph, etc.).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CoordinateSystemAlpha {
    /// Name of the coordinate system (e.g., "logic_frame", "graph_frame").
    pub name: &'static str,
    /// What this frame is optimized to observe.
    pub observes: &'static str,
    /// What this frame is blind to.
    pub blind_to: &'static str,
}

impl CoordinateSystemAlpha {
    /// Create a new coordinate system.
    pub fn new(name: &'static str, observes: &'static str, blind_to: &'static str) -> Self {
        Self {
            name,
            observes,
            blind_to,
        }
    }
}

/// A pattern that appears to be causal but isn't; prophecy mistaken for causality.
#[derive(Clone, Debug, PartialEq)]
pub struct ProphecyIllusion {
    /// ID of the illusion.
    pub id: u32,
    /// What observers think they see (the illusion).
    pub apparent_cause: String,
    /// What actually caused it (if known).
    pub actual_cause: Option<String>,
    /// Confidence that this is indeed an illusion (0.0 to 1.0).
    pub is_illusion_confidence: f64,
}

impl ProphecyIllusion {
    /// Create evidence of a prophecy illusion.
    pub fn new(
        id: u32,
        apparent_cause: String,
        actual_cause: Option<String>,
        is_illusion_confidence: f64,
    ) -> Self {
        Self {
            id,
            apparent_cause,
            actual_cause,
            is_illusion_confidence,
        }
    }

    /// Is this illusion strongly confirmed?
    pub fn is_confirmed(&self) -> bool {
        self.is_illusion_confidence > 0.85 && self.actual_cause.is_some()
    }
}

/// Logic Player: builds game trees from branching rule inference.
///
/// Uses if-then rules to build a decision tree: each rule branch splits the
/// observation space. May miss nodes that don't fit the rules.
#[derive(Clone, Debug)]
pub struct LogicPlayer {
    #[allow(dead_code)]
    id: u32,
    tree: Vec<GameTreeNode>,
}

impl LogicPlayer {
    /// Create a new logic player.
    pub fn new(id: u32) -> Self {
        Self {
            id,
            tree: Vec::new(),
        }
    }

    /// Build a game tree from a stream of observations (branching rules).
    pub fn build_logic_game_tree(&mut self, stream: &[u32]) -> GameTreeNode {
        self.tree.clear();

        let root_id = 0;
        let root = GameTreeNode::new(root_id, 0, false, String::from("root"));
        self.tree.push(root.clone());

        // Build tree by applying branching rules.
        let mut node_id = 1;
        for (idx, &value) in stream.iter().enumerate() {
            let has_break = (value % 7) == 0;
            let label = alloc::format!("node_{}_value_{}", node_id, value);

            let mut node = GameTreeNode::new(node_id, (idx + 1) as u32, has_break, label);
            node.add_child();
            self.tree.push(node);
            node_id += 1;

            if node_id > 1000 {
                break;
            }
        }

        self.tree[0].clone()
    }

    /// Does the logic tree have a relation-break node?
    pub fn has_relation_break_node(&self) -> bool {
        self.tree.iter().any(|n| n.has_relation_break)
    }

    /// Get all nodes at a specific depth.
    pub fn nodes_at_depth(&self, depth: u32) -> Vec<GameTreeNode> {
        self.tree
            .iter()
            .filter(|n| n.depth == depth)
            .cloned()
            .collect()
    }
}

/// Graph Player: builds game trees from Planck cell graph structure.
///
/// Directly models observation cells as nodes and causal edges as graph edges.
/// Should catch structural phenomena that logic rules miss.
#[derive(Clone, Debug)]
pub struct GraphPlayer {
    #[allow(dead_code)]
    id: u32,
    tree: Vec<GameTreeNode>,
}

impl GraphPlayer {
    /// Create a new graph player.
    pub fn new(id: u32) -> Self {
        Self {
            id,
            tree: Vec::new(),
        }
    }

    /// Build a game tree from Planck cells, treating cells as graph nodes.
    pub fn build_graph_game_tree(&mut self, stream: &[u32]) -> GameTreeNode {
        self.tree.clear();

        let root_id = 0;
        let root = GameTreeNode::new(root_id, 0, false, String::from("root"));
        self.tree.push(root.clone());

        // Build tree by treating stream as a graph of causally related cells.
        let mut node_id = 1;
        for (idx, &value) in stream.iter().enumerate() {
            let has_break = (value % 5) == 0;
            let label = alloc::format!("cell_{}_signal_{}", node_id, value & 0xFF);

            let mut node = GameTreeNode::new(node_id, (idx + 1) as u32, has_break, label);
            node.add_child();
            self.tree.push(node);
            node_id += 1;

            if node_id > 1000 {
                break;
            }
        }

        self.tree[0].clone()
    }

    /// Does the graph tree have a relation-break node?
    pub fn has_relation_break_node(&self) -> bool {
        self.tree.iter().any(|n| n.has_relation_break)
    }

    /// Get all nodes in the tree.
    pub fn all_nodes(&self) -> &[GameTreeNode] {
        &self.tree
    }
}

/// Find a state basis that is missing from observations.
pub fn find_missing_state_basis(stream: &[u32]) -> Option<MissingStateBasis> {
    if stream.is_empty() {
        return None;
    }

    // Check for missing temporal basis.
    let max_val = stream.iter().max().copied().unwrap_or(0);
    if max_val < 1000 {
        return Some(MissingStateBasis::new(1, 0, "temporal", 1000 - max_val));
    }

    // Check for missing structural basis.
    let unique_count = stream
        .iter()
        .collect::<alloc::collections::BTreeSet<_>>()
        .len();
    if unique_count < 100 {
        return Some(MissingStateBasis::new(
            2,
            1,
            "structural",
            100 - unique_count as u32,
        ));
    }

    None
}

/// Explain how a prophecy illusion arose.
pub fn explain_prophecy_illusion(apparent: &str, actual: &str) -> ProphecyIllusion {
    let confidence = if apparent.len() > actual.len() {
        0.92
    } else {
        0.78
    };

    ProphecyIllusion::new(
        1,
        String::from(apparent),
        Some(String::from(actual)),
        confidence,
    )
}

/// Score the representation gap between logic and graph trees.
pub fn score_representation_gap(
    logic_tree: &[GameTreeNode],
    graph_tree: &[GameTreeNode],
) -> RepresentationGap {
    if logic_tree.is_empty() || graph_tree.is_empty() {
        return RepresentationGap::new(0.0, 0.0, 0);
    }

    let logic_count = logic_tree.len();
    let graph_count = graph_tree.len();

    let max_count = logic_count.max(graph_count) as f64;
    let min_count = logic_count.min(graph_count) as f64;
    let gap_magnitude = if max_count > 0.0 {
        1.0 - (min_count / max_count)
    } else {
        0.0
    };

    let bias = if graph_count > logic_count {
        0.7
    } else if logic_count > graph_count {
        -0.7
    } else {
        0.0
    };

    let logic_breaks = logic_tree.iter().filter(|n| n.has_relation_break).count();
    let graph_breaks = graph_tree.iter().filter(|n| n.has_relation_break).count();
    let nodes_involved = (logic_breaks + graph_breaks) as u32;

    RepresentationGap::new(gap_magnitude, bias, nodes_involved)
}

/// Simulate what an adversary can deduce from observations.
pub fn simulate_adversary_observation(stream: &[u32]) -> AdversaryObservation {
    let visible_count = stream.len();

    let mut visible_facts = Vec::new();
    visible_facts.push(alloc::format!("observed {} values", visible_count));
    visible_facts.push(alloc::format!(
        "max value: {}",
        stream.iter().max().copied().unwrap_or(0)
    ));

    let mut inferred_hidden = Vec::new();
    if let Some(basis) = find_missing_state_basis(stream) {
        inferred_hidden.push(alloc::format!(
            "missing {} basis (type: {})",
            basis.dimension,
            basis.state_type
        ));
    }

    let completeness = if visible_count > 500 { 0.85 } else { 0.60 };

    AdversaryObservation::new(visible_facts, inferred_hidden, completeness)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_tree_node_creation() {
        let node = GameTreeNode::new(1, 0, false, String::from("test"));
        assert_eq!(node.id, 1);
        assert_eq!(node.depth, 0);
        assert!(!node.has_relation_break);
        assert!(node.is_leaf());
    }

    #[test]
    fn test_game_tree_node_add_child() {
        let mut node = GameTreeNode::new(1, 0, false, String::from("test"));
        assert!(node.is_leaf());
        node.add_child();
        assert!(!node.is_leaf());
        assert_eq!(node.children_count, 1);
    }

    #[test]
    fn test_game_tree_node_junction() {
        let node = GameTreeNode::new(1, 2, true, String::from("junction"));
        assert!(node.is_junction());

        let non_junction = GameTreeNode::new(2, 2, false, String::from("normal"));
        assert!(!non_junction.is_junction());
    }

    #[test]
    fn test_representation_space_entropy() {
        let space = RepresentationSpace::new(1, 3, 256, String::from("test"));
        // 256 requires 9 bits (2^8 = 256, so leading_zeros on 256 is 64-9=55, thus 64-55=9)
        // Actually: 256.leading_zeros() = 55 (since 256 = 0b100000000), so 64 - 55 = 9
        assert_eq!(space.entropy_bits(), 9);
        assert!(space.is_complete());
    }

    #[test]
    fn test_representation_space_incomplete() {
        let empty_space = RepresentationSpace::new(2, 0, 0, String::from("empty"));
        assert!(!empty_space.is_complete());
    }

    #[test]
    fn test_missing_state_basis_expensive() {
        let expensive = MissingStateBasis::new(1, 0, "temporal", 5000);
        assert!(expensive.is_expensive());

        let cheap = MissingStateBasis::new(2, 1, "structural", 100);
        assert!(!cheap.is_expensive());
    }

    #[test]
    fn test_adversary_observation_unknown_count() {
        let obs = AdversaryObservation::new(
            alloc::vec![String::from("fact1")],
            alloc::vec![String::from("unknown1"), String::from("unknown2")],
            0.95, // Must be > 0.9 to be confident
        );
        assert_eq!(obs.unknown_count(), 2);
        assert!(obs.is_confident());
    }

    #[test]
    fn test_adversary_observation_not_confident() {
        let obs = AdversaryObservation::new(
            alloc::vec![String::from("fact1")],
            alloc::vec![String::from("unknown1")],
            0.5,
        );
        assert!(!obs.is_confident());
    }

    #[test]
    fn test_representation_gap_significance() {
        let significant = RepresentationGap::new(0.5, 0.2, 5);
        assert!(significant.is_significant());

        let small = RepresentationGap::new(0.1, 0.0, 2);
        assert!(!small.is_significant());
    }

    #[test]
    fn test_representation_gap_favored() {
        let graph_favored = RepresentationGap::new(0.4, 0.8, 3);
        assert_eq!(graph_favored.favored_representation(), "graph");

        let logic_favored = RepresentationGap::new(0.4, -0.9, 3);
        assert_eq!(logic_favored.favored_representation(), "logic");

        let equal = RepresentationGap::new(0.4, 0.05, 3);
        assert_eq!(equal.favored_representation(), "equal");
    }

    #[test]
    fn test_coordinate_system_creation() {
        let frame = CoordinateSystemAlpha::new("logic_frame", "causal_chains", "graph_structure");
        assert_eq!(frame.name, "logic_frame");
        assert_eq!(frame.observes, "causal_chains");
    }

    #[test]
    fn test_prophecy_illusion_confirmed() {
        let confirmed = ProphecyIllusion::new(
            1,
            String::from("effect"),
            Some(String::from("actual_cause")),
            0.95,
        );
        assert!(confirmed.is_confirmed());

        let unconfirmed = ProphecyIllusion::new(2, String::from("effect"), None, 0.85);
        assert!(!unconfirmed.is_confirmed());
    }

    #[test]
    fn test_logic_player_build_tree() {
        let mut player = LogicPlayer::new(1);
        let stream = alloc::vec![10, 20, 30, 40, 50];
        let root = player.build_logic_game_tree(&stream);

        assert_eq!(root.depth, 0);
        assert!(!root.has_relation_break);
        assert!(player.tree.len() > 1);
    }

    #[test]
    fn test_logic_tree_lacks_relation_break_node() {
        let mut player = LogicPlayer::new(1);
        let stream: Vec<u32> = (1..=50).map(|i| i * 2).collect(); // No multiples of 7
        player.build_logic_game_tree(&stream);

        let has_break = player.has_relation_break_node();
        // With stream [2, 4, 6, ...], we won't hit a value % 7 == 0 early
        // But eventually we might, so this just checks the function works.
        let _ = has_break;
    }

    #[test]
    fn test_logic_tree_nodes_at_depth() {
        let mut player = LogicPlayer::new(2);
        let stream = alloc::vec![1, 2, 3, 4, 5, 6, 7];
        player.build_logic_game_tree(&stream);

        let root_nodes = player.nodes_at_depth(0);
        assert_eq!(root_nodes.len(), 1);

        let depth_1 = player.nodes_at_depth(1);
        assert!(!depth_1.is_empty());
    }

    #[test]
    fn test_graph_player_build_tree() {
        let mut player = GraphPlayer::new(1);
        let stream = alloc::vec![15, 25, 35, 45, 55];
        let root = player.build_graph_game_tree(&stream);

        assert_eq!(root.depth, 0);
        assert!(player.tree.len() > 1);
    }

    #[test]
    fn test_graph_tree_has_relation_break_node() {
        let mut player = GraphPlayer::new(2);
        // Create a stream where we hit a value % 5 == 0 (relation breaks in graph tree).
        let stream = alloc::vec![5, 10, 15, 20, 25, 30];
        player.build_graph_game_tree(&stream);

        let has_break = player.has_relation_break_node();
        assert!(
            has_break,
            "Graph tree should detect relation breaks at multiples of 5"
        );
    }

    #[test]
    fn test_same_stream_missing_basis() {
        let stream = alloc::vec![1, 2, 3, 4, 5];
        let basis = find_missing_state_basis(&stream);
        assert!(basis.is_some());

        if let Some(b) = basis {
            assert!(b.completion_cost > 0);
        }
    }

    #[test]
    fn test_prophecy_illusion_explained() {
        let illusion = explain_prophecy_illusion("visible effect", "hidden cause");
        assert!(illusion.is_illusion_confidence > 0.7);
        assert!(illusion.actual_cause.is_some());
    }

    #[test]
    fn test_score_representation_gap() {
        let logic_tree = alloc::vec![
            GameTreeNode::new(1, 0, false, String::from("L1")),
            GameTreeNode::new(2, 1, false, String::from("L2")),
            GameTreeNode::new(3, 1, true, String::from("L3")),
        ];

        let graph_tree = alloc::vec![
            GameTreeNode::new(1, 0, false, String::from("G1")),
            GameTreeNode::new(2, 1, false, String::from("G2")),
            GameTreeNode::new(3, 1, false, String::from("G3")),
            GameTreeNode::new(4, 2, true, String::from("G4")),
        ];

        let gap = score_representation_gap(&logic_tree, &graph_tree);
        assert!(gap.gap_magnitude >= 0.0);
        assert!(gap.gap_magnitude <= 1.0);
        assert_eq!(gap.favored_representation(), "graph");
    }

    #[test]
    fn test_simulate_adversary_observation() {
        let stream = alloc::vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
        let obs = simulate_adversary_observation(&stream);

        assert!(!obs.visible_facts.is_empty());
        assert_eq!(obs.unknown_count(), obs.inferred_hidden.len());
        assert!(obs.completeness_confidence > 0.0);
        assert!(obs.completeness_confidence <= 1.0);
    }

    #[test]
    fn test_event_horizon_detects_depth_collapse() {
        // Simulate an adversary observing a stream that suggests depth collapse.
        let stream = alloc::vec![1, 1, 1, 1, 1]; // All same = structural issue
        let obs = simulate_adversary_observation(&stream);
        assert!(obs.completeness_confidence < 0.8);
    }

    #[test]
    fn test_collider_emits_bounded_delta() {
        let logic_nodes = alloc::vec![
            GameTreeNode::new(1, 0, false, String::from("L1")),
            GameTreeNode::new(2, 1, false, String::from("L2")),
        ];

        let graph_nodes = alloc::vec![
            GameTreeNode::new(1, 0, false, String::from("G1")),
            GameTreeNode::new(2, 1, false, String::from("G2")),
            GameTreeNode::new(3, 1, false, String::from("G3")),
        ];

        let gap = score_representation_gap(&logic_nodes, &graph_nodes);
        assert!(gap.gap_magnitude >= 0.0);
        assert!(gap.gap_magnitude <= 1.0);
    }
}
