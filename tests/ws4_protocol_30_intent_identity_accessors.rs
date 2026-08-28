use wasm4pm_compat::prelude::*;
#[test]
fn intent_accessors_preserve_exact_identity(){let c=CapabilityContract::new("cap","https://e/c","sem","in","out",ConsequenceClass::Select,AuthorityMode::None,ReceiptPolicy::Optional,"evt");let i=ProtocolIntent::<SelectPhase>::try_new(&c,SubjectRef::new("subject","subject-digest"),"input-digest").unwrap();assert_eq!(i.capability_id(),"cap");assert_eq!(i.semantic_digest(),"sem");assert_eq!(i.subject().subject_digest,"subject-digest");assert_eq!(i.input_digest(),"input-digest");}
