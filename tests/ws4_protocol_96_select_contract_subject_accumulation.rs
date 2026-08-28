use wasm4pm_compat::prelude::*;
#[test]
fn select_intent_accumulates_contract_and_subject_refusals(){let c=CapabilityContract::new("cap","","d","in","out",ConsequenceClass::Select,AuthorityMode::None,ReceiptPolicy::Optional,"evt");let r=Intent::<SelectPhase>::try_new(&c,SubjectRef::new("","sd"),"id").unwrap_err();assert!(r.contains(&ProtocolRefusal::MissingPublicSemantic{capability_id:"cap".into()}));assert!(r.contains(&ProtocolRefusal::MissingSubjectId));}
