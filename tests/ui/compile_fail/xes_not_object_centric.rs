// COMPILE-FAIL: XES case-centric distinctness — XesLog cannot be substituted
// where OcelLog is required.
//
// Law: IEEE 1849-2023 — XES is a case-centric format; OCEL 2.0 is object-
// centric. These are different paradigms. A function accepting OcelLog must
// not accept XesLog, and vice versa.
//
// Expected error: mismatched types — XesLog is not OcelLog.
use wasm4pm_compat::ocel::OcelLog;
use wasm4pm_compat::xes::{XesLog, XesTrace, XesEvent, XesExtension};

fn requires_ocel(_log: &OcelLog) {}

fn main() {
    let xes_log = XesLog::new(
        "running-example",
        [XesExtension::new("Concept", "concept", "http://www.xes-standard.org/concept.xesext")],
        [XesTrace::new("c1", [XesEvent::new().with("concept:name", "place_order")])],
    );
    // XesLog is not OcelLog — passing a XesLog where OcelLog is required
    // must fail at compile time, proving the case-centric/object-centric
    // boundary is enforced at the type level.
    requires_ocel(&xes_log);
}
