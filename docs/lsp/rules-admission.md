# Diagnostic Rules Specification: Admission & Lossy Projections

This document specifies the LSP diagnostic rules for boundary admission validation (**W4PM-ADM**) and lossy projections (**W4PM-LOS**). These rules are enforced via static analysis of the Rust AST to prevent panic-based bypasses of the admission system and silent structural information loss during type or format projections.

---

## 1. Boundary Admission Validation (W4PM-ADM)

### Context & Goal
The `Admit` trait in `wasm4pm-compat` is the exclusive gatekeeper for converting untrusted, raw evidence (`Raw`) into trusted, admitted evidence (`Admitted`). The trait method is defined as:
```rust
fn admit(
    raw: Evidence<Self::Raw, Raw, Self::Witness>,
) -> Result<Admission<Self::Admitted, Self::Witness>, Refusal<Self::Reason, Self::Witness>>;
```
Boundary decisions must be first-class, returning a structured `Refusal` on validation failure. A `panic!` or `unwrap()` within an `Admit` implementation bypasses this contract, crashing the runtime/service instead of returning an auditable refusal verdict.

### Rule W4PM-ADM-001: Forbidden Panics and Unwraps in `Admit` Implementations
- **Severity**: `Error`
- **Scope**: Any implementation of the `Admit` trait (`impl Admit for T` or `impl<...> Admit for T`).
- **Trigger**: The presence of panic-inducing constructs within any function inside the `Admit` implementation.
- **Forbidden Constructs**:
  1. **Macros**: `panic!`, `todo!`, `unimplemented!`, `assert!`, `assert_eq!`, `assert_ne!`, `unreachable!`.
  2. **Method Calls**: `.unwrap()` and `.expect()`.

#### AST Pattern Matcher (syn)
For any `syn::ItemImpl` where the trait is `Admit`:
1. Iterate over `impl_items` to find functions (`ImplItem::Fn`).
2. Traverse the function's block (`syn::Block`) using an AST visitor.
3. Match `syn::ExprMacro` where the macro path matches one of the forbidden macros.
4. Match `syn::ExprMethodCall` where the method identifier is `unwrap` or `expect`.

#### Diagnostic Message
- Code: `W4PM-ADM-001`
- Message: `"Explicit panic or unwrap/expect in Admit implementation. Boundary decisions must return a Refusal instead of panicking."`

---

## 2. Lossy Projections (W4PM-LOS)

### Context & Goal
Projections between evidence shapes (specifically object-centric formats like OCEL and single-case formats like XES) are often lossy. The **Loss Law** demands that all structural loss be accountable:
$$\text{LossPolicy} \longrightarrow \text{ProjectionName} \longrightarrow \text{LossReport}$$
No silent structural loss is permitted. Any projection must carry a `LossPolicy` (decided by the caller before projection) and produce a `LossReport` containing the `ProjectionName` and the policy under which it was executed.

### Rule W4PM-LOS-001: Missing Loss Accounting in Projection Traits
- **Severity**: `Error`
- **Scope**: Any implementation of the `Project` trait (`impl Project for T` or `impl<...> Project for T`).
- **Requirements**:
  1. The implementation must define the associated types: `From`, `To`, `Lost`, `Reason`.
  2. The `project` function must accept a parameter of type `LossPolicy`.
  3. The return type of `project` must be `Result<LossReport<Self::From, Self::To, Self::Lost>, Self::Reason>`.

#### AST Pattern Matcher (syn)
For any `syn::ItemImpl` where the trait is `Project`:
1. Verify that `impl_items` contains associated types (`ImplItem::Type`) named `From`, `To`, `Lost`, and `Reason`.
2. Find the method named `project`.
3. Verify that its signature includes an argument of type `LossPolicy`.
4. Verify that its return type is a `Result` where the success variant is `LossReport<...>` (which resolves to `LossReport<Self::From, Self::To, Self::Lost>` or equivalent generic parameters).

#### Diagnostic Message
- Code: `W4PM-LOS-001`
- Message: `"Project implementation does not conform to the Loss Law. Implementations must take a LossPolicy and return a Result containing a LossReport."`

---

### Rule W4PM-LOS-002: Incomplete LossReport Construction
- **Severity**: `Warning` / `Error`
- **Scope**: Within the implementation of the `project` method (or custom projection functions).
- **Trigger**: Instantiation of a `LossReport` where:
  - The `projection` argument is not a valid `ProjectionName` wrapper.
  - The `policy` argument is not the passed `LossPolicy` variable.
  - The `ProjectionName` is constructed with an empty or trivial string literal (e.g. `""`, `"temp"`, or `"default"`).

#### AST Pattern Matcher (syn)
Within `project` (or any method returning `LossReport`):
1. Locate struct construction `LossReport { ... }` or function calls like `LossReport::new(...)`.
2. Inspect the arguments passed to `LossReport::new(projection, policy, lost)` or the fields initialized:
   - Ensure the policy argument corresponds to the `LossPolicy` parameter of the containing function.
   - Inspect the `ProjectionName(...)` expression. If it is a string literal, ensure it is non-empty and does not contain generic placeholders.

#### Diagnostic Message
- Code: `W4PM-LOS-002`
- Message: `"LossReport constructed with potential policy bypass or empty ProjectionName. Ensure the caller-provided LossPolicy is carried forward, and the ProjectionName is meaningful."`

---

## 3. Summary of Diagnostic Code Registry

| Diagnostic Code | Severity | Description | Remediation |
|---|---|---|---|
| **W4PM-ADM-001** | `Error` | Panic/unwrap inside `Admit` impl | Replace with `Err(Refusal::new(...))` |
| **W4PM-LOS-001** | `Error` | Non-conforming `Project` impl | Ensure method accepts `LossPolicy` and returns `LossReport` |
| **W4PM-LOS-002** | `Error` | Invalid `LossReport` instantiation | Pass the parameter `policy` and a non-empty `ProjectionName` |
