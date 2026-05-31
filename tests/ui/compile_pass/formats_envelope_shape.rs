// COMPILE-PASS: FormatEnvelope<W> constructs with a witness marker and FormatKind variant
//
// Proves that:
//   1. FormatEnvelope<W>::new takes a FormatKind and raw bytes.
//   2. kind, bytes, and len() are accessible.
//   3. FormatKind variants are distinct (OcelJson, XesXml, BpmnXml, etc.).
//   4. is_empty() reflects an empty bytes vec.
//   5. FormatKind::tag() returns a stable machine-readable string.
//   6. FormatKind::is_object_centric() distinguishes object-centric formats.

use wasm4pm_compat::formats::{FormatEnvelope, FormatKind};
use wasm4pm_compat::witness::Ocel20;

fn main() {
    // Construction with OCEL JSON bytes and an Ocel20 witness marker.
    let env = FormatEnvelope::<Ocel20>::new(FormatKind::OcelJson, b"{}".to_vec());
    assert_eq!(env.kind, FormatKind::OcelJson);
    assert_eq!(env.len(), 2);
    assert!(!env.is_empty());

    // Unit witness: acceptable when the caller does not need a specific witness family.
    let env_unit = FormatEnvelope::<()>::new(FormatKind::XesXml, b"<log/>".to_vec());
    assert_eq!(env_unit.kind, FormatKind::XesXml);
    assert_eq!(env_unit.len(), 6);

    // Empty envelope: must be refused at import — construction itself is fine.
    let empty_env = FormatEnvelope::<()>::new(FormatKind::PowlJson, vec![]);
    assert!(empty_env.is_empty());
    assert_eq!(empty_env.len(), 0);

    // FormatKind variants and their stable tags.
    assert_eq!(FormatKind::OcelJson.tag(),    "ocel_json");
    assert_eq!(FormatKind::OcelXml.tag(),     "ocel_xml");
    assert_eq!(FormatKind::OcelSqlite.tag(),  "ocel_sqlite");
    assert_eq!(FormatKind::XesXml.tag(),      "xes_xml");
    assert_eq!(FormatKind::BpmnXml.tag(),     "bpmn_xml");
    assert_eq!(FormatKind::PetriPnml.tag(),   "petri_pnml");
    assert_eq!(FormatKind::PowlJson.tag(),    "powl_json");

    // Object-centric discrimination: OCEL formats are object-centric; XES/BPMN are not.
    assert!(FormatKind::OcelJson.is_object_centric());
    assert!(FormatKind::OcelXml.is_object_centric());
    assert!(FormatKind::OcelSqlite.is_object_centric());
    assert!(!FormatKind::XesXml.is_object_centric());
    assert!(!FormatKind::BpmnXml.is_object_centric());
    assert!(!FormatKind::PetriPnml.is_object_centric());
    assert!(!FormatKind::PowlJson.is_object_centric());

    // Clone is implemented on FormatEnvelope.
    let env2 = FormatEnvelope::<Ocel20>::new(FormatKind::OcelJson, b"{}".to_vec());
    let env3 = env2.clone();
    assert_eq!(env2.kind, env3.kind);
    assert_eq!(env2.bytes, env3.bytes);
}
