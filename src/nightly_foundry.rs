//! Internal nightly foundry. Gated by `#[cfg(wasm4pm_compat_nightly)]` — NOT a public feature.
//!
//! This module is where stronger type-level structure is forged and stress-tested
//! before it is allowed to graduate into the always-on canon. It is intentionally
//! **empty on stable**: nothing here is part of the public, stable surface, and it
//! must never be reachable without explicitly enabling the `wasm4pm_compat_nightly`
//! custom cfg (e.g. `RUSTFLAGS="--cfg wasm4pm_compat_nightly"`).
//!
//! Keeping experiments quarantined behind a cfg — rather than a Cargo feature —
//! guarantees that stable Rust builds by default and that no downstream crate can
//! accidentally depend on unstable structure.

// (No items on stable. Experimental type-level structure is added here only
//  while iterating under the nightly foundry cfg.)
