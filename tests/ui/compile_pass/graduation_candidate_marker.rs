// COMPILE-PASS: GraduationCandidate sealed marker trait — a type that
// declares itself a graduation candidate compiles when it implements both
// graduation_seal::Sealed and GraduationCandidate.
//
// Law: graduation boundary covenant — structure-only in wasm4pm-compat;
// execution graduates to wasm4pm. The sealed marker makes the boundary
// explicit at the type level. This fixture proves the sealed path is open
// for explicitly declared candidates, and that the bound is enforced.
//
// The trait lives in wasm4pm_compat::interop (always-on), not behind the
// wasm4pm feature, because the graduation boundary declaration is core
// interop grammar — it is not optional.
use wasm4pm_compat::interop::{GraduationCandidate, graduation_seal};

/// A compat value that requires wasm4pm discovery to resolve.
struct PendingOcelDiscovery;

impl graduation_seal::Sealed for PendingOcelDiscovery {}
impl GraduationCandidate for PendingOcelDiscovery {}

/// A function that only accepts sealed graduation candidates.
fn only_graduation_candidates<T: GraduationCandidate>(_: &T) {}

fn main() {
    let candidate = PendingOcelDiscovery;
    // Only types that implement both Sealed and GraduationCandidate reach here.
    only_graduation_candidates(&candidate);
}
