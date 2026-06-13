use std::collections::BTreeMap;
use wasm4pm_compat::ocel::{
    EventObjectLink, Object, ObjectChange, ObjectObjectLink, OcelEvent, OcelLog,
};

/// A simple temporal query that returns the state of an object at a specific timestamp.
/// It merges the initial attributes of the object with any changes that occurred up to the timestamp.
fn get_object_state_at(
    log: &OcelLog,
    object_id: &str,
    timestamp_ns: u64,
) -> BTreeMap<String, String> {
    let mut state = BTreeMap::new();

    // 1. Get initial state from the object definition
    if let Some(obj) = log.objects().iter().find(|o| o.id() == object_id) {
        for attr in obj.attributes() {
            state.insert(attr.key.clone(), attr.value.to_string());
        }
    }

    // 2. Apply changes that happened at or before the given timestamp
    // Sort changes by timestamp to ensure correct application order
    let mut relevant_changes: Vec<_> = log
        .object_changes()
        .iter()
        .filter(|c| c.object_id() == object_id && c.timestamp_ns().unwrap_or(0) <= timestamp_ns)
        .collect();

    relevant_changes.sort_by_key(|c| c.timestamp_ns().unwrap_or(0));

    for change in relevant_changes {
        state.insert(change.attribute().to_string(), change.value().to_string());
    }

    state
}

fn main() {
    println!("--- Rough OCEL Object Evolution Tracker ---");

    // Define an object: a "Machine" that has some initial status
    let machine = Object::new("m1", "Machine").with_attribute(
        wasm4pm_compat::ocel::OcelAttribute::string("status", "idle"),
    );

    // Define some events (required for a valid OcelLog)
    let event1 = OcelEvent::new("e1", "Start").at_ns(100);
    let event2 = OcelEvent::new("e2", "Stop").at_ns(500);

    // Links
    let e2o = vec![
        EventObjectLink::new("e1", "m1"),
        EventObjectLink::new("e2", "m1"),
    ];

    // Define evolution of the machine's status
    let changes = vec![
        ObjectChange::new("m1", "status", "busy").at_ns(150),
        ObjectChange::new("m1", "temperature", "45.0").at_ns(200),
        ObjectChange::new("m1", "temperature", "55.5").at_ns(300),
        ObjectChange::new("m1", "status", "idle").at_ns(550),
        ObjectChange::new("m1", "temperature", "30.2").at_ns(600),
    ];

    // Create the log
    let log = OcelLog::new(
        vec![machine],
        vec![event1, event2],
        e2o,
        Vec::<ObjectObjectLink>::new(),
        changes,
    );

    // Test the temporal query
    let timestamps = vec![0, 100, 150, 250, 400, 600];

    for &ts in &timestamps {
        let state = get_object_state_at(&log, "m1", ts);
        println!("State of machine 'm1' at t={}: {:?}", ts, state);
    }

    // Validation check
    match log.validate() {
        Ok(_) => println!("Log is valid according to OCEL laws."),
        Err(e) => println!("Log validation failed: {}", e),
    }
}
