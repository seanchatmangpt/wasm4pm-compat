use wasm4pm_compat::prelude::*;
#[test]
fn whitespace_only_protocol_id_is_empty(){let b=ProtocolBundle{protocol_id:" \n ".into(),version:"1".into(),capabilities:vec![],surfaces:vec![]};assert!(b.validate().contains(&ProtocolRefusal::EmptyProtocolId));}
