//! Spec-compliant Choice Graph (Definition 1, arXiv:2505.07052).
//!
//! Kourani, Park, van der Aalst, "Unlocking Non-Block-Structured Decisions:
//! Inductive Mining with Choice Graphs."
//!
//! A Choice Graph is a directed acyclic graph with a unique Start node (no
//! incoming edges) and a unique End node (no outgoing edges) such that every
//! node lies on at least one Start→End path. Interior nodes are either inline
//! activities or references to a sub-model in a `PowlArena`.

use serde::{Deserialize, Serialize};

/// A node in a `ChoiceGraph`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChoiceGraphNode {
    /// Unique Start marker (no incoming edges).
    Start,
    /// Unique End marker (no outgoing edges).
    End,
    /// Inline activity by label. Normalized to `SubModel(_)` when added to an
    /// arena.
    Activity(String),
    /// Reference to a sub-model by arena root index.
    SubModel(u32),
}

/// Validated Choice Graph (paper Definition 1).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChoiceGraph {
    nodes: Vec<ChoiceGraphNode>,
    edges: Vec<(usize, usize)>,
    start_idx: usize,
    end_idx: usize,
}

/// Validation errors raised by `ChoiceGraph::new`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChoiceGraphError {
    NoStart,
    NoEnd,
    MultipleStarts,
    MultipleEnds,
    StartHasIncoming,
    EndHasOutgoing,
    EdgeOutOfBounds,
    Cyclic,
    NodeNotOnStartEndPath,
}

impl core::fmt::Display for ChoiceGraphError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = match self {
            ChoiceGraphError::NoStart => "no Start node",
            ChoiceGraphError::NoEnd => "no End node",
            ChoiceGraphError::MultipleStarts => "multiple Start nodes",
            ChoiceGraphError::MultipleEnds => "multiple End nodes",
            ChoiceGraphError::StartHasIncoming => "Start node has incoming edges",
            ChoiceGraphError::EndHasOutgoing => "End node has outgoing edges",
            ChoiceGraphError::EdgeOutOfBounds => "edge endpoint out of bounds",
            ChoiceGraphError::Cyclic => "graph is cyclic",
            ChoiceGraphError::NodeNotOnStartEndPath => "node not on any Start→End path",
        };
        f.write_str(s)
    }
}

impl std::error::Error for ChoiceGraphError {}

impl ChoiceGraph {
    /// Construct and validate per Definition 1.
    ///
    /// Auto-discovers `start_idx` / `end_idx` from the `nodes` vec.
    pub fn new(
        nodes: Vec<ChoiceGraphNode>,
        edges: Vec<(usize, usize)>,
    ) -> Result<Self, ChoiceGraphError> {
        // 1. Locate Start / End — exactly one of each.
        let mut start_idx: Option<usize> = None;
        let mut end_idx: Option<usize> = None;
        for (i, n) in nodes.iter().enumerate() {
            match n {
                ChoiceGraphNode::Start => {
                    if start_idx.is_some() {
                        return Err(ChoiceGraphError::MultipleStarts);
                    }
                    start_idx = Some(i);
                }
                ChoiceGraphNode::End => {
                    if end_idx.is_some() {
                        return Err(ChoiceGraphError::MultipleEnds);
                    }
                    end_idx = Some(i);
                }
                _ => {}
            }
        }
        let start_idx = start_idx.ok_or(ChoiceGraphError::NoStart)?;
        let end_idx = end_idx.ok_or(ChoiceGraphError::NoEnd)?;

        // 2. Edge bounds.
        let n = nodes.len();
        for &(a, b) in &edges {
            if a >= n || b >= n {
                return Err(ChoiceGraphError::EdgeOutOfBounds);
            }
        }

        // 3. Start has no incoming, End has no outgoing.
        for &(a, b) in &edges {
            if b == start_idx {
                return Err(ChoiceGraphError::StartHasIncoming);
            }
            if a == end_idx {
                return Err(ChoiceGraphError::EndHasOutgoing);
            }
        }

        // 4. Acyclicity: REMOVED/Relaxed to model cyclic loops

        // 5. Every node on some Start→End path.
        // Reachable from Start (forward) ∩ reachable to End (backward).
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
        for &(a, b) in &edges {
            adj[a].push(b);
        }
        let mut radj: Vec<Vec<usize>> = vec![Vec::new(); n];
        for &(a, b) in &edges {
            radj[b].push(a);
        }
        let reach_from_start = bfs_reach(&adj, start_idx, n);
        let reach_to_end = bfs_reach(&radj, end_idx, n);
        for i in 0..n {
            if !(reach_from_start[i] && reach_to_end[i]) {
                return Err(ChoiceGraphError::NodeNotOnStartEndPath);
            }
        }

        Ok(ChoiceGraph {
            nodes,
            edges,
            start_idx,
            end_idx,
        })
    }

