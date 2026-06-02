# AGENT 9 — Cross-Project Validation System

**Purpose:** Comprehensive, non-destructive cross-project security and compliance validation for the wasm4pm-compat ecosystem.

**Agent Role:** AGENT_9_VALIDATION  
**Scope:** Read-only, automated compliance checking  
**Outputs:** YAML receipts, JSON evidence logs, Markdown reports

---

## Quick Start

### Run Full Validation
```bash
bash scripts/validate_cross_project.sh
```

### Run Individual Checks
```bash
bash scripts/check_no_live_trading.sh
bash scripts/check_no_runtime_llm.sh
bash scripts/check_public_ip_boundary.sh
bash scripts/emit_receipts.sh
```

### View Receipts
```bash
cat receipts/validation_receipt.yaml
cat receipts/validation_receipt_detailed.json
cat AGENT_REPORTS/AGENT_09_VALIDATION_RECEIPTS.md
```

---

## Validation Checks

### 1. No Live Trading Surface (`check_no_live_trading.sh`)

**Purpose:** Ensure zero live trading, broker, exchange, or wallet APIs in source code.

**Patterns Checked:**
- `broker`, `exchange api`, `websocket`, `alpaca`, `interactive brokers`, `binance`, `coinbase`, `kraken`
- `fix protocol`, `order submit`, `wallet`, `private key`, `custodian`

**Search Scope:**
- Source files: `*.rs`, `*.toml` (excluding `target/`, `.git/`)
- Documentation: `*.md` (DOCS_ONLY acceptable)

**Classification:**
- `ABSENT`: No patterns found → **PASS** ✓
- `DOCS_ONLY`: Patterns in documentation only → **PASS** ✓ (acceptable for examples)
- `SOURCE_RISK`: Patterns in source code → **FAIL** ✗

**Exit Code:** 0 if ABSENT or DOCS_ONLY; 1 if SOURCE_RISK

---

### 2. No Runtime LLM Integration (`check_no_runtime_llm.sh`)

**Purpose:** Verify no LLM runtime clients, API integrations, or language model dependencies.

**Patterns Checked:**
- `openai`, `anthropic`, `claude`, `llm`, `chatcompletion`, `messages.create`, `api key` (runtime context)

**Search Scope:**
- Cargo.toml: `[dependencies]` sections only
- Runtime code: `*.rs` (excluding tests, examples, comments)
- Documentation: `*.md` (DOCS_ONLY acceptable)

**Classification:**
- `ABSENT`: No patterns found → **PASS** ✓
- `DOCS_ONLY`: Patterns in documentation only → **PASS** ✓ (references, examples)
- `SOURCE_RISK`: In dependencies or runtime code → **FAIL** ✗

**Exit Code:** 0 if ABSENT or DOCS_ONLY; 1 if SOURCE_RISK

---

### 3. Public IP Boundary (`check_public_ip_boundary.sh`)

**Purpose:** Ensure no private IP ranges or internal hostnames leak into public documentation.

**Patterns Checked:**
- **Private IP ranges:** `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`
- **Internal hostnames:** `*.internal`, `*.local`, `localhost` (outside examples)
- **Secret patterns:** `api key`, `api secret`, `password=`, `token=`, `credential`

**Search Scope:**
- Documentation only: `docs/` directory
- Allowed: RFC 1918 examples with "example" or "demonstration" context

**Classification:**
- `ABSENT`: No private IPs/hostnames → **PASS** ✓
- `DOCS_ACCEPTABLE`: Found with acceptable context → **PASS** ✓
- `SOURCE_RISK`: Private IPs in public docs without context → **FAIL** ✗

**Exit Code:** 0 if clean; 1 if SOURCE_RISK

---

### 4. Workspace Integrity

**Purpose:** Verify all required Cargo.toml manifests present and accessible.

**Checks:**
- Main crate: `/Cargo.toml`
- Workspace members:
  - `c8-adversary/Cargo.toml`
  - `c8-market/Cargo.toml`
  - `c8-time/Cargo.toml`
  - `c8-receipts/Cargo.toml`
  - `c8-instruments/Cargo.toml`

**Result:** All manifests present → **PASS** ✓

---

### 5. Type Law Compliance

Validates nightly-enforced invariants required by the wasm4pm-compat architecture.

#### 5a. Forbid Unsafe Code
```rust
#![forbid(unsafe_code)]
```
- Location: `src/lib.rs`
- Enforcement: Compiler-level, no exceptions
- Status: ✓ ENFORCED

#### 5b. Nightly Toolchain Pinning
```toml
[toolchain]
channel = "nightly"
```
- Location: `rust-toolchain.toml`
- Fallback: None (nightly-only)
- Status: ✓ ENFORCED

#### 5c. Nightly Features Declared
Required in `src/lib.rs`:
```rust
#![feature(generic_const_exprs)]
#![feature(adt_const_params)]
#![feature(const_trait_impl)]
#![feature(min_specialization)]
#![feature(portable_simd)]
#![allow(incomplete_features)]
```
- Status: ✓ DECLARED

#### 5d. Feature Model Compliance
Exactly 3 public features:
- `formats` (default) — Import/export contracts
- `strict` (opt-in) — Boundary judgment surfaces
- `wasm4pm` (opt-in) — Graduation bridge traits

Optional features:
- `ts` — TypeScript projection
- `wasm` — WASM boundary projection

Status: ✓ COMPLIANT (3-feature model enforced)

#### 5e. Doctest Disabled by Default
```toml
[lib]
doctest = false
```
- Location: `Cargo.toml`
- Rationale: Avoids 200+ nightly rustc invocations (per-doctest feature flags)
- Explicit opt-in: `cargo test --doc --all-features`
- Status: ✓ ENFORCED

