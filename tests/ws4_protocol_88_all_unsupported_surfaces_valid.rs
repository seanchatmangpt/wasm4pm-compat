use wasm4pm_compat::prelude::*;
#[test]
fn all_explicit_unsupported_surfaces_close_bundle(){let c=CapabilityContract::new("cap","https://p","d","in","out",ConsequenceClass::Select,AuthorityMode::None,ReceiptPolicy::Optional,"evt");let surfaces=PROTOCOL_SURFACES.into_iter().map(|s|SurfaceBinding::unsupported("cap",s,"d","not supported")).collect();let b=ProtocolBundle{protocol_id:"p".into(),version:"1".into(),capabilities:vec![c],surfaces};assert!(b.validate().is_empty());}
