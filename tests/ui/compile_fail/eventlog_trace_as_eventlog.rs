// COMPILE-FAIL: Event log structural law — Trace cannot be passed where EventLog is required.
// Law: Trace (one case, one sequence of events) and EventLog (the full collection of traces)
// are distinct types. A single trace must not be confused with an event log.
use wasm4pm_compat::eventlog::{EventLog, Trace};

fn requires_event_log(_log: EventLog) {}

fn main() {
    let trace = Trace::new("case-1", []);
    // This must fail: Trace is not EventLog.
    requires_event_log(trace);
}
