//! WASM Boundary Law modules.
//! Exposes structural types and validation functions to compile wasm targets.

pub mod abi;
pub mod bindings;
pub mod boundary;

pub use abi::*;
pub use bindings::*;
pub use boundary::*;
