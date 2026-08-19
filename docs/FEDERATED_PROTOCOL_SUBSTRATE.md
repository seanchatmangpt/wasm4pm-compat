# Federated Protocol Substrate

`wasm4pm-compat` owns the **structure-only** protocol law. It does not execute it.

The portable contract is intentionally smaller than Chatman Ecosystem or wasm4pm:

```text
public semantics
  -> capability contract
  -> explicit CLI | HTTP API | MCP | A2A disposition
  -> SELECT | CONSTRUCT | DO
  -> exact external authority decision for DO
  -> receiptability requirement
  -> observed consequence
  -> receipt
  -> OCEL event
  -> replay
```

## Ownership boundary

This crate defines `CapabilityContract`, `ProtocolBundle`, `SurfaceBinding`, typed phase intents, `AuthorityDecisionRef`, `ReceiptRequirement`, and typed refusals. It contains no planner, transport server, authority evaluator, broker, actuator, receipt hasher/verifier, OCEL writer, or replay engine.

`wasm4pm` owns the replaceable execution machinery. Its runtime may verify authority, call a broker, manufacture and replay receipts, and emit OCEL evidence, but it must not redefine the portable semantic contract.

Because the wasm4pm repository requires `wasm4pm-compat` through crates.io, the engine-side implementation uses zero-policy adapter traits until this exact type law is published. After publication, direct adapters from these compat types must contain no selection, authority inference, business policy, or actuation logic.

## Federation law

A transport surface is accessibility, not authority. Every capability must explicitly project, refuse, or declare unsupported status for each common surface. Missing surface declarations are invalid.

`SELECT != CONSTRUCT != DO` is preserved by type construction: reversible SELECT and CONSTRUCT intents have public constructors; a naked DO intent does not. `DoEnvelope` requires exact authority and receiptability references, while runtime verification remains downstream.

OCEL 2 is the initial neutral event-wire family. Public semantics are mandatory in `CapabilityContract`; custom semantics are an optional remainder.

## Falsifiers

The substrate is invalid if any implementation can:

- omit a CLI/API/MCP/A2A disposition silently;
- change semantic identity across surfaces;
- gain authority by being exposed as a transport;
- construct a consequential DO contract with no authority requirement;
- construct consequential DO without required receiptability;
- bind authority for another capability or subject;
- treat UNKNOWN, UNSUPPORTED, and REFUSED as equivalent;
- move execution, policy evaluation, or actuation into wasm4pm-compat.
