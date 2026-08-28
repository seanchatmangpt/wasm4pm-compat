use wasm4pm_compat::prelude::*;
fn c()->CapabilityContract{CapabilityContract::new("cap","https://e/c","sem","in","out",ConsequenceClass::Do,AuthorityMode::ExternalDecision,ReceiptPolicy::Required,"evt")}
#[test]
fn do_refuses_cross_subject_authority(){let e=DoEnvelope::try_new(&c(),SubjectRef::new("s","subject-a"),"input",AuthorityDecisionRef::new("auth","cap","subject-b","decision"),ReceiptRequirement::new("1","BLAKE3","replay")).unwrap_err();assert!(e.contains(&ProtocolRefusal::AuthoritySubjectMismatch{expected:"subject-a".into(),actual:"subject-b".into()}));}
