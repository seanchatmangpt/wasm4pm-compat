// Law: XesCaseCentricDistinctionLaw — XesLog is a case-centric format distinct from OcelLog; a function accepting OcelLog cannot accept XesLog (IEEE 1849-2023 vs OCEL 2.0)
// COMPILE-PASS: XES shape — case-centric log is distinct from OCEL.
// XesLog is not the same type as OcelLog; this proves the distinction compiles.
use wasm4pm_compat::xes::{XesLog, XesTrace, XesEvent, XesExtension};

fn only_xes(_: &XesLog) {}

fn main() {
    let log = XesLog::new(
        "running-example",
        [XesExtension::new("Concept", "concept", "http://www.xes-standard.org/concept.xesext")],
        [XesTrace::new("c1", [XesEvent::new().with("concept:name", "place_order")])],
    );
    only_xes(&log);
    assert_eq!(log.traces().len(), 1);
}
