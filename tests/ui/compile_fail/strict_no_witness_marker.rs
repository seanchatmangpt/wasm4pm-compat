// COMPILE-FAIL: StrictWithoutWitnessMarker — ReceiptBuilder requires W: Witness.
//
// Law: MissingWitness — any compat surface that carries evidence must thread a
// named witness marker through it. A bare struct that does not implement the
// Witness trait cannot serve as the authority for a ReceiptBuilder.
//
// `ReceiptBuilder<W>` has methods only on `impl<W: crate::witness::Witness>`.
// A type `W` that does not implement `Witness` cannot call `ReceiptBuilder::<W>::new()`.
//
// This fixture proves the witness requirement at the type level:
// `NoWitness` is a plain struct with no `Witness` impl. Passing it as `W` to
// `ReceiptBuilder::<NoWitness>::new()` produces E0277:
// "the trait `Witness` is not implemented for `NoWitness`".
//
// Expected error: E0277 — the trait `wasm4pm_compat::witness::Witness` is not
// implemented for `NoWitness`.
use wasm4pm_compat::receipt::ReceiptBuilder;

/// A plain struct that deliberately does NOT implement `wasm4pm_compat::witness::Witness`.
/// Using it as the witness parameter of ReceiptBuilder must be rejected.
struct NoWitness;

fn main() {
    // NoWitness does not implement Witness — this must fail at the type level.
    // The strict boundary covenant requires a named witness marker for any
    // receipt-carrying surface. Bare structs without a Witness impl are refused.
    let _b = ReceiptBuilder::<NoWitness>::new();
}
