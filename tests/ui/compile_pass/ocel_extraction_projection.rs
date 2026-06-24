#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use wasm4pm_compat::interop::{ExtractionProjection, MaterializationProjection, RelationalShape, OcelShape};
use wasm4pm_compat::loss::{LossPolicy, Project};

fn main() {
    let source_tables = vec!["orders".to_string(), "items".to_string()];
    let ext_proj = ExtractionProjection::new(source_tables, "orders.id = items.order_id");
    
    let report = ext_proj.project(LossPolicy::AllowLossWithReport).unwrap();
    assert_eq!(report.projection.as_str(), "relational-extract-to-ocel");

    let mat_proj = MaterializationProjection::new(vec!["order_table".to_string()]);
    let mat_report = mat_proj.project(LossPolicy::AllowNamedProjection).unwrap();
    assert_eq!(mat_report.projection.as_str(), "ocel-materialize-to-relational");
}
