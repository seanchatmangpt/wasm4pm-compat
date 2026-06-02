//! TypeScript type-law projection module.
//!
//! Exposes branded, generic DTO structures that represent the core typestates
//! and validation metadata of `wasm4pm-compat` in TypeScript, preventing browser
//! runtimes from collapsing complex type courts into generic strings.

pub mod brand;
pub mod export;
pub mod law_projection;

pub mod bpmn_ts;
pub mod process_tree_ts;
pub mod petri_ts;
pub mod declare_ts;
pub mod powl_ts;
pub mod causality_ts;
pub mod multiperspective_ts;
pub mod streaming_ts;
pub mod workflow_ts;
pub mod prediction_ts;

pub use brand::*;
pub use export::export_ts_bindings;
pub use law_projection::*;

pub use bpmn_ts::*;
pub use process_tree_ts::*;
pub use petri_ts::*;
pub use declare_ts::*;
pub use powl_ts::*;
pub use causality_ts::*;
pub use multiperspective_ts::*;
pub use streaming_ts::*;
pub use workflow_ts::*;
pub use prediction_ts::*;

