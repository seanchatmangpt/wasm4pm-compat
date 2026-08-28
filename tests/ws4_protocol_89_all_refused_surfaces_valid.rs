use wasm4pm_compat::prelude::*;
#[test]
fn all_explicit_refused_surfaces_close_bundle(){let c=CapabilityContract::new("cap","https://p","d","in","out",ConsequenceClass::Select,AuthorityMode::None,ReceiptPolicy::Optional,"evt");let surfaces=PROTOCOL_SURFACES.into_iter().map(|s|SurfaceBinding::refused("cap",s,"d","POLICY","denied")).collect();let b=ProtocolBundle{protocol_id:"p".into(),version:"1".into(),capabilities:vec![c],surfaces};assert!(b.validate().is_empty());}
