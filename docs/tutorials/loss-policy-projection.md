# Tutorial: Loss Policy Projection of Event Logs

This tutorial teaches you how to project/export an Object-Centric Event Log (OCEL) to a case-centric XML/XES format while enforcing a `LossPolicy` to account for discarded data.

## Learning Objectives

By the end of this tutorial, you will:
1. Understand why projecting object-centric logs to case-centric traces is lossy.
2. Select a `LossPolicy` strategy to govern the projection/export.
3. Execute the projection via an `ExportFormat` implementation.
4. Inspect the resulting `LossReport` to account for discarded elements.

---

## Prerequisites

- Nightly Rust toolchain.
- `wasm4pm-compat` version `26.6.4` included in your project, with the default `formats` feature enabled.

---

## The Concept of Lossy Projection

In process mining, converting an OCEL log (where events are linked to multiple heterogeneous objects) into an XES log (which has a single, flat case notion) is inherently lossy. Relationships between concurrent objects are dropped. Under the "no format laundering" covenant, you must declare a policy and inspect the report to account for this loss.

---

## Step 1: Import Loss-Control Modules

Import the format/projection traits, policies, and identifiers:

```rust
use wasm4pm_compat::formats::{ExportFormat, FormatExport, FormatKind};
use wasm4pm_compat::loss::{LossPolicy, LossReport, ProjectionName};
```

---

## Step 2: Establish the Loss Policy and Implement ExportFormat

You can configure three strategies under the `LossPolicy` enum:
- `LossPolicy::RefuseLoss`: Block the projection if any data is dropped.
- `LossPolicy::AllowNamedProjection`: Allow the projection if it matches a pre-approved static query shape.
- `LossPolicy::AllowLossWithReport`: Allow the projection but write a detailed record of discarded items.

Below is an exporter struct implementing `ExportFormat` that projects a simplified admitted OCEL to XES, checking the `LossPolicy`:

```rust
pub struct AdmittedOcel {
    pub object_types: Vec<&'static str>,
    pub case_type: &'static str,
}

#[derive(Debug, PartialEq, Eq)]
pub enum XesExportRefusal {
    FlatteningLoss,
}

pub struct OcelToXesExporter;

impl ExportFormat for OcelToXesExporter {
    type Source = AdmittedOcel;
    type Reason = XesExportRefusal;

    fn export(src: &Self::Source, policy: LossPolicy) -> Result<FormatExport, Self::Reason> {
        let dropped: Vec<String> = src
            .object_types
            .iter()
            .filter(|t| **t != src.case_type)
            .map(|t| format!("dropped_object_type={t}"))
            .collect();

        let bytes = format!("<log case-type=\"{}\"/>", src.case_type).into_bytes();

        if dropped.is_empty() {
            // Single object type: lossless flattening.
            Ok(FormatExport::lossless(FormatKind::XesXml, bytes))
        } else if policy == LossPolicy::RefuseLoss {
            // Forbidden loss becomes a named refusal — never a silent flatten.
            Err(XesExportRefusal::FlatteningLoss)
        } else {
            let report = LossReport::<(), (), Vec<String>>::new(
                ProjectionName("ocel-flatten-to-xes:by-case"),
                policy,
                dropped,
            );
            Ok(FormatExport::lossy(FormatKind::XesXml, bytes, report))
        }
    }
}
```

---

## Step 3: Run the Projection

Call `export()` on your exporter. This method accepts the source object and the chosen policy.

```rust
fn project_log(admitted_ocel: &AdmittedOcel) {
    // Execute projection
    let result = OcelToXesExporter::export(admitted_ocel, LossPolicy::AllowLossWithReport);
    
    match result {
        Ok(export) => {
            println!("Projection completed successfully.");
            
            // Inspect the loss report if it is lossy
            if let Some(report) = export.loss {
                inspect_report(report);
            } else {
                println!("Lossless projection.");
            }
        }
        Err(e) => {
            println!("Projection failed: {:?}", e);
        }
    }
}
```

---

## Step 4: Inspect the Loss Report

The `LossReport` contains the static projection metadata and the list of dropped components:

```rust
fn inspect_report(report: LossReport<(), (), Vec<String>>) {
    println!("--- LOSS ACCOUNTABILITY REPORT ---");
    println!("Projection Identity: {}", report.projection.0);
    println!("Applied Policy: {:?}", report.policy);
    println!("Dropped Elements:");
    for item in report.lost {
        println!(" - {}", item);
    }
}
```

---

## Complete Example

Here is a full compile-passing harness:

```rust
use wasm4pm_compat::formats::{ExportFormat, FormatExport, FormatKind};
use wasm4pm_compat::loss::{LossPolicy, LossReport, ProjectionName};

pub struct AdmittedOcel {
    pub object_types: Vec<&'static str>,
    pub case_type: &'static str,
}

#[derive(Debug, PartialEq, Eq)]
pub enum XesExportRefusal {
    FlatteningLoss,
}

pub struct OcelToXesExporter;

impl ExportFormat for OcelToXesExporter {
    type Source = AdmittedOcel;
    type Reason = XesExportRefusal;

    fn export(src: &Self::Source, policy: LossPolicy) -> Result<FormatExport, Self::Reason> {
        let dropped: Vec<String> = src
            .object_types
            .iter()
            .filter(|t| **t != src.case_type)
            .map(|t| format!("dropped_object_type={t}"))
            .collect();

        let bytes = format!("<log case-type=\"{}\"/>", src.case_type).into_bytes();

        if dropped.is_empty() {
            Ok(FormatExport::lossless(FormatKind::XesXml, bytes))
        } else if policy == LossPolicy::RefuseLoss {
            Err(XesExportRefusal::FlatteningLoss)
        } else {
            let report = LossReport::<(), (), Vec<String>>::new(
                ProjectionName("ocel-flatten-to-xes:by-case"),
                policy,
                dropped,
            );
            Ok(FormatExport::lossy(FormatKind::XesXml, bytes, report))
        }
    }
}

fn main() {
    let ocel = AdmittedOcel {
        object_types: vec!["order", "item", "delivery"],
        case_type: "order",
    };

    println!("[1] Attempting export with RefuseLoss policy:");
    match OcelToXesExporter::export(&ocel, LossPolicy::RefuseLoss) {
        Ok(_) => unreachable!(),
        Err(e) => println!("Refused as expected: {:?}", e),
    }

    println!("\n[2] Attempting export with AllowLossWithReport policy:");
    match OcelToXesExporter::export(&ocel, LossPolicy::AllowLossWithReport) {
        Ok(export) => {
            println!("Export succeeded (lossy).");
            if let Some(report) = export.loss {
                println!("Projection Name: {}", report.projection);
                println!("Lost items count: {}", report.lost.len());
                for item in report.lost {
                    println!("  - Lost item: {}", item);
                }
            }
        }
        Err(e) => eprintln!("Export failed: {:?}", e),
    }
}
```

---

## Next Steps

Now that you have completed the tutorials, explore the [How-To Guides](../how-to/verify-the-crate.md) to understand specific tasks like running the verification gate and use of strict boundaries.

