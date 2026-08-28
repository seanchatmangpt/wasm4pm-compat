use wasm4pm_compat::prelude::*;
#[test]
fn select_intent_preserves_exact_input_digest(){let c=CapabilityContract::new("cap","https://p","d","in","out",ConsequenceClass::Select,AuthorityMode::None,ReceiptPolicy::Optional,"evt");let i=Intent::<SelectPhase>::try_new(&c,SubjectRef::new("s","sd"),"input-d").unwrap();assert_eq!(i.input_digest(),"input-d");}
