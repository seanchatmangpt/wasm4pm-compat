// COMPILE-FAIL: OCEL/EventLog structural law — OcelLog cannot be passed where EventLog is required.
// Law: OcelLog (object-centric with multiple object types per event) and EventLog
// (case-centric, one case-id per trace) are distinct types. The flat log must not
// be confused with the object-centric log.
use wasm4pm_compat::eventlog::EventLog;
use wasm4pm_compat::ocel::OcelLog;

fn requires_event_log(_l: EventLog) {}

fn _test(ocel: OcelLog) {
    // This must fail: OcelLog is not EventLog.
    requires_event_log(ocel);
}

fn main() {}