---

### 6. Dependency Scope

**Purpose:** Verify no heavy async runtimes or ORMs in unconditional dependencies.

**Forbidden in base profile:**
- Async runtimes: `tokio`, `actix`, `hyper`, `tonic`
- ORMs: `sqlx`, `sea-orm`, `diesel`

**Allowed:**
- Serialization: `serde` (optional, feature-gated)
- Type reflection: `specta`, `tsify` (feature-gated)
- WASM bindings: `wasm-bindgen` (feature-gated)

**Result:** No heavy runtimes found → **PASS** ✓

---

## Receipt Structure

### validation_receipt.yaml
Structured compliance summary with all check results.

```yaml
---
receipt_id: validation-cross-project-<timestamp>
agent: AGENT_9_VALIDATION
timestamp: 2026-06-02T05:14:26Z
project_root: /Users/sac/wasm4pm-compat

validation_summary:
  total_checks: 10
  passed: 10
  failed: 0
  warnings: 0

checks:
  - name: no_live_trading
    status: pass
  - name: no_runtime_llm
    status: pass
  # ... more checks
```

### validation_receipt_detailed.json
Full JSON evidence log with detailed findings per category.

```json
{
  "receipt": {
    "id": "validation-receipt-<timestamp>-<hash>",
    "version": "1.0",
    "timestamp": "2026-06-02T05:14:26Z",
    "agent": "AGENT_9_VALIDATION"
  },
  "validation_summary": {
    "total_checks": 10,
    "categories": {
      "security_boundary": { ... },
      "manifest_integrity": { ... },
      "type_law_compliance": { ... }
    }
  },
  "compliance_matrix": { ... },
  "certification": { ... }
}
```

### AGENT_09_VALIDATION_RECEIPTS.md
Human-readable Markdown report with detailed explanations.

---

## Exit Codes

| Code | Meaning |
|------|---------|
| `0` | All validations **PASS** — codebase compliant |
| `1` | At least one validation **FAIL** — defects found |

### Individual Script Exit Codes

Each check script follows this convention:
- **0:** Check passed (ABSENT or acceptable DOCS_ONLY)
- **1:** Check failed (SOURCE_RISK or defect found)

---

## Compliance Rules

### Trading Surface Rule
```
SOURCE_RISK patterns in *.rs or *.toml → FAIL
DOCS_ONLY patterns (documentation examples) → PASS
ABSENT patterns → PASS
```

### Runtime LLM Rule
```
Dependencies or runtime code with LLM patterns → FAIL
DOCS_ONLY references (acceptable) → PASS
ABSENT patterns → PASS
```

### Public IP Boundary Rule
```
Private IPs/hostnames in docs WITHOUT example context → FAIL
Private IPs/hostnames WITH example/demo context → PASS
ABSENT patterns → PASS
```

---

## Integration Points

### CI/CD Integration
Add to GitHub Actions:

```yaml
- name: Run AGENT 9 Cross-Project Validation
  run: bash scripts/validate_cross_project.sh
  
- name: Archive validation receipts
  if: always()
  uses: actions/upload-artifact@v3
  with:
    name: agent-9-receipts
    path: |
      receipts/validation_receipt*.{yaml,json}
      AGENT_REPORTS/AGENT_09_VALIDATION_RECEIPTS.md
```

### Pre-Commit Hook
```bash
#!/bin/bash
bash scripts/validate_cross_project.sh || exit 1
```

### Release Gate
Validation receipts must pass before:
- Releasing to crates.io
- Tagging releases
- Publishing documentation

---

## Troubleshooting

### Why do I see DOCS_ONLY?
The validation found patterns (e.g., "broker", "order") in documentation. This is **acceptable** because:
- Documentation may reference process mining concepts
- Examples use realistic domain language
- Trust boundary is source code, not documentation

To investigate:
```bash
grep -r "broker" docs/
```

### Why is check_public_ip_boundary.sh ABSENT?
The docs directory may not exist, or no private IP patterns were detected. This is correct — the check is documentation-scoped only.

### How do I add new patterns?
Edit the corresponding check script:
- `FORBIDDEN_PATTERNS` array in `check_no_live_trading.sh`
- `FORBIDDEN_LLM_PATTERNS` array in `check_no_runtime_llm.sh`
- Pattern lists in `check_public_ip_boundary.sh`

### Why does validate_cross_project.sh take ~5 seconds?
It runs 10 independent checks with full codebase scans. This is expected for comprehensive validation.

---

## Design Philosophy

### Non-Destructive
All scripts are read-only. They never modify, delete, or create files except receipts in designated directories.

### Graduated Risk Levels
- **ABSENT:** Pattern not found anywhere (best)
- **DOCS_ONLY:** Pattern in documentation only (acceptable)
- **SOURCE_RISK:** Pattern in source code (requires remediation)

### Evidence-First
Every PASS or FAIL includes:
- Patterns searched
- Scope (files/directories checked)
- Classification (ABSENT, DOCS_ONLY, SOURCE_RISK)

### Receipt-Sealed
Validation receipts are immutable attestations:
- YAML receipt for automation
- JSON receipt with full evidence
- Markdown report for humans

---

## References

- **Crate:** `wasm4pm-compat`
- **Nightly requirement:** See `CLAUDE.md`
- **Type law surfaces:** See `NIGHTLY_TYPE_LAW.md`
- **Feature model:** See `Cargo.toml [features]` section
- **Agent role:** AGENT_9_VALIDATION (this system)

---

## Status

**Current Validation:** ✓ ALL PASS  
**Certification Level:** PRODUCTION_READY  
**Last Audit:** 2026-06-02T05:14:26Z  
**Recommended Next Audit:** 2026-07-02 (30 days)
