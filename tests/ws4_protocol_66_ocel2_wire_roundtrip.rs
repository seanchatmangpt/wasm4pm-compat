use wasm4pm_compat::prelude::EventWireFormat;
#[test]
fn ocel2_wire_roundtrips(){let s=serde_json::to_string(&EventWireFormat::Ocel2).unwrap();assert_eq!(serde_json::from_str::<EventWireFormat>(&s).unwrap(),EventWireFormat::Ocel2);}
