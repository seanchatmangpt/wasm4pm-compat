//! Verifier shapes — the *structure* of a residual-failset / replay report.
//!
//! ## What this module IS
//!
//! - Containers describing which compliance checks failed and the residual set
//!   left after a replay, as inert reportable data.
//!
//! ## What this module is **NOT**
//!
//! - **Not** a verifier engine. It runs no replay and decides no compliance; it
//!   holds the report shape an engine emits.
//!
//! Structure only. Graduate to `wasm4pm` to *perform* verification.

pub mod failset;
