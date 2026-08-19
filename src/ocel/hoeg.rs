//! Heterogeneous Object Event Graph (HOEG) mapping.
//!
//! Based on Smit et al. (2024) "HOEG: A New Approach for Object-Centric
//! Predictive Process Monitoring".
//!
//! This module also provides the structure-only canonicalization boundary used
//! when an admitted OCEL process geometry needs exact content identity. The
//! BLAKE3 digest seals one canonical representation; it does not replace the
//! geometry or turn a scalar digest into process standing.

extern crate alloc;

use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use hashbrown::HashMap;
use std::collections::BTreeSet;

/// Version marker for the deterministic process-geometry byte projection.
const STANDING_GEOMETRY_V1: &[u8] = b"HOEG-STANDING-GEOMETRY-V1";

/// A Semantic Node Type in the HOEG ($NT$).
/// Must include at least an `event` node and one object type.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeType(pub String);

/// An Edge Type in the HOEG ($ET$).
/// Described by a semantic triple: `(subject, predicate, object)`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    /// $NT$: Set of node types, held in canonical lexical order.
    pub node_types: Vec<NodeType>,
    /// $ET$: Set of semantic edge types, held in canonical lexical order.
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

    /// Extract deterministic HOEG matrices from an `OcelLog`.
    ///
    /// OCEL arrays are not used as accidental identity. Events are indexed by
    /// `(timestamp, id)` and objects by `(object_type, id)`, while relation
    /// coordinates are sorted after extraction. Equivalent logs therefore do
    /// not acquire different geometry merely because their JSON arrays or
    /// relation arrays were serialized in a different order.
    pub fn extract(log: &crate::ocel::OcelLog) -> Self {
        let mut node_types = BTreeSet::new();
        let event_nt = NodeType("event".to_string());
        node_types.insert(event_nt.clone());

        let mut object_indices = HashMap::new();
        let mut event_indices = HashMap::new();

        let mut events: Vec<_> = log.events().iter().collect();
        events.sort_by(|left, right| {
            left.timestamp_ns()
                .unwrap_or(0)
                .cmp(&right.timestamp_ns().unwrap_or(0))
                .then_with(|| left.id().cmp(right.id()))
        });

        let mut event_features = Vec::with_capacity(events.len());
        for (index, event) in events.into_iter().enumerate() {
            event_indices.insert(event.id().to_string(), index);
            event_features.push(event.attributes().len() as f32);
        }

        let mut objects: Vec<_> = log.objects().iter().collect();
        objects.sort_by(|left, right| {
            left.object_type()
                .cmp(right.object_type())
                .then_with(|| left.id().cmp(right.id()))
        });

        let mut obj_features: HashMap<NodeType, Vec<f32>> = HashMap::new();
        let mut type_counts: HashMap<NodeType, usize> = HashMap::new();

        for object in objects {
            let node_type = NodeType(object.object_type().to_string());
            node_types.insert(node_type.clone());
            let count = type_counts.entry(node_type.clone()).or_insert(0);
            object_indices.insert(object.id().to_string(), (node_type.clone(), *count));
            *count += 1;

            obj_features
                .entry(node_type)
                .or_default()
                .push(object.attributes().len() as f32);
        }

        let mut edge_types = BTreeSet::new();
        let mut adjacency_matrices: HashMap<EdgeType, AdjacencyMatrix> = HashMap::new();

        for link in log.event_object_links() {
            if let (Some(&event_index), Some((node_type, object_index))) = (
                event_indices.get(link.event_id()),
                object_indices.get(link.object_id()),
            ) {
                let edge_type = EdgeType {
                    subject: event_nt.clone(),
                    predicate: link.qualifier().unwrap_or("related").to_string(),
                    object: node_type.clone(),
                };
                edge_types.insert(edge_type.clone());
                adjacency_matrices
                    .entry(edge_type)
                    .or_insert(AdjacencyMatrix { edges: Vec::new() })
                    .edges
                    .push((event_index, *object_index));
            }
        }

        for link in log.object_object_links() {
            if let (Some((from_type, from_index)), Some((to_type, to_index))) = (
                object_indices.get(link.source_id()),
                object_indices.get(link.target_id()),
            ) {
                let edge_type = EdgeType {
                    subject: from_type.clone(),
                    predicate: link.qualifier().unwrap_or("related").to_string(),
                    object: to_type.clone(),
                };
                edge_types.insert(edge_type.clone());
                adjacency_matrices
                    .entry(edge_type)
                    .or_insert(AdjacencyMatrix { edges: Vec::new() })
                    .edges
                    .push((*from_index, *to_index));
            }
        }

        for matrix in adjacency_matrices.values_mut() {
            matrix.edges.sort_unstable();
        }

        let mut feature_matrices = HashMap::new();
        feature_matrices.insert(
            event_nt,
            FeatureMatrix {
                dimensions: 1,
                data: event_features,
            },
        );

        for (node_type, data) in obj_features {
            feature_matrices.insert(
                node_type,
                FeatureMatrix {
                    dimensions: 1,
                    data,
                },
            );
        }

        Self {
            node_types: node_types.into_iter().collect(),
            edge_types: edge_types.into_iter().collect(),
            feature_matrices,
            adjacency_matrices,
        }
    }

    /// Project this geometry into deterministic bytes suitable for exact-subject
    /// identity and replay comparison.
    ///
    /// Hash-map iteration order never enters this projection. Floating-point
    /// feature values are serialized by exact IEEE-754 bit pattern. The output
    /// is intentionally versioned so a future canonicalization law cannot be
    /// silently confused with this one.
    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        push_bytes(&mut out, STANDING_GEOMETRY_V1);

        let mut node_types = self.node_types.clone();
        node_types.sort();
        push_len(&mut out, node_types.len());
        for node_type in node_types {
            push_string(&mut out, &node_type.0);
        }

        let mut edge_types = self.edge_types.clone();
        edge_types.sort();
        push_len(&mut out, edge_types.len());
        for edge_type in edge_types {
            push_edge_type(&mut out, &edge_type);
        }

        let mut feature_types: Vec<_> = self.feature_matrices.keys().cloned().collect();
        feature_types.sort();
        push_len(&mut out, feature_types.len());
        for node_type in feature_types {
            push_string(&mut out, &node_type.0);
            let matrix = &self.feature_matrices[&node_type];
            push_usize(&mut out, matrix.dimensions);
            push_len(&mut out, matrix.data.len());
            for value in &matrix.data {
                out.extend_from_slice(&value.to_bits().to_le_bytes());
            }
        }

        let mut adjacency_types: Vec<_> = self.adjacency_matrices.keys().cloned().collect();
        adjacency_types.sort();
        push_len(&mut out, adjacency_types.len());
        for edge_type in adjacency_types {
            push_edge_type(&mut out, &edge_type);
            let mut edges = self.adjacency_matrices[&edge_type].edges.clone();
            edges.sort_unstable();
            push_len(&mut out, edges.len());
            for (source, target) in edges {
                push_usize(&mut out, source);
                push_usize(&mut out, target);
            }
        }

        out
    }

    /// BLAKE3 identity of the canonical geometry representation.
    ///
    /// This is an exact representation seal, not a scalar substitute for the
    /// OCEL process geometry that carries standing.
    #[must_use]
    pub fn canonical_blake3(&self) -> [u8; 32] {
        *blake3::hash(&self.canonical_bytes()).as_bytes()
    }
}

