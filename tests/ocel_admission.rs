//! Concrete admission test — the `LinkedOcel` boundary actually detects the law
//! it names. This is the behavioral companion to the type-law receipts: the
//! crate does not merely *name* `DanglingEventObjectLink`, a shipped function
//! *refuses with it* when an event links to an absent object, and *admits* an
//! otherwise-lawful log through the typed one-way door.

use wasm4pm_compat::admission::Admit;
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::ocel::{EventObjectLink, LinkedOcel, Object, OcelEvent, OcelLog, OcelRefusal};

fn log_with_link(event: &str, links_to: &str, declared_object: &str) -> OcelLog {
    OcelLog::new(
        [Object::new(declared_object, "order")],
        [OcelEvent::new(event, "place")],
        [EventObjectLink::new(event, links_to)],
        [],
        [],
    )
}

#[test]
fn lawful_ocel_is_admitted_through_the_one_way_door() {
    let lawful = log_with_link("e1", "o1", "o1");
    let admitted = LinkedOcel::admit(Evidence::raw(lawful)).expect("lawful log must admit");
    // The admitted value can graduate forward (into the Admitted typestate).
    let _evidence = admitted.into_evidence();
}

#[test]
fn dangling_link_is_refused_by_its_named_law() {
    let dangling = log_with_link("e1", "missing", "o1");
    let refusal =
        LinkedOcel::admit(Evidence::raw(dangling)).expect_err("dangling link must refuse");
    assert_eq!(refusal.reason, OcelRefusal::DanglingEventObjectLink);
}

#[test]
fn empty_e2o_is_refused_by_its_named_law() {
    let empty = OcelLog::new(
        [Object::new("o1", "order")],
        [OcelEvent::new("e1", "place")],
        [], // no E2O links — violates object-centricity
        [],
        [],
    );
    let refusal = LinkedOcel::admit(Evidence::raw(empty)).expect_err("empty E2O must refuse");
    assert_eq!(refusal.reason, OcelRefusal::EmptyEventObjectLinks);
}

#[test]
fn locality_violation_json_is_refused_by_its_named_law() {
    // Fixture shape of wasm4pm-compat-ts/test/fixtures/ocel/
    // ocel_v2_mutation_locality.json (RFC 3339 times): e2 re-points p2's team
    // reference from t1 to t2, an implicit deletion of (t1, p2).
    use wasm4pm_compat::ocel::validate_ocel_locality_json;
    let violating = r#"{
        "eventTypes": [{"name": "create team"}],
        "objectTypes": [{"name": "team"}, {"name": "employee"}],
        "events": [
            {"id": "e1", "type": "create team", "time": "2024-06-09T00:00:00Z",
             "relationships": [{"objectId": "t1", "qualifier": "team"},
                               {"objectId": "p2", "qualifier": "employee"}]},
            {"id": "e2", "type": "create team", "time": "2024-06-09T00:01:00Z",
             "relationships": [{"objectId": "t2", "qualifier": "team"},
                               {"objectId": "p2", "qualifier": "employee"}]}
        ],
        "objects": [{"id": "t1", "type": "team"},
                    {"id": "t2", "type": "team"},
                    {"id": "p2", "type": "employee"}]
    }"#;
    let err = validate_ocel_locality_json(violating, "team", "employee")
        .expect_err("locality violation must refuse");
    assert!(
        err.to_string().ends_with("law:ViolatesLocalityPrinciple"),
        "unexpected refusal: {err}"
    );
}
