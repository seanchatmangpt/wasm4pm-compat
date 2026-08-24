//! Verifies `src/bpmn_ontology.rs` — rendered from
//! `ggen/ontology/type-shapes/bpmn.ttl` (reusing the real sBPMN ontology) —
//! actually carries real per-field shapes, not an opaque bucket. Chicago
//! style: real generated types, real field construction, state-based
//! assertions, no mocks.

use wasm4pm_compat::bpmn_ontology::{
    Gateway, GatewayGatewayDirection, Process, SequenceFlow, Task,
};

#[test]
fn gateway_carries_real_direction_enum() {
    let g = Gateway {
        id: "g1".to_string(),
        name: Some("split".to_string()),
        gateway_direction: Some(GatewayGatewayDirection::Diverging),
    };

    assert_eq!(g.id, "g1");
    assert_eq!(g.name.as_deref(), Some("split"));
    assert_eq!(g.gateway_direction, Some(GatewayGatewayDirection::Diverging));
    assert_ne!(g.gateway_direction, Some(GatewayGatewayDirection::Converging));
}

#[test]
fn sequence_flow_carries_real_source_target_and_condition() {
    let flow = SequenceFlow {
        id: "f1".to_string(),
        name: None,
        source_ref: "a".to_string(),
        target_ref: "b".to_string(),
        condition_expression: Some("amount > 100".to_string()),
    };

    assert_eq!(flow.source_ref, "a");
    assert_eq!(flow.target_ref, "b");
    assert_eq!(flow.condition_expression.as_deref(), Some("amount > 100"));
    // sBPMN restores conditionExpression, a real BPMN 2.0 construct pm4py's
    // Python object model omits -- this field existing at all is the point.
}

#[test]
fn task_incoming_outgoing_are_real_fields() {
    let t = Task {
        id: "t1".to_string(),
        name: Some("approve".to_string()),
        incoming: vec!["f1".to_string()],
        outgoing: vec!["f2".to_string(), "f3".to_string()],
    };

    assert_eq!(t.incoming, vec!["f1".to_string()]);
    assert_eq!(t.outgoing.len(), 2);
}

#[test]
fn process_is_executable_and_flow_element_are_real_fields() {
    let p = Process {
        id: "p1".to_string(),
        name: Some("order-to-cash".to_string()),
        is_executable: Some(true),
        flow_element: vec!["t1".to_string(), "g1".to_string()],
    };

    assert_eq!(p.is_executable, Some(true));
    assert_eq!(p.flow_element.len(), 2);
}
