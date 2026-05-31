// COMPILE-PASS: Evidence<T, Admitted, W> can only be constructed via the lawful
// Admit path — Admission::new(value).into_evidence().
//
// Law: One-way door — the only public route to Admitted evidence is through an
// Admit impl returning Admission<T, W>. Evidence::sealed is pub(crate) and
// unreachable from outside the crate.
use wasm4pm_compat::admission::{Admit, Admission, Refusal};
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::state::{Admitted, Raw};
use wasm4pm_compat::witness::Ocel20;

/// A named boundary: admits OCEL logs whose top-level object count is non-zero.
enum NonEmptyObjectCount {}

impl Admit for NonEmptyObjectCount {
    type Raw = u32;
    type Admitted = u32;
    type Reason = &'static str;
    type Witness = Ocel20;

    fn admit(
        raw: Evidence<u32, Raw, Ocel20>,
    ) -> Result<Admission<u32, Ocel20>, Refusal<&'static str, Ocel20>> {
        if raw.value == 0 {
            Err(Refusal::new("ZeroObjectCount"))
        } else {
            Ok(Admission::new(raw.value))
        }
    }
}

fn main() {
    // The lawful path: raw → Admit::admit → Admission → into_evidence → Admitted.
    let raw: Evidence<u32, Raw, Ocel20> = Evidence::raw(5u32);
    let admission = NonEmptyObjectCount::admit(raw).expect("non-zero count must be admitted");
    let admitted: Evidence<u32, Admitted, Ocel20> = admission.into_evidence();

    // Admitted evidence exposes its value via into_inner.
    assert_eq!(admitted.into_inner(), 5u32);

    // The refusal path produces a named reason, not a bare error string.
    let raw_zero: Evidence<u32, Raw, Ocel20> = Evidence::raw(0u32);
    let refused = NonEmptyObjectCount::admit(raw_zero).unwrap_err();
    assert_eq!(refused.reason, "ZeroObjectCount");

    // Shorthand: Admission::new directly (used inside Admit impls or tests).
    let direct = Admission::<_, Ocel20>::new(42u32).into_evidence();
    assert_eq!(direct.into_inner(), 42u32);
}
