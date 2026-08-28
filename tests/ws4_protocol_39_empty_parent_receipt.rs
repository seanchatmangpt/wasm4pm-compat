use wasm4pm_compat::prelude::*;
fn c()->CapabilityContract{CapabilityContract::new("cap","https://e/c","sem","in","out",ConsequenceClass::Do,AuthorityMode::ExternalDecision,ReceiptPolicy::Required,"evt")}
#[test]
fn do_refuses_empty_parent_receipt_digest(){let e=DoEnvelope::try_new(&c(),SubjectRef::new("s","sd"),"input",AuthorityDecisionRef::new("auth","cap","sd","decision"),ReceiptRequirement::new("1","BLAKE3","replay").with_parent(" ")).unwrap_err();assert!(e.contains(&ProtocolRefusal::EmptyParentReceiptDigest));}
