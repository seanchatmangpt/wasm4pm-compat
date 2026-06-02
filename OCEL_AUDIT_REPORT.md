# OCEL 2.0 Audit Report - wasm4pm-compat

This report lists the discrepancies found during the audit of `OcelLogTs` and related structures for OCEL 2.0 completeness.

### 1. Timestamp Precision (Critical)
- **2.0 Requirement**: `TimestampNs` should be handled as BigInt or string in transport to prevent precision loss.
- **Finding**: `OcelAttributeValueTslots::TimestampNs` uses `i64`, which serializes to a JSON Number. In JavaScript, this will lose precision for nanosecond timestamps (exceeding `2^53 - 1`).
- **Finding**: `OcelEventTs.:timestamp_ns` and `ObjectChangeTs.:timestamp_ns` also use `i64`, facing the same issue.
- **Finding**: `AdmissionTs.:admitted_at_ns` and `ReceiptShapeTs.:timestamp_ns` use `f64`, which is architecturally unsafe for nanosecond precision/

### 2. Object Changes Typing
- **2.0 Requirement**: Object changes (evolution) should support typed values.
- **Finding**: `ObjectChangeTs.:value` is currently a `String`. It should be of type `OcelAttributeValueTs` to match the OCEL 2.0 specification for typed attribute evolution.

### 3. Missing OCEL 2.0 Type Tables
- **2.0 Requirement**: A complete OCEL 2.0 log should include `event-types` and `object-types` definitions.
- **Finding**: `OcelLogTs` is missing dedicated tables for `event_types` and `object_types`, which are necessary for full OCEL 2.0 completeness and schema enforcement.

### 4. Object-Object Links Support
- **Status**: Passed.
- **Observation**: `ObjectObjectLinkTs` is present and included in `OcelLogTs`, correctly supporting O2O relationships with qualifiers.
