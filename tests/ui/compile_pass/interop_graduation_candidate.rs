// Law: GraduationCandidateSealedTraitLaw — implementing the sealed graduation_seal::Sealed + GraduationCandidate is the lawful graduation path; the seal cannot be bypassed
// COMPILE-PASS: GraduationCandidate sealed trait — proves a type can implement the sealed GraduationCandidate

use wasm4pm_compat::interop::{graduation_seal, GraduationCandidate};

struct PendingOcelDiscovery;

impl graduation_seal::Sealed for PendingOcelDiscovery {}
impl GraduationCandidate for PendingOcelDiscovery {}

fn only_graduation_candidates<T: GraduationCandidate>(_: &T) {}

fn main() {
    only_graduation_candidates(&PendingOcelDiscovery);
}
