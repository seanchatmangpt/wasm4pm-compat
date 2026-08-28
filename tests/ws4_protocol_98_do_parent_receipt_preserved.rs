use wasm4pm_compat::prelude::*;
#[test]
fn valid_do_envelope_preserves_parent_receipt_identity(){let c=CapabilityContract::new("cap","https://p","d","in","out",ConsequenceClass::Do,AuthorityMode::ExternalDecision,ReceiptPolicy::Required,"evt");let a=AuthorityDecisionRef::new("auth","cap","sd","decision");let rr=ReceiptRequirement::new("1","blake3","replay-v1").with_parent("parent-d");let e=DoEnvelope::try_new(&c,SubjectRef::new("s","sd"),"id",a,rr).unwrap();assert_eq!(e.receipt.parent_receipt_digest.as_deref(),Some("parent-d"));}
