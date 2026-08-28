use wasm4pm_compat::prelude::*;
#[test]
fn whitespace_only_protocol_version_is_empty(){let b=ProtocolBundle{protocol_id:"p".into(),version:" \r\n ".into(),capabilities:vec![],surfaces:vec![]};assert!(b.validate().contains(&ProtocolRefusal::EmptyProtocolVersion));}
