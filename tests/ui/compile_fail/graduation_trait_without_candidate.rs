// COMPILE-FAIL: Rejects implementing GraduateToWasm4pm on a type that does not satisfy
// GraduationCandidate — proves the graduation seal cannot be forged.
//
// Law: GraduateToWasm4pm::candidate() must return a genuine GraduationCandidate.
// The trait is defined with a concrete return type (not a generic/associated type),
// so any implementor must produce exactly GraduationCandidate — not a structurally
// similar substitute, not a newtype wrapper, not a subtype. The graduation seal is
// non-forgeable at the type-law boundary.
//
// This fixture mirrors the real graduation module types inline so it can run without
// the `wasm4pm` feature gate. It proves the axiom holds for any Rust implementation
// of the trait: a ForgedCandidate is not a GraduationCandidate.
//
// Expected error: mismatched types — `fn candidate(&self) -> ForgedCandidate` is not
// compatible with the `GraduateToWasm4pm` trait's required return type `GraduationCandidate`.

// ── Mirror of wasm4pm_compat::graduation (always-on proof of the law) ──────────

/// Mirror of the real GraduationReason — the valid graduation signal set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GraduationReason {
    NeedsDiscovery,
}

/// Mirror of the real GraduationCandidate — the ONLY lawful graduation artifact.
struct GraduationCandidate {
    reason: GraduationReason,
    subject: String,
    evidence_ref: String,
}

/// Mirror of the real GraduateToWasm4pm trait — concrete return type, non-forgeable.
trait GraduateToWasm4pm {
    fn candidate(&self) -> GraduationCandidate;
}

// ── The forgery attempt ─────────────────────────────────────────────────────────

/// A structurally identical but type-distinct "ForgedCandidate".
/// Attempts to use this as the return type of `candidate()` must be rejected.
struct ForgedCandidate {
    reason: GraduationReason,
    subject: String,
    evidence_ref: String,
}

struct TypeWithoutCandidate;

impl GraduateToWasm4pm for TypeWithoutCandidate {
    // ERROR: the trait requires `-> GraduationCandidate`, not `-> ForgedCandidate`.
    // No amount of structural similarity bypasses the nominal type check.
    fn candidate(&self) -> ForgedCandidate {
        ForgedCandidate {
            reason: GraduationReason::NeedsDiscovery,
            subject: "forged".into(),
            evidence_ref: "blake3:forged".into(),
        }
    }
}

fn main() {
    let _ = TypeWithoutCandidate.candidate();
}
