use wasm4pm_compat::prelude::*;
#[test]
fn invalid_do_contract_accumulates_authority_and_receipt_refusals(){let c=CapabilityContract::new("cap","https://e/c","d","in","out",ConsequenceClass::Do,AuthorityMode::None,ReceiptPolicy::Optional,"evt");let e=c.validate();assert!(e.contains(&ProtocolRefusal::DoWithoutAuthority{capability_id:"cap".into()}));assert!(e.contains(&ProtocolRefusal::DoWithoutRequiredReceipt{capability_id:"cap".into()}));assert_eq!(e.len(),2);}
