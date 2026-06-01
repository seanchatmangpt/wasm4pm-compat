// COMPILE-FAIL: XES structural law — XesEvent cannot be passed where XesTrace is required.
// Law: XesEvent (single event with attributes) and XesTrace (ordered sequence of events)
// are distinct structural types. A single event is not a trace.
use wasm4pm_compat::xes::{XesEvent, XesTrace};

fn requires_xes_trace(_t: XesTrace) {}

fn main() {
    let ev = XesEvent::new();
    // This must fail: XesEvent is not XesTrace.
    requires_xes_trace(ev);
}
