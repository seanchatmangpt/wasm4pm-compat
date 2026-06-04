# How-To: Enforcing Opt-in Strict Boundaries

This guide demonstrates how to configure, write, and verify opt-in strict boundary declarations in `wasm4pm-compat` version `26.6.4` to validate process-evidence metadata.

---

## What is a Strict Boundary?

A strict boundary evaluates *process declarations* rather than raw event-log data. When you opt into strict mode, you declare your boundaries (such as import, export, or projection endpoints) and assert that they conform to covenants (such as declaring a loss policy and registering a refusal path). If you claim unauthorized capabilities (such as running engine conformance checks inside the compat layer), the boundary raises strict violations.

---

## Step 1: Enable the Strict Feature

In your `Cargo.toml`, enable the `strict` feature flag:

```toml
[dependencies]
wasm4pm-compat = { version = "0.1.0", features = ["strict"] }
```

---

## Step 2: Initialize a Process Boundary Declaration

Import the strict types and declare a process boundary. You must specify the boundary kind (e.g. `Export` or `Import` or `Projection`):

```rust
use wasm4pm_compat::strict::{ProcessBoundary, ProcessBoundaryKind, StrictViolation};

fn main() {
    // Construct a process boundary for evidence export
    let mut boundary = ProcessBoundary::new(
        "customer-billing-export".to_string(),
        ProcessBoundaryKind::Export,
    );
    
    println!("Boundary initialized.");
}
```

---

## Step 3: Configure strict boundary properties

To satisfy strict covenants, the boundary declaration must specify its loss policy and register its refusal handling path:

```rust
    // Set the registered loss policy
    boundary = boundary.with_loss_policy(
        wasm4pm_compat::loss::LossPolicy::AllowLossWithReport
    );

    // Assert that a structured refusal handler is present
    boundary = boundary.with_refusal_handler("CustomerBillingRefusalFormatter");
```

---

## Step 4: Run the Compliance Check

Invoke `.check()` to verify the declaration against the strict-mode covenants:

```rust
    match boundary.check() {
        Ok(_) => {
            println!("Boundary compliance check passed!");
        }
        Err(violations) => {
            println!("Compliance violations found:");
            for violation in violations {
                match violation {
                    StrictViolation::MissingLossPolicy => {
                        println!(" - Violation: Loss policy must be explicitly configured.");
                    }
                    StrictViolation::MissingRefusalPath => {
                        println!(" - Violation: Refusal handler must be registered.");
                    }
                    StrictViolation::HiddenProcessMiningGrowth => {
                        println!(" - Violation: Boundary claims engine replay or discovery capabilities (not allowed in compat).");
                    }
                }
            }
        }
    }
```

---

## Complete Example

Here is a full compile-passing code snippet:

```rust
use wasm4pm_compat::strict::{ProcessBoundary, ProcessBoundaryKind, StrictViolation};
use wasm4pm_compat::loss::LossPolicy;

fn main() {
    // 1. A fully-compliant boundary declaration
    let compliant_boundary = ProcessBoundary::new("compliant-egress", ProcessBoundaryKind::Export)
        .with_loss_policy(LossPolicy::RefuseLoss)
        .with_refusal_handler("EgressHandler");
        
    assert!(compliant_boundary.check().is_ok());

    // 2. An invalid boundary (violates strict covenants)
    let non_compliant = ProcessBoundary::new("unconfigured-egress", ProcessBoundaryKind::Export);
    
    match non_compliant.check() {
        Ok(_) => panic!("Should fail compliance check!"),
        Err(violations) => {
            println!("Correctly caught {} violations.", violations.len());
            // (structure only — strict boundaries graduate to wasm4pm)
        }
    }
}
```
