//! Stable implementation kernel admitted for wasm4pm correspondence work.
//!
//! This crate deliberately excludes the nightly type-law facade, floating-point
//! metrics, serialization, clocks, hashing, and engine actuation. Its first
//! bounded perimeter is D1 token-replay count arithmetic.

#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used)]

pub mod conformance_counts;

pub use conformance_counts::{ExactRatio, ReplayCountRefusal, ReplayCounts};
