use wasm4pm_compat::prelude::*;
#[test]
fn bundle_propagates_capability_refusal_before_surface_closure(){let c=CapabilityContract::new("cap","","d","in","out",ConsequenceClass::Select,AuthorityMode::None,ReceiptPolicy::Optional,"evt");let b=ProtocolBundle{protocol_id:"p".into(),version:"1".into(),capabilities:vec![c],surfaces:vec![]};let r=b.validate();assert!(r.contains(&ProtocolRefusal::MissingPublicSemantic{capability_id:"cap".into()}));assert_eq!(r.iter().filter(|x|matches!(x,ProtocolRefusal::MissingSurfaceBinding{..})).count(),4);}
