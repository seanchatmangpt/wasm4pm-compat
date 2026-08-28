use wasm4pm_compat::prelude::*;
#[test]
fn projected_disposition_serializes_explicitly(){let s=SurfaceBinding::projected("cap",SurfaceKind::Cli,"d","in","out");let v=serde_json::to_value(s).unwrap();assert_eq!(v["disposition"]["disposition"],"projected");}
