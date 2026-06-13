//! TypeScript / Zod binding surface for `wasm4pm-compat`.
//!
//! ## What this crate IS
//!
//! - The **sole home** for `specta` derives, Zod schema generation, and any
//!   TypeScript-facing tooling over the `wasm4pm-compat` process-evidence types.
//! - A thin sidecar that depends on `wasm4pm-compat` and re-exposes the shapes a
//!   frontend needs, without imposing TS concerns on the core crate.
//!
//! ## What this crate is **NOT**
//!
//! - **Not** part of the structure-only core. It exists precisely so that
//!   `wasm4pm-compat` can keep its two hard invariants intact:
//!   *exactly three public Cargo features* and *no runtime dependencies*.
//!   A `ts`/specta feature in the core crate would break both.
//! - **Not** an engine. Like the core crate, it is structure-only.
//!
//! ## Generated artifacts
//!
//! The Zod schemas under `bindings/` are rendered from `ggen/ontology/zod-types.ttl`
//! in the parent crate. The `zod_integration/` directory holds a runnable
//! TypeScript demo consuming them. Neither ships with the core crate's
//! `cargo publish` (both are in the parent's `exclude` list).

#![forbid(unsafe_code)]

// Re-export the core crate so downstream TS-generation code has a single import
// root and the dependency edge is explicit.
pub use wasm4pm_compat;
