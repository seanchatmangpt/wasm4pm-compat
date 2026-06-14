# Tutorial: Admitting Raw OCEL Evidence to the Process Court

This tutorial demonstrates how to wrap a raw Object-Centric Event Log (OCEL) in a typestate carrier, evaluate it against the `Ocel20` witness standard using a boundary admitter, and obtain a formal `Admission` or `Refusal` verdict.

## Learning Objectives

By the end of this tutorial, you will:
1. Wrap raw data in the `Evidence<T, Raw, W>` carrier.
2. Implement the `Admit` interface to validate the evidence.
3. Handle successful `Admission` type-state transitions.
4. Catch and analyze a structured `Refusal` containing a named law violation.

---

## Prerequisites

- Nightly Rust toolchain.
- `wasm4pm-compat` version `26.6.14` included in your workspace dependencies.

---

## Step 1: Set Up the Test Harness

Create a test file or add this to your main function. Import the required modules for evidence handling, admission, and the OCEL witness standard:

```rust
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::state::{Raw, Admitted};
use wasm4pm_compat::admission::{Admit, Admission, Refusal};
use wasm4pm_compat::witness::Ocel20;
use wasm4pm_compat::ocel::{OcelLog, OcelRefusal, Object, OcelEvent, EventObjectLink};
```

---

## Step 2: Initialize Raw Evidence

Before raw data can be evaluated, it must be loaded into the `Evidence` typestate carrier. The initial state is marked as `Raw`, and is parameterized by the witness standard it will answer to (in this case, `Ocel20`).

Let's construct a raw OCEL log and wrap it:

```rust
fn main() {
    // 1. Construct a structurally valid raw OCEL log model
    let raw_log = OcelLog::new(
        [Object::new("o1", "Order")],
        [OcelEvent::new("e1", "Register").at_ns(1_000)],
        [EventObjectLink::new("e1", "o1")],
        [],
        [],
    );

    // 2. Wrap it as Raw evidence targeting the Ocel20 standard
    let raw_evidence: Evidence<OcelLog, Raw, Ocel20> = Evidence::raw(raw_log);
    
    println!("Raw evidence wrapper minted.");
}
```

---

## Step 3: Execute the Admission Verdict

To transition raw evidence to `Admitted` state, you must pass it through the `Admit::admit()` function of a validator struct implementing the `Admit` trait. You cannot construct `Evidence<_, Admitted, _>` manually; it can only be minted via an `Admission`.

```rust
    // Define an Admitter struct that implements the Admit trait
    struct OcelAdmitter;

    impl Admit for OcelAdmitter {
        type Raw = OcelLog;
        type Admitted = OcelLog;
        type Reason = OcelRefusal;
        type Witness = Ocel20;

        fn admit(
            raw: Evidence<OcelLog, Raw, Ocel20>,
        ) -> Result<Admission<OcelLog, Ocel20>, Refusal<OcelRefusal, Ocel20>> {
            // Evaluate structural laws
            match raw.value.validate() {
                Ok(()) => Ok(Admission::new(raw.value)),
                Err(e) => Err(Refusal::new(e)),
            }
        }
    }

    // Attempt to admit the evidence
    let verdict = OcelAdmitter::admit(raw_evidence);
```

---

## Step 4: Handle the Admission and Refusal Paths

The `admit` function returns a `Result`. If validation succeeds, you receive an `Admission<T, W>`. If validation fails, you receive a structured `Refusal<R, W>` carrying a domain-specific refusal enum:

```rust
    match verdict {
        Ok(admission) => {
            // Transition the Admission wrapper to the Admitted Evidence type
            let admitted_evidence: Evidence<OcelLog, Admitted, Ocel20> = 
                admission.into_evidence();
            println!("Success! Evidence has been admitted under the Ocel20 standard.");
        }
        Err(refusal) => {
            // A refusal carries the named law violation (e.g. DanglingEventObjectLink)
            println!("Refusal! The evidence violated a structural law.");
            println!("Reason: {:?}", refusal.reason);
        }
    }
```

---

## Complete Example

Here is a full compile-passing harness:

```rust
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::state::{Raw, Admitted};
use wasm4pm_compat::admission::{Admit, Admission, Refusal};
use wasm4pm_compat::witness::Ocel20;
use wasm4pm_compat::ocel::{OcelLog, OcelRefusal, OcelEvent, Object, EventObjectLink};

struct OcelAdmitter;

impl Admit for OcelAdmitter {
    type Raw = OcelLog;
    type Admitted = OcelLog;
    type Reason = OcelRefusal;
    type Witness = Ocel20;

    fn admit(
        raw: Evidence<OcelLog, Raw, Ocel20>,
    ) -> Result<Admission<OcelLog, Ocel20>, Refusal<OcelRefusal, Ocel20>> {
        match raw.value.validate() {
            Ok(()) => Ok(Admission::new(raw.value)),
            Err(e) => Err(Refusal::new(e)),
        }
    }
}

fn main() {
    let log = OcelLog::new(
        [Object::new("o1", "Order")],
        [OcelEvent::new("e1", "Register").at_ns(1_000)],
        [EventObjectLink::new("e1", "o1")],
        [],
        [],
    );
    
    let raw_evidence = Evidence::raw(log);

    match OcelAdmitter::admit(raw_evidence) {
        Ok(admission) => {
            let admitted_evidence = admission.into_evidence();
            println!("Admitted successfully! Size of payload: {} bytes", 
                     std::mem::size_of_val(&admitted_evidence.value));
        }
        Err(refusal) => {
            eprintln!("Admit Refused: {:?}", refusal.reason);
        }
    }
}
```

---

## Next Steps

Now that you know how to admit evidence to the court, proceed to the [Loss Policy Projection Tutorial](loss-policy-projection.md) to learn how to transform admitted evidence via named lossy projections.
