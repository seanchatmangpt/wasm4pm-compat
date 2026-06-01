// COMPILE-PASS: FormatKind::tag — proves every FormatKind variant returns a
// stable, machine-readable tag string.
//
// Law: FormatKindTagLaw — each FormatKind variant has a unique, stable tag that
// identifies the external format for diagnostics and receipts.
use wasm4pm_compat::formats::FormatKind;

fn main() {
    assert_eq!(FormatKind::OcelJson.tag(), "ocel_json");
    assert_eq!(FormatKind::OcelXml.tag(), "ocel_xml");
    assert_eq!(FormatKind::OcelSqlite.tag(), "ocel_sqlite");
    assert_eq!(FormatKind::XesXml.tag(), "xes_xml");
    assert_eq!(FormatKind::BpmnXml.tag(), "bpmn_xml");
    assert_eq!(FormatKind::PetriPnml.tag(), "petri_pnml");
    assert_eq!(FormatKind::PowlJson.tag(), "powl_json");
}
