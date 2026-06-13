# How-To: Graduating to the wasm4pm Execution Engine

This guide demonstrates how to declare a graduation candidate, implement the `GraduateToWasm4pm` trait, and bridge your process-evidence data from the structure-only migrated to the `wasm4pm` execution engine in version `26.6.13`.

---

## Why Graduate?

`wasm4pm-compat` is strictly **structure-only**. It contains no algorithms to discover models, compute alignments, or simulate token replay. To run these operations, you must transition your evidence shapes through the graduation seam into the active execution engine, `wasm4pm`.

---

## Step 1: Enable the wasm4pm Feature Flag

To compile the graduation traits and structures, enable the `wasm4pm` feature flag:

```toml
[dependencies]
wasm4pm-compat = { version = "26.6.13", features = ["wasm4pm"] }
```

---

## Step 2: Implement the GraduateToWasm4pm Trait

You graduate your host structures (e.g. your database schemas or application states carrying event logs) by implementing the `GraduateToWasm4pm` trait.

```rust
use wasm4pm_compat::engine_bridge::{GraduateToWasm4pm, GraduationCandidate, GraduationReason};
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::state::Admitted;
use wasm4pm_compat::witness::Ocel20;
use wasm4pm_compat::ocel::OcelLog;

// 1. Declare your application context that holds process evidence
struct BillingAuditSession {
    session_id: String,
    evidence: Evidence<OcelLog, Admitted, Ocel20>,
}

// 2. Implement the graduation trait on your context
impl GraduateToWasm4pm for BillingAuditSession {
    fn name(&self) -> String {
        format!("session-{}", self.session_id)
    }

    fn reason(&self) -> GraduationReason {
        // Specify why this data is transitioning to the engine
        GraduationReason::NeedsDiscovery
    }

    fn package_candidate(&self) -> Result<GraduationCandidate, String> {
        // Build the graduation candidate envelope
        let mut candidate = GraduationCandidate::new(self.name(), self.reason());
        
        // Link the admitted evidence reference to the candidate
        candidate = candidate.with_evidence_ref("ref://billing-session-evidence-01");
        
        Ok(candidate)
    }
}
```

---

## Step 3: Package the Candidate

Construct and inspect the candidate envelope:

```rust
fn execute_graduation(session: BillingAuditSession) {
    match session.package_candidate() {
        Ok(candidate) => {
            println!("Graduation Candidate Packaged:");
            println!(" - Name: {}", candidate.name);
            println!(" - Reason: {:?}", candidate.reason);
            println!(" - Evidence Reference: {:?}", candidate.evidence_ref);
            
            // The candidate envelope is now ready to be handed to the wasm4pm engine intake.
            // (structure only — wasm4pm will execute judgment after graduation)
        }
        Err(e) => {
            println!("Graduation packaging failed: {}", e);
        }
    }
}
```

---

## Complete Example

Here is a full compile-passing snippet demonstrating graduation candidate construction:

```rust
use wasm4pm_compat::engine_bridge::{GraduateToWasm4pm, GraduationCandidate, GraduationReason};

struct SimpleCandidate {
    name: String,
}

impl GraduateToWasm4pm for SimpleCandidate {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn reason(&self) -> GraduationReason {
        GraduationReason::NeedsConformanceExecution
    }

    fn package_candidate(&self) -> Result<GraduationCandidate, String> {
        let mut candidate = GraduationCandidate::new(self.name(), self.reason());
        candidate = candidate.with_evidence_ref("ref://simple-evidence");
        Ok(candidate)
    }
}

fn main() {
    let host = SimpleCandidate { name: "test-run-001".to_string() };
    let candidate = host.package_candidate().unwrap();
    
    assert_eq!(candidate.name, "test-run-001");
    println!("Candidate package verified. (structure only — graduates to wasm4pm)");
}
```