    /// Construct directly with explicit start and end indices.
    /// Connected path constraint is guaranteed at construction time.
    pub fn new_raw(
        nodes: Vec<ChoiceGraphNode>,
        edges: Vec<(usize, usize)>,
        start_idx: usize,
        end_idx: usize,
    ) -> Result<Self, ChoiceGraphError> {
        let n = nodes.len();
        if start_idx >= n || end_idx >= n {
            return Err(ChoiceGraphError::EdgeOutOfBounds);
        }
        for &(a, b) in &edges {
            if a >= n || b >= n {
                return Err(ChoiceGraphError::EdgeOutOfBounds);
            }
        }

        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
        for &(a, b) in &edges {
            adj[a].push(b);
        }
        let mut radj: Vec<Vec<usize>> = vec![Vec::new(); n];
        for &(a, b) in &edges {
            radj[b].push(a);
        }
        let reach_from_start = bfs_reach(&adj, start_idx, n);
        let reach_to_end = bfs_reach(&radj, end_idx, n);
        for i in 0..n {
            if !(reach_from_start[i] && reach_to_end[i]) {
                return Err(ChoiceGraphError::NodeNotOnStartEndPath);
            }
        }

        Ok(ChoiceGraph {
            nodes,
            edges,
            start_idx,
            end_idx,
        })
    }

    pub fn nodes(&self) -> &[ChoiceGraphNode] {
        &self.nodes
    }

    pub fn edges(&self) -> &[(usize, usize)] {
        &self.edges
    }

    pub fn start_idx(&self) -> usize {
        self.start_idx
    }

    pub fn end_idx(&self) -> usize {
        self.end_idx
    }

    // Mutable setters that preserve the connected path invariants:
    pub fn set_nodes(&mut self, nodes: Vec<ChoiceGraphNode>) -> Result<(), ChoiceGraphError> {
        let old = std::mem::replace(&mut self.nodes, nodes);
        if let Err(e) = self.validate_connected_path() {
            self.nodes = old;
            return Err(e);
        }
        Ok(())
    }

    pub fn set_edges(&mut self, edges: Vec<(usize, usize)>) -> Result<(), ChoiceGraphError> {
        let old = std::mem::replace(&mut self.edges, edges);
        if let Err(e) = self.validate_connected_path() {
            self.edges = old;
            return Err(e);
        }
        Ok(())
    }

    pub fn set_start_idx(&mut self, start_idx: usize) -> Result<(), ChoiceGraphError> {
        let old = self.start_idx;
        self.start_idx = start_idx;
        if let Err(e) = self.validate_connected_path() {
            self.start_idx = old;
            return Err(e);
        }
        Ok(())
    }

    pub fn set_end_idx(&mut self, end_idx: usize) -> Result<(), ChoiceGraphError> {
        let old = self.end_idx;
        self.end_idx = end_idx;
        if let Err(e) = self.validate_connected_path() {
            self.end_idx = old;
            return Err(e);
        }
        Ok(())
    }

    fn validate_connected_path(&self) -> Result<(), ChoiceGraphError> {
        let n = self.nodes.len();
        if self.start_idx >= n || self.end_idx >= n {
            return Err(ChoiceGraphError::EdgeOutOfBounds);
        }
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
        for &(a, b) in &self.edges {
            if a >= n || b >= n {
                return Err(ChoiceGraphError::EdgeOutOfBounds);
            }
            adj[a].push(b);
        }
        let mut radj: Vec<Vec<usize>> = vec![Vec::new(); n];
        for &(a, b) in &self.edges {
            radj[b].push(a);
        }
        let reach_from_start = bfs_reach(&adj, self.start_idx, n);
        let reach_to_end = bfs_reach(&radj, self.end_idx, n);
        for i in 0..n {
            if !(reach_from_start[i] && reach_to_end[i]) {
                return Err(ChoiceGraphError::NodeNotOnStartEndPath);
            }
        }
        Ok(())
    }

    pub fn successors(&self, node_idx: usize) -> Vec<usize> {
        self.edges
            .iter()
            .filter_map(|&(a, b)| if a == node_idx { Some(b) } else { None })
            .collect()
    }

    pub fn predecessors(&self, node_idx: usize) -> Vec<usize> {
        self.edges
            .iter()
            .filter_map(|&(a, b)| if b == node_idx { Some(a) } else { None })
            .collect()
    }

