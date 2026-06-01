use serde::{Deserialize, Serialize};
use specta::Type;
use tsify::Tsify;

/// Branding tag for OCEL logs in the TypeScript type court.
#[derive(Serialize, Deserialize, Type, Tsify)]
pub struct OcelBrand;

/// Branding tag for XES logs in the TypeScript type court.
#[derive(Serialize, Deserialize, Type, Tsify)]
pub struct XesBrand;

/// Branding tag for WF-Net soundness in the TypeScript type court.
#[derive(Serialize, Deserialize, Type, Tsify)]
pub struct WfNetBrand;
