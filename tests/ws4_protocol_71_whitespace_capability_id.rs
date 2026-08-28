use wasm4pm_compat::prelude::*;
#[test]
fn whitespace_only_capability_id_is_empty(){let c=CapabilityContract::new(" \t ","https://p","d","in","out",ConsequenceClass::Select,AuthorityMode::None,ReceiptPolicy::Optional,"evt");assert!(c.validate().contains(&ProtocolRefusal::EmptyCapabilityId));}
