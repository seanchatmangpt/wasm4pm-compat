use wasm4pm_compat::prelude::*;
#[test]
fn unsupported_standing_wire_name_is_stable(){assert_eq!(serde_json::to_string(&ProtocolStanding::Unsupported).unwrap(),"\"UNSUPPORTED\"");}
