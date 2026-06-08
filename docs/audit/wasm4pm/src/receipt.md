# Formal Audit Report: `wasm4pm/src/receipt.rs`

## Cryptographic Provenance and Evidence Formalism

The `receipt.rs` module governs the verification of cryptographic receipts, serving as the final arbiter of evidence truth before the system accepts external models or logs.

### Observations:
1. **Anti-Synthetic Proofs**: The `SyntheticMarkerScanner` enforces strict structural rejection of placeholder identifiers, stubs, and mocks. This provides a hard boundary against "data laundering" where synthetic test data is improperly treated as real process evidence.
2. **Canonical Hashing**: The `CanonicalHashVerifier` correctly re-serializes OCEL and receipt payloads to verify `BLAKE3` and `SHA256` integrity. Crucially, it verifies the *contents* of the execution artifacts against the signed receipt hash, proving non-repudiation.
3. **Isomorphism Detection**: The `ExpectedObservedCloneDetector` serves as an advanced cryptographic oracle. It analyzes event streams to detect isomorphic "clones" (where expected paths are simply duplicated into observed paths with static timestamp templates). This formally blocks fixture mutation attacks.
4. **First-Class Diagnostic Refusals**: The `ReceiptDoctor` returns strongly-typed refusal codes (e.g., `ObservedOCELMissing`, `BoundaryEvidenceMissing`, `ClosureOverclaimed`) rather than strings, aligning with the "one-way door" admissibility laws of `wasm4pm-compat`.

### Conclusion:
The receipt verification module is forensically robust. It enforces process-mining data integrity at the cryptographic level, ensuring that only verified, structurally-sound, non-synthetic execution artifacts are accepted as truth.
