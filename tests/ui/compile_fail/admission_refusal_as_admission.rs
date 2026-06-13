// COMPILE-FAIL: Admission law — Refusal<R,W> cannot be passed where Admission<T,W> is required.
// Law: Admission<T,W> (the positive outcome) and Refusal<R,W> (the named refusal)
// are distinct types returned from Admit::admit(). A refusal must not be passed as an admission.
use wasm4pm_compat::admission::{Admission, Refusal};
use wasm4pm_compat::witness::Ocel20;

fn requires_admission(_a: Admission<String, Ocel20>) {}

fn _test(refused: Refusal<&'static str, Ocel20>) {
    // This must fail: Refusal is not Admission.
    requires_admission(refused);
}

fn main() {}
