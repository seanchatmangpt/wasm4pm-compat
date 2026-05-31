/// Type-law receipt gates — the ALIVE proof for wasm4pm-compat.
///
/// These tests verify that:
/// - **compile-fail** fixtures fail for the **intended named law**, not by accident.
/// - **compile-pass** fixtures compile successfully, proving the lawful path is open.
///
/// ## Running
///
/// These tests are marked `#[ignore]` so they do NOT run with the default
/// `cargo test` (which must complete in ~1 second for the dev cycle). Run
/// them explicitly as the ALIVE gate:
///
/// ```bash
/// # ALIVE gate — run the type-law receipt fixtures:
/// cargo test --test ui_tests -- --ignored
///
/// # Or both regular tests and receipts:
/// cargo test --all-features && cargo test --test ui_tests -- --ignored
/// ```

#[test]
#[ignore = "trybuild compile-time law receipts — run explicitly: cargo test --test ui_tests -- --ignored"]
fn compile_fail_fixtures() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/compile_fail/*.rs");
}

#[test]
#[ignore = "trybuild compile-time law receipts — run explicitly: cargo test --test ui_tests -- --ignored"]
fn compile_pass_fixtures() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/compile_pass/*.rs");
}
