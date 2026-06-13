// COMPILE-PASS: Family-authority law — a Standard-family witness lawfully
// satisfies StandardAuthority, and a Paper-family witness lawfully satisfies
// PaperAuthority. Proves the gated paths are open.
use wasm4pm_compat::witness::{Ocel20, PowlPaper};
use wasm4pm_compat::witness_law::{PaperWitness, StandardWitness};

fn main() {
    // Ocel20 is WitnessFamily::Standard.
    let _s: StandardWitness<Ocel20> = StandardWitness::new();
    // PowlPaper is WitnessFamily::Paper.
    let _p: PaperWitness<PowlPaper> = PaperWitness::new();
}
