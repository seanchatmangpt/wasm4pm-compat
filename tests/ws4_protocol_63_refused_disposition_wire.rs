use wasm4pm_compat::prelude::*;
#[test]
fn refused_disposition_serializes_code_and_reason(){let s=SurfaceBinding::refused("cap",SurfaceKind::A2a,"d","POLICY","denied");let v=serde_json::to_value(s).unwrap();assert_eq!(v["disposition"]["disposition"],"refused");assert_eq!(v["disposition"]["code"],"POLICY");assert_eq!(v["disposition"]["reason"],"denied");}