impl Default for HoegGraph {
    fn default() -> Self {
        Self::new()
    }
}

fn push_len(out: &mut Vec<u8>, value: usize) {
    push_usize(out, value);
}

fn push_usize(out: &mut Vec<u8>, value: usize) {
    let value = u64::try_from(value).expect("usize must fit u64 on supported targets");
    out.extend_from_slice(&value.to_le_bytes());
}

fn push_bytes(out: &mut Vec<u8>, bytes: &[u8]) {
    push_len(out, bytes.len());
    out.extend_from_slice(bytes);
}

fn push_string(out: &mut Vec<u8>, value: &str) {
    push_bytes(out, value.as_bytes());
}

fn push_edge_type(out: &mut Vec<u8>, edge_type: &EdgeType) {
    push_string(out, &edge_type.subject.0);
    push_string(out, &edge_type.predicate);
    push_string(out, &edge_type.object.0);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ocel::{EventObjectLink, Object, ObjectObjectLink, OcelEvent, OcelLog};

    fn log_with_order(reverse_events: bool, reverse_objects: bool, reverse_links: bool) -> OcelLog {
        let mut objects = vec![
            Object::new("order-1", "order"),
            Object::new("item-1", "item"),
        ];
        let mut events = vec![
            OcelEvent::new("event-1", "create").at_ns(10),
            OcelEvent::new("event-2", "complete").at_ns(20),
        ];
        let mut e2o = vec![
            EventObjectLink::new("event-1", "order-1").qualified("order"),
            EventObjectLink::new("event-1", "item-1").qualified("item"),
            EventObjectLink::new("event-2", "order-1").qualified("order"),
        ];
        let mut o2o = vec![ObjectObjectLink::new("order-1", "item-1").qualified("contains")];

        if reverse_events {
            events.reverse();
        }
        if reverse_objects {
            objects.reverse();
        }
        if reverse_links {
            e2o.reverse();
            o2o.reverse();
        }

        OcelLog::new(objects, events, e2o, o2o, [])
    }

    #[test]
    fn equivalent_ocel_serialization_orders_have_identical_geometry_seal() {
        let canonical = HoegGraph::extract(&log_with_order(false, false, false));
        let reordered = HoegGraph::extract(&log_with_order(true, true, true));

        assert_eq!(canonical.node_types, reordered.node_types);
        assert_eq!(canonical.edge_types, reordered.edge_types);
        assert_eq!(canonical.canonical_bytes(), reordered.canonical_bytes());
        assert_eq!(canonical.canonical_blake3(), reordered.canonical_blake3());
    }

    #[test]
    fn topology_change_changes_geometry_seal() {
        let baseline = HoegGraph::extract(&log_with_order(false, false, false));
        let changed = OcelLog::new(
            [
                Object::new("order-1", "order"),
                Object::new("item-1", "item"),
            ],
            [
                OcelEvent::new("event-1", "create").at_ns(10),
                OcelEvent::new("event-2", "complete").at_ns(20),
            ],
            [
                EventObjectLink::new("event-1", "order-1").qualified("order"),
                EventObjectLink::new("event-2", "item-1").qualified("item"),
            ],
            [ObjectObjectLink::new("item-1", "order-1").qualified("belongs_to")],
            [],
        );
        let changed = HoegGraph::extract(&changed);

        assert_ne!(baseline.canonical_bytes(), changed.canonical_bytes());
        assert_ne!(baseline.canonical_blake3(), changed.canonical_blake3());
    }

    #[test]
    fn node_and_edge_type_order_is_canonical() {
        let graph = HoegGraph::extract(&log_with_order(true, true, true));

        assert!(graph.node_types.windows(2).all(|pair| pair[0] <= pair[1]));
        assert!(graph.edge_types.windows(2).all(|pair| pair[0] <= pair[1]));
        assert!(graph
            .adjacency_matrices
            .values()
            .all(|matrix| matrix.edges.windows(2).all(|pair| pair[0] <= pair[1])));
    }

    #[test]
    fn feature_change_changes_geometry_seal_even_when_topology_is_constant() {
        use crate::ocel::OcelAttribute;

        let baseline = HoegGraph::extract(&log_with_order(false, false, false));
        let changed = OcelLog::new(
            [
                Object::new("order-1", "order")
                    .with_attribute(OcelAttribute::integer("priority", 1)),
                Object::new("item-1", "item"),
            ],
            [
                OcelEvent::new("event-1", "create").at_ns(10),
                OcelEvent::new("event-2", "complete").at_ns(20),
            ],
            [
                EventObjectLink::new("event-1", "order-1").qualified("order"),
                EventObjectLink::new("event-1", "item-1").qualified("item"),
                EventObjectLink::new("event-2", "order-1").qualified("order"),
            ],
            [ObjectObjectLink::new("order-1", "item-1").qualified("contains")],
            [],
        );
        let changed = HoegGraph::extract(&changed);

        assert_ne!(baseline.canonical_blake3(), changed.canonical_blake3());
    }
}
