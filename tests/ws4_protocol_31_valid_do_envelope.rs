use wasm4pm_compat::prelude::*;
fn c()->CapabilityContract{CapabilityContract::new("cap","https://e/c","sem","in","out",ConsequenceClass::Do,AuthorityMode::ExternalDecision,ReceiptPolicy::Required,"evt")}
#[test]
fn valid_do_envelope_is_admitted(){let d=DoEnvelope::try_new(&c(),SubjectRef::new("s","sd"),"input",AuthorityDecisionRef::new("auth","cap","sd","decision"),ReceiptRequirement::new("1","BLAKE3","replay-v1")).unwrap();assert_eq!(d.intent().consequence_class(),ConsequenceClass::Do);assert!(!d.intent().reversible());}
