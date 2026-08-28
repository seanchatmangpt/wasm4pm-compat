use wasm4pm_compat::prelude::*;
#[test]
fn custom_semantic_remainder_survives_roundtrip(){let c=CapabilityContract::new("cap","https://public","d","in","out",ConsequenceClass::Select,AuthorityMode::None,ReceiptPolicy::Optional,"evt").with_custom_semantic_iri("https://custom");let s=serde_json::to_string(&c).unwrap();let d:CapabilityContract=serde_json::from_str(&s).unwrap();assert_eq!(d.custom_semantic_iri.as_deref(),Some("https://custom"));assert_eq!(d.public_semantic_iri,"https://public");}
