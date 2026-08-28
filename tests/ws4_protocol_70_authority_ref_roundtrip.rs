use wasm4pm_compat::prelude::AuthorityDecisionRef;
#[test]
fn authority_decision_reference_roundtrips_exact_identity(){let a=AuthorityDecisionRef::new("auth","cap","subject-d","decision-d");let s=serde_json::to_string(&a).unwrap();let d:AuthorityDecisionRef=serde_json::from_str(&s).unwrap();assert_eq!(d,a);}
