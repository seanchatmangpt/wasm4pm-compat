# Explanation: The Nightly-Only Toolchain Requirement

This document discusses the architectural context and technical rationale for why `wasm4pm-compat` version `26.6.14` permanently requires the nightly Rust compiler toolchain.

---

## The Core Objective: Compile-Time Verification

In process mining, ensuring that a workflow model is structurally correct (for example, that condition cell bit-widths do not exceed limits or that metric ratings lie strictly between 0 and 1) is typically done at runtime via assertions and checks.

To achieve **zero-cost abstraction**, `wasm4pm-compat` shifts these verification checks from runtime execution to compile-time analysis. If a developer attempts to declare an invalid structure, the code will fail to compile. This requires features in Rust's type system that are currently only available on the nightly release channel.

---

## Required Nightly Feature Flags

The crate root (`src/lib.rs`) declares six specific nightly compiler features:

```rust
#![feature(generic_const_exprs)]
#![feature(adt_const_params)]
#![feature(unsized_const_params)]
#![feature(const_trait_impl)]
#![feature(min_specialization)]
#![feature(portable_simd)]
```

### 1. `generic_const_exprs`
This feature allows compile-time expressions inside type-level bounds. For example, when declaring the Blue River Dam covenant's `ConditionCell<const BITS: usize>`, the type bounds require the compiler to evaluate the expression `BITS <= 8`:

```rust
pub struct ConditionCell<const BITS: usize>
where
    Require<{ BITS <= 8 }>: IsTrue,
{
    _private: (),
}
```

Without `generic_const_exprs`, Rust cannot evaluate mathematical inequalities inside `where` bounds during compilation.

> **Durability note.** The compiler team now treats `generic_const_exprs` as superseded by
> `min_generic_const_args` (mGCA). This crate **cannot** migrate: mGCA forbids generic
> parameters in computed const operations (so `BITS <= 8` over a generic `BITS` is not
> expressible), and mGCA cannot coexist with `generic_const_exprs` in one crate. The crate
> therefore stays on `generic_const_exprs` behind a pinned dated nightly until mGCA's
> non-min expansion lands. See `docs/STABILITY.md` for the full finding.

### 2. `adt_const_params` and `unsized_const_params`
By default, Rust only supports primitive integers, characters, and booleans as const-generic parameters. These two features expand compile-time parameterization to user-defined structures (Abstract Data Types) and unsized values (such as static strings `&'static str`).
- Used in `ObjectState<const PHASE: ObjectLifecyclePhase>` to track the object lifecycle enum value at the type level.
- Used in `CorrelationKey<const SCHEMA: &'static str>` to compile schema identifiers directly into type signatures.

### 3. `const_trait_impl`
Enables calling trait methods inside `const fn` contexts. This is necessary to initialize constant-generic assertions and evaluate compliance rules when constructing compile-pass boundary shapes.

### 4. `min_specialization`
Allows providing specialized, optimized trait implementations for specific type combinations. This is used to define zero-cost transitions on specialized `Evidence` containers (for example, optimization paths when transitioning admitted OCEL logs compared to raw logs) without generating duplicate monomorphized code.

### 5. `portable_simd`
Enables vectorization checks for performance-critical ID and byte scanning in event log parsers.

---

## Conclusion & Outlook

The nightly toolchain is not a temporary choice for a prototype. It is a permanent architectural covenant. The compile-time guarantees, zero runtime size overhead (ZSTs), and type-level law checks are built directly on these cutting-edge compiler features.
