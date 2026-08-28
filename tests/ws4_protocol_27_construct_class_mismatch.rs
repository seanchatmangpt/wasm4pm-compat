use wasm4pm_compat::prelude::*;
#[test]
fn construct_intent_refuses_select_contract(){let c=CapabilityContract::new("cap","https://e/c","d","in","out",ConsequenceClass::Select,AuthorityMode::None,ReceiptPolicy::Optional,"evt");let e=ProtocolIntent::<ConstructPhase>::try_new(&c,SubjectRef::new("s","sd"),"input").unwrap_err();assert!(e.contains(&ProtocolRefusal::ConsequenceClassMismatch{capability_id:"cap".into(),expected:ConsequenceClass::Construct,actual:ConsequenceClass::Select}));}
