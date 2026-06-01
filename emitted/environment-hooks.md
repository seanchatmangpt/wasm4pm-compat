# Environment-Based Hooks & Build Flag Dependencies

## Summary

The `wasm4pm-compat` crate operates with **minimal environmental surface**. It is nightly-only with no runtime environment variables, no conditional feature gates driven by environment, no RUSTFLAGS, and no build-time hooks that depend on external state.

---

## Nightly Toolchain (Unconditional)

### File: `rust-toolchain.toml`

```toml
[toolchain]
channel = "nightly"
```

**Effect:** Locks all builds to Rust nightly. This is not configurable via environment — the crate **requires** nightly unconditionally.

**Nightly Features Declared** (in `src/lib.rs`, lines 148-154):
- `generic_const_exprs` — law machinery and `WfNetConst<SOUNDNESS>`
- `adt_const_params` — `ConditionCell<BITS>`, `Between01<NUM,DEN>`, and `Metric<KIND,NUM,DEN>`
- `const_trait_impl` — compile-time trait dispatch in law surfaces
- `min_specialization` — type-law narrowing in `nightly_foundry`
- `portable_simd` — SIMD-width type-law surface in `nightly_foundry`

**Rule:** There is no stable build target, no MSRV, and no fallback. Applications conform *upward* to the type law.

---

## Cargo.toml Feature Model

### File: `Cargo.toml` (lines 19-47)

Three public Cargo features control **capability stages** (not canon knowledge):

| Feature | Default | Gating |
|---------|:-------:|--------|
| `formats` | yes | import/export contracts, `src/formats.rs` module, loss surfaces |
| `strict` | no | opt-in boundary judgment, `src/strict.rs` module |
| `wasm4pm` | no | graduation bridge traits, `src/engine_bridge.rs` module |
| `ts` | no | TypeScript law projections, `src/ts/` module, requires `specta`, `serde`, `tsify`, `wasm-bindgen` |
| `wasm` | no | WASM-safe boundary projection, `src/wasm/` module, requires `wasm-bindgen`, `serde-wasm-bindgen`, `specta`, `serde`, `tsify` |

**Note:** There is no `nightly` feature. Nightly is required unconditionally (pinned via `rust-toolchain.toml`).

**Conditional Module Exports** (in `src/lib.rs`):

```rust
#[cfg(feature = "wasm4pm")]
pub mod engine_bridge;

#[cfg(feature = "formats")]
pub mod formats;

#[cfg(feature = "strict")]
pub mod strict;

#[cfg(feature = "ts")]
pub mod ts;

#[cfg(feature = "wasm")]
pub mod wasm;
```

**Always-On Modules** (line 157 comment):
All other modules (law, eventlog, ocel, xes, bpmn, petri, powl, conformance, etc.) are compiled regardless of feature flags. Canon knowledge is base profile.

---

## Build Configuration

### File: `.cargo/config.toml`

```toml
# wasm4pm-compat — nightly-only crate.
# The rust-toolchain.toml pins to nightly; no RUSTFLAGS are needed.
# The nightly feature attributes (#![feature(...)]) live in src/lib.rs.
```

**Effect:** Explicitly declares no RUSTFLAGS are needed. All nightly feature gating is compile-time via `#![feature(...)]` attributes in source, never environment-driven.

---

## Test Surfaces & Feature Gating

### ALIVE Gate (Type-Law Receipts)

File: `tests/ui_tests.rs` (lines 24-48)

Three distinct test targets with feature-conditional execution:

```rust
#[test]
#[ignore = "trybuild compile-time law receipts — run explicitly: cargo test --test ui_tests -- --ignored"]
fn compile_fail_fixtures() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/compile_fail/*.rs");
}

#[test]
#[ignore = "trybuild compile-time law receipts — run explicitly: cargo test --test ui_tests -- --ignored"]
fn compile_pass_fixtures() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/compile_pass/*.rs");
}

#[test]
#[cfg(feature = "wasm4pm")]
#[ignore = "trybuild compile-time law receipts (wasm4pm feature) — run explicitly: cargo test --test ui_tests --features wasm4pm -- --ignored"]
fn compile_pass_wasm4pm_fixtures() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/compile_pass_wasm4pm/*.rs");
}

#[test]
#[cfg(feature = "strict")]
#[ignore = "trybuild compile-time law receipts (strict feature) — run explicitly: cargo test --test ui_tests --features strict -- --ignored"]
fn compile_pass_strict_fixtures() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/compile_pass_strict/*.rs");
}
```

