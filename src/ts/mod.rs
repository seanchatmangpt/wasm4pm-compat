//! TypeScript type-law projection module.
//!
//! Exposes branded, generic DTO structures that represent the core typestates
//! and validation metadata of `wasm4pm-compat` in TypeScript, preventing browser
//! runtimes from collapsing complex type courts into generic strings.

pub mod brand;
pub mod export;
pub mod law_projection;

pub use brand::*;
pub use export::export_ts_bindings;
pub use law_projection::*;
