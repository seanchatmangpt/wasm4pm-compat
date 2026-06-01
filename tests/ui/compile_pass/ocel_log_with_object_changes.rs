// Law: OcelObjectChangesTableLaw — OcelLog preserves the ObjectChange table as a first-class fifth constituent; attribute-change records are accessible and structurally sound
// COMPILE-PASS: OcelLog with object changes — the fifth table (ObjectChange) is preserved and accessible.
use wasm4pm_compat::ocel::{OcelLog, OcelObject, OcelEvent, EventObjectLink, ObjectChange};

fn main() {
    let log = OcelLog::new(
        [OcelObject::new("ord-1", "order")],
        [OcelEvent::new("e1", "pay")],
        [EventObjectLink::new("e1", "ord-1")],
        [],
        [
            ObjectChange::new("ord-1", "status", "paid"),
            ObjectChange::new("ord-1", "status", "shipped").at_ns(1_700_000_000_000_000_000),
        ],
    );
    assert!(log.validate().is_ok());
    assert_eq!(log.object_changes().len(), 2);
    assert_eq!(log.object_changes()[0].attribute(), "status");
    assert_eq!(log.object_changes()[1].timestamp_ns(), Some(1_700_000_000_000_000_000));
}
