# Lifecycle Hooks Cross-Language Map

## Overview

This document maps lifecycle hook patterns across Rust, Python, Java, and TypeScript, with examples from the wasm4pm-compat codebase.

### Lifecycle Phases

Every system has five major lifecycle phases:

1. **INITIALIZATION** — Construct, setup, instantiate
2. **LOADING** — Parse, import, deserialize, read from external sources
3. **VALIDATION** — Check structure, verify constraints, conformance check
4. **EXECUTION** — Run, apply transformation, process state transition
5. **SHUTDOWN** — Cleanup, teardown, resource release, finalization

---

## Rust Lifecycle Hooks

### INITIALIZATION

**Purpose:** Setup, construction, instantiation

| Hook | Purpose |
|------|---------|
| `pub fn new()` | Primary constructor |
| `pub fn with_*()` | Builder pattern |
| `impl Default` | Default trait |
| `pub fn builder()` | Builder return |

### LOADING

**Purpose:** Parse, import, deserialize, read from external sources

| Hook | Purpose |
|------|---------|
| `pub fn from_*()` | Conversion from type |
| `pub fn parse()` | String parsing |
| `pub fn load()` | Load from file/IO |
| `pub fn import()` | Import from format |
| `impl From<T>` | From trait implementation |
| `impl TryFrom<T>` | Fallible conversion |

### VALIDATION

**Purpose:** Structural check, constraint verification, conformance check

| Hook | Purpose |
|------|---------|
| `pub fn validate()` | Full validation |
| `pub fn check()` | Quick validation |
| `pub fn verify()` | Verify property |
| `#[test]` | Unit test |
| `impl Verify` | Verification trait |

### EXECUTION

**Purpose:** Run, apply transformation, process state transition

| Hook | Purpose |
|------|---------|
| `pub fn run()` | Execute main logic |
| `pub fn execute()` | Execute operation |
| `pub fn process()` | Process data |
| `pub fn apply()` | Apply transformation |
| `pub fn transform()` | Data transform |
| `impl Executor` | Executor trait |

### SHUTDOWN

**Purpose:** Cleanup, teardown, resource release, finalization

| Hook | Purpose |
|------|---------|
| `impl Drop` | Drop trait for cleanup |
| `pub fn cleanup()` | Manual cleanup |
| `pub fn close()` | Close/release |
| `pub fn deinit()` | Deinitialization |

---

## Python Lifecycle Hooks

### INITIALIZATION

**Purpose:** Setup, construction, instantiation

| Hook | Purpose |
|------|---------|
| `def __init__()` | Initialization method |
| `def setup()` | Setup method |
| `def __new__()` | Object creation |

### LOADING

**Purpose:** Parse, import, deserialize, read from external sources

| Hook | Purpose |
|------|---------|
| `def from_*()` | Class method constructor |
| `def parse()` | Parse input |
| `def load()` | Load from file |
| `@classmethod` | Class method decorator |

### VALIDATION

**Purpose:** Structural check, constraint verification, conformance check

| Hook | Purpose |
|------|---------|
| `def validate()` | Full validation |
| `def check()` | Quick check |
| `def test_*()` | Test function |
| `def assert_*()` | Assertion |

### EXECUTION

**Purpose:** Run, apply transformation, process state transition

| Hook | Purpose |
|------|---------|
| `def run()` | Main execution |
| `def execute()` | Execute operation |
| `def process()` | Process data |
| `def apply()` | Apply logic |

### SHUTDOWN

**Purpose:** Cleanup, teardown, resource release, finalization

| Hook | Purpose |
|------|---------|
| `def __del__()` | Destructor |
| `def cleanup()` | Manual cleanup |
| `def close()` | Close resource |
| `def __exit__()` | Context manager exit |

---

## Java Lifecycle Hooks

### INITIALIZATION

**Purpose:** Setup, construction, instantiation

| Hook | Purpose |
|------|---------|
| `public Constructor()` | Constructor |
| `public static builder()` | Builder |
| `public void setup()` | Setup |

### LOADING

**Purpose:** Parse, import, deserialize, read from external sources

| Hook | Purpose |
|------|---------|
| `public static from*()` | Static factory |
| `public static parse()` | Parse method |
| `public static load()` | Load method |

### VALIDATION

**Purpose:** Structural check, constraint verification, conformance check

| Hook | Purpose |
|------|---------|
| `public void validate()` | Validation |
| `public boolean check()` | Check property |
| `@Test` | Test annotation |
| `public void assert*()` | Assertion |

### EXECUTION

**Purpose:** Run, apply transformation, process state transition

| Hook | Purpose |
|------|---------|
| `public void run()` | Main execution |
| `public void execute()` | Execute operation |
| `public void process()` | Process data |
| `implements Runnable` | Runnable interface |

### SHUTDOWN

**Purpose:** Cleanup, teardown, resource release, finalization

| Hook | Purpose |
|------|---------|
| `public void close()` | Close/release |
| `public void cleanup()` | Cleanup method |
| `@Override close()` | Override close |
| `protected void finalize()` | Finalizer |

---

## TypeScript Lifecycle Hooks

### INITIALIZATION

**Purpose:** Setup, construction, instantiation

| Hook | Purpose |
|------|---------|
| `constructor()` | Initialization |
| `static create()` | Factory |
| `setup()` | Setup method |

### LOADING

**Purpose:** Parse, import, deserialize, read from external sources

| Hook | Purpose |
|------|---------|
| `static from()` | Static factory |
| `static parse()` | Parse method |
| `static load()` | Load from source |

### VALIDATION

**Purpose:** Structural check, constraint verification, conformance check

