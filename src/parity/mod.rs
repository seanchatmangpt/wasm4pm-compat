//! Parity shapes — the *structure* of a drift/conformance-delta report.
//!
//! ## What this module IS
//!
//! - Containers describing the difference between two conformance results
//!   (e.g. a baseline vs. a current fitness/precision), as inert data.
//!
//! ## What this module is **NOT**
//!
//! - **Not** a drift detector. It computes no delta from logs; it holds a delta
//!   that an engine produced.
//!
//! Structure only. Graduate to `wasm4pm` to *detect* drift.

pub mod delta;