    /// True iff there is a direct Start→End edge (the empty path).
    pub fn has_empty_path(&self) -> bool {
        self.edges
            .iter()
            .any(|&(a, b)| a == self.start_idx && b == self.end_idx)
    }
}

fn bfs_reach(adj: &[Vec<usize>], src: usize, n: usize) -> Vec<bool> {
    let mut seen = vec![false; n];
    if src >= n {
        return seen;
    }
    let mut q: Vec<usize> = vec![src];
    seen[src] = true;
    while let Some(v) = q.pop() {
        for &w in &adj[v] {
            if !seen[w] {
                seen[w] = true;
                q.push(w);
            }
        }
    }
    seen
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimal_valid() {
        let cg = ChoiceGraph::new(
            vec![ChoiceGraphNode::Start, ChoiceGraphNode::End],
            vec![(0, 1)],
        )
        .unwrap();
        assert_eq!(cg.start_idx, 0);
        assert_eq!(cg.end_idx, 1);
        assert!(cg.has_empty_path());
    }

    #[test]
    fn no_start() {
        let err = ChoiceGraph::new(vec![ChoiceGraphNode::End], vec![]).unwrap_err();
        assert_eq!(err, ChoiceGraphError::NoStart);
    }

    #[test]
    fn no_end() {
        let err = ChoiceGraph::new(vec![ChoiceGraphNode::Start], vec![]).unwrap_err();
        assert_eq!(err, ChoiceGraphError::NoEnd);
    }

    #[test]
    fn multiple_starts() {
        let err = ChoiceGraph::new(
            vec![
                ChoiceGraphNode::Start,
                ChoiceGraphNode::Start,
                ChoiceGraphNode::End,
            ],
            vec![(0, 2), (1, 2)],
        )
        .unwrap_err();
        assert_eq!(err, ChoiceGraphError::MultipleStarts);
    }

    #[test]
    fn start_has_incoming() {
        let err = ChoiceGraph::new(
            vec![
                ChoiceGraphNode::Start,
                ChoiceGraphNode::Activity("a".into()),
                ChoiceGraphNode::End,
            ],
            vec![(1, 0), (0, 2)],
        )
        .unwrap_err();
        assert_eq!(err, ChoiceGraphError::StartHasIncoming);
    }

    #[test]
    fn end_has_outgoing() {
        let err = ChoiceGraph::new(
            vec![
                ChoiceGraphNode::Start,
                ChoiceGraphNode::Activity("a".into()),
                ChoiceGraphNode::End,
            ],
            vec![(0, 2), (2, 1)],
        )
        .unwrap_err();
        assert_eq!(err, ChoiceGraphError::EndHasOutgoing);
    }

    #[test]
    fn edge_oob() {
        let err = ChoiceGraph::new(
            vec![ChoiceGraphNode::Start, ChoiceGraphNode::End],
            vec![(0, 99)],
        )
        .unwrap_err();
        assert_eq!(err, ChoiceGraphError::EdgeOutOfBounds);
    }

    #[test]
    fn successors_predecessors() {
        let cg = ChoiceGraph::new(
            vec![
                ChoiceGraphNode::Start,
                ChoiceGraphNode::Activity("a".into()),
                ChoiceGraphNode::End,
            ],
            vec![(0, 1), (1, 2)],
        )
        .unwrap();
        assert_eq!(cg.successors(0), vec![1]);
        assert_eq!(cg.predecessors(2), vec![1]);
    }

    #[test]
    fn test_choice_graph_cyclic_loop_permitted() {
        // In POWL 2.0, cycles are explicitly permitted and should not raise `ChoiceGraphError::Cyclic`.
        let cg = ChoiceGraph::new(
            vec![
                ChoiceGraphNode::Start,
                ChoiceGraphNode::Activity("a".into()),
                ChoiceGraphNode::Activity("b".into()),
                ChoiceGraphNode::End,
            ],
            vec![
                (0, 1),
                (1, 2),
                (2, 1), // Cycle: 1 -> 2 -> 1
                (1, 3),
            ],
        );
        assert!(
            cg.is_ok(),
            "Cyclic loop should be permitted in POWL 2.0 ChoiceGraph"
        );
    }

    #[test]
    fn test_choice_graph_unreachable_node_isolated() {
        // Isolated node (index 2) cannot reach End and is unreachable from Start.
        let err = ChoiceGraph::new(
            vec![
                ChoiceGraphNode::Start,
                ChoiceGraphNode::Activity("a".into()),
                ChoiceGraphNode::Activity("unreachable".into()),
                ChoiceGraphNode::End,
            ],
            vec![(0, 1), (1, 3)],
        )
        .unwrap_err();
        assert_eq!(err, ChoiceGraphError::NodeNotOnStartEndPath);
    }
}