**Key Behavior:**
- All ALIVE tests are **marked `#[ignore]`** — they do NOT run in default `cargo test`.
- Feature-gated tests run only when the feature is enabled (e.g., `cargo test --test ui_tests --features wasm4pm -- --ignored`).
- No environment variables trigger ALIVE gate; gate is explicit invocation.

### Documentation Tests (Disabled by Default)

File: `Cargo.toml`, line 17:

```toml
doctest = false
```

**Reason:** Doctests touching nightly features (`generic_const_exprs`, `adt_const_params`) trigger separate `rustc` invocations. Running 200+ doctests would extend test time to 4+ minutes — unacceptable for dev loop.

**Explicit Opt-In:**
```bash
cargo test --doc --all-features
```

---

## Audit Scripts (No Environment Dependencies)

All audit scripts in `scripts/audit/` and `scripts/` are **stateless**. They perform structural code inspection with no environment-driven configuration.

### Representative Scripts:

1. **`audit_features.sh`** (lines 1-25)
   - Verifies exactly 3 public features exist: `formats`, `strict`, `wasm4pm`
   - Fails if `nightly` feature exists
   - Exits 1 on failure; 0 on success

2. **`audit_no_algorithm_exports.sh`**
   - Scans `src/lib.rs` for pub-use exports of algorithm types
   - Fails if Miner/Checker/Replayer/Aligner are exported
   - No environment variables; pure grep-based validation

3. **`audit_graduation_boundaries.sh`**
   - Scans `src/lib.rs` for execution engine API exports
   - Fails if engine symbols found
   - No environment-driven logic

4. **`crown_audit_runner.sh`** (lines 1-35)
   - Orchestrates all audit scripts in sequence
   - Outputs summary table; exits 1 if any hard audit fails
   - No environment hooks; deterministic bash flow

**Pattern:** Scripts use `set -euo pipefail` for strict error handling. Exit codes are:
- `0` = all checks passed
- `1` = hard failure (ALIVE gate breaker)

---

## Feature Isolation (Build-Time Only)

### Per-Feature Module Gating

| Feature | Module | Gating Directive |
|---------|--------|------------------|
| `formats` | `src/formats.rs` | `#[cfg(feature = "formats")]` |
| `strict` | `src/strict.rs` | `#[cfg(feature = "strict")]` |
| `wasm4pm` | `src/engine_bridge.rs` | `#[cfg(feature = "wasm4pm")]` |
| `ts` | `src/ts/` | `#[cfg(feature = "ts")]` |
| `wasm` | `src/wasm/` | `#[cfg(feature = "wasm")]` |

**None of these gating directives** are conditional on environment variables. All feature resolution happens at Cargo manifest time.

---

## Examples & Benchmarks

### Examples (Feature-Gated by File)

Located in `examples/`:
- `basic_eventlog.rs` — no features required
- `basic_ocel.rs` — no features required
- `ocel_to_xes_projection.rs` — requires `formats` feature
- `strict_boundary_claim.rs` — requires `strict` feature
- `graduation_candidate.rs` — requires `wasm4pm` feature

**Run with:**
```bash
cargo run --example <name>
cargo run --example <name> --features <feature>
```

### Benchmarks

Located in `benches/`:
- `zero_cost_types.rs`
- `law_bounds_bench.rs`
- `evidence_lifecycle_bench.rs`
- `id_operations_bench.rs`

**Run with:**
```bash
cargo bench
```

No environment-driven profiling or conditional compilation in benchmarks.

---

## Testing Matrix (Cargo.toml-Driven)

### Default Profile

```bash
cargo build
cargo test --all-features --tests
cargo doc --no-deps
```

**Effect:**
- All features enabled (`formats`, `strict`, `wasm4pm`, `ts`, `wasm`)
- All tests run except `#[ignore]` marked tests
- No environment variables; deterministic Cargo feature resolution

### Minimal Canon (No Defaults)

```bash
cargo build --no-default-features
cargo test --no-default-features --tests
```

**Effect:**
- Only always-on modules compile
- No `formats`, `strict`, `wasm4pm`, `ts`, or `wasm` modules
- Verifies base type law is feature-independent

### Per-Feature Isolation

