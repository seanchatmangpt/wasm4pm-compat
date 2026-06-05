# LSP Rules: Evidence Lifecycle & Format Laundering

This document specifies the diagnostic engine rules for the LSP compiler/linter. These rules enforce the structural integrity of the `wasm4pm-compat` boundary, preventing evidence lifecycle misuse (**W4PM-EVD**) and format laundering (**W4PM-FMT**).

---

## 1. W4PM-EVD: Evidence Lifecycle Verification

### Doctrine
Evidence has a strict, typestate-enforced lifecycle. Unvalidated (unadmitted) evidence is represented as `Raw` or `Parsed`. Before evidence can be projected, exported, or receipted, it **must** be promoted to `Admitted` via an `Admit::admit` implementation. Direct projection or export of `Raw` or `Parsed` evidence bypasses crucial validation gates, which constitutes evidence lifecycle misuse.

```
[Raw] ──into_parsed()──▶ [Parsed]
  │                        │
  │                     (Admit::admit)
  │                        │
  ▼                        ▼
[Refused]               [Admitted] ──into_projected()──▶ [Projected]
                           │                                │
                           ├──into_exportable()─────────────┤
                           │                                │
                           ▼                                ▼
                      [Exportable]                     [Receipted]
```

### Threat Model & Violations
1. **Raw Projection**: Attempting to project `Raw` or `Parsed` evidence directly using the `Project` trait or method calls.
2. **Raw Export**: Exporting `Raw` or `Parsed` evidence without passing through the admission gate.
3. **Invalid Typestate Transitions**: Calling `into_exportable()`, `into_projected()`, or `into_receipted()` on types that are not `Admitted` or `Projected`.

---

### Detection Specifications

#### A. Source Regex Patterns
These patterns provide fast, lightweight scanning of source files to raise warnings/errors before running full AST parsing.

1. **Direct Raw Export/Projection Regex**:
   - **Pattern**: `\bEvidence\s*(::\s*raw\b|<[^>]*Raw[^>]*>).*?\.(project|export|into_exportable|into_projected|into_receipted)\(`
   - **Rationale**: Catches inline creation of Raw evidence that is immediately projected, exported, or transitioned.
   
2. **Raw Type Direct Transition Regex**:
   - **Pattern**: `\b(RawOcelEvidence|RawXesEvidence|Evidence<.*,\s*Raw\s*,.*>|Evidence<.*,\s*Parsed\s*,.*>).*?\.(project|export|into_exportable|into_projected|into_receipted)\(`
   - **Rationale**: Identifies variables annotated or constructed as `Raw`/`Parsed` that invoke restricted lifecycle transitions.

#### B. AST Pattern Matchers
Using a Rust AST parser (e.g. `syn` crate), we construct precise matchers.

1. **Restricted Method Invocation on Raw/Parsed Receivers**:
   - **AST Node**: `syn::ExprMethodCall`
   - **Condition**: Method name is `project`, `export`, `into_projected`, `into_exportable`, or `into_receipted`.
   - **Rule**: If the receiver expression has a type resolved to `Raw` or `Parsed` (either through type annotations, variable binding analysis, or if the receiver is a call to `Evidence::raw`), emit `W4PM-EVD` Error.
   - **AST Representation**:
     ```rust
     // Matching receiver construction
     ExprMethodCall {
         method: "into_projected" | "into_exportable" | "into_receipted" | "project" | "export",
         receiver: Expr::Call(ExprCall {
             func: Expr::Path(func_path), ..
         })
     } // where func_path matches "Evidence::raw"
     ```

2. **Unlawful Helper Function Arguments**:
   - **AST Node**: `syn::ExprCall` or `syn::ExprMethodCall`
   - **Condition**: A call to a function/method that exports or projects a log (e.g., `export(...)` or `project(...)`).
   - **Rule**: If any argument is annotated or typed as `Raw` or `Parsed`, emit `W4PM-EVD` Error.

---

## 2. W4PM-FMT: Format Laundering Prevention

### Doctrine
A raw format byte stream (e.g., OCEL 2.0) must not be rewritten directly into another format byte stream (e.g., XES) without:
1. An **Admission** step to parse and validate the structure under a named `Witness`.
2. A **Project** step governed by an explicit `LossPolicy` that accounts for dropped details (such as dropping multi-object relationships) in a `LossReport`.

Direct format-to-format conversion without these steps is known as **Format Laundering** because it hides structural loss.

```
[Format A Bytes] ──admit──▶ [Admitted Log A] ──project (policy+report)──▶ [Projected Log B] ──export──▶ [Format B Bytes]
                                                                                                        
                                  vs.
                                                                                                        
[Format A Bytes] ────────────────────────── direct rewrite ──────────────────────────▶ [Format B Bytes] (LAUNDERED)
```

### Threat Model & Violations
1. **Ad-hoc Laundering Functions**: Custom functions that take OCEL representation and return XES directly without invoking `Project` or returning `LossReport`.
2. **Bypassing the Admission Gate**: Parsing raw bytes from one format and serializing to another directly within the same pipeline, skipping the typestate lifecycle.

---

### Detection Specifications

#### A. Source Regex Patterns
1. **Direct Conversion Names Regex**:
   - **Pattern**: `\b(ocel_to_xes|xes_to_ocel|xes_to_oced|ocel_flatten_to_xes|xes_enrich_to_oced)\b`
   - **Rationale**: Catches custom conversion functions that name the format transition directly without using the standard `Project` traits.
   
2. **Missing Loss/Policy Pipeline Regex**:
   - **Pattern**: `(ocel|xes|ocpq|dfg).*?(write|serialize|to_bytes).*?(ocel|xes|ocpq|dfg)` (excluding lines containing `LossPolicy`, `LossReport`, `Project`)
   - **Rationale**: Identifies blocks of code that process formats and output bytes without mentioning accountability structures.

#### B. AST Pattern Matchers
1. **Direct Format-to-Format Signature Matcher**:
   - **AST Node**: `syn::ItemFn` or `syn::ImplItemFn`
   - **Condition**: The function signature accepts an OCEL type (e.g., `OcelLog`, `Ocel20`, `RawOcelEvidence`) and returns a XES type (e.g., `XesLog`, `Xes1849`, `RawXesEvidence`) directly, or vice-versa.
   - **Rule**: If the function does not return a `Result<LossReport<...>, ...>` or does not implement the `Project` trait, emit `W4PM-FMT` Error.

2. **Unaccounted Serialization Matcher**:
   - **AST Node**: `syn::ExprCall` or `syn::ExprMethodCall`
   - **Condition**: Invocation of serialization/export logic where the source is parsed directly from raw bytes, without an intermediate type indicating admission.
   - **Rule**: If an export function is invoked and the input is tracked back to a parse/decode call without an intervening `Admit::admit` call, emit `W4PM-FMT` Error.