| Hook | Purpose |
|------|---------|
| `validate()` | Validation |
| `check()` | Quick check |
| `test: ()->void` | Test function |

### EXECUTION

**Purpose:** Run, apply transformation, process state transition

| Hook | Purpose |
|------|---------|
| `run()` | Main execution |
| `execute()` | Execute operation |
| `process()` | Process data |

### SHUTDOWN

**Purpose:** Cleanup, teardown, resource release, finalization

| Hook | Purpose |
|------|---------|
| `destroy()` | Cleanup |
| `close()` | Close resource |
| `cleanup()` | Manual cleanup |

---

## Cross-Language Patterns

### Constructor Patterns

| Pattern | Rust | Python | Java | TypeScript |
|---------|------|--------|------|-----------|
| Basic constructor | `fn new()` | `__init__()` | `Constructor()` | `constructor()` |
| Builder | `fn builder()` | `@classmethod` | `Builder` | `static create()` |
| Factory | `impl From<T>` | `@classmethod` | `static of()` | `static from()` |

### Validation Patterns

| Pattern | Rust | Python | Java | TypeScript |
|---------|------|--------|------|-----------|
| Full validation | `validate()` | `validate()` | `validate()` | `validate()` |
| Quick check | `check()` | `check()` | `isValid()` | `check()` |
| Unit test | `#[test]` | `test_*()` | `@Test` | `test:()=>void` |

### Resource Cleanup Patterns

| Pattern | Rust | Python | Java | TypeScript |
|---------|------|--------|------|-----------|
| Automatic cleanup | `impl Drop` | `__del__()` | `finalize()` | Garbage collection |
| Manual cleanup | `cleanup()` | `close()` | `close()` | `destroy()` |
| Context manager | `impl Drop` | `__exit__()` | try-with-resources | `finally` block |

---

## wasm4pm-compat Lifecycle Patterns

### Evidence Lifecycle (src/evidence.rs)

The core lifecycle of evidence through states:

```
Raw ──parse──▶ Parsed ──admit──▶ Admitted ──▶ {Projected | Exportable | Receipted}
  │                                  ▲
  └────────────── refuse ────────────┴──▶ Refused  (terminal; carries a named law)
```

**Key constructors:**
- `Evidence::raw(v)` — Instantiate raw evidence
- `evidence.into_parsed()` — Parse transition
- `Admit::admit()` — Admission gate (only legal path to admitted)
- `evidence.into_admitted()` — Finalize to admitted state
- `evidence.into_projected()` — Project to output format
- `evidence.into_receipted()` — Generate immutable receipt

### Admission & Refusal (src/admission.rs)

| Hook | Signature | Purpose |
|------|-----------|---------|
| Constructor | `pub fn raw(v: T) -> Evidence<T, Raw, W>` | Create raw evidence |
| Admission check | `pub fn validate() -> Result<(), *Refusal>` | Structural validation |
| Admission gate | `impl Admit<T, W>` | Only lawful path to admitted |
| Refusal reason | `Refusal<R, W>` | Named law that was violated |

### Validation (src/eventlog.rs, src/ocel.rs)

- `EventLog::validate()` — Structural event log checks
- `OcelLog::validate()` — Object-centric event log validation
- `Dfg::validate()` — Directly-follows graph conformance
- All return `Result<(), *Refusal>` (not bare `InvalidInput`)

### Execution Hooks (Examples)

- `EventLog::from_traces([…])` — LOADING: construct from trace list
- `OcelEvent::new(…).at_ns(…).by(…)` — INITIALIZATION: builder pattern
- `Process::run()` — EXECUTION: main transform
- `impl Drop for Evidence` — SHUTDOWN: automatic cleanup (if added)

### Loss Accounting (src/loss.rs)

| Hook | Phase | Purpose |
|------|-------|---------|
| `Project::new(policy)` | INITIALIZATION | Declare loss policy upfront |
| `project(policy)` | VALIDATION | Validate loss is acceptable |
| `LossReport::new()` | EXECUTION | Emit loss accounting |
| `into_exportable()` | SHUTDOWN | Finalize with loss report |

---

## Test Surface Lifecycle

### Unit & Integration Tests (`cargo test --tests`)

- **Pattern:** `#[test] fn test_*() { … }`
- **Phase:** VALIDATION
- **Setup:** `let log = EventLog::from_traces([…])`
- **Check:** `log.validate()?`
- **Cleanup:** Automatic (stack drop)

### Trybuild Fixtures (ALIVE gate, `cargo test --test ui_tests -- --ignored`)

- **Compile-fail:** Type law must reject at **intended** reason
- **Compile-pass:** Lawful path must compile
- **Pattern:** File path encodes law, `.stderr` validates error message
- **Phase:** VALIDATION (type-level)

### Doctests (`cargo test --doc --all-features`)

- **Pattern:** `/// # Example:` block in rustdoc
- **Phase:** VALIDATION (documentation correctness)
- **Disabled by default:** (`doctest = false` in Cargo.toml)

---

## Implementation Checklist

When adding lifecycle hooks to a new type:

- [ ] **INITIALIZATION** — Provide `fn new()` or builder
- [ ] **LOADING** — Provide `fn from_*()` or `impl From<T>`
- [ ] **VALIDATION** — Provide `fn validate() -> Result<(), Refusal>`
- [ ] **EXECUTION** — Document state transitions
- [ ] **SHUTDOWN** — Document cleanup behavior (auto vs manual)
- [ ] **Tests** — Add `#[test]` and doctest examples
- [ ] **Doc** — Document what each phase does, not just how