```bash
cargo build --no-default-features --features formats
cargo build --no-default-features --features strict
cargo build --no-default-features --features wasm4pm
```

**Effect:** Each feature compiles in isolation; no cross-feature leakage.

---

## No Runtime Environment Variables

### Search Results

A comprehensive scan of `src/` and `tests/` for runtime environment access:

```bash
grep -r "std::env\|getenv\|env::\|RUST_LOG\|CLAUDE_HOOKS\|NODE_OPTIONS\|PYTHONPATH\|JAVA_TOOL_OPTIONS" src/ tests/
```

**Result:** Zero matches (excluding docstring imports and path references).

**Implication:** The crate has no runtime behavior tuned by environment. All behavior is determined at compile time by:
1. **Nightly toolchain** (rust-toolchain.toml)
2. **Cargo features** (Cargo.toml)
3. **Conditional compilation directives** (#[cfg(...)] in source)

---

## CI/CD Integration Point

No CI/CD workflows (`.github/workflows/*.yml`) are present in the repository. The crate is designed to:

1. **Build deterministically** with `cargo build [--features ...]`
2. **Test deterministically** with `cargo test [--features ...] [--all-features]`
3. **Run ALIVE gate explicitly** with `cargo test --test ui_tests -- --ignored`
4. **Audit structural invariants** by calling `scripts/crown_audit_runner.sh`

All invocations are **environment-independent**. There are no environment-driven feature toggles, no conditional audit skipping, no RUST_LOG-based test filtering.

---

## Dockerfile & Container Patterns

No `Dockerfile`, `docker-compose.yml`, or `.dockerignore` files are present in the repository.

**Pattern for packaging this crate:**

```dockerfile
FROM rust:latest  # or rust:nightly specific

WORKDIR /build
COPY . .

# Nightly is pinned in rust-toolchain.toml — no need to override
RUN cargo build --release --all-features
RUN cargo test --all-features --tests
RUN cargo test --test ui_tests -- --ignored  # ALIVE gate
RUN ./scripts/crown_audit_runner.sh          # Audit mesh
```

**No environment configuration needed** in the container — the crate brings its own toolchain pin.

---

## Summary Table

| Hook Type | Name/Pattern | Scope | Environment-Driven? |
|-----------|--------------|-------|:---:|
| **Toolchain** | `rust-toolchain.toml` | Nightly unconditional | No |
| **Feature Gating** | `Cargo.toml` features | `formats`, `strict`, `wasm4pm`, `ts`, `wasm` | No (Cargo manifest) |
| **Module Gating** | `#[cfg(feature = "...")]` | Per-module export | No (compile-time) |
| **Test Ignore** | `#[ignore]` markers | ALIVE gate, docs audit | No (explicit invocation) |
| **Doctest Disable** | `Cargo.toml` line 17 | Default test run | No (manifest) |
| **Audit Scripts** | `scripts/audit/*.sh` | Structural validation | No (stateless) |
| **Runtime Env Vars** | (none found) | N/A | No (none used) |
| **Build Flags** | (none) | `.cargo/config.toml` | No (explicit: no flags) |

---

## Recommendations for Consumers

If you fork or depend on this crate:

1. **Pin the nightly date** — Use `rust-toolchain.toml` to lock a specific nightly, not just `"nightly"`.
2. **Test the feature matrix** — CI should run:
   ```bash
   cargo test --all-features
   cargo test --no-default-features
   cargo test --no-default-features --features formats
   cargo test --no-default-features --features strict
   cargo test --no-default-features --features wasm4pm
   ```
3. **Run ALIVE gate** — Make type-law receipts a gating check:
   ```bash
   cargo test --test ui_tests -- --ignored
   ```
4. **Run audit mesh** — Validate invariants before release:
   ```bash
   ./scripts/crown_audit_runner.sh
   ```
5. **Do NOT set RUST_LOG, RUSTFLAGS, or other Rust env vars** — They are unnecessary and may interfere with determinism.

---

## Conclusion

The `wasm4pm-compat` crate is **environment-minimal**. It achieves configuration entirely through:

1. **Toolchain pinning** (nightly)
2. **Explicit Cargo features** (no per-format flags)
3. **Feature-gated modules** (compile-time visibility)
4. **Marked test gates** (explicit invocation, not env-driven)

There are **no hidden hooks**, **no environment variables** that alter behavior, and **no build-time surprises**. The build is fully transparent, deterministic, and reproducible.
