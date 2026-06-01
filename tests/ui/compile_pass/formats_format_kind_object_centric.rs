// COMPILE-PASS: FormatKind::is_object_centric — proves the three OCEL format
// variants are correctly classified as object-centric; all others are flat.
//
// Law: FormatKindObjectCentricLaw — OCEL formats carry multiple object notions;
// XES/BPMN/Petri/POWL formats are single-case-notion (flat).
use wasm4pm_compat::formats::FormatKind;

fn main() {
    // Object-centric formats.
    assert!(FormatKind::OcelJson.is_object_centric());
    assert!(FormatKind::OcelXml.is_object_centric());
    assert!(FormatKind::OcelSqlite.is_object_centric());

    // Flat formats.
    assert!(!FormatKind::XesXml.is_object_centric());
    assert!(!FormatKind::BpmnXml.is_object_centric());
    assert!(!FormatKind::PetriPnml.is_object_centric());
    assert!(!FormatKind::PowlJson.is_object_centric());
}
