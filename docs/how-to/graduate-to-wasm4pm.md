# How-To: Graduating to the wasm4pm Execution Engine

This guide demonstrates how to declare a graduation candidate, implement the `GraduateToWasm4pm` trait, and bridge your process-evidence data from the structure-only compat layer to the `wasm4pm` execution engine.

---

## Why Graduate?

`wasm4pm-compat` is strictly **structure-only**. It contains no algorithms to discover models, compute alignments, or simulate token replay. To run these operations, you must transition your evidence shapes through the graduation seam into the active execution engine, `wasm4pm`.

---

## Step 1: Enable the wasm4pm Feature Flag

To compile the graduation traits and structures, enable the `wasm4pm` feature flag:

```toml
[dependencies]
wasm4pm-compat = { version = "26.6.14", features = ["wasm4pm"] }
```

---

## Step 2: Implement the GraduateToWasm4pm Trait

Implement the single-method `GraduateToWasm4pm` trait on your host structure. The method `candidate()` returns a `GraduationCandidate` describing what is graduating and why.

```rust
use wasm4pm_compat::engine_bridge::{GraduateToWasm4pm, GraduationCandidate, GraduationReason};
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::state::Admitted;
use wasm4pm_compat::witness::Ocel20;
use wasm4pm_compat::ocel::OcelLog;

// 1. Your application context carrying process evidence.
struct BillingAuditSession {
    session_id: String,
    evidence_hash: String,
    evidence: Evidence<OcelLog, Admitted, Ocel20>,
}

// 2. Implement the graduation trait.
impl GraduateToWasm4pm for BillingAuditSession {
    fn candidate(&self) -> GraduationCandidate {
        GraduationCandidate::new(
            GraduationReason::NeedsDiscovery,
            format!("session-{}", self.session_id),
            self.evidence_hash.clone(),
        )
    }
}
```

`GraduationCandidate::new` takes three arguments in order:
1. **`reason: GraduationReason`** — why this evidence needs the engine.
2. **`subject: impl Into<String>`** — a human-readable name for the subject being graduated.
3. **`evidence_ref: impl Into<String>`** — a stable reference (e.g. a BLAKE3 hash or URI) to the grounding evidence. Must be non-empty for the candidate to be considered grounded.

---

## Step 3: Inspect the Candidate

```rust
fn handoff(session: &BillingAuditSession) {
    let candidate = session.candidate();

    println!("Reason:   {:?} (hard signal: {})", candidate.reason, candidate.reason.is_hard_signal());
    println!("Subject:  {}", candidate.subject);
    println!("Evidence: {}", candidate.evidence_ref);
    println!("Grounded: {}", candidate.is_grounded());

    // Hand the candidate to the wasm4pm engine intake.
    // (structure only — wasm4pm executes judgment after graduation)
}
```

`candidate.is_grounded()` returns `true` when both `subject` and `evidence_ref` are non-empty. The engine intake should reject ungrounded candidates.

---

## GraduationReason Variants

| Variant | `is_hard_signal()` | When to use |
|---|:---:|---|
| `NeedsDiscovery` | yes | No process model exists yet |
| `NeedsConformanceExecution` | yes | A model exists but fitness is unmeasured |
| `NeedsBenchmarkGate` | yes | Alignment cost untested against baselines |
| `NeedsObjectCentricQueryExecution` | yes | OCPQ queries not yet executed against the log |
| `NeedsReplay` | no | Replay is stale or missing |
| `RebuildingProcessMiningLocally` | no | Local rebuild required before graduation |
| `NeedsReceipts` | no | Provenance receipts absent or incomplete |

Hard-signal reasons indicate the evidence cannot proceed without engine action. Non-hard-signal reasons are advisory.

Use `reason.tag()` to get a short string label (e.g. `"needs-discovery"`) for logging or serialization.

---

## Complete Example

```rust
use wasm4pm_compat::engine_bridge::{GraduateToWasm4pm, GraduationCandidate, GraduationReason};

struct PendingConformance {
    log_id: String,
    log_hash: String,
}

impl GraduateToWasm4pm for PendingConformance {
    fn candidate(&self) -> GraduationCandidate {
        GraduationCandidate::new(
            GraduationReason::NeedsConformanceExecution,
            self.log_id.clone(),
            self.log_hash.clone(),
        )
    }
}

fn main() {
    let host = PendingConformance {
        log_id: "billing-log-2026-06".to_string(),
        log_hash: "blake3:abc123def456".to_string(),
    };

    let c = host.candidate();
    assert_eq!(c.reason, GraduationReason::NeedsConformanceExecution);
    assert!(c.is_grounded());
    assert!(c.reason.is_hard_signal());
}
```
