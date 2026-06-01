// COMPILE-PASS: RefusedProjection carries a named PowlRefusal — not a bare error.
//
// Law: every POWL projection refusal names a specific structural law variant.
// This fixture proves IrreducibleProjection, CyclicPartialOrder, and
// LanguageMismatch can each be carried in a RefusedProjection.
use wasm4pm_compat::powl::{RefusedProjection, PowlRefusal};

fn main() {
    let r1 = RefusedProjection::new(PowlRefusal::IrreducibleProjection);
    assert_eq!(r1.reason(), &PowlRefusal::IrreducibleProjection);

    let r2 = RefusedProjection::new(PowlRefusal::CyclicPartialOrder);
    assert_eq!(r2.into_reason(), PowlRefusal::CyclicPartialOrder);

    let r3 = RefusedProjection::new(PowlRefusal::LanguageMismatch);
    let display = format!("{}", r3);
    assert!(display.contains("LanguageMismatch"));
}
