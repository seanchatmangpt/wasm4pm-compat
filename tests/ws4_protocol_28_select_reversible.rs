use wasm4pm_compat::prelude::*;
#[test]
fn select_phase_is_reversible(){let c=CapabilityContract::new("cap","https://e/c","d","in","out",ConsequenceClass::Select,AuthorityMode::None,ReceiptPolicy::Optional,"evt");let i=ProtocolIntent::<SelectPhase>::try_new(&c,SubjectRef::new("s","sd"),"input").unwrap();assert!(i.reversible());}
