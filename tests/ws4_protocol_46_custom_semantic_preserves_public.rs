use wasm4pm_compat::prelude::*;
#[test]
fn custom_semantic_remainder_does_not_replace_public_semantic(){let c=CapabilityContract::new("cap","https://public.example/cap","d","in","out",ConsequenceClass::Select,AuthorityMode::None,ReceiptPolicy::Optional,"evt").with_custom_semantic_iri("https://custom.example/cap");assert_eq!(c.public_semantic_iri,"https://public.example/cap");assert_eq!(c.custom_semantic_iri.as_deref(),Some("https://custom.example/cap"));}
