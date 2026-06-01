// COMPILE-FAIL: Graduation seal law — An external type cannot implement interop::GraduationCandidate
// without first implementing graduation_seal::Sealed.
// Law: GraduationCandidate is sealed; a bare struct cannot satisfy the trait bound
// because it lacks the graduation_seal::Sealed impl that only the sealing module controls.
use wasm4pm_compat::interop::GraduationCandidate;

// A user-defined type that did NOT implement graduation_seal::Sealed.
struct MyCandidate;

fn only_graduation_candidates<T: GraduationCandidate>(_: &T) {}

fn main() {
    // MyCandidate does not implement GraduationCandidate (missing seal).
    only_graduation_candidates(&MyCandidate);
}
