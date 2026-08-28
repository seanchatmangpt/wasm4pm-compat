use wasm4pm_compat::prelude::*;
#[test]
fn select_intent_preserves_exact_subject(){let c=CapabilityContract::new("cap","https://p","d","in","out",ConsequenceClass::Select,AuthorityMode::None,ReceiptPolicy::Optional,"evt");let i=Intent::<SelectPhase>::try_new(&c,SubjectRef::new("subject","subject-d"),"input-d").unwrap();assert_eq!(i.subject().subject_id,"subject");assert_eq!(i.subject().subject_digest,"subject-d");}
