// COMPILE-PASS: GraduationCandidate sealed trait — proves that a type
// implementing both graduation_seal::Sealed and GraduationCandidate satisfies
// the sealed bound and can be passed to graduation-candidate gates.
//
// Law: GraduationCandidateSealLaw — GraduationCandidate requires an explicit
// impl of graduation_seal::Sealed; third-party types cannot accidentally claim
// graduation status without declaring the boundary seam.
use wasm4pm_compat::interop::{graduation_seal, GraduationCandidate};

struct PendingOcelDiscovery;

impl graduation_seal::Sealed for PendingOcelDiscovery {}
impl GraduationCandidate for PendingOcelDiscovery {}

fn only_graduation_candidates<T: GraduationCandidate>(_: &T) {}

fn main() {
    only_graduation_candidates(&PendingOcelDiscovery);
}
