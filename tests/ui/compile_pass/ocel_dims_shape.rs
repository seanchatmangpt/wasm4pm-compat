// Law: OcelDimsShapeLaw — OcelDims exposes object_types and activities as publicly accessible fields; dimension query is structure-only, no engine
// COMPILE-PASS: OcelDims struct shape — object_types and activities fields are publicly accessible.
use wasm4pm_compat::ocel::{OcelDims, OcelLog, OcelObject, OcelEvent, EventObjectLink};

fn main() {
    let log = OcelLog::new(
        [OcelObject::new("ord-1", "order"), OcelObject::new("item-1", "item")],
        [OcelEvent::new("e1", "place"), OcelEvent::new("e2", "ship")],
        [EventObjectLink::new("e1", "ord-1"), EventObjectLink::new("e2", "item-1")],
        [],
        [],
    );
    let dims = OcelDims::from_log(&log);
    // object_types and activities are public Vec<String> fields.
    assert!(dims.object_types.contains(&"order".to_string()));
    assert!(dims.object_types.contains(&"item".to_string()));
    assert!(dims.activities.contains(&"place".to_string()));
    assert!(dims.activities.contains(&"ship".to_string()));
    assert!(!dims.is_empty());

    // OcelDims::default() is empty.
    let empty: OcelDims = OcelDims::default();
    assert!(empty.is_empty());
}
