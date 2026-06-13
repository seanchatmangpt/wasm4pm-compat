// COMPILE-FAIL: Family-authority law — a Paper-family witness cannot satisfy
// StandardAuthority.
//
// Law: StandardWitness<W> requires W: StandardAuthority, which is sealed to
// WitnessFamily::Standard witnesses only. PowlPaper is a Paper-family witness,
// so it does not implement StandardAuthority. The failure carries a teaching
// diagnostic (#[diagnostic::on_unimplemented]) naming the law, because the
// sealed-trait bound fails as E0277 — not the E0308 const-unification path that
// the arithmetic const-laws take.
//
// Expected error: `PowlPaper` is not a Standard-family authority
use wasm4pm_compat::witness::PowlPaper;
use wasm4pm_compat::witness_law::StandardWitness;

fn main() {
    // PowlPaper is WitnessFamily::Paper — cannot be a StandardWitness.
    let _forged: StandardWitness<PowlPaper> = StandardWitness::new();
}
