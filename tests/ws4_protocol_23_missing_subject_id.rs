use wasm4pm_compat::prelude::*;
fn c()->CapabilityContract{CapabilityContract::new("cap","https://e/c","d","in","out",ConsequenceClass::Select,AuthorityMode::None,ReceiptPolicy::Optional,"evt")}
#[test]
fn select_intent_refuses_missing_subject_id(){let e=ProtocolIntent::<SelectPhase>::try_new(&c(),SubjectRef::new(" ","sd"),"input").unwrap_err();assert!(e.contains(&ProtocolRefusal::MissingSubjectId));}
