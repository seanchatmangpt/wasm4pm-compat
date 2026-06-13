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
