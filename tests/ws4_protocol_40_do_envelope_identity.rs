use wasm4pm_compat::prelude::*;
fn c()->CapabilityContract{CapabilityContract::new("cap","https://e/c","sem","in","out",ConsequenceClass::Do,AuthorityMode::ExternalDecision,ReceiptPolicy::Required,"evt")}
#[test]
fn do_envelope_preserves_authority_and_receipt_identity(){let d=DoEnvelope::try_new(&c(),SubjectRef::new("s","sd"),"input",AuthorityDecisionRef::new("auth","cap","sd","decision"),ReceiptRequirement::new("1","BLAKE3","replay").with_parent("parent")).unwrap();assert_eq!(d.authority.authority_id,"auth");assert_eq!(d.receipt.parent_receipt_digest.as_deref(),Some("parent"));assert_eq!(d.intent().semantic_digest(),"sem");}
