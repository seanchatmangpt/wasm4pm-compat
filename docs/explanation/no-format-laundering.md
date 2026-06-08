# Explanation: The "No Format Laundering" Covenant

This document explains the conceptual underpinnings and technical implementation of the "no format laundering" covenant in `wasm4pm-compat` version `26.6.8`.

---

## What is Format Laundering?

Format laundering is the practice of silently converting process-mining logs between different schemas (for example, projecting an object-centric event log to a flat case-centric trace log) while ignoring or dropping data, relations, and qualifiers without reporting the loss.

In process intelligence, this leads to severe issues:
1. **Misleading Conformance Scores**: If attributes or concurrent links are silently discarded, subsequent conformance checking and alignments may show 100% compliance when the underlying data actually contained violations.
2. **Untraceable Audits**: Auditors cannot verify whether the flat event log represents a faithful or filtered projection of the original system history.
3. **Implicit Semantics**: Converting types implicitly shifts assumptions about the process boundaries (such as shifting from multi-perspective lifecycle tracking to flat trace-case identifiers) without documenting the change.

---

## The Covenant: Absolute Transparency

`wasm4pm-compat` implements the **"no format laundering"** covenant: *No model transformation or projection may occur silently. Every loss of structure or attribute metadata must be explicitly authorized by a policy and cataloged in an auditable report.*

```
+------------------------+
|   Original OCEL Log    |  (Heterogeneous, multi-object links)
+-----------+------------+
            |
            |  .project(ProjectionName, LossPolicy)
            v
+-----------+------------+
|   Target XES Trace Log  |  (Flat, case-centric)
+-----------+------------+
            +
+-----------+------------+
|       Loss Report      |  (Catalog of dropped attributes/links)
+------------------------+
```

---

## Technical Enforcement in the Crate

The `formats` feature enforces this covenant at the API level through three mechanisms:

### 1. Mandatory Loss Policy
The `Project` trait requires the caller to supply a `LossPolicy` parameter:
```rust
let (flat_log, report) = admitted_evidence.project::<EventLog>(
    ProjectionName("ocel-to-xes-case-customer"),
    LossPolicy::AllowLossWithReport
)?;
```
If you choose `LossPolicy::RefuseLoss`, the projection blocks compilation or execution if any data loss occurs, returning a `ProjectionError::FlatteningLoss`.

### 2. Monad-Style Return Envelope
You cannot obtain the projected event log without unpacking the tuple. The `.project()` method returns both the projected evidence and a `LossReport`:
```rust
Result<(Evidence<To, Projected, W>, LossReport<From, To, Items>), ProjectionError>
```
The compiler prevents you from silently discarding the report, forcing your application code to handle or store the audit ledger.

### 3. Static Projection Naming
Every projection requires a static, hardcoded `ProjectionName` string (e.g. `ProjectionName("ocel-to-xes-case-customer")`). This ensures that transformations are named explicitly in code and configuration files, preventing dynamic or untracked ad-hoc flattening.
