//! Verifies the 11 newly-authored ontology clusters' generated Rust types
//! (rendered from `ggen/ontology/type-shapes/*.ttl` via the same pipeline
//! proven for `bpmn` in `tests/bpmn_ontology_generated.rs`). Chicago style:
//! real generated types, real field construction, state-based assertions,
//! no mocks. One representative test per cluster, not exhaustive coverage
//! of every shape.

use wasm4pm_compat::dfg_ontology::DirectlyFollowsEdge;
use wasm4pm_compat::genetic_matrix_ontology::InputMapEntry;
use wasm4pm_compat::heuristics_net_ontology::{Edge, EdgeEdgeType};
use wasm4pm_compat::oc_causal_net_ontology::Marker;
use wasm4pm_compat::ocpn_ontology::Arc as OcpnArc;
use wasm4pm_compat::org_ontology::SocialNetworkConnection;
use wasm4pm_compat::petri_net_ontology::{Arc as PetriArc, ArcArcKind};
use wasm4pm_compat::process_tree_ontology::{ProcessTreeNode, ProcessTreeNodeOperator};
use wasm4pm_compat::random_variables_ontology::{
    Normal, RandomVariable, RandomVariableVariantKind,
};
use wasm4pm_compat::transition_system_ontology::Transition as TsTransition;
use wasm4pm_compat::trie_ontology::Trie;

#[test]
fn dfg_edge_carries_real_source_target_count() {
    let e = DirectlyFollowsEdge {
        source_activity: "approve".to_string(),
        target_activity: "ship".to_string(),
        count: 42,
    };
    assert_eq!(e.source_activity, "approve");
    assert_eq!(e.count, 42);
}

#[test]
fn genetic_matrix_input_map_entry_real_fields() {
    let e = InputMapEntry {
        node: "t1".to_string(),
        predecessor_set_id: 0,
        predecessor_activity: vec!["a".to_string(), "b".to_string()],
    };
    assert_eq!(e.predecessor_activity.len(), 2);
}

#[test]
fn petri_net_arc_kind_enum_round_trips() {
    let arc = PetriArc {
        source: "p1".to_string(),
        target: "t1".to_string(),
        weight: Some(1),
        arc_kind: Some(ArcArcKind::Inhibitor),
        properties: None,
    };
    assert_eq!(arc.arc_kind, Some(ArcArcKind::Inhibitor));
    assert_ne!(arc.arc_kind, Some(ArcArcKind::Reset));
}

#[test]
fn process_tree_operator_enum_uses_member_names_not_symbols() {
    // Regression test: the ontology originally used pm4py's raw Enum
    // .value strings ("->","X",...) as sh:in values, which are not valid
    // Rust identifiers -- fixed to use the member names (Sequence, Xor,
    // ...). This test would fail to compile if that regressed.
    let node = ProcessTreeNode {
        label: None,
        operator: Some(ProcessTreeNodeOperator::Sequence),
        child: vec!["n1".to_string(), "n2".to_string()],
    };
    assert_eq!(node.operator, Some(ProcessTreeNodeOperator::Sequence));
    assert_eq!(node.child.len(), 2);
}

#[test]
fn trie_final_field_uses_raw_identifier_escape() {
    // Regression test: `final` is a Rust reserved keyword; the template
    // must escape it as `r#final`. This test would fail to compile if
    // that regressed.
    let t = Trie {
        label: Some("root".to_string()),
        parent: None,
        child: vec![],
        r#final: false,
        depth: 0,
    };
    assert!(!t.r#final);
    assert_eq!(t.depth, 0);
}

#[test]
fn transition_system_transition_carries_real_from_to_state() {
    let t = TsTransition {
        transition_name: Some("fire".to_string()),
        from_state: "s0".to_string(),
        to_state: "s1".to_string(),
        event: vec!["approve".to_string()],
    };
    assert_eq!(t.from_state, "s0");
    assert_eq!(t.to_state, "s1");
}

#[test]
fn heuristics_net_edge_type_enum_is_valid_rust_identifiers() {
    // Regression test: pm4py's edge_type values are lowercase string
    // literals ("frequency"/"performance"); the template must title-case
    // them into valid, idiomatic Rust enum variants.
    let e = Edge {
        start_node: "n1".to_string(),
        end_node: "n2".to_string(),
        dependency_value: 0.0,
        dfg_value: 0.0,
        repr_value: 0.0,
        edge_label: None,
        repr_color: None,
        edge_type: Some(EdgeEdgeType::Frequency),
        edge_net_name: None,
    };
    assert_eq!(e.edge_type, Some(EdgeEdgeType::Frequency));
}

#[test]
fn oc_causal_net_marker_carries_real_count_range_split() {
    let m = Marker {
        related_activity: "ship".to_string(),
        object_type: "order".to_string(),
        count_min: 1,
        count_max: 1,
        marker_key: 7,
    };
    assert_eq!(m.count_min, 1);
    assert_eq!(m.marker_key, 7);
}

#[test]
fn ocpn_arc_carries_object_type_and_is_variable() {
    let a = OcpnArc {
        source_ref: "p1".to_string(),
        target_ref: "t1".to_string(),
        object_type: "order".to_string(),
        is_variable: true,
    };
    assert!(a.is_variable);
}

#[test]
fn org_social_network_connection_carries_real_weight() {
    let c = SocialNetworkConnection {
        source_resource: "alice".to_string(),
        target_resource: "bob".to_string(),
        connection_weight: 0.75,
    };
    assert!((c.connection_weight - 0.75).abs() < f64::EPSILON);
}

#[test]
fn random_variable_wrapper_discriminates_real_distribution_kind() {
    let n = Normal {
        priority: 0,
        weight: 1.0,
        mu: 0.0,
        sigma: 1.0,
    };
    assert_eq!(n.mu, 0.0);

    let wrapper = RandomVariable {
        variant_kind: RandomVariableVariantKind::Normal,
        random_variable_ref: "rv1".to_string(),
    };
    assert_eq!(wrapper.variant_kind, RandomVariableVariantKind::Normal);
}
