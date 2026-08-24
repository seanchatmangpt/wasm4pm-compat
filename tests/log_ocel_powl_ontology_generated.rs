//! Verifies the 3 clusters that reuse a real public ontology (log/ocel ->
//! OCEDO) plus the authored-fresh POWL cluster (`ggen/ontology/type-shapes/
//! {log,ocel,powl}.ttl`) actually generate real Rust types -- these were
//! left ungenerated after the first pipeline pass (bpmn + the 11 other
//! authored clusters) until ERRC cycle 5 flagged the gap. Chicago style:
//! real generated types, real field construction, no mocks.

use wasm4pm_compat::log_ontology::{Event, EventAttribute};
use wasm4pm_compat::ocel_ontology::{ObjectRelation, Observe};
use wasm4pm_compat::powl_ontology::Activity;

#[test]
fn log_event_carries_real_observed_at_field() {
    // observed_at is oced:observed_at from the real OCEDO ontology
    // (w3id.org/ocedo/core#) -- confirms the reused public terms actually
    // project into Rust, not just parse as ontology.
    let e = Event {
        observed_at: "2026-08-24T12:00:00Z".to_string(),
        has_event_attribute: vec!["attr1".to_string()],
    };
    assert_eq!(e.observed_at, "2026-08-24T12:00:00Z");
}

#[test]
fn log_event_attribute_is_a_real_key_value_pair() {
    let a = EventAttribute {
        event_attribute: "concept:name".to_string(),
        event_attribute_value: "approve".to_string(),
    };
    assert_eq!(a.event_attribute_value, "approve");
}

#[test]
fn ocel_observe_carries_real_qualifier() {
    // aux:Observe reifies OCEL's E2O relations/qualifier column, per
    // OCEDO's own real ontology design.
    let o = Observe {
        observe_event: vec!["e1".to_string()],
        observe_object: vec!["o1".to_string()],
        qualifier: Some("payer".to_string()),
    };
    assert_eq!(o.qualifier.as_deref(), Some("payer"));
}

#[test]
fn ocel_object_relation_carries_real_relation_type() {
    let r = ObjectRelation {
        from: vec!["o1".to_string()],
        to: vec!["o2".to_string()],
        relation_type: Some("contains".to_string()),
    };
    assert_eq!(r.relation_type.as_deref(), Some("contains"));
}

#[test]
fn powl_activity_carries_real_min_max_frequency() {
    let a = Activity {
        role: None,
        min_frequency: 1,
        max_frequency: Some(3),
        label: Some("approve".to_string()),
        organization: None,
    };
    assert_eq!(a.min_frequency, 1);
    assert_eq!(a.max_frequency, Some(3));
}
