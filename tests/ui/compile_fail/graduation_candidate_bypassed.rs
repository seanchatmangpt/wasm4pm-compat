// COMPILE-FAIL: GraduationCandidate bypassed — cannot implement GraduationCandidate
// without first implementing graduation_seal::Sealed.
//
// Law: The GraduationCandidate seal is non-forgeable. The trait is declared as
// `pub trait GraduationCandidate: graduation_seal::Sealed {}`. Any type that
// tries to implement GraduationCandidate while bypassing the seal (i.e., without
// explicitly implementing graduation_seal::Sealed) fails with E0277:
// "the trait bound `BypassAttempt: graduation_seal::Sealed` is not satisfied".
//
// The graduation_candidate_as_wasm4pm_bridge fixture proves that a type missing
// both Sealed and GraduationCandidate is rejected by a consumer function.
// THIS fixture proves a different angle: the compiler refuses to COMPILE an impl
// block `impl GraduationCandidate for BypassAttempt` if graduation_seal::Sealed
// is not implemented — the bypass attempt fails at the declaration site.
//
// Expected error: E0277 — trait bound `BypassAttempt: graduation_seal::Sealed`
// is not satisfied (supertrait missing).
use wasm4pm_compat::interop::GraduationCandidate;

/// A type that attempts to claim graduation candidate status without implementing
/// the required seal — a bypass attempt that the type law must reject.
struct BypassAttempt;

// Deliberately skipping:  impl graduation_seal::Sealed for BypassAttempt {}

// This impl block must be rejected because GraduationCandidate has a supertrait
// `graduation_seal::Sealed` that BypassAttempt does not satisfy.
impl GraduationCandidate for BypassAttempt {}

fn main() {
    let _ = BypassAttempt;
}
