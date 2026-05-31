// COMPILE-PASS: nightly_foundry::evidence_law sub-module exposes its law surface
// constructibly — covers the evidence-law paper-derived surface.
//
// Law: Blue River Dam — admitted vs raw label, resolved at compile time via
// min_specialization: blanket impl yields "raw", Admitted<T> override yields
// "admitted". No vtable, no branch, no heap.
#![feature(min_specialization)]

use wasm4pm_compat::nightly_foundry::evidence_law::{Admitted, EvidenceKind};

fn main() {
    // Every T that is not Admitted<_> labels itself "raw".
    let raw_u32: u32 = 99;
    assert_eq!(raw_u32.kind_label(), "raw");

    let raw_str: &str = "hello";
    assert_eq!(raw_str.kind_label(), "raw");

    // Admitted<T> labels itself "admitted".
    let admitted_u32 = Admitted(42u32);
    assert_eq!(admitted_u32.kind_label(), "admitted");

    // Admitted is repr(transparent): the inner value is accessible.
    assert_eq!(admitted_u32.0, 42u32);

    // Nested Admitted<Admitted<T>> labels "admitted" (outermost wins).
    let nested = Admitted(Admitted(7u32));
    assert_eq!(nested.kind_label(), "admitted");

    // The EvidenceKind trait is callable through a generic bound.
    fn check_kind<T: EvidenceKind>(v: &T) -> &'static str {
        v.kind_label()
    }

    assert_eq!(check_kind(&0u8), "raw");
    assert_eq!(check_kind(&Admitted("evidence")), "admitted");
}
