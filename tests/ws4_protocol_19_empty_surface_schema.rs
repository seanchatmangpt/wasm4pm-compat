use wasm4pm_compat::prelude::*;
fn c()->CapabilityContract{CapabilityContract::new("cap","https://e/c","d","in","out",ConsequenceClass::Select,AuthorityMode::None,ReceiptPolicy::Optional,"evt")}
#[test]
fn empty_projected_schema_is_refused(){let s=SurfaceBinding::projected("cap",SurfaceKind::Cli,"d"," ","out");let b=ProtocolBundle{protocol_id:"p".into(),version:"1".into(),capabilities:vec![c()],surfaces:vec![s]};assert!(b.validate().contains(&ProtocolRefusal::EmptySurfaceSchema{capability_id:"cap".into(),surface:SurfaceKind::Cli}));}
