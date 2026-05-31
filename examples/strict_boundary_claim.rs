//! A strict-mode boundary claim, declared and checked.
//!
//! Run with: `cargo run --example strict_boundary_claim --features strict`
//!
//! Strict mode is opt-in *judgment*: a host declares the process boundaries it
//! crosses and strict mode checks each declaration against the boundary covenant,
//! reporting *every* named violation. This example declares a healthy export
//! boundary, then a dishonest one, and shows the specifically-named verdicts.

#[cfg(feature = "strict")]
fn main() {
    use wasm4pm_compat::strict::{
        ProcessBoundary, ProcessBoundaryKind, StrictCheck, StrictViolation,
    };

    println!("Strict boundary claims (declaration + check, no data touched)\n");

    // 1. A fully-attested export boundary passes the covenant.
    let healthy = ProcessBoundary::fully_attested(ProcessBoundaryKind::ExportsFormat, "xes-out");
    match healthy.check() {
        Ok(()) => println!("[1] '{}' — clean: all obligations met.", healthy.name),
        Err(v) => unreachable!("a fully-attested boundary must pass: {v:?}"),
    }

    // 2. The same boundary, but it forgot its loss policy AND its refusal path.
    let mut dishonest =
        ProcessBoundary::fully_attested(ProcessBoundaryKind::ExportsFormat, "xes-out-bad");
    dishonest.has_loss_policy = false;
    dishonest.has_refusal_path = false;

    match dishonest.check() {
        Ok(()) => unreachable!("a dishonest export must be refused"),
        Err(violations) => {
            println!("\n[2] '{}' — refused with named laws:", dishonest.name);
            for v in &violations {
                println!("    - {}", v.law());
            }
            assert!(violations.contains(&StrictViolation::MissingLossPolicy));
            assert!(violations.contains(&StrictViolation::MissingRefusalPath));
        }
    }

    // 3. A boundary that secretly grew engine capability cannot stay in compat.
    let grown = ProcessBoundary::fully_attested(ProcessBoundaryKind::ClaimsReplay, "replay-here");
    match grown.check() {
        Ok(()) => unreachable!("a replay claim is engine capability; it must refuse"),
        Err(violations) => {
            println!("\n[3] '{}' — engine capability detected:", grown.name);
            assert!(violations.contains(&StrictViolation::HiddenProcessMiningGrowth));
            println!("    - {}", StrictViolation::HiddenProcessMiningGrowth.law());
            println!("    fix: graduate to wasm4pm (see docs/GRADUATION.md)");
        }
    }

    println!("\nStrict mode judged declarations only — it never replayed a log.");
}

#[cfg(not(feature = "strict"))]
fn main() {
    eprintln!(
        "This example needs the `strict` feature.\n\
         Run with: cargo run --example strict_boundary_claim --features strict"
    );
}
