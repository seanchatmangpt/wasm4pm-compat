// Law: OcelLogFiveTablesLaw — OcelLog is constructible with all five OCEL 2.0 tables (objects, events, E2O links, O2O links, object changes); all validate cleanly
// COMPILE-PASS: OcelLog five-table construction — all five constituent tables are populated and validate cleanly.
use wasm4pm_compat::ocel::{
    OcelLog, OcelObject, OcelEvent, EventObjectLink, ObjectObjectLink, ObjectChange,
};

fn main() {
    let log = OcelLog::new(
        [
            OcelObject::new("ord-1", "order"),
            OcelObject::new("item-1", "item"),
        ],
        [
            OcelEvent::new("e1", "place"),
            OcelEvent::new("e2", "ship"),
        ],
        [
            EventObjectLink::new("e1", "ord-1").qualified("placed_by"),
            EventObjectLink::new("e2", "item-1").qualified("shipped"),
        ],
        [
            ObjectObjectLink::new("ord-1", "item-1").qualified("contains"),
        ],
        [
            ObjectChange::new("ord-1", "status", "open"),
        ],
    );
    assert!(log.validate().is_ok());
    assert_eq!(log.objects().len(), 2);
    assert_eq!(log.events().len(), 2);
    assert_eq!(log.event_object_links().len(), 2);
    assert_eq!(log.object_object_links().len(), 1);
    assert_eq!(log.object_changes().len(), 1);
}
