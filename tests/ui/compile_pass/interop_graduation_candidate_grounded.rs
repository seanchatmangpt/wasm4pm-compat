// COMPILE-PASS: GraduationCandidate sealed trait — multiple graduation candidate
// types prove the sealed graduation seam is open for explicitly declared types.
//
// Law: GraduationCandidateSealedTraitLaw — the graduation boundary is always-on
// interop grammar; implementing GraduationCandidate requires the explicit seal.
// This fixture proves three distinct host types can implement the sealed trait,
// and that the bound is type-level (not a runtime check).
use wasm4pm_compat::interop::{GraduationCandidate, graduation_seal};

/// A compat value awaiting model discovery.
struct PendingDiscovery {
    log_ref: String,
}

impl graduation_seal::Sealed for PendingDiscovery {}
impl GraduationCandidate for PendingDiscovery {}

/// A compat value awaiting conformance execution.
struct PendingConformance;

impl graduation_seal::Sealed for PendingConformance {}
impl GraduationCandidate for PendingConformance {}

/// A compat value that is rebuilding process mining locally — hard graduation signal.
struct RebuildingPmLocally {
    reason: &'static str,
}

impl graduation_seal::Sealed for RebuildingPmLocally {}
impl GraduationCandidate for RebuildingPmLocally {}

/// Proves the bound is enforced: only sealed graduation candidates pass this gate.
fn only_graduation_candidates<T: GraduationCandidate>(_: &T) {}

fn main() {
    let discovery = PendingDiscovery {
        log_ref: "blake3:abc123".into(),
    };
    only_graduation_candidates(&discovery);

    let conformance = PendingConformance;
    only_graduation_candidates(&conformance);

    let rebuilding = RebuildingPmLocally {
        reason: "host is re-implementing inductive miner",
    };
    only_graduation_candidates(&rebuilding);
    assert_eq!(rebuilding.reason, "host is re-implementing inductive miner");
}
