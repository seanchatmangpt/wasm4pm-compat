use wasm4pm_compat::prelude::*;
#[test]
fn whitespace_only_output_type_is_missing(){let c=CapabilityContract::new("cap","https://p","d","in"," \t ",ConsequenceClass::Select,AuthorityMode::None,ReceiptPolicy::Optional,"evt");assert!(c.validate().contains(&ProtocolRefusal::MissingOutputType{capability_id:"cap".into()}));}
