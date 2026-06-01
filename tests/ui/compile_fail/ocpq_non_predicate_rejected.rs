// COMPILE-FAIL: OCPQ predicate seal law — A user-defined type cannot satisfy IsOcpqPredicate.
// Law: IsOcpqPredicate is sealed to the seven canonical OCPQ predicate witness markers.
// No external type can be inserted as an OCPQ predicate witness.
use wasm4pm_compat::ocpq::IsOcpqPredicate;

struct NotAPredicate;

fn needs_predicate<W: IsOcpqPredicate>() {}

fn main() {
    // NotAPredicate does not implement IsOcpqPredicate.
    needs_predicate::<NotAPredicate>();
}
