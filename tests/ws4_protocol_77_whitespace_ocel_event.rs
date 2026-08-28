use wasm4pm_compat::prelude::*;
#[test]
fn whitespace_only_ocel_event_type_is_missing(){let c=CapabilityContract::new("cap","https://p","d","in","out",ConsequenceClass::Select,AuthorityMode::None,ReceiptPolicy::Optional," \n ");assert!(c.validate().contains(&ProtocolRefusal::MissingOcelEventType{capability_id:"cap".into()}));}
