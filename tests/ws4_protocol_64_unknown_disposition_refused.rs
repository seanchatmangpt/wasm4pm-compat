use wasm4pm_compat::prelude::SurfaceDisposition;
#[test]
fn unknown_surface_disposition_cannot_deserialize(){let r=serde_json::from_str::<SurfaceDisposition>(r#"{"disposition":"magic"}"#);assert!(r.is_err());}
