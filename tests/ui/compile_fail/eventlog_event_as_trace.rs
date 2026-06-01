// COMPILE-FAIL: Event log structural law — Event cannot be passed where Trace is required.
// Law: Event (a single logged activity occurrence) and Trace (an ordered sequence of events)
// are distinct types. A raw event must not be confused with a case trace.
use wasm4pm_compat::eventlog::{Event, Trace};

fn requires_trace(_trace: Trace) {}

fn main() {
    let ev = Event::new("place_order");
    // This must fail: Event is not Trace.
    requires_trace(ev);
}
