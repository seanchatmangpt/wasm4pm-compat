//! `gymact-rs` — a Rust projection of the [GymAct](https://github.com/seanchatmangpt/gymact)
//! public-semantic execution profile.
//!
//! **Sidecar crate, own lifecycle.** This crate is excluded from the
//! `wasm4pm-compat` workspace (see root `Cargo.toml`'s `[workspace] exclude`)
//! so it can be lifted into its own repository later via a plain copy or
//! `git subtree split`, without ever having been entangled in
//! `wasm4pm-compat`'s workspace resolution or release cadence.
//!
//! `src/consequence.rs` is **ggen-rendered source. Do not hand-edit it.** To
//! change it, edit the vendored ontology at
//! `../ggen/ontology/gymact/profile.ttl` and re-run `ggen sync` (see
//! `../ggen/ggen_gymact.toml` for the exact invocation) — the same discipline
//! `wasm4pm-compat`'s own `src/witnesses.rs` follows for its ggen-rendered
//! source.
//!
//! This crate deliberately reuses `wasm4pm-compat`'s domain-agnostic
//! admission/authority/witness/receipt vocabulary (`wasm4pm_compat::authority`,
//! `wasm4pm_compat::admission`, `wasm4pm_compat::witness`,
//! `wasm4pm_compat::receipt`) rather than reimplementing it — those primitives
//! don't know about OCEL/XES/process-mining when checking a `Refusal<R, W>`
//! shape, which is exactly what makes them reusable here for the
//! benchmark-actuation domain.

/// GymAct consequence classes (`urn:gymact:scheme:consequence`), ggen-rendered
/// from the vendored ontology.
pub mod consequence;
/// GymAct interaction patterns (`urn:gymact:scheme:interaction`),
/// ggen-rendered from the vendored ontology.
pub mod interaction;
/// GymAct execution roles (`urn:gymact:scheme:roles`), ggen-rendered from the
/// vendored ontology.
pub mod role;
/// GymAct evidence standing (`urn:gymact:scheme:standing`), ggen-rendered from
/// the vendored ontology.
pub mod standing;

/// Authority gate pairing `consequence::ConsequenceClass` with
/// `wasm4pm_compat::authority::AuthorityEnvelope` — hand-written, **not**
/// ggen-rendered. This is the crate's first genuine integration point with
/// `wasm4pm-compat` rather than an aspirational doc-comment dependency.
pub mod gate;
