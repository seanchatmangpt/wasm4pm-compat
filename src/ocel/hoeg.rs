//! Heterogeneous Object Event Graph (HOEG) mapping.
//!
//! Based on Smit et al. (2024) "HOEG: A New Approach for Object-Centric
//! Predictive Process Monitoring".

extern crate alloc;

use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use hashbrown::HashMap;

/// A Semantic Node Type in the HOEG ($NT$).
/// Must include at least an `event` node and one object type.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeType(pub String);

/// An Edge Type in the HOEG ($ET$).
/// Described by a semantic triple: `(subject, predicate, object)`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EdgeType {
    pub subject: NodeType,
    pub predicate: String,
    pub object: NodeType,
}

/// A feature matrix ($X_i$) associated with a specific Node Type.
#[derive(Debug, Clone)]
pub struct FeatureMatrix {
    /// Number of features per node.
    pub dimensions: usize,
    /// Flattened matrix of features for the nodes of this type.
    pub data: Vec<f32>,
}

/// An Adjacency matrix ($A_i$) representing connections between nodes
/// based on a specific Edge Type ($ET_i$).
#[derive(Debug, Clone)]
pub struct AdjacencyMatrix {
    /// List of coordinate pairs (source_idx, target_idx).
    pub edges: Vec<(usize, usize)>,
}

/// The Heterogeneous Object Event Graph ($HOEG$).
///
/// See Definition 5 (Smit et al., 2024). This execution graph composes
/// related events and objects into heterogeneous matrices designed for GNN architectures.
#[derive(Debug, Clone)]
pub struct HoegGraph {
    /// $NT$: Set of node types.
    pub node_types: Vec<NodeType>,
    /// $ET$: Set of semantic edge types.
    pub edge_types: Vec<EdgeType>,
    /// $X$: Feature matrices mapped via `fnt_lookup`.
    pub feature_matrices: HashMap<NodeType, FeatureMatrix>,
    /// $A$: Adjacency matrices mapped via `feta_lookup`.
    pub adjacency_matrices: HashMap<EdgeType, AdjacencyMatrix>,
}

impl HoegGraph {
    /// Initializes an empty HOEG boundary ready for multi-dimensional extraction.
    pub fn new() -> Self {
        Self {
            node_types: Vec::new(),
            edge_types: Vec::new(),
            feature_matrices: HashMap::new(),
            adjacency_matrices: HashMap::new(),
        }
    }

    /// Extract HOEG matrices from an OcelLog.
    pub fn extract(log: &crate::ocel::OcelLog) -> Self {
        let mut node_types = std::collections::HashSet::new();
        let event_nt = NodeType("event".to_string());
        node_types.insert(event_nt.clone());
        
        let mut object_indices = HashMap::new();
        let mut event_indices = HashMap::new();

        let mut event_features = Vec::new();
        for (i, ev) in log.events().iter().enumerate() {
            event_indices.insert(ev.id().to_string(), i);
            event_features.push(ev.attributes().len() as f32); 
        }

        let mut obj_features: HashMap<NodeType, Vec<f32>> = HashMap::new();
        let mut type_counts: HashMap<NodeType, usize> = HashMap::new();

        for obj in log.objects() {
            let nt = NodeType(obj.object_type().to_string());
            node_types.insert(nt.clone());
            let count = type_counts.entry(nt.clone()).or_insert(0);
            object_indices.insert(obj.id().to_string(), (nt.clone(), *count));
            *count += 1;
            
            obj_features.entry(nt).or_default().push(obj.attributes().len() as f32);
        }

        let mut edge_types = std::collections::HashSet::new();
        let mut adjacency_matrices: HashMap<EdgeType, AdjacencyMatrix> = HashMap::new();

        for link in log.event_object_links() {
            if let (Some(&ev_idx), Some((nt, obj_idx))) = (
                event_indices.get(link.event_id()),
                object_indices.get(link.object_id())
            ) {
                let et = EdgeType {
                    subject: event_nt.clone(),
                    predicate: link.qualifier().unwrap_or("related").to_string(),
                    object: nt.clone(),
                };
                edge_types.insert(et.clone());
                adjacency_matrices.entry(et).or_insert(AdjacencyMatrix { edges: Vec::new() }).edges.push((ev_idx, *obj_idx));
            }
        }

        for link in log.object_object_links() {
            if let (Some((nt_from, from_idx)), Some((nt_to, to_idx))) = (
                object_indices.get(link.source_id()),
                object_indices.get(link.target_id())
            ) {
                let et = EdgeType {
                    subject: nt_from.clone(),
                    predicate: link.qualifier().unwrap_or("related").to_string(),
                    object: nt_to.clone(),
                };
                edge_types.insert(et.clone());
                adjacency_matrices.entry(et).or_insert(AdjacencyMatrix { edges: Vec::new() }).edges.push((*from_idx, *to_idx));
            }
        }

        let mut feature_matrices = HashMap::new();
        feature_matrices.insert(event_nt, FeatureMatrix {
            dimensions: 1,
            data: event_features,
        });

        for (nt, data) in obj_features {
            feature_matrices.insert(nt, FeatureMatrix { dimensions: 1, data });
        }

        Self {
            node_types: node_types.into_iter().collect(),
            edge_types: edge_types.into_iter().collect(),
            feature_matrices,
            adjacency_matrices,
        }
    }
}

impl Default for HoegGraph {
    fn default() -> Self {
        Self::new()
    }
}
