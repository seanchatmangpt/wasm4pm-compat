// Law: NamedLossCopyLaw — NamedLoss is Copy; a loss descriptor can be reused after assignment without ownership transfer
// COMPILE-PASS: NamedLoss Copy semantics — proves NamedLoss is Copy and can be used after a move

use wasm4pm_compat::loss::{NamedLoss, ProjectionName};

fn main() {
    let a = NamedLoss::new(ProjectionName("p"), "SomeLoss");
    let b = a; // copy, not move
    // a is still usable
    assert_eq!(a.category(), b.category());
    assert_eq!(a.projection().as_str(), "p");
}
